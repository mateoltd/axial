# Retained UI wire parity review

## Verified read-contract corrections

2026-10-06, changes after `db0baf50`. Three retained contracts have meaningful HTTP RED/GREEN checks:

- Discover returns provider results when optional installed-content annotation is unreadable or its valid instance ID is absent. Malformed IDs still refuse. The corrupted-manifest fixture retains direct Content/plan refusal, one accepted install settling Failed, unchanged managed/user bytes and no unresolved effects. Only the existing annotation chain changes.
- Individual resource lists scan their own subtree. A screenshots-directory symlink no longer blocks healthy mods/logs; screenshots and the aggregate resources endpoint still refuse, preserving the link and external canary. One private helper shares existing admission and final binding validation across five current reads; typed response arrays keep their wire shape.
- Debug command inspection restores the placeholder-only `command` array and `command_redacted`, counting the executable as legacy did. The actual fixture child independently reports only its element count; complete HTTP-response equality is checked after ordinary shutdown. Final review catches a duplicate projection bypassing the existing4,096-placeholder cap; `b2c34db9` reuses that owner, retaining exact count and the separate release refusal. The admitted16,384-argument limit remains intact; command values are never exposed.

Logs: `discovery-annotation-{red,green}.log`, `resource-list-isolation-{red,green}.log`, and `command-shape-{test-red,green}.log`. Initial wrapper misuse refuses before running the command test and is retained separately. The final owner-reuse follow-up passes all three existing projection tests, the actual composed journey and release API-library compilation (`command-owner-{focused,journey,release-check}.log`, normal0); the journey takes5.84s. Independent Standards/Spec reviews and scoped formatting are clear. These corrections do not change UI layout or establish native, authenticated or full parity. Broader verification is recorded in [integration](integration.md).

## Initial review

Date: 2026-09-26. Scope: current retained `frontend/src` consumers against the
replacement `apps/api` route composition and desktop command registration.
This was a source review, not a build, test run, browser exercise or installed
WebView proof. Other owners were editing concurrently; status below reflects
the final source read for this review. Intentional Guardian removal is excluded.

At the initial review, the principal blockers were
missing content and instance-resource adapters, absent native skin-file actions,
a launch-history response mismatch, and unfinished release/update behavior.
Two media query mismatches found here were corrected in source during the review.

## Current route inventory release gaps

Later integration supersedes the initial source observations below: content, resources,
system, music and benchmark routes are now composed; launch reports use the required
projection and native skin commands are authored. The latest source registrations
add both remaining baseline pairs, ordinary-instance import and metadata-import
receipt commands. Six source-inventory checks verify 142 current registrations
and all 122 baseline method/template pairs (`metadata-ui-route-inventory.log`). Both
metadata routes now bind to the actual retained import UI. There are no
known absent baseline method/template pairs in the current manifest.

Full pack installation and overrides now have a domain implementation and adapter;
their current integration is not yet verified. Route presence is not runtime parity.
Native update wiring and signed release inputs, import
preview without complete cutover, unverified native commands and the failure/restart gaps in
the integration ledger remain release blockers. The initial findings below record
what was observed, not a claim that those earlier omissions still all exist.

## High-impact findings

### P1 found and corrected in source: Profile skin identity query

`frontend/src/player-skin.ts:44` constructs `/skin/profile/file` URLs with
`profile`, optional `skin`, and optional `texture`. At the initial inspection,
`apps/api/src/routes/skin.rs:74` `ProfileFileQuery` accepted only `texture` and
used `deny_unknown_fields`, rejecting the current-profile texture path even
with a valid media ticket.

After notification, the skin owner added `profile` and `skin` to that query and
delegated to `ProfileMedia::profile_file_for_identity`, which checks the selected
profile, the requested skin/texture and the captured account after the fetch.
This corrects the inspected contract in source; real current/stale profile
requests through the assembled media-ticket boundary still need verification.

### P1: Nonempty launch history fails the retained response decoder

`apps/api/src/routes/launch.rs:261` serializes `LaunchProofRecord` directly for
`GET /launch/reports`; the detail handler does the same. The struct at
`core/app/src/launch/reports.rs:74` has no `view_model`. The retained
`frontend/src/dto-launch.ts` `isLaunchProofRecord` requires
`view_model.outcome_label`, `view_model.outcome_tone` and
`view_model.comparison`. `PerformanceLabProofHistory.tsx` consumes those fields
to display outcomes and comparisons.

An empty list can look healthy; after the first report, the entire history
response fails decoding. An owning-feature projection or coordinated retained
frontend adapter must supply the real presentation semantics. Sent to launch
and Performance owners.

### P1: Instance file screens have no composed API routes

The instance resource implementations under `core/app` do not by themselves
connect the retained screens. Current `apps/api/src/routes/instances.rs` serves
registry/create/edit/delete/duplicate only, and `apps/api/src/lib.rs` does not
merge a resource route family.

| Retained consumer | Missing route or contract |
| --- | --- |
| `views/instance/resources.ts:25` | `GET /instances/{id}/resources` |
| `views/instance/mod-actions.ts:132` | `PUT`/`DELETE /instances/{id}/mods/{name}`; mutation acknowledgment must have `status: "ok"` |
| `views/instance/world-actions.ts:24` | `PUT`/`DELETE /instances/{id}/worlds/{name}` and `POST .../backup`; backup acknowledgment needs `backup` and `location` |
| `views/instance/screenshot-actions.ts:29` | `PUT`/`DELETE /instances/{id}/screenshots/{name}` and `GET .../file`; rename acknowledgment needs the new `name` |
| `views/instance/logs.ts:98` | `GET /instances/{id}/logs/{name}` returning `name`, `size`, `truncated`, `text` |
| `views/instance/instance-actions.ts:13` | `POST /instances/{id}/open-folder?sub=...` |

