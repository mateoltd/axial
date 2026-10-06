# Architecture review

Updated 2026-10-06. Current scope: unchanged `10befcc8` real external-library Launch/Stop/report reopen, bounded preservation helpers and measured readiness-read latency. Product source is unchanged. This is not a parity or release certificate.

## Scope and ownership

Root owns generated profiles, API/game actions, witness execution, documentation and serialized shared verification. Independent owners inspect actual routes and review the frozen bounded helpers; they do not operate profiles or change product source. The unrelated API remains untouched. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

Existing Launch/status/report, recency, readiness and library owners suffice. Bind one Launch to its returned intent/session and direct Java child; verify profile-owned executable and external working directory without reading arguments. The before-action witness matches retained installed state. After Stop, require the existing serialized schema4 acknowledgement, exact recency/selection delta, unchanged protected game/runtime/canaries and a bounded game-written tree. Do not add a settlement classifier or retrospective baseline.

Correct cold-history observation to the durable intent/report owner, not live SessionManager status. Two helper prechecks end before Launch with zero history/no child; the original error category is unclassified. An unchanged Ready read completes in13.53s, exceeding the helper's10s bound. Reuse the existing180s bound without relaxing Ready assertions or replaying mutations. Log only bounded public categories/timing/hashes. These are tooling corrections, not product fixes. Existing AGENTS.md entrypoint/typed-ownership/evidence rules suffice; no new anecdotal rule or production churn is warranted.

## Validation

[External launch evidence](library-lifecycle.md#real-external-library-launch-and-report-reopen) records actual Running/boot4570ms, exact-session Stop, tree/output settlement and durable acknowledgement. All3,630 game files, the separately recorded142-file runtime, canaries and six empty non-log directories remain exact. Captured post-Stop/closed/reopened/final state and the full public-report projection match. All five collectors pass with empty error logs; independent helper review is clear. Two normal API SIGINT exits0 leave the exact owned APIs/game absent.

Detail-header reads remain13–14s on this debug artifact; that observation is not a controlled baseline, cause or optimized/native result. No game-screen, Continue, authentication, live switching, crash or installed-platform proof follows. No source change warrants another Cargo run. Prior [native probe](native-auth.md#controlled-probe-launch-and-preserved-reopen) and [external root-restoration](library-lifecycle.md#real-provider-external-installation-and-reopen) evidence remain separately scoped.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause; native screenshot/accessibility discrepancy remains unexplained. The completed launcher lifecycle does not expose the game window. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Disk-observation validity remains source-qualified and unreproduced.
- External-library native selection/switching and interruption, remaining failure matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
