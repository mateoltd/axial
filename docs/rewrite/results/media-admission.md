# Media admission

Status: implementation in progress; integrated and installed parity is not established.

Owned source: `core/app/src/media/` and `apps/desktop/src/native_skin.rs`.

## Implemented core behavior

- Retained Minecraft 64x32 to 64x64 conversion, padded/copied legacy recognition, opaque legacy hat removal, limb regions, Classic/Slim suggestion and base alpha correction from the predecessor's image leaf.
- Skin inputs are limited to 256 KiB and decoded with a 512 KiB decoder allocation budget. A complete static PNG is required; dimensions, truncation, CRC errors, animation and bytes after IEND are rejected.
- Cape inputs have the same byte limit, a 2 MiB decoder budget, dimensions up to 512x512 and complete frame validation. Reencoding strips arbitrary metadata. The predecessor checked cape headers only; complete decoding is a deliberate validation improvement.
- Head rendering retains scaling and overlay blend behavior, with a hard 512 pixel output bound. HTTP consumers retain their own existing size default and clamp.
- Texture identity remains lowercase SHA-256 of normalized PNG bytes.
- Native admission retains only bounded validated bytes behind a random 30 second single-use handle, scoped to a native-created window incarnation. No file path is serialized or reopened by consumption. New selections invalidate previous handles even when validation fails. Closing prevents late publication and drains active permits.

## Verification

`rustfmt --edition 2024` passed for owned core files. Behavior tests are authored for conversion, alpha, normalization identity, PNG rejection, cape complete decoding, head blending, hash vectors, handle scope/reuse/expiry/replacement and close/drain. Shared Cargo commands are reserved for the integration owner; no worker Cargo/build commands were run.

Requested integration command: `cargo test -p axial-app media:: -- --nocapture`, captured to a log with the result tail retained.

## Integration and remaining evidence

The root owns `pub mod media`, the `png = 0.17.16` dependency and shared wire registration. Skin-library and profile-media consumers agreed the normalization exports. Transport is owned by delivery and must restrict media credentials to GET/HEAD on actual registered media route shapes plus exact admitted origin checks. Native shell adaptation is being coordinated against the managed file and TaskOwner producers.

Real native picker/drop, filesystem replacement during admission, reset/close races and installed WebView HTTP/media journeys remain acceptance obligations. Unit test source alone does not satisfy them. All non-Guardian scope remains retained.
