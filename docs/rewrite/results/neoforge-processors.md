# NeoForge client processors

Status: official 21.1.252 / Minecraft 1.21.1 support passes focused/application/API checks and a real UI install, launch, clean Stop, launcher restart, relaunch and second clean Stop. Native gameplay and fresh derived reconstruction remain distinct open acceptance items. Official inspection preceded actual UI selection and execution; runtime evidence is below.

## Official evidence

The [installer](https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.252/neoforge-21.1.252-installer.jar) matches its [published SHA-1](https://maven.neoforged.net/releases/net/neoforged/neoforge/21.1.252/neoforge-21.1.252-installer.jar.sha1): `804b6fe37b75debaa215008b4506d6650dc1b179`. Diagnostic artifacts are under `/private/tmp/axial-neoforge-diagnostic.aHMSuf`, independent from application profiles.

`install_profile.json` declares spec 1, profile `NeoForge`, version `neoforge-21.1.252`, Minecraft `1.21.1` and omits `path`. `version.json` agrees on identity and parent. Its game arguments bind NeoForge `21.1.252`, FML `4.0.44`, Minecraft `1.21.1`, NeoForm `20240808.144430` and `forgeclient`; the main class is BootstrapLauncher. Installer/version libraries merge without conflicts. All 70 unique external library declarations carry hashes and sizes.

Of ten processors, six apply to clients. Every client processor omits `outputs`; no data entry supplies an output SHA. The exact sequence is:

| Step | Tool | Declared work |
| --- | --- | --- |
| 1 | `net.neoforged.installertools:installertools:2.1.2` | `MCP_DATA`, NeoForm ZIP to `MAPPINGS`, key `mappings` |
| 2 | same | `DOWNLOAD_MOJMAPS`, exact Minecraft and client side to `MOJMAPS` |
| 3 | same | `MERGE_MAPPING`, mappings plus Mojang mappings, `--classes --fields --methods --reverse-right` |
| 4 | `net.neoforged.installertools:jarsplitter:2.1.2` | client JAR to `MC_SLIM` and `MC_EXTRA`, using merged mappings |
| 5 | `net.neoforged:AutoRenamingTool:2.0.3:all` | slim to `MC_SRG`, four declared fix flags |
| 6 | `net.neoforged.installertools:binarypatcher:2.1.2:fatjar` | SRG plus embedded `data/client.lzma` to `PATCHED` |

`MCP_VERSION` is the full `1.21.1-20240808.144430`, unlike the Forge fixture's timestamp suffix. The three mappings are `@txt`; the [NeoForm input](https://maven.neoforged.net/releases/net/neoforged/neoform/1.21.1-20240808.144430/neoform-1.21.1-20240808.144430.zip) is `@zip`, SHA-1 `811e2bd86fa2cda2812e5e8e51d718ea8bd6d3f4`. Its config has spec 4, version `1.21.1`, and exact mappings entry `config/joined.tsrg`, compatible with existing native extraction.

## Required runtime and proof

[FML 4.0.44 sources](https://maven.neoforged.net/releases/net/neoforged/fancymodloader/loader/4.0.44/loader-4.0.44-sources.jar), SHA-1 `49ddb1aae0afcc7dfbaa46f7e557c568fb88a084` matching the official sidecar, establish all three required generated runtime libraries:

- `net.minecraft:client:1.21.1-20240808.144430:srg`
- `net.minecraft:client:1.21.1-20240808.144430:extra`
- `net.neoforged:neoforge:21.1.252:client`

`ProductionClientProvider` requires SRG and EXTRA; `NeoForgeClientLaunchHandler` overlays PATCHED and separately locates the provider-hashed universal artifact. None of the three generated libraries appears in the install/version library lists. None has a provider-expected digest. They must use locally derived, sealed provenance from the exact authenticated process chain, never relabel an observed digest as a provider expectation.

[Jarsplitter 2.1.2](https://maven.neoforged.net/releases/net/neoforged/installertools/jarsplitter/2.1.2/jarsplitter-2.1.2.jar) matches installer SHA-1 `8a7916be0a0e589897beab7c839072631067e48e`. Its [sources](https://maven.neoforged.net/releases/net/neoforged/installertools/jarsplitter/2.1.2/jarsplitter-2.1.2-sources.jar), SHA-1 `8b0c3bcd6db293a3b9440f626dc1536d6cb59779` matching the official sidecar, write an adjacent `.cache` for each output. Those caches belong only to bounded scratch, not the runtime inventory.

## Owned implementation and verification

`forge_installer.rs` recognizes explicit ForgeModern and NeoForgeModern cases within the existing six-step binder. Family-specific coordinates, ordered arguments and runtime identities remain strict. NeoForge's FML argument must match exactly one declared loader coordinate. Absent output hashes share one private per-binding derivation identity; authored hashes remain mandatory when present and conflicting declarations reject. Exactly EXTRA, SRG and PATCHED are terminal, even though SRG is also consumed by the patcher. Recognized download tasks cannot escape native preparation merely by adding output maps. Ordinary generic explicit-output recipes remain unchanged; unknown outputless recipes remain refused.

`bound_processors.rs` owns derived splitter scratch admission and promotion. Existing mapping acquisition, input reauthentication, settled exact diffs, final rescan, output sealing and receipt comparison remain authoritative. Declared-only reconstruction cannot fabricate derived proof; reconstruction reruns the authenticated pipeline. No new public contract, persistent proof journal or publication owner is needed.

All **51 binder / 19 execution / 787 application / 105 API tests pass** (`.rewrite-logs/neoforge-{binding,execution,app-api}.log`), with existing consumer ignores unchanged. Binder regressions cover the official omitted-field shape, all six actions, distinct binding identities, exact three terminals, foreign tools/data/arguments, runtime/profile drift, unsupported workloads, intermediate promotion, and supplied/conflicting hashes without action bypass. The existing actual step-runner fixture covers derived scratch success, missing/empty/wrong-size outputs before either canonical write, unexpected root effects and original-versus-foreign/provider expectation refusal. Independent review, scoped formatting and diff checks pass. All **43 composed strategies pass** (`neoforge-strategies.log`), but their existing NeoForge reconstruction fixture uses one provider-hashed output, not the derived six-step recipe.

## Real isolated runtime

The preserved UI selected exact recommended build 21.1.252, created `NeoForge fresh parity` and completed all six processors/publication. Ready appeared without reload. Actual Java 21/NeoForge/renderer initialization logs accompany Playing; Stop returned Ready and persisted an acknowledged stopped report. After normal API exit, a rebuilt API reopened the same profile, restored exact readiness and successfully relaunched/stopped again. Both APIs exited zero normally, both reports are acknowledged, and no install/launch obligations remain. Evidence: `.rewrite-logs/neoforge-runtime-acceptance.md`, `neoforge-{fresh,restart}-runtime.log`, `neoforge-installed-ready.png`, `neoforge-playing.png`, `neoforge-restarted-stopped.png`.

The profile is `/private/tmp/axial-neoforge-parity-nLdKKb/profile`, independent from the preserved deadlocked loader profile and all user installations. Offline Realms/user-property errors are not online-auth evidence. The game window was not exposed by app control; startup logs are not native gameplay evidence. Ordinary successful restart verifies persisted inventory against exact local files and does not rerun processors.

## Fresh derived reconstruction

The composed `derived_neoforge_pipeline_reconstructs_exact_publication_and_refuses_fresh_drift` test passes (72.71 seconds; `neoforge-derived-reconstruction.log`). It authenticates inputs, executes the actual six-step owner, seals/publishes/acknowledges exactly SRG/EXTRA/PATCHED, then refetches inputs and reconstructs fresh execution evidence. The full activation contract and inventory match; mappings, SLIM and caches are not published. Changing only the fixture patcher's resulting bytes rejects the original activation checkpoint while leaving the managed tree untouched.

Three mappings tests also pass, including invalid metadata, provider mismatch and byte/hash drift (`neoforge-derived-mappings.log`). The per-invocation transport seam is `cfg(test)` only: it connects an exact retained HTTPS descriptor to loopback while keeping the original hash, size and identity checks. Independent review and scoped formatting pass. Synthetic Java tool algorithms establish proof/lifecycle composition, not official-tool determinism or native gameplay.
