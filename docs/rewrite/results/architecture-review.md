# Architecture review

Updated 2026-10-04. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, delivery and current evidence. Development remains on `main`, preserving history. This intentionally breaking pre-release excludes predecessor-profile import/schema upgrades, not current-app persistence, accepted-operation recovery or older Minecraft/loaders. Root owns shared registrations, generated contracts, asset inventory and serialized verification. Feature changes are frozen; the remaining accountless Offline launch fix has explicit ownership. The full-parity goal remains active.

## Findings and corrections

- **Remove the superseded owner.** Predecessor preview/import routes, native commands, controls, receipt/archive/transfer protocols and dedicated tests are removed. Incremental table upgrades are folded into current initial DDL. Current create/duplicate/install/launch/Content/Performance recovery, atomic acknowledgements, compensation, exact filesystem authority and unknown-process fences remain.
- **Use actual current fixtures.** Retained recovery/projection tests now create instances through the current owner instead of predecessor import or fabricated registry rows. The test-only loader target constructor derives canonical coordinates. Four remaining no-caller predecessor helpers are removed, not generalized.
- **Keep native Resume native.** Benchmark history, qualification, saved requests, same-driver Resume and automatic restart retain their existing owner. Historical successor lineage, badges and parallel reconciliation are removed. API action projection and frontend same-id publication agree; settlement without a required report remains blocked.
- **Record ownership separately from credentials.** Negative entitlement sync preserves secure credentials while publishing known-false ownership. Launch preflight/implicit prepare and new skin actions refuse it; explicit re-sync can restore ownership. Accepted skin effects keep their obligations. Ownership changes retain account/selection revision fences and exact durable readback.
- **Reuse bounded provider resolution.** Update discovery resolves version-only dependency identities once through the existing bounded provider owner. Malformed/unavailable candidates cannot suppress independent healthy updates; local validation, cancellation and exact mutation admission remain.
- **Keep readiness publication in its owner.** SettingsPane uses the existing readiness refresh instead of independently publishing stale detail. Passive reads cannot displace a current retry owner; abandoned reads do not block fresh ones. Config, instance and session fences remain.
- **Capture absence without inventing identity.** Offline Play after final-account removal now captures None/Offline with its exact selection revision, including create/remove ABA. The existing preflight/pre-spawn path checks that capture and settings revisions. Current config and launch share one valid-name fallback; selected Microsoft authentication remains distinct.
- **Restore only requested artwork.** Five original loader SVGs and exact optical offsets are restored. Strict inventory/hash checks pass; provenance distinguishes historical restoration from official-artwork or rights proof. The surrounding UI comparison stays exact, with only these enumerated differences permitted.
- **Reconcile generated contracts.** Export into fresh scratch before replacing the dedicated generated directory; remove obsolete output only after successful export. Read-only equality checking remains separate. No new schema framework or runtime adapter.
- **Correct stale acceptance contracts.** The route manifest now points to the real readiness caller. The create comparison permits only the specifically restored original marks/offsets, not an arbitrary new expected screenshot or layout.

Existing AGENTS rules cover these patterns; compatibility-specific guidance is consolidated into the breaking-scope rule. No anecdotal rule, public rename or decorative architecture layer is added.

## Validation

Independent source/diff review is clear for native Resume, complete launch settlement, current publication recovery, replacement fixtures and absent-selection Offline launch. Full checks pass **759 app /98 API /77 desktop /132 Performance /429 frontend**, with five app and six API ignored helpers and one existing Guardian TODO. Delivery contracts pass73, strict assets pass29 retained files, semantic lint and generated-contract checks pass. Frontend generation `dcb70bac0468` builds and passes generation verification. Logs: `fresh-scope-{app-api-final,desktop-performance,frontend-final,delivery,assets-staged,semantic-lint,wire-check,generation-check}.log`.

The first frontend run had two stale assertions; exact corrected route/create suites pass26 before the full passing rerun. Backend integration exposed three missed fixture constants and a private target literal; corrections use current initial DDL and the existing canonical fixture constructor. Initial compiler refusal for stale frontend authority and a later disk-full LLVM failure are tooling/integration failures, not behavioral RED. Disk space is now available; no cache/profile cleanup was needed. Compatibility-only tests were removed, not counted as retained coverage.

Prior hosted [run37045436994](https://github.com/mateoltd/axial/actions/runs/37045436994) passes exact `0224d940`. The earlier restart fixture's `NoEffect(Busy)` failure remains unexplained; a later pass is not a causal fix.

## Unresolved handoffs

The final-account Offline source gap is corrected and independently reviewed: actual HTTP proves launch/Stop/reopen and unchanged empty account state; delayed probe/pre-spawn checks prove ABA and settings refusal. Native acceptance of the current build remains separate. Separately admitted external-library startup is a confirmed gap: production always opens Managed while switch/Existing consumers are test-only.

On the previous unsigned macOS ARM64 bundle, real Fabric installation, launch, native Stop back to Ready, matching stopped report/terminal acknowledgement and normal Quit exit0 are observed. Java and native processes are gone; protected canaries pass. This is not gameplay, acceptance of current source, or installed-release evidence. Generated profiles/diagnostics remain retained, including the aborted interrupted-Reset fixture.

Native gameplay, fresh-build readiness/latency, interrupted native workflows, four installed architectures and trusted signed-update inputs remain open. Horizon inventory is not Linux acceptance. No deployment, publication or legacy/user-profile mutation occurred.
