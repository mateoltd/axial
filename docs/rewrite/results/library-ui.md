# Library UI

Status: local view checks and bounded native create/duplicate/both-removal/reopen journeys pass. Full interface and installed-release acceptance remain outstanding.

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

On2026-10-04, the retained unsigned macOS ARM64 bundle `1b161d84` uses generated profile `/private/tmp/axial-native-current.YwnWYB/profile`. Offline NativeParity onboarding and real Vanilla1.20.1 creation/install/Launch/Logs/Stop/Quit/reopen pass. Its duplicated-instance menu creates two distinct IDs, preserves2048MB/automatic optimization off and exact copied `options.txt`; original/copy1/copy2 SHA256 is `9de23dedfc4dbcc6bbbabb5f0d0f6282e2dd1610c3e4623ff574d6e813199231`. Copies have empty saves/mods/config, so populated-content copying is not inferred.

The real confirmation exposes both choices. Remove, keep files removes copy1 `f36b7c53-fb30-44c0-a7d1-ecadbe0fcc79` from the registry while preserving its options. Delete instance and files removes copy2 `9ff652bb-8cb1-4d65-b8c3-47f7ae823d77` and its tree. Both durable intents become complete; UI notices/navigation/counts converge. Original `ddfa6e66-5a0b-4795-9b4a-ecd15088d21d`, retained copy bytes and sibling identity remain unchanged. Normal Quit exits0; reopen shows NativeParity and exactly the original instance, without resurrecting either copy. This intentionally deletes only a disposable generated copy, not user data. Exact diagnostics/IDs are in `.rewrite-logs/native-current-acceptance.md`.

Duplicate/deletion/shared-instance UI production paths are unchanged from this bundle through runtime checkpoint `6a28ab07`; the whole bundle is not claimed current or installed-release acceptance. Populated-content copy, rename, keyboard/focus/filtering, busy/failure deletion, browser reconnect and the four-platform installed matrix remain separate requirements.

- Shared instance transport must provide the real enriched instance projection: opaque identity, names/version identity, timestamps/art seed, backend version labels and mod support, resource counts, and backend `launch_action`. Existing bootstrap reads `GET /instances` as `{ instances, last_instance_id }`; any replacement route adaptation belongs to the shared interface owner.
- Install and session owners must supply the shared authoritative queue/session projections and presenters. These views must not infer successful installation, running state, readiness or retry policy from missing snapshots.
- Real API/browser acceptance must exercise populated and empty libraries, bootstrap failure/retry, grid/list filtering, keyboard focus/navigation, context actions, duplicate/rename, explicit keep-files/delete-files confirmation, busy refusal, deletion failure, and refreshed status indicators.
- Installed native acceptance and comparison with the retained application have not been performed by this package. The isolated checks are not a parity claim.
