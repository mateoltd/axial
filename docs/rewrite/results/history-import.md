# Terminal history import assessment

Status: bounded historical reports, suites, drivers, per-instance Performance commands and global rules refresh history integrated. Current verification checkpoints are in [integration evidence](integration.md); earlier authenticated report/suite/driver import/read/retry/restart journeys remain in `.rewrite-logs/suites-api-current.log`. Full profile cutover remains unavailable.

## Queued restart handoff preservation

An exact legacy `interrupted` driver with error `driver automatic resume queued after restart` may be retained as immutable history in an independently copied instance. All existing source schema, time, count, run/report relationships, bounded input and source/destination verification still apply. Malformed or active-session records remain refused. The source record, original error and retained-obligation preview are unchanged; `cutover_available` remains false.

This is not terminal-process evidence: the [legacy driver owner](../../../legacy/apps/api/src/state/benchmark_suite_drivers.rs) clears `active_session_id` in `admit_loaded_driver` while queuing replay, and `apply_driver_transition` retains the handoff obligation. Separate verified copying does not establish predecessor settlement.

Explicit Resume now accepts compatible queued history through the existing benchmark owner, creating separate destination work without consuming or settling the predecessor obligation. Source history alone never starts execution: import and ordinary reopen retain NULL driver requests and zero launch intents. Exact destination, current-plan, report, exclusion and successor-link checks remain mandatory. No schema, route or scheduler is added; automatic handoff and full cutover remain open.

Coverage includes both shared-suite driver orders, canonical all-pending/mixed plans, missing or incompatible destination evidence, immutable successor replay and lost responses. All 26 benchmark tests, nine composed import tests and four real-process Resume journeys pass (`queued-resume-{core-green,api-import,process}.log`). The process journeys reopen before Resume to prove no automatic execution, then execute only pending destination runs, reconcile a discarded response, reopen/replay the same successor and preserve source bytes/mtime and inherited reports. These use fixture Java/game processes, not gameplay. The two new queued cases failed specifically at continuation eligibility before the correction (`queued-resume-red.log`).

## Integrated slice

Convert only validated predecessor schema-3 terminal launch reports from `profile/benchmarks/launch/<session_id>.json` into the existing schema-4 launch report owner. Read through the admitted inventory's captured file capabilities and revalidate its fingerprint before publication. Never reopen caller-supplied paths or write the predecessor profile.

Import creates historical reports only: no live sessions, launch intents, process ownership, process probes, termination, relaunch, or resumed benchmark drivers. Use a deterministic, source-scoped non-UUID report ID, such as `legacy-<sha256(source_identity, legacy_session_id)>`, with unambiguous hash input encoding. This separates imported history from the canonical UUID identities accepted by launch intents. Remap comparison baseline IDs with the same function.

Source contracts: [legacy report schema and validator](../../../legacy/apps/api/src/state/launch_reports.rs), [legacy outcome reasons](../../../legacy/core/launcher/src/process/mod.rs), [replacement report owner](../../../core/app/src/launch/reports.rs).

## Admission and field mapping

Do not deserialize the legacy API export as the persisted record. Validate the actual strict schema-3 record: exact schema/version, filename/session correspondence, canonical bounded IDs and millisecond UTC timestamps, `recorded_at >= launched_at`, canonical scenario/device fields, coherent bounded stages/evidence, outcome relationships, crash correlation, and coherent comparison. Unknown fields, malformed data, oversize input, or lossy conversion remain unsupported.

Legacy reports also contain `running` and `degraded` snapshots. Those are not terminal evidence and remain retained. The legacy terminal set is `failed`, `exited`, `completed`, `stopped`, `cancelled`, and `canceled`. An `unknown` record requires a valid structured terminal outcome; otherwise it remains unsupported.

