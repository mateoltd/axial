# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: ordinary legacy import selection and Guardian-only eligibility records. No UI, wire, schema, table, coordinator or namespace changes.

## Findings and corrections

- **Keep selection with its owner.** Import validated but dropped the source last-successful-instance pointer, which is distinct from a saved UI route. One private source-derived boolean lets the existing registry fill an empty selection during first live publication. Existing destination choices win. Completed replay does not restore later changed/cleared selections; no launch timestamp or revision is fabricated.
- **Acknowledge actual completion.** Normal and recovered creation-completion updates now require one affected row. An ignored write rolls back both live publication and restored selection instead of returning false success. Existing transactions and retained creation receipts remain authoritative.
- **Separate Guardian eligibility from accepted effects.** Legacy startup writes rejection-streak history even when empty; the importer's unknown-record fallback blocked ordinary profiles. Strict raw v1 decoding now recognizes only this exact record, including original byte/count, canonical identity, ordering and count bounds. Valid records stay captured, fingerprinted and source-fenced but provide no imported repair authority. Unknown fields and duplicate fields cannot disappear through JSON normalization. Malformed/unsafe records, pending deletion and accepted-effect journals remain blocking.
- **Verify the suspected path.** The initial known-good blocker hypothesis was disproved: that exact cache directory is already excluded. It is ordinary installation provenance, not Guardian-only history, and source bytes never grant destination activation authority. No additional skip, parser or adoption path was added.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Selection tests first reproduce both absent mapping and successful import despite ignored selection publication (`import-selection-red.log`). After correction, all 66 other import tests pass while four new Guardian-boundary tests fail meaningfully (`import-streaks-red-selection-green.log`).
- All **70 focused import tests pass** with both corrections (`import-preservation-focused.log`), including real publication, replay, rollback, reopened source-readmitted recovery, source preservation and independent retained blockers.
- Full two-thread verification passes **876 app / 127 API tests**, with six ignores in each (`import-preservation-consumers.log`). The existing API import/replay case includes the routine legacy startup record and checks mapped selection and unchanged source bytes. This is not native-interface acceptance.
- Independent final source reviews cover both fixes and API integration. Scoped formatting and whitespace checks pass. Existing AGENTS rules already cover feature ownership, original-schema validation and affected-row acknowledgement; no redundant rule was added.
- Hosted run36394051101 passes application and delivery on `3a2b1faf`, before these import changes. Frontend product source remains unchanged. Earlier observer/budget fixes, local timeout qualifications and Minecraft default-concurrency failures remain recorded in the integration ledger; they are not erased by a later green checkpoint.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
