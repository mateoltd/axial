# Retained UI wire parity review

## Publication-read refusal diagnostics

2026-10-07, changes against `9da0128c` on `main`. The existing preflight owner preserves the exact publication-read acquisition refusal: native `WouldBlock` adds blocking `incomplete_install` with the retained wait message; other unsafe acquisition failures add blocking `installed_versions_degraded` with the retained safe-inspection message. Top-level `instance_busy`/`library_unavailable` remains unchanged. Facts use the existing diagnostic constructor after final target/instance/account/settings fences. No provider, repair, scanner, state owner, journal or UI change. Rust-owned generation adds only those two reason variants.

The genuine Ready HTTP tracer holds the actual publication guard. Initial RED joins101/2.91s at omitted `status:"ready"` (`preflight-publication-red.log`); GREEN joins0/2.30s. That first RED preceded the later preservation assertions, so it does not prove them. The corrected tracer defers all assertions until ordinary API/provider cleanup, then checks unchanged queue/provider requests/protected metadata and JAR bytes, empty sessions/reports and restored Ready before the diagnostic. An independently held target exclusion remains generic busy with no readiness facts. A real non-directory publication-lane conflict is preserved then restored. Expanded RED joins101/2.65s at omitted unsafe-inspection status after those controls; GREEN joins0/3.07s (`preflight-publication-unsafe-{red,green}.log`). Final fixture-only changes add top-level refusal assertions and shutdown-panic retention; their execution is covered by the subsequent full run, not reassigned to that GREEN.

Final Standards and Spec reviews clear the three frozen files: coordinator `5ba48a9e`, HTTP tracer `20b583cc`, generated reason `5235d659`. Fresh serialized affected libraries pass2,004 parent tests: API126/eight existing helper ignores/124.29s, app834/ten ignores/198.91s, Minecraft1,044/zero ignores/225.68s, zero failures, joined0; nested child summaries are excluded (`preflight-publication-full.log`). This includes the final tracer. Desktop passes83/one helper ignore/5.98s, joined0; the initial wrong `--lib` selector runs no tests and remains recorded separately (`preflight-publication-desktop-target-error.log`). Rust-owned generation/equality and scoped formatting pass; frontend531/one existing TODO, typing/semantic lint, build and independent generation verification join0, retaining `239a78614d6e` and budgets (`preflight-publication-{wire-generate,wire-check,format,frontend,types,semantic,build,generation}.log`). No current hosted result is claimed yet. These facts cover fresh publication acquisition, not cached/bulk revalidation, whole-library degraded scans, ordinary-launch scan admission, create-view/create scan gating or native/installed/gameplay/full parity.

## Actual browser default-runtime recovery

2026-10-07, unchanged product source `391bd32f` on `main`. Ordinary debug API SHA256 `a8921bf4d87cf7c8b9719ab02b60d12c1ff9a3662c50d03a8762b9f183b9bc66` embeds verified frontend `239a78614d6e`. A fresh canonical disposable profile installs real Vanilla1.20.1 and Java17 through ordinary providers with offline RuntimeParity. No fake Java, test-only endpoint, injected readiness, credentials or UI edit. The setup observer stops at `/instances` because its source expects null for the omitted `install_queue`; readonly reconciliation finds exactly one Ready target. Corrected baseline reads pass without repeating installation/account/configuration/creation mutations. The failed observer and original reviewed helper hash remain recorded.

Initial ordinary API shutdown joins0. Installed metadata and `/java` bind the exact generated gamma component; preserve it outside the profile, with no sidecar/destination conflict. Reopen the same executable: standalone `managed_runtime_missing`/`recoverable`, enriched Launch and zero managed runtimes/sessions/reports pass; actual filesystem ENOENT remains after these reads and browser navigation. Actual IAB tab7 shows Ready/Launch. One visible Launch click reacquires Java and reaches Playing/Stop. The public witness records session `02fd936b-4e80-497f-aced-1977aed2aa70`, revision4, actual PID81254, live process and observed boot, with zero reports.

One visible Stop click returns Ready/Idle/Launch and one stopped notice. Public terminal evidence retains tree settlement/output drainage and exactly one schema4 stopped/launcher-stopped report, observed boot3708ms. Ordinary shutdown joins0 before a third API process reopens the same profile/executable. A fresh browser document shows Ready/Launch; public reads preserve the exact report and regenerated managed listing with no live sessions. This is cold reopen, not retained-document reconciliation. Final ordinary shutdown joins0; independent exact-process/listener absence checks pass. Three selected immutable trees (`assets`, `libraries`, `versions`) preserve all3,630 files/732,946,528 bytes and their recorded entry identities/metadata/content hashes through Stop and final shutdown. No whole-profile immutability is claimed; normal instance/log/cache/report effects are outside that witness.

Evidence under `.rewrite-logs/`: `runtime-readiness-ui.md`, `runtime-readiness-{prepare,baseline,missing}.log`, `{running,stopped,reopened}.json`, `files-{before,stopped,final}.json`, three `api-{initial,reopened,final}.log` and durable `{ready,playing,stopped,reopened}.jpg` screenshots, all with the `runtime-readiness-` prefix. Preparation helper final `0d0fdfde`, public observer `b45b5805`, inventory witness `12edb08b`. The initial reviewed helper `2f666f6d` has the omitted-field expectation mistake; its corrected read-only baseline is not a replay of setup. Native inventory reported the Mac locked. No game menu/narrator/world creation or visible gameplay was verified. Native packaging, authenticated accounts and remaining installed/full-parity gates stay open.