| Source evidence | Schema-4 mapping |
| --- | --- |
| Session and instance IDs | Source-scoped report ID and the verified reserved destination UUID, published atomically with instance completion |
| Version, timestamps, scenario, device, resource budget | Preserve validated values; these structures already align |
| Exit code, boot duration, neutral crash and stage evidence | Preserve without inferring new process facts |
| Structured outcome | Preserve kind/reason; map `WatchdogKilled` to `StartupStalled` and retain the original reason as historical evidence |
| Missing structured terminal outcome | `Unknown / UnknownExit`, preserving the original report label as evidence; exit code `0` or `exited` alone never means clean success |
| Top-level failure class | Explicit matching enum into `session_outcome.failure_class`; unsupported classes remain blocked |
| Priority, failure detail, historical PID | Clearly labeled neutral evidence, never current process authority |
| Guardian/healing fields and their stage evidence | Exclude; retain unrelated neutral evidence |
| Missing logs | Empty logs with zero known dropped entries; do not invent output |
| Comparison | Preserve validated historical dimensions/measurements with remapped baseline ID through the report owner |

Reject contradictory report outcome, structured kind, and reason. Use the replacement's canonical outcome summary rather than interpreting legacy prose. Neutral top-level evidence can use an explicitly untimed historical stage (`ended_at_ms` and `duration_ms` absent), so it does not change completed-stage performance totals. If existing stage/evidence/text limits cannot preserve those fields, keep the report blocked rather than silently truncate it.

## Instance mapping and ordering

[Instance import](../../../core/app/src/instances/import.rs) durably records `(source_id, legacy_id, fingerprint) -> instance_id` with the ordinary creation reservation. Prepared history is rebound to that actual reserved UUID outside the final transaction; a retry must not use its newly prepared candidate UUID. Report rows, live registry publication, and creation completion commit together after exact source and payload verification. Never substitute a selected instance or match by name/version.

[Instance preparation](../../../core/app/src/import/prepare.rs) permits the retained-history blocker only when the exact converter validates every retained history obligation; each instance carries only its own reports. This avoids requiring a separately completed mapping before history can publish. Retained obligations remain visible and cutover stays unavailable. Unsupported history is not blanket-waived and still blocks import.

## Persistence and comparison fidelity

The opaque prepared report and benchmark batches validate and serialize before SQLite publication, then use owner-specific immutable insertion inside the existing instance transaction: identical retry succeeds, differing same-key content conflicts, and no historical record is overwritten. The entire instance/report/suite/driver batch rolls back on a final-commit error. Existing ready/published instance recovery requires source readmission and publishes once. Explicit completed retries verify both batches, including report index fields and absent driver requests, without recopying or repairing missing evidence. There is no second history store, route, status, receipt, or journal.

The ordinary `record()` deliberately clears supplied comparisons and derives a new comparison from up to 100 recent reports. Imported admission instead preserves the validated historical comparison, including it in immutable equality checks, while sharing report-row decoding and insertion with the same owner. It never opens a nested transaction or recalculates history against the destination's recent reports.

## Implementation boundaries and evidence required

The converter is in `core/app/src/import/history.rs`, with immutable inputs carried by the existing instance preparation. One concrete `BoundHistory` binds both owner batches to the actual reserved destination. Initial and recovered publication in `instances/create.rs` insert them inside the final metadata transaction. The existing instance-import route and retained task own completion and response-loss retry; no additional transport contract is introduced.

Preview converts source-wide history once locally before selecting each instance's
records; malformed history still leaves a readable preview with ordinary import
unavailable. Accepted import revalidates source authority separately. Independent
review confirmed atomic publication and durable ID binding after this change.
Report detail uses the report owner's identifier validation; live-session
status/kill/log/command endpoints retain UUID-only session admission.

Passing checks include schema-3 neutral fields and comparison preservation,
unknown/contradictory outcomes, source drift, immutable conflicts, final-transaction
rollback, ready/published recovery, selected-instance binding, and completed retry
without recopy. The composed HTTP journey verifies list/detail identity, repeated
import, unchanged source fingerprint/bytes/mtime, no launch intents or sessions,
and refusal to use imported report IDs as live-session controls. Source review
also corrected sensitive path admission and legacy optional comparison dimensions.

## Terminal suite and driver history

The converter reads the actual persisted contracts:

- `profile/benchmarks/suites/<suite_id>.json`: `axial.launch.benchmark.suite`, schema **2**, with instance/mode/timestamps and runs containing index, descriptors, benchmark ID, optional session/launch timestamp, and state.
- `profile/benchmarks/suite-drivers/<driver_id>.json`: strict **unversioned** `BenchmarkSuiteDriverStatus`, with suite/mode/state, interval, counts, pending/last indices, active/last session IDs, error, and timestamps. Do not invent a schema envelope.

