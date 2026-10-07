# Architecture review

Updated 2026-10-07. Scheduled review covers frozen Java-override preflight changes against `9b50bd2d` on `main`. Full non-Guardian parity remains active; this is not an installed-release certificate.

## Ownership and scope

Root owns production changes, generated integration, evidence and serialized verification. The HTTP fixture owner relinquished edits before root's final typed-match correction; independent Standards and Spec reviews cover all four frozen source files. Review excludes legacy profiles, user installations, signing, credentials and deployment. Preserve the existing UI, filesystem authority and current-app recovery.

## Findings and fixes

- Runtime spawn `NotFound` does not prove selected-file absence: a valid executable may name an absent interpreter. Remove that incorrect classification at the existing probe owner, retaining initial file-error and Rosetta semantics. No second filesystem scan.
- Missing and nonexecutable explicit overrides share one factual diagnostic projection. Match typed owner errors and captured override provenance; preserve their distinct public causes. Generic process-start failures do not acquire absence facts.
- Reuse the current diagnostic constructor/resource sampler. Retain bundle revalidation, captured account/config revisions and final instance/exclusion/settings fences; failed capture preserves the original runtime refusal. Bulk and implicit managed runtime do not acquire unused or mislabeled facts.
- The HTTP fixture shares assertions for the two applicable refusals, rather than duplicating setup or adding a test framework. Its genuinely installed Ready and exact failed-interpreter control prove the intended boundary, followed by settled no-effect/privacy assertions.
- Generated reason contracts remain Rust-owned. No new namespace, wrapper directory, pass-through layer, configuration or UI change is justified. Existing typed-error/evidence rules already cover this pattern; no additional AGENTS.md rule is warranted.

## Validation

Detailed RED/GREEN and source hashes stay in [runtime evidence](wire-parity-review.md#observed-java-override-readiness). The actual public-probe, missing-override HTTP and nonexecutable-override HTTP regressions pass after meaningful failures. Seven probe checks, generated export/equality and project-edition formatting pass. Independent Standards and Spec reviews each report zero actionable findings. Final current-source checks pass831 app/123 API/83 desktop/531 frontend, zero failures, with existing helper ignores/one frontend TODO and nested child results excluded. Build `239a78614d6e` verifies unchanged budgets; semantic/asset checks pass. Wrappers join0. [Integration](integration.md) retains checkpoint scope; no native/browser process-reopen acceptance is inferred.

The preceding client-file slice's [hosted run37564042012](https://github.com/mateoltd/axial/actions/runs/37564042012) passes both jobs at exact `9b50bd2d`, not this runtime diff. Its detailed531 frontend/830 app/122 API/83 desktop checks remain [client-file evidence](wire-parity-review.md#observed-client-file-readiness). The private overlapping watch probe passes after correcting its copy inventory, but does not reproduce or diagnose the historical timeout; no shipped debug code, deadline or guard change.

## Unresolved handoffs

- Launch: other artifact/runtime/early-refusal facts, global/component-origin coverage and actual retained-browser API-reopen acceptance remain pending. Source fixtures do not establish installed convergence or gameplay.
- [Updates](updates.md): channel policy, snapshot hydration and deferred intent have source/fixture evidence, not trusted installed-update acceptance. Keep asset-watch diagnosis open.
- [Accounts](native-auth.md): verify the current noninteractive credential adapter across distinct builds under the intended authorized stable signing identity. Earlier signed test binaries predate that adapter.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen remain open. Process survival and fakesink decoding do not prove them; retain graphics/protocol failures.
- [Forge](forge-loader.md), [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md), [world files](world-files.md) and [library lifecycle](library-lifecycle.md) retain their detailed recovery/version/runtime limits. Passing controls do not diagnose historical failures.
- [Integration](integration.md) owns full-parity acceptance, including actual saved-world reload and the installed-platform matrix. This review does not replace or reset that goal.

## Earlier evidence

Detailed evidence stays with its feature owner: [telemetry](telemetry.md), [capacity and benchmark continuation](current-benchmarks.md), [World backup](world-files.md), [managed historical rollback](performance-ui.md), [retained wire audit](wire-parity-review.md), [native acceptance](native-auth.md) and [released Linux packaging](linux-package.md). Prior client-file review remains recoverable in `git show 9b50bd2d:docs/rewrite/results/architecture-review.md`; earlier history is linked there. Those scopes are not acceptance for later edits.
