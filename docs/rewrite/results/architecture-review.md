# Architecture review

Updated 2026-10-07. Working branch: `main`. Full non-Guardian parity remains active; this record is not a release certificate. Historical results belong to their recorded source checkpoints, not later edits.

## Reviewed scope and ownership

Current scope is the test delta against `d0baa6a5`: dropped Setup resolution caller, deliberately disconnected catalog response, and HTTP creation surviving install-queue refusal. Root owns integration, result records and serialized verification. Authors relinquish their exact files before execution; independent Standards and Spec reviews cover the delta. No production, UI, generated contract, dependency, credential or signing change is in scope.

The committed [provider-await control](wire-parity-review.md#original-admission-across-provider-io) uses the existing HTTP fixture and original Admission. An unchanged-library positive control excludes cache publication/provider failure as alternate causes of the observed-drift refusal. Final review pins and focused/catalog/instance results remain in that feature record.

Next test-only delta against `ee3f21ab` covers [managed reselection retention](wire-parity-review.md#resolution-generation-retention). New admission closes, while accepted resolution retains its original valid generation until returned Admission drops. No cancellation policy is invented. Both review axes clear fixture `9a2e2e5a`; focused1 and instance93 pass, joined0. Root owns the relinquished source and verification; external-root/UI/persistence/reset gates remain separate.

## Findings and corrections

- The new HTTP control initially awaited API shutdown without an overall deadline. Domain timeouts do not bound its final HTTP/telemetry/rules joins. Its author adds direct fixture-level deadlines, retains unresolved owners and preserves the failed profile; timeout never authorizes reopen or proves settlement. Retained state does not prove continued task execution after fixture-runtime teardown. No production shutdown framework is introduced.
- Dropped-caller coverage stays at the existing resolve/TaskOwner interface. Only the disposable caller is aborted. Accepted work must settle while the actual provider body remains withheld; queue observers, provider thread and root pins must join/drain before preserve-only retirement. Deliberate gated response writes tolerate only BrokenPipe/ConnectionReset; ordinary fixture callers remain strict.
- Creation coverage uses one actual HTTP mutation, repeated reads and ordinary cold reopen. Queue admission closes before Create; refusal occurs at its postcommit handoff. This does not claim closure temporally after commit. The literal public committed-success notice and exact retained instance/files exclude a false rollback or automatic replay.
- No redundant namespace, speculative abstraction, duplicated authority or pass-through production layer warrants a change in this delta. Existing AGENTS ownership, revision and bounded-cleanup rules already cover the findings; no new rule or stable-contract rename is justified.
- This record replaces repeated checkpoint chronology with feature-owned evidence and Git history. Earlier detailed architecture findings remain at `git show d0baa6a5:docs/rewrite/results/architecture-review.md`.

## Validation and limits

The new dropped-caller control passes1 in0.12s; the paired live-response admission control passes1 in0.33s; catalog checks pass21/one existing ignore in0.25s, all joined0. After scoped formatting, instance checks pass92/zero ignores in14.80s and the corrected HTTP control passes1 in0.37s, both joined0. Formatting and whitespace checks pass. Logs: `dropped-resolution-architecture-{focused,admission,catalog}.log`, `architecture-current-instances.log`, `create-queue-refusal-architecture-final.log`. Standards and Spec reviews are clear at final source pins API `4b481151`, catalog `ee245f06`, setup `3703394f`. These are test-only controls, not loader/native or full-parity acceptance.

Hosted [run37626105236](https://github.com/mateoltd/axial/actions/runs/37626105236) passes both jobs at exact `d0baa6a5`; retained watch and independent SHA/job assertion agree. It does not validate later controls or explain prior failures. The reviewed cancellation/creation controls are committed at `02da044f`, preserving history.

Fresh current-source full verification passes API129/eight existing helper ignores/45.18s and app840/ten ignores/63.41s, joined0 (`owned-resolution-creation-app-api.log`). Nested subprocess summaries are excluded. These results validate the reviewed controls, not native/full parity or historical failures.

The earlier [run37624283183](https://github.com/mateoltd/axial/actions/runs/37624283183), at `e99dff08`, fails at API127/one/eight ignores: a45-second cold-reopen Kill request acknowledgement timeout. The adapter does not await process settlement, so acceptance and cause remain unknown.

Root's isolated original-source Linux run instead fails at API120/eight/eight ignores; focused external-library execution also fails at process settlement, not the original Kill-send boundary. No-init PID1 sleep retains adopted zombies. After repairing the failed hardlink copy with relative tar streaming, source and executed test-binary fingerprints match. Under init, that focused test passes1/4.03s with no observed zombies; both original-selection runs pass API128/eight ignores, then fail app836/two/seven ignores before replacement injection. A private probe confirms worker-admission refusal followed by command InspectionLimit, before native replacement injection; competing holders remain unidentified. Its additional Java-probe failure is separate. Exact original source is restored after capture. Both isolated app controls pass unchanged. Root owns corrective fixture isolation and verification; do not weaken safety or conflate these boundaries with the CI Kill-send timeout. Logs remain under `kill-ci-linux-*`.

## Unresolved owner handoffs

- [Readiness and wire parity](wire-parity-review.md#remaining-readiness-requirements): loader/provider I/O, generation switch, cancellation, cached/bulk publication, runtime postclaim cancellation, parent/origin/negative inheritance and managed-component selection. Vanilla coverage does not close loader or native gates.
- [Integration](integration.md): actual saved-world reload and installed-platform matrix; ordinary test/CI passes are not full parity.
- [Accounts](native-auth.md): noninteractive credential persistence across distinct builds under the intended authorized stable signing identity.
- [Updates](updates.md): trusted signed installed update; retain unresolved frontend-watch failure.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen. Process survival/fakesink decoding are insufficient.
- [Library lifecycle](library-lifecycle.md) and [Library UI](library-ui.md): busy, partial, interrupted and unavailable-status recovery.
- [Forge](forge-loader.md), [Content](pack-files.md), [world files](world-files.md), [Performance](performance-ui.md) and [benchmarks](current-benchmarks.md): preserve their recorded version/recovery/runtime limits and unexplained historical failures.

Earlier admission, publication diagnostics, runtime, browser and cleanup findings are linked from [wire parity](wire-parity-review.md), [integration](integration.md) and the feature records above. Their checkpoints and opaque receipts remain authoritative only for their recorded scope. This scheduled review neither replaces nor resets the full-parity goal.
