# Architecture review

Updated 2026-10-06. Current scope: unchanged `10befcc8` real-provider external-library acceptance and evidence/documentation quality. This is not a parity or release certificate.

## Scope and ownership

Root owns generated profiles, API processes, documentation and serialized shared verification. Independent owners inspect actual selection entrypoints and review the bounded witness/helper; they do not operate profiles or change product source. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

Existing profile admission, saved external selection, Create, queue, activation and readiness owners suffice. Correct helper preparation before startup: admit an empty profile normally before seeding its selection. Bind observation to the returned instance/install/operation identities instead of an active-operation fallback. Independently verify recorded game files and the separately application-owned runtime; do not invent a parallel readiness classifier or replay an uncertain mutation.

No product architecture, namespace, configuration, state owner or UI change is justified by this passing journey. The review record itself had grown to4,233 words of repeated closed chronology and ownership assignments. Following writing-for-agents guidance, retain one current record, link domain evidence and preserve prior detail in Git. Merge this recurring anti-duplication rule into the existing AGENTS.md bullet rather than adding another section.

## Validation

[External-library evidence](library-lifecycle.md#real-provider-external-installation-and-reopen) records actual Vanilla1.20.1 installation,3630 verified game files and a separate142-file Java17 runtime. Strict Ready passes before shutdown, ordinary reopen and restored-root reopen (03:57:03.056Z). Captured installed/final snapshots are byte-identical; protected metadata, five file canaries and native identities remain exact.

With the selected generated folder absent after a stopped same-filesystem rename, versions/instances refuse503, account/config remain usable, all21 captured table digests match and no Managed fallback appears. Restoration recovers the same root. All five owned APIs join normal SIGINT exit0. No game or credential action occurs. This does not prove whole-database physical equality, live switching, drive ejection, crash recovery or native/installed acceptance. No source change warrants another Cargo run.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause. Native access reports the Mac locked. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Disk-observation validity remains source-qualified and unreproduced.
- External-library launch/Stop/native switching, remaining failure/interruption matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
