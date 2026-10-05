# Native gameplay

Updated 2026-10-05. Current-source native acceptance is in progress, not full gameplay or installed-platform certification. Logs are under `.rewrite-logs/`.

## Build configuration tracking

Starting at `f7c7d08f`, an unsigned debug test bundle uses a separate product name, Axial Gameplay Parity, to preserve the running Microsoft diagnostic bundle. An initially rejected identifier override is removed in the next build. Its Info.plist then records the required `com.mateoltd.axial.rewrite`, but the executable still embeds the rejected override and refuses startup at the identity guard, before creating the test profile (`native-gameplay-f7c7d08f-package-identity.log`, `native-gameplay-f7c7d08f-runtime.log`). Raw and bundled executable hashes match; packaging a different executable is not the observed mismatch. Metadata or a successful package command alone would falsely pass this case.

Tauri codegen reads `TAURI_CONFIG` through the procedural macro's environment. The build script tracks that input, but the binary's compiler dep-info did not. The local compiler wrapper is kache; the logs do not establish skipped compilation or rustc-incremental reuse. Two lines beside the existing single context expansion make the input explicit through `option_env!`; no identity guard, lifecycle, schema, public API or UI policy changes.

The edit-forced build opens actual onboarding, and its actual native-menu Quit exits0. The subsequent source-unchanged override regression also passes: wrong identifier A rebuilds and exits1 with the exact identity refusal, leaving `/private/tmp/axial-native-gameplay.E56Jhx/rejected-config-a` absent; removing only that override for B rebuilds without cleaning or bypassing the wrapper and opens actual onboarding again. The B dep-info records `TAURI_CONFIG`. Its executable SHA256 is `fe268a2721237658e836e54bd409f283cb550ce25d083cbb7357e73cad39a84f`. Evidence is `native-gameplay-config-tracking-{green,a,b}-{package,runtime}.log`.

All82 desktop tests pass with one existing helper ignore (`native-config-tracking-desktop-tests.log`); scoped rustfmt/diff checks pass. Independent Standards and Spec source review each finds no issue; the actual override-only startup regression supplies validation beyond those source reviews. The retained Microsoft executable still hashes to `172e87eb318ac9d36616a7adaadeac824cc1972464b9471e016bb17cb20648da` and its process/profile remain untouched.

## Fresh-profile journey

The corrected B bundle ordinarily opens canonical profile `/private/tmp/axial-native-gameplay.E56Jhx/profile` as nativePID31690. Real onboarding creates offline GameplayParity, selects2GiB, retains the existing mood, selects silent music, no telemetry and no Discord sharing, and settles to Home. Actual keyboard traversal focuses the memory slider; Home/two Right keys set2GiB. The automation setter did not change that slider, and an earlier arrow attempt with focus outside it advanced onboarding instead; neither observation establishes a product defect.

Actual Create selects Fabric/Minecraft1.20.1, names GameplayFabricParity, retains2GiB and disables automatic optimization. The real queue selects Fabric0.19.5, completes and leaves no queued installation. The actual instance view then shows Ready and enabled Launch. Instance identity is `6798b0fc-ff87-44bf-b7cd-b3adf6442662`; a bounded registry read agrees on its live identity/version with SQLite quick-check OK.

Actual Launch progresses from Requesting launch to Playing. NativePID31690 owns JavaPID48824 under this profile's admitted java-runtime-gamma; no command arguments are inspected. Computer use lists both Axial apps but no Java game and rejects both `java` and the runtime's independently read bundle identity `net.java.openjdk.jdk`. Playing and a live child are not actual gameplay proof. A manual game-window/world/block-change/save/natural-Quit check is handed to the user; the game remains running. Actual world interaction, natural game Quit, saved-world reload and launcher Quit/reopen remain unproven.

This debug bundle preserves the enforced development identity but is not a signed release artifact. Microsoft authenticated acceptance, four installed architectures, trusted updater/restart and the separately recorded unexplained failures remain open. The full non-Guardian parity goal remains active.
