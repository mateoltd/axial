# Quilt loader handoff

Status: authenticated-sidecar correction, composed checks and real install/startup/Stop/reopen pass. Native gameplay and the representative-version matrix remain incomplete.

## Current integrity diagnosis

Read-only official-provider recheck at 2026-10-04T14:03Z reproduces the Quilt0.30.1/Minecraft1.20.1 mismatch. It also identifies a specific upstream cause: [MavenRepository.fileInfo](https://github.com/QuiltMC/update-quilt-meta/blob/cf90889394779ac7689a7b36f055695dc7893c56/src/main/java/org/quiltmc/meta/update/util/maven/MavenRepository.java) hashes checksum-file response bytes instead of decoding their text. [HashFunction](https://github.com/QuiltMC/update-quilt-meta/blob/cf90889394779ac7689a7b36f055695dc7893c56/src/main/java/org/quiltmc/meta/update/util/hash/HashFunction.java) confirms that operation.

The exact [Meta proof](https://meta.quiltmc.org/v3/versions/loader/1.20.1/0.30.1) SHA-1 equals SHA-1 of each raw40-byte official sidecar; that text equals the independently downloaded JAR's SHA-1:

| Coordinate | Meta SHA-1 / hash of raw sidecar | Sidecar text / downloaded JAR SHA-1 |
| --- | --- | --- |
| `org.quiltmc:quilt-loader:0.30.1` | `d057a81ff6252a795ba688ddd2431e5f88df1b35` | `2fa21d6438f63c8dd40f2e81290b0a09de1442b3` |
| `org.quiltmc:hashed:1.20.1` | `24d6332c2ed45545c8c8622ecb14ccc3022cb873` | `2348b0ec7955c354ecb49f700b8a7b3f78e65a75` |

Matching SHA-256 and SHA-512 chains are independently reproduced too. Canonical sidecars are [loader.sha1](https://maven.quiltmc.org/repository/release/org/quiltmc/quilt-loader/0.30.1/quilt-loader-0.30.1.jar.sha1) and [hashed.sha1](https://maven.quiltmc.org/repository/release/org/quiltmc/hashed/1.20.1/hashed-1.20.1.jar.sha1). No artifact was executed or persisted, and no profile/source behavior changed during this diagnosis.

The existing Quilt proof owner now authenticates canonical raw sidecar bytes against Meta before decoding the artifact SHA-1. At most three required coordinates use the existing bounded transport, with a 40-byte body limit and fixed repositories; version data remains escaped. Missing, oversized or unauthenticated sidecars preserve the original direct-artifact pair. Authenticated malformed text refuses. Exact Meta size, coordinate, downloaded JAR verification and publication/receipt owners remain unchanged. No new dependency, cache, wire contract or recovery layer is added.

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

## Verification

Root serialized checks pass **977 Minecraft /768 app /105 API /79 desktop**, with five app/seven API ignored helpers (`quilt-sidecar-{leaf-full,app-api-full,desktop}.log`). Independent final source review is clear. Scoped formatting and diff checks pass; no frontend/schema change requires regenerated contracts.

Six real loopback provider regressions cover authenticated raw bytes, direct-hash fallback, absent/partial tuples, malformed authenticated text, escaped version paths and dot-segment refusal. The composed installer verifies the resulting JAR and sealed metadata/receipt. Authenticated but wrong artifact digest or size reaches actual download and rejects publication. Chained historical mapping proofs reconstruct the installed activation sources exactly, without modifying managed files. Metadata-only reconstruction is not fresh artifact verification.

Meaningful RED is **11 pass/6 fail** against the original provider (`quilt-sidecar-red.log`), including the real installer checksum refusal. An undeclared `hex` call is corrected using the existing digest formatter without adding a dependency. A stale total-request assertion is corrected to distinguish metadata from sidecar reads; neither diagnostic is a behavioral RED. Final broad checks above pass.

Actual provider acceptance uses a fresh generated profile, exact Quilt0.30.1/Minecraft1.20.1 selection and the production API/queue. Initial harness mistakes used the loader display key instead of its wire component ID, then assumed a nested create response although it is flattened. The first refuses before mutation; the second occurs after one accepted creation. That existing instance/install is reconciled through read-only owner status, not another creation. These are harness errors, not product failures or acceptance proof.

Real official Quilt0.30.1/Minecraft1.20.1 installation succeeds through the production API/queue (`quilt-sidecar-live-observe.log`). Fresh profile `/private/tmp/axial-quilt-current.N1uZ6y/profile`, instance `4ceac07d-63e3-462e-baed-b5d16ea1cb36`, install `ca11db4f-b8b5-400b-ada7-7ed767285123`, operation `ed2075a9-6e4c-4ec5-918c-8a6e12f5be21`; terminal outcome succeeded and launch action Ready/Java17. API binary SHA-256 `d84dbe50719fd7fa8ca44948d5dadcd6e759422fdde4179912ec17ebd304c7cd`, provider source SHA-256 `643002fcc2734d1988227a1f41474b1856211df3e39abf9dad10387be63ee644`. Published loader and hashed JAR SHA-1/size equal the authenticated pair above: 3149891 and 798778 bytes. Sibling canary hash/native identity remains unchanged.

Actual offline launch uses synthetic `QuiltParity` and intent `6106aad2-a292-4c55-a1f4-18d8047f8afc`; session `3e804c7c-117e-4a62-8c2a-ce9438063fbc` reaches Running with JavaPID15647 and observed boot. Public redacted logs contain Quilt loading, LWJGL, OpenAL initialization, sound engine and atlas markers (`quilt-sidecar-live-{launch,session}.log`). Normal API Stop settles the tree/output, publishes stopped outcome, returns Ready and records matching durable report/terminal_ack1. Java is gone. SIGINT invokes normal joined API shutdown, exit0; ordinary reopen retains the same instance/version, synthetic account and Ready/Java17, with zero current sessions, then closes normally exit0 (`quilt-sidecar-live-{stop,reopen,reopen-observe}.log`). Published durable inventory stores the exact loader/mapping SHA-1 and size above. Sibling canary SHA-256 `5474cd8b8da140d1a758f371e4fe3f14a840b9c9f856db65aea6ce0a73d81a5c` and dev/inode/size/mtime `16777229:160426076:82:1791126315` remain exact. Generated profile/evidence are retained.

These production API observations prove real startup/lifecycle, not native Play or menu/world/save interaction: computer use is locked. Other representative versions, cancellation/interruption and native gameplay remain required. No legacy profile, user library or live native bundle image is modified by this correction.
