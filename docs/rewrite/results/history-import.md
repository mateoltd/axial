# Terminal history import assessment

Status: bounded historical reports, suites, drivers, per-instance Performance commands and global rules refresh history integrated. Current verification checkpoints are in [integration evidence](integration.md); earlier authenticated report/suite/driver import/read/retry/restart journeys remain in `.rewrite-logs/suites-api-current.log`. Full profile cutover remains unavailable.

## Ordinary profile preservation

The source registry's `last_instance_id` is distinct from the browser's saved route. First publication of that exact selected instance may fill an empty destination selection within the existing live-publication transaction. Existing destination choices win; completed replay never restores a later changed or cleared selection. No launch timestamp or revision is fabricated. Source-readmitted recovery uses the original fingerprint and reserved destination identity; ignored completion writes cannot acknowledge publication.

Legacy startup writes `state/persisted-state-rejection-streaks.json` even with no rejected records. Its only consumer supplies discretionary Guardian repair eligibility, while accepted effects are separately journaled. A valid strict v1 snapshot therefore no longer blocks ordinary import. Original raw bytes must meet the legacy 32-KiB/eight-entry, canonical identity, sorted unique entry and startup-count bounds. Capture, fingerprinting and source revalidation still include the record; no eligibility or filesystem authority is imported. Invalid snapshots, unsafe files, unknown state and accepted-effect journals retain their blockers. Known-good activation snapshots are a separate existing excluded cache; their bytes do not grant destination launch authority.

Successful launches also write `guardian-user-mod-witnesses.json`, whose only behavioral consumer adds Guardian diagnosis evidence. Exact valid v1 records no longer block ordinary copying. Original raw typed validation preserves the 2-MiB/1,024-record/1,024-entry bounds, strict instance ordering, nondecreasing digest/size/time tuples, full u64 values and bounded RFC3339 dates. Equal entries and stale instance witnesses remain valid as in the source schema; duplicate JSON fields and unknown fields do not. Source capture, hashing and revalidation remain mandatory. Five grouped checks first reproduce three valid-record failures, then pass real copy/reopen/replay without Guardian publication, inclusive bounds, malformed bytes, changed source and retained-effect refusals (`import-user-mod-witness-{red,green}.log`). The unresolved-operation test uses a valid running journal and requires capture success before its blocker assertion. No destination Guardian state, UI or wire changes; full cutover remains unavailable.

Ordinary music use creates `music/` with the fixed `vapor-halo.mp3` and `sublunar-hum.mp3` cache files. An empty directory or exact direct files within the original32-MiB per-file bound no longer blocks independent instance import. Zero-byte cache files match the source owner's admission behavior. Files remain captured, hashed and source-fenced; unknown names, aliases, nested directories, scratch records, links and oversized files still block. Music preferences retain their existing settings conversion, but this recognition neither copies the cache nor promises offline cached playback in the destination.

A strictly validated saved wardrobe also no longer blocks copying an unrelated instance. Preparation reuses the existing immutable inventory result from the owning skin converter instead of decoding the same PNG batch per instance. The public saved-skin obligation and full-cutover refusal remain; only explicit skin import publishes destination wardrobe records. Malformed, extra, unsafe or changed source records stay refused. Six regressions first reproduce the availability failures, then pass real copy/reopen/replay, independent destination state and source-drift checks (`import-ordinary-records-{red,green}.log`).

## Queued restart handoff preservation

An exact legacy `interrupted` driver with error `driver automatic resume queued after restart` may be retained as immutable history in an independently copied instance. All existing source schema, time, count, run/report relationships, bounded input and source/destination verification still apply. Malformed or active-session records remain refused. The source record, original error and retained-obligation preview are unchanged; `cutover_available` remains false.

This is not terminal-process evidence: the [legacy driver owner](../../../legacy/apps/api/src/state/benchmark_suite_drivers.rs) clears `active_session_id` in `admit_loaded_driver` while queuing replay, and `apply_driver_transition` retains the handoff obligation. Separate verified copying does not establish predecessor settlement.

Explicit Resume now accepts compatible queued history through the existing benchmark owner, creating separate destination work without consuming or settling the predecessor obligation. Source history alone never starts execution: import and ordinary reopen retain NULL driver requests and zero launch intents. Exact destination, current-plan, report, exclusion and successor-link checks remain mandatory. No schema, route or scheduler is added; full cutover remains unavailable.

The earlier “automatic cross-profile handoff” label conflated retained same-profile restart with a new transfer protocol. [Legacy startup](../../../legacy/apps/api/src/application/launch/benchmark.rs) resumes its own drivers; [replacement startup](../../../apps/api/src/lib.rs) already invokes its normal-driver owner. No legacy cross-profile transfer entrypoint was established. The [import contract](../work-packages.json) requires unresolved effects to stay preserved/unavailable, not to acquire execution authority in a second profile. Same-profile automatic restart still needs its own runtime evidence and bounded refusal/overflow corrections; this distinction does not waive source obligations or establish full parity.

