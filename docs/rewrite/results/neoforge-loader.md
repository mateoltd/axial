# NeoForge loader

Status: NeoForge21.1.252 / Minecraft1.21.1 has recorded real install, startup, launcher Stop, clean restart and relaunch acceptance. Fresh derived reconstruction also passes its composed check. See `.rewrite-logs/neoforge-runtime-acceptance.md` and [integration evidence](integration.md). This is not all-era, native-gameplay or later-source acceptance. The original boundary handoff below is historical.

The feature module is `core/app/src/loaders/neoforge.rs`. It delegates to the retained `axial-minecraft` leaf instead of duplicating provider parsing, authenticated installer binding, processor execution, or publication. No legacy files were changed by this package.

## Retained behavior

- Old numbering maps `20.4.239` to `1.20.4`, `21.0.167` to `1.21`, and `21.11.5-beta` to `1.21.11`. Year numbering maps `26.1.0.7-beta` to `26.1` and `26.1.2.7-beta` to `26.1.2`. Zero-prefixed snapshot builds stay excluded from supported Minecraft mappings.
- A target with beta builds only remains available with an unstable hint. A target with any stable build receives a stable hint. Exact beta build identities remain selectable.
- Catalog reads preserve the retained cache freshness, stale/failure metadata, Minecraft enrichment, and display ordering. Pure metadata parsing preserves the provider result before display normalization.
- Selected builds resolve through live provider authority. The install boundary rejects another component and delegates the full exact live-record comparison to the leaf.
- The installer retains `NeoForgeModern`, the exact installer artifact URL, profile identity binding, processor output contracts, checksum verification, and scoped file publication.
- `LoaderInstallPublicationOutcome` and `LoaderInstallError` remain intact. A base commit still requires activation and continuation; a child receipt still requires verification and activation. Indeterminate publication retains its recovery capability. This module does not produce a boolean readiness result.
- The install callback forwards intermediate progress unchanged, including byte counts at full transfer, but withholds every `done` event. The queue must publish terminal success or failure after activation and acknowledgement settle; leaf progress cannot terminalize the operation early.

## Interface and ownership

The module exposes `COMPONENT_ID`, `minecraft_version_for`, `parse_supported_versions`, `parse_builds`, `fetch_supported_versions`, `fetch_builds`, `fetch_cached_builds`, `resolve_build`, and `install_build`.

Effectful methods accept the retained `ManagedLibraryOperation`; installation additionally accepts `ManagedRuntimeCache`, the exact `LoaderBuildRecord`, and a progress callback. Catalog and installer output/error types come directly from the retained leaf. The install queue owns generic base activation, `continue_install_build_after_base`, child activation, cancellation, and retained error settlement.

Required shared changes: register `loaders::neoforge`, add the already agreed `axial-minecraft` dependency, and expose the retained NeoForge provider parser functions. The installation integration owner owns those changes. This package adds no other dependencies or authored public wire DTOs.

The shared owner has now exposed the requested pure parser functions. Static readback confirms the wrapper signatures match. The existing development dependency enables the retained `test-support` feature for the isolated managed-library install rejection test.

## Historical processor compatibility inspection

Compared the retained `loaders/forge_installer.rs`, `loaders/bound_processors.rs`, and `loaders/strategies/common.rs` against their legacy counterparts with `cmp`; each returned exit status 0. The shared processor path was preserved byte for byte.

NeoForge continues to accept an empty client workload or a runnable client workload with authenticated output contracts. The existing `UnsupportedMissingOutputs` disposition applies when at least one client processor has an empty output map. That baseline limitation produces an explicit invalid-profile failure before child install effects. Missing terminal sizes do not create a blanket rejection: the retained runtime execution verifies observed output bytes. No additional numbering-era, beta, processor, or version policy cutoff was introduced.

## Original verification handoff

Authored eight application tests covering both numbering eras, beta-only/mixed target stability, exact beta selection, artifact/profile identity, absent-target behavior, wrong-component install against an isolated scoped library with no publication/progress, invalid/foreign selection rejection before provider I/O, and withholding terminal callback events while preserving intermediate progress. Ran `rustfmt --edition 2024 core/app/src/loaders/neoforge.rs` and the focused `git diff --check`; both completed successfully. No Cargo or shared build command was run by this package owner.

The follow-up progress review found the same raw-callback forwarding in Fabric and Quilt and notified their owners. Forge and Vanilla already guard terminal callbacks. The retained leaf base-install path itself suppresses base `done` events, while each child continuation emits `done` immediately after returning its publication receipt, before application activation. Therefore the shared queue/continuation boundary also needs the same suppression; this was reported to the installation integration owner and is not claimed verified by the NeoForge callback unit test.

Integration owner should run and record:

```sh
cargo test -p axial-app loaders::neoforge::tests
cargo test -p axial-minecraft unsupported_neoforge_processors_are_rejected_without_install_effects
cargo test -p axial-minecraft every_installer_strategy_rejects_sha1_mismatch_before_base_effects
cargo test -p axial-minecraft binds_representative_real_format_forge_neoforge_and_legacy_profiles
cargo test -p axial-minecraft spec_zero_forge_binds_outputs_while_outputless_neoforge_is_unsupported
cargo test -p axial-minecraft missing_size_processor_reconstruction_matches_install_for_supported_shapes
cargo test -p axial-minecraft loader_child_publication_preserves_recovery_authority
```

These retained tests are executable coverage candidates, not claimed passing evidence. They exercise source/profile authentication and processor rejection before effects, or retained publication authority. They do not establish a complete NeoForge install-and-launch journey by themselves.

Outstanding evidence: application compilation with its real consumers; recorded focused test results; representative real NeoForge install/launch across supported numbering eras; API queue publication/activation and failure settlement; cancellation/restart behavior with the replacement lifecycle. Full feature parity is not claimed.
