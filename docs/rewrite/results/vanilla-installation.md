# Vanilla installation

Status: implementation handoff, awaiting shared build and real queue integration. This package does not claim the offline journey or full runtime parity has passed.

## Boundary

`core/app/src/install/vanilla.rs` exposes:

```rust
async fn install<F>(
    library: &ManagedLibraryOperation,
    runtime_cache: ManagedRuntimeCache,
    version_id: &str,
    send: F,
) -> Result<KnownGoodInstallReceipt, DownloadError>
where F: FnMut(DownloadProgress);
```

`install_with_facts` adds the retained `FnMut(ExecutionDownloadFact)` diagnostic consumer. Both use the real retained `axial_minecraft::Downloader`; there is no replacement planner, raw destination path, fixture success branch, or dependence on legacy Application/State/Guardian orchestration.

The catalog may resolve a display selection first. The installer independently authenticates a fresh Mojang listing and the chosen version metadata, preserving its existing install trust boundary. The catalog's metadata URL is not accepted as caller-authored download authority.

## Retained behavior and readiness

The retained `download/install.rs`, `download/libraries.rs`, `download/assets.rs`, and runtime source/publication implementations continue to own manifest authentication, checksum and size verification, selected library rules, native classifiers, logging configuration, client artifacts, ordinary Java provisioning, unique asset objects, legacy virtual assets, and managed publication/retry obligations. Loader-specific installs continue through their own retained strategies.

The adapter revalidates the exact admitted library generation and forwards nonterminal progress. It suppresses the leaf's terminal progress callbacks because the leaf's successful return only yields a publication receipt. The queue must classify durable publication, verify the receipt against that evidence, persist the resulting activation source, and acknowledge publication before readiness. An error callback likewise does not prove concurrent effects have settled.

The queue retains target exclusion and the exact library/runtime authority across cancellation and settlement. `PublicationIndeterminate` retains its concrete retry capability. Dropping an install future, returning a receipt, or receiving a cancellation request does not authorize a fabricated terminal outcome. The shared installer-primitives owner supplies publication/activation/acknowledgement coordination for Vanilla and loader receipts.

## Verification handoff

Added focused behavior checks:

- Published and failed leaf progress events cannot mark the queue terminal; ongoing byte progress remains intact.
- Invalid and traversal identities fail before acquisition, emit no terminal progress or artifact facts, and leave the isolated library unchanged.

Shared-owner command: `cargo test -p axial-app install::vanilla::tests`. Tests require the retained `axial-minecraft` dev dependency with its `test-support` feature. No shared Cargo/build command was run by this package owner, as required by repository conventions. Local non-writing source checks: `rustfmt --edition 2024 --check core/app/src/install/vanilla.rs` and `git diff --check -- core/app/src/install/vanilla.rs docs/rewrite/results/vanilla-installation.md`.

The integration owner must additionally run the retained `axial-minecraft` installer tests and the actual offline-account create/install/launch/restart journey. The retained source inventory and these adapter tests are not proof of an installed runnable game, cancellation settlement, restart acknowledgement, or platform coverage.

No legacy files or existing-user payloads were modified. No replacement algorithm or removed legacy code is claimed: this package deliberately retains the proven leaf implementation through one feature-owned entrypoint.
