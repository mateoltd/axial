# Architecture review

Updated 2026-10-06. Current scope: measured ordinary Ready-read latency from `5f7f7cc2`, the focused managed-file batch simplification `f460df92`, unchanged-source native visibility diagnosis at `e4d5ace8`, and exact-source Linux verification at `1f1dbc85`. This is not a parity or release certificate.

## Scope and ownership

Root owns the changed managed-file source, generated profile, API actions, witness execution, documentation and serialized shared verification. Independent owners inspect actual routes, batch/native semantics and frozen evidence; they do not operate profiles or edit source. The unrelated API remains untouched. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

The actual detail GET performs fresh installed-artifact verification before Java selection. Optimized reads remain about11s; a five-second CPU sample lands in nested retained-directory/absolute-binding checks. Native directory revision primitives already validate retained ancestry, while managed wrappers repeat that work. Reuse those native primitives only inside the existing batch, removing the duplicate wrappers and one redundant initial leaf check. Retain every child managed check, final managed settlement/admission, exact-name refresh and original child identity, leaf namespace/revision, outer operation fences, hashes and the final inventory revision pass. Empty-parent and absent-leaf paths still end in managed validation.

No readiness cache, interface, coordinator, schema, UI or global validator change is added. Independent safety and simplicity reviews are clear. Existing AGENTS.md single-owner/direct-implementation and required-safety rules cover this finding; no new anecdotal rule is needed.

The separate [native visibility probe](native-auth.md#current-source-native-visibility-probe) observes a hidden document and pending entrance animation, not a proved layout failure or the normal artifact's cause. Keep diagnostic builds separate; do not add CSS/activation machinery from a blank capture alone. The collector refuses the historical full-settings digest after ordinary navigation/Quit without a captured field-level baseline. Retain that refusal and prepare fresh actual projections before further mutations; do not manufacture retrospective preservation. No product fix or additional architecture rule follows from this diagnosis.

The new before-third-game witness has independent review and one actual passing bounded capture, not acceptance of its unexecuted Stop/reopen phases. Review corrects a tree budget charged from an earlier stat instead of the actual validated read; observed Quilt caches require a measured32MiB/file/64MiB/tree capacity, retaining the failed smaller-budget capture and all guards. Existing total-batch-budget guidance already covers the accounting finding. Evidence review also removes unproved physical-occlusion, normal-artifact-cause and no-automatic-credential-read claims. No new state owner or source/UI change follows.

## Validation

[Measured API evidence](performance-ui.md#measured-ordinary-read-validation-cost) records optimized old→changed→old with the same bounded helper and fixed generated fixture. All nine reads are strict Ready. Median10,973ms→8,439ms→11,193ms supports about23% lower elapsed time in this comparison, not a general latency guarantee. The10-second feedback budget is observational, not an SLO; debug and profiled reads are separate. Eight seconds remains substantial.

Existing16 batch, one inventory-integrity and10 preflight checks pass, followed by all1,012 retained Minecraft tests, grouped readiness and the composed external install/Launch/Stop/reopen test (fake Java, not gameplay); scoped formatting/diff checks and optimized build pass. Both changed/restored runtimes exit0 on ordinary SIGINT and are absent. The final bounded preservation witness exactly matches the retained external-launch snapshot; this is captured final-state equality, not whole-profile or no-transient-effect proof. No game, credential or native action occurs in the timing comparison. Prior [external launch](library-lifecycle.md#real-external-library-launch-and-report-reopen), [native probe](native-auth.md#controlled-probe-launch-and-preserved-reopen) and [root restoration](library-lifecycle.md#real-provider-external-installation-and-reopen) evidence remain separately scoped.

[Exact-source Linux selection](performance-ui.md#current-source-linux-verification) now passes all six libraries:2,311 checks,14 existing ignores, zero failures. Matching Docker exec events prove wrapper exit0 despite the SSH handle's broken pipe. Source/generation hashes verify; no source change or test relaxation follows. The settled disposable container is stopped with evidence retained and unrelated services unchanged. Independent Standards/Spec review is clear on native Unix/Windows ancestry and retained managed fences. No new architectural pattern or AGENTS.md rule is warranted; installed/native and user-reported failures remain open.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause; native screenshot/accessibility discrepancy remains unexplained. The completed launcher lifecycle does not expose the game window. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Disk-observation validity remains source-qualified and unreproduced.
- External-library native selection/switching and interruption, remaining failure matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
