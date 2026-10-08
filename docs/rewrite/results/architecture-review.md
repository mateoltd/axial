# Architecture review

Updated 2026-10-08. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is runtime conflict coverage, [native reload setup and busy refusal](native-auth.md#current-native-world-reload-setup), and [browser screenshot bulk/partial deletion/reopen](screenshot-files.md#browser-partial-deletion-and-cold-reopen). Root owns integration, evidence and serialized verification; private authors relinquish before integration. Independent axes clear runtime test `cc25fbb1` and private screenshot witnesses `49e39e53`/`9453cd2d`. Earlier checkpoints retain their evidence in feature records.

UI, wire contracts, native proof checks, credentials and signing are unchanged. Earlier evidence applies only to its recorded source and admission scope.

## Findings and corrections

- Runtime conflict controls reuse the existing postclaim gate and materializer. One concrete64-byte reader replaces duplicated readers: prior cancellation sampling stays192bytes; both new fixtures total384bytes and four namespace entries. Exact foreign-directory refusals retain preservation and joined settlement. Unknown joins retain the running executor and native owners. No workflow framework, stricter matching-canonical reuse policy or duplicate AGENTS rule follows.
- The temporary bounded four-event Stop probe retains its test-only scope, unchanged deadlines and removal obligation. Capture loss is distinct from HTTP/process completion; no speculative Stop fix or parallel owner follows. Detailed source pins and limits remain in [wire parity](wire-parity-review.md#temporary-hosted-acknowledgement-capture).
- Screenshot acceptance uses existing resource actions and ordinary process lifetimes. Private witness review reserves cleanup before I/O and requires explicit SQLite heap-cap readback before profile sampling. The retained unsupported-cap refusal is observer evidence, not a product defect. Existing budget rules suffice; no new production abstraction or AGENTS rule follows.

## Validation and limits

Current missing-runtime controls5/1.26s and full Minecraft with `test-support`1,057/zero failures/zero ignores/85.88s pass, joined0. Both source-review axes clear; parent totals exclude four nested helper summaries. Final source/control/summary assertions and scoped checks are recorded separately. This is expected-green materializer evidence, not a product fix, executed unknown-join path or process/native/installed proof.

Earlier [ordinary binary preservation](library-lifecycle.md#ordinary-executable-startup-preservation) verifies normal shutdown and two natural refusal exits with exact stopped payloads, excluding only native lease content. It is Mac API-process acceptance, not repair, general interruption or native/installed proof.

Exact `99daa535` hosted [run37704903473](https://github.com/mateoltd/axial/actions/runs/37704903473) completes both jobs successfully; watch and independent final run/SHA/job assertions join0. It predates the partial-deletion record. Earlier hosted successes retain their scopes in [integration](integration.md).

Actual browser bulk Delete completes two200/ok responses and clears selection. A separate two-document stale-name batch reaches one200/ok, one404/not-found and no third DELETE; its UI reports1of3 and retains only the untouched third selection. Five bulk and six partial phases verify exact protected trees/all22 metadata tables through ordinary shutdown/cold reopen/final shutdown, allowing only declared file effects and parent namespace/metadata changes. Both journeys' API/frontend pairs join0; final PIDs/listeners/openers are absent. Explicit refusal is not unknown-outcome recovery; interruption, native/installed acceptance and whole-profile immutability remain unproved.

The older unbound native selection supplies no current regression or preservation proof. Current source explicitly launches the frozen ARM64 executable against the recorded valid profile before selection and creates a separate Vanilla target through ordinary UI. Both axes correct private sampler/copier accounting; the existing AGENTS total-work rule now explicitly includes verification, probes and exclusive creations before I/O. Exact copied bytes/distinct identities and original-world/backup comparison pass. Actual native Launch reaches Playing; [Duplicate visibly refuses](library-ui.md#native-duplicate-refusal-while-playing), preserving instance-table projections and protected original/backup state. Game-window automation, gameplay reload and final closure/preservation remain open. No production fix or credential action follows.

The completed three-attempt campaign at `859c1ad7` captures no hosted API trace; the earlier Vanilla run remains cancelled. Original local and [hosted45s Kill acknowledgement failures](wire-parity-review.md#accepted-worker-provider-cancellation), Linux no-init settlement, Java/worker admission and frontend-watch failures remain unresolved with preservation/cleanup evidence retained. The Kill acknowledgement failures are not later tree-settlement timeouts. Delivery success/nonrecurrence neither diagnoses them nor certifies later edits; no speculative Stop fix or deadline increase follows.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk and other runtime publication interference, plus remaining parent/origin/runtime-selection variants. Current controls close only their recorded paths; Vanilla coverage does not close loader or native gates.
- Root owns recurring Kill acknowledgement diagnosis. The new held-client control verifies one postmetadata refusal/joined-success/file-backed-reopen path, not arbitrary interruption or write-failure recovery. Legacy/current Downloads has no active Cancel button; the extra internal endpoint is starting-only, so a held-body active-Cancel success is not a retained parity gate. Pre-admission lookup alone is not a retained-effect shutdown defect.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): remaining busy, partial, interrupted and unavailable-status recovery; the current preserve-only admission control closes only its recorded path.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
