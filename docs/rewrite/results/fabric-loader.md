# Fabric loader

Status: integrated. Real Fabric0.19.5/Minecraft1.20.1 install, launch, Stop and ordinary API reopen are qualified in [current-profile acceptance](current-profile-browser.md#launch-stop-and-relaunch). Historical installation/launch and broader interruption/native/installed gates remain open. This is not feature-complete or full-parity acceptance.

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

The original worker performed no live provider download, real Java launch, cancellation race or restart workflow. Subsequent representative modern integration evidence is linked above; historical install/launch and broader cancellation/settlement cases remain incomplete. Source reuse and cache tests alone do not establish those outcomes.

Cancellation remains owned by the accepted installation task. The adapter does not drop a publishing future or convert an indeterminate publication into cancellation. The integration owner must demonstrate cancellation before publication and explicit settlement after publication wins.

No user profile, baseline mutable library, legacy source, Cargo manifest, module registration, or generated file was modified by this package.