Hosted [run37599483279](https://github.com/mateoltd/axial/actions/runs/37599483279) passes both jobs at exact `391bd32f00edb5a669f76981a99a4eedc4ad2611`, confirmed by joined watch and independent matching SHA/job query (`missing-default-runtime-readiness-ci-{watch.log,current.json}`). This verifies committed source checks, not native/installed acceptance.

The same frozen executable/profile is tried again while native automation is available. Actual browser Launch/Playing/Stop/Ready, real session `63cbe55d-4090-4d51-a658-1651e9a22430`, boot3353ms, exact new stopped report and prior-report/runtime preservation pass; API joins0, exact processes/listener are absent, and all selected immutable files remain byte-identical. The native inventory omits the Java game, and neither Minecraft nor java resolves through its app selector. No native menu/world/gameplay acceptance is established. The snapshot checker initially confuses phase with outcome; corrected assertions require phase `exited`, outcome `stopped`. Failure evidence remains retained without UI replay. Detailed scope and logs are in `.rewrite-logs/runtime-readiness-native-ui.md`. This repeats browser startup/Stop at `391bd32f`, not the later publication-diagnostic source. Hosted [run37602541183](https://github.com/mateoltd/axial/actions/runs/37602541183) passes both jobs at documentation checkpoint `9da0128c`, with joined watch and independent exact SHA/job query (`runtime-readiness-evidence-ci-{watch.log,current.json}`).

## Recoverable default-runtime readiness

2026-10-07, working changes against `805fb325` on `main`. The existing provider-backed journey now asserts standalone readiness and the actual instance enrichment used by Play before its ordinary acquisition. Its genuine RED reaches the omitted diagnostic status (`Null` versus `ready`) with `runtime_unavailable`/`launchable:false`, after successful acquisition/child Stop, protected-byte checks, explicit/corrupt refusals, no-provider readiness reads and joined cleanup (`missing-default-runtime-readiness-red.log`, joined101/3.74s). The failure profile is retained. Initial GREEN passes, joined0/3.46s (`missing-default-runtime-readiness-green.log`): literal `managed_runtime_missing`/`recoverable`, launchable readiness and enriched Launch action, then real fixture acquisition/process/Stop. This is not actual browser/native gameplay acceptance.

The runtime cache retains a read-only absence proof with the original root revision and exact canonical/staging/quarantine namespace checks. Revalidation never adopts newer evidence. Existing canonical directories still use strict selection, even with sidecars; failed admission, aliases, wrong types, explicit selections and probe disappearance are not recoverable absence. The existing preflight owner retains the proof through its final target/settings/bundle checks. Shared option validation checks configuration without fabricating a Java probe; runtime-dependent preset/GC/version/architecture checks still run during actual preparation. No new scanner, journal, state owner or UI policy. Rust-owned generation adds only the existing reason/severity enum variants; generated equality passes, joined0 (`missing-default-runtime-readiness-wire-{generate,check}.log`). Five public native controls pass, joined0/0.60s (`missing-default-runtime-readiness-native.log`), including independently observed namespace-revision change, preserved conflicts/aliases and replaced-root refusal.

Expanded HTTP controls initially fail after joined cleanup because the fixture expects raw JVM arguments in a deliberately redacted public instance (`missing-default-runtime-readonly-guards.log`, joined101/3.96s); the profile is retained. The narrow test correction captures/restores the private setting through the actual registry and requires public arguments to stay empty. Corrected replay passes one tracer, joined0/4.17s (`missing-default-runtime-readonly-guards-corrected.log`): missing-default readiness/enriched Launch, invalid instance arguments yield `plan_rejected` and blocked enrichment, explicit global component absence retains blocking `java_override_missing`/global origin, and reads make no provider requests or runtime changes. Existing actual Play/Stop, same-size corruption, protected-byte, restored-directory identity and settlement controls remain.

Current scoped checks pass139 native runtime/zero ignores,136 application launch/two existing helper ignores and531 frontend/one existing TODO, zero failures, joined0 (`missing-default-runtime-readiness-{runtime,launch,frontend}.log`). TypeScript and scoped Rust formatting pass, joined0 (`missing-default-runtime-readiness-{types,format}.log`). These are not the preceding checkpoint's full affected-library or desktop checks. Final Standards: no documented violation or actionable smell. Final Spec: no concrete blocker across ten frozen source/generated files. Frozen pins: HTTP `9c2b27cc`, coordinator `3b77042d`, plan `c9b77b06`, discovery `666be682`, native install `c954827d`, layout `1120c74a`, exports `17b80703`, tests `7544f601`, generated reason `b243c868`/severity `5368fd2c`. Actual browser Play, native/installed acceptance, publication-time conflict injection and postclaim cancellation remain open.

Full serialized affected-library verification at those pins passes2,003 parent checks, zero failures, joined0 (`missing-default-runtime-readiness-full.log`):125 API/eight existing helper ignores,834 application/ten ignores and1,044 Minecraft/zero ignores. Nested subprocess summaries are excluded. Fresh desktop composition passes83/one existing helper ignore, joined0/5.52s (`missing-default-runtime-readiness-desktop.log`). These results are not reassigned provisioning evidence. Actual browser acceptance remains pending.

Fresh frontend build and independent generation verification pass, joined0, unchanged `239a78614d6e` within retained budgets (`missing-default-runtime-readiness-{build,generation}.log`). The ordinary debug API build passes, joined0/27.72s (`missing-default-runtime-readiness-api-build.log`); no test-only provider hook or fake Java is requested. Building this executable is not browser or installed acceptance.

## Ordinary default-runtime provisioning

2026-10-07, working changes against `17340bea` on `main`. One real provider-backed backend tracer installs a version normally, joins shutdown, removes only its generated default Java component, reopens the current-app profile and submits ordinary Play. The first attempt fails on a fixture mistake: queue epochs/revisions legitimately reset on reopen (`missing-default-runtime-play-red.log`, joined101/2.63s). Corrected checks compare stable empty content across reopen and the complete queue within the reopened process. The intended RED reaches HTTP409 `runtime_unavailable`, “The selected Java executable is missing,” after protected-byte and joined cleanup checks (`missing-default-runtime-play-boundary-red.log`, joined101/2.27s). Preflight and the instance Play action are also blocked at that checkpoint.

Initial backend GREEN passes, joined0/4.09s (`missing-default-runtime-play-green.log`): fresh provider catalog/manifest/file responses, exact runtime bytes/full-tree verification, real fixture child, ordinary Stop and settled report, unchanged install status/queue and protected game/save bytes. Failure evidence is preserved. This is fixture-process provisioning, not real JVM/gameplay or installed acceptance.

The existing runtime module now prepares only an empty override and a genuinely absent recognized default component. Explicit selections, damaged components and admission/probe failures do not fall back. The existing component mutex and stage/publication implementation are reused. Missing-only mode refuses conflicting canonical/staging/quarantine entries before effects and forbids displacement/rotation through publication; a matching concurrent canonical requires exact source verification. Native source/tree receipts feed the application's existing owned probe and launch receipt. Cancellation signals the existing before-publication control and awaits the same producer through cleanup/settlement. Move-attempt failures retain effect classification; missing-only failures do not invent quarantine ownership. Native obligations remain in the retained cache root, settled by existing shutdown only after producers join. No new journal, coordinator, state/schema/wire or UI change.

Expanded HTTP GREEN passes, joined0/3.19s (`missing-default-runtime-play-guards.log`): existing same-size Java corruption refuses without repair; global explicit managed-component selection refuses absence without fallback. Both preserve provider request counts, sessions/reports, protected files and restored runtime/configuration. The explicit control restores the same directory identity. Existing application selection/probe checks pass nine, joined0/0.80s (`missing-default-runtime-app.log`). Three new native guards initially pass, joined0/1.18s (`missing-default-runtime-native-guards.log`); after formatting only their new code, the complete native runtime suite passes134, zero failures/ignores, joined0/9.35s (`missing-default-runtime-native-runtime.log`). This includes canonical/staging/quarantine conflict preservation, concurrent publication/reuse and cancellation at the existing preclaim gate, with lease release and cache settlement. These initially-green controls are not manufactured REDs. Publication-time conflict injection and postclaim cancellation remain unexecuted.

Final Standards review: no documented breach or actionable smell. Final Spec review: no concrete backend-slice blocker across nine frozen source files. Production pins are API `a789e255`, coordinator `a4cd5026`, discovery `f229e7f6`, model `7cba33f6`, ensure `3897ccbb`, install `35208cf5`, exports `17dea132`; final HTTP `dac6f058` and native tests `3deac9cc`. Scoped formatting and diff-check pass (`missing-default-runtime-format-final.log`); the earlier new-test formatting refusal remains recorded separately.

Full serialized affected-library verification passes, joined0 (`missing-default-runtime-full.log`):125 API/eight existing helper ignores,834 application/ten ignores and1,039 Minecraft/zero ignores, zero failures. The1,998 parent checks exclude nested child summaries. Desktop consumer checks pass83/one existing helper ignore, joined0 (`missing-default-runtime-desktop.log`). Rust-owned generated equality passes, joined0 (`missing-default-runtime-wire-check.log`); no public contract changed. Pinned frontend checks pass531/one existing TODO and TypeScript, joined0 (`missing-default-runtime-{frontend,types}.log`); no new frontend build is claimed. Recoverable read-only preflight and visible Play availability remain the next slice; they are not closed by raw POST success. Native/installed/full-parity acceptance remains open.

Hosted [run37595072272](https://github.com/mateoltd/axial/actions/runs/37595072272) passes both jobs at exact `805fb3253d16c680c911977b3b5f38ba52d36ae4`, confirmed by joined watch and independent SHA/job query (`missing-default-runtime-ci-{watch.log,current.json}`). The normal `main` push preserves history. It verifies committed backend provisioning, not subsequent read-only readiness changes or native/installed/full parity.

## Observed simultaneous artifact damage

2026-10-07, changes against `21adb150` on `main`. Diagnostic inspection collects at most seven supported distinct artifact reasons in the existing installed-file verification pass. Ordinary inspection retains first-refusal behavior. Both results share retained inventory evidence: all present compact revisions, including corrupt files, and one original absence witness per missing reason. Every descriptor and available metadata is still checked; invalid recorded evidence, failed admission/read/symlink checks, corrupt metadata and ordinary-object damage remain generic refusals and suppress collected facts. No repair, second scanner, state/schema/wire or UI change.

The genuine installed-provider library/client removal goes RED with only `libraries_missing` versus two literal expected reasons (`preflight-multiple-missing-red.log`, joined101/4.00s). Initial composed GREEN passes, joined0/6.32s. The final expanded tracer also passes, joined0/5.07s (`preflight-multiple-missing-guards.log`): later corrupt metadata suppresses earlier library diagnostics, and simultaneous logging/library absence produces exactly one `libraries_missing` reason. Exact restoration, unchanged queue/sessions/reports, no repair, canary/link preservation and joined application/provider cleanup precede diagnostic assertions; subsequent privacy controls pass.

Adversarial review finds and corrects two production gaps. Native absence now retains ancestor portable-name admission through the existing parent-chain cursor, without updating stored revisions; real invalid ancestor namespace supplies separate RED/GREEN. Failed final damage revalidation now stays in the result and passes the existing target/instance/settings fences instead of returning early. Diagnostics capture account/config/resource inputs after blocking revalidation, with no later await. The composed ordering race has no existing deterministic test seam and is not claimed executed. No new hooks or coordination owner are introduced.

Current safety coverage passes four producer checks and21 native checks, joined0 (`preflight-multiple-{artifact,native}-guards.log`). The native total is four absence controls,16 existing batch controls and one unrelated existing continuation. Controls cover restored absence, same-byte corrupt-file replacement, later malformed inventory versus ordinary first refusal, replaced parent with both trees preserved, actual ancestor revision changes and benign churn; distinct aliases remain conditional on filesystem support. They are additional initially-green coverage, not manufactured REDs. Final Standards/Spec reviews clear all six files: artifacts `ccaa4c73`, queue `3e787087`, coordinator `97fb2573`, native `d00deec6`, exports `9974d264`, HTTP `f5fba618`.

Scoped formatting/diff-check and Rust-owned generated equality pass; public reason generation remains `dba06e95`. Pinned frontend checks pass531/one TODO and TypeScript, joined0. The first full affected Rust run stops during compilation on actual disk exhaustion, joined101, without a test failure (`preflight-multiple-full.log`). Scoped Cargo cleanup of the three affected packages joins0 while preserving profiles/bundles/evidence (`preflight-multiple-clean.log`). The serialized retry joins0:124 API/eight helper ignores,834 application/ten helper ignores and1,036 Minecraft checks pass, zero failures (`preflight-multiple-full-retry.log`); nested helper summaries are excluded. Desktop consumer checks pass83/one helper ignore, joined0 (`preflight-multiple-desktop.log`). No new frontend build or native/installed acceptance is claimed. Publication/runtime/origin and full-parity gates remain open.

Hosted [run37590030543](https://github.com/mateoltd/axial/actions/runs/37590030543) passes both jobs at exact `17340bea8de5a321db45bd8ba17d3689bd80bd47`, confirmed by joined watch and independent SHA/job query (`preflight-multiple-ci-{watch.log,current.json}`). Normal push preserves history. This verifies artifact aggregation, not subsequent runtime provisioning or native/installed/full parity.

## Observed logging-configuration readiness

The scope and open gates below describe this historical checkpoint; simultaneous artifact coverage is recorded above.

2026-10-07, changes against `25ad13f2` on `main`. The existing installed-file verifier classifies validated `assets/log_configs/<single portable filename>` alongside required libraries, matching the retained installer filename contract and legacy `log_config` mapping. Successfully observed absence returns `LibrariesMissing`; measured size/digest mismatch returns `LibrariesCorrupt`. Existing preflight preserves the retained blocking literals. Invalid recorded evidence, failed reads, unsafe paths/symlinks and ordinary asset objects remain generic; ordinary launch refusal, HTTP409 and captured fences are unchanged. No scanner, repair, state/wire/schema or UI change.

The existing genuine-install HTTP tracer binds logging bytes to `/artifacts/log.xml` after proving launchable Ready. Exact removal goes RED at omitted diagnostic status, wrapper101, then GREEN, wrapper0/3.66s (`preflight-log-config-missing-{red,green}.log`). A subsequent vertical slice flips one byte without changing length: its separate RED fails at the same intended boundary, wrapper101; GREEN passes the complete tracer, wrapper0/4.17s (`preflight-log-config-corrupt-{red,green}.log`). Changed bytes are captured after preflight; equality, restoration, external-canary/link identity, generic symlink refusal and unchanged reports/queue/sessions are asserted after joined application/provider cleanup, before intended RED. Later privacy checks pass in GREEN. Direct corruption coverage is same-size damage, not a separate size-mismatch case or simultaneous failures.

Independent Standards/Spec reviews clear verifier `68a6696f` and HTTP fixture `51c43c7c`. Scoped formatting/whitespace and Rust-owned contract equality pass; generated reasons remain `dba06e95`. Serialized focused checks pass one verifier,34 queue,44 coordinator/one existing helper ignore, three routes and83 desktop/one helper ignore, zero failures, joined0 (`preflight-log-config-{artifacts,queue,launch,routes,desktop,wire-check}.log`); nested child results are excluded. Pinned Node24.13.1 frontend checks pass531/one TODO and typing, joined0 (`preflight-log-config-{frontend,types}.log`). No new frontend build or native/credential/signing journey is claimed. Publication/runtime/origin, simultaneous reasons and native/installed/full-parity gates remain open.

Hosted [run37583223808](https://github.com/mateoltd/axial/actions/runs/37583223808) passes both jobs at exact `21adb1506b44afd5a8750d710396eda2fbe93be4`, confirmed by joined watch and matching independent SHA/job query (`preflight-log-config-ci-{watch.log,current.json}`). It verifies the committed logging checkpoint, not the working aggregation leaf or native/installed/full parity.

## Observed asset-index corruption

The scope and open gates below describe this historical checkpoint; later logging and simultaneous artifact coverage are recorded above.

2026-10-07, changes against `40cafa58` on `main`. The existing verifier carries typed `AssetIndexCorrupt` only for measured size/digest mismatch of a validated recorded canonical asset-index file. Failed reads, invalid recorded checksums, unsafe/admission/symlink failures and ordinary asset objects remain generic. The existing preflight owner preserves retained `asset_index_corrupt`/`blocking`, “Asset index is corrupt. Repair this version before launching.” Ordinary launch remains `InstallUnavailable`; install HTTP remains409. No scanner, repair, state/schema or UI change; publication/account/settings/instance fences are unchanged.

The genuine installed-index HTTP tracer flips one byte without changing length, captures the refusal and changed bytes, then continues existing absence/symlink/object/library/client/restoration controls. Preservation and no-effect assertions run after joined application/provider cleanup and precede the intended RED at omitted `status:"ready"`, wrapper101; GREEN passes the same tracer including later privacy assertions, wrapper0,3.98s (`preflight-asset-index-corrupt-{red,green}.log`). Direct damage coverage is same-size corruption, not a separate size-mismatch fixture or simultaneous failures.

Independent Standards and Spec reviews clear all six frozen files: verifier `1ee90ec6`, queue `f08488b7`, coordinator `395e4cde`, route `debe6a51`, tracer `85a1b64c`, generated reason `dba06e95`. Rust-owned generation/equality and scoped formatting/whitespace pass. Current focused checks pass one installed-file verifier,34 queue,44 launch-coordinator/one existing helper ignore, three install-route and83 desktop/one helper ignore tests, zero failures, joined0 (`preflight-asset-index-corrupt-{artifacts,queue,launch,routes,desktop}.log`); the nested one-test child is excluded. Pinned frontend checks pass531/one TODO, typing/semantic lint pass, joined0. Build and independent generation verification pass unchanged `239a78614d6e`/budgets, exit0 (`preflight-asset-index-corrupt-{frontend,types,semantic,build,generation}.log`). Earlier full831 app/124 API results belong to `40cafa58`, not this later source. Logging, bounded simultaneous facts, runtime/publication/origin coverage and native/installed/full-parity gates remain open; no native, credential or signing action occurs.

Hosted [run37581643137](https://github.com/mateoltd/axial/actions/runs/37581643137) passes both jobs at exact `25ad13f288d38e59566c1f64161f861cbb2e39c2`, confirmed by joined watch and matching independent SHA/job query (`preflight-asset-index-corrupt-ci-{watch.log,current.json}`). This verifies the committed index-corruption checkpoint, not later logging changes or native/installed/full parity.

## Observed asset-index absence

2026-10-07, changes against `879ca1dc` on `main`. The existing installed-file verifier distinguishes successful absence observation of a validated recorded `assets/indexes/<portable filename>.json` from generic refusal. Structural classification reuses the existing portable-name codec and the same predicate for index flag parsing; ordinary asset objects cannot become index diagnostics. Standalone preflight preserves retained `asset_index_missing`/`blocking`, “Asset index is missing. Install this version before launching.” It keeps completed `status:"ready"`, `launchable:false`, safe `install_unavailable` and captured memory/origin/resource facts. Ordinary launch remains `InstallUnavailable`; install HTTP remains409. Recorded checksum/path/namespace validation, admission/read failures and existing publication/account/settings/instance fences remain intact. No scanner, repair, persistence owner, schema or UI change.

The existing real HTTP tracer first completes genuine provider installation and proves launchable Ready. Installed index bytes match the actual provider response before exact removal; a separate external-canary symlink requires generic refusal and preserved link identity/target/bytes. After restoring the index, removing the genuine provider asset object separately retains generic refusal. Original bytes are restored before existing library/metadata/client controls. No-repair/no-effect assertions and joined application/provider cleanup precede the intended RED at omitted diagnostic status (`Null` versus `"ready"`), wrapper101; GREEN passes the same tracer including later privacy assertions, wrapper0,3.81s (`preflight-asset-index-missing-{red,green}.log`). This does not establish asset-index corruption or every readiness origin.

Independent Standards/Spec source reviews clear the six frozen files: verifier `cdc985b8`, queue `234b0df8`, coordinator `db5c80c7`, route `794f9400`, tracer `f4441cf5`, generated reason `68efcb79`. Rust-owned generation/equality and scoped formatting/whitespace pass (`preflight-asset-index-missing-{wire-generate,wire-check,format-check}.log`). Full serialized application/API checks pass 831/124, zero failures and ten/eight existing helper ignores (`preflight-asset-index-missing-app-api.log`); a nested one-test child result is excluded. The original shell chain ends101 only afterward, because the desktop `--lib` invocation selects no target (`preflight-asset-index-missing-desktop.log`). Corrected `--bin axial-desktop` passes 83 checks/one existing helper ignore, joined0 (`preflight-asset-index-missing-desktop-bin.log`). Pinned frontend checks pass 531/one TODO, typing and semantic lint pass, joined0. Build and independent generation verification pass unchanged `239a78614d6e`/budgets, exit0 (`preflight-asset-index-missing-{frontend,types,semantic,build,generation}.log`). Current native inventory explicitly reports the Mac locked; no native, credential or signing action occurs. Native and installed/full-parity acceptance remain open.

Hosted [run37580471844](https://github.com/mateoltd/axial/actions/runs/37580471844) passes both jobs at exact `40cafa58ee1a64a81d6b9ddff87d74a1a5c3eb2d`, confirmed by joined terminal watch and independent SHA/job query (`preflight-asset-index-missing-ci-{watch.log,current.json}`). This verifies the absence checkpoint, not subsequent corruption or native/installed/full parity.

## Remaining readiness requirements

The following source-audited gates remain open, not waived by the isolated facts above:

- Publication and degraded scans had source-demonstrated gaps at `21adb150`: retained publication contention/unsafe inspection map to `incomplete_install`/`installed_versions_degraded` ([retained owner](../../../legacy/core/launcher/src/readiness.rs)). The [publication-read slice](#publication-read-refusal-diagnostics) now verifies fresh acquisition facts through genuine HTTP; cached/bulk revalidation remains separate. Whole-library degradation is still discarded by [SetupService](../../../core/app/src/instances/setup.rs) and absent from standalone/ordinary-launch admission. Retained create-view/create scan gating also remains unimplemented. Next controls must damage an unrelated installed version, prove `/versions` degraded, then verify healthy-target preflight/list/detail, creation and ordinary Play refusal, combined artifact facts and restoration. Those controls have not run. An absent ready row and malformed recorded evidence remain distinct from observed file absence.
- Default managed runtime provisioning was a source-demonstrated gap at `21adb150`: retained ordinary preparation calls `ensure_runtime_with_events` independently of Guardian. The [backend slice](#ordinary-default-runtime-provisioning) restores missing-only acquisition through ordinary Play with genuine HTTP RED/GREEN, explicit/corrupt refusal controls and native conflict/concurrency/preclaim cancellation coverage. The [read-only slice](#recoverable-default-runtime-readiness) verifies recoverable `managed_runtime_missing` and enriched Launch availability over actual HTTP without provider effects. [Actual browser recovery](#actual-browser-default-runtime-recovery) now verifies real-provider Launch/Stop/cold reopen. Publication-time conflict injection, postclaim cancellation, broader origins and native/installed/gameplay acceptance remain unestablished; no fabricated probe facts or waived negative evidence.
- Parent-version absence has narrower applicability: source-audited current supported loader activation authenticates complete composed child metadata/JAR and launch consumes that child; inherited runtime/assets do not require a live parent JSON. This does not waive degraded-scan/publication obligations or prove runtime acceptance. A genuine settled loader/base-metadata removal/restoration control remains unexecuted; unmaterialized inherited metadata must still refuse.
- Simultaneous artifact failures were a confirmed gap at `21adb150`: legacy preflight bounds Tier0 facts and deduplicates reasons before Guardian policy ([collection](../../../legacy/apps/api/src/execution/integrity.rs), [mapping](../../../legacy/apps/api/src/application/launch/session/readiness.rs)), while the replacement returned only its first sorted-file damage. The [integrated artifact slice](#observed-simultaneous-artifact-damage) now has genuine HTTP RED/GREEN, deduplication/generic-refusal controls, retained-witness mutation coverage and passing full affected libraries. This does not establish aggregation across separate publication/runtime domains. An earlier wrong module selector ran zero tests and supplies no coverage (`preflight-multiple-absence-existing-batch.log`).
- Both generations expose only instance-addressed standalone preflight, not global/component endpoints. Global/Instance are settings origins; a Java component is a runtime selection value. Existing successful HTTP diagnostics cover inherited/global/instance settings. The missing-default tracer verifies one explicit global managed-component refusal, its blocking diagnostic/origin and blocked enrichment. Other negative inheritance/selection controls remain unverified. The production UI uses SetupService enrichment, whose bulk view intentionally requests top-level readiness; preserve that behavior rather than inventing standalone-preflight UI consumers.

## Observed required-library corruption

2026-10-07, changes against `10e99d7e` on `main`. The existing verifier now carries a typed `LibrariesCorrupt` error only for a validated recorded `libraries` file whose observed size or digest differs. Invalid recorded checksums, path/admission failures, symlinks and failed reads retain generic refusal. The existing preflight projection returns retained `libraries_corrupt`/`blocking`, “Required libraries are corrupt. Repair this version before launching.” Ordinary launch remains `InstallUnavailable`; install HTTP remains409. No repair, scanner, new state owner, schema or UI change; captured publication/account/settings/instance fences remain intact.

The same genuinely installed-provider-JAR tracer flips one byte without changing length, captures the public refusal, verifies changed bytes remain untouched, then continues its existing absence/symlink/restoration/metadata/client controls. No-effect and joined-cleanup checks precede the intended RED at omitted `status:"ready"`, wrapper101; GREEN passes the same tracer including later privacy assertions, wrapper0 (`preflight-library-corrupt-{red,green}.log`). This directly exercises same-size corruption, not a separate size-mismatch fixture or every artifact family.

Independent Standards/Spec review clears all six frozen files: verifier `135c4c35`, queue `87dc980c`, coordinator `fe5184e0`, tracer `802b397f`, route `79362f0b`, generated reason `beea0ebc`. Rust-owned generation/equality and scoped formatting pass. Focused current-source checks pass90 install/Content,44 launch-coordinator/one existing helper ignore, three install-route and83 desktop/one helper ignore cases, zero failures, joined0 (`preflight-library-corrupt-{install,launch,routes,desktop}.log`). Pinned canonical typed frontend checks pass531/zero failures/one TODO; build and independent generation verification retain `239a78614d6e`/budgets, joined0 (`preflight-library-corrupt-{frontend,build,generation}.log`). Earlier full831 app/124 API results belong to the absence checkpoint, not this later source. Assets, logging configuration, other negative facts and native/installed/full-parity acceptance remain open.

## Observed required-library absence

2026-10-07, changes against `810346fa` on `main`. The existing installed-file verifier distinguishes observed absence of a validated recorded `libraries` entry from generic refusal. Standalone preflight returns the retained `libraries_missing`/`blocking` fact and “Required libraries are missing. Install this version before launching.” Completed observation keeps `status:"ready"`, `launchable:false`, the safe `install_unavailable` error and captured memory/origin/resource facts. Checksum/path/namespace validation precedes observation; symlink/admission errors and measured corruption remain generic in this first slice. Existing account/config/bundle/final-instance fences and ordinary launch refusal remain intact. No scanner, repair, persistence owner, schema or UI change.

The real HTTP tracer first installs the provider-declared JAR and proves launchable Ready, then removes that actual recorded file. A separate external-canary symlink requires the exact generic refusal. Original library bytes are restored before the metadata/client controls; application/provider shutdown joins before preservation, no-effect and privacy assertions. RED reaches omitted diagnostic status (`Null` versus `"ready"`), wrapper101; the minimal owner-chain correction passes GREEN, wrapper0 (`preflight-library-missing-{red,green}.log`). Reports/queue/sessions, library/canary bytes and symlink identity remain protected, with no repair or launch. Rust-owned export/equality passes (`preflight-library-contracts-{generate,check}.log`), adding only the generated reason union. Library corruption, assets and native/installed/full-parity acceptance are not established by this slice.

Independent Standards and Spec reviews clear all six files: verifier `612c718a`, queue `9c6b34f0`, coordinator `84e57dbc`, HTTP tracer `0dd32280`, route `aa155cc8`, generated reason `bbaa7754`. The route retains HTTP409 for the new typed error. Full serialized app/API checks pass831/124, zero failures and ten/eight existing helper ignores, wrapper0 (`preflight-library-app-api.log`); the full API result precedes the final one-line route classification. After that correction, all three install-route controls and the exact HTTP tracer pass again, followed by83 desktop composition checks/one existing helper ignore, zero failures, wrapper0 (`preflight-library-{final-routes,final-http,desktop}.log`). Pinned Node24.13.1 canonical frontend checks pass531/zero failures/one existing TODO, wrapper0 (`preflight-library-frontend.log`). Project-edition formatting, whitespace, TypeScript and semantic lint pass (`preflight-library-{format-check,types,semantic}.log`). Frontend build and independent generation verification pass unchanged `239a78614d6e`/budgets (`preflight-library-{build,generation}.log`), wrapper0. These source/HTTP fixtures do not certify native credential persistence, installed UI or full parity.

Hosted [run37577763103](https://github.com/mateoltd/axial/actions/runs/37577763103) passes both jobs at exact `10e99d7ea0d2db9ec8ecc5e90f05d8ba18ffc671`; terminal watch joins0 and the independent SHA/job query agrees (`preflight-library-ci-{watch.log,current.json}`). These are source checks for that absence checkpoint, not the subsequent corruption source or installed/full parity.

## Observed installed-metadata readiness

2026-10-07, changes against `d3afc108` on `main`; intervening `50243953` changes only the credential test/evidence. The existing installed-version verifier now distinguishes observed absence of its exact recorded version JSON from generic refusal. Standalone preflight retains completed `status:"ready"`, top-level `launchable:false`, `install_unavailable` and the independently retained legacy fact: `version_json_missing`, `blocking`, “Installed version metadata is missing. Install this version before launching.” The shared diagnostic constructor preserves captured memory, override origins and resource budget. Account/config/bundle/final instance fences, bulk/ordinary refusal and install HTTP409 remain unchanged. No second scanner, state owner, repair, schema or UI change.

The existing real HTTP installed-file tracer first establishes launchable Ready, removes only the recorded JSON and observes no repair, then substitutes an external-canary symlink and requires the exact generic refusal. It restores original metadata before the existing client-corruption/symlink/absence controls. Application/provider shutdown joins before assertions checking preserved metadata/client/canary/link identity, unchanged reports/queue/sessions and serialized privacy. Its RED reaches missing diagnostic status (`Null` versus `"ready"`), wrapper101; the minimal owner-chain correction yields one passing GREEN, wrapper0 (`preflight-metadata-{red,green}.log`). Malformed recorded checksums, observation/admission errors, byte mismatches and invalid JSON retain generic refusal in inspected source; this slice adds no direct malformed/corrupt-metadata test.

Independent Standards and Spec reviews each report zero actionable findings at verifier `7b6cf2b7`, queue `896513b9`, coordinator `4e792090`, route `59104691` and HTTP fixture `dc599f69`. Rust-owned export/equality and scoped Rust2024 formatting/whitespace pass (`preflight-metadata-wire-{generate,check}.log`, `preflight-metadata-format-check.log`). Serialized full source checks pass831 application/123 API, zero failures, ten/eight existing helper ignores, wrapper0 (`preflight-metadata-app-api-final.log`); nested child results are excluded.

The initial pinned Node24.13.1 full frontend run fails two unchanged harness controls, with529 passed/2 failed/one existing TODO (`preflight-metadata-frontend-final.log`). The lease-collision fixture's initial blocker bind gets EADDRINUSE before its assertion; the runner timeout fixture refuses unprovable Darwin process-group inspection rather than proving a surviving child. No contemporaneous owner/probe detail establishes either cause. After Cargo quiescence, isolated existing generation and runner controls pass25/5 respectively, followed by full source/test checking531/zero failures/one existing TODO (`preflight-metadata-{generation,runner}-harness-isolated.log`, `preflight-metadata-frontend-serial.log`), wrappers0. These passes do not diagnose the failures. An initial combined-selector command refuses before execution because only one exact file is supported (`preflight-metadata-harness-isolated.log`). No timeout, lease or settlement guard is changed.

Desktop composition passes83/zero failures/one existing helper ignore, wrapper0 (`preflight-metadata-desktop-composition.log`). The preceding `--lib` command refuses because this package has no library target (`preflight-metadata-desktop-final.log`); that setup error is not a test/product failure. No native app or bundle is opened/rebuilt. This is source/HTTP fixture evidence, not native/installed/full parity.

## Actual retained-browser API reopen

2026-10-07, unchanged product checkpoint `bd270792` on `main`, with documentation-only `853eef87`. The clean-source API executable SHA256 is `8b8758140525850f0df1670994e52387189b0d89ff8db65b52ce960cda894959`, embedding frontend `239a78614d6e`. A disposable Fabric0.19.5/Minecraft1.20.1 instance has factual Ready and an instance-owned fake-Java override. The baseline begins after its creation/configuration and actual browser Ready, not before setup; no Microsoft credentials or installed application are involved.

Actual IAB tab2 submits Launch once and visibly reaches Playing/Stop, receiving live revision4. A private loopback proxy gates both traffic directions and cancels retained streams. Ordinary first-API shutdown joins0 before the same executable/profile reopens; the unchanged browser document still shows Playing while gated. Opening transport forwards a genuine stale-capability401, fresh bootstrap, typed missing-live-session404 and the exact original accepted-intent terminal proof at cold revision1. The real UI clears Playing/Stop, restores Ready/Idle and enabled Launch, and shows exactly one terminal notice. Final proxy evidence counts one document request/response, one Launch attempt/handoff, zero Kill attempts/handoffs and zero gated mutation/document attempts. No reload, state injection, Retry, mutation replay or replacement intent is used. Response forwarding alone is not the browser-consumption proof; root's actual DOM/control and screenshot observations establish the visible transition.

Read-only witnesses preserve all four prior raw intents/reports, stable tables, all five complete instance trees including target file identities/metadata, both recorded inventories and all3,640 unique recorded files/760,421,199 bytes, marker and sibling canary. Exactly one acknowledged schema4 stopped report records observed boot32ms, requested stop/signal9, settled tree/drained output, no observed failure classes and zero dropped logs. Only target revision/recency, last-instance selection pointing to the captured target, and that intent/report change before settlement. Complete captured settled state remains exact through browser reconciliation and fresh post-join capture, with zero checked obligations/SQLite OK. Original admission authenticates native authority; cold terminal reads validate persisted evidence, not fresh receipt authentication.

Logs under `.rewrite-logs/`: `browser-reopen-runtime.log`, `browser-reopen-{before,settled-corrected,reopened,final-joined}.json`, `browser-reopen-ui.md` and `browser-reopen-absence.json`. Proxy SHA256 `43661554fadc746900f46c48ca4d9f5036645682f4981b6aec4145f077abdc03`; final witness `e784ce165ee7901fcaf93de54a9593ad585dace3e3d744ba67279cef6b14963e`. Independent helper Standards/Spec review is clear. The initial settled witness refuses because it wrongly assumes last-instance selection immutable; the existing successful-launch transaction explicitly updates it. Its failed output/log and prior helper `5ddaf48a` remain retained; the narrow correction requires exactly singleton1/captured target, not arbitrary table drift. An earlier final capture precedes explicit tool-join observation and is not used as post-exit proof. Both API processes and the proxy join0; separate exact-PID, listener, fixture-process and profile-owner checks find none remaining.

Screenshots were captured/displayed through computer use but not durably exported. This verifies real retained-browser transport/session convergence with a synthetic process and bounded preservation, not a real JVM, gameplay, whole-profile preservation, native/installed-platform acceptance or full parity. No production or UI edit follows. Hosted [run37565859463](https://github.com/mateoltd/axial/actions/runs/37565859463) separately passes both jobs at exact `bd270792`; hosted source checks do not substitute for this journey or the remaining gates.

## Observed Java-override readiness

2026-10-07, changes against `9b50bd2d` on `main`. Runtime probing no longer classifies a spawn `NotFound` as executable absence: an existing executable with an absent interpreter is `Failed`, while actual path absence remains `Missing`. The public probe regression goes RED then GREEN (`runtime-spawn-cause-{red,green}.log`, wrappers101/0); all seven probe checks pass (`runtime-spawn-cause-owner.log`). Rosetta classification and executable evidence remain unchanged.

Standalone preflight shares its existing diagnostic constructor for typed `Missing` or `NotExecutable` only with a captured explicit override. Both retain blocking `java_override_missing` and safe memory/origin/resource facts, while preserving their distinct original `runtime_unavailable` errors. Legacy runtime readiness also rejects regular files without executable bits; this is retained factual behavior, not a new repair policy. Bundle revalidation, account/config revisions and final instance/exclusion/settings fences remain. Capture failure preserves the original refusal; bulk/default managed-runtime absence and other errors do not acquire override facts. No scanner, state owner, fallback, credential access or UI change.

The actual HTTP tracer first establishes installed Ready, then observes absent override, broken interpreter and mode0600 regular file. Missing diagnostics go RED then GREEN (`preflight-java-override-{red,green}.log`); extending the shared assertion loop to nonexecutability independently goes RED then GREEN (`preflight-java-nonexec-{red,green}.log`, wrappers101/0, final test3.03s). Failed-interpreter exact response remains generic without diagnostics. Normal shutdown precedes assertions on unchanged fixture/canary bytes, reports, queue and sessions; serialized facts omit private paths, JVM arguments and capability. Final independent Standards/Spec reviews are clear for probe `4880d66f…47fc4f1`, coordinator `83eb007f…74b415f`, HTTP `ad1e144e…6990702` and generated ID `5f4d3a16…fbb833b`. Generation/equality passes; broader current-source checks are recorded in [integration](integration.md). These fixtures cover instance-origin external overrides, not every global/component or negative-readiness case, native/installed acceptance or full parity.

Final serialized libraries pass831 application/123 API, zero failures/ten and eight existing helper ignores, normal wrapper0 (`preflight-java-final-libraries.log`, nested child result excluded). Pinned Node24.13.1 frontend checks pass531/zero failures/one existing TODO; build remains `239a78614d6e` and independent generation verification confirms unchanged budgets (`preflight-java-final-{frontend-pinned,build-pinned,generation}.log`). Final wire equality, scoped project-edition formatting and semantic/asset policies pass (`preflight-java-final-{wire-check,rustfmt,semantic,assets}.log`). These source checks do not establish native credential, installed rendering or full parity.

The serialized desktop composition suite passes83/zero failures/one existing helper ignore, normal wrapper0 (`preflight-java-final-desktop.log`, test6.95s). This verifies native source composition, not a packaged or interactive native journey.

## Observed client-file readiness

2026-10-07, frozen working changes against `8edd5bcf` on `main`. Missing/corrupt client reasons originate in the existing installed-inventory verifier: exact admitted client absence or measured size/hash mismatch, respectively. Read/admission errors, unsafe symlinks and malformed recorded digests remain generic refusals. The scheduled review reproduces a nonhex recorded digest falsely classified as corrupt, then corrects only that verifier: decode the recorded SHA1 into20 bytes, require its existing canonical lowercase representation, and compare observed digest bytes. No second scanner, stored schema or public validation layer.

Standalone negative facts share the successful diagnostic constructor and resource sampler. Captured account/settings revisions, bundle revalidation and final instance/exclusion/settings fences remain; capture failure retains the installation refusal without fabricated facts. Completed observation retains `status:"ready"`, blocking `client_jar_missing`/`client_jar_corrupt`, top-level `launchable:false` and `install_unavailable`. Ordinary/bulk errors and install HTTP409 remain unchanged. Rust-generated reason aliases replace unused handwritten mirrors; no UI change.

Actual HTTP missing and same-size corruption regressions go RED then GREEN (`preflight-client-{missing,corrupt}-{red,green}.log`). The combined tracer first proves installed Ready, then checks corruption, an external-canary symlink and absence through serialized responses; it joins shutdown before checking unchanged bytes/canary, reports, queue and sessions. The public installed-read regression independently goes RED with `ClientJarCorrupt` for a malformed reference, then GREEN with `NotReady` and unchanged client bytes (`preflight-client-digest-{red,green}.log`, wrappers101/0). Final generated export/equality and project-edition formatting pass (`preflight-client-final-wire-{generate,check}.log`, `preflight-client-review-format-2024.log`). Independent Standards and Spec reviews are clear for final verifier hash `46c72fcb…5dc36d`. These are source/fixture checks, not native/installed acceptance; other negative/runtime facts and full parity remain open.

Final review verification passes 90 selected install checks, nine prefix-selected preflight guards, the separate exact missing-ownership guard and the combined HTTP tracer (`preflight-client-review-{install,guards,ownership,http}.log`, normal wrappers0). Canonical frontend source/test checking runs under the available Node24.19.0:531 pass, zero failures, one existing TODO; production formatting and build also pass, generation unchanged `239a78614d6e` (`preflight-client-review-{frontend,frontend-format,build}.log`). This is not the pinned CI runtime or a full current-source library suite. Broader integration remains root's handoff.

Subsequent frozen-source integration passes830 application/122 API library tests, zero failures and ten/eight existing helper ignores, normal wrapper0 (`preflight-client-integration-libraries.log`). The nested child helper is not counted twice. Pinned Node24.13.1 source/test checking passes531/zero failures/one existing TODO, build remains `239a78614d6e`, and independent generation verification passes unchanged budgets (`preflight-client-integration-{frontend-pinned,build-pinned,generation}.log`). Semantic checks cover289 files with no fixes; asset policy passes. These source checks do not establish native credential persistence, installed rendering or full parity.

The serialized desktop consumer suite also passes83/zero failures/one existing child-helper ignore, wrapper0 (`preflight-client-integration-desktop.log`). This verifies current native source composition, not a new packaged/interactive native journey.

Hosted [run37564042012](https://github.com/mateoltd/axial/actions/runs/37564042012) subsequently passes both application and delivery-contracts jobs at exact `9b50bd2ddf36fe8f2938cd7f5184536732114876`. The terminal watch joins0 and the final SHA/job query agrees (`preflight-client-ci-{watch.log,complete.json}`). This verifies the committed client-file slice, not later runtime changes or installed/full parity.

## Launch diagnostics and terminal convergence

2026-10-07, changes against `640fc7fa` on `main`. Standalone successful preflight now retains safe effective memory/clamp, captured global/instance override origins, ready status and all nine resource-budget facts. Inheritance owns origins; the existing preflight projection owns diagnostics after its final account/settings/instance/filesystem fences. Shared host input is sampled once, and bulk setup avoids unused per-row budget sampling. Actual serialization preserves omitted origins and explicit nullable estimates. Rust-generated contracts replace implemented frontend mirrors through the existing exporter; early-refusal diagnostics and typed negative readiness remain pending.

Browser sessions retain their original accepted intent. A typed missing live-session response may query that exact intent, but only exact instance/session/start identity and complete authenticated tree/output settlement can clear Playing. Cold proof does not reset live revision ordering. The existing completion owner publishes terminal state before final-log drainage; its existing fence prevents delayed SSE, successful Stop and refused/lost Stop responses from restoring nonterminal state. No process adoption, mutation replay, journal, new lifecycle owner or layout change.

Verification uses real HTTP preflight and the existing exported frontend launch/actions/decoder workflow at its request boundary:

- `launch-preflight-diagnostics-red.log` fails the actual successful response's missing status; GREEN passes the same journey. All ten existing preflight guard cases pass (`launch-preflight-owner.log`). Generation and equality checks pass (`launch-preflight-wire-{generate,check}.log`). Reports, queue and sessions remain unchanged in the read-only journey.
- Four meaningful frontend RED/GREEN boundaries cover cold accepted proof, pending-log terminal publication, late live SSE and pending Stop. Final focused selection passes101/101 (`launch-cold-intent-*.log`), including refused/malformed/unsettled/wrong-binding and replacement controls. This is source workflow acceptance, not an actual retained-browser API-reopen journey.
- Canonical typed frontend checks pass531/zero failures/one existing TODO; serialized libraries pass829 application and121 API/zero failures, with ten/eight existing helper ignores (`launch-contract-{frontend,libraries}.log`). Nested child results are not counted twice. Ordinary frontend build publishes `239a78614d6e`; scoped production Prettier/rustfmt and whitespace checks pass (`launch-contract-{build,format,rustfmt}.log`). Wrappers join0. Existing engine/compiler warnings remain retained.

Independent Standards and Spec reviews report zero remaining actionable source findings in each bounded slice. Native/installed verification, real API-reopen convergence, detailed negative preflight facts and full parity remain open; the build and fixture passes do not close them.

The subsequent serialized desktop consumer suite at exact `8edd5bcf` passes83/zero failures/one existing child-helper ignore, wrapper0 (`launch-contract-desktop.log`). This verifies native source composition, not a packaged or interactive native journey.

## Verified read-contract corrections

2026-10-06, changes after `db0baf50`. Three retained contracts have meaningful HTTP RED/GREEN checks:

- Discover returns provider results when optional installed-content annotation is unreadable or its valid instance ID is absent. Malformed IDs still refuse. The corrupted-manifest fixture retains direct Content/plan refusal, one accepted install settling Failed, unchanged managed/user bytes and no unresolved effects. Only the existing annotation chain changes.
- Individual resource lists scan their own subtree. A screenshots-directory symlink no longer blocks healthy mods/logs; screenshots and the aggregate resources endpoint still refuse, preserving the link and external canary. One private helper shares existing admission and final binding validation across five current reads; typed response arrays keep their wire shape.
- Debug command inspection restores the placeholder-only `command` array and `command_redacted`, counting the executable as legacy did. The actual fixture child independently reports only its element count; complete HTTP-response equality is checked after ordinary shutdown. Final review catches a duplicate projection bypassing the existing4,096-placeholder cap; `b2c34db9` reuses that owner, retaining exact count and the separate release refusal. The admitted16,384-argument limit remains intact; command values are never exposed.

Logs: `discovery-annotation-{red,green}.log`, `resource-list-isolation-{red,green}.log`, and `command-shape-{test-red,green}.log`. Initial wrapper misuse refuses before running the command test and is retained separately. The final owner-reuse follow-up passes all three existing projection tests, the actual composed journey and release API-library compilation (`command-owner-{focused,journey,release-check}.log`, normal0); the journey takes5.84s. Independent Standards/Spec reviews and scoped formatting are clear. These corrections do not change UI layout or establish native, authenticated or full parity. Broader verification is recorded in [integration](integration.md).

## Initial review

Date: 2026-09-26. Scope: current retained `frontend/src` consumers against the
replacement `apps/api` route composition and desktop command registration.
This was a source review, not a build, test run, browser exercise or installed
WebView proof. Other owners were editing concurrently; status below reflects
the final source read for this review. Intentional Guardian removal is excluded.

At the initial review, the principal blockers were
missing content and instance-resource adapters, absent native skin-file actions,
a launch-history response mismatch, and unfinished release/update behavior.
Two media query mismatches found here were corrected in source during the review.

## Current route inventory release gaps

Later integration supersedes the initial source observations below: content, resources,
system, music and benchmark routes are now composed; launch reports use the required
projection and native skin commands are authored. The latest source registrations
add both remaining baseline pairs, ordinary-instance import and metadata-import
receipt commands. Six source-inventory checks verify 142 current registrations
and all 122 baseline method/template pairs (`metadata-ui-route-inventory.log`). Both
metadata routes now bind to the actual retained import UI. There are no
known absent baseline method/template pairs in the current manifest.

Full pack installation and overrides now have a domain implementation and adapter;
their current integration is not yet verified. Route presence is not runtime parity.
Native update wiring and signed release inputs, import
preview without complete cutover, unverified native commands and the failure/restart gaps in
the integration ledger remain release blockers. The initial findings below record
what was observed, not a claim that those earlier omissions still all exist.

## High-impact findings

### P1 found and corrected in source: Profile skin identity query

`frontend/src/player-skin.ts:44` constructs `/skin/profile/file` URLs with
`profile`, optional `skin`, and optional `texture`. At the initial inspection,
`apps/api/src/routes/skin.rs:74` `ProfileFileQuery` accepted only `texture` and
used `deny_unknown_fields`, rejecting the current-profile texture path even
with a valid media ticket.

After notification, the skin owner added `profile` and `skin` to that query and
delegated to `ProfileMedia::profile_file_for_identity`, which checks the selected
profile, the requested skin/texture and the captured account after the fetch.
This corrects the inspected contract in source; real current/stale profile
requests through the assembled media-ticket boundary still need verification.

### P1: Nonempty launch history fails the retained response decoder

`apps/api/src/routes/launch.rs:261` serializes `LaunchProofRecord` directly for
`GET /launch/reports`; the detail handler does the same. The struct at
`core/app/src/launch/reports.rs:74` has no `view_model`. The retained
`frontend/src/dto-launch.ts` `isLaunchProofRecord` requires
`view_model.outcome_label`, `view_model.outcome_tone` and
`view_model.comparison`. `PerformanceLabProofHistory.tsx` consumes those fields
to display outcomes and comparisons.

An empty list can look healthy; after the first report, the entire history
response fails decoding. An owning-feature projection or coordinated retained
frontend adapter must supply the real presentation semantics. Sent to launch
and Performance owners.

### P1: Instance file screens have no composed API routes

The instance resource implementations under `core/app` do not by themselves
connect the retained screens. Current `apps/api/src/routes/instances.rs` serves
registry/create/edit/delete/duplicate only, and `apps/api/src/lib.rs` does not
merge a resource route family.

| Retained consumer | Missing route or contract |
| --- | --- |
| `views/instance/resources.ts:25` | `GET /instances/{id}/resources` |
| `views/instance/mod-actions.ts:132` | `PUT`/`DELETE /instances/{id}/mods/{name}`; mutation acknowledgment must have `status: "ok"` |
| `views/instance/world-actions.ts:24` | `PUT`/`DELETE /instances/{id}/worlds/{name}` and `POST .../backup`; backup acknowledgment needs `backup` and `location` |
| `views/instance/screenshot-actions.ts:29` | `PUT`/`DELETE /instances/{id}/screenshots/{name}` and `GET .../file`; rename acknowledgment needs the new `name` |
| `views/instance/logs.ts:98` | `GET /instances/{id}/logs/{name}` returning `name`, `size`, `truncated`, `text` |
| `views/instance/instance-actions.ts:13` | `POST /instances/{id}/open-folder?sub=...` |

These calls currently have no matching authenticated route. Resource listing
is the first failure for several tabs; adding only mutation handlers would not
restore the workflows. File adapters must retain registered-instance authority
and mutation acknowledgments. Sent to API and filesystem owners.

### P1: Discover, content installation and pack creation are not routed

All current `frontend/src/content.ts` calls below are absent from the composed
API at review time:

- `GET /content/search`, `/content/item`, `/content/modpack/target`,
  `/content/modpack/files`.
- `POST /content/plan`, `/content/install`, `/content/compatibility`,
  `/content/modpack/install`.
- `GET /instances/{id}/content`, `/instances/{id}/content/updates` and
  `POST /instances/{id}/content/uninstall`.
- `POST /instances/setup/plan`.

`frontend/src/instance-create.ts:52` also selects `POST /instances/setup` for a
resolved setup plan and `POST /instances/modpack` for pack creation. Neither is
registered by the current instance router. The ordinary `/instances` create
path being present does not cover these two branches.

Installation/uninstallation callers expect an `InstallQueueStateResponse`,
not a detached success string. Plans use retained target/selection DTOs and pack
requests preserve selected optional files and `include_overrides`. Sent to
content and API owners; this finding identifies missing adapters/composition,
not an assertion that the domain implementations are absent.

### P1: Desktop skin picking and native drop admission are missing

`frontend/src/native.ts` invokes `pick_skin_file` and `consume_skin_drop`.
The retained upload picker (`use-saved-skin-upload-workflow.ts:167`) and texture
replacement picker (`use-saved-skin-edit-workflow.ts:213`) call this native
path whenever Tauri is present. A rejected native command reaches the error
handler; it does not fall back to the browser file input.

Neither command is registered in `apps/desktop/src/main.rs` or permitted by
`apps/desktop/capabilities/main.json`. The retained native drop listener expects
`axial:desktop:skin-drag` with an admitted token, but there is no replacement
emitter/admission path in the current desktop source. Desktop owner confirmed
both gaps. Existing authenticated skin upload endpoints cannot compensate for
the missing file-selection entrypoint.

## Other retained release blockers

| Surface | Evidence and consequence |
| --- | --- |
| Native development reset | `AdvancedSettingsSection.tsx:74` calls `requestNativeAppReset`, which invokes `app_reset`; that command and its capability are absent. This is gated by development mode and native runtime, but is retained scope. Desktop owner confirmed it remains unfinished. |
| Music and hardware summary | `bootstrap.ts:67` and `:70` call `/system` and `/music/status`, currently absent. They catch failures and use null/default state, so bootstrap success does not prove parity. `music.ts:201` and `:253` use `/music/track?t=...`, also absent; enabled music cannot obtain audio. System absence changes memory recommendations. |
| Updates | `updater.ts` still checks `/update`, downloads through `/update/download` and applies through `/update/apply`. All three explicitly return 501 `update_unsupported` in `routes/update.rs`. This is honest unavailable behavior, but retained check/download/verify/apply/restart behavior is not release ready. |
| Cutover and import | Preview remains read-only and reports `cutover_available: false`; no retained UI/native source-selection consumer or semantic commit is wired. `acceptance/cutover/README.md` records the remaining publication, preference, reauthentication and settlement gates. |

## Other findings corrected during this review

The initial transport middleware authenticated a media ticket but passed
`axial_ticket` through to strict domain query decoders. Because
`frontend/src/api.ts` appends that parameter to media URLs, profile/cape/lookup
media requests with a valid grant were rejected as unknown query fields. The
API owner was notified and the final source read now shows
`authenticate_request` removing the transport grant with `without_ticket`
before domain extraction. This fixes the inspected mismatch in source; it has
not been exercised by this review. The related `profile`/`skin` mismatch above
was also patched during the review.

The benchmark router existed but was not registered at the initial inspection,
leaving retained `PerformanceLabCard.tsx` and `PerformanceLabSuiteDrivers.tsx`
matrix, qualification and driver calls without handlers. After notification,
`routes/mod.rs` now exports `benchmarks` and `lib.rs` merges its router with a
constructed benchmark service. Matrix and driver response fields match the
inspected frontend decoder shape. Runtime driver lifecycle, persistence and
qualification evidence remain to be exercised by the integration owner.

## Checks that did not produce a finding

The current composition registers config/status/onboarding, ordinary instance
create/edit/delete/duplicate, version/loader catalogs, Java discovery,
accounts/auth, flags, telemetry, install queue, launch/session and skin routes.
Spot checks found matching config revision/selection fields, ordinary create
settings, Java runtime list shape, queue event envelope, and launch named
`status`/`log` events. Registration and a spot check are not runtime parity.

The old `start_install_events`, `start_loader_install_events` and
`start_launch_events` native helper definitions remain in `native.ts`, but no
live consumers were found. Downloads and launch use authenticated HTTP/SSE.
Their missing native registrations are therefore not listed as regressions.
Installed transport evidence remains a separate release gate.

## Required follow-up evidence

Use the real assembled API and retained UI for one successful and one failed
operation in each missing family. Include a nonempty launch history, a current
profile skin URL with both identity hints and a media ticket, native upload and
drop selection, resources after actual game output, a content dependency plan
and selected pack files, and a persisted benchmark driver. Check exact request
and response DTOs after adapters land. Then repeat installed native workflows
across the required artifact matrix; neither source inspection nor empty
fixtures can establish these outcomes.
