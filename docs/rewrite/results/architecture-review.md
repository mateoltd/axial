# Architecture review

Updated 2026-10-06. Scope: recorded managed-file removal recovery after `f0f1bb01` and the locally installed ARM64 Vanilla control at `318e7ede` on `main`. This is not a release or full-parity certificate.

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

Prior [exact recorded-metadata recovery](forge-loader.md#exact-recorded-metadata-recovery), [game-directory fix](forge-loader.md#legacy-game-directory-binding) and [unlocked paint control](native-auth.md#fresh-unlocked-paint-control) retain their bounded evidence. Their runtime artifacts predate this slice; no new installed or visual acceptance is inherited.

[Current-source installed Vanilla](native-auth.md#current-source-installed-vanilla-journey) independently binds its actual app/DMG and fresh profile. Native onboarding/install, Playing/output, Busy Quit refusal, acknowledged Stop and ordinary Quit/reopen pass locally. A fresh post-game logical database and all24 saved-world file hashes remain exact. Menu/world entry/save are human-reported; the launcher lists the saved world. A second reload/settlement is pending. No product abstraction, UI or speculative visibility/authentication workaround is introduced. The existing evidence rules suffice.

## Unresolved handoffs

- [Accounts/startup](native-auth.md): the earlier Invalid session and paint failures remain undiagnosed despite later passing controls; Java-window access and authenticated distinct-build continuity remain open. No further password or permission experiment is required by this review.
- [Forge](forge-loader.md): other retained-era recorded-byte/failure matrices and gameplay remain open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md) and [library lifecycle](library-lifecycle.md) retain their documented qualification/recovery limits. Passing retries do not diagnose historical Busy, no-child, timeout or hosted failures.
- [Integration](integration.md) owns native switching, playable world/save, installed-platform and trusted update/restart gates. No deployment, publication, signing or credential-permission change follows.

## Prior reviews

Detailed chronology remains in the linked feature reports and Git history: `git show f0f1bb01:docs/rewrite/results/architecture-review.md`. Read-contract/codec corrections remain in [wire parity](wire-parity-review.md) and [benchmark persistence](current-benchmarks.md).

The full-parity goal remains active; this review neither replaces nor resets it.