These calls currently have no matching authenticated route. Resource listing
is the first failure for several tabs; adding only mutation handlers would not
restore the workflows. File adapters must retain registered-instance authority
and mutation acknowledgments. Sent to API and filesystem owners.

### P1: Discover, content installation and pack creation are not routed

All current `frontend/src/content.ts` calls below are absent from the composed
API at review time:

- `GET /content/search`, `/content/item`, `/content/modpack/target`,
  `/content/modpack/files`.
- `POST /content/plan`, `/content/install`, `/content/compatibility`,
  `/content/modpack/install`.
- `GET /instances/{id}/content`, `/instances/{id}/content/updates` and
  `POST /instances/{id}/content/uninstall`.
- `POST /instances/setup/plan`.

`frontend/src/instance-create.ts:52` also selects `POST /instances/setup` for a
resolved setup plan and `POST /instances/modpack` for pack creation. Neither is
registered by the current instance router. The ordinary `/instances` create
path being present does not cover these two branches.

Installation/uninstallation callers expect an `InstallQueueStateResponse`,
not a detached success string. Plans use retained target/selection DTOs and pack
requests preserve selected optional files and `include_overrides`. Sent to
content and API owners; this finding identifies missing adapters/composition,
not an assertion that the domain implementations are absent.

### P1: Desktop skin picking and native drop admission are missing

`frontend/src/native.ts` invokes `pick_skin_file` and `consume_skin_drop`.
The retained upload picker (`use-saved-skin-upload-workflow.ts:167`) and texture
replacement picker (`use-saved-skin-edit-workflow.ts:213`) call this native
path whenever Tauri is present. A rejected native command reaches the error
handler; it does not fall back to the browser file input.

Neither command is registered in `apps/desktop/src/main.rs` or permitted by
`apps/desktop/capabilities/main.json`. The retained native drop listener expects
`axial:desktop:skin-drag` with an admitted token, but there is no replacement
emitter/admission path in the current desktop source. Desktop owner confirmed
both gaps. Existing authenticated skin upload endpoints cannot compensate for
the missing file-selection entrypoint.

## Other retained release blockers

| Surface | Evidence and consequence |
| --- | --- |
| Native development reset | `AdvancedSettingsSection.tsx:74` calls `requestNativeAppReset`, which invokes `app_reset`; that command and its capability are absent. This is gated by development mode and native runtime, but is retained scope. Desktop owner confirmed it remains unfinished. |
| Music and hardware summary | `bootstrap.ts:67` and `:70` call `/system` and `/music/status`, currently absent. They catch failures and use null/default state, so bootstrap success does not prove parity. `music.ts:201` and `:253` use `/music/track?t=...`, also absent; enabled music cannot obtain audio. System absence changes memory recommendations. |
| Updates | `updater.ts` still checks `/update`, downloads through `/update/download` and applies through `/update/apply`. All three explicitly return 501 `update_unsupported` in `routes/update.rs`. This is honest unavailable behavior, but retained check/download/verify/apply/restart behavior is not release ready. |
| Cutover and import | Preview remains read-only and reports `cutover_available: false`; no retained UI/native source-selection consumer or semantic commit is wired. `acceptance/cutover/README.md` records the remaining publication, preference, reauthentication and settlement gates. |

## Other findings corrected during this review

The initial transport middleware authenticated a media ticket but passed
`axial_ticket` through to strict domain query decoders. Because
`frontend/src/api.ts` appends that parameter to media URLs, profile/cape/lookup
media requests with a valid grant were rejected as unknown query fields. The
API owner was notified and the final source read now shows
`authenticate_request` removing the transport grant with `without_ticket`
before domain extraction. This fixes the inspected mismatch in source; it has
not been exercised by this review. The related `profile`/`skin` mismatch above
was also patched during the review.

The benchmark router existed but was not registered at the initial inspection,
leaving retained `PerformanceLabCard.tsx` and `PerformanceLabSuiteDrivers.tsx`
matrix, qualification and driver calls without handlers. After notification,
`routes/mod.rs` now exports `benchmarks` and `lib.rs` merges its router with a
constructed benchmark service. Matrix and driver response fields match the
inspected frontend decoder shape. Runtime driver lifecycle, persistence and
qualification evidence remain to be exercised by the integration owner.

## Checks that did not produce a finding

The current composition registers config/status/onboarding, ordinary instance
create/edit/delete/duplicate, version/loader catalogs, Java discovery,
accounts/auth, flags, telemetry, install queue, launch/session and skin routes.
Spot checks found matching config revision/selection fields, ordinary create
settings, Java runtime list shape, queue event envelope, and launch named
`status`/`log` events. Registration and a spot check are not runtime parity.

The old `start_install_events`, `start_loader_install_events` and
`start_launch_events` native helper definitions remain in `native.ts`, but no
live consumers were found. Downloads and launch use authenticated HTTP/SSE.
Their missing native registrations are therefore not listed as regressions.
Installed transport evidence remains a separate release gate.

## Required follow-up evidence

Use the real assembled API and retained UI for one successful and one failed
operation in each missing family. Include a nonempty launch history, a current
profile skin URL with both identity hints and a media ticket, native upload and
drop selection, resources after actual game output, a content dependency plan
and selected pack files, and a persisted benchmark driver. Check exact request
and response DTOs after adapters land. Then repeat installed native workflows
across the required artifact matrix; neither source inspection nor empty
fixtures can establish these outcomes.
