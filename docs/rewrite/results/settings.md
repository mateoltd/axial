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

Focused short-name regressions use persisted Microsoft account metadata to cover reads, unrelated preference writes, reopening settings, both mode switches, rejected short offline edits and removal of the last online account. These new cases await integration-owner Cargo execution.

Shared Cargo execution belongs to the integration owner. Requested focused commands:

```text
cargo test -p axial-app settings:: -- --nocapture
cargo test -p axial-api routes::config::tests -- --nocapture
```

Status: implementation handed off; shared test results and real retained-UI acceptance are still required before integrated parity is claimed.
