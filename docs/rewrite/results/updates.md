# Update discovery and reload

2026-10-07, changes against `ff4dbbe1` on `main`. These corrections cover discovery and frontend recovery, not a signed installed update/restart or full-parity certificate.

## Corrections

The native updater used ordinary SemVer ordering, rejecting dev-to-alpha/beta promotion and permitting reverse-channel offers. Restore the retained numeric-core, dev/alpha/beta/rc/stable and numeric-prerelease ordering in one private comparator shared by production and the actual Tauri plugin fixture. Build metadata does not create an update. Reuse the already resolved workspace SemVer dependency; HTTPS, target, signature/version binding, package bounds, installation and shutdown fences remain unchanged.

Fresh frontend state previously lost access to an already staged package: it checked for another update while the authoritative Ready owner correctly refused Busy. Startup now reads the existing update snapshot, decodes and coherently publishes its info/flow, and checks only when that owner permits it. Check sequence, mutation sequence and flow revision independently fence late publication. Ready hydration issues no command. Previously accepted Downloading/Applying work reuses the existing poll/completion owner; successful installation and normal activity/native shutdown fences still precede restart. No client journal, mirrored lifecycle, new wire contract or UI layout change.

Independent Spec review catches the mock entrypoint's missing snapshot route. Its actual consumer regression goes RED, then the existing mock owner directly composes its state using generated UpdateFlow/Info/Snapshot contracts. No second state owner. Full frontend verification separately catches a stale route-inventory assertion from the prior telemetry correction. The test-only matcher now recognizes the exact current startup wrapper, retaining direct production delegation, both test-input exclusions, unchanged-result return and negative controls; no production composition change or routing guard waiver.

## Verification

- `update-channel-red.log`: one actual plugin check fails dev.5 → alpha.1. Its unchanged18-case test passes in `update-channel-green.log`; full serialized desktop tests pass83/zero failures/one existing helper ignore, wrapper0 (`update-channel-desktop.log`). No test installs a package or requests native restart.
- `update-reload-red.log`: real frontend startup remains Idle despite a retained Ready snapshot. GREEN adds late-snapshot/explicit-check protection; `update-reload-mock-red.log` reproduces the actual missing mock route. Final focused checks pass18/18, wrapper0 (`update-reload-mock-green.log`).
- The first full frontend run has one stale composition-contract failure. Focused contract checks pass6/6 after correction, including the negative controls (`composition-contract-final.log`). Final canonical typed frontend runner passes496/zero failures/one existing TODO, wrapper0 (`update-reload-frontend-final.log`);497 total includes the TODO, not another pass.
- Final frontend build joins0 and publishes generation `80cac045259c`; scoped Prettier, rustfmt and whitespace checks pass (`update-reload-build-final.log`, `update-reload-format-final.log`, `update-channel-format-final.log`). Independent Standards and Spec source reviews are clear after the mock correction. Compiler warnings and the earlier failures remain retained, not silently reclassified.

## Remaining acceptance

Trusted signed artifacts, actual installed download/staging/apply/restart/failure and applicable platform WebViews remain open. Native inventory reports a locked Mac; no signing, Keychain, credential, host policy or installation action occurs. Fixture checks and a frontend build do not close those gates or the full-parity goal.

## Deferred install after activity

Changes after `09bfaced` correct the inherited “Update & restart” promise: Ready previously cleared the requested install intent while games/downloads were active, then stopped polling. Retain that intent in the existing owner and reuse its poll until activity permits Apply. Consume intent only at request admission, before awaiting the result; uncertain responses remain read-only reconciliation, never a replay. Announcements remain transition-only; Ready snapshot hydration still cannot initiate Apply. No new observer, timer owner, state, wire contract or UI layout.

The actual public workflow goes RED when its Ready polling disappears (`update-deferred-red.log`). GREEN passes21 focused checks, including game, active-download and queue controls: no Apply while busy, exactly one after idle, no repeated waiting notice, and restart only after installation. Final scoped Prettier/whitespace and frontend build pass, normal0; generation `9ab22a994e57` (`update-deferred-{green,format-final,build-final}.log`). Independent Standards/Spec source reviews are clear.

The initial full run passes499/zero failures/one existing TODO. A repeat while a separate ordinary frontend build ran hits one unrelated five-second asset-watch timeout (498 passes/one failure/one TODO), retained in `update-deferred-frontend-final.log`. The unchanged generation contract then passes25/25 alone, and a full run without concurrent build passes499/zero failures/one TODO (`update-deferred-generation-control.log`, `update-deferred-frontend-serial.log`, wrappers0). Those passes do not diagnose the timeout; its watcher/event/readiness investigation remains open, with no deadline or guard change.

Hosted [run37558179919](https://github.com/mateoltd/axial/actions/runs/37558179919) passes both jobs at exact `09bfacedf6da5b44d59b2ca878bbcf26f0b53377`, before this deferred correction. The prior `ff4dbbe1` hosted failure matches the corrected stale composition assertion. Watch and exact final query join0 (`update-recovery-{prior-ci-failure.log,ci-watch.log,ci-complete.json}`). This is source-checkpoint verification, not the later source or installed/full-parity acceptance.
