# Fabric loader

Status: implementation supplied; shared compilation, queue integration, and installed launch evidence pending. This is not a feature-complete or parity-complete claim.

## Scope and interface

`core/app/src/loaders/fabric.rs` owns the Fabric application boundary. It uses the retained `axial_minecraft` leaf rather than duplicating its provider parser, canonical build identity, compatibility selection, or materialization algorithms. No legacy Application, State, or Guardian orchestration is imported.

The module exports `COMPONENT`, `supported_versions`, `builds`, `cached_builds`, `resolve`, and `start`. Catalog payloads are the existing `LoaderGameVersion`, `LoaderBuildRecord`, and `LoaderCatalogState` types. Installation accepts a `ManagedLibraryOperation`, `ManagedRuntimeCache`, exact `LoaderBuildRecord`, and progress callback, and returns the retained `LoaderInstallPublicationOutcome` or `LoaderInstallError`.

The integration owner must register `pub mod fabric;` under `loaders`. Production requires `axial-minecraft`; the local tests use its `test-support` feature through a development dependency. No additional dependencies or public request DTOs were introduced.

## Retained behavior

- Supported game versions retain provider stability hints, manifest-based Minecraft ordering, and explicit fresh/stale/cache availability.
- Build listing retains complete compatibility checks: exact Fabric loader Maven coordinates, exact Minecraft intermediary identity, and a nonempty client main class. Historical universal main-class strings and current client/server objects remain supported. Incomplete rows remain omitted by the provider parser; conflicting duplicate identities reject the catalog.
- Selection resolves against live compatibility. Cached or caller-authored records never independently authorize installation. `start` rejects another loader component before effects, and the leaf requires the requested record to equal the live provider record.
- Profile materialization retains independent exact profile proof, base/profile composition, verified artifact downloads, authenticated publication, and canonical installed-version identity. Fabric checksumless profile libraries use freshly streamed bytes and sealed integrity evidence in the retained implementation.
- `BaseCommitted` remains intermediate. Shared installation primitives must verify, activate, and acknowledge the base receipt before continuing the exact retained plan. The child receipt also requires settlement before queue readiness. Publication errors retain recovery authority instead of becoming success or cancellation.

## Verification supplied

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

## Remaining evidence

The queue and shared primitives must integrate receipt settlement and lifecycle ownership before this package can become complete. Real representative historical/current installations must then launch through the replacement coordinator and survive restart. No live provider download, real Java launch, cancellation race, or restart workflow has been executed by this package worker. Source reuse and cache tests do not establish those outcomes.

Cancellation remains owned by the accepted installation task. The adapter does not drop a publishing future or convert an indeterminate publication into cancellation. The integration owner must demonstrate cancellation before publication and explicit settlement after publication wins.

No user profile, baseline mutable library, legacy source, Cargo manifest, module registration, or generated file was modified by this package.
