# World resources

Status: ordinary backup and rename have real-interface evidence; full resource parity remains incomplete.

Production ownership is `core/app/src/resources/{worlds,service}.rs`, with registered-instance authority and retained accepted work. Legacy source remains untouched.

The 2026-09-28 scheduling correction moves the existing backup body into an awaited blocking worker. Its current-thread regression first fails because the copy blocks the async worker, then passes alongside worker-panic retention and existing backup/rename/delete coverage (`world-backup-scheduling-{red,green}.log`). Dropping the request waiter leaves the exact task, instance exclusion and generation pin retained; shutdown waits until the real copy settles. Source/canary bytes and exactly one backup are checked. Copy limits, native receipts and unresolved-effect retention are unchanged. This is production-owner filesystem/scheduling evidence, not new native-interface or hard-crash acceptance.

On checkpoint `42b2d598`, the generated `/private/tmp/axial-report-parity-V1RoWt/profile` contains a clearly named disposable world-folder fixture in Fabric instance `d6926227-f5e6-47a3-a736-f6c1cef14435`. It has two text files totaling 177 bytes, including a nested canary, and is explicitly not a playable Minecraft world.

The real Worlds view lists the fixture. Back up creates exactly one directory under `backups/worlds`; Rename changes only the original save-directory name. Independent SHA256 checks match both original file proofs in the renamed tree and backup. Old save name is absent, and unrelated Sodium/configuration canaries remain identical. Evidence: `.rewrite-logs/resource-fixture-before.sha256`, `resource-ui-files.log`, `world-renamed.png`.

During a real Fabric launch at Playing, Back up is refused with the in-use cause and no second backup. Stop returns Ready; durable settlement and report acknowledgement are present, with no remaining game child or content batch and healthy SQLite. The API then exits0 normally (`world-playing-refusal.png`, `resource-ui-settlement.log`, `resource-ui-runtime.log`).

Normal reopen on the queued-Resume source and frontend generation `6fcbb7ed8ef3` retains the renamed 177-byte world in the real Worlds view. All renamed-tree/backup/image hashes and unrelated mod/configuration canaries match (`world-restart.png`, `resource-restart-files.log`). That API also exits0 normally. This proves launcher/file behavior, not gameplay, native folder opening, deletion, interrupted backup or every world failure/restart case.
