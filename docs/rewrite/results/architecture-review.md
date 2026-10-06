# Architecture review

Updated 2026-10-06. Current scope: measured ordinary Ready-read latency from `5f7f7cc2` and the focused managed-file batch simplification `f460df92`. This is not a parity or release certificate.

## Scope and ownership

Root owns the changed managed-file source, generated profile, API actions, witness execution, documentation and serialized shared verification. Independent owners inspect actual routes, batch/native semantics and frozen evidence; they do not operate profiles or edit source. The unrelated API remains untouched. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

The actual detail GET performs fresh installed-artifact verification before Java selection. Optimized reads remain about11s; a five-second CPU sample lands in nested retained-directory/absolute-binding checks. Native directory revision primitives already validate retained ancestry, while managed wrappers repeat that work. Reuse those native primitives only inside the existing batch, removing the duplicate wrappers and one redundant initial leaf check. Retain every child managed check, final managed settlement/admission, exact-name refresh and original child identity, leaf namespace/revision, outer operation fences, hashes and the final inventory revision pass. Empty-parent and absent-leaf paths still end in managed validation.

No readiness cache, interface, coordinator, schema, UI or global validator change is added. Independent safety and simplicity reviews are clear. Existing AGENTS.md single-owner/direct-implementation and required-safety rules cover this finding; no new anecdotal rule is needed.

## Validation

[Measured API evidence](performance-ui.md#measured-ordinary-read-validation-cost) records optimized old→changed→old with the same bounded helper and fixed generated fixture. All nine reads are strict Ready. Median10,973ms→8,439ms→11,193ms supports about23% lower elapsed time in this comparison, not a general latency guarantee. The10-second feedback budget is observational, not an SLO; debug and profiled reads are separate. Eight seconds remains substantial.

Existing16 batch, one inventory-integrity and10 preflight checks pass, followed by all1,012 retained Minecraft tests, grouped readiness and the composed external install/Launch/Stop/reopen test (fake Java, not gameplay); scoped formatting/diff checks and optimized build pass. Both changed/restored runtimes exit0 on ordinary SIGINT and are absent. The final bounded preservation witness exactly matches the retained external-launch snapshot; this is captured final-state equality, not whole-profile or no-transient-effect proof. No game, credential or native action occurs in the timing comparison. Prior [external launch](library-lifecycle.md#real-external-library-launch-and-report-reopen), [native probe](native-auth.md#controlled-probe-launch-and-preserved-reopen) and [root restoration](library-lifecycle.md#real-provider-external-installation-and-reopen) evidence remain separately scoped.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause; native screenshot/accessibility discrepancy remains unexplained. The completed launcher lifecycle does not expose the game window. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Disk-observation validity remains source-qualified and unreproduced.
- External-library native selection/switching and interruption, remaining failure matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
