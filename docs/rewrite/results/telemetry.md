# Telemetry

Status: frontend checks, 18 Rust exporter/domain tests, two telemetry route tests, and configured application startup/shutdown fixture pass. Additional instance/install/launch producers are implemented with owner-level tests awaiting the next shared checkpoint. Startup-failure and installed application evidence remain incomplete.

The replacement telemetry owner starts without consent and reads no environment or installed profile. `CollectorConfig::new` and `posthog` explicitly configure export. Collector URLs permit HTTPS, or HTTP only for loopback/localhost, reject credentials/query/fragment, and never follow redirects. A request has a three-second timeout. Errors return only a boolean or the fixed configuration error; credentials, URLs, provider bodies and request errors are never logged by this module.

`TelemetryEnvironment::from_label` preserves the existing deployment-label override: trim/lowercase, at most 32 ASCII letters, digits, hyphens or underscores. Direct `Custom` variants are revalidated during collector construction. Composition owns reading any explicit replacement configuration; legacy environment variables do not implicitly configure this module.

Consent uses a send-admission fence. Settings obtains `consent_change_owned`, moves that guard into accepted persistence work, and calls `publish` only with the committed consent and identity. An already admitted request can finish before revocation commits. A waiting consent writer blocks later sends; publication clears queued events when consent is disabled or identity changes. Invalid/missing identity and absent collector suppress ordinary, frontend and panic event admission. Dropping a failed settings write's guard without publication retains prior committed consent. The settings owner creates/rotates the UUID; telemetry only validates its canonical shape.

Events are a closed typed vocabulary: app started, launch started/completed, instance created, and the seven retained non-Guardian exception kinds. Properties contain only the anonymous UUID, fixed runtime/loader/outcome/error labels and explicit deployment label. Events capture a UTC timestamp when queued. Frontend name/message input is bounded and discarded by the backend. The frontend itself sends only fixed summaries and known built-in error class labels; it never reads source messages, stacks, filenames or thrown-value strings.

The memory queue retains 64 events and drops the oldest on overflow. Exports drain at most 20 per request. Errors are limited to 30 per process and five per kind; consent changes do not reset those budgets. Failed or cancelled sends are not retried because delivery may already have occurred. The owned run loop flushes every 30 seconds and on panic notification. Shutdown waits for an in-flight request and attempts at most four remaining batches. Producers must settle before this final drain; queued telemetry remains best effort. Panic capture uses a nonblocking state lock and always preserves the prior process hook.

The frontend caps attempts at five per session, deduplicates fixed payloads, suppresses concurrent reporting, and stops after three consecutive failures. A synchronous transport exception now removes its deduplication reservation, allowing the same error to retry within those bounds.

## Verification

`node --test frontend/test/rewrite/telemetry.test.mjs` passes nine tests, with no skipped tests. Tests execute the actual TypeScript module and cover consent, hostile thrown values, fixed payloads, browser-hook preservation, deduplication, storm/concurrency limits, revocation/re-enabling, and synchronous/asynchronous failure containment. This is an isolated module fixture, not installed WebView evidence.

Direct Rust formatting, focused Prettier formatting, and `git diff --check` completed. This worker did not run shared Cargo/build commands. Integration-owner commands:

```sh
cargo test -p axial-app telemetry::tests --lib
cargo test -p axial-api routes::telemetry::tests --lib
```

The 18 core tests use actual loopback TCP collectors and reqwest. They cover disabled/keyless/invalid-identity suppression, exact event properties and timestamps, queue eviction and batch bounds, consent revocation versus in-flight/later sends, identity rotation, abandoned settings waiters with retained owned guards, send cancellation, provider failure, redirect refusal, real timeout, multi-batch shutdown, closed shutdown channel, process error budgets, nonblocking panic capture, collector validation and custom deployment labels. The two route tests cover source-free public rejections and real collector payloads under enabled/disabled consent. Socket failures fail tests; no provider credential or external service is required.

Integration's `.rewrite-logs/app-api-integration-tests-4.log` records all 18 telemetry domain tests, both telemetry route tests, and `configured_telemetry_restores_consent_and_joins_final_flush` passing. The latter opts in through the real authenticated settings API, reopens the isolated profile, observes the same identity, and receives its startup batch at a local collector before shutdown returns. The full application suite in that checkpoint had unrelated failures; this is scoped telemetry evidence, not a whole-build claim.

## Integration obligations

Composition now initializes telemetry explicitly from `AXIAL_REWRITE_TELEMETRY_API_KEY` and optional `AXIAL_REWRITE_TELEMETRY_HOST`/`AXIAL_REWRITE_TELEMETRY_ENVIRONMENT`, defaulting to no collector without a valid explicit key. It constructs settings with the matching exporter-availability policy, publishes saved consent/identity before admitting events, registers the route behind authenticated exact-origin transport, retains the background task handle, and joins it after foreground producers settle. Startup emits `TelemetryEvent::AppStarted { state_inspector }`. `install_panic_capture(&Arc<Telemetry>)` installs the global hook; local domain tests exercise capture without replacing process-global hooks.

Feature owners now accept the same telemetry instance through additive `InstanceService::with_telemetry`, `InstallQueue::with_telemetry`, and `LaunchCoordinator::with_telemetry` builders. Composition wires all three before downstream cloning and publishes persisted consent before startup recovery. No producer supplies a name, identifier, path, URL, exception text or log content.

Instance creation emits after the actual registry visibility transaction commits, including a newly completed recovery but not replay of an already completed row. Installation emits its fixed failure kind only after failed terminal status persists, never for cancellation, invalid input, repeated completion or polling. Launch tracks one accepted attempt across preparation and process startup. Its single completion describes startup: observed boot or the retained live-with-output timeout means success; genuine preparation/spawn/preboot failure or stop means failure. Later process exits cannot duplicate startup completion. Spawn/startup exceptions use only their corresponding fixed enum variants. A live silent process is not relabeled successful solely because time elapsed.

Owner-level producer tests inspect the real consent-gated typed queue without starting an exporter; the exporter itself retains separate actual loopback HTTP coverage above. Those new tests are not yet execution evidence until the integration owner's next checkpoint. Telemetry is best effort, so a crash after a committed feature outcome but before enqueue may lose that event; no analytics journal or replay mechanism is introduced.

The `StartupFailed` event still lacks a composition-level error/flush owner. Early profile or metadata errors cannot safely assume saved consent. The retained startup watchdog for a live silent process is also a launch parity gap, not something telemetry fabricates. Settings routes remain responsible for exact committed consent publication and suppressing errors from an explicit disable request. Frontend reporting must initialize through the retained bootstrap and error-boundary call sites. Neither local fixtures nor route registration establish full telemetry parity, cross-platform panic-hook behavior, or installed application lifecycle evidence.
