# Mod files

Status: implementation in progress; no integrated parity claim.

Owned implementation: `core/app/src/resources/mods.rs`.

## Retained behavior

The resource list preserves `{name, size, modified_at, enabled}` with UTC RFC 3339 timestamps. It lists regular `.jar` and `.jar.disabled` files, orders them by the portable enabled basename, and rejects ambiguous case/Unicode or enabled/disabled aliases. The baseline scan budget is 50,000 entries and 1 TiB of file sizes.

Toggle responses preserve `{status: "ok", name, enabled}`. Delete responses preserve `{status: "ok"}`. Toggle destinations cannot replace existing files. An exact managed file must be removed through content operations. A file that was replaced before admission is treated as local only when its current bytes do not match the stored ownership proof; its stale provenance remains untouched. Replacement after observation must fail without altering the replacement.

Baseline references:

- `legacy/apps/api/src/application/instances/resources.rs`: mod listing, request/response types, limits and portable-name validation.
- `legacy/apps/api/src/application/content/operation.rs`: retained local mutation lifecycle and content transaction execution.
- `legacy/core/content/src/managed_transaction.rs`: observed-byte ownership, toggle projection, managed delete refusal and manifest settlement.
- `legacy/apps/api/src/application/instances/tests.rs`: toggle/no-op/conflict, zero-byte file, managed provenance and preexisting drift behavior.

## Interfaces

Initial DTOs are `InstanceModInfo`, `UpdateModRequest`, `UpdateModResponse`, `DeleteModResponse`, and `ModFilesError` with sanitized HTTP status mapping.

The integration owner owns module registration, routes and dependency declarations. `chrono` is requested for the retained timestamp format. File access must consume a registered instance capability from the instance owner. Mutations must use the content owner's exact file/provenance transaction and retain task, instance and library authority until settlement.

## Evidence and remaining work

Source inspection only so far. Shared Cargo/build commands have not been run by this package owner. Authority-backed listing, content mutation integration, focused filesystem/race tests, API consumers and actual UI validation are still required.
