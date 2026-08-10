use crate::platform::{self, BindingState};
use crate::recovery::{
    MAX_RECOVERABLE_FILE_BYTES, RecoveryFileProof, RecoveryJournal, RecoveryName, RecoveryPhase,
    RecoveryRecord, RecoveryRegistration, recovery_park_leaf, recovery_stage_leaf,
};
use crate::recovery_owns_park;
use crate::{EntryKind, MAX_DIRECTORY_LIST_ENTRIES, identity_changed, leaf_names_equivalent};
use sha2::{Digest as _, Sha256};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs::File;
use std::io;

#[cfg(test)]
thread_local! {
    static REPLAY_PARENT_VALIDATION_HOOK: std::cell::RefCell<Option<Box<dyn FnOnce()>>> =
        std::cell::RefCell::new(None);
    static PROOF_BYTES_READ: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
pub(crate) fn set_replay_parent_validation_hook(hook: impl FnOnce() + 'static) {
    REPLAY_PARENT_VALIDATION_HOOK.with(|slot| {
        assert!(slot.borrow_mut().replace(Box::new(hook)).is_none());
    });
}

#[cfg(test)]
fn run_replay_parent_validation_hook() {
    REPLAY_PARENT_VALIDATION_HOOK.with(|slot| {
        if let Some(hook) = slot.borrow_mut().take() {
            hook();
        }
    });
}

#[cfg(test)]
pub(crate) fn reset_proof_bytes_read() {
    PROOF_BYTES_READ.with(|bytes| bytes.set(0));
}

#[cfg(test)]
pub(crate) fn proof_bytes_read() -> u64 {
    PROOF_BYTES_READ.with(std::cell::Cell::get)
}

struct ObservedFile {
    handle: File,
    identity: platform::Identity,
    receipt: (u64, platform::FileStamp),
    proof: Option<RecoveryFileProof>,
}

enum ObservedEntry {
    Absent,
    File(ObservedFile),
    Unowned,
    UnownedOccupied,
}

struct RetainedParentBinding {
    parent: platform::DirectoryHandle,
    parent_identity: platform::Identity,
    parent_revision: platform::DirectoryStamp,
    name: RecoveryName,
    child_identity: platform::Identity,
}

struct PlanningAncestry {
    parent: platform::DirectoryHandle,
    bindings: Vec<RetainedParentBinding>,
}

struct ReplayPlan {
    registration: RecoveryRegistration,
    record: RecoveryRecord,
    parent: platform::DirectoryHandle,
    parent_bindings: Vec<RetainedParentBinding>,
    parent_revision: platform::DirectoryStamp,
    stage: ObservedEntry,
    target: ObservedEntry,
    park: ObservedEntry,
    publication: Option<platform::PublicationReceipt>,
}

pub(crate) struct RecoveryReplay {
    journal: RecoveryJournal,
    plans: Option<Vec<ReplayPlan>>,
    planning_files: Vec<(platform::Identity, File)>,
    planning_ancestries: Vec<PlanningAncestry>,
    retained_partial_plans: Vec<ReplayPlan>,
    attempted: bool,
}

impl RecoveryReplay {
    fn resume(
        mut self,
        root: &platform::RootGuard,
        lease: &platform::LeaseHandle,
    ) -> Result<(RecoveryJournal, Vec<crate::RecoveryOrphan>), (io::Error, RecoveryReplay)> {
        if self.journal.is_uncertain() {
            if let Err(error) = self.journal.reconcile_uncertain(lease) {
                return Err((error, self));
            }
            if let Some(plans) = self.plans.as_mut() {
                let mut alignment_error = None;
                plans.retain_mut(|plan| {
                    let Some(record) = self.journal.record(plan.registration) else {
                        return false;
                    };
                    if record.operation_id != plan.record.operation_id {
                        alignment_error = Some(codec_error());
                        return true;
                    }
                    plan.record = record.clone();
                    if !plan.record.phase.owns_target() {
                        plan.target = ObservedEntry::Unowned;
                    }
                    if !recovery_owns_park(&plan.record) {
                        plan.park = ObservedEntry::Unowned;
                    }
                    true
                });
                if let Some(error) = alignment_error {
                    return Err((error, self));
                }
            }
        }
        if self.attempted && self.plans.is_some() {
            for file in self
                .plans
                .as_ref()
                .expect("attempted replay retains plans")
                .iter()
                .flat_map(|plan| [&plan.stage, &plan.target, &plan.park])
                .filter_map(|entry| match entry {
                    ObservedEntry::File(file) => Some(file),
                    _ => None,
                })
            {
                if !self
                    .planning_files
                    .iter()
                    .any(|(identity, _)| *identity == file.identity)
                {
                    match file.handle.try_clone() {
                        Ok(handle) => self.planning_files.push((file.identity, handle)),
                        Err(error) => return Err((error, self)),
                    }
                }
            }
            match plan_replay(
                root,
                &self.journal,
                &mut self.planning_files,
                &mut self.planning_ancestries,
            ) {
                Ok(mut plans) => {
                    if let Some(previous) = self.plans.as_ref() {
                        for old in previous.iter().filter(|plan| plan.publication.is_some()) {
                            let Some(fresh) = plans
                                .iter()
                                .find(|plan| plan.record.operation_id == old.record.operation_id)
                            else {
                                return Err((
                                    invalid("pending recovery publication lost its journal record"),
                                    self,
                                ));
                            };
                            if !absent(&fresh.stage)
                                || !matches!(&fresh.target, ObservedEntry::File(_))
                            {
                                return Err((
                                    identity_changed(
                                        "pending recovery publication changed topology",
                                    ),
                                    self,
                                ));
                            }
                        }
                    }
                    if let Some(previous) = self.plans.as_mut() {
                        for old in previous
                            .iter_mut()
                            .filter(|plan| plan.publication.is_some())
                        {
                            plans
                                .iter_mut()
                                .find(|plan| plan.record.operation_id == old.record.operation_id)
                                .expect("pending publication mapping was prevalidated")
                                .publication = old.publication.take();
                        }
                    }
                    self.plans = Some(plans);
                    self.planning_files.clear();
                    self.planning_ancestries.clear();
                    self.retained_partial_plans.clear();
                    self.attempted = false;
                }
                Err((error, partial)) => {
                    self.retained_partial_plans.extend(partial);
                    return Err((error, self));
                }
            }
        }
        let planned_now = self.plans.is_none();
        if planned_now {
            self.plans = match plan_replay(
                root,
                &self.journal,
                &mut self.planning_files,
                &mut self.planning_ancestries,
            ) {
                Ok(plans) => {
                    self.planning_files.clear();
                    self.planning_ancestries.clear();
                    self.retained_partial_plans.clear();
                    Some(plans)
                }
                Err((error, partial)) => {
                    self.retained_partial_plans.extend(partial);
                    return Err((error, self));
                }
            };
        }
        #[cfg(test)]
        if planned_now {
            run_replay_parent_validation_hook();
        }
        let plans = self.plans.as_mut().expect("replay plans are retained");
        self.attempted = true;
        if let Err(error) = replay(root, lease, &mut self.journal, plans) {
            return Err((error, self));
        }
        for plan in plans.iter() {
            if self.journal.record(plan.registration).is_some()
                && let Err(error) = validate_parent_snapshot(root, plan)
            {
                return Err((error, self));
            }
        }
        let orphans = match recovery_orphans(&self.journal, plans) {
            Ok(orphans) => orphans,
            Err(error) => return Err((error, self)),
        };
        drop(
            self.plans
                .take()
                .expect("completed replay retains its plans"),
        );
        Ok((self.journal, orphans))
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn codec_error() -> io::Error {
    invalid("root recovery control contains an invalid record")
}

fn open_parent(
    root: &platform::RootGuard,
    components: &[RecoveryName],
) -> io::Result<(platform::DirectoryHandle, Vec<RetainedParentBinding>)> {
    let mut parent = platform::clone_root(root)?;
    let mut parent_identity = platform::directory_identity(&parent)?;
    let mut bindings = Vec::with_capacity(components.len());
    for component in components {
        let expected = OsStr::new(component.as_str());
        let parent_revision = platform::directory_revision(&parent)?;
        let listing = platform::entries(&parent, MAX_DIRECTORY_LIST_ENTRIES)?;
        if !listing.complete {
            return Err(invalid("recovery parent enumeration exceeded its bound"));
        }
        let mut matches = listing
            .entries
            .into_iter()
            .filter(|(name, _)| leaf_names_equivalent(name, expected));
        let Some((actual, kind)) = matches.next() else {
            return Err(invalid("recovery parent is absent"));
        };
        if matches.next().is_some() || actual != expected || kind != EntryKind::Directory {
            return Err(invalid("recovery parent is aliased or has the wrong type"));
        }
        let (child, child_identity) = platform::open_directory(&parent, expected)?;
        if platform::directory_revision(&parent)? != parent_revision {
            return Err(identity_changed(
                "recovery ancestor changed while being opened",
            ));
        }
        bindings.push(RetainedParentBinding {
            parent,
            parent_identity,
            parent_revision,
            name: component.clone(),
            child_identity,
        });
        parent = child;
        parent_identity = child_identity;
    }
    Ok((parent, bindings))
}

fn validate_parent_chain(root: &platform::RootGuard, plan: &ReplayPlan) -> io::Result<()> {
    platform::validate_root(root)?;
    let current_root = platform::clone_root(root)?;
    let root_identity = platform::directory_identity(&current_root)?;
    if let Some(first) = plan.parent_bindings.first()
        && (first.parent_identity != root_identity
            || platform::directory_identity(&first.parent)? != root_identity)
    {
        return Err(identity_changed("recovery root handle changed"));
    }
    for (index, binding) in plan.parent_bindings.iter().enumerate() {
        if platform::directory_identity(&binding.parent)? != binding.parent_identity {
            return Err(identity_changed("recovery parent handle changed identity"));
        }
        if platform::directory_revision(&binding.parent)? != binding.parent_revision {
            return Err(identity_changed("recovery ancestor directory changed"));
        }
        let retained_child = plan
            .parent_bindings
            .get(index + 1)
            .map(|next| &next.parent)
            .unwrap_or(&plan.parent);
        if platform::directory_identity(retained_child)? != binding.child_identity {
            return Err(identity_changed("recovery retained parent chain changed"));
        }
    }
    if platform::directory_identity(&plan.parent)?
        != plan
            .parent_bindings
            .last()
            .map_or(root_identity, |binding| binding.child_identity)
    {
        return Err(identity_changed(
            "recovery destination parent changed identity",
        ));
    }
    Ok(())
}

fn validate_parent_snapshot(root: &platform::RootGuard, plan: &ReplayPlan) -> io::Result<()> {
    validate_parent_chain(root, plan)?;
    if platform::directory_revision(&plan.parent)? != plan.parent_revision {
        return Err(identity_changed(
            "recovery destination directory changed after planning",
        ));
    }
    validate_plan_bindings(plan)?;
    if platform::directory_revision(&plan.parent)? != plan.parent_revision {
        return Err(identity_changed(
            "recovery destination directory changed during validation",
        ));
    }
    validate_parent_chain(root, plan)
}

fn validate_plan_bindings(plan: &ReplayPlan) -> io::Result<()> {
    let stage = recovery_stage_leaf(plan.record.operation_id);
    let park = recovery_park_leaf(plan.record.operation_id);
    for (name, observed) in [
        Some((&stage, &plan.stage)),
        plan.record
            .phase
            .owns_target()
            .then_some((&plan.record.destination_leaf, &plan.target)),
        recovery_owns_park(&plan.record).then_some((&park, &plan.park)),
    ]
    .into_iter()
    .flatten()
    {
        let ObservedEntry::File(file) = observed else {
            continue;
        };
        let name = OsStr::new(name.as_str());
        if platform::file_identity(&file.handle)? != file.identity
            || platform::file_receipt_fields(&file.handle)? != file.receipt
            || platform::file_binding_state(&plan.parent, name, file.identity)?
                != BindingState::Exact
        {
            return Err(identity_changed(
                "recovery carrier changed after proof admission",
            ));
        }
    }
    Ok(())
}

fn refresh_parent_snapshot(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    validate_parent_chain(root, plan)?;
    let revision = platform::directory_revision(&plan.parent)?;
    validate_plan_topology(plan)?;
    if platform::directory_revision(&plan.parent)? != revision {
        return Err(identity_changed(
            "recovery destination directory changed during refresh",
        ));
    }
    plan.parent_revision = revision;
    validate_parent_chain(root, plan)
}

fn validate_plan_topology(plan: &ReplayPlan) -> io::Result<()> {
    let listing = platform::entries(&plan.parent, MAX_DIRECTORY_LIST_ENTRIES)?;
    if !listing.complete {
        return Err(invalid("recovery leaf revalidation exceeded its bound"));
    }
    let stage = recovery_stage_leaf(plan.record.operation_id);
    let park = recovery_park_leaf(plan.record.operation_id);
    for (expected, observed) in [
        Some((&stage, &plan.stage)),
        plan.record
            .phase
            .owns_target()
            .then_some((&plan.record.destination_leaf, &plan.target)),
        recovery_owns_park(&plan.record).then_some((&park, &plan.park)),
    ]
    .into_iter()
    .flatten()
    {
        let expected = OsStr::new(expected.as_str());
        let mut matches = listing
            .entries
            .iter()
            .filter(|(candidate, _)| leaf_names_equivalent(candidate, expected));
        let matched = matches.next();
        if matches.next().is_some() || matched.is_some_and(|(actual, _)| actual != expected) {
            return Err(invalid("recovery leaf acquired a portable alias"));
        }
        match (observed, matched) {
            (ObservedEntry::Absent, None) => {}
            (ObservedEntry::File(file), Some((_, EntryKind::File)))
                if platform::file_identity(&file.handle)? == file.identity
                    && platform::file_receipt_fields(&file.handle)? == file.receipt
                    && platform::file_binding_state(&plan.parent, expected, file.identity)?
                        == BindingState::Exact => {}
            (ObservedEntry::Unowned | ObservedEntry::UnownedOccupied, _) => {}
            _ => return Err(identity_changed("recovery leaf topology changed")),
        }
    }
    Ok(())
}

pub(crate) fn prove_file(
    parent: &platform::DirectoryHandle,
    name: &OsStr,
    file: &File,
    identity: platform::Identity,
) -> io::Result<RecoveryFileProof> {
    prove_file_with_receipt(parent, name, file, identity).map(|(proof, _)| proof)
}

fn prove_file_with_receipt(
    parent: &platform::DirectoryHandle,
    name: &OsStr,
    file: &File,
    identity: platform::Identity,
) -> io::Result<(RecoveryFileProof, (u64, platform::FileStamp))> {
    let before = platform::file_receipt_fields(file)?;
    if before.0 > MAX_RECOVERABLE_FILE_BYTES {
        return Err(invalid("recovery file exceeds its size bound"));
    }
    let mut digest = Sha256::new();
    let mut bytes = [0; 64 * 1024];
    let mut offset = 0;
    while offset < before.0 {
        let length = usize::try_from((before.0 - offset).min(bytes.len() as u64))
            .map_err(|_| invalid("recovery read length overflowed"))?;
        let read = platform::read_at(file, &mut bytes[..length], offset)?;
        if read == 0 {
            return Err(io::ErrorKind::UnexpectedEof.into());
        }
        digest.update(&bytes[..read]);
        #[cfg(test)]
        PROOF_BYTES_READ.with(|total| {
            total.set(
                total
                    .get()
                    .checked_add(read as u64)
                    .expect("recovery proof byte counter overflowed"),
            );
        });
        offset = offset
            .checked_add(read as u64)
            .ok_or_else(|| invalid("recovery read offset overflowed"))?;
    }
    if platform::file_receipt_fields(file)? != before
        || platform::file_identity(file)? != identity
        || platform::file_binding_state(parent, name, identity)? != BindingState::Exact
    {
        return Err(identity_changed("recovery file changed while being proven"));
    }
    Ok((
        RecoveryFileProof {
            size: before.0,
            sha256: digest.finalize().into(),
        },
        before,
    ))
}

fn observe(
    parent: &platform::DirectoryHandle,
    name: &RecoveryName,
    prove: bool,
    recoverable_stage: bool,
    planning_files: &mut Vec<(platform::Identity, File)>,
) -> io::Result<ObservedEntry> {
    let expected = OsStr::new(name.as_str());
    let listing = platform::entries(parent, MAX_DIRECTORY_LIST_ENTRIES)?;
    if !listing.complete {
        return Err(invalid("recovery leaf enumeration exceeded its bound"));
    }
    let mut matches = listing
        .entries
        .into_iter()
        .filter(|(candidate, _)| leaf_names_equivalent(candidate, expected));
    let Some((actual, kind)) = matches.next() else {
        return Ok(ObservedEntry::Absent);
    };
    if matches.next().is_some() || actual != expected || kind != EntryKind::File {
        return Err(invalid("recovery leaf is aliased or has the wrong type"));
    }
    let mut handle = platform::open_file(parent, expected)?;
    let identity = platform::file_identity(&handle)?;
    if platform::file_binding_state(parent, expected, identity)? != BindingState::Exact {
        return Err(identity_changed("recovery file changed binding"));
    }
    if recoverable_stage {
        drop(handle);
        handle = match planning_files
            .iter()
            .find(|(retained, _)| *retained == identity)
        {
            Some((_, retained)) => retained.try_clone()?,
            None => platform::open_recoverable_stage(parent, expected, identity)?,
        };
    }
    if !planning_files
        .iter()
        .any(|(retained, _)| *retained == identity)
    {
        planning_files.push((identity, handle.try_clone()?));
    }
    let (proof, receipt) = if prove {
        let (proof, receipt) = prove_file_with_receipt(parent, expected, &handle, identity)?;
        (Some(proof), receipt)
    } else {
        let receipt = platform::file_receipt_fields(&handle)?;
        let (size, _) = receipt;
        if size > MAX_RECOVERABLE_FILE_BYTES {
            return Err(invalid("recovery file exceeds its size bound"));
        }
        (None, receipt)
    };
    Ok(ObservedEntry::File(ObservedFile {
        handle,
        identity,
        receipt,
        proof,
    }))
}

fn observe_unowned_coordinate(
    parent: &platform::DirectoryHandle,
    name: &RecoveryName,
) -> io::Result<ObservedEntry> {
    let expected = OsStr::new(name.as_str());
    let listing = platform::entries(parent, MAX_DIRECTORY_LIST_ENTRIES)?;
    if !listing.complete {
        return Err(invalid("recovery leaf enumeration exceeded its bound"));
    }
    if listing
        .entries
        .iter()
        .any(|(candidate, _)| leaf_names_equivalent(candidate, expected))
    {
        Ok(ObservedEntry::UnownedOccupied)
    } else {
        Ok(ObservedEntry::Absent)
    }
}

fn present(entry: &ObservedEntry) -> bool {
    matches!(
        entry,
        ObservedEntry::File(_) | ObservedEntry::UnownedOccupied
    )
}

fn absent(entry: &ObservedEntry) -> bool {
    matches!(entry, ObservedEntry::Absent)
}

fn matches_proof(entry: &ObservedEntry, proof: Option<RecoveryFileProof>) -> bool {
    matches!(entry, ObservedEntry::File(file) if file.proof == proof)
}

fn prove_observed(
    parent: &platform::DirectoryHandle,
    name: &RecoveryName,
    entry: &mut ObservedEntry,
) -> io::Result<()> {
    if let ObservedEntry::File(file) = entry {
        file.proof = Some(prove_file(
            parent,
            OsStr::new(name.as_str()),
            &file.handle,
            file.identity,
        )?);
    }
    Ok(())
}

fn require_proof(entry: &ObservedEntry, expected: RecoveryFileProof) -> io::Result<()> {
    match entry {
        ObservedEntry::File(file) if file.proof == Some(expected) => Ok(()),
        ObservedEntry::File(_) => Err(identity_changed("recovery content proof changed")),
        ObservedEntry::Absent | ObservedEntry::Unowned | ObservedEntry::UnownedOccupied => {
            Err(invalid("recovery payload is absent"))
        }
    }
}

fn refresh_namespace_receipt(file: &mut ObservedFile) -> io::Result<()> {
    let receipt = platform::file_receipt_fields(&file.handle)?;
    if receipt.0 != file.receipt.0
        || !platform::file_content_stamp_matches(file.receipt.1, receipt.1)
    {
        return Err(identity_changed(
            "recovery carrier content changed during namespace mutation",
        ));
    }
    file.receipt = receipt;
    Ok(())
}

fn revalidate(
    parent: &platform::DirectoryHandle,
    name: &RecoveryName,
    file: &ObservedFile,
    expected: Option<RecoveryFileProof>,
) -> io::Result<()> {
    let name = OsStr::new(name.as_str());
    if platform::file_identity(&file.handle)? != file.identity
        || platform::file_receipt_fields(&file.handle)? != file.receipt
        || platform::file_binding_state(parent, name, file.identity)? != BindingState::Exact
    {
        return Err(identity_changed("recovery payload changed binding"));
    }
    if let Some(expected) = expected
        && file.proof != Some(expected)
    {
        return Err(identity_changed(
            "recovery payload changed after validation",
        ));
    }
    Ok(())
}

fn plan_replay(
    root: &platform::RootGuard,
    journal: &RecoveryJournal,
    planning_files: &mut Vec<(platform::Identity, File)>,
    planning_ancestries: &mut Vec<PlanningAncestry>,
) -> Result<Vec<ReplayPlan>, (io::Error, Vec<ReplayPlan>)> {
    let mut identities = HashSet::new();
    let mut coordinates: Vec<(platform::Identity, RecoveryName)> = Vec::new();
    let records = journal.records();
    let mut plans = Vec::with_capacity(records.size_hint().0);
    let result = (|| -> io::Result<()> {
        for (registration, record) in records {
            let record = record.clone();
            let (parent, parent_bindings) = open_parent(root, &record.destination_parent)?;
            planning_ancestries.push(PlanningAncestry {
                parent,
                bindings: parent_bindings,
            });
            let ancestry = planning_ancestries
                .last()
                .expect("current replay ancestry is retained");
            let parent = &ancestry.parent;
            let parent_identity = platform::directory_identity(&parent)?;
            let parent_revision = platform::directory_revision(&parent)?;
            let stage_name = recovery_stage_leaf(record.operation_id);
            let park_name = recovery_park_leaf(record.operation_id);
            for name in [
                Some(&stage_name),
                recovery_owns_park(&record).then_some(&park_name),
                record
                    .phase
                    .owns_target()
                    .then_some(&record.destination_leaf),
            ]
            .into_iter()
            .flatten()
            {
                if coordinates.iter().any(|(physical, prior)| {
                    *physical == parent_identity
                        && leaf_names_equivalent(
                            OsStr::new(prior.as_str()),
                            OsStr::new(name.as_str()),
                        )
                }) {
                    return Err(invalid("recovery records overlap one physical footprint"));
                }
                coordinates.push((parent_identity, name.clone()));
            }
            let prove_stage = !matches!(record.phase, RecoveryPhase::StagePrepared);
            let replacing = record.old.is_some();
            let owns_park = recovery_owns_park(&record);
            let mut stage = observe(
                &parent,
                &stage_name,
                prove_stage && !replacing,
                true,
                planning_files,
            )?;
            let mut target = if !record.phase.owns_target() {
                ObservedEntry::Unowned
            } else if replacing {
                observe(
                    &parent,
                    &record.destination_leaf,
                    false,
                    true,
                    planning_files,
                )?
            } else if record.phase == RecoveryPhase::StageSealed
                || record.phase == RecoveryPhase::PublishPrepared && present(&stage)
            {
                observe_unowned_coordinate(&parent, &record.destination_leaf)?
            } else {
                observe(
                    &parent,
                    &record.destination_leaf,
                    true,
                    false,
                    planning_files,
                )?
            };
            let mut park = if owns_park {
                observe(&parent, &park_name, false, true, planning_files)?
            } else {
                ObservedEntry::Unowned
            };
            if replacing {
                if [&stage, &target, &park]
                    .into_iter()
                    .filter(|entry| matches!(entry, ObservedEntry::File(_)))
                    .count()
                    > 2
                {
                    return Err(invalid(
                        "replacement recovery retains more than two physical carriers",
                    ));
                }
                if prove_stage {
                    prove_observed(&parent, &stage_name, &mut stage)?;
                }
                prove_observed(&parent, &record.destination_leaf, &mut target)?;
                prove_observed(&parent, &park_name, &mut park)?;
            }
            for entry in [&stage, &target, &park] {
                if let ObservedEntry::File(file) = entry
                    && !identities.insert(file.identity)
                {
                    return Err(invalid("recovery records alias one physical file"));
                }
            }
            if platform::directory_revision(&parent)? != parent_revision {
                return Err(identity_changed(
                    "recovery destination directory changed while being planned",
                ));
            }
            match record.phase {
                RecoveryPhase::StagePrepared => {
                    if owns_park && !absent(&park) {
                        return Err(invalid("prepared replacement park is occupied"));
                    }
                }
                RecoveryPhase::StageSealed => {
                    require_proof(&stage, record.new.ok_or_else(codec_error)?)?;
                    if owns_park && !absent(&park) {
                        return Err(invalid("sealed replacement park is occupied"));
                    }
                }
                RecoveryPhase::ReplacePrepared => {
                    let stage_new = matches_proof(&stage, record.new);
                    let target_old = matches_proof(&target, record.old);
                    let park_old = matches_proof(&park, record.old);
                    if !(stage_new && target_old && absent(&park)
                        || stage_new && absent(&target) && park_old
                        || stage_new && absent(&target) && absent(&park)
                        || stage_new && present(&target) && absent(&park)
                        || absent(&stage) && target_old && absent(&park)
                        || absent(&stage) && absent(&target) && park_old)
                    {
                        return Err(invalid("replacement preparation topology is invalid"));
                    }
                }
                RecoveryPhase::PublishPrepared if replacing => {
                    let stage_new = matches_proof(&stage, record.new);
                    let target_new = matches_proof(&target, record.new);
                    let target_old = matches_proof(&target, record.old);
                    let park_old = matches_proof(&park, record.old);
                    if !(stage_new && absent(&target) && park_old
                        || absent(&stage) && target_new && park_old
                        || absent(&stage) && target_new && absent(&park)
                        || stage_new && target_old && absent(&park)
                        || absent(&stage) && target_old && absent(&park)
                        || absent(&stage) && absent(&target) && park_old)
                    {
                        return Err(invalid("replacement publication topology is invalid"));
                    }
                }
                RecoveryPhase::PublishPrepared => match (&stage, &target) {
                    (
                        ObservedEntry::File(_),
                        ObservedEntry::Absent | ObservedEntry::UnownedOccupied,
                    ) => {
                        require_proof(&stage, record.new.ok_or_else(codec_error)?)?;
                    }
                    (ObservedEntry::Absent, ObservedEntry::File(_)) => {
                        require_proof(&target, record.new.ok_or_else(codec_error)?)?;
                    }
                    _ => return Err(invalid("recovery publication carrier is not linear")),
                },
                RecoveryPhase::RemovePrepared => {
                    if present(&stage) {
                        require_proof(&stage, record.new.ok_or_else(codec_error)?)?;
                    }
                }
                RecoveryPhase::RemoveCommitted if replacing => {
                    if !absent(&stage)
                        || !matches_proof(&target, record.new)
                        || !(absent(&park) || matches_proof(&park, record.old))
                    {
                        return Err(invalid("committed replacement topology is invalid"));
                    }
                }
                RecoveryPhase::RemoveCommitted => {
                    if present(&stage) || !present(&target) {
                        return Err(invalid("committed recovery topology is invalid"));
                    }
                    require_proof(&target, record.new.ok_or_else(codec_error)?)?;
                }
            }
            let ancestry = planning_ancestries
                .pop()
                .expect("completed replay ancestry is retained");
            plans.push(ReplayPlan {
                registration,
                record,
                parent: ancestry.parent,
                parent_bindings: ancestry.bindings,
                parent_revision,
                stage,
                target,
                park,
                publication: None,
            });
        }
        Ok(())
    })();
    match result {
        Ok(()) => Ok(plans),
        Err(error) => Err((error, plans)),
    }
}

fn remove_stage(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let ObservedEntry::File(stage) = &plan.stage else {
        return Ok(());
    };
    let name = recovery_stage_leaf(plan.record.operation_id);
    revalidate(
        &plan.parent,
        &name,
        stage,
        (plan.record.phase != RecoveryPhase::StagePrepared)
            .then_some(plan.record.new)
            .flatten(),
    )?;
    let mut cleanup = platform::clone_stage_cleanup(
        &plan.parent,
        OsStr::new(name.as_str()),
        &stage.handle,
        stage.identity,
    )?;
    validate_parent_snapshot(root, plan)?;
    let effect = platform::remove_parked_file(
        &plan.parent,
        OsStr::new(name.as_str()),
        &mut cleanup,
        stage.identity,
    );
    match platform::file_binding_state(&plan.parent, OsStr::new(name.as_str()), stage.identity)? {
        BindingState::Absent => plan.stage = ObservedEntry::Absent,
        BindingState::Exact => {
            return Err(effect.err().unwrap_or_else(|| {
                identity_changed("recovery stage removal reported success without effect")
            }));
        }
        BindingState::Occupied => {
            return Err(identity_changed("recovery stage changed during removal"));
        }
    }
    refresh_parent_snapshot(root, plan)
}

fn advance_phase(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &mut ReplayPlan,
    phase: RecoveryPhase,
) -> io::Result<()> {
    let mut intended = plan.record.clone();
    intended.phase = phase;
    validate_parent_snapshot(root, plan)?;
    journal.advance(lease, plan.registration, intended)?;
    plan.record.phase = phase;
    validate_parent_snapshot(root, plan)
}

fn sync_replay_parent(root: &platform::RootGuard, plan: &ReplayPlan) -> io::Result<()> {
    validate_parent_snapshot(root, plan)?;
    #[cfg(unix)]
    platform::sync_publication_directory(&plan.parent)?;
    validate_parent_snapshot(root, plan)
}

#[cfg(unix)]
fn clear_replay_record(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &ReplayPlan,
) -> io::Result<()> {
    validate_parent_snapshot(root, plan)?;
    journal.clear(lease, plan.registration)?;
    validate_parent_snapshot(root, plan)
}

fn park_replacement_target(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let ObservedEntry::File(target) = &plan.target else {
        return Err(invalid("replacement target disappeared before parking"));
    };
    let old = plan.record.old.ok_or_else(codec_error)?;
    let park_name = recovery_park_leaf(plan.record.operation_id);
    revalidate(
        &plan.parent,
        &plan.record.destination_leaf,
        target,
        Some(old),
    )?;
    let cleanup = platform::open_parked_file(
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
        target.identity,
    )?;
    validate_parent_snapshot(root, plan)?;
    let effect = platform::park_file_no_replace(
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
        &target.handle,
        target.identity,
        OsStr::new(park_name.as_str()),
        &cleanup,
    );
    let target_binding = platform::file_binding_state(
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
        target.identity,
    )?;
    let park_binding = platform::file_binding_state(
        &plan.parent,
        OsStr::new(park_name.as_str()),
        target.identity,
    )?;
    match (target_binding, park_binding) {
        (BindingState::Absent, BindingState::Exact) => {}
        (BindingState::Exact, BindingState::Absent) => {
            return Err(match effect {
                Err(
                    platform::ParkFileError::NoEffect(error)
                    | platform::ParkFileError::AppliedUnverified(error),
                ) => error,
                Ok(()) => identity_changed("replacement park reported success without effect"),
            });
        }
        _ => {
            return Err(identity_changed(
                "replacement target changed while being parked",
            ));
        }
    }
    if target.proof != Some(old) {
        return Err(identity_changed(
            "replacement target proof changed while parking",
        ));
    }
    let ObservedEntry::File(target) = &mut plan.target else {
        unreachable!("replacement target remains observed until it is parked");
    };
    refresh_namespace_receipt(target)?;
    plan.park = std::mem::replace(&mut plan.target, ObservedEntry::Absent);
    refresh_parent_snapshot(root, plan)?;
    sync_replay_parent(root, plan)?;
    Ok(())
}

fn restore_replacement_park(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let ObservedEntry::File(park) = &plan.park else {
        return Err(invalid("replacement park disappeared before restoration"));
    };
    let old = plan.record.old.ok_or_else(codec_error)?;
    let park_name = recovery_park_leaf(plan.record.operation_id);
    revalidate(&plan.parent, &park_name, park, Some(old))?;
    let mut cleanup =
        platform::open_parked_file(&plan.parent, OsStr::new(park_name.as_str()), park.identity)?;
    validate_parent_snapshot(root, plan)?;
    let effect = platform::restore_parked_file(
        &plan.parent,
        OsStr::new(park_name.as_str()),
        &mut cleanup,
        park.identity,
        OsStr::new(plan.record.destination_leaf.as_str()),
    );
    let park_binding =
        platform::file_binding_state(&plan.parent, OsStr::new(park_name.as_str()), park.identity)?;
    let target_binding = platform::file_binding_state(
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
        park.identity,
    )?;
    match (park_binding, target_binding) {
        (BindingState::Absent, BindingState::Exact) => {}
        (BindingState::Exact, BindingState::Absent) => {
            return Err(effect.err().unwrap_or_else(|| {
                identity_changed("replacement restoration reported success without effect")
            }));
        }
        _ => {
            return Err(identity_changed(
                "restored replacement target changed content",
            ));
        }
    }
    if park.proof != Some(old) {
        return Err(identity_changed("restored replacement proof changed"));
    }
    let ObservedEntry::File(park) = &mut plan.park else {
        unreachable!("replacement park remains observed until it is restored");
    };
    refresh_namespace_receipt(park)?;
    plan.target = std::mem::replace(&mut plan.park, ObservedEntry::Absent);
    refresh_parent_snapshot(root, plan)?;
    sync_replay_parent(root, plan)?;
    Ok(())
}

fn remove_replacement_park(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let ObservedEntry::File(park) = &plan.park else {
        return Err(invalid("replacement park disappeared before removal"));
    };
    let park_name = recovery_park_leaf(plan.record.operation_id);
    revalidate(&plan.parent, &park_name, park, plan.record.old)?;
    let mut cleanup =
        platform::open_parked_file(&plan.parent, OsStr::new(park_name.as_str()), park.identity)?;
    validate_parent_snapshot(root, plan)?;
    let effect = platform::remove_parked_file(
        &plan.parent,
        OsStr::new(park_name.as_str()),
        &mut cleanup,
        park.identity,
    );
    match platform::file_binding_state(&plan.parent, OsStr::new(park_name.as_str()), park.identity)?
    {
        BindingState::Absent => {}
        BindingState::Exact => {
            return Err(effect.err().unwrap_or_else(|| {
                identity_changed("replacement park removal reported success without effect")
            }));
        }
        BindingState::Occupied => {
            return Err(identity_changed("replacement park changed during removal"));
        }
    }
    plan.park = ObservedEntry::Absent;
    refresh_parent_snapshot(root, plan)?;
    sync_replay_parent(root, plan)?;
    Ok(())
}

fn publish_replacement_stage(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let ObservedEntry::File(stage) = &plan.stage else {
        return Err(invalid("replacement stage disappeared before publication"));
    };
    if !absent(&plan.target) {
        return Err(invalid("replacement target is occupied before publication"));
    }
    let stage_name = recovery_stage_leaf(plan.record.operation_id);
    revalidate(&plan.parent, &stage_name, stage, plan.record.new)?;
    let (size, stamp) = platform::file_receipt_fields(&stage.handle)?;
    let attempt = u64::from_le_bytes(plan.record.operation_id[..8].try_into().unwrap()).max(1);
    let mut receipt = platform::prepare_publication(
        attempt,
        &stage.handle,
        size,
        stamp,
        &plan.parent,
        OsStr::new(stage_name.as_str()),
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
    )?;
    validate_parent_snapshot(root, plan)?;
    let effect = platform::rename_no_replace(
        &mut receipt,
        attempt,
        &plan.parent,
        OsStr::new(stage_name.as_str()),
        &stage.handle,
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
    );
    let stage_binding = platform::file_binding_state(
        &plan.parent,
        OsStr::new(stage_name.as_str()),
        stage.identity,
    )?;
    let target_binding = platform::file_binding_state(
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
        stage.identity,
    )?;
    match (stage_binding, target_binding) {
        (BindingState::Absent, BindingState::Exact) => {}
        (BindingState::Exact, BindingState::Absent) => {
            return Err(effect.err().unwrap_or_else(|| {
                identity_changed("recovery publication reported success without effect")
            }));
        }
        _ => {
            return Err(identity_changed(
                "recovery publication topology is indeterminate",
            ));
        }
    }
    let ObservedEntry::File(stage) = &mut plan.stage else {
        unreachable!("replacement stage remains observed until it is published");
    };
    refresh_namespace_receipt(stage)?;
    plan.target = std::mem::replace(&mut plan.stage, ObservedEntry::Absent);
    plan.publication = Some(receipt);
    refresh_parent_snapshot(root, plan)?;
    settle_pending_publication(root, plan)
}

fn settle_pending_publication(root: &platform::RootGuard, plan: &mut ReplayPlan) -> io::Result<()> {
    let Some(receipt) = plan.publication.as_mut() else {
        return Ok(());
    };
    let ObservedEntry::File(target) = &plan.target else {
        return Err(invalid("recovery publication lost its target carrier"));
    };
    let stage_name = recovery_stage_leaf(plan.record.operation_id);
    let attempt = u64::from_le_bytes(plan.record.operation_id[..8].try_into().unwrap()).max(1);
    validate_parent_snapshot(root, plan)?;
    platform::settle_publication(
        receipt,
        attempt,
        &target.handle,
        &plan.parent,
        OsStr::new(stage_name.as_str()),
        &plan.parent,
        OsStr::new(plan.record.destination_leaf.as_str()),
    )?;
    plan.publication = None;
    validate_parent_snapshot(root, plan)
}

fn replay_publication(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &mut ReplayPlan,
) -> io::Result<()> {
    settle_pending_publication(root, plan)?;
    if plan.record.phase == RecoveryPhase::StageSealed {
        advance_phase(root, lease, journal, plan, RecoveryPhase::PublishPrepared)?;
    }
    match (&plan.stage, &plan.target) {
        (ObservedEntry::File(_), ObservedEntry::Absent) => {
            publish_replacement_stage(root, plan)?;
        }
        (ObservedEntry::Absent, ObservedEntry::File(target)) => {
            revalidate(
                &plan.parent,
                &plan.record.destination_leaf,
                target,
                plan.record.new,
            )?;
            #[cfg(unix)]
            validate_parent_snapshot(root, plan)?;
            #[cfg(unix)]
            platform::sync_publication_directory(&plan.parent)?;
        }
        _ => return Err(invalid("recovery publication topology changed")),
    }
    validate_parent_snapshot(root, plan)?;
    #[cfg(unix)]
    clear_replay_record(root, lease, journal, plan)?;
    #[cfg(windows)]
    {
        advance_phase(root, lease, journal, plan, RecoveryPhase::RemoveCommitted)?;
    }
    Ok(())
}

fn replay_removal(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &mut ReplayPlan,
) -> io::Result<()> {
    if plan.record.phase != RecoveryPhase::RemovePrepared {
        advance_phase(root, lease, journal, plan, RecoveryPhase::RemovePrepared)?;
    }
    remove_stage(root, plan)?;
    plan.stage = ObservedEntry::Absent;
    #[cfg(unix)]
    {
        sync_replay_parent(root, plan)?;
        clear_replay_record(root, lease, journal, plan)?;
    }
    Ok(())
}

fn retire_replacement_no_effect(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &mut ReplayPlan,
) -> io::Result<()> {
    if plan.record.phase != RecoveryPhase::RemovePrepared {
        advance_phase(root, lease, journal, plan, RecoveryPhase::RemovePrepared)?;
    }
    #[cfg(unix)]
    {
        sync_replay_parent(root, plan)?;
        clear_replay_record(root, lease, journal, plan)?;
    }
    Ok(())
}

fn replay_replacement(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plan: &mut ReplayPlan,
) -> io::Result<()> {
    loop {
        settle_pending_publication(root, plan)?;
        let stage_new = matches_proof(&plan.stage, plan.record.new);
        let target_old = matches_proof(&plan.target, plan.record.old);
        let target_new = matches_proof(&plan.target, plan.record.new);
        let park_old = matches_proof(&plan.park, plan.record.old);
        match plan.record.phase {
            RecoveryPhase::StagePrepared => {
                remove_stage(root, plan)?;
                plan.stage = ObservedEntry::Absent;
                #[cfg(unix)]
                {
                    sync_replay_parent(root, plan)?;
                    clear_replay_record(root, lease, journal, plan)?;
                }
                return Ok(());
            }
            RecoveryPhase::StageSealed if target_old => {
                advance_phase(root, lease, journal, plan, RecoveryPhase::ReplacePrepared)?;
            }
            RecoveryPhase::StageSealed => {
                replay_removal(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::ReplacePrepared if stage_new && target_old && absent(&plan.park) => {
                park_replacement_target(root, plan)?;
            }
            RecoveryPhase::ReplacePrepared if stage_new && absent(&plan.target) && park_old => {
                advance_phase(root, lease, journal, plan, RecoveryPhase::PublishPrepared)?;
            }
            RecoveryPhase::ReplacePrepared if stage_new && absent(&plan.park) => {
                replay_removal(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::ReplacePrepared if absent(&plan.stage) && target_old => {
                retire_replacement_no_effect(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::ReplacePrepared
                if absent(&plan.stage) && absent(&plan.target) && park_old =>
            {
                restore_replacement_park(root, plan)?;
            }
            RecoveryPhase::PublishPrepared if stage_new && absent(&plan.target) && park_old => {
                publish_replacement_stage(root, plan)?;
            }
            RecoveryPhase::PublishPrepared if absent(&plan.stage) && target_new && park_old => {
                remove_replacement_park(root, plan)?;
            }
            RecoveryPhase::PublishPrepared
                if absent(&plan.stage) && target_new && absent(&plan.park) =>
            {
                advance_phase(root, lease, journal, plan, RecoveryPhase::RemoveCommitted)?;
            }
            RecoveryPhase::PublishPrepared if stage_new && target_old && absent(&plan.park) => {
                replay_removal(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::PublishPrepared if absent(&plan.stage) && target_old => {
                retire_replacement_no_effect(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::PublishPrepared
                if absent(&plan.stage) && absent(&plan.target) && park_old =>
            {
                restore_replacement_park(root, plan)?;
            }
            RecoveryPhase::RemovePrepared => {
                replay_removal(root, lease, journal, plan)?;
                return Ok(());
            }
            RecoveryPhase::RemoveCommitted if park_old => {
                remove_replacement_park(root, plan)?;
            }
            RecoveryPhase::RemoveCommitted => {
                #[cfg(unix)]
                {
                    sync_replay_parent(root, plan)?;
                    clear_replay_record(root, lease, journal, plan)?;
                }
                return Ok(());
            }
            _ => return Err(invalid("replacement replay topology changed")),
        }
    }
}

fn replay(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
    journal: &mut RecoveryJournal,
    plans: &mut [ReplayPlan],
) -> io::Result<()> {
    for plan in plans {
        if plan.record.old.is_some() {
            replay_replacement(root, lease, journal, plan)?;
            continue;
        }
        match plan.record.phase {
            RecoveryPhase::StagePrepared => {
                remove_stage(root, plan)?;
                plan.stage = ObservedEntry::Absent;
                #[cfg(unix)]
                {
                    sync_replay_parent(root, plan)?;
                    clear_replay_record(root, lease, journal, plan)?;
                }
            }
            RecoveryPhase::StageSealed => {
                if present(&plan.target) {
                    replay_removal(root, lease, journal, plan)?;
                } else {
                    replay_publication(root, lease, journal, plan)?;
                }
            }
            RecoveryPhase::PublishPrepared => {
                if matches!(&plan.stage, ObservedEntry::File(_)) && present(&plan.target) {
                    replay_removal(root, lease, journal, plan)?;
                } else {
                    replay_publication(root, lease, journal, plan)?;
                }
            }
            RecoveryPhase::RemovePrepared => replay_removal(root, lease, journal, plan)?,
            RecoveryPhase::RemoveCommitted => {
                #[cfg(unix)]
                {
                    sync_replay_parent(root, plan)?;
                    clear_replay_record(root, lease, journal, plan)?;
                }
            }
            RecoveryPhase::ReplacePrepared => return Err(codec_error()),
        }
    }
    Ok(())
}

fn recovery_orphans(
    journal: &RecoveryJournal,
    plans: &[ReplayPlan],
) -> io::Result<Vec<crate::RecoveryOrphan>> {
    plans
        .iter()
        .filter(|plan| journal.record(plan.registration).is_some())
        .map(|plan| {
            let mut ancestors = plan
                .parent_bindings
                .iter()
                .map(|binding| binding.parent_identity)
                .collect::<Vec<_>>();
            let parent_identity = plan.parent_bindings.last().map_or_else(
                || platform::directory_identity(&plan.parent),
                |binding| Ok(binding.child_identity),
            )?;
            ancestors.push(parent_identity);
            Ok(crate::RecoveryOrphan {
                registration: plan.registration,
                ancestors,
                files: [
                    Some(&plan.stage),
                    plan.record.phase.owns_target().then_some(&plan.target),
                    recovery_owns_park(&plan.record).then_some(&plan.park),
                ]
                .into_iter()
                .flatten()
                .filter_map(|entry| match entry {
                    ObservedEntry::File(file) => Some(file.identity),
                    _ => None,
                })
                .collect(),
            })
        })
        .collect()
}

pub(crate) fn resume_replay(
    replay: RecoveryReplay,
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
) -> Result<(RecoveryJournal, Vec<crate::RecoveryOrphan>), (io::Error, RecoveryReplay)> {
    replay.resume(root, lease)
}

pub(crate) fn initialize_and_replay(
    root: &platform::RootGuard,
    lease: &platform::LeaseHandle,
) -> Result<(RecoveryJournal, Vec<crate::RecoveryOrphan>), (io::Error, Option<RecoveryReplay>)> {
    let journal = RecoveryJournal::load(lease).map_err(|error| (error, None))?;
    RecoveryReplay {
        journal,
        plans: None,
        planning_files: Vec::new(),
        planning_ancestries: Vec::new(),
        retained_partial_plans: Vec::new(),
        attempted: false,
    }
    .resume(root, lease)
    .map_err(|(error, replay)| (error, Some(replay)))
}
