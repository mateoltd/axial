# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: instance Rename/next-launch settings while Playing, with an explicit instance/session boundary and unchanged API contract. No UI, schema, table, coordinator or namespace changes. Prior import and resource acceptance checkpoints remain in the integration ledger.

## Findings and corrections

- **Lend existing authority instead of bypassing exclusion.** Legacy permits metadata edits while Playing, but replacement update admission conflicted with its own live session. A current Running session now lends its retained instance clone to the instance writer. A status boolean alone never authorizes the write. Weak preparation references prevent terminal session history from retaining root/artifact/account ownership; only the required instance lease/pin survives through commit.
- **Validate the right snapshot.** Normal launch recency already makes the original captured metadata row stale. The shared physical-binding check verifies the retained directory receipt, immutable target and current live row, then the existing revision-CAS update publishes next-launch settings. Full pre-spawn validation is unchanged. Stale revisions, owner mismatches, pending content/Performance/setup effects and retargeting remain refused.
- **Keep lifecycle checks at admission.** Starting, dead, stopping, unresolved and closing sessions cannot lend admission. A loan accepted before Stop keeps the same exclusion until its bounded metadata write ends, preventing deletion/content from entering the gap. API adaptation passes the shared session owner; it contains no new business logic. Ordinary non-session update callers remain exclusive.
- **Respect production diagnostics in test fixtures.** The first successful edit journey failed only its raw JVM-flag log assertion. The fixture now derives and emits a numeric memory value from the actual child arguments; production redaction stays strict. The regression still checks actual running and next-launch process inputs, not just a parallel scenario model.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- The real fixture-process journey first fails at Playing Rename with HTTP 409 Busy and safely settles before reporting RED (`playing-metadata-red.log`). It now passes two edits, stale/retarget/delete refusals, original running-process memory, clean Stop/reopen and new next-launch memory (`playing-metadata-green-final.log`).
- All **30 matching app metadata tests pass**, including five new loan/binding/pending-effect checks (`playing-metadata-guards.log`). Full two-thread consumers pass **889 app / 128 API tests**, with six ignores in each, plus **88 desktop tests** (`playing-metadata-consumers.log`, `playing-metadata-desktop-final.log`). Independent final source review, scoped formatting and whitespace checks pass. Existing ownership, snapshot and diagnostics rules cover this correction; no redundant AGENTS rule added.
- Prior import checkpoint `58b7f26f` passes all **109 matching import/publication tests** and hosted [run36399529249](https://github.com/mateoltd/axial/actions/runs/36399529249). Backup checkpoint `0037c3b4` also passes hosted verification and its real world-resource journey. Earlier failures and qualifications remain in the ledger.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Read-only follow-up found two concrete import gaps for the next owner: successful-launch `guardian-user-mod-witnesses.json` is classified as unknown despite its Guardian-only behavioral consumer; Generic InstallVersion/ModifyInstanceContent histories are routed into the rules converter. Recognize only exact original Guardian records, and distinguish clean terminal history from checkpoints/reconciliation/file effects before permitting independent copies. Do not blanket-skip filenames, waive by status alone or grant queue execution from history.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
