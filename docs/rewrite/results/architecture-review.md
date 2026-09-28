# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, integration evidence and active ownership. Development continues on `main`, preserving history. Import/report/API/frontend owners are frozen; root owns serialized verification and integration. Independent review covers archived terminal reports whose source instances were deleted, their metadata receipt, ordinary instance publication and actual consumers.

## Findings and corrections

- **Preserve historical identity without live authority.** Legacy deletion removes registry membership but retains globally readable launch reports. Reuse the report store/read routes and existing metadata receipt; no dummy instance, new table, route, scheduler or coordinator. Archived identity is source-scoped and non-UUID. A shared immutable batch supports either publication order without duplicating payloads or adding a receipt prerequisite.
- **Prove absence at the source boundary.** Validate the captured original registry envelope, required fields, raw duplicate keys, canonical IDs and captured membership. An unsupported registry's empty projection or an unselected instance cannot authorize archival identity. Live nonterminal history and unresolved suite/file/process obligations remain blocked independently of metadata report preservation.
- **Bound actual stored work.** Review found replay and final verification capped individual rows but not aggregate persisted bytes: whitespace-expanded, semantically identical rows could exceed the prepared batch budget. Both reads now enforce remaining 64 MiB in SQL before allocation/decode, accounting for new canonical inserts. Keep the final batch reread because later writes may corrupt earlier rows. The real SQLite boundary test accepts exactly 64 MiB and refuses overflow before a later malformed row.
- **Acknowledge the final transaction state.** The receipt binds exact source snapshot IDs/payloads; old NULL-proof completion preserves later destination edits. Completed replay verifies, never repairs. Final settings and creation writes are followed by owner verification. Trigger regressions cover ignored/rewritten proof rows and late report corruption through initial, ready and published recovery.
- **No stylistic churn.** Existing AGENTS rules already cover source schemas, shared immutable preparation, batch bounds and final write acknowledgement. No new anecdotal rule, public-contract rename, UI redesign or speculative abstraction was added.

## Validation

Logs are under `.rewrite-logs/`. Real composed HTTP and frontend regressions fail at lost report preservation/invalid count acceptance before implementation (`archived-reports-{api,frontend}-red.log`). Final frozen-source app/API/desktop suites pass **945 / 133 / 88**, with six app/API ignored subprocess helpers each (`archived-reports-{consumers,desktop}.log`). Frontend passes **482**, with one existing Guardian TODO; all **57** focused decoder/workflow checks pass (`archived-reports-frontend{,-green}.log`). Source TypeScript, generated-contract equality (`archived-reports-wire-check.log`) and scoped authored formatting pass. Independent review is clear after the aggregate-limit correction.

## Unresolved handoffs

This slice adds no native-interface acceptance and does not establish full cutover. Archived suites/drivers, missing-target Performance/content histories, unsupported records and source-effect settlement remain separate. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
