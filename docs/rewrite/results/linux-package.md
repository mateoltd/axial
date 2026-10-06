# Linux AppImage build

2026-10-06, source `c471e96ddf12625376e671fb9433a972c43ebb1e`. A normal optimized x86_64 AppImage builds successfully in an isolated Horizon container. This is a diagnostic package and inspected payload, not installed-runtime, publisher-trust or full-parity acceptance. No production source/UI, host packages, application profiles, signing or publication changed.

## Inputs and isolation

Fresh rootless container `installed-linux-TbgISuLv`, ID `feee3cf195795da518368e2b37f34b46dd2660bc02cd9a8204f08f786c315f90`, uses official Rust1.93.1 Bookworm image `docker.io/library/rust@sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31`; resolved amd64 image ID `78a50754b62786434bc4ebfd874ef382348701b5f01b2c0d21b8e17cfad97969`. No mounts, GUI/DBus sockets, published ports, host networking, devices or privileged mode. Sources, tools and target belong to its writable layer; shared build verification remains serialized with six Cargo jobs. The older `parity-1f1dbc85` container is absent, not reused.

The tracked archive contains all seven workspace members through `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `core`, `apps`, `scripts` and `frontend`; no `legacy/`. This selected compilation input is not a tagged release-source receipt. The existing frontend generation is `fff9567f184b5b2a0b1a56b5fe9317da75900a9e2d06f8f1bb121a329a6bf411`; its unchanged verifier passes after transfer and after bundling. Frontend source matches its earlier `318e7ede` build. That build used Node24.19.0 against the requested24.13.1; this Linux verifier uses copied Linux OCI Node24.21.0. No fresh frontend rebuild or fully pinned release-toolchain claim follows.

| Input | SHA256 |
| --- | --- |
| `installed-linux-source.tar` | `5fe1a62a625d64d698e1bf838f04f5f474481a9a7c0861051ce2a94ad9818b10` |
| Initial generation archive | `062aa1564a677829a0d01a3a61a3ecd107aaa3dae04299f5888bed16df6645b9` |
| Metadata-free generation archive | `ea9f3c2300418c353e4188cad7a1aa147ba178f26ba997b68a092f0029b031f4` |
| Linux Node binary | `7fde7b8afa198da66257f42ee2001d874c7355631e6d1579a5fb5ef1f246df4c` |

The initial generation transfer twice refuses `generation_inventory_drift`: GNU extraction exposes89 AppleDouble extras alongside84 expected files, with none missing. Preserve that tree at `/work/frontend-dist-with-metadata`. Recreate only the transfer archive using `COPYFILE_DISABLE=1 tar --no-xattrs --no-acls`; the unchanged verifier then passes. No inventory waiver, manifest edit or product fix. An earlier container creation refuses because its workdir does not exist; that unused container is removed and recreated without the workdir. The unused Node extraction container is also removed after its copy. Both are recreatable from retained images; actual build inputs/evidence remain.

Container-only apt supplies the repository's Linux build dependencies plus DBus development and packaging tools: WebKit4.1 `2.50.6`, GTK3 `3.24.38`, DBus `1.14.10`; Cargo/Rust1.93.1. Tauri CLI2.11.2 installs with `--locked` into `/work/tauri-cli`. No host dependency installation follows. See [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/#linux).

## Build and output

Inside `/work/source`, with `/work/tauri-cli/bin` on PATH:

```sh
node scripts/cargo-target.mjs run -- cargo tauri build \
  --target x86_64-unknown-linux-gnu --bundles appimage --no-sign \
  --config '{"build":{"beforeBuildCommand":""},"bundle":{"createUpdaterArtifacts":false}}' \
  -- --locked
