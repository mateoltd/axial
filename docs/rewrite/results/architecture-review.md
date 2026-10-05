# Architecture review

Updated 2026-10-05. Changed-scope review from `a3dcca86` on `main`, including the current working tree. This is not a parity or release certificate. Detailed history belongs in [integration evidence](integration.md), [Content evidence](pack-files.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, delivery, work ownership and current integration evidence. Full non-Guardian parity and the existing UI remain required. Predecessor-profile import/upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

Root owns shared verification, filesystem adapter, loader fixture cleanup and documentation. Native transaction and Content files are frozen after separate ownership and read-only review. Scope is complete recorded replacement restoration through live compensation, retained effects and cold Resume. No new journal, coordinator, configuration, wire/UI contract, folder layer or stable-contract rename.

## Findings and corrections

- Restoration extends the existing bounded checkpoint and exact receipt CAS. It reserves the complete future restored proof before effects, preserves immutable original/publication evidence, freezes fresh restoration evidence once and requires durable acknowledgement before private cleanup. Optional omission is not acknowledgement.
- A valid near-limit receipt could prevent proof-free cleanup from running. Pass zero proof capacity to the existing native reconciliation on Capacity; ordinary cleanup proceeds, but required restoration still refuses without room for its proof. The actual public-mutation child reproduced the stall after its cleanup canary was removed.
- Cold retries lost progress when guarded removal or restoration applied before returning an error. The existing filesystem boundary now distinguishes definite applied removal from NoEffect and Indeterminate, preserving original errors for established callers. Retained progress is classified before inventory: settle definite removal and verify exact absence; recognize a moved original only through its exact retained revision. Unrefreshed/indeterminate outcomes remain preserving refusals.
- The live retry classified an already removed replacement through its retired guard, yielding Unknown even when the exact original had been restored. Reuse known successful removal progress, require stage absence and exact prior destination, then release the retired guard. This does not authorize Unknown.
- That liveness correction exposed a second defect: live reprojection could offer a new restored proof after an equal-byte rewrite with restored mtime. Enrolled restoration now authenticates existing original, manifest and payload revisions, refusing fresh guard/body-only adoption. Real native witness changes and repeated preserving refusals are tested. AGENTS.md refines its existing ownership rule to retain effect classifications and exact revisions across retries; no anecdotal rule block is added.
- One concrete crash-owner setup serves seven existing fixtures; repeated witness mutation uses one concrete test helper. Remove the always-Some settlement return in favor of Self/map. The DELETE-refusal fixture asserts exact Busy while the library is open. Pending Content correctly refuses installed reads; settled reopens still require v1. No production read policy is weakened.
- Retire the obsolete live-network negative loader fixture: Forge/NeoForge installer reconstruction is implemented, any network error satisfied its assertion, and its sentinel directory was never passed to reconstruction. Existing deterministic success and outputless-NeoForge InvalidProfile/inventory/request-boundary checks remain. No production loader or timeout changes; the existing fixture-boundary rule already covers this drift.
- Apply the user's purpose-first naming rule to the new private FileRemovalFailure type, omitting its redundant managed/guarded namespace prefixes. Stable contracts remain unchanged. Fold that rule into the existing naming bullet, not another guidance section.

## Validation

Real native/Content RED/GREENs cover admission, public Resume, post-effect retries, receipt capacity, unchanged live restoration and equal-byte/restored-mtime drift. The DELETE-refusal/three-open control passes after correcting only its test expectation: pending installed reads correctly refuse Unavailable. [Content evidence](pack-files.md#recorded-replacement-restoration) owns detailed phases and logs.

The first broad command fails `receipt_binds_the_requested_alias_and_symlink_target` with ProbeTimedOut at its first probe. Twenty exact and ten six-test group reruns do not reproduce it. The unfiltered control passes probes and deterministic loader controls but remains in the obsolete network fixture; root verifies exact PID/parent/executable and no observed direct children, terminates only that disposable test after8m22s, and preserves the log. Exit143 is not a suite pass. No probe or deadline changes.

After fixture retirement, the serialized six-suite command exits0:114 API/eight ignores,800 app/seven ignores,213 filesystem,1018 Minecraft,132 Performance and8 Resource (`replacement-restoration-core-verified.log`), including all49 Content/58 native transaction cases. Separate desktop82/one ignore checks pass. After private naming/import cleanup, the same six-suite selection compiles successfully and freshly rebuilt binaries pass12 publication, three live-restoration and one public replacement controls (`replacement-restoration-final-{compile,native-controls,live-controls,public-control}.log`). Scoped formatting and whitespace checks pass. None of these passes diagnoses the earlier timeout.

Independent final Standards and Spec review have no remaining findings; the previous optional settlement simplification is resolved. Shared Cargo/build checks remain serialized. Earlier hosted success at exact `a3dcca86` does not certify this working tree. [Signed offline acceptance](native-gameplay.md#signed-current-source-launch-and-reopen) remains bounded to its recorded artifact, not this later restoration source.

## Unresolved handoffs

- Recovery/native owner: fresh native Content acceptance remains open. Recorded complete publication does not recover rename-before-proof-CAS, arbitrary partial/unrecorded intervals, unavailable/overbudget proofs or unknown effects. Preserve the earlier failed/terminated evidence; full parity remains open.
- Diagnosis: the current first-receipt Minecraft timeout and older application Java-probe, Linux Busy, hosted abort and benchmark no-child failures remain distinct and unexplained. No correlated owned-child observation establishes a cause. Passing reruns are not diagnoses; [integration](integration.md) and [Content evidence](pack-files.md) retain their signatures.
- Accounts: human Keychain permission reuse and prompt-free authenticated reopening remain unverified; the earlier Microsoft Request failure is unexplained. Signed synthetic persistence and fewer new secure-store reads are not human-account acceptance. Rotation, logout, skin/cape, cookie isolation and installed-release checks remain open. [Account evidence](native-auth.md) owns the boundaries; do not inspect or replay human secrets.
- Native/delivery: actual game-window/world/save interaction, controlled latency, other native exit/folder journeys, four installed artifact architectures and trusted signed-update/restart evidence remain open. No deployment, release publication or legacy/user-profile mutation is authorized.

The full-parity goal remains active; this review neither replaces nor resets it.
