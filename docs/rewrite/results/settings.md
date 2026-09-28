# Settings and feature flags

Implemented the retained configuration fields, local flag overrides and onboarding completion in feature-owned storage and HTTP routes. Guardian fields are rejected. Library paths and telemetry identities remain absent from public configuration.

## Interfaces

- `SettingsStore::new(metadata)` constructs a keyless profile. `SettingsStore::new_with_telemetry_identity(metadata, exporter_configured)` accepts composition's explicit exporter availability and applies the idempotent settings migration.
- `ConfigRouteState::new(settings, accounts, telemetry, tasks)` verifies shared metadata and matching exporter policy. `config::router(state)` returns `Router<()>` with `GET/PUT /api/v1/config` and `POST /api/v1/onboarding/complete`.
- `flags::router(settings, telemetry, tasks)` returns `Router<()>` with `GET /api/v1/flags` and `PUT /api/v1/flags/{key}`.
- All writes require `expected_revision`. Configuration identity edits additionally require `expected_account_selection_revision`, obtained from `ConfigView.account_selection_revision`. Omitted preference fields remain unchanged; explicit null clears nullable preferences. Flag `enabled` must be present; null removes the override.
- `prepare_legacy_import(value)` returns a validated preference/flag preview with a signal for excluded library metadata. It never imports old telemetry identity or grants destination authority. Import commit remains the import owner's responsibility.

## Behavior and boundaries

Account username/mode projection reads inside the same metadata snapshot. Account rename/selection and settings writes commit or roll back together. A stale account selection cannot be renamed through an old settings form even if ordinary settings are unchanged. Invalid fields, unsupported enums, unsafe names and numeric bounds fail before publication; errors never echo untrusted JSON, SQL details or local paths.

Authenticated names preserve the provider's 1-to-16-character contract; offline name edits retain the 3-to-16-character requirement. Mode changes validate the final account projection, not the previous account's name paired with the requested mode. Removing the last short-named online account restores a valid offline display fallback without creating an account.

The shared task owner retains accepted writes across dropped HTTP waiters and joins them at shutdown. Config writes acquire telemetry's owned consent lease before persistence and hold it through committed identity publication. Failed opt-out writes suppress save-failure telemetry. Exporter availability is an explicit boolean in persistence; the store has no telemetry runtime dependency. Keyless profiles retain enabled consent without generating an identity, and a configured restart initializes an identity once. Revocation clears it and later re-enablement rotates it.

## Verification

Feature and route tests cover default wire shape, durable reopen, competing/stale revisions, null versus omission, invalid/corrupt data preservation, flag visibility/reset, inheritance, importer rejection, atomic account rollback, onboarding failure, dropped HTTP waiters, shutdown admission and committed consent. Rustfmt completed on owned Rust files.

Focused short-name regressions use persisted Microsoft account metadata to cover reads, unrelated preference writes, reopening settings, both mode switches, rejected short offline edits and removal of the last online account. Integrated checkpoints and remaining runtime gaps are recorded in [integration status](integration.md).

Shared Cargo execution belongs to the integration owner. Requested focused commands:

```text
cargo test -p axial-app settings:: -- --nocapture
cargo test -p axial-api routes::config::tests -- --nocapture
```

## Instance settings while Playing

Legacy allows Rename and next-launch settings edits while a game runs. The rewrite's exclusive directory admission incorrectly refused those enabled controls. The instance owner now accepts a narrow loan from the current Running session: its weak preparation reference yields an exact retained instance lease/pin, held through the existing metadata revision-CAS write. Terminal history never owns the preparation. Fresh physical binding and immutable launch target are verified; startup recency or earlier metadata edits may have advanced the row without changing that binding.

Starting, stopping, dead, unresolved or closing sessions cannot lend admission. Pending content/Performance/setup effects, different metadata/exclusion owners, changed physical binding, retargeting and stale revisions still refuse. Ordinary callers retain exclusive admission. Running command inputs stay captured; changes apply to the next launch. No new table, journal or general mutation coordinator.

The actual fixture-process HTTP journey first fails at Playing Rename with HTTP 409 Busy (`playing-metadata-red.log`), then passes Rename, memory edit, stale/retarget/delete refusal, unchanged running-process memory, clean Stop, reopen and a second launch with the new memory (`playing-metadata-green-final.log`). Thirty matching app metadata tests pass, including five new loan/binding/pending-effect guards (`playing-metadata-guards.log`). The initial green attempt reached Stop but failed its output assertion because the correct redactor removed a raw JVM flag; the fixture now emits only the parsed numeric memory value. Redaction is unchanged (`playing-metadata-green.log`).

Full two-thread verification passes 889 app and 128 API tests, with six ignores in each, plus 88 desktop tests (`playing-metadata-consumers.log`, `playing-metadata-desktop-final.log`). Independent review and scoped formatting/whitespace checks pass. The existing UI is unchanged.

The API rebuilt at `58e60a48` also passes the preserved browser interface with real Fabric/Minecraft in the isolated `axial-report-parity-V1RoWt/profile`. Rename to `Fabric Playing edit verification` and a 7-GB maximum heap save while Playing, persist through normal Stop/restart, and apply to the next real launch. Session `07cb7ed7-2153-4c95-9e9b-8b66d24a355f` retains its captured 8192-MB request; subsequent session `383850ea-f2dc-461c-8bb3-f15a6cc526df` records 7168 MB. Both report stopped with terminal acknowledgements; both API processes exit zero normally. SQLite is healthy with no unresolved accepted launches. Five previously recorded mod/screenshot/backup canaries remain unchanged (`playing-edit-ui-{acceptance.md,first-settlement.log,final-settlement.log,canaries.log}`). The unchanged legacy slider also clamps the minimum to 1 GB when editing this inherited range; this is not a maximum-only mutation. The running Java window remains unavailable through app controls, so native gameplay is still unverified.
