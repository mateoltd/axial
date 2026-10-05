# Performance UI handoff

Status: scoped adaptation, isolated behavior checks and the bounded browser mode/inheritance journey below pass. Full Performance and installed desktop parity remain unverified.

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
