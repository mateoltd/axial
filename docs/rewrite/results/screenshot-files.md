# Screenshot files

Status: implementation integrated; bounded real-interface evidence below, not full parity verified.

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

The latest shared checkpoint `42b2d598` passes 829 app / 117 API tests and hosted application/delivery checks. This includes resource owners and adapters, not every runtime screenshot journey.

The generated resource acceptance profile uses a disposable copy of the repository's 512-by-512 icon as `axial-disposable-2026-09-28.png` in instance `d6926227-f5e6-47a3-a736-f6c1cef14435`. The real Screenshots view lists 10,113 bytes and decodes the image in its lightbox. Rename updates both disk and the open lightbox to `axial-renamed-disposable-2026-09-28.png`, preserving SHA256 `88c17d7e98353f93809c2e0c68649823945b521f3cec25f46f9950833221ed7f`. A PNG-to-JPG draft shows the type-preservation error and disables Rename; cancellation leaves no JPG or other file effect. Evidence: `.rewrite-logs/screenshot-lightbox.png`, `resource-ui-files.log`, `resource-fixture-before.sha256`. This is an image fixture, not a game-captured screenshot.

Normal restart retains the renamed screenshot and exact bytes (`resource-restart-files.log`). Nested prompt inspection reproduced an inherited shared-modal issue: both prompt and lightbox claimed modal accessibility, and Escape closed both. The final correction uses the existing dialog owner to suspend the lightbox and cancel/restore focus without layout changes. On generation `660aaf2f2c4c`, real browser accessibility exposes only the foreground prompt, the input receives focus, Tab/Shift+Tab wrap inside it, and Escape preserves the lightbox and returns focus to Rename. Cancel preserves the type-validation behavior; a separate Escape closes the lightbox and returns to its View button. Evidence: `modal-dialog-browser.md`, `modal-dialog-focus.png`, `modal-dialog-escape.png`. Both acceptance API runs exit0 normally. Deletion, native folder opening and interrupted-publication acceptance remain open.
