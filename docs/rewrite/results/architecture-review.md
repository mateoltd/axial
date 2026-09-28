# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues on `main`, preserving history. Root owns shared verification and this record; feature authors and independent reviewers own bounded source slices. No naming or nesting issue warrants style-only churn.

Current scope: same-profile continuation of prepared Performance commands, their existing command/status owner, and composed restart fixtures. Independent production and fixture reviews are clear. Shared verification remains serialized; no UI or wire redesign.

## Findings and corrections

- **One owner.** Legacy resumes Prepared Performance work, including synchronous entrypoints. Queued and synchronous Apply/Reapply, Remove and Rollback now share the existing command/task path and command ID. No second journal, schema or generic coordinator. Imported history and old unlinked records gain no replay authority.
- **Exact durable proof.** New preparation and subsequent writes require exact command linkage and transactional readback. Fresh graphs must match the original proof; rollback retains the selected snapshot and target/count evidence. Terminal status and pending removal commit atomically before reservation release. Planning-only and effect-started work are never replayed.
- **Lifetime and startup.** One serial owned continuation avoids a per-target task-capacity burst without awaiting provider I/O during startup. Existing retained claims prevent duplicate work and preserve unknown effects; completed targets release independently. Failed status publication unclaims finished workers without discarding pending proof. Known pre-effect cancellation settles; unknown leaf inspection preserves exclusion. Receipt validation runs outside the shared map lock.
- **Behavioral fixtures.** Subprocesses reach the real committed boundary; no injected preparation replaces the workflow. Review corrected missing source prerequisites, invalid provider IDs, fresh admission against reserved targets and a test omitting completion publication. The temporary duplicate planner diagnostic was removed after composed journeys passed.
- **Test network seams.** Literal-loopback syntax is enabled only for test/test-support planning. Normal pack policy and public transfers remain HTTPS-only, public-address-checked and pinned. Fixture transfers require an explicitly injected exact-origin resolver; release and normal-composition policy is unchanged.
- **Portable leaf coverage.** Canonicalized the temporary fixture parent without weakening physical-root admission. Removed an obsolete non-Linux unsupported-stage expectation and enabled the existing real graph-publication and rejected-checkpoint cleanup tests on every platform. Named native stages now implement that behavior; only macOS execution is verified here.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- **862 app / 127 API / 88 desktop / 132 Performance leaf** pass, with six app/API ignores each; parent tests exercise the new crash helpers (`prepared-performance-{consumers,desktop,leaf-final}.log`). Nine owner tests cover all actions, no-replay boundaries, failed/ignored writes, command-link corruption, cancellation and exact rollback proof. Scoped formatting and whitespace checks pass. Initial leaf fixture/platform-assumption failures remain in `prepared-performance-leaf.log` and `prepared-performance-leaf-canonical.log`.
- Four composed HTTP journeys prove queued Apply, synchronous Apply/Reapply and valid changed-graph refusal: responsive startup while provider I/O is held, Busy target admission, same-command exact files or pre-effect refusal, source/user-file preservation and no second-reopen transfer. These synthetic artifacts are not gameplay evidence.
- Meaningful pre-fix crashes show missing synchronous and mismatched queued command identity (`prepared-performance-red-behavior.log`). Earlier fixture/compile failures and the initial focused twelve-pass/two-failure core run remain in the integration ledger; the full corrected run above passes.
- Prior automatic-workflow/Java/revision evidence remains at `2f716df7`, with green hosted run36385128372. Its Minecraft result was 953 passes/nine timeouts, with all nine passing unchanged bounded reruns, not one full green run. Frontend source and its earlier 479-test checkpoint are unchanged.

## Retained lessons

Existing AGENTS.md rules cover this slice: one completion owner, complete proof/readback, release-before-wakeup, realistic fixture preconditions and safe unknown outcomes. No anecdotal rule is added. Earlier corrections to content receipts, heap-allocated async scratch, exact filesystem authority and revision-fenced projections remain required.

## Unresolved handoffs

Root retains full-parity work. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Its profile remains untouched. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate requirements.

Open evidence includes the interrupted-Reset Preserve-files choice (system alert inaccessible to the UI tool), OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement/full cutover, four installed architectures and trusted signed-update inputs. Three sampled Quilt hash disagreements remain strict refusals, not proof every release fails. Native queued-history acceptance still waits for its separate picker handoff; see [history import](history-import.md). Earlier observed-settlement source review is complete; runtime/platform gaps remain.

Aggregate pending-inventory admission remains an evidenced follow-up: the replacement bounds individual rows, not their combined read size; legacy bounds its whole journal at 8 MiB. The serial task does not solve that budget. Do not truncate existing obligations or add a scheduler to address it. Root owns that follow-up and hosted verification of this slice.

The full-parity goal remains active. The architecture automation targets `main` and retains its pre-existing paused status. No deployment or release publication.
