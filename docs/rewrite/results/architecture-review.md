# Architecture review

Updated 2026-10-07. Changed scope: benchmark continuation and Intel installed-package acceptance at unchanged `5d06ec9b` on `main`, following the recorded Linux packaging correction. This is not a release or full-parity certificate.

## Ownership

Root owns integration, evidence, profiles and serialized builds. Independent Standards/Spec reviewers inspect frozen source and evidence without operating profiles or builds. Preserve non-Guardian behavior, current UI, filesystem authority and current-app recovery; predecessor compatibility remains excluded.

## Findings and fixes

- Correct the observed EGL/media packaging boundary through the released CLI2.12.0, matching existing toolchain/CI pins, the existing Linux media overlay and explicit base/good GStreamer build prerequisites. Runtime crates, product Rust, frontend generation and UI remain unchanged.
- No shipped preload/backend override, private bundler, codec implementation, new state owner, wrapper extension or speculative abstraction. Diagnostic controls stay private. Released tooling also produces a relative working icon link and0755 wrapper; do not manually patch the package.
- No new redundant naming/layer or recurring pattern warrants another simplification or AGENTS.md rule. Existing ownership, real-boundary testing and evidence rules cover this slice.
- Continue the original two-run suite through its existing Tick/intent/session/report owners, without a replacement driver or journal. Correct only the private witness's normalized-path sorting and require exact accepted/current bindings; do not refresh changed evidence or reinterpret labels as settlement.

## Validation

Earlier Linux source reviews: Standards has no documented violations/actionable smells; Spec has no source-scope/implementation findings. Its focused delivery/dependency/generation selection passes60/0 in0.97s. Current additions are private fixtures and acceptance documentation; independent evidence review corrects two overbroad summary phrases. Whitespace checks pass; installed acceptance remains incomplete.

[Benchmark continuation](current-benchmarks.md#completing-the-retained-two-run-suite) preserves all prior histories and protected files, plus the original intents, through real boot/Stop/completion/cold reopen/final normal exit. Independent helper/evidence review is clear; its initial refused fixture comparison is retained. [Intel installed evidence](native-auth.md#installed-intel-package-under-rosetta) uses the released pinned CLI, existing macOS overlay and a fresh isolated copy/profile, with no source, signing-policy, UI or extra runtime path. Logical preservation is not whole-profile identity; Rosetta is not physical Intel, and boot is not gameplay.

[Linux evidence](linux-package.md#released-packager-and-media-correction) binds the joined0 media build, unchanged raw executable, verified generation and all1345 staged/extracted payload entries. Ordinary no-override startup no longer shows the original EGL abort or missing media elements in its bounded control. Matched tools load the actual extracted plugins on Fedora and decode the retained sound sprite to a fakesink. Diagnostic termination143 is not normal Quit; decoding is not audible playback. No other-platform, credential, trust or full-parity claim follows.

Original EGL/DMABUF failures, interrupted build-exit uncertainty and post-reboot renderer-only Wayland error71 remain recorded. The private fixture now captures the current desktop environment; that correction does not diagnose the protocol error. Final media startup still emits Mesa/GTK warnings, with no graphics-policy or host-driver change. Existing Horizon RustDesk reconnect fails; no unrelated viewer/security configuration is changed.

## Unresolved handoffs

- Linux visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen; diagnose retained graphics/protocol observations without assuming process survival proves rendering. Other architectures and trusted installed update/restart remain release gates.
- [Accounts](native-auth.md): current noninteractive adapter needs distinct-build verification under an authorized stable signing identity. Earlier signed test binaries predate that adapter; no new password or permission experiment follows from their existence.
- [Forge](forge-loader.md), [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md) and [library lifecycle](library-lifecycle.md) retain their documented recovery/version/runtime limits. Passing controls do not diagnose historical failures.
- [Integration](integration.md) owns remaining full-parity acceptance, including actual saved-world reload. No deployment, publication, signing-policy or credential-permission change follows.

## Prior scope

[Recorded managed-file removal](pack-files.md#recorded-managed-file-removal-recovery), [installed Vanilla](native-auth.md#current-source-installed-vanilla-journey) and [effect-intent recovery](performance-ui.md#effect-intent-remove-preserves-the-bundle) retain their bounded evidence. Earlier details remain in those feature reports and `git show 93ae02b0:docs/rewrite/results/architecture-review.md`; they are not acceptance for this package.

The full-parity goal has resumed and remains active. This review does not replace or narrow it.