```

The existing target lease owns the build; the inline override skips only the already-verified frontend rebuild and updater-artifact generation. Normal application ID/product/capabilities remain unchanged, with no Inspector/debug feature. CLI installation and the real package command both join exit0; optimized application compilation takes3m19s and bundling finishes one AppImage. Compiler dead-code warnings remain recorded, not fixed by this packaging slice. Pinned bundler [extract-and-run support](https://raw.githubusercontent.com/tauri-apps/tauri/tauri-cli-v2.11.2/crates/tauri-bundler/src/bundle/linux/appimage/linuxdeploy.rs) needs no container FUSE or privilege change.

Output under `/work/source/target/x86_64-unknown-linux-gnu/release/`:

| Output | SHA256 |
| --- | --- |
| `axial-desktop` | `aa604c3b3e12ce9223e008c2e27ed34dd21fb1e3f1ac84cede5d44562fc4ccfe` |
| AppDir and extracted `usr/bin/axial-desktop` | `3015f4b9591914653fa9badac0995478bbfb6f1618288c326618da9f209f7119` |
| `bundle/appimage/Axial Rewrite_0.4.0-dev.5_amd64.AppImage` | `d6989cd530d0536a04c520d0d0dd267d675967896769312f41f2635d0f0e77ae` |

AppImage size is120,769,016 bytes. Its identical hash is verified after container→Horizon copy to `/var/tmp/installed-linux.TbgISuLv/candidate.AppImage`, then SCP to local `.rewrite-logs/installed-linux-candidate.AppImage`. Neither copy is launched. No release collector receipt is fabricated for this untagged diagnostic input.

## Payload inspection and limits

- Both main executables are unstripped ELF64/x86_64, with identical BuildID. Ordered needed libraries and `.text`, `.data`, `.data.rel.ro`, `.rela.dyn`, `.rela.plt`, `.eh_frame` match. Exactly three `.rodata` bytes change `UNK`→`APP`, explained by the pinned [package-marker owner](https://github.com/tauri-apps/tauri/blob/tauri-cli-v2.11.2/crates/tauri-bundler/src/bundle.rs#L18-L74), which restores the unpatched release binary after bundling. Dynamic inspection shows added `$ORIGIN/../lib` RUNPATH and relocated/enlarged string table. These measured packaging changes are not whole-binary equality or a semantic-equivalence certificate.
- AppImage extract-only inspection never starts AppRun/the launcher. Its first runtime extraction creates0700 directories, although the SquashFS lists0755. Container-only `squashfs-tools` inspection first exits2 for unsupported destination SELinux xattrs; retain that result. Fresh extraction with explicit `-no-xattrs` exits0: all404 entries match AppDir by kind, file size/hash, mode and symlink target. UID/time/xattr preservation is not certified.
- Actual payload contains `libwebkit2gtk-4.1.so.0`, both WebKit subprocesses, injected bundle, desktop entry and GTK hook. The hook selects `GDK_BACKEND=x11`; Horizon's active KDE Wayland session alone proves neither XWayland nor visible runtime acceptance. The overlay leaves media-framework bundling disabled, so audio portability remains open. See [AppImage guidance](https://v2.tauri.app/distribute/appimage/) and [GTK helper](https://raw.githubusercontent.com/tauri-apps/linuxdeploy-plugin-gtk/master/linuxdeploy-plugin-gtk.sh).
- The archive retains an absolute `.DirIcon` target under the container's build path and a root-owned0770 `AppRun.wrapped`. Other desktop/icon links are relative. Record these portability/runtime handoffs; do not infer successful icon resolution or launch outside the container, or silently patch the package.
- Post-build comparison with a fresh source-archive extraction finds every original file unchanged. Only build-generated desktop `gen`, frontend `dist`, frontend `._dist` metadata and `target` are additional. Generation verification does not itself inspect every embedded asset or prove a fresh source rebuild.

Prepared helper hashes are retained because several upstream download URLs are mutable. CLI pinning alone does not pin every packaging byte; linuxdeploy's recorded hash is after the pinned bundler's integration-marker patch. The downloaded output-plugin file is present; the quiet bundle log does not certify whether linuxdeploy selected it or its built-in fallback.

| Prepared helper | SHA256 |
| --- | --- |
| `AppRun-x86_64` | `f30140a43a0a59e46db21bdefdf749b9e9f2c6946e92afabbacf98b8ae73fb4f` |
| `linuxdeploy-x86_64.AppImage` | `20eebde3c18ae2e44279bd624fc72482503aece216d5d77f10932235342f71c1` |
| `linuxdeploy-plugin-gtk.sh` | `7804c9eef13e59bf2783aad9882ef9db8f3f3f9e8d631874b1d348d550a3693f` |
| `linuxdeploy-plugin-gstreamer.sh` | `c107b49d84edbffc6ab226ed1007e0626a4f7aa2c3a36b7782bef62351d49e94` |
| `linuxdeploy-plugin-appimage.AppImage` | `49d6a17160675a6bd1781699aae6bdf7692d98552e02a3671d2183d10547842e` |

## Evidence and remaining work

Logs are `.rewrite-logs/installed-linux-*`: identity/source/generation/setup/CLI/package, `payload-{inspection,detail,proof}`, `appimage-extract`, `squashfs-inspection`, `final-extraction`, `runtime-inputs`, `helper-hashes`, `host-copy`, `local-copy` and `cleanup`. The original research handoff remains at `.rewrite-logs/linux-package-research.md`.

After all commands/copies join, exact container inspection shows only PID1 `sleep`, no build children. `podman stop` escalates that idle PID1 to SIGKILL after10s; stopped container exit137 is not an application/build exit. Exact ID reports `running=false`; writable layer, tools, failed transfers, payload and outputs remain. Package command exit0 is independently joined, not inferred from cleanup or a file appearing.

Independent Standards, Spec and architecture reviews are clear; both two-axis reviewers independently hash the local AppImage. No new application tests or AGENTS.md rule are needed for this source-unchanged packaging/evidence slice.

Next: ordinary launch on Horizon from this hash-bound package, canonical isolated profile and supported native observation; verify runtime permissions/icon portability, WebView/API/SSE/media/opener/game lifecycle and persistence. Host WebKit absence is not automatically a blocker for bundled dependencies. No UI/runtime case in [installed acceptance](../../../acceptance/native/matrix.mjs), other architecture, publisher authenticity, signed updater or full-parity gate is closed by this build.
