# NeoForge loader

Status: current optimized native NeoForge21.1.256 / Minecraft1.21.1 passes the qualified ordinary lifecycle below. Earlier21.1.252 browser/process acceptance and fresh derived reconstruction remain recorded in `.rewrite-logs/neoforge-runtime-acceptance.md` and [integration evidence](integration.md). Neither establishes all-era, native-gameplay or full parity. The original boundary handoff below is historical.

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

## Current ordinary native lifecycle

2026-10-06. The unchanged optimized `5c9fd5ce` candidate, executable SHA256 `f6897fcc42524cb57eeeb39d16210459bbaeea4a556b4957a0cdab09fcdce5f8`, opens only `/private/tmp/native-neoforge.z2aK4i/profile`. Actual onboarding creates offline NeoParity with2048MiB, telemetry/Discord/music off. One ordinary Create selects Minecraft1.21.1 and the explicitly recommended stable NeoForge21.1.256, names NativeNeoForgeParity and disables automatic optimization. Instance `e58e2e11-30ca-4e31-8474-3bc0a04a8f4e`, queue `646a198c-7bd1-44db-8cb5-4d9a976b1f1c` and operation `2c55d9d3-4fb7-4531-87a8-21a61c3ee0a9` remain exactly bound. Downloads progresses through base publication, six-step loader preparation and final publication; terminal succeeded/Final and both ready versions are verified before Launch. Long0/1 phases,99% before terminal, and temporary Unavailable before Ready are observations, not latency fixes or causes.

The frozen bounded collector checks all7955 recorded entries,4010 distinct files and998155996bytes against their manifests and exact file witnesses; inventory digest is `eaaba974abd621dd094fdf207c562a60283ace3989e7a784faacba04475347ea`. The zero-history installed snapshot is `c2610f0954463f0b6946e540a1a61c24a5dca8eaa475fed35e34e090749e46e8`. A separate complete pre-Create settings comparison permits only null/revision0 becoming exact default UI preferences, Downloads/revision2 and the exact instance route/revision3; every other document field stays unchanged. It does not infer defaults from onboarding labels.

Actual Launch reaches Playing with owned Java21 PID67418 parented to native PID81716 under this profile's runtime-delta; a separate same-executable version check confirms21.0.7. Actual launcher Stop returns Ready and persists session `c697b752-9061-428a-a31c-4bc17a182b6d`, boot8230ms, stopped/launcher_stopped, no failure classes or dropped logs and terminal acknowledgement. The schema4 launch report, matching version1 settlement and report payload hash `312c5a3f4ec78139f4c3fd52aa2e5d7917b19b6f6a02b5a57781236f303a643a` verify. Recorded files, protected table hashes, marker/sibling and empty saves stay exact; successful-launch instance/selection recency hashes are excluded from that prelaunch comparison. Ordinary menu Quit exits0. Explicit reopen PID75093 restores NeoParity, the exact loader and Ready without another Launch; its ordinary Quit also exits0. Both native processes and the game are absent. Complete captured post-Stop/Quit/reopen/second-Quit snapshots are byte-identical, SHA256 `62b1284e540349b6d6130d1a5c969737e3acd5c32a3761a51860258904e3fa32`, with zero checked obligations and SQLite OK.

Evidence uses `native-neoforge-*`; independently reviewed snapshot/launch/settings helpers are `98afab94…37a0523`, `91ebda66…58763250` and `94f8ce33…fff7186`. The settings reader was tightened to existing bounded no-follow file access; this is fixture tooling, not a product fix. The native filter reported three versions while captures showed zero, two, then three after input; a mistaken half-scale outside click closed the first modal without accepting Create. Reopening and selecting the visibly rendered row allowed ordinary progress, but establishes no rendering cause or fix. Computer-use inventory still exposes no Java/Minecraft window. No password input, credential access or safety bypass occurs. This is recorded-file/captured-state and launcher/process evidence, not whole-profile preservation, clean narrator startup, gameplay, authentication, independent provider pins, interrupted recovery, all-era or installed-platform/full parity.
