# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: global install-history preservation without current instances, metadata completion/replay, authenticated reads and frontend receipt validation. `67dde617` passes hosted verification. Existing UI, failed-history eligibility, instance availability and execution authority remain unchanged.

## Findings and corrections

- **Reuse the existing owners.** Strict global history binds without an instance and publishes through the metadata transaction into the existing install-history store. One nullable proof on the existing receipt replaces neither owner and introduces no journal, placeholder instance or execution authority. Unsupported history leaves accounts/settings eligibility intact; old NULL receipts can complete history without rewriting later destination edits.
- **Prove a snapshot, not a changing collection.** Source binding, sorted unique IDs and one digest identify the exact original payloads, including empty history. Later imports cannot invalidate an earlier receipt. Each read verifies at most 128 records/8 MiB, applying aggregate limits before payload allocation; it does not rescan a full batch per record. The receipt's stored source/fingerprint/import ID must agree. Completed replay never inserts or repairs history.
- **Verify after the last write.** A real late-settings trigger demonstrated false completion after earlier verification. The specialized settings-import callback now returns its existing receipt and verifies it after the final settings write, still inside the same transaction. This removes captured mutable receipt state without introducing a generic transaction framework. A review also restored transaction-local cancellation checks around old-receipt completion. Affected rows and exact readback remain mandatory.
- **Keep settlement separate.** The preceding review rejected broad Failed-history admission because ordinary terminals and missing best-effort progress cannot prove file/process settlement. Those guards remain unchanged. Existing AGENTS rules already cover ownership, bounded batches and atomic completion; no duplicate rule was added.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Real zero-instance HTTP and frontend regressions fail at missing/invalid history completion before implementation. A separate late-settings-write regression fails at false success. All nine focused owner groups and 55 focused frontend checks subsequently pass (`global-history-{api-red,frontend-red,frontend-green,late-write-red,owner-final}.log`). One existing test-only settings caller required private-signature adaptation; its compilation failure remains in `global-history-owner-green.log`. Independent frozen-source review is clear.
- Final **932 app / 131 API / 88 desktop / 480 frontend** tests pass, with six app/API ignored helpers each and one existing frontend Guardian TODO (`global-history-{consumers,desktop,frontend-final}.log`). TypeScript, generated contracts and scoped authored formatting pass. A first frontend tooling test hit an independently confirmed unrelated listener on its lease port; unchanged focused 25-test and full reruns pass, with no server or guard changes. Original failure and diagnosis remain in `global-history-{frontend,port-collision,generation-rerun}.log`.
- Previous `67dde617` passes hosted [run36411161884](https://github.com/mateoltd/axial/actions/runs/36411161884). Earlier native queued-history import/reopen evidence remains qualified to the `aad07cd0` package in [integration evidence](integration.md), not inherited by this source change.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. The queued-history folder-picker handoff is resolved. Both native runs are stopped, and the fixture keeper remains live for actual installation/developer-UI continuation. Packaged builds intentionally omit the developer lab; use the normal development frontend. Reclaim only verified regenerable build artifacts before starting large downloads with about1.1GiB free. See [history import](history-import.md).

Failed installs without exact compensation evidence, other cancellation/recovery shapes and instance-bound histories without current targets remain separate source-backed obligations. Do not blanket-skip records, treat best-effort diagnostic gaps as settlement, or grant queue execution from history.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
