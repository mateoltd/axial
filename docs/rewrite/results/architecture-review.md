# Architecture review

Updated 2026-10-07. Scheduled review covers frozen client-file preflight changes against `8edd5bcf` on `main`, following the integrated successful diagnostics/browser convergence slice. Full non-Guardian parity remains active; this is not an installed-release certificate.

## Ownership and scope

Root owns shared exports/generated integration, evidence and serialized verification. The preflight owner relinquished the frozen source before root's validator correction; independent Standards and Spec reviews cover that final correction. Review excludes legacy profiles, user installations, signing, credentials and deployment. Preserve the existing UI, filesystem authority and current-app recovery.

## Findings and fixes

- Keep client missing/corrupt causes at the existing admitted inventory verifier. Require actual absence or measured mismatch; unsafe/read failures remain generic. No new scanner, readiness framework or persisted state.
- Review reproduces malformed recorded SHA1 being reported as corrupt bytes. Decode and validate the existing canonical digest before observation, then compare typed digest bytes. This reduces duplicated interpretation without weakening admission, revision or hash guards.
- Share diagnostic construction/resource sampling between real successful and negative callers. Keep captured account/settings checks, bundle revalidation and original final target/exclusion/settings fences. Capture failure must preserve the original installation refusal; bulk does not acquire unused diagnostics.
- Generate implemented reason contracts at their Rust owner rather than maintaining frontend unions. Global generated filenames are not redundant feature-owned wrappers; no stable contract rename or folder churn is justified.
- Distinct install, launch and HTTP error projections retain their own semantics; a common mapping layer would obscure ownership. Existing typed-owner rules already cover the evidenced checksum mistake; no additional AGENTS.md rule is warranted.

## Validation

Detailed RED/GREEN, source hashes and final wire checks stay in [client-file evidence](wire-parity-review.md#observed-client-file-readiness). Missing/corrupt HTTP and malformed-record public-read regressions pass after meaningful failures. Final independent Standards and Spec reviews report no actionable production findings. Export/equality, project-edition formatting,90 selected install checks, all ten preflight guards across prefix/exact selections and final HTTP recheck pass. Subsequent frozen-source libraries pass830 app/122 API, zero failures/ten and eight existing helper ignores, nested child result excluded; desktop composition passes83/one existing helper ignore. Pinned Node24.13.1 checks pass531/zero failures/one TODO; build remains `239a78614d6e` and independently verifies unchanged budgets. Semantic/asset checks pass. Wrappers join0. No native/browser process-reopen acceptance is inferred.

The integrated predecessor's [hosted run37561256950](https://github.com/mateoltd/axial/actions/runs/37561256950) passes both jobs at exact `8edd5bcf`, not this working diff. Its detailed531 frontend/829 app/121 API/83 desktop checks remain [prior launch evidence](wire-parity-review.md#launch-diagnostics-and-terminal-convergence). The private overlapping watch probe passes after correcting its copy inventory, but does not reproduce or diagnose the historical timeout; no shipped debug code, deadline or guard change.

## Unresolved handoffs

- Launch: other artifact/runtime/early-refusal facts and actual retained-browser API-reopen acceptance remain pending. The runtime audit identifies spawn failure being confused with observed executable absence; verify/correct that owner before publishing override-missing facts. Source fixtures do not establish installed convergence or gameplay.
- [Updates](updates.md): channel policy, snapshot hydration and deferred intent have source/fixture evidence, not trusted installed-update acceptance. Keep asset-watch diagnosis open.
- [Accounts](native-auth.md): verify the current noninteractive credential adapter across distinct builds under the intended authorized stable signing identity. Earlier signed test binaries predate that adapter.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen remain open. Process survival and fakesink decoding do not prove them; retain graphics/protocol failures.
- [Forge](forge-loader.md), [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md), [world files](world-files.md) and [library lifecycle](library-lifecycle.md) retain their detailed recovery/version/runtime limits. Passing controls do not diagnose historical failures.
- [Integration](integration.md) owns full-parity acceptance, including actual saved-world reload and the installed-platform matrix. This review does not replace or reset that goal.

## Earlier evidence

Detailed evidence stays with its feature owner: [telemetry](telemetry.md), [capacity and benchmark continuation](current-benchmarks.md), [World backup](world-files.md), [managed historical rollback](performance-ui.md), [retained wire audit](wire-parity-review.md), [native acceptance](native-auth.md) and [released Linux packaging](linux-package.md). Prior successful-diagnostic and browser-convergence review remains recoverable in `git show 8edd5bcf:docs/rewrite/results/architecture-review.md`; earlier history is linked there. Those scopes are not acceptance for later edits.
