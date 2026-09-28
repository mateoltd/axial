# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Root owns shared verification and this record; bounded feature owners and independent reviewers are frozen for verification. No style-only naming or nesting churn.

Current scope: bounded Performance recovery inventory and installation observer shutdown. Prepared-command continuation is already committed as `2c425148`. No UI, wire, table or generic coordinator is added.

## Findings and corrections

- **Bound total inventory.** Pending records now have inclusive 128-row/8-MiB limits, retaining the existing 2-MiB row limit. Reads stop before decoding excess rows; begin/save check actual stored sizes before and after transactional publication. Oversized existing inventories fail preserved, never truncate. Exact settlement and inspection completion use the existing target record and can reduce an oversized inventory.
- **Bound active restoration too.** New/nonactive-to-active commands require capacity in the same transactional owner; existing active progress and terminal writes remain allowed. Startup validates the whole bounded active set before mutation. V2 adds one partial active-command index; applied V1 stays byte-identical. The existing command/history owner remains authoritative.
- **Preserve unknown effects.** A refused late result write retains prior pending proof, exclusion and truthful unsettled status. Freeing capacity does not recreate a missing result or guarantee eventual rollback settlement. No headroom-reservation protocol or fabricated completion is introduced.
- **Join notification lifetimes.** Four detached install-queue observers could retain the real profile after TaskOwner shutdown. Queue-local join handles now cover ordinary completion, scheduling and both recovery paths. Registration is serialized with sealing, finished handles are reaped, and cancelled/concurrent drains preserve ownership. API shutdown drains after accepted work joins and before file preservation. Observer errors are diagnostics, not invented file obligations. Setup releases its queue clone before waking its response waiter.
- **Use real boundaries.** The current-thread scheduler test proves ordinary reopen succeeds without an observer and returns `NoEffect(Busy)` with the unpolled observer before correction. After correction it also checks cancelled/concurrent drain and late registration refusal. Budget tests use actual registered checkpoints and native Remove effects. Fixture-name collisions and unresolved-root teardown were corrected without weakening production admission or native guards.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Seven budget checks and all 29 queue checks pass (`performance-budget-green.log`, `queue-observer-{green,suite}.log`). Full verification passes **870 app tests with two threads / 127 API tests at default concurrency / 88 desktop tests**, with six app/API ignores each (`recovery-bounds-app-bounded.log`, `recovery-bounds-consumers-final.log`, `recovery-bounds-desktop.log`). Scoped formatting and whitespace checks pass; independent frozen-source reviews cover both fixes and API/setup integration.
- The first consumer run times out at an existing benchmark stop-request deadline; its unchanged exact case passes in 5.11 seconds and the full API rerun passes. The latter run's app stage aborts after a five-second music fixture timeout; its unchanged exact case passes in 0.23 seconds, then the full bounded app run passes. Neither timeout’s cause is established. Deadlines and assertions are unchanged; failures remain recorded.
- Meaningful observer red is recorded in `queue-observer-red.log`. The earlier budget run had three passes/four fixture-admission failures; its next run aborted during deliberate unresolved fixture teardown. Native crash evidence identifies `RootSession::drop`, not stack overflow. Normal proof-backed fixture cleanup now follows the unchanged refusal assertions.
- Commit `2c425148` passed 862 app / 127 API / 88 desktop / 132 Performance leaf tests locally. Hosted run36389949364 passes delivery, policies, build and wire generation, but API has 126 passes/one first-reopen failure; app/frontend tests were not reached. The observer-class gap is reproduced, but attribution of that hosted event remains unproven pending verification.
- Frontend source and its earlier 479-test checkpoint are unchanged. Previous Minecraft qualification remains 953 passes/nine default-concurrency timeouts, all nine passing unchanged bounded reruns, not one full green run. Earlier evidence is retained in the integration ledger.

## Retained lessons

The existing AGENTS.md lifetime rule now explicitly requires joining root-retaining observers before acknowledging shutdown, covering the repeated queue/setup pattern without adding another rule. Existing guidance covers aggregate limits, exact durable readback and real fixture preconditions. Simplicity still preserves filesystem authority, recovery fences and the existing UI.

## Unresolved handoffs

Root retains full-parity work. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Its profile remains untouched. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate requirements.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement/full cutover, four installed architectures and trusted signed-update inputs. Three sampled Quilt hash disagreements remain strict refusals, not proof every release fails. Native queued-history acceptance still waits for its separate picker handoff; see [history import](history-import.md).

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment or release publication.
