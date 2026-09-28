# Parity requirements

Status: source inventory and planned acceptance, not runtime verification. Baseline: `2bda9b42c29d02828dd5a2b91d9cf8c8a431ffb0`. The accepted target is all existing non-Guardian behavior.

## Inventory and evidence

[parity.json](parity.json) maps the 122 current method/template pairs to behavior groups and also inventories UI, native, persistence and background surfaces. The source contains 85 product UI, 18 developer diagnostic, 17 internal runtime and 2 transport internal rows. A route being internal or developer-only does not authorize its removal.

This inventory establishes source traceability. Runtime characterization must still determine exact successful behavior, known failures, advertised version/platform support, and existing gaps. Each group's evidence array is intentionally empty until checks run. A source link, planned test name, or route count cannot satisfy a passing behavior claim.

The baseline is evidence, not an infallible specification. When it has a defect, record the observed behavior, desired outcome and regression case. Review intentional differences explicitly; never regenerate expected fixtures from the rewrite to make a comparison pass.

## Guardian disposition

| Retain | Defer |
| --- | --- |
| Readiness, Java/JVM validation, safe defaults, normal provisioning | Guardian diagnosis/rule fusion, modes and automatic configuration correction |
| Explicit install retry and user-directed reinstall/reapply | Guardian automatic launch retry, repair ladders and remembered suppression |
| Install/content/Performance interruption recovery and compensation | Guardian-driven component reconstruction or rollback |
| Performance modes, plans, health, rules, apply/remove/rollback, benchmarks and non-Guardian proofs | Guardian-specific facts, intervention fields and proof assertions |
| Exact ownership, scoped files, transaction-specific temporary storage and cleanup | Discretionary corrupt-record quarantine and idle integrity repair |
| Safe operational logs, reports, telemetry controls and neutral failures | Guardian-only telemetry and safety copy |
| Palette logic in `ui/look-guardian.ts` | Backend Guardian behavior only; matching the word in a filename proves nothing |

Mixed route responses and settings retain their non-Guardian fields. Standalone preflight retains factual validation even if its old response also described Guardian. Developer benchmarks and Performance transaction records remain required where they support retained behavior. Automatic continuation of a previously accepted transaction is not a new repair decision.

## Acceptance contract

A behavior scenario contains:

1. A fixed profile and external-provider fixture with source provenance.
2. User actions or domain commands using canonical identities.
3. Observable success: rendered/action state, process outcome and exact relevant file effects.
4. Invalid-input, provider failure, cancellation and restart cases wherever effects can cross those boundaries.
5. Forbidden outcomes: unintended identity changes, falsely ready state, stale terminal state, secret output or unrelated file mutation.
6. Applicable OS/architecture/version rows, intended differences and evidence location.

Example: creating a Fabric instance must select the intended Minecraft/build identity, complete its required artifacts, become launchable, launch with the expected classpath and retain identity/settings after restart. A screenshot of its library card or a mocked `success` response is insufficient.

## Test implementation

Start with existing runners, reviewed fixtures and a few concrete journey scripts. Add small reusable helpers for provider stubs, fake Java, isolated roots, semantic observations and fault injection only when multiple retained cases need them. Do not build a scenario language, generalized replay engine, or new test framework before the first journey passes.

| Layer | Evidence |
| --- | --- |
| Pure domain | Property/table cases for version ordering, loader normalization, dependency resolution, argument validation, path identity and redaction |
| Real workflow | Actual feature owner plus real persistence/files with stubbed external services; inject failure at each durable transition |
| Transport | Actual serialized DTOs, auth/origins, media, stream subscription/reconnect/revision behavior and status convergence |
| Browser interface | Built frontend with real local backend; keyboard actions, lifecycle state and error recovery through controls |
| Native artifact | Installed release candidate exercising WebView, OAuth window, keyring, native files/opener/chrome, process ownership and update/restart |
| Provider canary | Limited real-provider metadata/auth checks separate from deterministic fixtures; record network outages instead of rewriting fixtures |

Generated wire types eliminate duplication but do not replace runtime validation. A handwritten UI mock is for presentation development and cannot be the only server in a parity test. Mock coverage should derive from reviewed contract examples, with absent behavior failing visibly.

Use semantic comparison across old/new runs on separate profile copies. Preserve relationships when normalizing IDs and timestamps. Do not erase event order, ownership identities, path relationships or terminal outcomes. A small comparator must demonstrably fail for a wrong loader, lost settings, stale success, changed user file or leaked synthetic token; targeted mutations are sufficient.

Apply bounded fuzzing/property tests to parsers and untrusted paths/media/archives. Concentrate interruption tests on filesystem/metadata publication, keyring changes, import, content/Performance batches, process shutdown and updater application. Avoid exhaustively multiplying unrelated UI theme choices with every provider failure.