Coverage includes both shared-suite driver orders, canonical all-pending/mixed plans, missing or incompatible destination evidence, immutable successor replay and lost responses. All 26 benchmark tests, nine composed import tests and four real-process Resume journeys pass (`queued-resume-{core-green,api-import,process}.log`). The process journeys reopen before Resume to prove no automatic execution, then execute only pending destination runs, reconcile a discarded response, reopen/replay the same successor and preserve source bytes/mtime and inherited reports. These use fixture Java/game processes, not gameplay. The two new queued cases failed specifically at continuation eligibility before the correction (`queued-resume-red.log`).

The `aad07cd0` macOS ARM64 debug package now passes real native picker/import/normal-Quit/reopen for the generated all-pending fixture. One historical interrupted driver and suite remain durable, both runs pending, zero launched runs and NULL executable request; no launch, report or install-queue rows appear. Both native processes exit0 normally, the restored interface shows the imported instance/world still requiring Install, and source/repository/sibling canaries remain unchanged (`queued-native-acceptance.md`, `queued-native-{metadata,imported,restarted}.png`). This closes the earlier picker handoff, not actual installation or developer-UI Resume. The packaged frontend deliberately omits that lab; continuation acceptance needs the normal development frontend.

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

Authenticated import/status routes, rules-aware native preview outside the selection lock and the existing dialog action are integrated. Tests exercise source preservation, trust failures, storage rollback, dropped waiters, offline status, full-width sequences and uncertain-response reconciliation. This is implementation/behavioral evidence, not real native migration acceptance. Nonterminal commands, unknown effects, unsupported records and unsettled source work remain explicit full-cutover blockers, not scope cuts. Automatic execution transfer between profiles is not an established legacy requirement.

## Successful install/content history

Both Generic dispatches now distinguish rules refresh from successful InstallVersion and ModifyInstanceContent records. The strict original numeric codecs retain duplicate-field rejection and all thirteen full-width content counters. The install owner validates the actual producer shapes: Vanilla committed publication, ordered loader base/child checkpoints, and content's Downloading terminal with typed metrics. It retains original target projection and structured-token rules separately; typed publication and loader identities use their existing codecs. Readback reuses this same validator.

`install/history.rs` owns immutable evidence in `install_history`. The live queue cannot truthfully store it: these records contain no executable request, acceptance timestamp or destination installation authority. Version records remain source-global; content binds only to its recorded legacy instance and actual reserved destination UUID. Preparation shares immutable global records and checks the stored-envelope batch budget before preview eligibility. Initial and recovered instance publication insert history atomically; completed retries verify exact evidence without repairing missing rows. Affected-row checks and final whole-batch readback reject ignored or rewritten writes.

Authenticated `GET /api/v1/install/history?instance_id=<uuid>&after=<history-id>` reads through the instance owner's completed/live mapping, not caller-supplied source identity. Pages contain at most 32 records of at most 256 KiB each; two indexed, individually bounded source-global/exact-instance branches avoid scanning unrelated history. Public sequence and counters are decimal strings; no fabricated timestamps, readiness, queue controls or execution are exposed. Generated public types share the Rust DTO owner; existing UI is unchanged.

Full local verification passes 909 application and 129 API tests, including nine owner checks, four importer checks, initial/recovered publication rollback, completed replay and authenticated source-free reopen. Generated wire types, source preservation and zero live execution are checked; counts and initial failures are in [integration evidence](integration.md). Failed/cancelled records, recovery-bearing effects, histories without a current source instance and full profile cutover remain explicit follow-ups. Successful-history support neither settles predecessor effects nor transfers execution authority.

## Pre-worker content initialization failure

The same validator now recognizes the exact reservation-cleanup record: `Failed`, not `Cancelled`, with `failure_point=content_initialization_cancelled`, one `content_progress_initializing` Failed step, exactly `install_phase:initializing`, `install_done:true`, `install_error:true`, and no metrics or changed target. Original cleanup verifies the planned record; worker hand-off disarms it before content execution ([cleanup](../../../legacy/apps/api/src/application/install.rs), [producer](../../../legacy/apps/api/src/application/install/operation.rs), [regression](../../../legacy/apps/api/src/application/install/tests.rs)). Ordinary waiter cancellation keeps accepted queued work alive and is not this case.

