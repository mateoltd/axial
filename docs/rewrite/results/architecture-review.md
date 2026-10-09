# Architecture review

Updated 2026-10-09. Working branch: `main`. Full non-Guardian parity remains active. This record summarizes current decisions; detailed checkpoint evidence and historical findings remain in linked feature records and Git history.

## Scope and ownership

Latest source scope is [absolute directory handle ownership](performance-ui.md#absolute-directory-handle-ownership) against `b4312584`. Root remains the only writer and serialized build/integration owner; retained workers are read-only. Independent Standards and Spec reviews clear all five filesystem source pins, led by `2bdb286d/fa16b317`.

Earlier independent Standards and Spec reviews clear the four-document/private-helper correction. Previous codec audits do not justify a new parser, guessed memory multiplier or partial heap-admission claim. Native sampling reuse below requires sole handle ownership, not skipping validation of an independently owned handle.

## Findings and corrections

- Absolute directory ownership: store the guard as the sole operation-handle owner and derive identity at construction. Reuse its checked native identity without changing ancestry, authority, file proofs, Linux preallocated checks or the independent root-session clone. Remove the superseded clone function. Existing public observation tests now exercise external bindings and joined revocation/reset, not only ordinary roots. Temporary target-leaf samples6-to3 are removed; composed2,293 parent passes/22 existing ignores and desktop checks join0. Actual Ready/Mods2 and ordinary shutdown pass; reads still take8.04/7.15s. No latency or parity closure follows, and no new rule is needed.
- Earlier [matcher admission reuse](performance-ui.md#matcher-admission-reuse) removes one adjacent repeated check through its existing owner. Original guards remain and temporary instrumentation is removed. Its successful checks and slow-read observations remain checkpoint-specific, not a responsiveness or parity closure.
- Private diagnostic: reduction to two requests accidentally dropped array-shape validation and deadline checks after JSON parsing and final assertions. Restore those checks in the existing helper, without another framework or production seam. Independent review closes both findings at helper SHA256 `09bd469ced8c2cb53a76e36e1ec6a2094ed36613a4252503a86521a6a948a3b6`. Existing typed-wire and bounded-work rules in AGENTS already cover this; no duplicate rule is added.
- Review record: replace repeated historical chronology with linked feature evidence and current owner handoffs. Historical qualifications remain authoritative at their original checkpoints; compaction does not waive unresolved failures.
- [Ordinary Vanilla capture](wire-parity-review.md#ordinary-vanilla-capture-coverage): three actual callers reuse the bounded recorder and runtime. Preserve the canonical fixture on panic; body-local unwind and surviving workers mean retention is not joined settlement. Keep the temporary probe until diagnosis/resolution, not merely passing controls.
- [Inventory-size controls](performance-ui.md#bounded-inventory-size-controls): reuse ordinary provider/install/create and remove temporary probes after original joins. Preserve the failed1,024-object setup. Its40s installation terminal-event wait is not the45s Kill deadline or a measured list failure. Count, byte size and hash-directory distribution change together; no isolated cause or fast path follows.

## Validation and limits

Current selected filesystem/Minecraft/API/application verification joins0 with2,293 parent passes/22 existing ignores; desktop composition and scoped formatting pass. Both source axes clear the final five pins. Detailed original handles, intermediate compile refusal, probe removal and test scopes remain in [ownership evidence](performance-ui.md#absolute-directory-handle-ownership).

Frozen optimized `399deb21` retains exact Ready/Mods2, but list-body8,037.196/7,148.634ms still exceeds the existing1,000ms diagnostic. Body timing excludes JSON parsing; campaign deadlines include it. Ordinary API shutdown joins0 with separate PID/listener/metadata-and-lease absence checks. These are neither complete settlement/preservation proofs nor a causal speedup, hard heap/transport bound or native/parity certificate.

Exact hosted success belongs to `b4312584`, as recorded in [integration](integration.md), not the subsequent filesystem change. Current native inventory is locked; Linux/Windows source review does not prove execution or installed acceptance. Earlier helper, sampling and runtime qualifications remain in linked evidence and Git history; no historical failure is waived.

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
