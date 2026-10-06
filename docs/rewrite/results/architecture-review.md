# Architecture review

Updated 2026-10-06. Scope: exact recorded-metadata recovery changes after `064e4ab2` on `main`. This is not a release or full-parity certificate.

## Ownership

Root owns queue/export integration, evidence and serialized shared verification. The reconstruction owner supplies leaf changes; independent Standards/Spec reviewers inspect frozen source and controls without operating profiles or builds. Preserve non-Guardian behavior, current UI, filesystem authority and current-app recovery. Predecessor imports/schema upgrades remain excluded.

## Findings and fixes

- Canonical serialization fixes future materialization, not already recorded arbitrary-order JSON. The existing committed lease or registered inventory supplies bounded exact bytes through an opaque immutable input. Authenticated semantic derivation and the complete recorded activation contract still decide acceptance. No guessed permutations, digest waiver, new lease, journal, schema or UI change.
- Independent review catches ACK-before-Ready: native acknowledgement clears the marker while the activation row remains `activating`. The queue now admits `activating` and `ready` only under the same library/version/contract/install identity. This supplies reconstruction input without granting readiness.
- Missing committed metadata refuses before reconstruction and retains the same evidence, native fence and exclusion. Native controls preserve foreign bytes, reject changed members and replaced markers, then restore/acknowledge before assertions. Registered reconstruction preserves full inventory and retained raw metadata; a wrong full contract is refused.
- Changes remain feature-owned. Unrelated formatting is restored; a local bounded-read closure removes repeated test boilerplate. Existing owner-composition and recorded-contract rules already cover the finding, so AGENTS.md needs no additional rule.

## Validation

The historical-order regression and ACK-before-Ready test have meaningful REDs after safe settlement/teardown. Focused GREENs pass; all33 queue checks pass in18.37s and all1020 Minecraft library checks pass in239.28s. Consumers pass824 app/eight ignored in204.03s,118 API/eight ignored in91.49s and82 desktop/one ignored in6.16s; serialized wrappers0. Native and registered controls pass1/0.30s and1/0.77s. Independent source/control Standards and Spec reviews are clear. Changed-region formatting and whitespace checks pass; whole-file formatting still flags unrelated existing regions.

[Forge evidence](forge-loader.md#exact-recorded-metadata-recovery) owns commands, hashes, controls and qualifications. The correction covers earliest-Forge raw-byte reconstruction, not the public fixed-provider rebuild entrypoint, a complete cancellation/process-exit matrix or other-era serialization.

The earlier [game-directory fix](forge-loader.md#legacy-game-directory-binding) has real1.4.7 boot/Stop/reopen evidence with exact preservation and joined exits. That frozen runtime predates this recovery slice; no new native or installed acceptance is claimed.

## Unresolved handoffs

- [Accounts/startup](native-auth.md): narrator Continue → Invalid session, normal Startup versus Inspector paint, Java-window access and authenticated distinct-build continuity remain open. No further password or permission experiment is required by this review.
- [Forge](forge-loader.md): other retained-era recorded-byte/failure matrices and gameplay remain open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md) and [library lifecycle](library-lifecycle.md) retain their documented qualification/recovery limits. Passing retries do not diagnose historical Busy, no-child, timeout or hosted failures.
- [Integration](integration.md) owns native switching, playable world/save, installed-platform and trusted update/restart gates. No deployment, publication, signing or credential-permission change follows.

## Prior reviews

Detailed chronology remains in the linked feature reports and Git history: `git show 064e4ab2:docs/rewrite/results/architecture-review.md`. Read-contract/codec corrections remain in [wire parity](wire-parity-review.md) and [benchmark persistence](current-benchmarks.md).

The full-parity goal remains active; this review neither replaces nor resets it.
