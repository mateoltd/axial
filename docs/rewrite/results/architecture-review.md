# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: successful install/content history import, immutable persistence/readback, existing instance publication/replay, authenticated reads and generated contracts. Prior witness-import, Playing-edit and resource checkpoints remain in the integration ledger. UI, legacy source and live install execution are unchanged.

## Findings and corrections

- **Keep evidence with its feature owner.** One install-owned immutable table is justified: the live queue requires requests, timestamps and executable authority absent from Generic history. No new coordinator, journal, worker or recovery protocol. Existing import preparation and instance final transactions own publication; completed retry verifies without repairing history.
- **Bound actual query work.** The initial source/id index could scan every other instance's accumulated history before satisfying the page limit. The corrected source/instance/id index supports two individually bounded branches, global and exact instance, before bounded merge and primary-key payload reads.
- **Preserve distinct source validators.** Original target text projection and persisted structured-token admission have different rules. Review caught both missing rejection and an overrestrictive attempted reuse. Keep exact typed loader/publication codecs, separate structured-token checks and complete progress-fact validation; do not apply generic secret heuristics to typed identities. Raw metrics retain duplicate-field rejection and full u64 values, publicly encoded as decimal strings.
- **Validate before advertising eligibility.** Cached stored-envelope sizes support one source-wide batch check before per-instance preview, without repeated serialization. Global immutable records share preparation. Final insertion still verifies affected rows and the entire batch after writes, catching later triggers that alter earlier rows.
- **Reuse mapping authority.** History reads reuse the instance owner's completed/live mapping checks in one read transaction. Validated cursors apply equally to mapped and ordinary instances. Callers cannot substitute a source ID, name or version; history cannot create queue, readiness or mutation authority.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Current successful-history integration passes **909 app / 129 API / 88 desktop tests**, with six app/API ignores each, plus **479 frontend tests** and one existing Guardian TODO. All nine history-owner checks and four real import checks pass inside that full run, alongside actual publication/recovery and authenticated source-free reopen. Independent frozen-source review, scoped Rust formatting, TypeScript and generated-contract equality pass (`install-history-consumers.log`, `install-history-desktop.log`, `install-history-frontend-verified.log`, `install-history-types-final.log`, `install-history-format-final.log`, `install-history-wire-check.log`). The initial null-metrics expected-JSON mutation was test-only; route-inventory and formatting corrections retain their original gates. See the integration ledger for those failures.
- The focused pre-fix run reports **two passes / three expected valid-record failures**. All **five grouped regressions now pass**, including actual copy/reopen/replay, no Guardian publication, inclusive bounds, malformed records, source drift and retained effects (`import-user-mod-witness-{red,green}.log`). Independent final source/test review and scoped formatting/whitespace checks pass. Full two-thread consumers pass **894 app / 128 API tests**, with six ignores in each, plus **88 desktop tests** (`import-user-mod-witness-consumers.log`, `import-user-mod-witness-desktop.log`).
- Prior Playing-edit checkpoint `8d4a305a` passes hosted [run36400638582](https://github.com/mateoltd/axial/actions/runs/36400638582), following local **889 app / 128 API / 88 desktop** passes. Prior import and backup checks, failures and runtime qualifications remain in the ledger. Existing source-schema and intended-boundary rules cover this correction; no redundant AGENTS rule added.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Successful Generic history passes local application/API integration. Failed/cancelled records, recovery-bearing effects and histories without a current source instance remain separate source-backed obligations. Do not blanket-skip records or grant queue execution from history. Existing AGENTS source-contract, bounded-work and atomic-publication rules cover these findings; no duplicate rule added.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
