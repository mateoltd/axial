# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is [preserve-only external admission](library-lifecycle.md#preserve-only-exit-after-unresolved-external-admission) against `42da27ab`. Root owns integration, evidence and serialized verification; the private author relinquishes source before integration. Standards and Spec independently clear lifecycle `85548376` and API tests `66e97da3`. Earlier [materialization shutdown/reopen](wire-parity-review.md#materialization-shutdown-and-file-backed-reopen), Vanilla and diagnostic source pins retain their recorded evidence in the feature record. The private Java-inheritance handoff is not integrated in this slice.

UI, wire contracts, native proof checks, credentials and signing are unchanged. Earlier evidence applies only to its recorded source and admission scope.

## Findings and corrections

- The existing preservation owner unconditionally refused an unresolved external-root acquisition without attempting its native preserve acknowledgement. Genuine composed startup RED confirms the exit dead end. The minimal correction acknowledges through the retained native owner and restores its exact obligation on refusal; replaced-binding refusal/retry and both lease releases are verified. Destructive guards remain intact. Existing AGENTS rules already require preserve-only exit after destructive refusal; no duplicate rule or new coordinator is needed.
- The temporary bounded four-event Stop probe retains its test-only scope, unchanged deadlines and removal obligation. Capture loss is distinct from HTTP/process completion; no speculative Stop fix or parallel owner follows. Detailed source pins and limits remain in [wire parity](wire-parity-review.md#temporary-hosted-acknowledgement-capture).
- Materialization coverage reuses existing HTTP/queue/task/file-backed owners. Review closes retry-masked gate expiry, client teardown and reopened-negative gaps before execution. One default-off provider gate and bounded cleanup avoid a fixture framework; existing AGENTS rules suffice.
- Earlier [Vanilla acquisition](wire-parity-review.md#vanilla-metadata-acquisition-cancellation) and [accepted loader acquisition](wire-parity-review.md#accepted-worker-provider-cancellation) retain effects outside cancellation races and remove duplicated resolvers/tree inspection. Their RED/GREEN, source pins and detailed acceptance limits remain feature-owned. Other historical corrections and checkpoint chronology stay in [wire parity](wire-parity-review.md) and Git history.

## Validation and limits

Current preservation RED fails at its intended assertion0.05s, joined101; minimal GREEN passes0.08s. Final binding control1/0.11s, lifecycle26/0.99s, native preservation1/0.01s and full API131/eight existing helper ignores/46.73s pass, joined0. Source reviews, scoped formatting and diff checks pass. This is same-process composed startup/preservation, not executable-process/native-dialog or arbitrary interruption acceptance. Detailed fixture retention and preservation evidence remain feature-owned.

Materialization commit `42da27ab` hosted [run37689125297](https://github.com/mateoltd/axial/actions/runs/37689125297), attempt1, completes both jobs successfully with exact SHA/job assertions. The original watch ends on a transport error, not CI failure; its resumed observer joins0. That checkpoint does not validate this later preservation correction. Its same-process file-backed reopen is not cold-process or whole-profile proof.

Diagnostic commit `859c1ad7` hosted run37675584447 completes all three capped attempts successfully with joined watches0 and exact assertions. Tails omit the API trace; recorded rerun delivery success is not fresh execution. This completed nonrecurrence campaign does not diagnose Kill failures or certify later edits. The earlier Vanilla run remains cancelled, not a hosted pass or failure.

Original local and [hosted45s Kill acknowledgement failures](wire-parity-review.md#accepted-worker-provider-cancellation) and their preservation/cleanup evidence remain retained; they are not later tree-settlement timeouts. Diagnosis feedback-loop admission stays open. Linux no-init settlement, Java/worker admission and frontend-watch failures also remain unresolved, not repaired by nonrecurrence. Earlier counts and detailed chronology retain their pinned scope in feature evidence; no speculative Stop change or deadline increase follows.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- Root owns recurring Kill acknowledgement diagnosis. The new held-client control verifies one postmetadata refusal/joined-success/file-backed-reopen path, not arbitrary interruption or write-failure recovery. Legacy/current Downloads has no active Cancel button; the extra internal endpoint is starting-only, so a held-body active-Cancel success is not a retained parity gate. Pre-admission lookup alone is not a retained-effect shutdown defect.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): remaining busy, partial, interrupted and unavailable-status recovery; the current preserve-only admission control closes only its recorded path.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
