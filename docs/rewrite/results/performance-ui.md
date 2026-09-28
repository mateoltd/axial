# Performance UI handoff

Status: scoped adaptation and isolated behavior checks pass. Integrated browser and installed desktop parity remain unverified.

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
