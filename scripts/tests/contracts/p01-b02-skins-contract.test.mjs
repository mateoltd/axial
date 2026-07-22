import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import test from "node:test";

const repository = fileURLToPath(new URL("../../../", import.meta.url));
const read = (path) => readFile(join(repository, path), "utf8");

const sourceSlice = (source, startMarker, endMarker) => {
  const start = source.indexOf(startMarker);
  assert.notEqual(start, -1, `missing source marker: ${startMarker}`);
  const end = source.indexOf(endMarker, start + startMarker.length);
  assert.notEqual(end, -1, `missing source marker: ${endMarker}`);
  return source.slice(start, end);
};

test("native skin callbacks synchronously establish core filesystem authority", async () => {
  const [
    commands,
    main,
    nativeSkin,
    native,
    capabilities,
    desktopCargo,
    filesystemPlatform,
    filesystemTests,
  ] = await Promise.all([
    read("apps/desktop/src/commands/mod.rs"),
    read("apps/desktop/src/main.rs"),
    read("apps/desktop/src/native_skin.rs"),
    read("frontend/src/native.ts"),
    read("apps/desktop/capabilities/main.json"),
    read("apps/desktop/Cargo.toml"),
    read("core/fs/src/platform.rs"),
    read("core/fs/src/lib.rs"),
  ]);

  assert.doesNotMatch(commands, /fn\s+read_skin_file\s*\(|path:\s*String/);
  assert.doesNotMatch(main, /commands::read_skin_file/);
  assert.doesNotMatch(
    native,
    /readNativeSkinFile|tauri\.dialog|dialog\?:|tauri:\/\/drag-/,
  );
  assert.doesNotMatch(capabilities, /dialog:allow-open/);
  assert.match(
    commands,
    /fn\s+pick_skin_file\s*\([\s\S]*state:\s*State<'_, AppState>/,
  );
  assert.match(desktopCargo, /axial-fs = \{ path = "\.\.\/\.\.\/core\/fs" \}/);
  assert.doesNotMatch(desktopCargo, /^libc\.workspace/m);
  assert.doesNotMatch(desktopCargo, /windows-sys/);
  assert.match(
    main,
    /handle_native_skin_drag\([\s\S]*Arc::clone\(close_event_state\.root_session\(\)\)[\s\S]*event/,
  );

  const pickerFunction = sourceSlice(
    commands,
    "pub async fn pick_skin_file(",
    "#[tauri::command]\npub async fn consume_skin_drop",
  );
  const pickerCallback = sourceSlice(
    pickerFunction,
    ".pick_file(move |selected| {",
    "        });",
  );
  assert.match(pickerCallback, /\.into_path\(\)/);
  assert.match(
    pickerCallback,
    /NativeSkinFileAdmission::admit\(&root_session, path\)/,
  );
  assert.ok(
    pickerCallback.indexOf("NativeSkinFileAdmission::admit") <
      pickerCallback.indexOf("selected_tx.send"),
  );

  const pickerAfterCallback = sourceSlice(
    pickerFunction,
    "        });",
    "    tauri::async_runtime::spawn_blocking",
  );
  assert.match(
    pickerFunction,
    /spawn_blocking\(move \|\| admission\.read\(\)\)/,
  );
  assert.doesNotMatch(pickerAfterCallback, /into_path|::admit|open_file/);
  assert.match(commands, /fn\s+consume_skin_drop\s*\(\s*token:\s*String/);
  assert.match(main, /WindowEvent::DragDrop\(event\)/);
  assert.match(
    nativeSkin,
    /use axial_fs::\{FileCapability, FileRevision, LeafName\}/,
  );
  assert.doesNotMatch(
    filesystemPlatform,
    /external directory cannot be (?:the filesystem|a volume) root/,
  );
  assert.equal(
    [...filesystemPlatform.matchAll(/if guard\.identity == root\.identity/g)]
      .length,
    2,
  );
  assert.match(
    filesystemTests,
    /admitted_absolute_directory_accepts_the_filesystem_root/,
  );
  assert.match(
    filesystemTests,
    /admitted_absolute_directory_accepts_the_volume_root/,
  );
  assert.match(
    filesystemTests,
    /admit_absolute_directory_authority_outside_root\(temporary\.path\(\)\)[\s\S]*AbsoluteDirectoryOutsideRootAdmission::InsideRoot/,
  );
});

test("native skin drag owns one expiring capability-backed token", async () => {
  const [nativeSkin, native, hook] = await Promise.all([
    read("apps/desktop/src/native_skin.rs"),
    read("frontend/src/native.ts"),
    read("frontend/src/views/accounts/use-saved-skin-native-drag-drop.ts"),
  ]);

  assert.match(
    nativeSkin,
    /const SKIN_DROP_TOKEN_TTL: Duration = Duration::from_secs\(30\);/,
  );
  assert.match(nativeSkin, /pending: Option<PendingNativeSkinDrop>/);
  assert.match(
    nativeSkin,
    /file: FileCapability,[\s\S]*revision: FileRevision/,
  );
  assert.match(nativeSkin, /Semaphore::new\(1\)/);
  assert.match(nativeSkin, /try_acquire_owned\(\)/);
  assert.match(
    nativeSkin,
    /root_session\s*\n\s*\.admit_absolute_directory\(parent\)/,
  );
  assert.match(nativeSkin, /parent[\s\S]*\.open_file\(&leaf\)/);
  assert.match(nativeSkin, /let revision = file[\s\S]*\.revision\(\)/);
  assert.match(
    nativeSkin,
    /file\.into_revision_reader\(revision, SKIN_FILE_MAX_BYTES\)/,
  );
  assert.match(nativeSkin, /reader\.read_to_end\(&mut bytes\)/);
  assert.match(nativeSkin, /failure\.into_parts\(\)/);
  assert.match(nativeSkin, /let \(file, revision\) = reader\.cancel\(\)/);
  assert.match(nativeSkin, /match reader\.finish\(\)/);
  assert.match(nativeSkin, /failure\.into_reader\(\)\.cancel\(\)/);
  assert.doesNotMatch(nativeSkin, /\.read_bounded\(SKIN_FILE_MAX_BYTES\)/);

  const dropArm = sourceSlice(
    nativeSkin,
    "NativeSkinDropSelection::One(path) => {",
    "fn emit_drag(",
  );
  assert.match(
    dropArm,
    /NativeSkinFileAdmission::admit\(&root_session, path\)/,
  );
  assert.ok(
    dropArm.indexOf("NativeSkinFileAdmission::admit") <
      dropArm.indexOf("tauri::async_runtime::spawn"),
  );
  assert.doesNotMatch(
    dropArm.slice(dropArm.indexOf("tauri::async_runtime::spawn")),
    /::admit|open_file|PathBuf/,
  );

  for (const retired of [
    "NativeSkinFileRevision",
    "open_native_skin_file",
    "windows_path_has_local_disk_prefix",
    "GetFileType",
    "FILE_FLAG_OPEN_REPARSE_POINT",
    "VOLUME_NAME_GUID",
    "MetadataExt",
    "libc::",
  ]) {
    assert.doesNotMatch(nativeSkin, new RegExp(retired));
  }
  assert.doesNotMatch(nativeSkin, /local disk path|local volume/);

  const beginDrag = sourceSlice(
    nativeSkin,
    "fn begin_drag",
    "fn drag_eligible",
  );
  const beginDrop = sourceSlice(nativeSkin, "fn begin_drop", "fn cancel_drag");
  const cancelDrag = sourceSlice(
    nativeSkin,
    "fn cancel_drag",
    "fn try_begin_admission",
  );
  assert.doesNotMatch(beginDrag, /pending\s*=/);
  assert.match(beginDrop, /state\.pending\s*=\s*None/);
  assert.doesNotMatch(cancelDrag, /pending\s*=/);
  assert.doesNotMatch(cancelDrag, /advance_generation/);
  assert.match(nativeSkin, /token\.len\(\) != 32/);
  assert.match(nativeSkin, /if pending\.token != token/);
  assert.match(nativeSkin, /state\.pending\.take\(\)/);
  assert.match(nativeSkin, /tokio::time::sleep\(SKIN_DROP_TOKEN_TTL\)/);
  assert.match(nativeSkin, /expiry_coordinator\.expire\(&expiry_token\)/);
  assert.match(native, /listen\('axial:desktop:skin-drag'/);
  assert.match(native, /invoke<unknown>\('consume_skin_drop', \{ token \}\)/);
  assert.doesNotMatch(native, /paths:\s*string\[\]/);
  assert.doesNotMatch(hook, /\.paths|Path|isPngPath/);
});

test("Application strictly decodes bounded static skin PNGs", async () => {
  const [nativeSkin, skinModule, skinImage, skinTests] = await Promise.all([
    read("apps/desktop/src/native_skin.rs"),
    read("apps/api/src/application/skin.rs"),
    read("apps/api/src/application/skin/image.rs"),
    read("apps/api/src/application/skin/tests/saved_library.rs"),
  ]);

  assert.match(
    skinModule,
    /pub use image::\{SKIN_PNG_MAX_BYTES, SkinPngValidationError, validate_skin_png\}/,
  );
  assert.match(skinImage, /pub const SKIN_PNG_MAX_BYTES: usize = 256 \* 1024/);
  assert.match(skinImage, /SKIN_PNG_DECODER_BUDGET_BYTES/);
  assert.match(skinImage, /png::Decoder::new_with_limits/);
  assert.match(skinImage, /decoder\.set_ignore_text_chunk\(true\)/);
  assert.match(skinImage, /decoder\.set_ignore_iccp_chunk\(true\)/);
  assert.match(skinImage, /fn png_ends_exactly_at_iend/);
  assert.match(skinImage, /chunk_end == bytes\.len\(\)/);
  assert.match(
    skinImage,
    /info\.width != SKIN_WIDTH[\s\S]*LEGACY_SKIN_HEIGHT \| SKIN_HEIGHT/,
  );
  assert.match(skinImage, /info\.animation_control\.is_some\(\)/);
  assert.match(skinImage, /reader[\s\S]*\.finish\(\)/);
  assert.match(nativeSkin, /validate_skin_png\(&bytes\)/);
  assert.doesNotMatch(nativeSkin, /bytes\.starts_with\(PNG_SIGNATURE\)/);
  assert.match(
    skinTests,
    /skin_png_validator_rejects_signature_bearing_malformed_png/,
  );
  assert.match(skinTests, /skin_png_validator_rejects_invalid_dimensions/);
  assert.match(skinTests, /skin_png_validator_rejects_bytes_after_iend/);
  assert.match(
    skinTests,
    /skin_png_validator_ignores_compressed_text_and_profile_chunks/,
  );
  assert.match(
    skinTests,
    /skin_png_validator_enforces_the_decoder_allocation_budget/,
  );
  assert.match(
    skinTests,
    /skin_png_validator_accepts_the_maximum_bounded_input/,
  );
});

test("architecture records native skin authority timing and remote-volume policy", async () => {
  const architecture = await read("docs/ARCHITECTURE.md");

  assert.match(
    architecture,
    /Native skin picker and drag ingress establish filesystem authority while the Tauri[\s\S]*callback still owns the OS-selected path/,
  );
  assert.match(architecture, /30-second, one-shot opaque[\s\S]*token/);
  assert.match(
    architecture,
    /One revision-pinned capability operation reads the exact admitted length/,
  );
  assert.match(architecture, /remote volume/);
  assert.match(architecture, /no local-volume policy/);
  assert.match(architecture, /not a hard kernel I\/O[\s\S]*deadline/);
});
