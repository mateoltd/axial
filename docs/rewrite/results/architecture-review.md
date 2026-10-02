# Architecture review

Updated 2026-10-02. Changed-scope review, not a parity or release certificate. Historical evidence remains in [integration evidence](integration.md) and Git.

## Scope and ownership

Read AGENTS.md, conventions, ADR7, integration evidence and active ownership. Development continues on `main`, preserving history. Quilt/pre-worker history is integrated at `e863869d`. Managed-filesystem test admission and metadata-only interruption preservation are frozen after independent review. Root owns docs, serialized verification and the disposable native acceptance harness.

## Findings and corrections

- **Preserve the integrated historical boundary.** Existing `install_history`, shared immutable batches and original-registry absence preserve missing-target Content without live authority. Completed global/archive reads share one 128-row/8-MiB actual-byte budget; exact bytes, scope-bound digests and final-write verification prevent repair or false acknowledgment. Previous redundant readers are removed. Details and older RED evidence remain in integration/history records.
- **Canonicalize the fixture, not admission.** The existing temporary-profile helper now creates roots under the canonical system temporary parent, following the existing AGENTS rule. This removes the need for native journey scripts to override Python's process-global temporary-directory setting. Production path admission and reset decisions are unchanged.
- **Keep compatibility at the provider boundary.** Quilt's existing install and reconstruction paths share one private mapping validator. Omission requires an authenticated matching base with a valid UTC release time at or after 2025-12-16; present mappings retain exact identity/integrity and declaration checks. No new persisted proof, public contract or checksum fallback.
- **Separate readability from admission.** The existing install-history validator derives ordinary-safe global/instance history or an exact terminal global observation. Stored reads/optional metadata retain the latter; strict construction and ordinary binding reject it before the support ledger. Pre-worker cancellation retains its original narrower proof. Diagnostic memory remains excluded evidence, not Guardian behavior or settlement. No schema, wire contract or parallel journal is added.
- **Reuse retained filesystem admission.** `open_for_test` now retains the existing absolute-directory guard, verifies it against the leased root and keeps its native session. The duplicate test-only fresh raw-path branch and weak-root plumbing are removed. Exact ancestor/root identity, spelling, revision fences and cached-root reopening are unchanged; no new cache or production admission policy.

## Validation

Logs are under `.rewrite-logs/`; previous Content checkpoint and canonical-fixture canary verification remain in [integration evidence](integration.md). Frontend generation `0e9d96fdf1b8` and the previous488-test frontend result are unchanged, not new interface acceptance.

Quilt's official omitted-mapping decoder RED precedes correction; all seven provider checks and the complete 968-test Minecraft suite pass, including real install/reconstruction composition and old-date/malformed/proof-downgrade refusals (`quilt-mapping-{provider-green,minecraft}.log`). The install-history owner RED reproduces rejection of both exact source variants. All five initialization-history checks and the corrected authenticated HTTP lost-waiter/replay/reopen journey pass. Both earlier HTTP attempts were confounded by a fixture short key (`fabric`) instead of the original component ID (`net.fabricmc.fabric-loader`); they are not independent parser RED evidence. Existing typed-contract guidance covers the correction, so no new AGENTS rule is added.

Frozen combined consumers pass1,011 app,141 embedded API and88 desktop tests, six app/API ignored helpers each. Generated-contract equality and scoped formatting pass (`install-initialization-{app-final,api-embedded-final,desktop-final,wire-check,format}.log`). Frontend markup and generation are unchanged; no fresh native acceptance follows.

Hosted [run37032409962](https://github.com/mateoltd/axial/actions/runs/37032409962) passes exact `e863869d`; terminal watch and independent SHA/conclusion agree. New test-admission root-replacement/case-only-rename checks pass before and after simplification. Full Minecraft passes970 tests in95.89s, versus968 in1478.13s before; the unchanged isolated asset-cache case passes27.40s before and1.51s after (`test-admission-{baseline,green,asset-baseline,asset-green,minecraft-final}.log`). These are local observations under unchanged command/environment, not controlled production latency claims.

Metadata observations first reproduce reader/global-preparation refusal and missing HTTP count after ordinary POST422. Three focused owner/import checks and the actual lost-response/replay/reopen journey then pass, with ordinary import blocked before/after preservation, immutable source canaries and empty execution tables. Frozen combined consumers pass1,014 app,142 embedded API and88 desktop tests, six app/API ignores each (`worker-observation-{consumers-final,desktop-final}.log`). Scope formatting and generated-contract equality pass (`worker-observation-{format,wire-check}.log`).

Hosted [run37036184707](https://github.com/mateoltd/axial/actions/runs/37036184707) passes exact integrated `cbcea749`; independent SHA/conclusion matches terminal watch. Unsigned macOS ARM64 packaging also passes with unchanged generation, not installed-interface acceptance.

## Unresolved handoffs

Nonterminal records remain outside this slice: missing outcome and evolving immutable identity need a separate contract decision, not a parser allowance. Other interrupted shapes remain unsupported. Existing AGENTS ownership/contract rules cover these findings; no anecdotal rule is added.

Unsupported history and source-effect settlement remain separate. Changed-source replay retains its fingerprint conflict. Earlier Forge/NeoForge and managed-Java runtime evidence is qualified to its recorded checkpoint, not inherited by later edits. Read-only planning found no intact launch-ready installation in the retained three gameplay fixtures; historical Ready rows are not present readiness proof.

Interrupted-Reset Preserve-files UI, OAuth/game-window journeys, remaining failure/restart cases, four installed architectures and trusted signed-update inputs remain open. Retained generated fixtures and unknown predecessor process intents are untouched. Never infer settlement from PID absence, age or best-effort progress.

The fresh debug bundle opened/quit normally, then published v2 interruption waited with matching idle fingerprints and unchanged siblings. macOS dialog automation timed out, and no manual choice arrived. Root verified/stopped only failed test PID64514 with SIGTERM; it exited143 with no successor. Intent, marker and synthetic files retain their waiting hashes, protected canaries pass, and the profile/keeper remain retained. This is test abortion, not native Preserve acceptance (`native-current-interrupted-reset-acceptance.md`); the earlier stale handoff also lacks acceptance evidence.

The rebuilt bundle now uses fresh fixture `axial-scenario-qbkeu197`, keeper43579 and PID18922/session18631. Native automation reports the Mac locked; manual unlock is requested, with no UI input or acceptance inferred. Preserve this live process image until it closes before rebuilding again.

The full-parity goal remains active. Architecture automation retains its existing paused status. No deployment, release publication or legacy/user-profile mutation.
