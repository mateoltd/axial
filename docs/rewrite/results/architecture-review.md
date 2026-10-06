# Architecture review

Updated 2026-10-06. Reviewed scope: earliest Forge startup diagnosis/planner correction, authenticated FML inputs and recovery constraints, exact continuation preservation and unlocked native paint controls. This is not a parity or release certificate.

## Ownership and constraints

Root owns integration, generated profile/API actions, documentation and serialized shared Cargo verification. Independent owners review source and frozen witnesses; none operates profiles or shared builds. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before editing. Work stays on `main`, preserving history, non-Guardian behavior and UI. Predecessor import/schema upgrades are excluded; current-app recovery and supported Minecraft/loaders remain required. The unrelated API stays untouched.

## Findings and corrections

The current planner's unconditional LWJGL direct-path property reproduces a macOS JNI load refusal with the actual pinned LWJGL 2.9.0 library and Java 8. Removing only that property makes the identical probe initialize successfully. The existing planner now retains standard `java.library.path`, provider arguments, native source/receipt validation, pre-spawn revalidation and immutable-native/scratch separation. No archive alias, renamed payload, version dispatcher, adapter or persistence owner is added. Standard JVM lookup is not an exclusive-path sandbox; do not claim identical runtime lookup semantics.

Review also exposes a separate preexisting validation gap: assigned reserved `-D` properties are refused, but bare forms can override owned paths. Exact property-key matching closes it inside the existing validator. Both corrections have meaningful existing-test REDs before production changes. Independent final Standards/Spec review is clear; no UI change or new recurring AGENTS rule is warranted.

The first parallel launch check has ten silent fake-Java probe timeouts before the changed planner. The same binary passes serially. Actual entrypoint ordering and missing first-marker observations locate these failures before command construction, not their host/security/scheduling cause. Preserve failed evidence; do not increase deadlines, invent a race coordinator or claim a passing retry diagnoses the failure.

The runtime replay reuses the existing intent/session/report owners. A separate bounded read-only continuation witness pins the exact first settlement, preserves its raw history/proof hashes and all four recorded inventories, and admits one fresh history pair plus existing best-effort recency. The original one-launch witness remains unchanged. Boot-positive Stop and settlement-only results have distinct verdicts; safe cleanup alone is not startup acceptance. A wrong-session control refuses at the actual history-binding boundary.

The next1.4.7 attempt remains non-booting but settles and reopens exactly. Its separately reviewed bounded witness preserves both prior histories and a freshly captured first-instance tree, admitting only one exact Java CAS/new history/recency. Closed retained stack methods narrow failure to FML acquisition; the actual old HTTP source gives the same Java8 a301 body instead of the checksum-pinned library. Inspecting the real caller/artifact rules out the existing1.5.x/legacyfixer mirror path: this FML has no mirror-property reader. Keep the eventual correction in the existing installation/preparation owner, not a speculative flag, download coordinator or parallel journal. No production edit is made yet.

All four declared library inputs now have exact HTTPS download witnesses, including refusal of a same-named but different ASM artifact. [The correction boundary](forge-loader.md#verified-fml-inputs-and-correction-boundary) separates immutable installation sources from process-writable copies. Review identifies a necessary current-app recovery constraint: adding inventory entries changes activation digests. Preserve settled historical verification; reconstruct interrupted operations against their recorded contract before ordinary repair. Do not turn this feature fix into a digest waiver, compatibility import or another recovery owner.

Unlocked native controls distinguish complete unchanged Inspector paint from incomplete normal Startup paint. Focus changes the latter but does not complete it; a later hidden Inspector document has a finished animation and nonzero document timeline. One timing sample does not establish ongoing advancement. Neither hidden state nor a passing diagnostic artifact establishes a CSS, activation or normal-release fix. No animation/style workaround or new AGENTS rule follows from these differing observations.

## Validation

Source `0d18318e` passes 126 serialized launch checks/one existing helper ignore and 118 API-library checks/eight ignores, normal wrapper 0. Exact manifest and reserved-property REDs fail 101 then pass; scoped formatting/diff checks and independent final review are clear. Release API compilation exits 0. These are selected consumers, not every target or installed artifact. [Forge evidence](forge-loader.md#current-source-runtime-failure-and-native-lookup-correction) owns commands, logs, frozen hashes and retained failures.

The [real earliest-client replay](forge-loader.md#fixed-earliest-client-startup-and-preserved-continuation) observes boot in 3386ms/no owner failure classes, acknowledges one stopped report and restores exact tuple/Java 8/Ready on reopen. Both APIs join ordinary SIGINT 0; owned Java/APIs/listeners are absent. Complete captured settlement remains equal through final exit, with the first failed history, private protected state, exact recorded files and canaries preserved. Game-writable files are outside that installation proof. Native inventory lists Java but cannot bind its window, so menu/narrator/gameplay are unobserved.

[Universal continuation](forge-loader.md#earliest-universal-startup-refusal-and-preserved-reopen) passes strict settled/reopened/final equality and two normal API exits0, with zero checked obligations and owned processes/listeners absent; its verdict remains settlement-only. The acquisition minimizer joins42/checksum refusal, not timeout. [Native controls](native-auth.md#unlocked-paint-comparison) join three menu Quits0 with unchanged logical empty-profile hashes. These runtime diagnostics change no production source and do not require another Cargo run; they do not close visual, gameplay or parity gates.

The result-document diff from `db4daac0` passes independent Standards and Spec review with no remaining findings. Review removes an unsupported ongoing-timeline claim and closes the class-equality evidence gap with root's retained bounded comparison/exit0; it does not execute downloaded classes or establish the proposed fix. Scoped diff checks pass. No new recurring architecture rule is warranted.

## Unresolved handoffs

- [Accounts/startup](native-auth.md): narrator Continue → Invalid session and native visibility discrepancy remain undiagnosed. No-dialog containment is not authenticated distinct-build continuity under the intended stable signing identity.
- [Forge](forge-loader.md): earliest-universal1.4.7 startup, other retained-era/failure matrices and actual gameplay remain open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md), [Content](pack-files.md) and [library lifecycle](library-lifecycle.md) retain qualification, interruption and recovery limits. Passing runtime reruns do not diagnose historical Busy, no-child, timeout or hosted failures.
- [Integration](integration.md) owns remaining native switching, world/save, four installed architecture and trusted update/restart gates. Parallel fake-Java timeout sensitivity above remains unexplained. No deployment, publication, signing or credential-permission change follows.

## Prior scope

Detailed read-contract/codec corrections remain in [wire parity](wire-parity-review.md) and [benchmark persistence](current-benchmarks.md). Earlier Finder, response-loss, Remove, Fabric dependency, disk and Content reviews remain in the linked feature reports. The pre-compaction record is recoverable with `git show 5388835e:docs/rewrite/results/architecture-review.md`; earlier reviews remain in that file's history.

The full-parity goal remains active; this review neither replaces nor resets it.
