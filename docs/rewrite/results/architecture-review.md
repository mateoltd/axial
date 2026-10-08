# Architecture review

Updated 2026-10-08. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is [post-probe inventory freshness](performance-ui.md#post-probe-inventory-freshness) against `136e7cc6`. Root owns integration, evidence and serialized verification. Both independent review axes clear coordinator `92ce4633`: the original Ready proof is checked after scan/probe awaits in the existing accepted task; failed reuse is invalidated without bypassing remaining refusal fences. A local retained proof preserves runtime-absence validation after that invalidation. The final target check compares current library ID and generation with the captured pin after every await: retained physical authority alone cannot prove current selection. No new cache, authority, state owner, public contract or UI follows.

Earlier [revision-only/batch corrections](performance-ui.md#revision-only-inventory-validation), [staged Discover creation/reopen](pack-files.md#actual-staged-discover-creation-and-cold-reopen), [queued Fabric503](fabric-loader.md#queued-artifact-failure-diagnostics) and hosted checks retain their checkpoint-specific evidence in feature records and [integration](integration.md). They do not certify the current source or close remaining parity gates.

The latest read-only [native reload observation](native-auth.md#current-native-world-reload-setup) finds the launcher absent and its Java game still running, with the original launcher handle unavailable. Exit cause, ordinary closure and gameplay acceptance remain unknown; the game/profile are untouched. Earlier [Accounts](native-auth.md#browser-offline-identity-lifecycle) and [Music](system-music.md#browser-volume-persistence) witnesses qualify only their recorded checkpoints.

## Findings and corrections

- [Generated naming](settings.md#mock-startup-and-generated-chunk-naming) removes redundant `chunks/chunk-`, fixing the reproduced bundle overrun without changing budgets/content-hash ownership. The mock bootstrap adds only its missing idle-session response; unsupported Launch stays501. No speculative module split follows.
- [Settings](settings.md#redacted-java-selection-and-stale-edits) derives Java presentation through the native codec without private authority. Observed stale Reset/held replies drive captured-revision, own-ack and snapshot fences in existing owners. Mock projections use that same redaction boundary. No parallel store, migration or UI redesign follows.
- [Historical Fabric](fabric-loader.md#historical-maven-transport-correction) corrects exact Maven Central transport. Its [tweaker correction](fabric-loader.md#historical-client-tweaker-correction) carries the official first-client selection through existing proof/sealing, not launch-time injection or restrictive catalog filtering. Installation/persistence pass; separate launch limits remain recorded.
- Earlier [Performance](performance-ui.md), [materialization](wire-parity-review.md#runtime-publication-conflicts), [resource](screenshot-files.md) and [Library UI](library-ui.md#separate-reconciliation-survivor-cleanup-and-cold-reopen) corrections retain existing owners. Linked records preserve failed trials, pins, joins and limits; no parallel journal/coordinator/recovery framework follows.
- [Watch controls](updates.md#watch-minimization-controls) retain original owners/deadlines and trace-after-cleanup. A single palette-test cut reproduces missing callbacks in40 files; the29-file campaign's separate port contention remains unclassified. Collectors retain exact failure boundaries and joined cleanup. Shipped watcher remains unchanged, with no cause/fix/minimality claim.
- The earlier [file-batch correction](performance-ui.md#detaillist-samples) retains deepest managed-directory and original namespace/file checks. Its work-budget regression and frozen timings remain at their checkpoint; untouched workspace-format differences are not expanded into style churn.

## Validation and limits

The current-row slice has genuine stale-Ready, combined-refusal and same-library generation REDs; final preflight controls11, directly selected application851/ten existing ignores/64.38s and API132/twelve existing helper ignores/50.48s at four test threads join0 at `92ce4633`. Review fixes a fixture admission panic before release/join. The preceding-source default-parallel API run fails on the recurring Kill acknowledgement; isolated and controlled-concurrency passes do not diagnose or waive that failure. The final frozen optimized witness retains exact Ready/Mods2 and ordinary shutdown/absence; median list23.726s is slower than the preceding16.572s campaign, not an isolated causal estimate or responsiveness pass. Detailed pins, logs and limits belong in the linked feature record; existing AGENTS rules suffice.

[Fabric503](fabric-loader.md#queued-artifact-failure-diagnostics) preserves bounded safe diagnostics through real owners. Unknown-join fixture retention is source-reviewed, not executed fault coverage; one provider refusal does not establish every mapping.

[Staged creation](pack-files.md#actual-staged-discover-creation-and-cold-reopen) retains actual Ready/Mods2 and cold persistence, with initial wrapper failures/incomplete trace qualified. It establishes neither gameplay, native/installed nor responsiveness acceptance.

[Settings](settings.md#redacted-java-selection-and-stale-edits) retains actual stale-edit/cold-persistence evidence; mock process-memory navigation is not native persistence. [Mixed Mods Delete](mod-files.md#actual-mixed-managedlocal-deletion) retains partial refusal, survivor continuation and cold persistence with named preservation. Witness review corrects identity/budgets, not production behavior.

Historical Fabric's primary Java8 tweaker/install succeeds before a separate null GLFW monitor-buffer crash. The later unlocked repeat and normal Stop/cold report persistence qualify only their frozen binary/profile. Boot/Playing and report preservation are not menu/gameplay or current-build acceptance.

Earlier evidence remains authoritative only at its recorded source/scope. Provider503 does not prove every failure mapping. Unexplained account29.46s, local/hosted45s Kill acknowledgement, Linux no-init, Java/worker and watch failures remain open; the `859c1ad7` campaign exposes no hosted API trace and earlier Vanilla remains cancelled. Kill acknowledgement is not later tree-settlement timeout; no deadline increase follows. The test-only Stop probe retains its removal obligation.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk and other runtime publication interference, plus remaining parent/origin/runtime-selection variants. Current controls close only their recorded paths; Vanilla coverage does not close loader or native gates.
- Root owns historical Fabric [gameplay acceptance](fabric-loader.md#unlocked-historical-launch-comparison). One observed first-thread flag weakens the missing-flag hypothesis, not proof of display causation. The separate queued503 correction retains a generic source label, not distinct Maven identity.
- Root owns recurring Kill acknowledgement diagnosis. The held-client control qualifies one postmetadata refusal/join/reopen path, not arbitrary interruption/write failure. Downloads has no active Cancel button; its internal Cancel is starting-only, so held-body cancellation is not a parity gate. Pre-admission lookup alone is not a retained-effect shutdown defect.
- Root owns [grouped final-inventory freshness and optimized latency](performance-ui.md#post-probe-inventory-freshness). Final publication must cover every original earlier-version inventory under aggregate retention/work bounds; the last proof and catalog scan are insufficient. Reuse existing resource/task owners, avoiding incremental scratch-reservation deadlock. The new current-row sweep is not a performance fix; retained optimized observations remain slow and checkpoint-specific.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- Root owns the mock progress-stream warning. Idle fixtures/Settings navigation do not prove live progress or Launch; absent behaviors remain explicit.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): remaining busy, partial, interrupted and unavailable-status recovery; the current preserve-only admission control closes only its recorded path.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Feature records retain historical checkpoints and receipts, authoritative only for their recorded scope. This review neither replaces nor resets the full-parity goal.
