# Performance UI handoff

Status: scoped adaptation, isolated behavior checks and bounded browser/native mode, settlement and reopen checks pass. Native preparation/readiness latency, full Performance and installed desktop parity remain unverified.

## Changes and retained behavior

- `PerformanceSection.tsx` retains the existing settings sheet, Managed/Vanilla/Custom choices, copy, autosave, saving state and failure rollback. Only the Guardian mode and Guardian idle-integrity settings rows and their local state/imports were removed.
- `PerformanceLabProofHistory.tsx` reads the existing backend-authored `view_model.evidence` directly, removing its dependency on the Guardian-named presenter. Neutral evidence, outcome, resource budgets, benchmark details, comparison, stale history and sanitized proof-copy interaction remain. The existing CSS class is retained to avoid presentation drift.
- Performance Lab stays in its current developer disclosure. Matrix, qualification and suite-driver components, routes, controls, interval constraints and backend action availability are unchanged. Instance performance inheritance and `/performance/health` remain unchanged.
- No new controls, styles, screen layout or frontend policy were introduced. Shared API/types, generated output, registrations and manifests were not edited.

The existing `SettingsSection`, `SettingRow`, `ChoicePills`, `Button` and `Pill` primitives are reused. This is contract adaptation on the existing sensitive settings/Performance surfaces. No cards, decorative styling or explanatory UI copy were added. Real interface smoke evidence is still required.

## Checks

`node --test frontend/test/rewrite/performance-ui.test.mjs`: 6 passed, 0 failed on 2026-09-08.

The isolated test loads actual view functions, the autosave hook and Performance DTO decoders into an in-memory module context. Hooks and primitive boundaries are modeled; config decoding is a shared-owner seam. No files are built, no installed profile is accessed and no network or native work runs.

Cases cover:

1. Three mode options, exact autosave patch, saving state and rollback after a rejected save; no Guardian settings controls.
2. Instance inheritance and encoded health query with backend-authored health copy preserved by the actual decoder.
3. Neutral proof evidence, budget, comparison, outcome and stable JSON copy with encoded session identity.
4. Bounded copy-failure feedback without raw service details and retained stale history.
5. Developer-only Lab disclosure with all four existing blocks and original read routes.
6. Driver action visibility controlled by backend booleans, retained progress/history and disabled Start without an instance.

Focused Prettier check and `git diff --check` pass for the edited source. No shared type/build/Cargo commands were run by this owner.

## Integration requirements

- `/root/interface` agreed to retain optional neutral `LaunchProofViewModel.evidence` while removing Guardian fields from the shared launch DTOs. Producer evidence must omit Guardian/autonomous-repair policy rather than asking the UI to filter raw diagnostics.
- Performance plan and benchmark owners were given the exact retained health, matrix, driver, qualification and proof consumer requirements. Their actual routes and persisted producers still need end-to-end verification.
- Shared composition may retire the unused `launch-proof-presenters.ts` after checking other replacement consumers; that helper is outside this package's ownership.
- Full frontend type/build integration, browser interaction/layout comparison and installed WebView proof are not established by these isolated checks. This package must not be marked feature-complete until its real producer/consumer checks pass.

## Actual browser mode inheritance

2026-10-05, unchanged `849976bb` executable/frontend and the generated profile in [current-profile acceptance](current-profile-browser.md). Actual Settings → Performance saves global Vanilla. Instance Settings initially shows Inherit and names Vanilla in its description; selecting Custom saves an Overridden launch profile. Ordinary Launch reaches Playing, and ordinary Stop returns Ready/Idle. Session `c09c60a8-f936-40ae-b458-08b1d8561405` records actual scenario mode `custom`, a stopped report, owner-observed child-exit settlement and terminal acknowledgement.

Actual Reset beside Launch profile clears only that override, showing Inherit/Vanilla. Another ordinary Launch/Playing/Stop/Ready journey creates distinct session `28a08eb3-1fba-47f9-b9d6-0c6a5b8923b2` with actual mode `vanilla` and acknowledged stopped settlement. Both earlier Managed reports and the new Custom report remain exact. Selecting Vanilla does not remove manually installed Sodium; the enabled entry and all3640 recorded game-file hashes reverify unchanged.

Exact API PID45861 ordinarily exits0 on SIGINT. Same executable/profile reopens as PID58517, and actual settings show global Vanilla plus instance Inherit/Vanilla, Ready/Idle and enabled Sodium. All four report proofs and the mode/settings summary match byte-for-byte across reopen; exact screenshot/skin witnesses also match. Actual global Managed is then restored to the fixture's original default; the empty instance override and all historical reports remain intact. No current setting is substituted for a report's recorded mode.

