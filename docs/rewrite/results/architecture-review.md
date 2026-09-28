# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: ordinary music-cache and saved-wardrobe import classification, plus real world backup/deletion acceptance. No UI, wire, schema, table, coordinator or namespace changes. A separate Playing metadata-edit correction is in progress under its instance/launch owner and is not included in this checkpoint.

## Findings and corrections

- **Recognize actual ordinary source topology.** Legacy background music creates `music/`, including an empty cache. Inventory previously made it an unknown global blocker. Only the exact root directory and direct fixed track names under the original per-file bound are now recognized. Bytes, directory revisions and fingerprints stay captured; unknown/nested/alias/link/scratch/oversized entries remain refused. No blanket directory skip or destination cache authority.
- **Keep independent publication independent.** Valid saved skins blocked instance copy even after a separate skin import. Instance preparation now recognizes only records already validated by the strict wardrobe converter. It reuses the immutable capture result instead of decoding a potentially large PNG batch per instance. No new receipt, copied state or skin side effect; public obligations and full-cutover refusal remain honest.
- **Verify actual resource outcomes.** Current backup code passes the browser-driven backup/delete/reopen journey for the named177-byte synthetic world. Both backups retain exact bytes, unrelated files are unchanged and both API processes exit0. This closes ordinary deletion acceptance, not gameplay or interrupted-file recovery.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- Meaningful RED: three expected availability failures for empty music cache, valid saved wardrobe and preparation needed for the source-drift case; three unsafe/unknown controls already pass (`import-ordinary-records-red.log`). All **six focused tests pass** after correction (`import-ordinary-records-green.log`).
- All **109 matching import/instance-publication tests pass** from the same compiled app checkpoint (`import-ordinary-records-suite-final.log`). Full consumer/hosted verification of this later correction is pending. Independent final source review, scoped formatting and whitespace checks pass. Existing AGENTS rules cover source-schema, bounded-work and ownership lessons; no redundant rule added.
- Prior backup checkpoint `0037c3b4` passes **878 app / 127 API tests** and hosted [run36397803521](https://github.com/mateoltd/axial/actions/runs/36397803521). Its real resource journey is recorded in `world-resource-acceptance.md`; earlier failures and qualifications remain in the ledger.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
