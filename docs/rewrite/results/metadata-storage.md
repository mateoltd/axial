# Metadata storage

Implemented `core/app/src/storage/mod.rs` with one mutex-owned SQLite connection, feature-owned SQL migrations, query-only reads, and typed feature transactions. Callbacks are synchronous metadata work; async consumers must call them from `spawn_blocking`. The connection does not own feature decisions, payload files, or an application service locator.

Exports: `MetadataStore`, `Migration`, `StorageError`, `StorageResult<T>`, `StorageLimits`, and the `rusqlite` reexport. `MetadataStore::open`, `in_memory`, and their `*_with_limits` forms construct the owner. `read<T,E>` and `transaction<T,E>` preserve domain errors via `E: From<StorageError>`. `migrate` applies a supplied ordered group atomically, retains previously applied unrelated groups, and rejects changes to the exact SQL for an existing ID. Feature migrations are registered in composition in dependency order.

The connection requires foreign keys, DELETE/FULL journaling for disk metadata, a distinct AXIR application ID, no-follow opening of the database file, and an integrity check before persistent configuration. Corrupt or foreign-application files are preserved. Failed typed callbacks receive a bounded integrity check, and an unresolved integrity or rollback failure closes mutation admission. Mutex poisoning also closes admission. SQLite lock waits default to 750 ms; SQL execution defaults to five seconds and 100 million virtual machine instructions. These limits cannot preempt arbitrary non-SQL work inside a callback, which is forbidden by the private interface contract.

Focused tests cover persistence, relational constraints, typed rollback, atomic migrations, duplicate/changed migration rejection, query-only reads, corrupt/foreign database preservation, SQL/lock budgets, panic rollback, Unix symlink refusal, and real subprocess termination before versus after commit. The subprocess fixture uses only fresh temporary databases. No test claims hardware power-loss durability or platform filesystem coverage beyond its actual run.

Integration dependencies: `rusqlite` 0.37 with `bundled` and `hooks`, existing `thiserror`, and `tempfile` for tests. Root registration: `pub mod storage;`.

Verification command for the serialized integration owner:

```sh
cargo test -p axial-app storage::tests --lib
```

Author verification: direct rustfmt completed using installed Rust 1.93.1. Cargo/build/test execution is reserved for the integration owner and has not been run by this worker. Feature persistence consumers and runtime recovery evidence remain required before integrated completion or parity claims.
