# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: exact pre-worker content initialization failure history, optional original failure reason, and actual import/read/reopen/replay coverage. The previous successful-history slice passes hosted verification on `117e6d84`. UI, legacy source and live install execution are unchanged.

## Findings and corrections

- **Prove the boundary from the source owner.** Legacy initialization-reservation cleanup verifies the exact planned record before recording the failure; worker hand-off disarms that cleanup before content execution. Ordinary queue waiter cancellation does not imply no effects. Admit only the exact Content/Failed tuple and singleton initializing failure, with no metrics, changed target, Guardian evidence or earlier step. Other source obligations retain their blockers.
- **Extend one existing validator.** The narrow terminal branch follows unchanged common source, identity and plan checks. Persisted readback uses it too. Source evidence already contains the failure point, so no schema, table, coordinator, recovery loop or execution owner is added. Existing atomic binding/publication/replay and bounded queries remain unchanged.
- **Preserve facts, not a friendlier outcome.** Public history retains original `Failed` and `content_initialization_cancelled`; it never invents `Cancelled`, readiness or retry authority. The optional failure point is omitted for successful records. Mixed success/failure, actual copy/reopen/replay, source fences and persisted corruption are tested through their production owners.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- The pre-worker failure slice passes **916 app / 129 API / 88 desktop tests**, with six app/API ignores each, plus **479 frontend tests** and one existing Guardian TODO. Coverage includes three owner groups, four real importer checks and authenticated source-free reopen/replay. Its real HTTP regression first fails at ordinary import availability, not compilation or unrelated setup (`content-init-history-{red,consumers,desktop,frontend}.log`). Independent source/fixture review, scoped formatting, TypeScript and generated-contract equality pass (`content-init-history-{format,wire}.log`).
- Previous successful-history checkpoint `117e6d84` passes hosted [run36406064180](https://github.com/mateoltd/axial/actions/runs/36406064180), following local **909 app / 129 API / 88 desktop / 479 frontend** passes. Earlier fixes, initial failures and runtime qualifications remain in the integration ledger and Git; they are not new acceptance evidence for this slice.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Successful Generic history and the exact pre-worker content failure pass local application/API integration. Broader failed/cancelled records, recovery-bearing effects and histories without a current source instance remain separate source-backed obligations. Do not blanket-skip records or grant queue execution from history. Existing AGENTS source-contract and feature-ownership rules cover this extension; no duplicate rule added.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
