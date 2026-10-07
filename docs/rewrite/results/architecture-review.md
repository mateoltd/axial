# Architecture review

Updated 2026-10-07. Reviewed uncommitted launch changes against `640fc7fa` on `main`: standalone preflight diagnostics, generated contracts, settings inheritance and browser terminal convergence. Full non-Guardian parity remains active; this is not an installed-release certificate.

## Ownership and scope

Root owns shared exports/generated integration, evidence and serialized verification. The preflight and frontend launch owners have frozen their edits; independent Standards and Spec reviews are complete. Review excludes legacy profiles, user installations, signing, credentials and deployment. Preserve the existing UI, filesystem authority and current-app recovery.

## Findings and fixes

- Keep memory clamp and override-origin facts at the existing settings inheritance owner. Project safe successful diagnostics at the existing preflight owner, retaining final account/settings/instance/filesystem fences. Capture shared host input once; avoid unused per-row process/disk sampling in bulk readiness. No second sampler, cache, timer or readiness framework.
- Replace implemented handwritten frontend preflight mirrors with Rust-generated aliases through the existing exporter. Retain omission versus explicit-null semantics. Successful diagnostics are not a complete early-refusal/negative-readiness contract.
- Recover retained Playing only through the original authenticated accepted intent after a typed missing live-session response. Require exact instance/session/start identity and complete tree/output settlement; absence alone is not exit proof. Reuse existing completion/log/readiness owners without process adoption, write replay, persistence or revision rebasing.
- Independent review reproduced terminal publication delayed by log drainage, then stale SSE and pending Stop responses restoring nonterminal state. Publish the verified terminal view before drainage and fence each observed publication path with the existing `finishingSessions` owner. Preserve ordinary monotonic live-status convergence.
- Remove unrelated whole-file test-formatting churn while preserving syntax, comments and assertions. No source naming/layer change is justified solely for style.
- Compact this record instead of retaining repeated chronology. Existing typed-owner, accepted-completion, publication-fence and real-boundary rules cover these findings; no additional AGENTS.md rule is warranted.

## Validation

Actual HTTP preflight regression goes RED then GREEN; all ten existing preflight guard cases pass. Export generation and equality checks pass. Frontend completion regressions have four meaningful RED/GREEN boundaries, with the final focused selection passing101 cases. Canonical frontend checks pass531 with zero failures and one existing TODO. Serialized library checks finish829 application and121 API cases with zero failures; ten and eight existing helper ignores respectively. The nested child-helper result is not double-counted. Wrappers are joined; whitespace checks pass.

Logs and detailed scope stay in [launch wire evidence](wire-parity-review.md#launch-diagnostics-and-terminal-convergence). Final independent Standards and Spec reviews report zero remaining actionable source findings in their respective success-diagnostics and terminal-convergence scopes. Ordinary frontend build publishes `239a78614d6e`; scoped production formatting and whitespace pass, wrappers0. No native/browser process-reopen acceptance is inferred.

Earlier [hosted run37559031946](https://github.com/mateoltd/axial/actions/runs/37559031946) passes both jobs at exact parent `640fc7fa`, not the working diff. Preserve the earlier asset-watch timeout as unexplained despite passing isolated/full serial checks; no deadline or guard waiver.

## Unresolved handoffs

- Launch: typed early-refusal/missing/corrupt readiness facts and actual retained-browser API-reopen acceptance remain pending. Source fixtures do not establish installed convergence or gameplay.
- [Updates](updates.md): channel policy, snapshot hydration and deferred intent have source/fixture evidence, not trusted installed-update acceptance. Keep asset-watch diagnosis open.
- [Accounts](native-auth.md): verify the current noninteractive credential adapter across distinct builds under the intended authorized stable signing identity. Earlier signed test binaries predate that adapter.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen remain open. Process survival and fakesink decoding do not prove them; retain graphics/protocol failures.
- [Forge](forge-loader.md), [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md), [world files](world-files.md) and [library lifecycle](library-lifecycle.md) retain their detailed recovery/version/runtime limits. Passing controls do not diagnose historical failures.
- [Integration](integration.md) owns full-parity acceptance, including actual saved-world reload and the installed-platform matrix. This review does not replace or reset that goal.

## Earlier evidence

Detailed evidence stays with its feature owner: [telemetry](telemetry.md), [capacity and benchmark continuation](current-benchmarks.md), [World backup](world-files.md), [managed historical rollback](performance-ui.md), [retained wire audit](wire-parity-review.md), [native acceptance](native-auth.md) and [released Linux packaging](linux-package.md). Earlier reviewed scopes and exact hashes remain recoverable in `git show 640fc7fa:docs/rewrite/results/architecture-review.md`; they are not acceptance for later edits.