History retains the original failure reason through an optional public field, omitted for successes. Existing source persistence, schema, immutable binding, atomic publication, replay verification and reads are unchanged. The actual composed HTTP regression first fails at ordinary import availability (`content-init-history-red.log`). Importer/owner checks cover mixed histories, exact wire evidence, source drift, malformed effects and persisted corruption; the same authenticated HTTP journey checks source-free reopen and zero live execution. Final verification is recorded in [integration evidence](integration.md).

Other worker failures/interruption do not prove settled file effects. Initialization records containing typed Guardian terminal evidence require separate exact handling. These remain full-parity obligations, not implicit exclusions.

## Settled failed-install rollback history

The existing history validator now admits exact failed Vanilla rollback, loader-base rollback, and loader-base commit followed by child rollback. Each requires prior recovering progress, version-bound publication evidence, a `RollingBack`/`Completed` checkpoint with rollback `Applied`, and the later exact `install_progress_error` failure terminal. The enclosing operation keeps its original `Failed` outcome and `NotApplicable` rollback. Rollback checkpoints contain no activation contract; a preserved base commit still requires its original contract.

The [original sequence owner](../../../legacy/apps/api/src/application/install/operation.rs), [Vanilla settlement](../../../legacy/apps/api/src/application/install.rs) and [loader settlement](../../../legacy/apps/api/src/application/install/loader.rs) write rollback before native acknowledgment and defer terminal failure when acknowledgment is unresolved. Therefore checkpoint-only records remain blocked. No further progress or publication is accepted between rollback and terminal failure. This preserves recorded settlement, not a claim that no files ever changed.

Original Guardian annotations are strictly decoded before JSON normalization, checked against their closed source registries, bounds, cross-references and historical retry-memory target/time contract, then omitted from public history. They supply no compensation proof or active Guardian state. Diagnostic-only and absent annotations remain valid. The same immutable owner, raw codec, persisted readback, source fences, atomic instance publication and replay checks apply; no schema, UI, queue or recovery owner is added.

The four importer regressions first produce three intended availability/preparation failures and one refusal pass (`rollback-history-red.log`). Final consumer verification is recorded in [integration evidence](integration.md). Failed installs without exact compensation evidence, other cancellation/recovery shapes and full cutover remain open.

## Remaining history and availability boundary

Do not broaden ordinary import from a terminal label alone. An [unreaped loader processor](../../../legacy/core/minecraft/src/loaders/bound_processors.rs) is mapped to the ordinary failure writer by [the loader strategy](../../../legacy/core/minecraft/src/loaders/strategies/common.rs) and [application owner](../../../legacy/apps/api/src/application/install/loader.rs). Native publication mismatch can also refuse before reconciling another operation's intent. Progress persistence is best-effort and may stop while work continues; diagnosis labels are lossy classifications. Thus no-checkpoint or base-only `Failed` records do not prove the affected source obligations settled.

The current `PreparedSourceHistory.supports` feeds instance availability, including the unresolved-operation waiver. Keeping full cutover false does not make that waiver safe. Broad failed-content records have the same problem: normal terminal error can follow worker panic, and scratch absence is not universal cleanup proof. Existing bounded instances-parent capture already detects nonregistered `.axial-pack-*` leftovers; do not add a duplicate scan to compensate for misreading the later inventory pass. No broader admission landed.

Next work is source-global evidence independent of live instances: currently validated InstallVersion history is attached only to per-instance batches and read through current mappings, so an account/settings profile with no instances cannot preserve or read it. Reuse existing metadata completion and history ownership without fabricating instances; keep accounts/settings usable when other history is unsupported, preserve old receipt semantics and later destination edits, and leave unresolved-effect availability fenced. Archived per-instance history, installed-selection reconstruction through normal destination downloads, source-bound preference completion and the complete cutover rehearsal remain separate required work.

Proposed next-slice contract, not yet implemented: optional strict global batch preparation; install-owner global binding with no instance ID; a bounded nullable count/digest proof on the existing metadata receipt via a new migration. Missing proof means incomplete/unsupported, not verified empty. An explicit metadata POST may complete an old exact-source receipt's history without reapplying accounts/settings or changing its original revisions; subsequent replay verifies immutable evidence and never repairs missing rows. Existing metadata import availability must remain unchanged when history is unsupported. A source-free authenticated history read resolves source identity through the stored metadata receipt, reusing existing page/record/batch bounds rather than accepting caller source authority.

First regression must compose a zero-instance source, actual metadata import and absent global evidence. Verification must include lost response/reopen, old-receipt upgrade with preserved destination edits, existing identical/conflicting instance-published rows, unsupported history without metadata regression, source drift, pagination/bounds and atomic ignored/rewritten publication. Require affected-row and final readback proof. Add a narrow final-settings-write verification seam only if a composed regression demonstrates the need; no speculative transaction framework.
