# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is [accepted-worker provider cancellation](wire-parity-review.md#accepted-worker-provider-cancellation) against `e315b876`, with earlier explicit cancellation, private Kill tracing and CI pinned below. Authors relinquished source; root owns integration, shared verification and unresolved cause investigation. Independent Standards and Spec reviews clear queue `da9f1c08`; root is the only tracked-source editor. A read-only next-boundary audit owns no edits or runtime actions.

UI, wire contracts, native proof checks, credentials and signing are unchanged. Earlier evidence applies only to its recorded source and admission scope.

## Findings and corrections

- An accepted loader worker ignored both existing tokens while acquiring its record. Genuine held-shutdown RED precedes the correction. Target and worker now share one concrete queue-owned resolver; the superseded free function and temporary duplicate adapter are removed. Typed cancellation uses existing durable terminal completion and retains Retry/exclusion if acknowledgement fails. Materialization/publication remain fully awaited; no new coordinator, state, contract or namespace/nesting churn is justified.
- The fixture observes original pins/exclusion, public status and same-owner shutdown before body release, then joins cleanup and checks exact preservation. Its initial size failures are separated from RED; whole-tree comparison includes the owner-defined4MiB lease rather than excluding safety evidence. Queue reconstruction uses the same in-memory metadata, not cold reopen. Both review axes find no additional actionable drift or new recurring AGENTS rule.
- Earlier [explicit cancellation](wire-parity-review.md#explicit-precommit-resolution-cancellation) retains original Admission/identity checks and Ready bypass; two distinct REDs prove missing forwarding and erased typed cause. Its detailed evidence remains feature-owned.
- Earlier [catalog cancellation](wire-parity-review.md#supported-version-catalog-cancellation), [automatic selection](wire-parity-review.md#automatic-loader-selection) and [postcommit cancellation](wire-parity-review.md#postcommit-loader-resolution-cancellation) reuse existing owners, preserve global-manifest seeding and keep publication outside acquisition races. Their source-qualified findings and controls remain in feature evidence.
- [Copy-fixture isolation](wire-parity-review.md#launch-copy-fixture-budget-isolation) retains real global cross-owner limits; [parent-readiness diagnostics](wire-parity-review.md#actual-parent-metadata-readiness) retain bounded restoration and complete DB-family admission. Neither private instrumentation nor passing reruns establish an unexplained failure's cause.
- Repeated checkpoint chronology is kept in feature evidence and Git history, not duplicated here. Earlier detailed findings remain at `git show d0baa6a5:docs/rewrite/results/architecture-review.md`.

## Validation and limits

Current worker control passes1/0.17s after genuine held-shutdown RED2.13s. Fresh normal app848/ten ignores/66.86s and API129/eight ignores/48.98s pass977 parent checks/18 existing helper ignores, joined0. Desktop composition checks0/11.19s, joined0. Scoped formatting/diff checks and independent parent-summary assertions pass; queue remains `da9f1c08`, Cargo.lock unchanged. Active-Cancel admission, acknowledgement-failure injection, native materialization and native/full-parity acceptance are not established.

At the preceding explicit checkpoint, the initial combined command fails at45-second missing-runtime Kill acknowledgement: API128/one failure/eight ignores, app skipped. Original log/session/`.tmp6cW24r` remain retained; this is not tree settlement. Reviewed private tracing passes129/eight ignores/46.47s with one complete HTTP200 header acknowledgement2ms after send, at original concurrency/deadlines. Review fixes mutable restoration, wrapper-exit conflation and inherited interruption status before execution. Exact original API sources are restored; targeted diagnostic outputs are removed under the existing exclusive lease/settlement owner. The fresh normal API pass follows cleanup and has zero markers. Nonrecurrence does not diagnose either Kill failure. Empty late PID sets cannot prove absence; no speculative production Stop change is made. Detailed hashes, logs and cleanup limits remain in feature evidence.

Hosted [run37667062402](https://github.com/mateoltd/axial/actions/runs/37667062402) passes both jobs at exact `51ed73fa9cd14db9bb1ac9597d6d1326e8f9f8c6`, with joined watch0 and independent SHA/run/job assertions (`explicit-resolution-ci-{watch.log,current.json,confirmation.log}`). This validates the preceding explicit correction, not current worker edits or installed/full parity. Earlier CI/library/private-probe results retain their pinned scope in [wire parity](wire-parity-review.md) and [integration](integration.md). Linux no-init settlement, Java/worker admission and frontend-watch failures remain unresolved, not repaired by nonrecurrence.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- Root owns Kill acknowledgement diagnosis and integration of the reviewed worker correction. Pure provider acquisition now has executed RED/GREEN; cancellation during native materialization remains a separate boundary. Pre-admission enqueue lookup alone is not a retained-effect shutdown defect.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): busy, partial, interrupted and unavailable-status recovery.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
