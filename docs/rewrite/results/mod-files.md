# Mod files

Status: implementation in progress; no integrated parity claim.

Listing and resource commands: `core/app/src/resources/{mods,service}.rs`. Provenance mutations and their durable recovery fence belong to `core/app/src/content/install.rs`.

## Retained behavior

The resource list preserves `{name, size, modified_at, enabled}` with UTC RFC 3339 timestamps. It lists regular `.jar` and `.jar.disabled` files, orders them by the portable enabled basename, and rejects ambiguous case/Unicode or enabled/disabled aliases. The baseline scan budget is 50,000 entries and 1 TiB of file sizes.

Toggle responses preserve `{status: "ok", name, enabled}`. Delete responses preserve `{status: "ok"}`. Toggle destinations cannot replace existing files. An exact managed file must be removed through content operations. A file that was replaced before admission is treated as local only when its current bytes do not match the stored ownership proof; its stale provenance remains untouched. Replacement after observation must fail without altering the replacement.

Baseline references:

- `legacy/apps/api/src/application/instances/resources.rs`: mod listing, request/response types, limits and portable-name validation.
- `legacy/apps/api/src/application/content/operation.rs`: retained local mutation lifecycle and content transaction execution.
- `legacy/core/content/src/managed_transaction.rs`: observed-byte ownership, toggle projection, managed delete refusal and manifest settlement.
- `legacy/apps/api/src/application/instances/tests.rs`: toggle/no-op/conflict, zero-byte file, managed provenance and preexisting drift behavior.

## Interfaces

Resource listing uses `InstanceModInfo`; mutation responses use `ResourceCommand` with sanitized `ResourceError`. Content provenance retains its own contracts. File access consumes a registered instance capability. Mutations must retain task, instance and library authority until settlement and reuse the content owner's durable fence whenever a file/provenance transaction is involved.

## Evidence and remaining work

On main checkpoint `93354581`, real browser-driven Fabric 0.19.5 / Minecraft 1.20.1 checks passed for managed Sodium listing, disable, clean restart retaining Disabled, enable, mutation refusal while Playing, Stop, named-mod removal and compatible Discover reinstall. Original/reinstalled bytes match exactly; the final queue is terminal, no content batch remains and the API exits0 normally. Evidence: `.rewrite-logs/mods-runtime-acceptance.md`, `mods-*.png` and runtime logs. This is a bounded ordinary managed-file journey, not gameplay or every mod operation.

The corrected local-file path now reuses the existing content receipt/pending/native-settlement owner. Its private intent is limited to a captured stale-name source and its exact enabled/disabled destination or deletion; raw provenance stays byte-identical. Exact managed delete remains refused. Receipt insertion verifies one affected row and exact transactional readback before any file effects. No new table or recovery owner. Busy admission reports the existing in-use cause. Required managed mods may explicitly be disabled/re-enabled, matching the legacy resource UI; dependency removal remains refused.

Independent review and **33 resource / 5 local receipt / 814 app / 117 API tests** pass (`mods-resources-final.log`, `mods-local-recovery-green.log`, `mods-app-api-final.log`). Coverage includes zero-byte replacements, raw whitespace, source/destination/manifest byte drift, tampered receipts, ignored/altered SQL insertion, cancellation and reconstructed-owner restart fencing. Receipt planning binds bytes; exact file identity is captured by the existing later native transaction. These restart tests do not simulate a hard crash inside native file publication.

The unchanged binary genuinely reproduced the Local-file Disable refusal in the browser (`mods-local-baseline-refused.png`). The corrected binary then disabled the same disposable file, retained Disabled after normal restart, re-enabled and deleted it through the existing UI. Raw provenance and an unrelated config canary stayed identical; no content batch remained and both APIs exited0. Official Sodium was restored afterward with its original hash. See `.rewrite-logs/mods-local-runtime-acceptance.md`. The earlier source red used incomplete Performance test composition and is not isolated causal evidence; its failure log is retained honestly.

## Disabled managed update

On `e7dc745b`, API PID62302/port57456 uses only the generated `/private/tmp/axial-report-parity-V1RoWt/profile`. In Fabric instance `d6926227-f5e6-47a3-a736-f6c1cef14435`, the real Mods view disables Lithium0.11.2, then Update selects the offered compatible0.11.4 release. Queue `dcb70325-ab48-4ab7-a2c5-ec5ea9275f59`, operation `1e2ee5c2-9514-4e8d-8d41-e25da3e8a900`, succeeds. The view retains Disabled on the new file (`mod-update-disabled.png`).

Independent provider-version lookup verifies `iEcXOkz4`, exact 691,483 bytes and SHA512 `31938b7e849609892ffa1710e41f2e163d11876f824452540658c4b53cd13c666dbdad8d200989461932bd9952814c5943e64252530c72bdd5d8641775151500`. Only `lithium-fabric-mc1.20.1-0.11.4.jar.disabled` remains; provenance retains `enabled=false` (`mod-update-verified.json`). Sodium and all three configuration canaries are unchanged; no content batch remains and SQLite is healthy. Old Lithium bytes were replaced intentionally by this managed update; earlier evidence captures their proof.

Normal Ctrl-C exits PID62302 zero. An immediate overlapping reopen was correctly refused while that owner was still releasing; it did not acquire or mutate the profile (`mod-update-restart-runtime.log`). Only after confirmed exit did PID66161/port57708 open successfully (`mod-update-reopened-runtime.log`). The reopened interface retains Lithium0.11.4 Disabled; Enable restores the exact new bytes and provenance flag. Launch reaches Playing and explicit Stop returns Ready. Intent `aa8db1c1-7b18-4297-babf-0d387ab29bc0`, session `a73c7f69-4e3f-4824-b524-035c301a44b3`, has durable settlement and report acknowledgement. No child process or content batch remains, Sodium/configuration canaries still match, SQLite is healthy, and normal API shutdown exits0. Evidence: `mod-update-{restarted-disabled,playing,stopped}.png` and `mod-update-enabled-proof.log`. This proves the ordinary managed update/restart/enable/launch/stop journey, not update failure, response loss, interrupted native publication or game-window/world interaction.
