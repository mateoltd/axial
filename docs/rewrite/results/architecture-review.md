# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: exact settled failed-install rollback history, strict excluded diagnostic evidence, and actual import/read/reopen/replay coverage. The previous pre-worker content failure slice passes hosted verification on `7404bbfa`. UI, legacy source and live install execution are unchanged.

## Findings and corrections

- **Prove settlement, not merely a checkpoint.** Ordinary/startup rollback writes its checkpoint before native acknowledgment. Require the later exact failure terminal as well. Admit only Vanilla rollback, loader-base rollback, or base commit followed by child rollback. Review caught a permissive draft continuation after rollback; a direct fence now refuses further progress/publication before terminal failure. Checkpoint-only and missing-compensation histories remain blocked.
- **Validate excluded data at its source boundary.** One shared strict raw codec rejects duplicate/unknown nested fields before JSON normalization. Closed predecessor registries match all 117 fact and 76 diagnosis labels, including phase variants. Original bounds, diagnosis references and canonical five-minute historical retry windows remain enforced without consulting today's clock or restoring Guardian state. The same validation protects persisted readback.
- **Extend the existing owner.** The successful checkpoint validator now handles the exact rollback shape; no new schema, table, coordinator, recovery loop, public contract or execution owner. Original `Failed`, failure point and neutral publication evidence remain unchanged. Rollback is recorded settlement, not absence of previous effects. Atomic binding/publication/replay and bounded queries are unchanged. Four owner groups and real importer/API journeys cover source drift, malformed proof, wire fidelity and stored corruption.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Rollback-history regressions compile and produce three intended valid-import failures plus one refusal pass before implementation (`rollback-history-red.log`). Final two-thread verification passes **923 app / 129 API / 88 desktop tests**, with six app/API ignores each, plus **479 frontend tests** and one existing Guardian TODO (`rollback-history-{consumers,desktop,frontend}.log`). Independent frozen-source review, scoped formatting, TypeScript and generated-contract equality pass (`rollback-history-{format,wire}.log`).
- Previous pre-worker failure checkpoint `7404bbfa` passes hosted [run36407771277](https://github.com/mateoltd/axial/actions/runs/36407771277), following local **916 app / 129 API / 88 desktop / 479 frontend** passes (`content-init-history-hosted-ci.log`). Earlier fixes, initial failures and runtime qualifications remain in the integration ledger and Git; they are not new acceptance evidence for this slice.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Failed installs without exact compensation evidence, other cancellation/recovery shapes and histories without a current source instance remain separate source-backed obligations. Do not blanket-skip records or grant queue execution from history. Existing AGENTS source-contract and feature-ownership rules cover this extension; no duplicate rule added.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
