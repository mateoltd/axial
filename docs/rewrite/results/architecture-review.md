# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, integration evidence and active ownership. Development continues on `main`, preserving history. Import/benchmark/API/frontend owners are frozen; root owns serialized verification and integration. Independent review covers deleted-instance suites/drivers, their metadata proof, ordinary instance publication, qualification and driver capacity. Root finished the final two review findings after the workers' usage failures; the independent reviewer subsequently verified both corrections.

## Findings and corrections

- **Reuse historical owners.** Legacy retains profile-wide benchmark records after instance deletion. Shared immutable batches use the existing tables, read routes and metadata receipt, source-scoped archive identity and report conversion. Unsupported benchmark conversion preserves independent report/account/settings import. No dummy instance, new table, route, scheduler or coordinator.
- **Refuse execution before projection fails.** Broadening historical identity alone would make pending archive rows fail UUID parsing during Resume availability. The existing source-suite admission now rejects archive continuation early; list/detail remains readable with `can_resume=false`, NULL requests/lineage and no scheduled work. Ordinary UUID continuation retains its checks.
- **Preserve qualification reads.** Review found archived release-validation GET returned503 despite legacy returning incomplete historical evidence. The archive-only qualification branch reuses the existing target/proof matcher: no managed proof yields missing proof evidence; a matching proof adds `managed_install_state_invalid`. Current-install authority stays false. Live UUID inspection is unchanged.
- **Apply restore ceilings at every insertion.** Imported batch checks alone allowed ordinary or successor creation to add row4,097 and break the next read/startup. All three insertion paths now enforce the existing4,096 limit atomically with one constant. Replay, updates and settlement remain available at capacity; no retained evidence is pruned. This repeated omission justified refining the existing bounded-work AGENTS rule.
- **Verify bounded final evidence.** Snapshot proofs bind exact source IDs/payloads and cap actual persisted reads before decode. Old NULL-proof upgrades preserve later edits; completed replay never repairs. Final settings and creation writes are followed by owner verification, including report/suite/driver corruption across initial/ready/published recovery. No speculative abstraction or UI change landed.

## Validation

Logs are under `.rewrite-logs/`. Initial metadata/frontend regressions fail at absent completion and invalid count acceptance (`archived-benchmarks-{owner,frontend}-red.log`). The final findings have meaningful RED evidence: accepted ordinary-driver overflow and qualification HTTP503 (`archived-benchmarks-capacity-red.log`, `archived-benchmarks-qualification-red-qualified.log`). The first qualification invocation selected zero tests; its next fixture assumed two release runs instead of the actual eight. Both diagnostic failures remain recorded; the corrected fixture exercises the real endpoint before the production fix.

All **15** focused owner groups pass (`archived-benchmarks-owner-final.log`), including actual persisted64-MiB reads and4095/4096 ordinary/successor admission, rollback, replay and reopen. Full app/API/desktop suites pass **960 / 137 / 88**, with six ignored app/API subprocess helpers each (`archived-benchmarks-consumers.log`, `archived-benchmarks-desktop.log`). The API suite includes archive qualification with and without a matching managed proof, both import orders, all-pending/zero-instance history, immutable source and refused mutation. Frontend passes **484**, with one existing Guardian TODO, and source/test typing passes (`archived-benchmarks-frontend.log`, `archived-benchmarks-typescript.log`). Generated-contract equality passes (`archived-benchmarks-wire-check.log`); scoped authored formatting and independent final review are clear.

## Unresolved handoffs

This slice adds no native-interface acceptance and does not establish full cutover. Driver-only records after parent-suite pruning, missing-target Performance/content histories, unsupported records and source-effect settlement remain separate. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
