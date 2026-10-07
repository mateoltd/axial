# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is the two-file [explicit precommit cancellation](wire-parity-review.md#explicit-precommit-resolution-cancellation) correction against `69991c92`, plus bounded private Kill tracing and completed CI at `51ed73fa`. Authors relinquished source; root owns integration, shared verification and unresolved cause investigation. Independent Standards and Spec reviews clear the frozen source/diagnostic pins; scheduled reinspection finds no additional actionable drift. Root is the only tracked-source editor; the next accepted-worker regression is frozen privately for root review, with no executed RED or production fix yet.

UI, wire contracts, native proof checks, credentials and signing are unchanged. Earlier evidence applies only to its recorded source and admission scope.

## Findings and corrections

- Explicit fallback resolution ignored its existing token; forwarding alone also erased the typed cancellation cause. Two genuine REDs distinguish those defects. The correction directly reuses the existing acquisition-only resolver and maps only its cancellation variant. Original Admission/identity checks and Ready bypass remain; no helper, state owner, namespace/nesting churn or public contract changes are justified.
- The paired fixture's length supports observable live-caller cancellation, positive original Admission, no publication, bounded cleanup and exact root preservation. It adds no parallel decision model. Root reinspection agrees with both review axes: no additional drift or new recurring AGENTS rule is evidenced.
- Earlier [catalog cancellation](wire-parity-review.md#supported-version-catalog-cancellation), [automatic selection](wire-parity-review.md#automatic-loader-selection) and [postcommit cancellation](wire-parity-review.md#postcommit-loader-resolution-cancellation) reuse existing owners, preserve global-manifest seeding and keep publication outside acquisition races. Their source-qualified findings and controls remain in feature evidence.
- [Copy-fixture isolation](wire-parity-review.md#launch-copy-fixture-budget-isolation) retains real global cross-owner limits; [parent-readiness diagnostics](wire-parity-review.md#actual-parent-metadata-readiness) retain bounded restoration and complete DB-family admission. Neither private instrumentation nor passing reruns establish an unexplained failure's cause.
- Repeated checkpoint chronology is kept in feature evidence and Git history, not duplicated here. Earlier detailed findings remain at `git show d0baa6a5:docs/rewrite/results/architecture-review.md`.

## Validation and limits

Current paired control passes after held-shutdown and erased-cancellation REDs. Separate normal API129/eight ignores/45.64s and app847/ten ignores/66.51s pass976 parent checks/18 existing helper ignores, joined0. Desktop composition checks0/11.58s, joined0. Scoped formatting/diff checks and independent parent-summary assertions pass; source pins remain Setup `e58f0fb5`/fixture `856b6d8d`, Cargo.lock unchanged. These are not ordinary resolution HTTP cancellation, native or full-parity acceptance.

The initial combined command fails at45-second missing-runtime Kill acknowledgement: API128/one failure/eight ignores, app skipped. Original log/session/`.tmp6cW24r` remain retained; this is not tree settlement. Reviewed private tracing passes129/eight ignores/46.47s with one complete HTTP200 header acknowledgement2ms after send, at original concurrency/deadlines. Review fixes mutable restoration, wrapper-exit conflation and inherited interruption status before execution. Exact original API sources are restored; targeted diagnostic outputs are removed under the existing exclusive lease/settlement owner. The fresh normal API pass follows cleanup and has zero markers. Nonrecurrence does not diagnose either Kill failure. Empty late PID sets cannot prove absence; no speculative production Stop change is made. Detailed hashes, logs and cleanup limits remain in feature evidence.

Hosted [run37667062402](https://github.com/mateoltd/axial/actions/runs/37667062402) passes both jobs at exact `51ed73fa9cd14db9bb1ac9597d6d1326e8f9f8c6`, with joined watch0 and independent SHA/run/job assertions (`explicit-resolution-ci-{watch.log,current.json,confirmation.log}`). This validates the committed explicit correction, not the private worker regression or installed/full parity. Earlier CI/library/private-probe results retain their pinned scope in [wire parity](wire-parity-review.md) and [integration](integration.md). Linux no-init settlement, Java/worker admission and frontend-watch failures remain unresolved, not repaired by nonrecurrence.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- Root owns Kill acknowledgement diagnosis and general accepted-worker cancellation; the explicit correction is committed and pushed. Source predicts a held first worker-provider fetch can refuse bounded owner shutdown; an actual RED must precede a repair. Pre-admission enqueue lookup alone is not a retained-effect shutdown defect. Native materialization remains a separate boundary.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): busy, partial, interrupted and unavailable-status recovery.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
