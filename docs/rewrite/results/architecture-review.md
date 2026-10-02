# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, integration evidence and active ownership. Development continues on `main`, preserving history. Benchmark/import/API owners are frozen; root owns shared registration, docs and serialized verification. This review covers detached drivers after legitimate source suite pruning, immutable metadata proofs, namespace observability and later parent admission. Previous archived-instance evidence remains in [integration evidence](integration.md).

## Findings and corrections

- **Represent independent history explicitly.** Legacy validates driver status without a parent and releases terminal suite claims. One nullable private column in the existing driver table distinguishes supported detached history from required-parent corruption. Existing batches, receipts and readers remain the owners; no fabricated instance/suite/report, public DTO, table, route or scheduler.
- **Preserve proof and action boundaries.** Optional sorted detached indices retain absent-field/v1 compatibility. Exact provenance/payload replay never repairs. A later matching parent does not rewrite the driver or its receipt; bounded reverse validation rejects conflicting publication. Missing-parent actions refuse, and subsequent explicit continuation still uses existing report/plan/instance admission. Queued obligations are not waived.
- **Prove empty namespaces.** Review found aliases and unreadable exact namespaces could hide records and falsely acknowledge zero. One unconditional validator now checks both flat namespaces, including names, ancestor files, nesting and unsafe entries. Supported accounts/reports remain independently eligible. The recurring empty-decoder mistake justified refining the existing imported-history AGENTS rule.
- **Charge actual bytes.** Shared parent reads and reverse scans remain bounded. Provenance SQL uses BLOB byte lengths and checked subtraction; ignored-constraint multibyte corruption refuses before allocation/decode. No stack-limit increase, parallel journal or general recovery framework.

## Validation

Logs are under `.rewrite-logs/`. Three meaningful RED regressions expose required-parent rejection and missing metadata/HTTP completion (`detached-driver-{owner,metadata,api}-red.log`). The first compile records three fixture error-conversion mistakes (`detached-driver-owner-green.log`). The next focused run passes11 and exposes three incorrect fixture assumptions (`detached-driver-owner-final.log`): hardlinks refuse capture, and invalid SQLite constraints refuse fresh open. Tests now distinguish those earlier guards from composed metadata/row-reader behavior; admission was not weakened.

Full frozen-source app/API/desktop verification passes **974 / 138 / 88**, with six ignored app/API subprocess helpers each (`detached-driver-consumers.log`, `detached-driver-desktop.log`), including all14 new owner/import groups. The composed HTTP case covers metadata-only and both survivor publication orders, source-free reads/actions/replay/reopen, immutable source and destination bytes, and no implicit execution. Frontend passes **484** with one existing Guardian TODO; source/test typing and generated-contract equality pass (`detached-driver-frontend.log`, `detached-driver-typescript.log`, `detached-driver-wire-check.log`). Independent final review and scoped authored formatting are clear. Hosted [run37007725628](https://github.com/mateoltd/axial/actions/runs/37007725628) passes preceding archived benchmark checkpoint `b61143a4`, not these later edits.

## Unresolved handoffs

This slice adds no native-interface acceptance and does not establish full cutover. Missing-target Performance/content histories, unsupported records and source-effect settlement remain separate. Changed-source metadata replay retains its existing fingerprint conflict. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
