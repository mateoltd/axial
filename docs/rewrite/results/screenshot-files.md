# Screenshot files

Status: implementation in progress; not integrated or parity verified.

Owned source: `core/app/src/resources/screenshots.rs`.

## Retained behavior

Baseline references are `legacy/apps/api/src/application/instances/resources.rs`,
its screenshot tests in `legacy/apps/api/src/application/instances/tests.rs`,
`legacy/apps/api/src/routes/instances.rs`, and the existing screenshot actions,
pane and lightbox in `legacy/frontend/src/views/instance/`.

- Inventory fields are `name`, byte `size`, and RFC 3339 `modified_at` (empty when
  unavailable), ordered newest first, then by name. The scanner is bounded at
  50,000 entries and 1 TiB of accounted file sizes.
- PNG, JPG, JPEG and WEBP are admitted with exact portable filename spelling.
  File bytes are preserved without decoding or recompression. Media is bounded
  at 32 MiB and must carry its image content type plus `nosniff`.
- Rename preserves the image type; JPG to JPEG is allowed. Portable aliases and
  occupied destinations fail without replacement. Successful rename returns
  `{ "status": "ok", "name": "..." }`; deletion returns `{ "status": "ok" }`.
- Filesystem paths and native diagnostics do not appear in public errors.

## Integration requirements

The composition owner owns root module exports, HTTP registrations, shared
resource aggregation and generated wire integration. The retained route family
is `/instances/{id}/screenshots`, `/instances/{id}/screenshots/{name}`, and
`/instances/{id}/screenshots/{name}/file`.

Filesystem calls require an exact registered-instance capability with its
library generation retained. Media authentication belongs to the transport.
Accepted work must outlive a dropped HTTP waiter and preserve unresolved file
effects. Names alone must never grant authority to arbitrary filesystem paths.

Requested dependency: `chrono` for the existing RFC 3339 timestamp format.
Focused command for the sole shared build owner:
`cargo test -p axial-app resources::screenshots`.

## Evidence

No Cargo or build commands have been run by this package owner. The initial
wire assertions are source only until the integration owner runs them.
Runtime and UI parity must remain open until the real scoped filesystem,
registered instance, HTTP media and resource UI consumers pass together.
