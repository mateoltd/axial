# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, integration evidence and active ownership. Development continues on `main`, preserving history. Quilt's three loader files, install history and its HTTP fixture are frozen after independent review. Root owns docs, serialized verification and the disposable native acceptance harness.

## Findings and corrections

- **Reuse the historical owner.** Supported missing-target Content records use existing `install_history`, immutable shared batches, unchanged source grammar and original-registry absence. Archive IDs preserve references, never live instance, filesystem, queue or execution authority. Source-global and ordinary survivor publication stay independent of current registry membership.
- **Bound the composed snapshot.** Metadata history reads verify both completed proofs under one 128-row/8-MiB budget, charging actual stored bytes before allocation. Scope-bound digests prevent empty-proof swapping; exact row/index bytes prevent replay repair. Aggregate archive-envelope overflow withholds only archive completion, preserving valid global history and account/settings import.
- **Verify the last write.** The existing creation matrix reproduces lost history after final completion. Both initial and recovered paths now verify install history after that write. Existing settings/import final verification also covers archived completion. Old receipt completion preserves later account/settings state and original revisions.
- **Remove unused layering.** The temporary global-reader adapter and new test-only archived-proof passthrough are removed; real consumers and tests use the combined reader. No new table, route, scheduler, recovery owner or UI controls. Existing typed-contract and durable-write rules cover the production findings. Repeated stale old-receipt fixtures refine the existing AGENTS fixture rule rather than add another rule list.
- **Canonicalize the fixture, not admission.** The existing temporary-profile helper now creates roots under the canonical system temporary parent, following the existing AGENTS rule. This removes the need for native journey scripts to override Python's process-global temporary-directory setting. Production path admission and reset decisions are unchanged.
- **Keep compatibility at the provider boundary.** Quilt's existing install and reconstruction paths share one private mapping validator. Omission requires an authenticated matching base with a valid UTC release time at or after 2025-12-16; present mappings retain exact identity/integrity and declaration checks. No new persisted proof, public contract or checksum fallback.
- **Retain source history without widening authority.** The existing immutable install-history validator admits only the producer's exact pre-worker Vanilla/loader cancellation shape. One small helper preserves the prior Content predicate. Original diagnostic memory is recorded evidence, not restored Guardian behavior; source-effect guards and global publication/readback stay unchanged. The record does not establish whole-profile idleness.

## Validation

Logs are under `.rewrite-logs/`. Owner RED rejects supported archive binding (`archived-content-owner-red.log`); metadata/HTTP/frontend RED reproduce absent count or incorrect confirmation (`archived-content-metadata-red.log`, `archived-content-api-red-behavior.log`, `archived-content-frontend-red.log`). The first API attempt is a fixture macro compile error, not behavioral evidence. Final-creation RED reproduces `install_history/initial` loss after the other matrix families pass (`archived-content-final-creation-red.log`).

The first broad run passes136 API tests and fails one obsolete old-receipt fixture: it removed only global proof while retaining legitimately completed empty archive proof. The corrected fixture removes both; archive-only completion has independent positive coverage. Final frozen verification is recorded in [integration evidence](integration.md), including the complete late-write matrix, source-free reads, exact replay, both publication orders, raw-byte limits and independent adversarial review. Frontend passes488 tests, one existing Guardian TODO; source typing, focused63 checks and generation `0e9d96fdf1b8` pass.

Hosted [run37022906365](https://github.com/mateoltd/axial/actions/runs/37022906365) passes exact Content checkpoint `54a4ce54`; terminal watch and independently queried SHA/conclusion agree. The canonical fixture's existing canary cleanup and exact parent assertions pass (`native-canonical-fixture.log`). Qualified Forge/NeoForge runtime evidence is relabelled as superseding historical failures, without claiming native gameplay or later-source acceptance. Quilt's checked official digest mismatch remains unresolved; no integrity bypass.

Quilt's official omitted-mapping decoder RED precedes correction; all seven provider checks and the complete 968-test Minecraft suite pass, including real install/reconstruction composition and old-date/malformed/proof-downgrade refusals (`quilt-mapping-{provider-green,minecraft}.log`). The install-history owner RED reproduces rejection of both exact source variants. All five initialization-history checks and the corrected authenticated HTTP lost-waiter/replay/reopen journey pass. Both earlier HTTP attempts were confounded by a fixture short key (`fabric`) instead of the original component ID (`net.fabricmc.fabric-loader`); they are not independent parser RED evidence. Existing typed-contract guidance covers the correction, so no new AGENTS rule is added.

Frozen combined consumers pass1,011 app,141 embedded API and88 desktop tests, six app/API ignored helpers each. Generated-contract equality and scoped formatting pass (`install-initialization-{app-final,api-embedded-final,desktop-final,wire-check,format}.log`). Frontend markup and generation are unchanged; no fresh native acceptance follows.

## Unresolved handoffs

The existing metadata and instance history readers use short completed read-only SQLite snapshots. Review confirms the connection mutex, query-only setting, execution budget and leaked-transaction refusal preserve this boundary. The misleading `MetadataStore::read` comment now permits snapshots that finish inside the callback, without allowing escaped transactions or configuration changes. No runtime change, mutation-admitted read, new API or framework.

This slice adds no native-interface acceptance or full cutover. Unsupported history and source-effect settlement remain separate. Changed-source metadata replay retains its fingerprint conflict. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The old interrupted-Reset handoff has no current marker/intent/canaries and cannot establish acceptance. A fresh rebuilt debug bundle opens and quits normally, then genuinely published v2 interruption waits with matching complete idle fingerprints and unchanged protected siblings. macOS dialog automation times out; an explicit manual Preserve choice is pending. No UI action, reset bypass, deletion or native acceptance is inferred (`native-current-interrupted-reset-acceptance.md`).

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
