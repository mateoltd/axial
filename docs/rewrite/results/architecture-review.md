# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR 7, integration evidence and active ownership. Development continues on `main`, preserving history. Performance/import/API/frontend owners are frozen; root owns shared registration, docs and serialized verification. This review covers missing-target Performance history, immutable metadata proofs, namespace observability and final publication. Previous detached-driver and archived-instance evidence remains in [integration evidence](integration.md).

## Findings and corrections

- **Retain one historical owner.** Legacy by-ID Performance reads survive registry deletion. Existing historical command storage, converter, shared batch and status read own the archive. Normal mutation/recovery remain UUID-only. No fabricated instance, new table, route, scheduler or client journal.
- **Preserve exact proof boundaries.** Nullable v6 metadata proof binds source and exact bounded payloads; old-receipt completion preserves later accounts/settings. Completed replay verifies without repair. Unsupported/nonterminal records remain unavailable; historical terminal labels do not waive effects.
- **Reuse namespace admission.** One narrow journal validator serves archive, ordinary, global-install and rules preparation. Aliases, unsafe entries, ancestor files and namespace directories cannot falsely prove empty history. Existing AGENTS guidance already covers this recurring pattern; no duplicate rule is added.
- **Verify after the last write.** A real final-creation trigger reproduced completion after history deletion. Both creation paths now verify operations after their last write, through the existing owner. SQL byte bounds precede allocation and both indexed identity and exact persisted bytes are checked. No recovery framework or stack-limit change.

## Validation

Logs are under `.rewrite-logs/`. Owner, metadata, HTTP and frontend RED expose unreadable archive/missing completion (`archived-performance-{owner,metadata,api,frontend}-red.log`). The final creation regression fails at `performance_commands/initial` with only the final checks removed (`archived-performance-final-write-red-behavior.log`); two preceding fixture type-annotation compile errors remain diagnostic logs, not behavioral evidence. Both checks are restored.

The first broad run passes139 API/986 app and finds only an obsolete valid-deleted-target refusal. That case now tests malformed identity, with archive admission independently covered. Final verification passes **987 app / 139 API / 88 desktop / 486 frontend**, six app/API ignored subprocess helpers each and one existing Guardian TODO (`archived-performance-app-final.log`, `archived-performance-consumers.log`, `archived-performance-desktop.log`, `archived-performance-frontend.log`). All13 new owner/import groups and the three-phase final-publication matrix pass. Focused import tests pass61; source/test typing, generated equality, scoped authored formatting and independent final review pass. Hosted [run37011647301](https://github.com/mateoltd/axial/actions/runs/37011647301) passes the preceding `dd226b3a` checkpoint, not these edits.

## Unresolved handoffs

This slice adds no native-interface acceptance and does not establish full cutover. Missing-target content history, the evidenced per-instance latest-operation registry-gating mismatch, unsupported records and source-effect settlement remain separate. Changed-source metadata replay retains its existing fingerprint conflict. Actual managed Java automatic-restart evidence stays qualified to its earlier unchanged product checkpoint; it is not inherited by later edits.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
