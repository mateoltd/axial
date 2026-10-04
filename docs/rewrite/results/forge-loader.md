# Forge loader

Status: modern Forge47.4.10/Minecraft1.20.1 and legacy-FML Forge14.23.5.2860/Minecraft1.12.2 have qualified real install/startup/Stop/reopen evidence. This is not all-era or gameplay parity. Current runtime-slice verification passes **980 Minecraft /768 app /105 API /79 desktop tests**. Earlier handoff details below are historical; current composed evidence is in [integration](integration.md).

## Legacy FML runtime acceptance

On 2026-10-04, production installation of14.23.5.2860 first fails before the installer with `runtime_source/metadata_invalid`. Mojang's authenticated macOS `jre-legacy` manifest includes a valid empty `lib/security/trusted.libraries` with size0 and SHA1 `da39a3ee5e6b4b0d3255bfef95601890afd80709`. The runtime owner had rejected every size0 file. The narrow correction requires the exact empty digest, uses existing awaited authenticated managed import and retains positive transfer bounds, compressed verification, exact whole-tree admission, cancellation cleanup and cache/rebuild. No new provider, persistence owner or dependency is added.

Full verification passes980 leaf/768 app/105 API/79 desktop; scoped formatting, diff checks and independent final review pass (`runtime-empty-*` logs). Raw/compressed fixtures exercise authenticated acquisition, staging, publication, exact cache verification, tamper refusal and rebuild. Cancellation follows observation of the actual staged empty file. Two positive cases reproduce pre-fix refusal; negative admission already passes. The improved post-write cancellation is GREEN evidence, not an earlier RED claim. The initial desktop `--lib` invocation has no target and is corrected before the full79-test run.

The rebuilt API binary SHA256 is `f887659ce47fe5f247a47332462251af037174e979c1d1724b47050e75ecfe07`. It reuses only generated profile `/private/tmp/axial-quilt-current.N1uZ6y/profile` and instance `d80ce6df-b723-490b-be1e-a7c4c1b54291`. Previous install `88ec71aa-81a3-44f9-bd79-d2a5215ddded` is read back as settled failed before normal retry. New install `d4dfacf4-074a-4f75-a592-9b8f25d2a2a8`, operation `33c3338f-a261-4e59-924f-1c59f88bc9ea`, succeeds; the official managed runtime's exact empty file is present. Immediate detail initially reports not launchable, while later ordinary reads report Ready. No instantaneous-readiness guarantee or causal fix is inferred.

Mojang's8u74 runtime remains below the unchanged8u312 launch floor. Test-only Temurin8u504-b01 x64 JRE comes from official Adoptium metadata/GitHub bytes, archive SHA256 `f06abadad0fa97e04866d7229a2baad53261579ab5d44875ffa8846a28f28d86`; archive paths, bundle signature and real translated probe pass. The existing instance override references `/private/tmp/axial-java8-fixture.ffU40a/jdk8u504-b01-jre/Contents/Home/bin/java`; no global installation, fallback provider or floor relaxation occurs. Durable revision3 retains it; public instance detail deliberately redacts the path.

Launch intent `1ce2c81e-f952-4ca8-866b-21d814b7376b` accepts session `78450d4d-9455-48b0-a906-22fe95ee71e2`, JavaPID19052. Running/boot, Forge loaded-mod, LWJGL, sound and texture markers pass. A single nonfatal exception belongs to offline Realms authorization. Normal launcher Stop leaves tree settled/output drained, outcome `stopped`, matching durable report with16106ms boot duration and `terminal_ack=1`; Java is gone. API3885 and reopened26145 both exit0 after normal SIGINT. Ordinary reopen retains Forge1.12.2/14.23.5.2860, Java8, Ready and zero sessions. Sibling canary SHA256 `5474cd8b8da140d1a758f371e4fe3f14a840b9c9f856db65aea6ce0a73d81a5c` and dev/inode/size/mtime `16777229:160426076:82:1791126315` remain unchanged.

Evidence: `forge-legacy-{retry-runtime,retry-install-accepted,launch,session-observe,stop,settled,reopen-runtime,reopen,error-final}.log`. Helper wire/queue assertions fail after known accepted configuration/install/Stop effects; observation is reconciled, not blindly replayed. Computer use lists `Launch/net.java.openjdk.cmd` but cannot bind it by observed name/identifier; no menu/world/gameplay evidence is claimed. Profiles and diagnostics remain retained.

## Historical failures and subsequent acceptance

The fresh isolated UI run of Forge 47.4.10 / Minecraft 1.20.1 passed base acknowledgement and installer download, then failed with `identity_mismatch`. The exact official installer SHA-1 is `66bfea9963bfa60d88bab6b2750e74a958392715`, matching its official sidecar. Its `install_profile.json` has `path: null`; the rewrite incorrectly required a shim coordinate despite independently matching profile, version, Minecraft, effective parent and universal-root identity.

The narrow correction accepts null/omitted path only for ForgeModern. Conflicting present paths, all other identities, NeoForge policy and spec-zero Forge requirements remain strict. Its original 42 installer tests and 25 application-loader tests passed (`forge-null-path-tests.log`, `forge-null-path-app-loaders.log`); the later 46-test binder suite includes this correction.

Five of six applicable client processors omit explicit `outputs`. Their mapping intermediates include text files; final `PATCHED_SHA` exists in authenticated data despite the binarypatcher's missing outputs map. Official launch code requires **MC_SRG, MC_EXTRA and PATCHED**, while MC_SRG has no provider-declared digest. The binder recognizes this exact recipe, keeps all three runtime artifacts and distinguishes authenticated provider digests from execution-bound derived provenance. Native preparation reuses exact MCP extraction and bounded transfer against retained Mojang metadata. The later composed checks and real acceptance below supersede this original implementation gap. Unknown outputless recipes remain refused. The first API shut down normally with exit zero after both failed attempts; profile/evidence remains available (`rules-retry-runtime-acceptance.md`).

