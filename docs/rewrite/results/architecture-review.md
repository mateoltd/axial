# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: remaining failed-history eligibility, the no-current-instance preservation gap, and queued-history native import/reopen. `f3bea7e5` passes hosted verification. No production, UI, legacy or execution changes in this review.

## Findings and corrections

- **Readability is not an availability attestation.** Source audit found normal `Failed` terminals from unreaped loader processors, foreign native-lane refusal and content worker interruption. Progress writes may fail while execution continues; diagnostic reductions are not compensation proof. The existing history-support consumer also waives unresolved-operation blockers, so broad admission here would violate the delivery contract even with full cutover false. That proposal was rejected before production edits; its uncompiled API test extension was removed. Preserve broader history separately from effect settlement. AGENTS' existing history rule now states this actionable distinction.
- **Inspect the complete owner before adding machinery.** A proposed extra scan for retained `.axial-pack-*` archives was unnecessary: the later inventory pass already bounded-lists and revision-fences the instances parent, blocking every nonregistered child. No duplicate scanner was added. Existing post-rollback fencing and exact terminal validation remain intact.
- **Close the real missing composition.** Validated global history currently publishes/reads only through live-instance mappings, losing coverage when no source instances remain. The next slice should bind it globally and compose with existing metadata completion while preserving unsupported-history metadata import, old receipts and later destination edits. No dummy instance, imported queue authority or universal migration coordinator. Archived targets, independent re-download of managed caches and preference completion remain distinct owners.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- `f3bea7e5` passes hosted [run36409688478](https://github.com/mateoltd/axial/actions/runs/36409688478), following **923 app / 129 API / 88 desktop / 479 frontend** local passes, with six app/API ignores each and one frontend Guardian TODO (`rollback-history-{consumers,desktop,frontend,hosted-ci}.log`). Earlier failed/passing checkpoints and wire/format evidence remain in the integration ledger and Git. No new tests were run for this read-only design review.
- Native queued-history import and no-execution reopen pass on the original `aad07cd0` package, frontend `660aaf2f2c4c`. Both normal Quits exit0; source/repository/sibling canaries and SQLite quick_check pass. One historical interrupted driver retains NULL request, two pending runs and zero launched runs; no launch/report/install-queue rows are created. Screenshots and profile/process correlation are in `queued-native-acceptance.md`. This proves the bounded native journey, not new package acceptance for later source changes.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. The queued-history folder-picker handoff is resolved. Both native runs are stopped, and the fixture keeper remains live for actual installation/developer-UI continuation. Packaged builds intentionally omit the developer lab; use the normal development frontend. Reclaim only verified regenerable build artifacts before starting large downloads with about1.1GiB free. See [history import](history-import.md).

Failed installs without exact compensation evidence, other cancellation/recovery shapes and histories without a current source instance remain separate source-backed obligations. Do not blanket-skip records, treat best-effort diagnostic gaps as settlement, or grant queue execution from history.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
