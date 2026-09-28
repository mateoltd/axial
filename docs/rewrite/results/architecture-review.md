# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Detailed historical checkpoints and failures remain in [integration evidence](integration.md) and Git history.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues on `main`, preserving rewrite and upstream history. Root owns shared verification and this record; feature authors and independent reviewers own their bounded source slices. No changed-scope naming or nesting issue warrants style-only churn.

Current scope: request-local readiness proof reuse, its queue-release boundary, exact settlement validation during startup, and one test-fixture lifetime correction. Launch owns both proofs; Setup owns list grouping; HTTP remains an adapter. Separate independent reviews of readiness and settlement are clear. Shared verification is serialized.

## Current findings

- **Repeated readiness input.** Lists repeatedly reconstructed the same installed-version proof. One request-local projection now retains at most one inventory, keyed by the admitted generation/library and the receipt's exact version. Publication-lane and file-revision checks precede reuse; account/settings/rules/runtime and final target fences remain per row. Ordinary preflight and mutation admission remain fresh. Grouping preserves response order; the shared reader can defer a conflicting install until group release. No global cache or mutable readiness mirror. Review removed redundant version-key fields.
- **Release versus wakeup.** Extending the proof past TaskOwner completion could strand an already queued install after its last wake. A deterministic production-queue regression reproduces this. Projection destruction now drops its proof before calling the existing queue's resume method, including early admission refusals. Accepted row work still owns its guards after waiter loss; abandoning a list does not execute its remaining rows. No new scheduler or notification service.
- **Recovery proof versus version marker.** Startup previously excluded any integer-version1 settlement envelope before validating its bound payload and canonical observation. Malformed evidence could bypass the restoration fence. A v4 index includes every unacknowledged accepted/interrupted row; exact validation precedes the waiver. Applied v3 is unchanged. Observation remains independent of optional reports, without invented acknowledgements or process adoption. Ordinary acknowledged lifetime history remains excluded; exceptional reportless history consumes the existing 4096-row/32-MiB budget and safely refuses at its bound.
- **Fixture root lifetime.** A music retry test could dispose its temporary root after result publication but before the worker released its retained pin. Its existing idle wait now covers the successful retry too. The full-run abort stack points to root cleanup during runtime cancellation, not stack overflow; the exact abort predicate remains unproven. No production safety or timeout was weakened.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- **837 app / 120 API / 88 desktop** pass, unchanged five/three app/API ignores; 35 focused coordinator tests plus one composed Setup journey, scoped formatting and API build pass. Both independent source reviews are clear. Logs: `readiness-recovery-{focused,consumers-final,desktop,format,build}.log`, `readiness-group-setup-final.log`.
- Meaningful red checks reproduce repeated full verification, missing queue wakeup and malformed-v1 restoration. Green covers exact generation/lane/leaf/ancestor changes, row-specific errors, dropped waiters, response order, v3 upgrade/reopen, immutable mixed evidence and exact budget boundaries. Logs: `readiness-group-red.log`, `readiness-release-red.log`, `settlement-scan-red.log`.
- Initial NoLane fixture failed before its behavior because the publication directory was absent. Initial desktop verification selected a nonexistent library target. The first combined run then aborted in the music fixture; the unchanged focused case passes and the corrected full rerun passes. These failures remain recorded, not relabeled as green. Low-space cleanup removed only verified regenerable compiler/test artifacts; profiles and evidence remain.
- Previously integrated queued Resume, nested-dialog ownership and preserve-only Quit retain their recorded evidence and safety fences. Hosted run 36378355416 passes `a388eb68`. Frontend source and the 479-test checkpoint are unchanged; see the ledger, [history import](history-import.md), and [screenshots](screenshot-files.md).

## Retained architectural lessons

Previous corrections remain documented in the integration ledger and feature records: exact durable observation before optional report persistence; content-owned receipts for local mod effects; bounded same-identity filesystem proof refresh; heap-allocated decoder scratch across async yields; and projection invalidation only after accepted ownership releases. Their safety fences remain required, not complexity to remove.

Three matched same-profile debug list reads improve from 45,201 / 45,290 / 45,055 ms to 35,274 / 34,674 / 34,801 ms: median **23.0% lower**, with exact ordered readiness outcomes preserved in all six responses. No concurrent builds, UI work or game launches; both APIs exit normally. Ordinary preflight/detail still take roughly 15 seconds. Full file-revision walks and per-row Java checks remain; this local comparison is not a general performance guarantee. Logs: `readiness-group-{before,after,comparison}.log`.

AGENTS.md now refines existing ownership/proof rules with release-before-wakeup ordering and complete recovery validation. Other rules were retained, not duplicated. Historical details remain in the ledger and Git.

## Unresolved handoffs

Root retains full-parity work. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence, or infer settlement from PID absence/time. Its profile remains untouched. Same-boot process authority and genuine different-boot/native-cleanup proof are separate unresolved recovery requirements.

Other open evidence includes the manual interrupted-Reset Preserve-files choice (system alert inaccessible to the UI tool), OAuth/game-window journeys, remaining content/pack failure/restart cases, developer-interface benchmark continuation, automatic cross-profile handoff/cutover, four installed architectures and trusted signed-update inputs. Three sampled Quilt provider hash disagreements remain strict refusals, not proof every release fails. The historical observed-settlement review gap is now closed by independent review and the correction above; runtime/platform gaps remain.

The full-parity goal remains active. The architecture automation targets `main` and retains its pre-existing paused status. No deployment or release publication.
