# Native skins

2026-10-04. Bounded unsigned macOS ARM64 debug acceptance, not provider-account or installed-release parity. Generated profile `/private/tmp/axial-native-current.YwnWYB/profile`; unchanged frontend generation `dcb70bac0468`.

## Correction and verification

At `dc4825d0`, a saved private-loopback texture renders actual3D, while Steve and upload previews show3D preview unavailable and the default strip is blank. Existing consumers use fetch for embedded/normalized data URLs and File blob URLs; img-src permission alone does not authorize these fetches ([CSP connect-src](https://www.w3.org/TR/CSP3/#directive-connect-src)). The native policy omitted those schemes. Only connect-src adds `data: blob:` in the runtime owner and packaged mirror. Exact admitted API origin, self-only scripts, navigation and backend payload bounds remain unchanged. No UI or decoding owner is replaced.

The disposable engine probe uses actual `loadSkinBitmap`, embedded Steve, actual normalized saved PNG as data, a File object URL and a separately allowed loopback PNG. Old policy rejects the first three with observed connect-src violations; allowed loopback decoding succeeds. Corrected policy decodes all four64×64. A localhost-alias sentinel remains blocked by connect-src and receives zero requests under both policies. Probe/server are normally closed (`skin-local-policy-engine-{red,green}.jpg`); this is browser-engine enforcement, not a native WebView by itself.

All79 desktop tests pass, including exact packaged/runtime origin substitution and unchanged script/default restrictions (`skin-local-policy-desktop.log`). Native packaging passes after normal Quit of the previous image. Corrected executable SHA256 `1eb41dcf54b4d1a2e5357e27fe0d970c0a37eb521b4b9eae56be73b22f96a3ac`; same profile reopens and physically renders Steve, Alex, every default thumbnail and the saved texture. Actual normalized replacement preview also renders (`native-skin-{default,alex,replacement}-corrected.jpg`). Independent source/probe review is clear.

## Local workflow

Fixtures follow the existing skin tests:64×64RGBA8, opaque `[red,20,30,255]`. Generated inputs are retained under `/private/tmp/axial-skin-current.t67qQC`: first/red80,209bytes/SHA256 `6cce481204ed3a2a06977116fd188c7f27f8ee8466114efd5e67418c1d4a51c8`; replacement/red160,208bytes/SHA256 `d73d5291d60148995a3dc91ec15e6b18a1bac9611fc9fec4495178b0bf8e18ab`. They are not built-in identities and require no real account or third-party skin lookup.

Actual native Add skin/picker admits first.png, normalizes and Saves locally as NativeSkinParity. Save & apply remains correctly disabled for the offline identity. The stored normalized PNG is823bytes, key/SHA256 `558b3d4b204dfcfc94ac266d8ee8edfa0963476d421342c1314634f2f15b334b`; actual card and stage render. Raw fixture digest is not confused with normalized texture identity. Initial Go-to navigation required fresh sheet state and a final Open; no upload/save was replayed after an unknown outcome.

The corrected bundle retains this record after ordinary reopen. Edit skin renames it NativeSkinRenamed and selects Slim; Save changes metadata but preserves the texture key. A second Edit → Replace PNG uses the actual picker, accepts replacement.png, shows a rendered Replacement ready preview and suggests Classic. Save publishes one replacement record under key/SHA256 `9c967c81ebf3e1d923420572028ff14ad8f28ec21e3a7cc1433ce64fd37a3b03`,823bytes, Classic/local_upload. Actual Download PNG writes `/Users/mateo/Downloads/NativeSkinRenamed.png`; its823bytes and SHA256 match the persisted replacement exactly. Toast/request success alone is not the export proof.

The corrected native process exits 0 on normal Quit after editing/export. Another ordinary reopen retains NativeParity, both instances and exactly one NativeSkinRenamed card with the replacement stage rendering. Durable key/name/Classic/source/823bytes remain exact.

With the user's confirmation, the actual Delete action removes only NativeSkinRenamed and returns the stage to Steve. Durable saved_skins and saved_skin_accounts counts are both zero. Normal Quit exits 0; ordinary reopen finishes startup, retains the offline identity and both instances, and shows the empty library with rendered defaults (`native-skin-deleted-reopen.jpg`). The exported PNG retains its exact replacement digest. Sibling hash/identity remains `175f4eb97ac9c3e89f73caf9a5102778a46a66db7735d68547f326fd7b1edaa6` / `16777229:160385925:73:1791121305`. Only the generated saved record is deleted; retained fixtures and export allow reconstruction. The user's standing permission for disposable rewrite-test artifacts is recorded in AGENTS.md.

Hosted [run 37220525662](https://github.com/mateoltd/axial/actions/runs/37220525662) passes exact source `2bb6793af83e507ee4a82714514ecfc76a1cb6aa`, both application and delivery jobs. Real online skin apply/reset, invalid-file/drop cases and other installed architectures remain open.
