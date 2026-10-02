# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, integration evidence and active ownership. Development continues on `main`, preserving history. Latest-Performance/API owners are frozen; root owns registry lookup, docs and serialized verification. This review covers the per-instance latest read and exact registry identity/lifecycle. Archived Performance and earlier import evidence remains in [integration evidence](integration.md). Content-history preparation and loader acceptance auditing are separately owned and not included in this checkpoint.

## Findings and corrections

- **Keep read lifecycles distinct.** Legacy per-instance latest requires registry membership; global operation-by-ID does not. The existing latest owner now checks the exact live record before its unchanged query. Missing/deleted returns404, reserved/deleting409, corrupt/unavailable storage503. Global historical reads remain unchanged.
- **Validate at the existing owner.** A real regression reproduced lookup accepting another valid instance's JSON under the requested identity. The registry now checks requested identity and indexed lifecycle against decoded evidence. Performance reuses that crate-private lookup, rather than introducing a parallel eligibility model.
- **Use current coordination.** Both reads run inside one existing query-only metadata callback, serialized by its connection mutex. No recursive lock, new transaction API, filesystem admission, task, state table, public DTO or UI change. Existing typed-contract guidance covers the correction; no duplicate AGENTS rule.

## Validation

Logs are under `.rewrite-logs/`. Actual HTTP exposes unknown UUID200 instead of404; two owner RED cases expose unknown/deleted latest admission, and registry RED exposes wrong-identity acceptance (`latest-performance-{api,owner,registry}-red.log`). Frozen full checks pass **991 app / 140 API / 88 desktop**, six ignored app/API subprocess helpers each (`latest-performance-consumers.log`, `latest-performance-desktop.log`). The composed case imports A/B, keeps B's files during actual deletion, and reopens while latest refuses and global bytes remain exact. Owner cases cover real reserved/deleting rows and corrupt registry evidence. Scoped formatting, generated equality and independent final review pass (`latest-performance-rust-format.log`, `latest-performance-wire-check.log`). Frontend source is unchanged, so its preceding486-test checkpoint is not rerun or treated as new native evidence.

Hosted [run37015561967](https://github.com/mateoltd/axial/actions/runs/37015561967) passes the preceding archived Performance `70d75049` checkpoint. The cancelled duplicate run is not acceptance evidence; neither result attests these later edits.

## Unresolved handoffs

This slice adds no native-interface acceptance and does not establish full cutover. Missing-target content history, unsupported records and source-effect settlement remain separate. Changed-source metadata replay retains its existing fingerprint conflict. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