`PreparedBenchmarkImport` reuses `benchmark_suites`, `benchmark_drivers`, and existing read routes. Only validated terminal suites (`failed`, `stopped`, `exited`, `completed` runs) and drivers (`complete`, `failed`, `stopped`, `interrupted`) are admitted. Source-scoped suite/driver/session IDs retain exact relationships. One source-wide 1,024-record / 64 MiB bound covers all three history kinds; per-record owner bounds remain enforced without truncation.

Historical suites and drivers carry `historical: true`, have no launch intents or runnable driver requests, and retain source timestamps, descriptors, sparse indices and evidence. The absent marker defaults to false for existing operational records; reserved historical ID prefixes cannot be used by live creation. All suite/driver mutation, private save and startup-resumption paths reject historical authority. Existing driver rows show a read-only label and disabled mutation actions; retained pending indices are labeled “Recorded pending.” Qualification remains a read-only GET. No new screen is added.

Validation follows the original schema and recorded time, not today's exact matrix or a later snapshot. It accepts legacy `+` descriptor characters, compares AutoSi/Millis timestamps as instants, preserves a terminal driver's older last-session evidence and a never-reserved pending index, and handles both original outcome writers without relabeling. Conflicting session ownership or unwitnessed last-session relationships remain blocked.

Passing checks cover immutable retries, conflicting/missing evidence, transaction rollback across all owners and instance visibility, ready/published restart readmission, exact selected-instance binding, unchanged source bytes/mtime/fingerprint, existing HTTP reads and refused mutations. Actual server reopen creates no launch intents or sessions and retains NULL driver requests. Frontend source/test typing and 369 tests pass (`suites-frontend-current.log`), including historical read-only rows and unchanged ordinary Resume. Independent architecture and adversarial source reviews found no remaining blocker in this bounded slice. Native migration interaction is still unverified.

Nonterminal runs/drivers, running/degraded report snapshots, unknown reports without terminal evidence, missing/orphan/conflicting references, lossy stage-bound conversions, and malformed/unsafe/oversized/unsupported/source-changed records remain preserved and blocking. Exact queued handoff snapshots have the history-only path above, not execution authority. Historical preservation does not by itself establish full history parity or cutover readiness.

## Terminal Performance commands

Strict schema-10 operation journals admit only genuinely terminal per-instance Performance intents. The original 8 MiB/128-entry bounds, envelope, ownership, timestamps, sequence, requested/actual action and prepared proof are validated before conversion. Empty envelopes are not exempt from validation. Original historical artifact bounds and references remain historical evidence, not current filesystem authority.

The existing `performance_commands` table stores these records under a private `historical` discriminator. Existing status responses expose optional typed history; live responses omit it. Source-bound IDs and the reserved destination UUID are verified by the same final instance transaction and completed-replay path. Live recovery and pruning cannot resume or discard imported history. No new table, queue or mutation owner exists.

The current checkpoint includes real Managed copy/publication/reopen, exact immutable replay, missing/conflicting evidence, ignored INSERT rollback, live-history ordering/pruning and unchanged live wire tests. All 56 focused import tests pass (`history-import-focused.log`). Two initial reader failures were a missing rules migration in the import fixture, corrected locally without relaxing production checks.

## Global rules refresh history and signed cache

Terminal global RefreshPerformanceRules commands retain their actual source sequence and outcome without fabricated instance IDs, timestamps or live authority. Source/domain sequences remain u64; receipt fields are canonical decimal strings so values above JavaScript's exact integer range survive responses, reopen and replay.

The existing rules owner admits cache bytes only under destination-configured policy and signature trust. Absent cache adopts, exact bytes acknowledge and differing bytes conflict. One immutable completed fact supports source-independent status and edit-preserving replay without restoring an old cache after later refreshes. Receipt-aware instance preparation, final publication, recovery and completed retry verify that exact fact.

Authenticated import/status routes, rules-aware native preview outside the selection lock and the existing dialog action are integrated. Tests exercise source preservation, trust failures, storage rollback, dropped waiters, offline status, full-width sequences and uncertain-response reconciliation. This is implementation/behavioral evidence, not real native migration acceptance. Nonterminal commands, unknown effects, unsupported records and automatic-resume handoffs remain explicit full-cutover blockers, not scope cuts.