Real retries exposed and corrected Java probe parsing, then identified official jarsplitter 1.1.4's two `.cache` files. The narrow scratch-output correction preserves exact publication checks and generic recipes. Rebuilt API 20062 completed all six processors/publication, launched real Forge to initialization and renderer startup, and recorded an acknowledged stopped report after UI Stop. Restarted API 22544 restored Ready and last-played state, relaunched to Playing for over a minute and settled another acknowledged stopped report. Both APIs exited zero normally; the isolated profile is stopped (`forge-processors-runtime-acceptance.md`).

## Behavior

The current implementation and remaining verification are in [processor contracts and acceptance](forge-processors.md), including the derived SRG proof and all three generated runtime artifacts.

`core/app/src/loaders/forge.rs` admits immutable Forge selections and delegates to the retained `axial-minecraft` leaf. It uses the existing provider, identity codec and installation algorithms. There is no Minecraft version floor and no replacement installer parser.

| Retained strategy | Materialization |
| --- | --- |
| Earliest Forge | Authenticated client/universal ZIP overlay into the child client, preserving the base client |
| Legacy FML installer | Authenticated installer profile and embedded Maven artifacts, including child-only META-INF stripping |
| Modern Forge | Authenticated installer declarations, bound processor inputs, contained execution and verified declared outputs |

The retained provider handles Maven order, Minecraft prerelease coordinates, latest/recommended promotions and optional-promotion failures. The feature validates canonical opaque build identity, canonical child identity, component, strategy, artifact kind and the exact provider-derived source before accepting a selection. Structural preparation does not establish freshness or grant file authority: the installation leaf resolves the live record again and authenticates downloaded bytes.

`install_base` and the queue's canonical `install_build` entrypoint return the original `LoaderInstallPublicationOutcome`. A base checkpoint cannot be reported as completed Forge installation. The coordinator must activate its exact receipt and consume the resulting `LoaderInstallBaseContinuation` through the retained child installer. Failed processors, download failures and publication uncertainty retain their original error types; no fabricated ready state replaces them. Leaf terminal progress callbacks are withheld until the queue settles activation and publication. Request cancellation must remain owned by the install queue until cleanup/publication settles.

## Integration contract

- `PreparedForgeBuild::prepare(LoaderBuildRecord)`, `record`, `into_record`, and `install_base(&ManagedLibraryOperation, ManagedRuntimeCache, progress)`.
- `install_build(&ManagedLibraryOperation, ManagedRuntimeCache, LoaderBuildRecord, progress)` provides the canonical queue entrypoint; `resolve_build` returns its selected record.
- `validate_record`, `fetch_supported_versions`, `fetch_builds`, `fetch_cached_builds`, and `resolve` retain leaf DTOs and provider error classifications. Catalog queries accept the admitted `ManagedLibraryOperation` and preserve `LoaderCatalogState`, including fresh, stale and cache-hit status.
- The version catalog owns freshness, cache projection, Minecraft release metadata and ordering. The queue owns accepted work, cancellation, base activation, child continuation, terminal publication and restart recovery.
- Required retained exports: `loaders::api::validate_loader_build_record_identity` and `loaders::providers::{forge_install_source, apply_forge_promotion_selection, infer_loader_build_metadata}`. The latter two are used by focused compatibility tests.
- Required dependency: `axial-minecraft`; tests use its `test-support` feature and the existing Tokio and tempfile dependencies. No new public wire schema, generated types or generic installation framework.

The installation owner owns retained-leaf edits; the composition owner owns module registration and shared builds. This package modifies only its source file and this result record. Legacy sources remain unchanged.

## Verification

Completed: `rustfmt --edition 2024 core/app/src/loaders/forge.rs` passed. This establishes syntax/formatting only.

Focused app tests supplied for serialized integration execution:

```sh
cargo test -p axial-app loaders::forge
```

Eight tests check exact archive/installer source URLs from 1.1 through modern and prerelease coordinates, reject swapped build/child/component/strategy/source authority, require distinct exact base/child identities, preserve promotion ranks (including unstable recommendations), and reject malformed or other-component selections before provider I/O. Real temporary-library checks exercise promoted build-cache persistence across the three installer eras and invalid-source rejection without file or progress effects. A progress check prevents a leaf publication/error callback from prematurely completing the queue.

Retained real filesystem/process tests requested from the installation owner:

| Test filter in `axial-minecraft` | Required evidence |
| --- | --- |
| `legacy_archive_overlays_base_client_without_mutating_base` | Real ZIP overlay and untouched base |
| `strip_meta_legacy_installer_install_strips_child_not_base_client` | Real legacy installer outputs and untouched base |
| `installer_base_continuation_acquires_source_only_after_base_checkpoint` | Installer source ordering after exact base activation |
| `legacy_base_continuation_acquires_source_only_after_base_checkpoint` | Archive source ordering after exact base activation |
| `run_failure_precedes_writes_and_cleans_workspace` | Failed processor cannot publish child files |
| `zero_source_legacy_managed_install_survives_cancellation_and_settles` | Cancellation does not abandon owned publication |
| `contained_nonzero_cancel_and_output_limit_are_reaped` | Process failure, cancellation, bounded output and reaping |
| `loader_child_publication_preserves_recovery_authority` | Uncertain child publication retains exact recovery authority |

Results remain pending until the integration owner runs these tests. Source reuse and unit fixtures do not establish live queue/API behavior, restart settlement or platform parity. Full Forge completion still requires those consumers and real processor/publication evidence on the supported platforms.
