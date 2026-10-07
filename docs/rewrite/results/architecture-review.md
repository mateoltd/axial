# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is the two-file launch-copy owner delta against `e2a4ed57`: `core/app/src/launch/libraries.rs` and its existing resource test-support dev-dependency. Root owns integration, result records and serialized verification; the author relinquished the source before execution. Independent Standards and Spec reviews clear source pins `7e4da80b`/`a47ef54f`. Production retains the same global physical-work owner and limits. UI, public contracts, native proof checks, credentials and signing are unchanged.

The committed [provider-await control](wire-parity-review.md#original-admission-across-provider-io) uses the existing HTTP fixture and original Admission. An unchanged-library positive control excludes cache publication/provider failure as alternate causes of the observed-drift refusal. Final review pins and focused/catalog/instance results remain in that feature record.

The committed [managed reselection control](wire-parity-review.md#resolution-generation-retention) closes new admission while accepted resolution retains its original valid generation until returned Admission drops. No cancellation policy is invented; external-root/UI/persistence/reset gates remain separate.

## Findings and corrections

- Full-suite Linux execution twice refused two native-replacement controls before their intended mutation. A private branch probe identifies exhausted process worker admission, not native-proof failure or scratch exhaustion. Launch-copy Retention now captures the existing concrete owner once; Prepared, prepare and settlement share it. Only fixtures select the existing same-limit isolated owner. A new control reserves all four permits, requires typed Unavailable/WouldBlock, releases them and requires successful native revalidation. No replacement check is bypassed; no production pool, generic injection layer, retry or limit increase is added. Existing global cross-owner coverage remains intact.
- Earlier cancellation/creation review adds overall fixture shutdown deadlines without a production framework. Accepted work, observers and pins settle before preserve-only retirement; retained objects alone cannot prove continued execution after runtime teardown. One actual HTTP Create survives postcommit queue refusal, repeated reads and reopen without replay. Queue closure precedes Create, not its commit; exact scope remains in the feature record.
- No redundant namespace, speculative abstraction, duplicated authority or pass-through production layer warrants a change in this delta. Existing AGENTS ownership, revision and bounded-cleanup rules already cover the findings; no new rule or stable-contract rename is justified.
- This record replaces repeated checkpoint chronology with feature-owned evidence and Git history. Earlier detailed architecture findings remain at `git show d0baa6a5:docs/rewrite/results/architecture-review.md`.

## Validation and limits

Focused copy7/coordinator1 and current macOS app842/Minecraft1044/resource8 pass, joined0, including existing global cross-pool coverage. Formatting/diff checks pass; Cargo.lock is unchanged. The budget control reserves permits, not four executing workers, and does not identify competing holders. The first Linux pass retains stale diagnostic output and is excluded. After package-scoped disposable-build cleanup, a fresh Linux six-library run passes2379 parent tests/15 existing ignores, joined0, with rebuilt app/API/resource and zero diagnostic markers. Original baseline `e99dff08` plus only these two changed files is verified; later controls and installed parity are not. Exact source/executable hashes, checks and preserved failed evidence are in [fixture budget isolation](wire-parity-review.md#launch-copy-fixture-budget-isolation).

Earlier dropped-caller/provider/committed-creation controls and exact source pins remain in [wire parity](wire-parity-review.md#original-admission-across-provider-io), with joined cleanup and independent reviews. Those results are not native/full-parity acceptance.

Hosted [run37633978420](https://github.com/mateoltd/axial/actions/runs/37633978420) passes both jobs at exact `e2a4ed57c97b09a46227cfdbf90f1918fe8edf18`; watch joins0 and independent SHA/job assertion agrees. This validates committed generation retention, not the working launch-copy fix or historical failures.

The earlier [run37624283183](https://github.com/mateoltd/axial/actions/runs/37624283183), at `e99dff08`, fails at API127/one/eight ignores: a45-second cold-reopen Kill request acknowledgement timeout. The adapter does not await process settlement, so acceptance and cause remain unknown.

Original-source Linux no-init execution instead fails at process settlement with adopted zombies. The fingerprinted init comparison passes that focused control, then twice refuses the same two application positives before replacement. Private probes identify worker admission/command InspectionLimit; competing holders and an additional Java-probe failure remain separate unknowns. Source is restored after capture; subsequent stale diagnostic build reuse is excluded above. The linked [Linux diagnosis](wire-parity-review.md#launch-copy-fixture-budget-isolation) retains full failed/positive evidence. Root owns correction and verification; never conflate these gates with Kill acknowledgement.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): busy, partial, interrupted and unavailable-status recovery.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
