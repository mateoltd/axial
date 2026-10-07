# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is [materialization shutdown/reopen](wire-parity-review.md#materialization-shutdown-and-file-backed-reopen) against `859c1ad7`, after the capped hosted acknowledgement campaign. Root owns integration, evidence and serialized verification; the private author relinquishes source before integration. Standards and Spec independently clear final fixture `3aec4462`; the diagnostic route remains `f5447c40`. Product source is unchanged. Five Vanilla source pins retain their recorded evidence in the feature record.

UI, wire contracts, native proof checks, credentials and signing are unchanged. Earlier evidence applies only to its recorded source and admission scope.

## Findings and corrections

- The existing bounded four-event private probe is temporarily integrated at the same test seam to capture CI recurrence. Hooks match the Unix fixture module; production has no recorder. No parallel Stop owner, timeout increase, replay or speculative fix is added. Its runtime helper serves existing consumers, and capture loss is distinct from HTTP/process completion. Remove after diagnosis/resolution; no new AGENTS rule is justified.
- The materialization control reuses existing HTTP/queue/task/file-backed constructors, not a parallel lifecycle model. Review closes retry-masked gate expiry, client teardown and reopened-negative evidence gaps before execution. One concrete default-off provider gate and explicit bounded cleanup remain preferable to a fixture framework. Existing AGENTS rules already cover these findings; no additional lesson is needed.
- Earlier [Vanilla acquisition](wire-parity-review.md#vanilla-metadata-acquisition-cancellation) and [accepted loader acquisition](wire-parity-review.md#accepted-worker-provider-cancellation) retain effects outside cancellation races and remove duplicated resolvers/tree inspection. Their RED/GREEN, source pins and detailed acceptance limits remain feature-owned. Other historical corrections and checkpoint chronology stay in [wire parity](wire-parity-review.md) and Git history.

## Validation and limits

Current materialization control passes1/3.60s and full API130/eight existing helper ignores/46.71s, joined0. Independent assertions confirm parent counts, control selection, original shutdown refusal and a separate local Kill header acknowledgement2ms after send. Final source reviews, scoped formatting and diff checks pass. File-backed reopen is same-process, not process restart or whole-profile preservation; native/installed and broader effects gates remain open.

Diagnostic commit `859c1ad7` hosted run37675584447 completes all three capped attempts successfully with joined watches0 and exact SHA/attempt/job assertions. Successful log tails omit the API trace; recorded delivery success on rerun is not fresh execution. Passing nonrecurrence does not diagnose either Kill failure or certify the later fixture. The unchanged-source/deadline/concurrency campaign is complete, not extended blindly; pushes were deferred until its terminal result. The earlier Vanilla run remains cancelled, not a hosted pass or reproduced failure.

Original local and [hosted45s Kill acknowledgement failures](wire-parity-review.md#accepted-worker-provider-cancellation) and their preservation/cleanup evidence remain retained; they are not later tree-settlement timeouts. Diagnosis feedback-loop admission stays open. Linux no-init settlement, Java/worker admission and frontend-watch failures also remain unresolved, not repaired by nonrecurrence. Earlier counts and detailed chronology retain their pinned scope in feature evidence; no speculative Stop change or deadline increase follows.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- Root owns recurring Kill acknowledgement diagnosis. The new held-client control verifies one postmetadata refusal/joined-success/file-backed-reopen path, not arbitrary interruption or write-failure recovery. Legacy/current Downloads has no active Cancel button; the extra internal endpoint is starting-only, so a held-body active-Cancel success is not a retained parity gate. Pre-admission lookup alone is not a retained-effect shutdown defect.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): busy, partial, interrupted and unavailable-status recovery.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
