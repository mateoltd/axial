# Import and cutover acceptance

Full profile cutover remains unavailable: every preview and import result reports
`cutover_available: false`. A process-correlated native picker, metadata/ordinary
instance import and clean Quit/restart journey passed on the isolated macOS fixture.
Preference-file application/reload and complete cutover remain unverified.
See [current integration status](../../docs/rewrite/results/integration.md)
for the verified source checkpoint, logs and interface limits; passing checks do
not establish cutover readiness.

## Integrated subset

- Read-only semantic preview accepts native-admitted sources. Instance import
  accepts the admitted fingerprint and legacy instance ID, never a caller path.
  Preview does not open the old profile as an application root, access its keyring,
  copy files, admit stored absolute paths, or make an external source a destination.
- Ordinary-instance publication reuses receipt-bound staging, promotion and an
  atomic source-to-destination mapping. Accepted work survives preview closure;
  ready/published restart recovery requires exact source readmission. Completed
  retry verifies immutable history without recopying the destination payload.
- Offline and Microsoft identities, selection, settings and flags commit atomically
  with a completed receipt. Credentials are not copied; Microsoft identities require
  sign-in. Replay preserves later edits and current telemetry consent.
- Saved skins import through the existing library owner without applying them to an
  account. Bounded terminal reports, suites and drivers publish with the instance;
  historical records cannot launch or resume work. [History import](../../docs/rewrite/results/history-import.md)
  records the supported schemas, bounds and remaining history obligations.
- The predecessor preference-export control and replacement dialog have tested
  runtime consumers. Browser-local preferences require explicit source association
  and current receipt-backed references; their application remains separate from
  SQLite receipt completion. Unknown mutation outcomes use status reads, not blind
  write retries.

## Verification boundaries

Shared application, API, frontend and desktop checks pass at the linked integration
checkpoint. Evidence includes source non-mutation, bounds and unsafe-file refusal,
source drift, stale previews, cancellation, atomic rollback, immutable retries,
selected-instance binding and ready/published recovery. Authenticated HTTP history
tests also reopen the server and verify no live sessions, launch intents or runnable
driver requests. These checks are not real native migration or full-profile rehearsal.

Fixtures use separate replacement roots, including the independent profile in
`acceptance/fixtures/profiles/offline-vanilla`. Source canaries include bytes, inode
identity and Unix link counts. Raw obligations stay in the private inventory; HTTP
exposes content-bound identifiers and affected instance IDs, not journal paths or
credentials. Physical overlap checks use admitted ancestry; arbitrary Unix
bind-mount subtree aliasing remains unverified.

The integration owner runs focused shared checks with output captured to logs and
inspects their tails:

```sh
cargo test -p axial-app import::
cargo test -p axial-api routes::import::
```

## Remaining cutover obligations

- Extend the passing native source/ordinary-instance import journey to external
  payload selection, failure handling and preference selection/application/reload.
  The bounded macOS run does not establish complete migration or the platform matrix.
- Complete owning-feature settlement or conversion for unsupported loaders and
  instance metadata, Performance state, content provenance, pending deletion/file
  effects, nonterminal history and automatic-resume handoffs, and every other
  retained unsupported obligation. Supported skin/history subsets are not blanket
  waivers for unknown, malformed, conflicting or oversized records.
- Verify Microsoft reauthentication and an actual imported-instance
  install/launch/stop/restart journey through the retained interface.
- Rehearse full-profile repeated import, insufficient disk, source changes during
  copy, cancellation, and process crash/restart at staging, promotion and metadata
  boundaries. Preserve the untouched predecessor and demonstrate rollback plus
  rescue export for subsequent destination changes.
