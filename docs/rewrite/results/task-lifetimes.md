# Task lifetimes and exclusion

Status: implementation ready for serialized integration verification. This report does not claim installed runtime parity.

`core/app/src/tasks/` owns cooperative cancellation, bounded accepted work, retained target/artifact admission, and in-memory revisioned projections. Feature code still owns durable state, process settlement, publication decisions, and domain outcomes.

## Private contracts

- `Exclusions::try_acquire(targets, artifacts)` reserves every target and artifact exclusively. Existing installation writers keep this API.
- `Exclusions::try_acquire_read_artifacts(targets, artifacts)` reserves targets exclusively and artifacts for shared use, returning the same `ExclusionLease` type. Separate instances may share `ArtifactKey::new(library_id, "managed-game-artifacts")`; any retained reader excludes installation mutation of that exact key, and an installer excludes all readers.
- Reservations validate bounded identities, deduplicate keys, check every conflict, and publish the complete reservation under one short mutex. Shared-count overflow refuses admission before mutation. Failed admission cannot retain a target, add a reader, or reserve an unrelated artifact. Admission is nonblocking and does not promise queue fairness.
- An `ExclusionLease` clone retains the existing reservation. Only its final clone releases that reservation's target locks and one artifact-reader permit. Independent readers each own a separate permit. `covers_artifact` proves read-or-write coverage; mutation checks must use `covers_artifact_write`.
- Acquire the admitted library-generation pin first, the complete exclusion lease second, then short metadata/filesystem locks. Keep the pin and lease for the full operation, live session, or unresolved domain receipt; no memory mutex is held during I/O.
- `TaskOwner::try_spawn` accepts work atomically against capacity and closure. Dropping its `TaskHandle` or cancelling a request does not cancel accepted work. Explicit cancellation and shutdown request cooperation; completion releases retained resources before declaring the owner idle.
- `TaskOwner::subscribe()` returns a coalescing `watch::Receiver<()>`. Subscribe before testing capacity or status, await changes after refusal, and retry authoritative admission. A notification does not reserve a free slot for a subscriber.
- Worker panic retains resources and leaves an unsettled record. A supervisor drop caused by runtime interruption now does the same, including work whose supervisor was never polled, and resolves its waiter as `TaskJoinError::Interrupted`. Shutdown cannot issue a receipt while records remain unsettled. Composition must retain the owner while such obligations exist; disposing the final owner is not proof of settlement. Resource destructors must not panic.
- `Projection` publishes a revision and full snapshot under one lock, rejects stale revisions, and permits one terminal publication per incarnation. Subscriptions rebase to the latest snapshot; they do not replay mutations or provide durable recovery.

## Evidence and handoff

There are 25 focused tests covering cancellation before subscription, accepted work surviving dropped waiters, capacity and close admission, shutdown settlement and timeout, worker panic, polled and unpolled runtime interruption, shared readers versus exclusive writers, all-or-none multikey conflicts, clone retention, exact key/owner coverage, invalid and unbounded requests, snapshot rebasing, and stale/terminal revisions. Barrier-coordinated threads exercise simultaneous reader/writer admission, admission versus close, and competing terminal publications. Shared lease cancellation tests retain an escaped receipt after task settlement and verify writer refusal until that receipt drops.

The serialized integration owner should run:

```sh
cargo test -p axial-app tasks:: --lib
```

Direct `rustfmt --edition 2024` completed for the changed task sources. This worker has not run Cargo, builds, or tests, as shared verification belongs to integration. No new dependency, manifest edit, registration, wire export, legacy mutation, or production-profile effect is required. Installation/launch consumer verification must confirm that both use the same `Exclusions` composition, exact library identity and artifact key, and retain guards through real publication or process/output settlement.
