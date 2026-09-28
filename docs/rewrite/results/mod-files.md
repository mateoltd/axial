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
