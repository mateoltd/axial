# Current-app benchmark drivers

2026-10-04. Real same-driver Resume and completed-history reopen pass. This is current-app lifecycle evidence, not predecessor import, performance comparison, gameplay or installed-release acceptance.

## Fixture and controls

Production source is `9011f266`, with API executable SHA256 `cab534c1b11cffdb5aee7a2147efa3e61fd3a220ce1d2b78bfa53c33106d529a`. The existing browser development build connects to the ordinary production API against generated profile `/private/tmp/axial-native-current.YwnWYB/profile`. Its installed Vanilla1.20.2 instance is `41bd3f19-18fe-4716-9427-8a6ca93afd9e`, with offline NativeParity and real managed Java17. Packaged builds omit this developer lab.

Actual UI Start selects this instance, Development and 300 seconds. It creates driver `benchmark-suite-driver-6190037251d44c1b9eb478b232a96ca8` and suite `suite-id-6d2174a10c0b2942`. The baseline contains three existing intents/reports and no drivers/suites. Benchmark descriptors do not alter the instance's configured Managed mode; these runs are not a Vanilla-versus-Managed measurement.

## Completed observations

Run 1 uses intent `916e41c1-36e1-4ad9-b32c-a077c996c49c` and session `18c795c3-dabc-486a-a4fd-3bdf63c8853f`. Actual driver Stop cancels future scheduling while Java PID63333 remains alive. Reload hydrates the session into ordinary Playing/Stop controls. Ordinary Stop terminates Java and publishes a stopped report with 3,209 ms observed boot and terminal acknowledgement. A stopped scheduler can retain the suite's stale Running label; that label is not settlement proof.

Normal API/frontend shutdown exits 0. Actual API restart and browser reload retain the exact stopped driver/suite bytes, request hash, first-run mapping/report/settlement and unreserved pending run 2. The UI exposes Resume on the same driver. Actual Resume is clicked once, not Start. Run 2 uses the original planned intent `b5d806da-2e57-4114-80b7-8130025080a4` and new session `18412d6a-5b16-4839-b99d-fb6a1f240f8a`; Java PID77521 starts under API PID74101. Ordinary Stop after reload settles it with 3,267 ms boot, stopped report and terminal acknowledgement; Java is gone.

The unmodified saved interval reaches natural completion at 19:40:14Z: 2/2 launched, no pending or active run. Another normal API/frontend shutdown exits 0. Actual production API startup and browser reload retain completed history; the lab shows both stopped proofs and the same Complete driver, with neither Resume nor Stop. Exactly two new intents/reports exist, five total; no third game launches. Final servers shut down normally with exit 0, all six API/frontend parents and both Java processes are absent, and the temporary test tab closes.

## Evidenced interface gap

After actual driver Refresh reports run 2's active session and 2/2 launched, the ordinary instance still shows Ready/Launch and global Idle, with no Stop. Reload restores Playing and functional Stop. `current-driver-second-unadopted.jpg` records this before reload. Independent comparison finds the same discovery omission in legacy, so this is not claimed as a removed legacy behavior. A narrow known-session handoff through the existing launch owner is proposed; no automatic discovery, new timer or production correction is yet verified.

## Verification and limits

The ignored read-only `current-driver-verify.mjs` validates bounded SQLite snapshots against the actual UI request and independent two-run plan. `current-driver-{baseline,first,first-reopened}.json` prove exactly one additional intent/report at the first checkpoint, unchanged prior history, same captured request and first-run proof, exact current target binding, authenticated settlement and empty native scratch. Process absence and actual UI action provenance are checked separately; tree/output settlement is backed by the owner's receipt, not a standalone process-tree inventory.

`current-driver-{complete,complete-reopened}.json` validate both current mapped runs and compare the complete driver/suite/request and both receipts/reports across actual restart. The final invocation takes the pre-reopen completion snapshot, not merely the original first-run snapshot. Current target binding, stopped boot/child observations, atomic terminal acknowledgements, unchanged prior history and empty native scratch all pass. Independent verifier/evidence review is clear.

All named historical source, original options, sibling and retained-image witnesses match before/after both games and the final restart (`current-driver-files-{before,first-reopened,second-stopped,final}.json`). These are not whole-tree claims for game-generated files. Screenshots record stopped-first, first-reopened, complete and complete-reopened controls. No SQL mutation, handcrafted success, repeated uncertain write, changed interval, legacy profile mutation or release action occurs.

Hosted [run37227449144](https://github.com/mateoltd/axial/actions/runs/37227449144) passes exact `9011f266`, both application and delivery-contract jobs. This does not replace real benchmark, native or installed-platform evidence. Automatic continuation after interrupted process ownership remains a separate gate.