## Platform and version coverage

Retain the existing artifact architectures from [release configuration](../../.github/workflows/release.yml): Linux x86_64, Windows x86_64, macOS x86_64 and macOS arm64. Record native host, OS build, artifact digest, toolchain and scenario outcome. Cross-compilation is build evidence only. Minimum OS and browser-engine claims remain unverified until characterized; do not invent broader support.

Each loader family needs fixtures for its distinct installation algorithms and observed compatibility limits. Retain all three current Forge strategies. Pick representative version/build boundaries during baseline capture, then add exact failing/edge cases from existing tests and user profiles. Do not claim all possible version combinations were executed.

Tauri's mock runtime does not execute native WebViews. Its current documentation distinguishes direct WebDriver availability from other tooling, so select and prove an automation path per OS rather than assuming one driver covers the entire matrix. [Tauri test documentation](https://v2.tauri.app/develop/tests/). Where automation is unavailable, require recorded manual/native-interface evidence; never silently treat a skipped row as passing.

## Failure and race matrix

| Boundary | Required cases |
| --- | --- |
| Downloads/extraction | Disconnect, timeout, response/decompression limit, missing or wrong checksum, malicious path, collision, disk full, permissions |
| Publication | Before effect, mid-effect, after filesystem promotion, before/after metadata commit, cancellation winning/losing arbitration |
| Instances/resources | Launch versus delete/mutate, source replacement, keep-files, duplicate partial copy, external library unavailable, Unicode/case aliases |
| Root lifecycle | Library switch/reset against active work, retiring generations and escaped guards/receipts; no mutation authority outlives its valid pinned generation |
| Launch | Missing/incompatible Java, malformed/reserved args, spawn failure, early exit, huge output, stop races, app loss, descendants, relaunch |
| Accounts/skins | Refresh versus logout/switch/remove, keyring unavailable/ambiguous save, stale profile response, partial skin/cape success, superseded apply |
| Content/Performance | Target drift, dependency cycles/pins, changed user file, interrupted batch, failed compensation, exact rollback snapshot |
| Transport | Wrong origin/capability, server restart, stale subscription, broadcast lag, expired/reused ticket, UI reconnect after terminal |
| Import/update | Repeated intent, unsupported legacy data, source changes, staging/promotion/commit crash, unsafe update artifact, active-work exclusion |

## Interface parity

Preserve the product's current desktop design and primitives: creation overlay and staged-content handoff; route/back/forward/scroll memory; one account switcher host; queued install state across screens; autosave and override reset; default skin deduplication; preview before apply; resource context/bulk actions; theme/sounds/shortcuts; update prompts and platform chrome. Screenshots supplement semantic checks for changed layouts.

UI preservation is an explicit user constraint, not merely a default. Do not redesign layouts, replace the component system or restyle screens as part of backend integration. A narrowly justified polish change has separate before/after evidence and review. Every other visible difference must be corrected before declaring the UI package complete.

The approved feature difference is removal of Guardian-only controls/copy. Record those exact elements in the baseline comparison and preserve their surrounding interface; do not suppress entire screenshots or settings sections to hide the difference.

Desktop and compact desktop widths, keyboard access, accessible control names, focus restoration and reduced-motion behavior are included. Existing animation behavior that contradicts an intended accessibility requirement is documented as a corrected baseline defect, not silently treated as retained behavior.

## Evidence gates and efficiency

- During preparation: parse inventory and dependency data, reconcile source mappings, check links and ownership. Do not run application CI or tests.
- During implementation: formatting/types plus the affected domain, transport and real-interface cases. Broaden to consumers when changing storage, transport, startup or shared contracts.
- During integration: complete deterministic family journeys and failure cases against merged code; continuously keep the first vanilla journey passing.
- Before release: all retained groups have applicable evidence, the full existing suite relevant to preserved behavior passes, and installed workflows/import/update pass on the artifact matrix.

Capture command output and review the tail unless diagnosing a failure. Schedule Cargo/build writers according to existing leases. Broader or repeated checks need a changed dependency, failed check or unresolved concern; they do not run merely to increase counts.

Measure startup-ready latency, retained-instance launch preparation excluding provider time, time to show progress/stop, memory at idle and with logs/skin previews, and install throughput on named fixtures and hardware. Record baseline distributions before setting budgets. There are no invented percentages or timing promises in this plan.

## Done

Every retained inventory group has passing evidence from its actual implementation and relevant consumers, all intended differences are explicit, and all unsupported/unverified platform/version rows remain visible. Old data remains recoverable, unrelated files are preserved, public output is sanitized, and installed updates keep working. Guardian-only behavior is absent throughout settings, startup, background work, mixed DTOs and evidence claims.

A release remains incomplete while any retained group is supported only by mocks, source assertions, unexecuted scenarios, a development build, or a compilation check for a different host.
