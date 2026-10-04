# Native Quit

2026-10-04. Bounded unsigned macOS ARM64 debug acceptance, not installed-release, gameplay or full termination coverage. Evidence is under `.rewrite-logs/`; only the generated `/private/tmp/axial-native-current.YwnWYB/profile` is used.

## Observed bypass

The `ba05bca6` bundle builds successfully (`launch-checkpoint-native-package.log`, executable SHA256 `6539d48702d834e96a38cb10c954eb20344163b5325990aefb38011450d88091`). NativePID54468 opens the exact fixture metadata and restores six instances/NativeParity. Ordinary Launch of copy3 (`c71d6d5d-1e94-4f44-8cb4-7909c3181346`) reaches Playing/Last played just now with Java57260. Accepted intent `2ca9268a-e215-492e-b5ed-c707e61dcfe1` maps session `29b5199b-e530-4e4c-be50-1da0f3f5a084`, without report, settlement or terminal acknowledgement.

Actual application-menu Quit exits the native parent0 instead of refusing Busy. Java survives with parent1; accepted evidence remains byte-identical. This is meaningful native RED, not graceful shutdown. A subsequently observed PID57732 has no explicit fixture-profile environment and displays a metadata startup error; its origin is unproven and is not evidence of an application-requested restart. Its error window is closed without retry. Only the exact orphaned test Java is then terminated, after executable/cwd/no-child verification. Its receipt and native scratch are retained without fabricated settlement. Named file witnesses remain unchanged; this is not whole-profile immutability.

Independent review finds the pinned platform bypass: Tauri2.11.2 default Quit uses Muda0.19.3's AppKit `terminate:`; Tao0.35.3 translates application termination directly into loop destruction, and Wry emits Exit rather than cancellable ExitRequested. The existing post-loop cleanup is not an AppKit termination veto. The earlier assumption that all native Quit paths shared Axial's close fence was incorrect.

The RED verifier checks accepted binding against the current target, not a frozen pre-Play target tuple. It proves accepted/unsettled metadata, not process liveness; separate UI/process observations supply Playing and the surviving child.

## Narrow correction

The existing desktop entrypoint retains the default menu, replaces only its validated application-submenu Quit item with a normal menu command, and delegates that private ID to the existing close owner. Native text, position and Cmd-Q remain; the application's content UI is unchanged. No lifecycle state, process adoption, recovery owner, dependency or retry is added. Unexpected menu shape fails construction instead of silently retaining unsafe Quit.

All79 desktop tests pass (`native-quit-menu-desktop.log`), scoped formatting/diff checks and independent source review pass. Those tests exercise existing lifecycle owners, not an AppKit menu. The corrected bundle builds (`native-quit-menu-package.log`, executable SHA256 `667e13e04cddc2869f37d71524e255b8c11d150ad1ca344a7a45be1cae66cc7d`), with frontend generation `c3c685ff26c3`.

## Actual corrected journey

NativePID68412 restores the same profile, keeping copy3 Unavailable and its exact unacknowledged receipt/native scratch. No unknown process is adopted or obligation cleared. Copy2 (`8e716bac-96c5-47a0-a09e-e6e1323b4ee8`) remains Ready. A fresh ordinary Launch reaches Playing with Java72201, intent `42cde921-8d62-4a25-b9f2-95db6e8ec2f0`, session `08f327a9-6f0b-412c-b7af-c90d4510c438`.

Actual menu Quit and Cmd-Q independently show the exact Busy toast. Both original processes remain alive, and accepted/unacknowledged proof snapshots are identical. The observed Quit action changes from `terminate:` to the ordinary menu callback. Ordinary Stop remains usable and returns Ready/Idle with the stopped notice. The v4 report, terminal acknowledgement and v1 settlement authenticate the same accepted payload, record owner-observed stopped-child settlement/boot3879ms, and agree on identity, times and exit code. Java is absent. Fresh menu Quit exits0; the settled snapshot remains byte-identical after exit.

The separate GREEN verifier freezes the target's binding tuple before Play and checks ordinary-launch context, exactly one added intent/report and all prior history hashes (10 intents,9 reports,4 drivers/suites). It preserves RED's unresolved evidence. Native scratch returns to the baseline's one retained directory/name/device/inode, not global emptiness or unchanged old contents. Protected named file snapshots match. It uses bounded read-only transactions and emits only admitted identifiers/hashes, not private launch payloads. Independent review catches and corrects the target-tuple and retained-scratch witness gaps before GREEN Play.

Actual same-profile nativePID74032 reopens the same corrected bundle and visibly restores NativeParity/copy2 Ready/Idle without another game. Exact settled proof/prior-history hashes, target tuple, retained native entry identities and named files agree. Normal Cmd-Q exits0; final snapshots remain identical to reopened snapshots, and all test native/Java PIDs are absent. The current app keeps the unrelated interrupted copy3 unavailable instead of guessing settlement. Screenshots `native-quit-menu-green-{playing,refused,stopped,reopened}.jpg` record the bounded journey. The slow debug reopen has a one-second process sample, not a controlled latency comparison or diagnosed cause.

Dock Quit, other AppKit/OS termination requests, installed platforms, arbitrary active-process interruption and gameplay remain separate. Menu/Cmd-Q coverage must not be presented as protection for all native termination routes.

Hosted [run37234309487](https://github.com/mateoltd/axial/actions/runs/37234309487) passes exact `ba05bca6`, both jobs, not the subsequent menu correction. Full parity remains active.
