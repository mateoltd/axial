# Quilt loader handoff

Status: implementation supplied; integration and executable acceptance pending.

`core/app/src/loaders/quilt.rs` is the feature boundary. It uses the retained
`axial_minecraft` provider, catalog and installation code; it introduces no
parallel profile parser, Java rules, public DTOs or filesystem authority.

## Interface

- `fetch_supported_versions(&ManagedLibraryOperation)` returns
  `(Vec<LoaderGameVersion>, LoaderCatalogState)`.
- `fetch_builds(&ManagedLibraryOperation, minecraft_version)` returns
  `(Vec<LoaderBuildRecord>, LoaderCatalogState)`.
- `fetch_cached_builds(&ManagedLibraryOperation, minecraft_version)` returns an
  optional fresh catalog with the same records and state.
- `resolve_build(build_id)` resolves an opaque Quilt build against live provider
  authority. Invalid IDs and IDs for another loader fail before provider I/O.
- `install_build(&ManagedLibraryOperation, ManagedRuntimeCache,
  LoaderBuildRecord, FnMut(DownloadProgress))` returns the retained
  `LoaderInstallPublicationOutcome` or `LoaderInstallError` without flattening
  owned checkpoints or unresolved publication recovery.

Dependency: `axial-minecraft`; tests additionally require its `test-support`
feature and the existing `tempfile` dependency. The composition owner registers
`loaders::quilt`; the queue and shared installer own admission, progress,
cancellation, base activation and continuation. A base receipt is not a completed
Quilt installation. The retained continuation already contains the exact loader
plan, so it remains shared instead of being rewrapped per loader.

## Retained behavior

The leaf's Quilt provider uses the v3 metadata endpoint and preserves game
`stable` hints. Build labels, ordering and installed/build identity encoding use
the same retained catalog and metadata algorithms.

Installation refreshes the selected exact build and rejects record drift. Live
proof must match the loader version, game version, canonical profile ID, parent,
client main class and all three Maven coordinates: `org.quiltmc:quilt-loader`,
`org.quiltmc:hashed` and `net.fabricmc:intermediary`. The authenticated base keeps
its Java, client, assets and logging declarations. Profile attempts to override
those declarations fail. Library merging, platform selection, required-library
selection and conflicting merge-key rejection remain in the retained leaf.

For Quilt metadata, SHA-1 and positive size are either both provided or both
absent. Complete pairs are checked and normalized; partial or malformed pairs
fail. Absent pairs require fresh bounded library transfer and archive validation,
then the resulting exact SHA-1 and size are sealed into published metadata and
the receipt. An unchecked existing checksumless JAR cannot establish readiness.

Source references retained from the baseline:

- `legacy/core/minecraft/src/loaders/providers/quilt.rs`
- `legacy/core/minecraft/src/loaders/index/{query,normalize,cache}.rs`
- `legacy/core/minecraft/src/loaders/strategies/common.rs`
- `legacy/core/minecraft/src/known_good_libraries.rs`
- `legacy/core/minecraft/src/loaders/compose.rs`

## Verification handoff

Authored wrapper tests exercise component rejection before provider I/O,
fresh-cache ordering with preserved exact records and stability evidence,
component-isolated caches, mismatched cached identities, and invalid install
rejection without progress or version-directory creation.

The feature worker ran a non-writing `rustfmt --check` parse/format check.
Shared Cargo commands are reserved for the composition owner and have not been
run by this worker. Suggested focused commands, with output captured and only
tails reported:

```sh
cargo test -p axial-app loaders::quilt::tests
cargo test -p axial-minecraft --lib quilt
cargo test -p axial-minecraft --lib profile_source_rejects_identity_drift_and_base_owned_overrides
cargo test -p axial-minecraft --lib profile_sealing_rejects_partial_pairs_fabric_pairs_and_merge_key_collisions
cargo test -p axial-minecraft --lib profile_reconstruction_matches_install_and_leaves_all_managed_state_untouched
```

These retained checks include nested mapping hashes, absent/partial integrity,
fresh checksumless Quilt JAR publication and SHA-1 receipts, authenticated base
Java override refusal, incompatible library declarations and real profile
installation/reconstruction equivalence. They are supplied checks, not a claim
that the replacement has passed them.

Remaining acceptance: compile and run the supplied tests against the replacement
leaf; integrate the real queue/API with base and child activation; verify
cancellation and restart settlement; install and launch representative Quilt
versions with Java/library incompatibility failures through the replacement.
Neither source reuse nor a base checkpoint establishes runnable parity. No
legacy source, manifest, root registration, installed profile or user library
was modified by this feature worker.
