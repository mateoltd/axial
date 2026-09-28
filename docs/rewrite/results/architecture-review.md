# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: world-backup scheduling and stale parity evidence. Prior import corrections remain recorded in the integration ledger. No UI, wire, schema, table, coordinator or namespace changes.

## Findings and corrections

- **Keep blocking work off async workers without detaching it.** World backup performed its entire bounded tree copy inside an async task with no yield; legacy dispatched the filesystem work to a blocking worker. The existing accepted task now awaits `spawn_blocking`. Its cancellation-before-effects, retained instance, exact native authority, copy limits and unresolved-effect owner are unchanged. Panics propagate to TaskOwner so they cannot masquerade as settled file errors. No new executor or recovery layer.
- **Review actual ownership, not just completion.** A dropped request waiter leaves accepted copying owned. Shutdown waits for the blocking body and resource release; it does not abort/detach a copy on cancellation. The regression observes independent current-thread runtime progress, retained exclusion/generation pin, pending shutdown, exactly one backup and unchanged source/canary bytes. A separate injected worker panic keeps the existing unsettled fence.
- **Do not invent parity requirements from missing machinery.** Legacy startup creates empty session/process maps; it does not reconstruct surviving process capabilities. Current Unix subprocess journeys already prove preservation, exact restored fences, refused replay/mutation and preserve-only Quit. They do not establish adoption, native-window or Windows acceptance. The integration ledger now distinguishes these facts and corrects superseded normal-shutdown wording.
- **Keep the active ledger current.** Ordinary Mods and world-backup/rename/restart acceptance existed in feature reports but remained listed as entirely unverified in the master blockers. Those bullets now target outstanding failure/interruption, native and gameplay cases. The old task-lifetime implementation handoff is replaced with integrated evidence; no gate is marked complete from documentation alone.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Meaningful RED: the actual backup hook runs on the async runtime thread (`world-backup-scheduling-red.log`). The fixture settles before asserting, avoiding an unresolved-root teardown abort.
- All **three focused backup tests pass** after correction, including existing backup/rename/delete and the new scheduling/panic cases (`world-backup-scheduling-green.log`). The injected panic is expected and verified as unsettled, not a suite failure.
- Full two-thread verification passes **878 app / 127 API tests**, with six ignores in each (`world-backup-consumers.log`). Independent final source review is clear; final scoped formatting and whitespace checks pass after correcting formatting-only wraps. Existing AGENTS ownership/lifetime rules cover this correction; no redundant rule was added.
- Hosted [run36395898759](https://github.com/mateoltd/axial/actions/runs/36395898759) passes application and delivery on prior import checkpoint `913dbd9f`, not this later backup edit. Earlier failures remain recorded in the integration ledger and are not erased by a green checkpoint.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
