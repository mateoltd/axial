# Architecture review

Updated 2026-09-28. Changed-scope review, not a parity or release certificate. Historical checkpoints and failures remain in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, delivery/parity requirements, integration evidence and active ownership. Work continues directly on `main`, preserving history. Feature owners and independent reviewers are frozen; root owns serialized verification and integration.

Current scope: strict recognition of the Guardian-only successful-launch user-mod witness during ordinary instance import, plus a read-only audit of remaining install/content history. No UI, schema, table, coordinator or namespace changes. Prior Playing-edit and resource acceptance checkpoints remain in the integration ledger.

## Findings and corrections

- **Recognize evidence at its existing owner.** Ordinary legacy launches persist `guardian-user-mod-witnesses.json`, but its only behavioral consumer supplies Guardian diagnosis evidence. Inventory now recognizes only the exact root filename and valid original schema. No new migration owner, destination state or replay authority is introduced.
- **Preserve the source contract before normalization.** Typed raw decoding rejects duplicate/unknown fields and preserves full u64 size/time values. The original bounds, strict instance ordering and nondecreasing entry tuples remain; legal duplicates, offset RFC3339 dates and stale instance witnesses are not tightened against current state. Existing capture, hash, fingerprint and source revalidation remain unchanged.
- **Prove the intended refusal.** The retained-effect regression now uses the existing valid journal fixture transitioned to Running, requires successful capture and asserts `UnsettledOperation`. An incomplete JSON entry could have passed merely through malformed-source rejection. Hardlink admission may still refuse before inventory, preserving the native boundary.
- **Do not waive ordinary history by terminal status.** The follow-up audit found that successful installs retain committed publication checkpoints, and content success retains typed metrics. Their eventual converter must preserve exact historical evidence without inventing timestamps, queue requests or current installed authority. The bounded handoff is recorded in [history import](history-import.md); it is not yet implemented.

## Validation

Logs are under `.rewrite-logs/`; counts apply only to their recorded checkpoint.

- The focused pre-fix run reports **two passes / three expected valid-record failures**. All **five grouped regressions now pass**, including actual copy/reopen/replay, no Guardian publication, inclusive bounds, malformed records, source drift and retained effects (`import-user-mod-witness-{red,green}.log`). Independent final source/test review and scoped formatting/whitespace checks pass. Full two-thread consumers pass **894 app / 128 API tests**, with six ignores in each, plus **88 desktop tests** (`import-user-mod-witness-consumers.log`, `import-user-mod-witness-desktop.log`).
- Prior Playing-edit checkpoint `8d4a305a` passes hosted [run36400638582](https://github.com/mateoltd/axial/actions/runs/36400638582), following local **889 app / 128 API / 88 desktop** passes. Prior import and backup checks, failures and runtime qualifications remain in the ledger. Existing source-schema and intended-boundary rules cover this correction; no redundant AGENTS rule added.

## Unresolved handoffs

Full profile cutover remains unavailable; individual import success is not full migration acceptance. Native queued-history acceptance still waits for its folder-picker handoff. The latest read-only canary check passes; no import was accepted and no fixture process was restarted. See [history import](history-import.md).

Generic InstallVersion/ModifyInstanceContent histories still route into the rules converter. The next owner needs command-aware dispatch in both instance and rules preparation, immutable install-owned readable history, exact source/global versus instance binding, and atomic publication/recovery/replay checks. Successful Vanilla/loader checkpoints and content metrics need faithful validation; failed/cancelled and recovery-bearing effects remain separate obligations. Do not blanket-skip records or grant queue execution from history.

Open evidence includes interrupted-Reset Preserve-files choice, OAuth/game-window journeys, content/pack failure and restart cases, developer-interface benchmark continuation, source-effect settlement, four installed architectures and trusted signed-update inputs. Preserve-only Quit must not repair old intent `3ce61942-4c5b-4cdb-a899-70e0f545fa64`, clear its fence or infer settlement from PID absence/time. Same-boot process authority and genuine different-boot/native-cleanup proof remain separate.

The full-parity goal remains active. Architecture automation targets `main` and retains its pre-existing paused status. No deployment, release publication or legacy/user-profile mutation.
