# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Detailed historical checkpoints and failures remain in [integration evidence](integration.md) and Git history.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues on `main`, preserving rewrite and upstream history. Root owns shared verification and this record; feature authors and independent reviewers own their bounded source slices. No changed-scope naming or nesting issue warrants style-only churn.

Current implementation: preserve-only Quit after interrupted-launch restoration. The API/native lifecycle owner handles shutdown admission; the process-fixture owner handles crash/reopen evidence; an independent reviewer checks authority and admission ordering. Frozen-diff review and full API/desktop verification pass.

## Current findings

- **Interrupted launch versus Quit.** API shutdown refused before draining this process's work, although the existing library preservation path can settle local effects without revoking or declaring the older process stopped. The correction removes that early ordinary-shutdown refusal, not the durable obligation or exclusions.
- **Terminal action admission.** Restored launch reservations do not occupy TaskOwner. Native Update can skip API shutdown, so its idle-task check did not enforce the interrupted-launch fence. Restart, Reset and Update now check the existing library authority before preferences or task admission close. Native Reset refuses before retaining its deletion pin. A direct API facade exposes the same owner check, not new state or policy. No new journal, process adoption, mutable mirror or inferred settlement.
- **Explicit queued-history Resume, integrated.** Legacy manual Resume accepts interrupted queued markers. The earlier blanket destination refusal confused new explicit destination execution with consumption of the predecessor obligation. Removing that predicate preserves independent copying, immutable source history, exact current-plan/report checks, destination exclusion, successor links and idempotency. Import/startup still starts no historical execution; cutover stays unavailable. See [history import](history-import.md).
- **Nested dialog ownership, integrated.** Screenshot Rename exposed two modal claims and Escape closing both surfaces. Suspension now derives from the existing dialog signal. Escape is consumed before clearing it; one bounded focus-return frame retains the opener across replacement/chaining and cancels on new dialogs or intentional focus movement. Browser testing caught missing initial prompt focus; the existing guarded frame now focuses its input. No CSS, new portal framework or mirrored modal stack. See [screenshot evidence](screenshot-files.md).

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Queued Resume: **831 app / 119 API**, unchanged five/two ignores; 26 benchmark, nine composed import and four actual-process journeys pass. Two new queued cases fail at old eligibility before correction. Lost responses/reopen preserve the same successor, inherited reports and unchanged source bytes/mtime, with zero execution before explicit Resume. Frozen-diff review is clear; hosted run36376686191 passes for `698c105d`. Fixture processes are not gameplay.
- Dialog: five composed regressions and **479 frontend tests**, one unchanged Guardian TODO; semantic lint, formatting, API build and generation `660aaf2f2c4c` pass. Independent review is clear. Real browser checks prove initial focus, Tab containment, sole foreground accessibility and separate Escape/restore steps; validation and layout remain unchanged. Renamed world/image restart and hashes pass; APIs exit normally.
- Initial dialog fixtures incorrectly attached Preact refs; that failure preceded behavior. An intermediate focus run overlapped a temporary production-hunk revert and failed/timed out. Only the frozen final runs support green claims.
- Preserve-only Quit: unchanged-source red tests reproduce late desktop refusal and the real crash/reopen shutdown refusal. The corrected real-process journey passes: a valid bound accepted intent survives launcher exit before observation, two reopens retain exact intent/native bytes, replay and conflicting writes refuse, and ordinary shutdown succeeds while the fixture game and descendant are still independently alive. Only the test then stops those processes through its private channels; no report, acknowledgement or settlement is fabricated. This is Unix fixture-process evidence, not native UI, gameplay or Windows survival proof. Logs: `preserve-quit-{desktop,process}-red.log`, `preserve-quit-process-green.log`.
- **120 API / 88 desktop tests pass**, three API ignores including the new subprocess helper exercised by its parent. Two focused desktop checks prove refusal before preference/task admission and reset-pin retention; their failed-restoration fixture deliberately establishes only the sticky fence, while the actual-process journey covers valid bound restoration. Formatting and independent review pass (`preserve-quit-{api,desktop,desktop-green,format}.log`). Core/frontend source is unchanged; their earlier checkpoints were not rerun without a changed dependency.

## Retained architectural lessons

Previous corrections remain documented in the integration ledger and feature records: exact durable observation before optional report persistence; content-owned receipts for local mod effects; bounded same-identity filesystem proof refresh; heap-allocated decoder scratch across async yields; and projection invalidation only after accepted ownership releases. Their safety fences remain required, not complexity to remove.

Readiness is still slow: the last isolated debug detail/list samples measured 15,073/45,224 ms after removing redundant outer hash checks, versus 16,096/48,469 ms before. This single comparison is not a stable benchmark; fixture and sampling caveats remain in the ledger. Per-request artifact reuse is reviewed but unimplemented.

Existing AGENTS.md rules cover the current findings. Do not add duplicate anecdotal rules. Historical details were consolidated here rather than repeated from the integration ledger; evidence remains recoverable.

## Unresolved handoffs

Root retains full-parity work. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence, or infer settlement from PID absence/time. Its profile remains untouched. Same-boot process authority and genuine different-boot/native-cleanup proof are separate unresolved recovery requirements.

Other open evidence includes the manual interrupted-Reset Preserve-files choice (system alert inaccessible to the UI tool), OAuth/game-window journeys, remaining content/pack failure/restart cases, developer-interface benchmark continuation, automatic cross-profile handoff/cutover, four installed architectures and trusted signed-update inputs. Three sampled Quilt provider hash disagreements remain strict refusals, not proof every release fails. Earlier observed-settlement work lacked a final independent review at its original checkpoint; do not silently treat historical tests as that review.

The full-parity goal remains active. The architecture automation targets `main` and retains its pre-existing paused status. No deployment or release publication.
