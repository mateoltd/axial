# Architecture review

Updated 2026-10-05. Changed-scope review, not a parity or release certificate. Checkpoint-specific history stays in [integration evidence](integration.md), feature reports and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, delivery, work ownership and current evidence. Development remains on `main`, preserving history. Full non-Guardian parity remains required; predecessor-profile import/schema upgrades are excluded, not current-app persistence, accepted-operation recovery or supported Minecraft/loaders.

Root owns integration, shared verification, diagnostics and this record. The current integrated scope is recorded create-only Content recovery; the native owner delivered its test-fixture consolidation. No UI redesign, stable public-contract rename or new recovery coordinator is authorized by this scope.

## Findings and fixes

- Recorded create-only publication extends the existing bounded checkpoint and receipt CAS with complete indexed post-move proofs. Immutable original proofs remain intact. Cold recovery validates the entire public/private topology before exact guarded cleanup; persistence refusal retains compensation and its originating error. Reusing Ready preparation removes a hidden caller ordering requirement. No second journal, codec owner or wire/UI change.
- The existing live-cleanup fixture injected its foreign canary before the newly required proof inventory, triggering an earlier refusal instead of its intended cleanup boundary. Move only noncancel injection to the existing pre-manifest hook after persisted proof/public moves; retain original-task lifetime, cleanup, witnesses and deadlines.
- Fixed-range review (`657b8254...553ddc05`) found no Spec finding or Standards blocker. Its optional duplicate-fixture finding is addressed by one existing helper accepting the already-used replacement mode:18 fewer lines, unchanged production and assertions.
- This review record had accumulated historical checkpoint prose already owned by feature reports. Keep only current findings, validation and handoffs here; historical evidence remains linked and recoverable in Git. No new AGENTS.md anecdote is warranted.
- Native packaging retained stale inline configuration while its metadata reflected the corrected override. Track the procedural macro's existing environment input beside its single expansion; two lines, unchanged identity guard and no new configuration owner. Actual override-only refusal/startup controls and all82 desktop tests pass. No broader cache-mechanism claim or new architecture rule is warranted; [native gameplay evidence](native-gameplay.md) records the regression and isolated fresh-profile journey.

## Validation and limits

Actual API/native/direct-caller REDs precede the create-only correction. Focused application/native/API checks and all48 native transaction cases pass, including actual publication/cleanup exits42/43, repeated private-absent recovery, eight foreign-object controls, complete codec mappings and callback refusal/drift. After fixture consolidation, all48 pass again (`content-created-fixture-consolidation-native.log`). See [pack evidence](pack-files.md#recorded-create-only-publication-recovery).

The corrected broad command passes114 API/eight ignores and786 app/five ignores but fails nine Java-probe cases (`content-created-core-final.log`). It stops before the other four suites. Both corrected live-cleanup cases pass within that command. Broader green is not claimed.

A four-test cached-binary group reproduces the Java timeout. Parent polling stays responsive; sampled startup stacks are not correlated to a failed probe. Releasing regenerable caches does not eliminate the failure. The existing test-only diagnostic now captures the owned child PID before polling can reap it; independent review finds no privacy or lifecycle issue. Its intentional-timeout check passes after removing the temporary debug tag (`java-probe-pid-observation-final.log`). No deadline or production behavior changes. The original executable/hash and RED logs remain retained.

Separate complete application and Minecraft runs pass795/five ignores and1001/no ignores (`java-probe-pid-full-app-1.log`, `content-created-minecraft-standalone-full.log`). The application run has only the intentional51ms timeout, not a correlated3s failure. These standalone feature configurations do not certify the original combined command or resolve its intermittent failures. Horizon currently refuses SSH; no new Linux result is inferred.

The preceding Content checkpoint passes46 focused Horizon/Linux checks and both hosted jobs at exact `b5f6ab46` ([run37301035959](https://github.com/mateoltd/axial/actions/runs/37301035959)). Those results do not validate the later extension. Source tests do not establish native UI, authentication, gameplay or installed-update acceptance.

Both hosted jobs pass at exact `cab10ae6` ([run37322335571](https://github.com/mateoltd/axial/actions/runs/37322335571)), including the selected six-library configuration, retained interface and delivery contracts. These Linux jobs do not compile the native desktop; the context correction instead has its recorded macOS82-test and actual startup controls. Hosted green does not diagnose earlier intermittent failures or certify installed/native parity.

## Unresolved handoffs

- Root: complete broader verification and fresh-profile native install/content. Recorded create-only recovery does not waive replacement recovery, unrecorded publication intervals, unsupported/overbudget proofs or unknown filesystem effects. Preserve exact authority and current-app recovery obligations.
- Diagnosis: explain the current Java-probe failures and the separately recorded historical Linux Busy/hosted abort failures. Passing reruns, stale lease names, parent cadence or low disk alone are not causes or fixes; [integration evidence](integration.md) and [pack evidence](pack-files.md) retain their distinct signatures.
- Accounts: the fresh human-led native retry succeeds; selected online readiness, acknowledged publication, normal Quit/reopen and non-mutating profile sync now have bounded evidence in the unchanged diagnostic account owners. The earlier Request failure remains unexplained; passing retry is not a fix. Token rotation/expiry, logout, skin/cape actions, cookie isolation and installed-release acceptance remain open. Never inspect/replay credentials, callbacks, provider bodies or keyring values; [native sign-in](native-auth.md#authenticated-success-and-reopen) owns evidence.
- Native acceptance: actual game-window/world/save interaction, folder-manager observation, Dock/OS/startup-modal exit and controlled latency remain open. Existing/legacy World Import opens the saves folder; do not invent archive-import scope to replace that observation.
- Delivery: four installed artifact architectures and trusted signed-update/restart evidence remain missing. Compile/package checks cannot replace them. No deployment, release publication or legacy/user-profile mutation is authorized.

The full-parity goal remains active. Architecture review automation remains paused; this record neither replaces nor resets the goal.
