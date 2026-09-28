# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: real queued-benchmark installation, explicit continuation and reopen on `a471f980`, plus independent source reviews of session discovery and sampled settlement work. The preceding global-history correction remains unchanged and passes hosted verification. No product code or UI changed in this acceptance slice.

## Findings and corrections

- **Keep execution separate from history.** Installation and explicit Resume use existing owners. Import and the first reopen start nothing; one linked successor executes two pending runs while the historical source remains unchanged. Final reopen neither repeats work nor rewrites evidence.
- **Do not manufacture a parity defect.** Benchmark-created sessions require reload before ordinary Stop controls adopt them. Legacy also lacks ongoing discovery. Optional polish should compose explicit driver Refresh with a fenced, additive read in the launch owner, not reuse startup's wholesale assignment or introduce another poller/state owner. Preserve concurrent preparation, existing sessions, revisions and accepted Resume outcomes. No implementation landed.
- **Measure before simplifying safety.** A one-second sample locates active post-download canonical settlement validation behind the 99% display. Immediate path planning and observation repeat a leaf-alias check, a possible narrow future simplification; delayed observations must still recheck aliases. Settlement admission and later shard cleanup are separated by file effects and cannot share stale authority. The sample proves neither total duration nor prospective speedup. No checks were removed.
- **Retain prior boundaries.** Global history keeps its bounded snapshot proof and final transaction verification; ordinary Failed labels or absent best-effort progress still do not prove settlement. No new recurring pattern justified another AGENTS rule.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Real zero-instance HTTP and frontend regressions fail at missing/invalid history completion before implementation. A separate late-settings-write regression fails at false success. All nine focused owner groups and 55 focused frontend checks subsequently pass (`global-history-{api-red,frontend-red,frontend-green,late-write-red,owner-final}.log`). One existing test-only settings caller required private-signature adaptation; its compilation failure remains in `global-history-owner-green.log`. Independent frozen-source review is clear.
- Final **932 app / 131 API / 88 desktop / 480 frontend** tests pass, with six app/API ignored helpers each and one existing frontend Guardian TODO (`global-history-{consumers,desktop,frontend-final}.log`). TypeScript, generated contracts and scoped authored formatting pass. A first frontend tooling test hit an independently confirmed unrelated listener on its lease port; unchanged focused 25-test and full reruns pass, with no server or guard changes. Original failure and diagnosis remain in `global-history-{frontend,port-collision,generation-rerun}.log`.
- Current `a471f980` passes hosted [run36413557245](https://github.com/mateoltd/axial/actions/runs/36413557245). Real development-interface installation, reopen, explicit Resume, two managed Java sessions stopped through ordinary controls, and final reopen pass (`queued-browser-acceptance.md`). Two stopped reports and complete2/2 successor remain byte-identical after reopen, with no third launch. All journey services exit0 normally; SQLite quick_check and protected canaries pass. Earlier native import evidence remains qualified to the `aad07cd0` package, not inherited by this run.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import and continuation success is not full migration acceptance. The picker and development-interface queued-continuation handoffs are resolved. Journey services are stopped; the keeper intentionally retains the installed generated fixture for reuse. Only 17 inspected unused build artifacts were removed under the shared lease, reclaiming 2,586,232,704 regenerable bytes without touching source, profiles or diagnostics. See [history import](history-import.md).

Failed installs without exact compensation evidence, other cancellation/recovery shapes and instance-bound histories without current targets remain separate source-backed obligations. Do not blanket-skip records, treat best-effort diagnostic gaps as settlement, or grant queue execution from history.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, same-profile automatic benchmark restart, source-effect settlement, four installed architectures and trusted signed-update inputs. The inherited session-discovery gap and sampled duplicate leaf check are optional bounded follow-ups, not proven full-parity blockers or completed fixes. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
