# Forge loader

Status: one real modern tuple, Forge 47.4.10 / Minecraft 1.20.1, passes install, startup and launcher Stop. This is not all-era or gameplay parity. Latest verification passes **46 binder / 19 execution / 8 workspace / 787 app / 105 API tests** (`.rewrite-logs/forge-scratch-*.log`). Earlier handoff details below are historical; current composed evidence is in [integration](integration.md).

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
