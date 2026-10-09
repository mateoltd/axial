# Architecture review

Updated 2026-10-09. Working branch: `main`. Full non-Guardian parity remains active. This record summarizes current decisions; detailed checkpoint evidence and historical findings remain in linked feature records and Git history.

## Scope and ownership

Latest source scope is [matcher admission reuse](performance-ui.md#matcher-admission-reuse) against `bec7a9e3`, following the scheduled four-document/private-helper review. Root remains the only writer and serialized build/integration owner; retained workers are read-only. Independent Standards and Spec reviews clear managed source `cdd9e455`.

Independent Standards and Spec reviews report zero findings in the committed four-document diff. It adds no namespace, folder, state owner, abstraction, configuration, public contract or UI change. Previous native-binding and codec audits do not justify another observation deletion, new parser, guessed memory multiplier or partial heap-admission claim.

## Findings and corrections

- Matcher admission: move one repeated private-helper entry check to its only caller with intervening I/O. The other three callers have just verified the same pin. All original native, revision, missing-file, final and recovery checks remain; no new interface or rule follows. Temporary public-hash counter23-to22 is removed after original RED/GREEN joins. Direct Minecraft and composed API/application controls join0 with2,056 parent passes/22 existing ignores; desktop composition passes. Current optimized Ready/Mods2 and ordinary shutdown pass, but7–8s reads remain slow; no causal speedup or parity closure follows.
- Private diagnostic: reduction to two requests accidentally dropped array-shape validation and deadline checks after JSON parsing and final assertions. Restore those checks in the existing helper, without another framework or production seam. Independent review closes both findings at helper SHA256 `09bd469ced8c2cb53a76e36e1ec6a2094ed36613a4252503a86521a6a948a3b6`. Existing typed-wire and bounded-work rules in AGENTS already cover this; no duplicate rule is added.
- Review record: replace repeated historical chronology with linked feature evidence and current owner handoffs. Historical qualifications remain authoritative at their original checkpoints; compaction does not waive unresolved failures.
- [Ordinary Vanilla capture](wire-parity-review.md#ordinary-vanilla-capture-coverage): three actual callers reuse the bounded recorder and runtime. Preserve the canonical fixture on panic; body-local unwind and surviving workers mean retention is not joined settlement. Keep the temporary probe until diagnosis/resolution, not merely passing controls.
- [Inventory-size controls](performance-ui.md#bounded-inventory-size-controls): reuse ordinary provider/install/create and remove temporary probes after original joins. Preserve the failed1,024-object setup. Its40s installation terminal-event wait is not the45s Kill deadline or a measured list failure. Count, byte size and hash-directory distribution change together; no isolated cause or fast path follows.

## Validation and limits

Corrected private helper syntax checking joins0. Original73010 joins1 after two HTTP200 requests and the exact expected one-row Ready wire assertions: list-body7,656.035ms exceeds the existing1,000ms diagnostic; accepted bytes1,487/read attempts4 (`list-read-reviewed-control.log`). The measured interval excludes JSON parsing; campaign checks include it. This is still a slow-read RED, not an SLA, performance fix, hard heap/transport bound or native/parity proof.

The pre-correction invocation4226 joins1 at7,717.484ms; it does not have the restored shape/deadline guarantees. Sampled PID28146/listener64558 belongs to frozen `2c19f0c4` from `a9e0ac05`, not a newly built current-source binary. Ordinary shutdown subsequently joins0 and separate absence checks pass. Process binding, Mods2 and preservation are separate evidence; this helper does not verify them.

The private-helper review leaves product fixture byte-identical `4f135596`; its correction changes no product/UI/credential/manifests/lockfile. Ordinary capture evidence remains checkpoint-specific in [wire evidence](wire-parity-review.md#ordinary-vanilla-capture-coverage). Hosted success belongs to exact `6f659596`, as recorded in [integration](integration.md), not this source correction.

Prior owner-local simplifications and refused alternatives remain in [performance evidence](performance-ui.md): encoded input admission, concrete codec limits, receipt/lease metadata reuse, guarded hashing, request-owned catalogue evidence, parent sampling, grouped/revision-only validation and the retired prefix experiment. Their guards, capacity limits and recovery obligations remain required. Source clearance or work-count reductions are not responsiveness acceptance.

## Unresolved owner handoffs

- Root: prepared-request attribution and [slow inventory reads](performance-ui.md#bounded-inventory-size-controls), preserving per-file external-binding checks, exact proofs, managed rebind/settlement fences and bounded churn fallback. No cross-request Ready cache, watcher authority or changed UI semantics is approved.
- Root: [encoded/decoded admission](performance-ui.md#recorded-recovery-input-admission), including SQLite internals, decoded strings/slots/parser scratch and aggregate inspection/body heap. Fallible field copies alone do not close physical admission.
- Root: recurring45s Kill acknowledgement diagnosis and eventual capture removal; [processor cleanup](performance-ui.md#processor-cleanup-diagnosis-controls), the [mock progress-stream warning](settings.md#mock-startup-and-generated-chunk-naming) (mock Launch remains unsupported) and unexplained account, Java/worker, Linux no-init and [watch](updates.md#watch-minimization-controls) failures. Later passes do not waive original failures, unknown settlement or retained workspaces.
- [Readiness/wire parity](wire-parity-review.md#remaining-readiness-requirements): remaining loader/provider, generation, cancellation, cached/bulk, runtime-selection and snapshot interference controls.
- [Native acceptance](native-auth.md) and [Finder drop](native-skins.md#native-dragdrop-attempt): actual current-build saved-world entry/save, native drop/preview/Save/reopen, live progress and installed matrix. Missing original launcher exit status and absent visible actions remain unknown, not inferred successes.
- [Accounts](native-auth.md): noninteractive persistence across distinct builds under the authorized stable signing identity. [Updates](updates.md): trusted signed installed update; release authority is not inferred.
- [Linux](linux-package.md#fresh-current-source-build): visible HTTP/media/SSE, audible playback and ordinary Quit/reopen. Exact candidate and SSH access do not establish native acceptance; no host/viewer settings change is authorized.
- [Library lifecycle](library-lifecycle.md), [library UI](library-ui.md), [Forge](forge-loader.md), [Content](pack-files.md), [worlds](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): remaining version, busy/interruption, runtime and recovery requirements retain their feature-record limits. Historical empty-ZIP Forge is not four-source FML acceptance.

This review neither replaces nor resets the full-parity goal. No deploy, publication, legacy-profile or user-installation mutation is performed.
