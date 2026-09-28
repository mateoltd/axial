# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Detailed historical checkpoints and failures remain in [integration evidence](integration.md) and Git history.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues on `main`, preserving rewrite and upstream history. Root owns shared verification and this record; feature authors and independent reviewers own their bounded source slices. No changed-scope naming or nesting issue warrants style-only churn.

Current scope: same-profile benchmark restart, configured periodic rules refresh, managed-Java import and one duplicate file-revision observation. Their existing feature owners retain state and authority; API composition retains the idle rules worker. Independent source reviews are clear; remaining shared verification is serialized.

## Current findings

- **Restart scope and durable refusal.** Legacy automatic restart is same-profile behavior, not an execution-transfer protocol for imported history. The correction checkpoints overflow before scheduling and isolates only a candidate's missing suite or invalid captured request; storage, corruption and admission failures still stop startup. Exact transaction readback preserves request/source binding. Pre-fix disposable profiles may retain older same-suite restart candidates; UUIDs and wall-clock timestamps do not establish supersession, and this patch does not invent a cohort framework to choose one.
- **Retained background behavior.** Configured rules lost their immediate and periodic refresh during the rewrite. The correction uses the existing TaskOwner, gate, signed-provider verification and atomic publication. Only accepted attempts count as active work; the idle completion-relative wait cannot block native idle closure. Composition owns and joins that idle worker. No separate scheduler or persistence owner.
- **Typed Java selection.** Import's blanket nonempty-path refusal also rejected known managed component IDs. The correction reuses the runtime parser and preserves the effective source selection against destination defaults. Executable paths remain refused; source runtimes are neither probed nor copied. Public instance responses remain redacted.
- **Duplicate observation.** After exact revision validation, the filesystem simplification uses that same revision's size instead of observing it again. Identity, ancestry, read/hash and final operation fences remain. The focused 32-test lifecycle suite passes; broad-run and bounded-rerun evidence are distinguished below. No speed claim.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- **853 app / 123 API / 88 desktop** pass with five app/API ignores each; parent tests execute new ignored subprocess helpers. Four focused API journeys, scoped formatting and independent reviews pass (`background-parity-{consumers,desktop,api-focused,format}.log`). Frontend source and its prior 479-test checkpoint are unchanged.
- Real-process startup resumes only the remaining run after a proved settled boundary, then preserves two exact mappings/reports through another reopen. A surviving uncertain session instead remains fenced without replay, fabricated settlement or report. Helpers use fake Java, not gameplay. Configured signed rules publish through ordinary API startup without a refresh command.
- Meaningful red checks reproduce overflow retry, unrelated-driver suppression, absent refresh and managed-Java refusal. The first core green attempt exposed a test expectation against a redacted public response; the final test separately verifies redaction and private persistence. No production redaction changed (`background-parity-red.log`, `import-java-red.log`, `background-parity-core-green.log`).
- The default-concurrency Minecraft run is **953 passed / 9 timed out** at existing 120-second deadlines. The exact lease-cancellation case passes unchanged in 25.40s in isolation; all nine pass with two threads in 241.24s. Every case has passed on this source, but not in one full green run. A live sample shows active test-only ancestry reconstruction through a 7,703-entry temporary parent. This supports a workload hypothesis, not proof of the sole cause (`revision-size-{minecraft,exact-default,bounded}.log`). No timeout, fixture root or guard changed.

## Retained architectural lessons

Previous corrections remain in the ledger and Git: request-local proof reuse with release-before-wakeup ordering; exact recovery-proof validation; accepted-owner fixture lifetime; content-owned receipts; bounded same-identity proof refresh; heap-allocated async scratch; and revision-fenced projections. Their safety boundaries remain required. Existing AGENTS.md rules already cover this review's lessons, including verifying actual entrypoints before adding machinery; no anecdotal rule is added.

## Unresolved handoffs

Root retains full-parity work. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence, or infer settlement from PID absence/time. Its profile remains untouched. Same-boot process authority and genuine different-boot/native-cleanup proof are separate unresolved recovery requirements.

Other open evidence includes the manual interrupted-Reset Preserve-files choice (system alert inaccessible to the UI tool), OAuth/game-window journeys, remaining content/pack failure/restart cases, developer-interface benchmark continuation, source-effect settlement/full cutover, four installed architectures and trusted signed-update inputs. Three sampled Quilt provider hash disagreements remain strict refusals, not proof every release fails. The historical observed-settlement review gap is now closed by independent review and the correction above; runtime/platform gaps remain.

See [history import](history-import.md) for corrected scope evidence. Native queued-history acceptance still waits for a folder-picker handoff in its separate disposable profile; its source canaries are unchanged. Hosted run36381431343 passes aad07cd0; current locally verified corrections do not inherit that hosted result.

The full-parity goal remains active. The architecture automation targets `main` and retains its pre-existing paused status. No deployment or release publication.