The bounded read-only `current-browser-performance.mjs` checks persisted settings, concrete report mode/hashes, preservation of earlier reports, zero Content/Performance/unfinished-queue/pending-or-unacknowledged-launch obligations and SQLite quick-check. The existing `current-browser-launch-proof.mjs` separately validates every accepted intent/report/settlement join after each Stop and restart; the mode helper is not a second settlement codec. Logs are `current-browser-performance-{custom-setting,custom,inherit-setting,inherit,reopen,restored}.log`, their launch-proof companions and inventory/screenshot/skin proofs. Independent review is clear after adding pending-intent coverage and preserving the new Custom report across later phases. Live screenshot/AX observations establish the rendered controls; no saved screenshot path is asserted. This closes this bounded settings/effective-launch/reopen journey, not measured optimization, explicit bundle apply/remove/rollback, native gameplay, all mode controls or full Performance parity.

## Actual native mode inheritance

2026-10-05, working source `ec75ec3d`, using the unchanged `4ba166c6` ARM64 no-sign debug artifact from [native Content acceptance](pack-files.md#current-native-add-update-disable-and-reopen). Its executable SHA256 remains `2ea433c919de70ddc7c1f2bda4a7296dd34db08d37fe6957fb119ea20c038bbd`. The exact generated offline profile is `/private/tmp/axial-native-gameplay.E56Jhx/profile`; target `6798b0fc-ff87-44bf-b7cd-b3adf6442662` is GameplayFabricParity, Fabric0.19.5/Minecraft1.20.1. No human credentials, signing key or legacy profile is accessed.

Before UI mutations, `native-performance-proof.mjs capture` retains full and explicitly protected settings/instance projections, four prior intent/report digests, protected metadata and exact disabled Sodium manifest/payload witnesses in `native-performance-ec75-before.json`. Actual global Settings → Performance saves Vanilla; instance Settings shows Inherit/Vanilla, then saves Custom. Ordinary Launch eventually reaches Playing; ordinary Stop reaps observed child52566. Session `2b771753-46da-4acf-b5d3-e6699c393105` records `custom_launch`/`custom`, boot7746ms and an acknowledged stopped outcome.

Actual Reset beside Launch profile clears only the override, showing Inherit/Vanilla. After normal Instances/back navigation shows Ready, ordinary Launch reaches Playing and Stop reaps child61725. Distinct session `eaeca860-8f69-41a2-b213-20ac1da8c288` records `vanilla_launch`/`vanilla`, boot8601ms and an acknowledged stopped outcome. Both process settlements require actual child exit, Stop requested, boot observed, no spawn failure and matching report/accepted-payload binding. Current settings never substitute for historical modes.

Actual Quit Content from the application menu exits nativePID36745/session52922 with0; no password input is required. A preceding Cmd-Q attempt has no observed effect and is not credited as Quit. Explicit same-binary/profile reopening as PID64412/session97542 restores Ready, account label GameplayParity, disabled Sodium and the rendered Inherit/Vanilla choices. Full settings, registered instance, six intent/report records and both new session summaries match the pre-Quit witness exactly. Actual global Managed is restored to the captured original, retaining the empty instance override and all history; a second menu Quit exits0, with both launcher PIDs absent.

The bounded collector allows only mode, revision, navigation and launch-recency changes. Protected account/selection, installed inventory, queue/skin/creation metadata, marker and exact Content bytes/identities/timestamps remain unchanged. Content, Performance and unfinished installation/creation obligations are absent; all six launch intents have terminal acknowledgement and SQLite quick-check is OK. This is not an entire-profile inventory or actual world/save proof. The collector delegates new sessions only to the existing stopped-launch verifier; independent review strengthens that verifier with exact intent-key binding and canonical ordered timestamps. Collector SHA256 is `33da4425f4a3be3f3c53dff516afd432d7685bc57b3087f86e4b3a9492e2591e`, shared verifier `643ffe968e1d66753ffbd82c04c0a1cf0b0e55d1982c755e9cf15495031c36f6`. Retained witnesses are `native-performance-ec75-{before,custom-settled,inherit,quit,reopened,restored,restored-quit}.json` and their error logs; successful phases exit0 with empty errors. No shared build or production source changes occur in this journey.

Preparation/readiness is not a clean pass: the first Launch remains Requesting launch with one pending intent and no observed direct child across repeated observations; `native-performance-ec75-custom-errors.log` records an actual failed settlement check. It later accepts the same intent without retry. An idle process sample is retained in `native-performance-pending-sample.txt`. After Custom Stop, Unavailable persists through the saved Reset and changes to Ready following normal navigation. These observations do not identify a cause or prove a fix. Session timestamps/boot durations start after preparation and cannot retrospectively measure that wait. The second pre-click clock is21:41:58Z and its accepted session starts21:42:31.633Z; this is not a controlled latency measurement. Independent source diagnosis narrows candidate waits to shared filesystem admission, installed/runtime/native verification and the rules gate; no guessed lock, deadline or safety-policy change follows.

This closes the recorded mode/history/preservation/reopen checks, not controlled launch latency, managed health, explicit bundle apply/remove/rollback, benchmarks, game-window interaction, authenticated continuity, installed release or full Performance parity. No `/performance/health` response was captured; absence of a banner is not health evidence.
