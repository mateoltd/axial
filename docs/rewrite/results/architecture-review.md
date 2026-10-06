# Architecture review

Updated 2026-10-06. Current scope: Linux failed disk observation and ordinary-process recorded Content recovery after `6816e124`, with final source-qualified macOS/Linux consumers. This is not a parity or release certificate. Earlier measured Ready-read, managed-file simplification and native visibility evidence remain in their feature reports and Git history.

## Scope and ownership

Root owns production integration, generated profile/API actions, witness execution, documentation and serialized shared verification. Independent owners prepare diagnostic/witness helpers and inspect platform semantics and frozen evidence; they do not run shared builds or operate profiles. The unrelated API remains untouched. Read AGENTS.md, conventions, ADR7, delivery, package ownership and current integration evidence before work.

Full non-Guardian behavior and the existing UI remain required on `main`. Predecessor import/application upgrades are excluded; current-app persistence, accepted-operation recovery and supported Minecraft/loaders remain required.

## Findings and fixes

A bounded Linux denial reproduces failed statvfs being published as Some(0). The existing deepest eligible canonical mount remains the selection owner; only its Linux successful native sample becomes observation proof. Failure/overflow stays absent and successful zero remains valid. Do not query excluded network mounts or add another sampler owner/cache. Spec review catches an overbroad first candidate: plain statvfs would change macOS reclaimable-capacity semantics. Remove that candidate's non-Linux changes and new Windows feature; preserve existing platform metrics and leave their failure validity open.

Standards review identifies duplicate subprocess supervision in the load/disk diagnostics. One concrete test helper retains exact selectors, isolated markers, bounded wait and kill/reap handling; production bytes remain unchanged by that consolidation. AGENTS.md merges a recurring load/disk lesson into the existing observation rule: successful sampling is proof, library defaults/refresh booleans are not, and platform metrics must remain intact. No schema, UI, monitor, dependency or speculative framework is introduced.

The Content witness uses the existing public-move helper and production recovery/queue owners, not a parallel decision model or recovery implementation. Review corrects aggregate SQL-output charging; the final bounded baseline matches the first byte-for-byte. Ordinary process settlement owns compensation, one fresh uninstall owns its completion, and complete captured state is compared after Quit/reopen. Neither a409 alone nor a terminal queue label substitutes for exact receipt/file/metadata checks. Existing budget and commit-boundary rules cover this witness correction; no additional anecdotal rule is needed.

## Validation

[Disk evidence](current-benchmarks.md#failed-linux-disk-observation) records confirmed native failure around production capture: RED101, first GREEN0, and the final Linux-only source after platform-semantics review. Final source hashes match inputs. All six Linux libraries pass2,312 checks/14 ignores with normal wrapper0; macOS API/app consumers pass925/16 ignores with normal0. Child summaries are not double-counted. Scoped Rust2024 formatting/diff checks pass. After verification, only init/sleep remain before stopping the exact disposable container; source/evidence and unrelated services are preserved.

[Content evidence](pack-files.md#ordinary-process-recorded-content-recovery) records actual test-assisted exit42 with a complete publication proof, ordinary API compensation, original-tree restoration and a fresh bound uninstall. Both real service processes join ordinary SIGINT0, PIDs/listeners are absent, and the entire captured settled state matches through reopen/final Quit. The helper is read-only and bounded; production owns authority decoding. No game, credential, native UI or provider mutation occurs. Standards/Spec review is clear after the platform correction; feature evidence remains explicitly narrower than full parity.

## Unresolved handoffs

- [Content recovery](pack-files.md): incomplete, unrecorded, partial, unsupported or overbudget proofs/effects remain preserving refusals; acknowledgement and exact retained revisions cannot be waived.
- [Accounts/startup](native-auth.md): first-narrator Continue → Invalid session still lacks exact reproduction/cause; native screenshot/accessibility discrepancy remains unexplained. The completed launcher lifecycle does not expose the game window. No-dialog containment is not repaired old credential access or authenticated continuity; distinct-build acceptance under the intended stable identity remains open.
- [Performance](performance-ui.md), [benchmarks](current-benchmarks.md) and [integration](integration.md): controlled latency and real comparable/Managed qualification remain open. Historical Busy, probe timeout, hosted abort and no-child failures remain distinct and undiagnosed; passing reruns are not causes. Failed Linux disk sampling is corrected; non-Linux failure validity and excluded nested mount attribution remain open.
- External-library native selection/switching and interruption, remaining failure matrices, real gameplay/world/save, four installed artifact architectures and trusted signed-update/restart inputs remain open. No deployment or publication follows.

## Prior reviews

Feature reports and [integration evidence](integration.md) own detailed results and log names. The pre-compaction record remains in local Git: `git show dbc16bba:docs/rewrite/results/architecture-review.md`. Earlier historical entries are available through that file's history; links to this current record do not certify those source checkpoints.

The full-parity goal remains active; this review neither replaces nor resets it.
