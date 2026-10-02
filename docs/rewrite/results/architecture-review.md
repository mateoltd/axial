# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, delivery, current evidence and active ownership. Development continues on `main`, preserving history. Root owns shared registration, generated contracts, serialized Cargo verification and native acceptance. Independent feature owners delivered frozen changes; independent review is clear. Architecture automation remains paused; the full-parity goal remains active.

## Findings and corrections

- **Reuse actual readiness.** Ready credential refresh uses existing launch-credential validation and retained account admission, preserving credentials and revisions. Negative ownership synchronization remains a separate gap; credentials alone are not entitlement.
- **Keep uncertain outcomes authoritative.** Updater recovery uses status GETs, existing revision/request fences and owner-coded refusals, without mutation replay. Terminal installation failure exposes the existing restart action; retryable download failure retains retry. A lost pre-admission refusal with unchanged status remains unknown.
- **Isolate external failures narrowly.** Content discovery skips unavailable/malformed project responses while retaining local validation, cancellation and mutation admission. Version-only dependency canonicalization remains a separate advertisement gap.
- **Bound existing reads.** The download registry owner joins both prior GETs before reading the newest cursor, skipping superseded cursors. Generation/publication fences remain; no new cache, journal or coordinator. Independent SettingsPane reads and native latency remain separate.
- **Preserve typed safe causes.** Launch/preflight carry bounded Java-owner failure text while serializing the existing `runtime_unavailable` code. Old persisted code-only refusals remain readable; no provider text or credentials cross the boundary.
- **Publish faithful metadata atomically.** Accountless/unselected legacy profiles preserve absence, Offline settings and username. The v8 migration changes only the account-count constraint, preserving all13 columns/proofs. Zero-account receipts require an empty mapping. Account writes verify affected rows and exact typed readback. No account/credential is invented; accountless offline Play remains a separate gap.
- **Use admitted feature inputs.** Instance Performance planning uses existing installed-mod inspection and retains its plan warnings. Non-managed health returns Disabled without touching retained managed files. Existing task/rules/file ownership remains; the superseded inspection entrypoint is removed.
- **Join effects, not disposable waiters.** Existing autosave queues own outcomes. Save/Reload join settings/JVM/music drafts and interface persistence; Discard parks unsent drafts and joins begun effects. Failure releases seals only after both owners settle. Reload shares pending calls and fences native request/release ABA. No UI layout, parallel persistence or shutdown coordinator is introduced.

Existing AGENTS rules cover these evidenced patterns. No anecdotal rule or stylistic rename is added.

## Validation

Meaningful focused RED/GREEN precedes corrections; fixture/compiler/default-Node failures are distinguished in [integration evidence](integration.md). Frozen full checks pass **1,028 app / 147 API / 88 desktop / 516 frontend tests**, six app/API ignored helpers each and one existing Guardian TODO. Source typing, semantic lint, scoped formatting, generated-contract equality and frontend build/generation verification pass. Logs: `parity-wave-{app-api-full,desktop-full,frontend-final,source-types,semantic-lint,wire-check,frontend-build,generation-check}.log`. Generation `928f504bcdb9` is not the live bundle generation.

Hosted [run37043521520](https://github.com/mateoltd/axial/actions/runs/37043521520) fails exact `54f91a97` at the existing Performance restart fixture with `NoEffect(Busy)`, despite1,017 app passes. That case passes locally in the full wave. Independent causal investigation remains open; no retry/sleep or weaker lifecycle fence is accepted as a fix.

## Unresolved handoffs

Negative entitlement sync, accountless offline Play, version-only dependency advertisement and separately admitted external-library startup remain source-backed gaps. SettingsPane can publish an older detail read over a newer registry projection; a real consumer RED is proposed before changing it. Install already invalidates after worker release, so terminal-before-release alone does not prove the native Busy cause.

The unlocked previous unsigned macOS ARM64 debug bundle completes onboarding, real Fabric installation, normal Quit, same-profile reopen and real managed-Java Playing. The first installed card required restart to restore Ready. Computer use cannot bind Java; title/menu/world/save and clean Stop remain unverified. Evidence applies to `cbcea749`/`0e9d96fdf1b8`. PID88609/Java536 and keeper43579 retain the bundle image; protected canaries pass. The separately aborted interrupted-Reset fixture/keeper7604 remains preserved; abortion is not Preserve-files acceptance.

Horizon inventory establishes capacity, not Linux acceptance; its dirty checkout and login greeter remain untouched. Unsupported source effects/history, interrupted native workflows, four installed architectures and trusted signed-update inputs remain open. No deployment, publication or legacy/user-profile mutation occurred.
