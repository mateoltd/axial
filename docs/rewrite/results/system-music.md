# System and music API

Status: retained domain behavior, route adapters and isolated tests implemented. Shared Rust execution is owned by integration; installed audio playback and cross-platform interruption evidence remain separate gates.

`system::system_resource_status()` queries the real host through the existing sysinfo dependency. Its four public fields and recommendation formula match the legacy route: reserve two GiB, constrain the recommended minimum to two through four GiB and maximum to four through eight GiB, and reduce both for small hosts. `routes::system::router()` exposes `GET /api/v1/system` and performs the host query on a blocking worker. No Guardian fields or readiness decisions are included.

`MusicService::new(library: LibraryLifecycle, tasks: TaskOwner) -> Result<Self, MusicError>` constructs the fixed two-track inventory without downloading or creating a cache directory. The retained files and sources are `vapor-halo.mp3` and `sublunar-hum.mp3` from the existing `music-v2` GitHub release. `routes::music::router(Arc<MusicService>)` exposes the existing `/api/v1/music/status` and `/api/v1/music/track` GET routes. An omitted track index selects zero and excessive unsigned indices select the last track. Malformed or unknown query fields produce fixed public errors.

Music operations use the shared TaskOwner and acquire application-root pins from the authoritative LibraryLifecycle. Status reports actual bounded cache presence and does not create directories. Cache misses use the retained create-only managed transfer leaf with the existing 32 MiB limit, exact GitHub/release-assets origins, no automatic retry, ten-second connect timeout and 120-second request/read bounds. Publication uses admitted capabilities and fixed leaf names. Existing cache occupants are never overwritten; oversized, symbolic-link and case-alias occupants are preserved and refused. Returned media uses `audio/mpeg` and shares the bounded byte allocation with the HTTP body.

Concurrent requests for a track share one accepted operation. Dropping an HTTP waiter does not drop the transfer. Shutdown cancellation is forwarded to the transfer and joined; complete data is discarded when cancellation is observed before publication. Blocking mutation panics remain panics to the TaskOwner so unknown settlement retains its pin. Failed responses and partial/oversized bodies cannot publish a track.

Directory creation, transfer cleanup, publication and discard obligations are retained together with their exact application-root pin. `MusicService::settle()` performs synchronous bounded reconciliation; composition must invoke it on a blocking worker after shared tasks drain and before releasing the library root. `has_unsettled_effects()` exposes unresolved work. An obligation that cannot prove settlement remains retained and reports unavailability; the implementation does not manufacture a ready track or discard its ownership.

## Verification

This worker ran direct rustfmt and focused `git diff --check`. No Cargo command or external music download was run by this worker. The integration owner runs:

```sh
cargo test -p axial-app music::tests --lib
cargo test -p axial-app system::tests --lib
cargo test -p axial-api routes::music::tests --lib
cargo test -p axial-api routes::system::tests --lib
```

Eleven music domain tests exercise actual loopback HTTP downloads, full 32 MiB create-only publication (above the distinct 16 MiB metadata-stage budget), destination admission/cancellation, simultaneous waiter sharing, abandoned HTTP waiters, cancellation and joined cleanup, provider errors/truncation/oversize with retry, persistent cache reads across service reconstruction, legacy index clamping, cached-file size/alias refusal, Unix symlink refusal, and closed library admission. The symlink case is platform-specific; sockets are required rather than silently skipped.

Two system tests cover independent expected recommendations and real host memory. The music route tests use an actual local HTTP server and real temporary cached files for status/audio/query errors and shutdown refusal. The system route test invokes the actual host query through its HTTP adapter. All test files are isolated temporary data; no installed profile, legacy source, or external library is modified.

## Remaining evidence

Integration's initial macOS run passed the two system domain tests and system/music route tests, but real music download tests exposed `StageCreate(Unsupported)` in the retained filesystem transient-stage backend. The filesystem owner is implementing the shared native-boundary fix. Music retains the full 32 MiB managed-transfer contract; the smaller recoverable metadata-stage API is not a compatible substitute. A passing rerun of the full-limit and cancellation fixtures is required before this feature can be called complete.

The unchanged frontend bootstrap and audio controls already consume these route shapes; no UI code or styling changed. Composition must register both routes behind its authenticated transport and settle music before root release. Browser/native audio decoding, audible playback, resume/fade interaction, packaged platform behavior and abrupt process termination during cache publication have not been demonstrated by these module fixtures. A service reconstruction test is not a process-crash test. Existing filesystem/transfer recovery leaves are reused, but their platform proof does not establish complete music integration by itself.
