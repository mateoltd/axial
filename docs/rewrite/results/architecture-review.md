# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: same-profile automatic benchmark restart with actual managed Java/Minecraft on unchanged `a471f980` product source at `9ed285de`. Independent reviewers checked source parity and the interruption boundary. No product code or UI changed in this acceptance slice.

## Findings and corrections

- **Keep execution separate from history.** Installation and explicit Resume use existing owners. Import and the first reopen start nothing; one linked successor executes two pending runs while the historical source remains unchanged. Final reopen neither repeats work nor rewrites evidence.
- **Verify the actual settlement owner.** Automatic-restart acceptance interrupted only the disposable API after exact session/tree/output/native/report settlement and while run2 remained unreserved. Stale suite labels and the driver's last active-session field are not unresolved effects when the canonical owner proves settlement; waiting for the next tick would also launch the second run. Restart used the existing driver/request without client Resume/Tick. Legacy normal Quit stores Stopped, so automatic continuation after orderly Quit is not a parity requirement.
- **Do not manufacture a parity defect.** Benchmark-created sessions require reload before ordinary Stop controls adopt them. Legacy also lacks ongoing discovery. Optional polish should compose explicit driver Refresh with a fenced, additive read in the launch owner, not reuse startup's wholesale assignment or introduce another poller/state owner. Preserve concurrent preparation, existing sessions, revisions and accepted Resume outcomes. No implementation landed.
- **Measure before simplifying safety.** A one-second sample locates active post-download canonical settlement validation behind the 99% display. Immediate path planning and observation repeat a leaf-alias check, a possible narrow future simplification; delayed observations must still recheck aliases. Settlement admission and later shard cleanup are separated by file effects and cannot share stale authority. The sample proves neither total duration nor prospective speedup. No checks were removed.
- **Retain prior boundaries.** Global history keeps its bounded snapshot proof and final transaction verification; ordinary Failed labels or absent best-effort progress still do not prove settlement. No new recurring pattern justified another AGENTS rule.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Real zero-instance HTTP and frontend regressions fail at missing/invalid history completion before implementation. A separate late-settings-write regression fails at false success. All nine focused owner groups and 55 focused frontend checks subsequently pass (`global-history-{api-red,frontend-red,frontend-green,late-write-red,owner-final}.log`). One existing test-only settings caller required private-signature adaptation; its compilation failure remains in `global-history-owner-green.log`. Independent frozen-source review is clear.
- Final **932 app / 131 API / 88 desktop / 480 frontend** tests pass, with six app/API ignored helpers each and one existing frontend Guardian TODO (`global-history-{consumers,desktop,frontend-final}.log`). TypeScript, generated contracts and scoped authored formatting pass. A first frontend tooling test hit an independently confirmed unrelated listener on its lease port; unchanged focused 25-test and full reruns pass, with no server or guard changes. Original failure and diagnosis remain in `global-history-{frontend,port-collision,generation-rerun}.log`.
- Current `a471f980` passes hosted [run36413557245](https://github.com/mateoltd/axial/actions/runs/36413557245). Real development-interface installation, reopen, explicit Resume, two managed Java sessions stopped through ordinary controls, and final reopen pass (`queued-browser-acceptance.md`). Two stopped reports and complete2/2 successor remain byte-identical after reopen, with no third launch. All journey services exit0 normally; SQLite quick_check and protected canaries pass. Earlier native import evidence remains qualified to the `aad07cd0` package, not inherited by this run.
- `9ed285de` passes hosted [run36416204476](https://github.com/mateoltd/axial/actions/runs/36416204476). Its real-game pending-boundary interruption resumes only run2 automatically, completes2/2 on the configured interval and preserves exact history on another clean reopen without a fifth total launch (`automatic-browser-acceptance.md`). The guarded interruption is deliberately not graceful-close evidence. The stale history assessment's claim that bounded restart/overflow corrections were still unimplemented is corrected; existing production checks remain unchanged.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import and continuation success is not full migration acceptance. The picker and development-interface queued-continuation handoffs are resolved. Journey services are stopped; the keeper intentionally retains the installed generated fixture for reuse. Only 17 inspected unused build artifacts were removed under the shared lease, reclaiming 2,586,232,704 regenerable bytes without touching source, profiles or diagnostics. See [history import](history-import.md).

Failed installs without exact compensation evidence, other cancellation/recovery shapes and instance-bound histories without current targets remain separate source-backed obligations. Do not blanket-skip records, treat best-effort diagnostic gaps as settlement, or grant queue execution from history.

Next concrete candidate: legacy deletion removes instance registry membership while launch-report reads remain independent; current `import/history.rs` rejects a report whose source instance is absent. Preserve those archived terminal reports through existing evidence owners, subject to historical-identity and public-read review before choosing any receipt extension. Do not fabricate a live instance, rebind the record to another instance, or waive nonterminal/suite/file obligations.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, applicable native/platform automatic benchmark restart, source-effect settlement, four installed architectures and trusted signed-update inputs. The browser/API real-game restart case now passes. The inherited session-discovery gap and sampled duplicate leaf check are optional bounded follow-ups, not proven full-parity blockers or completed fixes. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
