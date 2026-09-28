# Library UI

Status: local view adaptation and isolated component checks complete; real producer integration and interface acceptance remain outstanding.

The preserved Home and Instances views still use the existing featured banner, cover grid, list, instance visuals, controls, menus, selection tray and navigation. No CSS or layout changed. Performance remains outside these two views and untouched.

## Adaptation

- The Home featured action now presents the shared instance snapshot's `launch_action` label, icon and blocked reason. A pending install or blocked instance no longer displays an unconditional Play action. As before, activating the action opens instance detail; the view does not launch or install directly.
- Instance list rows now support Enter and Space with the same nested-control guard used by the existing banner and cover cards. Selection and overflow controls retain independent actions. The overflow control names its target in its existing tooltip.
- No view-local instance, install, launch or loading authority was introduced. Both views continue to consume `store.instances`, `versionById`, `launchSessions`, `instanceInstallStatus` and shared session presenters. Deletion, confirmation, keep-files intent, duplication and rename remain with the existing shared instance workflows.
- Initial loading and bootstrap failure remain owned by the shared BootSplash. The interface owner confirmed that failed instance refresh retains the prior snapshot and reports the error; these views do not replace a failed request with an empty library.

## Checks

`node --test frontend/test/rewrite/library-ui.test.mjs`

Nine checks pass. They execute the actual TSX modules in memory and inspect component output and event callbacks against explicit fixtures. They cover recent ordering and the fourteen-card limit, create/See all navigation, backend install/blocked labels, backend session and queue labels, keyboard activation with nested-control exclusion, trimmed case-insensitive filtering, grid selection/context forwarding, and bulk-delete handoff without premature clearing or local removal.

The test harness substitutes shared store/workflow hooks and visual primitives. It uses the real session presenters, but does not establish DOM focus behavior, actual deletion confirmation, SSE ordering, persistence, process ownership, or backend readiness. It resolves root TypeScript/Preact dependencies when installed, with the preserved frontend dependencies as an isolated development fallback. It writes no compiled output.

Focused Prettier validation was run for the two view files and the test. No shared build, Cargo command, legacy edit, generated contract edit, or installed-profile mutation was performed.

## Integration requirements

- Shared instance transport must provide the real enriched instance projection: opaque identity, names/version identity, timestamps/art seed, backend version labels and mod support, resource counts, and backend `launch_action`. Existing bootstrap reads `GET /instances` as `{ instances, last_instance_id }`; any replacement route adaptation belongs to the shared interface owner.
- Install and session owners must supply the shared authoritative queue/session projections and presenters. These views must not infer successful installation, running state, readiness or retry policy from missing snapshots.
- Real API/browser acceptance must exercise populated and empty libraries, bootstrap failure/retry, grid/list filtering, keyboard focus/navigation, context actions, duplicate/rename, explicit keep-files/delete-files confirmation, busy refusal, deletion failure, and refreshed status indicators.
- Installed native acceptance and comparison with the retained application have not been performed by this package. The isolated checks are not a parity claim.
