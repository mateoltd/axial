# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, integration evidence and active ownership. Development continues on `main`, preserving history. Content-history, importer, metadata and API/frontend owners are frozen; root owns migration/wire integration, final creation checks, docs and serialized verification. Quilt's independent decoder RED remains outside this checkpoint.

## Findings and corrections

- **Reuse the historical owner.** Supported missing-target Content records use existing `install_history`, immutable shared batches, unchanged source grammar and original-registry absence. Archive IDs preserve references, never live instance, filesystem, queue or execution authority. Source-global and ordinary survivor publication stay independent of current registry membership.
- **Bound the composed snapshot.** Metadata history reads verify both completed proofs under one 128-row/8-MiB budget, charging actual stored bytes before allocation. Scope-bound digests prevent empty-proof swapping; exact row/index bytes prevent replay repair. Aggregate archive-envelope overflow withholds only archive completion, preserving valid global history and account/settings import.
- **Verify the last write.** The existing creation matrix reproduces lost history after final completion. Both initial and recovered paths now verify install history after that write. Existing settings/import final verification also covers archived completion. Old receipt completion preserves later account/settings state and original revisions.
- **Remove unused layering.** The temporary global-reader adapter and new test-only archived-proof passthrough are removed; real consumers and tests use the combined reader. No new table, route, scheduler, recovery owner or UI controls. Existing typed-contract and durable-write rules cover the production findings. Repeated stale old-receipt fixtures refine the existing AGENTS fixture rule rather than add another rule list.

## Validation

Logs are under `.rewrite-logs/`. Owner RED rejects supported archive binding (`archived-content-owner-red.log`); metadata/HTTP/frontend RED reproduce absent count or incorrect confirmation (`archived-content-metadata-red.log`, `archived-content-api-red-behavior.log`, `archived-content-frontend-red.log`). The first API attempt is a fixture macro compile error, not behavioral evidence. Final-creation RED reproduces `install_history/initial` loss after the other matrix families pass (`archived-content-final-creation-red.log`).

The first broad run passes136 API tests and fails one obsolete old-receipt fixture: it removed only global proof while retaining legitimately completed empty archive proof. The corrected fixture removes both; archive-only completion has independent positive coverage. Final frozen verification is recorded in [integration evidence](integration.md), including the complete late-write matrix, source-free reads, exact replay, both publication orders, raw-byte limits and independent adversarial review. Frontend passes488 tests, one existing Guardian TODO; source typing, focused63 checks and generation `0e9d96fdf1b8` pass.

Hosted [run37017956131](https://github.com/mateoltd/axial/actions/runs/37017956131) passes the preceding latest-Performance `5cbb8612` checkpoint, not these later edits. Qualified Forge/NeoForge runtime evidence is relabelled as superseding historical failures, without claiming native gameplay or later-source acceptance. Quilt's checked official digest mismatch remains unresolved; no integrity bypass.

## Unresolved handoffs

The existing metadata and instance history readers use short completed read-only SQLite snapshots. Review confirms the connection mutex, query-only setting, execution budget and leaked-transaction refusal preserve this boundary. The misleading `MetadataStore::read` comment now permits snapshots that finish inside the callback, without allowing escaped transactions or configuration changes. No runtime change, mutation-admitted read, new API or framework.

This slice adds no native-interface acceptance or full cutover. Unsupported history and source-effect settlement remain separate. Changed-source metadata replay retains its fingerprint conflict. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
