# Architecture review

Updated 2026-10-06. Changed scope: the three pending Linux packaging/toolchain edits over `3757c6d1` on `main`, including the interrupted diagnostic build. Earlier feature evidence remains linked below. This is not a release or full-parity certificate.

## Ownership

Root owns integration, evidence, profiles and serialized shared verification. Separate native and application-fixture owners edit their exclusive files; independent Standards/Spec reviewers inspect frozen source and controls without operating profiles or builds. Preserve non-Guardian behavior, current UI, filesystem authority and current-app recovery. Predecessor imports/schema upgrades remain excluded.

## Findings and fixes

- The existing checkpoint records Downloads but omits ordinary managed-file Remove after its actual backup move. Extend that same bounded codec/rollback owner to complete removal proofs, rather than introducing another recovery path.
- A download requires its index/proof pair; a removal requires neither and an exact original backup. Require complete distinct mutation/payload coverage. Schema1 download encoding, public contracts and already-absent no-op commits remain unchanged.
- Removal destinations remain fenced by absence; changed or replaced backups and new public files refuse before restoration. Retained guarded retries and restored-proof acknowledgement still precede cleanup. Unsaved ambiguity remains preserved.
- `PublishedChange` names the broadened private responsibility. Shared subprocess fixtures avoid duplication. No journal, schema, coordinator, application production path, UI change or speculative credential fix. Existing exact-proof/owner rules cover the finding; AGENTS.md needs no added rule.

## Validation

The public-owner removal crash/Resume test has a meaningful Pending RED after safe teardown, then GREEN1/0.34s with complete proof and another reopen. Existing replacement/unsaved control passes1/0.85s. Native publication controls pass16/2.71s and all62 transaction checks pass40.62s. All825 app checks pass with eight threads/eight ignores in66.55s. Independent Standards/Spec reviews and scoped formatting/whitespace checks are clear.

[Content evidence](pack-files.md#recorded-managed-file-removal-recovery) owns commands, hashes and qualifications. The default-parallel app run fails the existing two-second preflight marker rendezvous despite eventual Ready; isolated GREEN does not diagnose it. No timeout change follows. Mixed removal tests cover cold recovery plus live post-move retry and cold restored-proof replay, not every crash boundary or installed-process removal.

One later default-parallel app control passes825/eight ignores in71.12s with temporary failure-only timing. No failure recurs, so marker production versus observation delay remains undiagnosed. The diagnostic patch is retained privately and removed from tracked source; no speculative hook, deadline increase or permanent test churn is introduced.

Prior [exact recorded-metadata recovery](forge-loader.md#exact-recorded-metadata-recovery), [game-directory fix](forge-loader.md#legacy-game-directory-binding) and [unlocked paint control](native-auth.md#fresh-unlocked-paint-control) retain their bounded evidence. Their runtime artifacts predate this slice; no new installed or visual acceptance is inherited.

[Current-source installed Vanilla](native-auth.md#current-source-installed-vanilla-journey) independently binds its actual app/DMG and fresh profile. Native onboarding/install, Playing/output, Busy Quit refusal, acknowledged Stops and ordinary Quit/reopen pass locally. A fresh post-game logical database and all24 saved-world file hashes remain exact; native backup produces a byte-identical real-world copy. Menu/world entry/save are human-reported; the launcher lists the saved world. The unanswered second gameplay-reload handoff is retired without acceptance; all owned native/game processes are closed. No product abstraction, UI or speculative visibility/authentication workaround is introduced. The existing evidence rules suffice.

[Performance effect-intent recovery](performance-ui.md#effect-intent-remove-preserves-the-bundle) uses the existing checkpoint and recovery owner, with no production change or duplicate test. Actual diagnostic exit43 precedes file removal; ordinary recovery fails the same command and preserves the complete nonempty bundle/internal tree through Quit/reopen. The bounded witness's restoration-only directory link-count premise is corrected to the exact observed transition; every later file proof remains exact. Independent Standards/Spec reviews are clear. Existing filesystem-fixture rules suffice; no AGENTS.md addition is needed.

[Linux package inspection](linux-package.md) uses the existing target lease, ordinary bundle configuration and unchanged frontend verifier. Transfer-only AppleDouble correction introduces no product abstraction, verifier waiver or UI change. Real optimized build joins0 and404 extracted payload entries match staged bytes/types/modes/links; expected marker/RUNPATH transformations remain separately qualified. Exact container stop follows child/copy joins and retains inputs/evidence; its idle PID1 exit137 is not application settlement. Independent Standards, Spec and architecture reviews are clear. Existing fixture/evidence rules suffice, with no AGENTS.md addition. No installation, credential, trust or runtime workaround follows.

The pending remedy stays at the packaging boundary: matching CLI2.12.0 pins in `toolchain.json`/release CI and media-framework inclusion in the existing Linux overlay. Runtime crates, application owners, frontend and UI remain unchanged; no host-library preload, rendering override, wrapper extension or private bundler fork is added. The original renderer abort, failed DMABUF control and diagnostic host-Wayland control remain failures/qualified observations, not installed acceptance. No redundant namespace, extra layer or recurring new pattern warrants a source simplification or AGENTS.md rule.

The retained focused contract run passes60/0 in1.23s; whitespace checks pass. The newer-bundler-only build log reports one finished package, but interruption lost handle55943, so its final command exit is uncertified. Read-only inspection finds only container PID1, an unchanged raw executable (`aa604c3b…fc4ccfe`) and candidate `b4951892…209956`; the pending media overlay was not part of that build. No rebundled runtime, media or other-platform acceptance is inherited. Root retains both old/new packages and diagnostic profiles; [Linux evidence](linux-package.md) owns the detailed provenance. Separate Standards/Spec source and qualification reviews are clear; runtime evidence remains a handoff.

## Unresolved handoffs

- [Accounts/startup](native-auth.md): the earlier Invalid session and paint failures remain undiagnosed despite later passing controls; Java-window access and authenticated distinct-build continuity remain open. No further password or permission experiment is required by this review.
- [Forge](forge-loader.md): other retained-era recorded-byte/failure matrices and gameplay remain open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md) and [library lifecycle](library-lifecycle.md) retain their documented qualification/recovery limits. Passing retries do not diagnose historical Busy, no-child, timeout or hosted failures.
- [Integration](integration.md) owns native switching, playable world/save, installed-platform and trusted update/restart gates. No deployment, publication, signing or credential-permission change follows.
- [Linux package](linux-package.md) owns the pending upstream-bundler renderer retest, media-enabled package/playback verification, absolute icon-link and wrapped-executable permission handoffs. Confirm actual new payload/helper/source hashes and ordinary no-override behavior before integrating the remedy. Payload equality and host session presence do not prove an ordinary native launch or portable runtime.

## Prior reviews

Detailed chronology remains in the linked feature reports and Git history: `git show f0f1bb01:docs/rewrite/results/architecture-review.md`. Read-contract/codec corrections remain in [wire parity](wire-parity-review.md) and [benchmark persistence](current-benchmarks.md).

The full-parity goal is currently paused according to its authoritative status. This scheduled review neither resumes nor replaces it; completion still requires full non-Guardian parity.
