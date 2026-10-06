# Architecture review

Updated 2026-10-06. Current scope: retained `5c9fd5ce` Candidate native probe lifecycle, visual-feedback tooling and preservation evidence. Product source is unchanged. This is not a parity or release certificate.

## Scope and ownership

Root owns generated profiles, native processes/actions, witness execution, documentation and serialized shared verification. Independent owners inspect actual native entrypoints and audit the frozen bounded helper; they do not operate profiles or change product source. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

Existing Launch/status/report, recency and readiness owners suffice. Bind one native Launch to its returned intent/session and direct managed Java child. The before-action witness captures original raw history and protected state; after Stop, reuse the retained serialized-proof verifier and allow only the owning recency/selection changes. Do not introduce another settlement classifier or accept a retrospective baseline.

Correct two helper-only errors: an AX diff is not a full tree, and status belongs to the exact session route, not a guessed instance route. Later accessibility exposes loaded content despite partial screenshots, so missing HTML is not a stable rendering oracle. Neither error supplies a product RED, cause or source fix. No namespace, configuration, state-owner or UI change is justified. Existing AGENTS.md rules cover exact entrypoints, typed ownership and compact evidence; no additional anecdotal rule is needed.

## Validation

[Native probe evidence](native-auth.md#controlled-probe-launch-and-preserved-reopen) records one actual Launch/Playing/Stop, acknowledged boot11676ms and two normal Quit0 exits with Ready restored between them. Original history/instance/files/protected state and3643 recorded game files remain exact; post-Stop/closed/reopened/final captured snapshots are byte-identical. All five bounded collectors pass with empty error logs; independent helper review is clear. All owned game/launcher processes are absent after final Quit.

Screenshots still omit main content despite loaded accessibility. Supported computer use cannot bind the Java game; no first-screen/Continue observation follows. This is not a narrator fix, full painted-UI acceptance, whole-profile physical equality, authentication, gameplay or current-source packaging. No source change warrants another Cargo run. Prior [external installation/root-restoration evidence](library-lifecycle.md#real-provider-external-installation-and-reopen) remains separately scoped.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause; native screenshot/accessibility discrepancy remains unexplained. The completed launcher lifecycle does not expose the game window. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Disk-observation validity remains source-qualified and unreproduced.
- External-library launch/Stop/native switching, remaining failure/interruption matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
