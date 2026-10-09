# Architecture review

Updated 2026-10-09. Working branch: `main`. Full non-Guardian parity remains active. Historical checkpoint qualifications, original joins and retained failures remain authoritative in the linked feature evidence and Git; compaction waives none of them.

## Scope and ownership

Against `38472d49`, root owns [inventory-worker admission and its consumers](performance-ui.md#installed-inventory-decode-admission), all edits and serialized verification. Retained Standards/Spec reviewers are read-only. Use one compiler job and two test threads; stop/join disposable apps and games after each check. No UI, public wire, schema, quota, parser or framework changes.

## Findings and corrections

- Inventory decoding/inspection escaped the existing four-worker limit. Acquire its nonwaiting Read admission after the existing row/encoded checks; release it before returning receipts or later library preparation. No nested worker wait is introduced. Concurrency admission is not a decoded-heap bound or swap diagnosis.
- Setup interpreted scratch/worker capacity refusal as missing installation and selected fallback. Keep this repeated distinction in the install owner. Preserve the published creation outcome and existing warning on postcommit verification refusal. Play reuses its existing typed install-error mapper. Accepted Setup-content execution remains a separate handoff below.
- Consolidate direct readiness and actual Setup/Play pressure controls into one existing-owner fixture. Establish quiescence before dropping dependencies; retain unsettled owners before fallible diagnostics. Merge that recurring lesson into AGENTS' existing retention rule, without another fixture framework.
- [List summaries](performance-ui.md#instance-list-metadata-summaries) own display metadata, not strict launch integrity. Keep typed evidence distinct; retain positive/negative observations and accepted-task ownership through the final sweep. Charge retained capacity and nested owned storage. No cross-request Ready cache or watcher authority follows.
- Earlier owner-local reductions retain independent filesystem proofs, refusal cleanup and exact lifecycle guards. Their detailed decisions and refused alternatives remain in [performance evidence](performance-ui.md). Required safety is not redundant code.
- [Forge](forge-loader.md#genuine-ack-before-ready-queue-recovery) and [Fabric](fabric-loader.md#queued-retry-across-service-reopen) controls use genuine current operation owners. Queue/store recovery is not normal app Quit, process-crash or gameplay acceptance. Canonical IDs, complete request vectors and retained failure evidence remain required.
- [Pending-Kill capture](wire-parity-review.md#pending-request-live-capture) retains bounded, joined observations under its current owner. An absent trigger or later passing campaign does not explain the historical timeout. Match actual package cwd and preserve the original diagnostic.
- User-reported swap prompted [owner-mediated native test cleanup](native-auth.md#current-recovery-checkpoint-native-continuation). Stop games, Quit/join launchers and verify absence; retain unresolved recovery ownership and named-world evidence. Relaunch only when a check is ready.

## Validation and limits

Current readiness/Setup/Play controls reproduce intended failures before correction. Final composed focused57686 joins0 with one parent pass, excluding its nested child. Both source axes clear the five Rust files. Composed85833 and desktop85869 join0 with1,089 selected parent passes/23 existing ignores; exact count/source assertions, scoped formatting and whitespace pass. Full pins/logs are in the [feature record](performance-ui.md#installed-inventory-decode-admission). No test launcher/game/compiler remains. Postcommit creation's pressure branch is source-reviewed, not separately fault-injected.

Frozen `b9e48446` passes exact hosted application/delivery verification and ordinary [native packaging/cold reopen](native-auth.md#metadata-summary-checkpoint-cold-reopen). Native launchers2442/61609 join0 after ordinary Quit, with independent absence and scoped world-preservation checks. No game is launched, so saved-world gameplay/save remains unverified at that checkpoint. Its earlier composed37788 verifies1,006 app/API parent checks with22 existing ignores. Same-profile baseline/candidate/baseline retains exact Ready, with only the candidate passing the1s diagnostic; no SLA, whole-heap or full-parity claim follows.

Earlier source pins, execution scopes, original joins, failed diagnostics and cleanup qualifications remain in their feature records. Current passes do not waive original failures or certify later native builds. No deploy, publication, legacy-profile or user-installation mutation is performed.

## Unresolved owner handoffs

- Root: [inventory admission](performance-ui.md#installed-inventory-decode-admission) still lacks SQLite-internal, decoded-string/slot/parser-scratch and aggregate inspection/body heap bounds. Test accepted Setup-content `run_content` capacity classification through the existing prerequisite-order fixture before correcting its separate mapping. Preserve row/encoded refusal precedence and avoid nested permits.
- Root: list metadata now passes a same-profile diagnostic, but broader native preparation/readiness and strict inspection costs remain unverified. Optimize measured work without dropping exact proofs, external bindings, managed rebind/settlement fences or bounded churn refusal. Inclusive timings are not additive.
- Root: [Forge recovery](forge-loader.md#genuine-ack-before-ready-queue-recovery) needs applicable interruption/cancellation and native gameplay acceptance. Preserve the exact fixture/patch; do not manufacture obligations in the accepted profile or substitute Guardian-purpose rebuild.
- Root: historical45s Kill cause and eventual capture removal remain unresolved. Also retain [processor cleanup](performance-ui.md#processor-cleanup-diagnosis-controls), [mock progress-stream](settings.md#mock-startup-and-generated-chunk-naming), unexplained account/Java-worker/Linux-no-init failures and [watch diagnosis](updates.md#watch-minimization-controls). Later passes, missing PID-only markers and terminal labels are not settlement proof.
- [Readiness/wire parity](wire-parity-review.md#remaining-readiness-requirements): remaining loader/provider, generation, cancellation, cached/bulk, runtime-selection and snapshot interference controls.
- [Native acceptance](native-auth.md) and [Finder drop](native-skins.md#native-dragdrop-attempt): current-build saved-world entry/save, drop/preview/Save/reopen, live progress and installed matrix. Missing original exits or visible actions remain unknown.
- [Accounts](native-auth.md): noninteractive persistence across distinct builds under the authorized stable signing identity. [Updates](updates.md): trusted signed installed update. Release authority is not inferred.
- [Linux](linux-package.md#fresh-current-source-build): visible HTTP/media/SSE, audible playback and ordinary Quit/reopen. Candidate/SSH access is not native acceptance; host/viewer settings changes remain unauthorized.
- [Library lifecycle](library-lifecycle.md), [library UI](library-ui.md), [Forge](forge-loader.md), [Content](pack-files.md), [worlds](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): remaining version, busy/interruption, runtime and recovery requirements retain their feature-record limits. Historical empty-ZIP Forge is not four-source FML acceptance.

This review does not replace or reset the active full-parity goal.
