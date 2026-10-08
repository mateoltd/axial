# Fabric loader

Status: integrated. Real Fabric0.19.5/Minecraft1.20.1 install, launch, Stop and ordinary API reopen are qualified in [current-profile acceptance](current-profile-browser.md#launch-stop-and-relaunch). Historical1.14/0.2.0.71 installation and cold persistence now pass the scope below; its actual launch fails. Broader interruption/native/installed gates remain open. This is not feature-complete or full-parity acceptance.

## Scope and interface

`core/app/src/loaders/fabric.rs` owns the Fabric application boundary. It uses the retained `axial_minecraft` leaf rather than duplicating its provider parser, canonical build identity, compatibility selection, or materialization algorithms. No legacy Application, State, or Guardian orchestration is imported.

The module exports `COMPONENT`, `supported_versions`, `builds`, `cached_builds`, `resolve`, and `start`. Catalog payloads are the existing `LoaderGameVersion`, `LoaderBuildRecord`, and `LoaderCatalogState` types. Installation accepts a `ManagedLibraryOperation`, `ManagedRuntimeCache`, exact `LoaderBuildRecord`, and progress callback, and returns the retained `LoaderInstallPublicationOutcome` or `LoaderInstallError`.

`loaders/mod.rs` registers `pub mod fabric;`. Production requires `axial-minecraft`; the local tests use its `test-support` feature through a development dependency. No additional dependencies or public request DTOs were introduced.

## Retained behavior

- Supported game versions retain provider stability hints, manifest-based Minecraft ordering, and explicit fresh/stale/cache availability.
- Build listing retains complete compatibility checks: exact Fabric loader Maven coordinates, exact Minecraft intermediary identity, and a nonempty client main class. Historical universal main-class strings and current client/server objects remain supported. Incomplete rows remain omitted by the provider parser; conflicting duplicate identities reject the catalog.
- Selection resolves against live compatibility. Cached or caller-authored records never independently authorize installation. `start` rejects another loader component before effects, and the leaf requires the requested record to equal the live provider record.
- Profile materialization retains independent exact profile proof, base/profile composition, verified artifact downloads, authenticated publication, and canonical installed-version identity. Fabric checksumless profile libraries use freshly streamed bytes and sealed integrity evidence in the retained implementation.
- `BaseCommitted` remains intermediate. Shared installation primitives must verify, activate, and acknowledge the base receipt before continuing the exact retained plan. The child receipt also requires settlement before queue readiness. Publication errors retain recovery authority instead of becoming success or cancellation.

## Original package verification handoff

Five application tests exercise the real admitted library/cache boundary, not a fake provider success response:

- Historical `1.14` / `0.2.0.71` and current-shape-era `26.1` / `0.19.3` records retain exact fields and fresh cached availability through the application adapter.
- Missing and expired cache data remains unavailable through the non-network lookup.
- A cache row for another Minecraft identity fails validation.
- Cross-component and forged build identities fail before progress or new library entries, preserving a sentinel user file.
- Opaque identities for Quilt, Forge, and NeoForge cannot resolve through Fabric.

Formatting passed with `rustfmt --edition 2024 --check core/app/src/loaders/fabric.rs`. Only the integration owner runs Cargo/build writers. Requested focused commands:

```sh
cargo test -p axial-app loaders::fabric::tests
cargo test -p axial-minecraft loaders::providers::fabric::tests
cargo test -p axial-minecraft profile_source_rejects_identity_drift_and_base_owned_overrides
cargo test -p axial-minecraft profile_base_continuation_uses_exact_plan_without_base_or_catalog_rerun
cargo test -p axial-minecraft fabric_install_ignores_bogus_profile_integrity_and_streams_fresh_bytes
```

The retained provider cases independently cover incomplete compatibility rows, historical main-class strings, exact profile proof, duplicate collapse, and conflicting duplicates. The retained materialization cases cover inherited profile restrictions, continuation identity, and fresh Fabric library integrity. Their presence is implementation coverage, not a claim that they were executed during this package handoff.

## Current catalog admission and remaining evidence

At `7564b141`, ordinary API `9b67193c` opens a separate empty generated catalog profile. One bootstrap POST and one exact create-view GET return253 enabled builds, including Minecraft1.14/Fabric0.2.0.71 as enabled Beta, not recommended and not installed. Reviewed driver `3bb1f38a` joins0, receives96733bytes in five reads and pins the catalog SHA256 `c1ff2f31`. Its whole90-second/two-request body/read/allocation reservations remain conservative estimates, not physical I/O or hard heap measurements. The API's original handle also joins0 and PID/listener disappear; no account, instance, install or launch is created. Evidence is `historical-fabric-catalog-{admission,runtime}.log`. This is actual catalog admission only, not historical materialization or Java8 launch acceptance.

The original worker performed no live provider download, real Java launch, cancellation race or restart workflow. Subsequent representative modern integration evidence is linked above; the historical journey below closes only its recorded installation/persistence scope. Broader cancellation/settlement cases remain incomplete. Source reuse and cache tests alone do not establish those outcomes.

## Historical Maven transport correction

Against `caf8d6aa`, ordinary onboarding creates offline Fabric114Parity and one HistoricalFabric114 instance, Minecraft1.14/Fabric0.2.0.71,2GiB, auto-optimize off, in a separate generated profile. One Create200 queues installation; it fails with the base1.14 registered Ready and a Base checkpoint. Normal API/frontend exits join0. After cold restart Downloads retains no visible Retry card; one same-instance ordinary Install200 is a new attempt, not Retry acceptance. It also fails. Existing sanitized warnings identify artifact ProviderFailure facts, not their status or distinct artifact count. Both failed records remain durable.

Bounded private probe `7a53b28f` receives the exact official profile (1072bytes, SHA256 `04884826`) and follows only admitted repository redirects. The five declared ASM7.0 HTTP Maven Central artifacts return HEAD/GET501; original command joins1. Changing only that repository's scheme gives all five GET200 and joins0. Bodies are cancelled unread, so this is transport availability, not byte/integrity proof. Earlier observer refusals remain retained; their missing default-repository/redirect admission is not a product defect. [Sonatype documents the501 HTTPS requirement](https://support.sonatype.com/hc/en-us/articles/360041287334-Central-501-HTTPS-Required).

The existing library-plan owner upgrades only `http://repo.maven.apache.org/maven2/` (including input without a trailing slash), preserving coordinates, paths and integrity. No arbitrary repository, port, path or explicit artifact URL is rewritten. Genuine historical-shape planner RED joins101 with HTTP-versus-HTTPS mismatch; focused GREEN1, library selection10/0.38s, full Minecraft1047/82.61s and application848/ten existing ignores/66.80s pass, original handles joined0. The Minecraft count excludes four nested child summaries. Both source-review axes clear frozen `libraries.rs` SHA256 `efba9c14`; the ordinary API build joins0 in35.45s. No wire/UI, sanitizer, publication, signing or credential policy changes.

Frozen corrected API `7fb6c219` reopens the same profile. One ordinary Install POST200 retains the exact canonical build, install `e682366b-c991-4435-94d3-9883337217ac`, operation `e28f6adb-9663-4cf0-bb2a-4ea28441a5d8`. It reaches durable Succeeded/Final and registers the canonical child Ready. The untruncated admission slice contains exactly one queue POST. Base JSON/JAR hashes and account-directory/selection/global-settings digests remain exact. This is the corrected real provider/materialization outcome, not a historical gameplay certificate.

Fresh explicit Temurin8u504 probe passes0. One instance Java-path PUT200 leads to visible Ready/Launch; no Java floor is waived. One actual Launch POST200 admits session `0b5b99a6-4c6f-475e-b905-8357c52e22fb` on Java8, but it exits1 without observed boot. Public SSE verifies tree/output settlement, failed/startup_failed, and terminal acknowledgement/report persistence. Safe local log classification identifies VanillaTweaker, ClassNotFoundException for `net.minecraft.client.Minecraft`, and LaunchClassLoader NullPointerException; raw output/arguments are not exported. A separate bounded root read (`historical-fabric-companion-proof.log`, helper `5e6f001c`, joined0) pins the [exact official companion metadata](https://meta.fabricmc.net/v2/versions/loader/1.14/0.2.0.71) at1746bytes/SHA256 `8a13b557`: it declares FabricClientTweaker, while the generated profile's game arguments are empty. Provider/proof omission, not a demonstrated generic argument-parser loss, owns the next correction. No guessed launch injection or second launch is performed.

Normal exit, cold API/frontend reopen and final exit preserve the complete captured installed/failed-history/report/acknowledgement projection, including exact report/settlement hashes. Cold UI retains Fabric114Parity, the installed version and Launch availability. The private Java override persists at the exact instance record, but the existing public redaction makes Settings display Managed; this does not prove accurate override presentation. Both corrected process pairs join0 and their exact PIDs/esbuild children/listeners disappear; failed Java PID64432 is absent. Base inputs and the three protected metadata digests remain unchanged. Native world-reload launcher44237/JVM57948 remain untouched. Startup placeholders and the earlier stale Install label are not diagnosed.

Evidence: `historical-fabric-{provider-probe-*,central-{red,green,api-build,api*,frontend*,installed-status},library-plans,minecraft-full,app-full,base-before.sha256,protected-metadata-*,first-launch-safe-cause,profile-*-shape,terminal-*}` under `.rewrite-logs/`; private terminal SQL observes only its named rows with verified16MiB SQLite heap readback. Reservations/probe limits are logical bounds, not physical-I/O or hard JavaScript-heap measurements. No arbitrary interruption, visible menu/gameplay, native installed or full-parity claim follows. Root owns the exact provider-declared tweaker regression and the remaining launch acceptance.

Cancellation remains owned by the accepted installation task. The adapter does not drop a publishing future or convert an indeterminate publication into cancellation. The integration owner must demonstrate cancellation before publication and explicit settlement after publication wins.

No user profile, baseline mutable library, legacy source, Cargo manifest, module registration, or generated file was modified by this package.
