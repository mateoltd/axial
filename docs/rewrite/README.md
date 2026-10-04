# Rewrite preparation

Status: implementation in progress on `main`, following the user-authorized history-preserving integration on 2026-09-28. The initial `feat/clean-rewrite` branch remains a recovery checkpoint. The plan was prepared 2026-09-08 against `2bda9b42c29d02828dd5a2b91d9cf8c8a431ffb0`. The baseline is preserved under `legacy/`. The replacement is not release-ready and no complete runtime parity result is claimed. See [current integration status](results/integration.md).

The accepted scope is **all existing non-Guardian parity**. The earlier Vanilla/Fabric-only recommendation is superseded. Guardian is deferred; other removals need an explicit product decision. Implementation order never implies a feature cut.

The user has explicitly removed old-profile/version compatibility from this pre-release scope. Start fresh; installing over an earlier app may break its data. Remove predecessor import, schema-upgrade compatibility, historical transfer/repair branches and their dedicated tests. Preserve new-app operation recovery and same-version persistence, ordinary file/modpack workflows, and supported Minecraft/loader versions. This decision supersedes earlier cutover/import requirements below and in historical evidence; it does not narrow the retained feature set. Restore the original loader-logo presentation rather than the later neutral glyphs.

The existing UI is an explicit preservation constraint. Keep its layout, components, styling, navigation and interaction behavior. Frontend changes reconnect the current views to replacement contracts. A specific polish improvement must identify its problem, stay narrowly scoped and receive separate visual/behavior review; the rewrite is not authorization for a general redesign.

Removing Guardian-only controls and copy follows the accepted Guardian deferral. Limit that change to the affected controls; it does not justify redesigning the surrounding screen.

The objective is the smallest maintainable implementation of that scope, with short integration cycles and evidence from the actual application. Independently owned modules run in one desktop application. Additional network services, databases per feature, and distributed runtime coordination are not required.

The active completion goal is a fully functional launcher with verified non-Guardian parity, not a test-count milestone. All retained behavior and installed-platform gates in this plan must pass; missing external inputs remain blockers, not implicit exceptions. Recurring architecture review and correction are also completion conditions: follow `AGENTS.md` and [the architecture review](results/architecture-review.md), keeping responsibilities, naming and implementation simple throughout execution.

## Read and execute

1. [Architecture decision](../adr/0007-feature-owned-rewrite.md): target boundaries, contracts, reuse, and alternatives.
2. [Delivery plan](delivery.md): readiness, ownership, dependency scheduling, integration, and completion.
3. [Work packages](work-packages.json): behavior names, exact ownership, dependencies, acceptance checks, and proof requirements.
4. [Parity requirements](parity.md): acceptance method, failure coverage, platform matrix, and Guardian boundary.
5. [Coverage inventory](parity.json): every current API method/template plus UI, native, persisted, and background behavior.

The JSON files are execution inventory, not a new runtime framework or a replacement test runner. A mapped behavior is inventoried, not verified. Package status and evidence distinguish source implementation, focused checks and actual integrated behavior. Tests and fixture implementations are explicit delivery work.

## Preserved scope

- Vanilla, Fabric, Quilt, Forge (including currently implemented historical strategies), NeoForge, and existing version classification and ordering.
- Microsoft/offline accounts, secure refresh credentials, profile sync, account switching, full saved/default/profile skin and cape workflows.
- Instance creation, duplication, removal with and without files, existing-library setup, settings, resources, world backups, screenshots, logs, and mod management.
- Discover, all existing Modrinth content kinds, dependency/conflict handling, updates, uninstall, staging, and modpack flows.
- Performance modes, plans, verified managed mutations, explicit reapply/rollback, health, benchmark suites, qualification, and non-Guardian proof/history behavior.
- Onboarding, themes, audio/music, shortcuts, command palette, Discord, telemetry consent, non-Guardian developer tools, browser development, native integration, and in-app updates.

Only Guardian-specific diagnosis, autonomous repairs/relaunch, suppression memory, idle integrity automation, quarantine policy, modes, and evidence/copy for those actions are deferred. Ordinary interrupted-operation recovery and Performance transaction compensation remain required.

## Decisions to prove early

- Shared HTTP/SSE must pass the packaged WebView transport scenario before native domain event pumps are retired.
- SQLite must reduce metadata ownership and pass interrupted-operation cases before replacing existing stores. Database transactions do not include adjacent game files.
- Public Rust wire types should generate frontend types; choose a maintained generator after an omission/null/tagged-union experiment. Keep validation at untrusted boundaries.
- Prefer maintained updater/packaging facilities if they preserve the existing check/download/apply/restart workflow across the artifact matrix. Do not substitute a download link and call it updater parity.
- Record baseline version and minimum-OS coverage during characterization. A few representative passing versions do not establish an unlimited compatibility claim.

The current conventions and current-state architecture remain authoritative for the existing application. The proposed restructuring is described in the ADR; update affected conventions and current architecture only when the replacement behavior lands.
