# Architecture review

Updated 2026-10-07. Scheduled review covers Java-override preflight changes against `9b50bd2d` and the pending private browser-reopen witnesses at `bd270792` on `main`. No later production diff is present. Full non-Guardian parity remains active; this is not an installed-release certificate.

## Ownership and scope

Root owns production changes, generated integration, evidence and serialized verification. The HTTP fixture owner relinquished edits before root's final typed-match correction; independent Standards and Spec reviews cover all four frozen source files. Review excludes legacy profiles, user installations, signing, credentials and deployment. Preserve the existing UI, filesystem authority and current-app recovery.

Private proxy and persisted-witness authors have separate exclusive files under `.rewrite-logs/`; root owns actual browser execution and observations. Independent review precedes execution. These disposable helpers are not production state or recovery owners.

## Findings and fixes

- Runtime spawn `NotFound` does not prove selected-file absence: a valid executable may name an absent interpreter. Remove that incorrect classification at the existing probe owner, retaining initial file-error and Rosetta semantics. No second filesystem scan.
- Missing and nonexecutable explicit overrides share one factual diagnostic projection. Match typed owner errors and captured override provenance; preserve their distinct public causes. Generic process-start failures do not acquire absence facts.
- Reuse the current diagnostic constructor/resource sampler. Retain bundle revalidation, captured account/config revisions and final instance/exclusion/settings fences; failed capture preserves the original runtime refusal. Bulk and implicit managed runtime do not acquire unused or mislabeled facts.
- The HTTP fixture shares assertions for the two applicable refusals, rather than duplicating setup or adding a test framework. Its genuinely installed Ready and exact failed-interpreter control prove the intended boundary, followed by settled no-effect/privacy assertions.
- Generated reason contracts remain Rust-owned. No new namespace, wrapper directory, pass-through layer, configuration or UI change is justified. Existing typed-error/evidence rules already cover this pattern; no additional AGENTS.md rule is warranted.
- Private browser witnesses now require the complete Fabric fixture target tree unchanged: its synthetic child has no justified file writes. They count mutation attempts before transport refusal, recognize document requests beyond `/`, accept a genuine observed zero boot duration, and preserve opaque native receipts instead of duplicating their private encoding. Original admission authenticates native authority; cold reads validate persisted terminal evidence. Compare observed live/cold revisions, not an assumed zero; the cold owner currently starts at1. Forwarded responses are not browser-consumption proof. Existing AGENTS.md rules cover these findings without another rule or framework.

## Validation

Detailed RED/GREEN and source hashes stay in [runtime evidence](wire-parity-review.md#observed-java-override-readiness). The actual public-probe, missing-override HTTP and nonexecutable-override HTTP regressions pass after meaningful failures. Seven probe checks, generated export/equality and project-edition formatting pass. Independent Standards and Spec reviews each report zero actionable findings. Final current-source checks pass831 app/123 API/83 desktop/531 frontend, zero failures, with existing helper ignores/one frontend TODO and nested child results excluded. Build `239a78614d6e` verifies unchanged budgets; semantic/asset checks pass. Wrappers join0. [Integration](integration.md) retains checkpoint scope; no native/browser process-reopen acceptance is inferred.

The runtime slice's [hosted run37565859463](https://github.com/mateoltd/axial/actions/runs/37565859463) passes both jobs at exact `bd270792b81b84daa0a3e4b2540e906c8e48064a`; retained terminal watch and SHA/job query agree. The preceding client-file slice's detailed checks remain [client-file evidence](wire-parity-review.md#observed-client-file-readiness). The private overlapping watch probe passes after correcting its copy inventory, but does not reproduce or diagnose the historical timeout; no shipped debug code, deadline or guard change.

Private helper syntax checks pass; independent Standards and Spec rereview find no remaining code-level findings, followed by Spec closure of the cold-authentication wording. Frozen proxy `43661554` and witness `5ddaf48a` are under `.rewrite-logs/browser-reopen-{proxy,proof}.mjs`; root's syntax logs are `browser-reopen-{proxy,proof}-syntax-review.log`. These checks do not establish process settlement, filesystem preservation, retained-document convergence or absence of replay. No browser journey has run with these helpers.

## Unresolved handoffs

- Launch: other artifact/runtime/early-refusal facts, global/component-origin coverage and actual retained-browser API-reopen acceptance remain pending. Source fixtures do not establish installed convergence or gameplay.
- [Updates](updates.md): channel policy, snapshot hydration and deferred intent have source/fixture evidence, not trusted installed-update acceptance. Keep asset-watch diagnosis open.
- [Accounts](native-auth.md): verify the current noninteractive credential adapter across distinct builds under the intended authorized stable signing identity. Earlier signed test binaries predate that adapter.
- [Linux](linux-package.md): visible UI, HTTP/media/SSE, audible playback and ordinary Quit/reopen remain open. Process survival and fakesink decoding do not prove them; retain graphics/protocol failures.
- [Forge](forge-loader.md), [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md), [world files](world-files.md) and [library lifecycle](library-lifecycle.md) retain their detailed recovery/version/runtime limits. Passing controls do not diagnose historical failures.
- [Integration](integration.md) owns full-parity acceptance, including actual saved-world reload and the installed-platform matrix. This review does not replace or reset that goal.

## Earlier evidence

Detailed evidence stays with its feature owner: [telemetry](telemetry.md), [capacity and benchmark continuation](current-benchmarks.md), [World backup](world-files.md), [managed historical rollback](performance-ui.md), [retained wire audit](wire-parity-review.md), [native acceptance](native-auth.md) and [released Linux packaging](linux-package.md). Prior client-file review remains recoverable in `git show 9b50bd2d:docs/rewrite/results/architecture-review.md`; earlier history is linked there. Those scopes are not acceptance for later edits.
