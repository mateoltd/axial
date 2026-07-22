use axial_api::application::skin::{
    SKIN_PNG_MAX_BYTES, SkinPngValidationError, validate_skin_png,
};
use axial_config::AppRootSession;
use axial_fs::{FileCapability, FileRevision, LeafName};
use serde::Serialize;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::dpi::PhysicalPosition;
use tauri::{DragDropEvent, Emitter, WebviewWindow};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use uuid::Uuid;

const SKIN_FILE_MAX_BYTES: u64 = SKIN_PNG_MAX_BYTES as u64;
const SKIN_DROP_TOKEN_TTL: Duration = Duration::from_secs(30);
const NATIVE_SKIN_DRAG_EVENT: &str = "axial:desktop:skin-drag";
const SKIN_DROP_LOCK_INVARIANT: &str =
    "desktop skin-drop lock poisoned; native file authority may be inconsistent";

#[derive(Debug, Eq, PartialEq, Serialize)]
pub(crate) struct NativeSkinFile {
    name: String,
    bytes: Vec<u8>,
}

pub(crate) struct NativeSkinFileAdmission {
    name: String,
    file: FileCapability,
    revision: FileRevision,
}

#[derive(Clone)]
pub(crate) struct NativeSkinDropCoordinator {
    shared: Arc<Mutex<NativeSkinDropState>>,
    admission_gate: Arc<Semaphore>,
}

struct NativeSkinDropState {
    generation: u64,
    drag_eligible: bool,
    pending: Option<PendingNativeSkinDrop>,
}

struct PendingNativeSkinDrop {
    token: String,
    expires_at: Instant,
    admission: NativeSkinFileAdmission,
}

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
enum NativeSkinDragType {
    Enter,
    Over,
    Drop,
    Leave,
}

#[derive(Clone, Copy, Serialize)]
struct NativeSkinDragPosition {
    x: f64,
    y: f64,
}

#[derive(Serialize)]
struct NativeSkinDragPayload {
    r#type: NativeSkinDragType,
    eligible: bool,
    token: Option<String>,
    position: Option<NativeSkinDragPosition>,
    error: Option<&'static str>,
}

enum NativeSkinDropSelection {
    None,
    Multiple,
    One(PathBuf),
}

impl NativeSkinDropCoordinator {
    pub(crate) fn new() -> Self {
        Self {
            shared: Arc::new(Mutex::new(NativeSkinDropState {
                generation: 0,
                drag_eligible: false,
                pending: None,
            })),
            admission_gate: Arc::new(Semaphore::new(1)),
        }
    }

    fn begin_drag(&self, eligible: bool) {
        let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
        advance_generation(&mut state);
        state.drag_eligible = eligible;
    }

    fn drag_eligible(&self) -> bool {
        self.shared
            .lock()
            .expect(SKIN_DROP_LOCK_INVARIANT)
            .drag_eligible
    }

    fn begin_drop(&self) -> u64 {
        let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
        advance_generation(&mut state);
        state.drag_eligible = false;
        state.pending = None;
        state.generation
    }

    fn cancel_drag(&self) {
        let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
        state.drag_eligible = false;
    }

    fn try_begin_admission(&self) -> Option<OwnedSemaphorePermit> {
        Arc::clone(&self.admission_gate)
            .try_acquire_owned()
            .ok()
    }

    fn publish(
        &self,
        generation: u64,
        admission: NativeSkinFileAdmission,
    ) -> Option<String> {
        let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
        if state.generation != generation {
            return None;
        }
        let token = Uuid::new_v4().simple().to_string();
        state.pending = Some(PendingNativeSkinDrop {
            token: token.clone(),
            expires_at: Instant::now() + SKIN_DROP_TOKEN_TTL,
            admission,
        });
        Some(token)
    }

    fn expire(&self, token: &str) {
        let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
        if state
            .pending
            .as_ref()
            .is_some_and(|pending| pending.token == token)
        {
            state.pending = None;
        }
    }

    pub(crate) fn consume(&self, token: &str) -> Result<NativeSkinFile, String> {
        if token.len() != 32 || !token.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("Dropped skin file token is invalid.".to_string());
        }
        let pending = {
            let mut state = self.shared.lock().expect(SKIN_DROP_LOCK_INVARIANT);
            let Some(pending) = state.pending.as_ref() else {
                return Err("Dropped skin file is no longer available.".to_string());
            };
            if Instant::now() >= pending.expires_at {
                state.pending = None;
                return Err("Dropped skin file expired. Drop it again.".to_string());
            }
            if pending.token != token {
                return Err("Dropped skin file token is invalid.".to_string());
            }
            state.pending.take().expect("validated pending skin drop")
        };
        pending.admission.read()
    }
}

fn advance_generation(state: &mut NativeSkinDropState) {
    state.generation = state
        .generation
        .checked_add(1)
        .expect("desktop skin-drop generation overflowed");
}

pub(crate) fn handle_native_skin_drag(
    window: &WebviewWindow,
    coordinator: NativeSkinDropCoordinator,
    root_session: Arc<AppRootSession>,
    event: &DragDropEvent,
) {
    match event {
        DragDropEvent::Enter { paths, position } => {
            let eligible = matches!(skin_drop_selection(paths), NativeSkinDropSelection::One(_));
            coordinator.begin_drag(eligible);
            emit_drag(window, NativeSkinDragType::Enter, eligible, None, *position, None);
        }
        DragDropEvent::Over { position } => emit_drag(
            window,
            NativeSkinDragType::Over,
            coordinator.drag_eligible(),
            None,
            *position,
            None,
        ),
        DragDropEvent::Drop { paths, position } => {
            let generation = coordinator.begin_drop();
            let position = *position;
            match skin_drop_selection(paths) {
                NativeSkinDropSelection::None => emit_drag(
                    window,
                    NativeSkinDragType::Drop,
                    false,
                    None,
                    position,
                    None,
                ),
                NativeSkinDropSelection::Multiple => emit_drag(
                    window,
                    NativeSkinDragType::Drop,
                    false,
                    None,
                    position,
                    Some("Drop one PNG skin file."),
                ),
                NativeSkinDropSelection::One(path) => {
                    let Some(admission_permit) = coordinator.try_begin_admission() else {
                        emit_drag(
                            window,
                            NativeSkinDragType::Drop,
                            false,
                            None,
                            position,
                            Some("Another skin file is still being checked."),
                        );
                        return;
                    };
                    let admission = NativeSkinFileAdmission::admit(&root_session, path);
                    drop(admission_permit);
                    let (token, error) = match admission {
                        Ok(admission) => (coordinator.publish(generation, admission), None),
                        Err(error) => (None, Some(error)),
                    };
                    if let Some(token) = token.as_ref() {
                        let expiry_coordinator = coordinator.clone();
                        let expiry_token = token.clone();
                        tauri::async_runtime::spawn(async move {
                            tokio::time::sleep(SKIN_DROP_TOKEN_TTL).await;
                            expiry_coordinator.expire(&expiry_token);
                        });
                    }
                    emit_drag(
                        window,
                        NativeSkinDragType::Drop,
                        token.is_some(),
                        token,
                        position,
                        error.as_deref(),
                    );
                }
            }
        }
        DragDropEvent::Leave => {
            coordinator.cancel_drag();
            let _ = window.emit(
                NATIVE_SKIN_DRAG_EVENT,
                NativeSkinDragPayload {
                    r#type: NativeSkinDragType::Leave,
                    eligible: false,
                    token: None,
                    position: None,
                    error: None,
                },
            );
        }
        _ => {}
    }
}

fn emit_drag(
    window: &WebviewWindow,
    drag_type: NativeSkinDragType,
    eligible: bool,
    token: Option<String>,
    position: PhysicalPosition<f64>,
    error: Option<&str>,
) {
    let error = match error {
        Some("Choose a PNG skin file.") => Some("Choose a PNG skin file."),
        Some("Skin file is too large; choose a PNG under 256 KiB.") => {
            Some("Skin file is too large; choose a PNG under 256 KiB.")
        }
        Some("Choose a valid PNG skin file.") => Some("Choose a valid PNG skin file."),
        Some("Skin image must be 64x64 or 64x32.") => {
            Some("Skin image must be 64x64 or 64x32.")
        }
        Some("Drop one PNG skin file.") => Some("Drop one PNG skin file."),
        Some("Another skin file is still being checked.") => {
            Some("Another skin file is still being checked.")
        }
        Some("Could not read dropped skin file.") => Some("Could not read dropped skin file."),
        Some(_) => Some("Could not read dropped skin file."),
        None => None,
    };
    let _ = window.emit(
        NATIVE_SKIN_DRAG_EVENT,
        NativeSkinDragPayload {
            r#type: drag_type,
            eligible,
            token,
            position: Some(NativeSkinDragPosition {
                x: position.x,
                y: position.y,
            }),
            error,
        },
    );
}

fn skin_drop_selection(paths: &[PathBuf]) -> NativeSkinDropSelection {
    let mut png_paths = paths
        .iter()
        .filter(|path| has_png_extension(path.as_path()));
    let Some(first) = png_paths.next() else {
        return NativeSkinDropSelection::None;
    };
    if png_paths.next().is_some() || paths.len() != 1 {
        return NativeSkinDropSelection::Multiple;
    }
    NativeSkinDropSelection::One(first.clone())
}

impl NativeSkinFileAdmission {
    pub(crate) fn admit(root_session: &AppRootSession, path: PathBuf) -> Result<Self, String> {
        if !path.is_absolute() {
            return Err("Could not read skin file.".to_string());
        }
        if !has_png_extension(&path) {
            return Err("Choose a PNG skin file.".to_string());
        }
        let parent = path
            .parent()
            .filter(|parent| parent.is_absolute())
            .ok_or_else(|| "Could not read skin file.".to_string())?;
        let file_name = path
            .file_name()
            .ok_or_else(|| "Could not read skin file.".to_string())?;
        let leaf = LeafName::new(file_name.to_os_string())
            .map_err(|_| "Could not read skin file.".to_string())?;
        let parent = root_session
            .admit_absolute_directory(parent)
            .map_err(|_| "Could not read skin file.".to_string())?;
        let file = parent
            .open_file(&leaf)
            .map_err(|_| "Could not read skin file.".to_string())?;
        let revision = file
            .revision()
            .map_err(|_| "Could not read skin file.".to_string())?;
        if revision.size() > SKIN_FILE_MAX_BYTES {
            return Err("Skin file is too large; choose a PNG under 256 KiB.".to_string());
        }
        let name = leaf
            .as_os_str()
            .to_str()
            .filter(|name| !name.trim().is_empty())
            .unwrap_or("skin.png")
            .to_string();
        Ok(Self {
            name,
            file,
            revision,
        })
    }

    pub(crate) fn read(self) -> Result<NativeSkinFile, String> {
        let Self {
            name,
            file,
            revision,
        } = self;
        let expected_size = usize::try_from(revision.size())
            .map_err(|_| "Skin file is too large; choose a PNG under 256 KiB.".to_string())?;
        let mut reader = match file.into_revision_reader(revision, SKIN_FILE_MAX_BYTES) {
            Ok(reader) => reader,
            Err(failure) => {
                let error = native_skin_read_error(failure.error());
                let (_, file, revision, _) = failure.into_parts();
                drop((file, revision));
                return Err(error);
            }
        };
        let mut bytes = Vec::with_capacity(expected_size);
        if let Err(error) = reader.read_to_end(&mut bytes) {
            let error = native_skin_read_error(&error);
            let (file, revision) = reader.cancel();
            drop((file, revision));
            return Err(error);
        }
        match reader.finish() {
            Ok(file) => drop(file),
            Err(failure) => {
                let error = native_skin_read_error(failure.error());
                let (file, revision) = failure.into_reader().cancel();
                drop((file, revision));
                return Err(error);
            }
        }
        if bytes.len() != expected_size {
            return Err("Skin file changed while it was being read. Choose it again.".to_string());
        }
        validate_skin_png(&bytes).map_err(native_skin_validation_error)?;
        Ok(NativeSkinFile { name, bytes })
    }
}

fn native_skin_read_error(error: &std::io::Error) -> String {
    if matches!(
        error.kind(),
        std::io::ErrorKind::InvalidData | std::io::ErrorKind::UnexpectedEof
    ) {
        "Skin file changed while it was being read. Choose it again.".to_string()
    } else {
        "Could not read skin file.".to_string()
    }
}

fn native_skin_validation_error(error: SkinPngValidationError) -> String {
    match error {
        SkinPngValidationError::TooLarge => {
            "Skin file is too large; choose a PNG under 256 KiB.".to_string()
        }
        SkinPngValidationError::InvalidPng => "Choose a valid PNG skin file.".to_string(),
        SkinPngValidationError::InvalidDimensions => {
            "Skin image must be 64x64 or 64x32.".to_string()
        }
    }
}

fn has_png_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("png"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axial_config::AppPaths;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_dir(name: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("test clock should be after unix epoch")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "axial-desktop-skin-{name}-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).expect("test dir");
        dir
    }

    fn test_root_session(root: &Path) -> Arc<AppRootSession> {
        let paths = AppPaths::from_root(root.to_path_buf()).expect("test app paths");
        Arc::new(paths.open_root_session().expect("test root session"))
    }

    fn test_skin_png(width: u32, height: u32) -> Vec<u8> {
        let pixels = vec![255; (width * height * 4) as usize];
        let mut bytes = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut bytes, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("write png header");
            writer.write_image_data(&pixels).expect("write png pixels");
        }
        bytes
    }

    #[test]
    fn native_skin_read_uses_the_admitted_file_revision() {
        let dir = test_dir("revision");
        let path = dir.join("player.png");
        let original = test_skin_png(64, 64);
        fs::write(&path, &original).expect("write original png");
        let root_session = test_root_session(&dir);
        let admission =
            NativeSkinFileAdmission::admit(&root_session, path.clone()).expect("admit skin");
        fs::write(&path, b"replacement").expect("replace png bytes");

        assert_eq!(
            admission.read(),
            Err("Skin file changed while it was being read. Choose it again.".to_string())
        );
        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }

    #[test]
    fn skin_drop_token_is_one_shot_and_forgery_does_not_consume_it() {
        let dir = test_dir("token");
        let path = dir.join("player.png");
        let png = test_skin_png(64, 64);
        fs::write(&path, &png).expect("write png");
        let root_session = test_root_session(&dir);
        let coordinator = NativeSkinDropCoordinator::new();
        let generation = coordinator.begin_drop();
        let token = coordinator
            .publish(
                generation,
                NativeSkinFileAdmission::admit(&root_session, path.clone()).expect("admit skin"),
            )
            .expect("publish token");

        assert_eq!(
            coordinator.consume("forged"),
            Err("Dropped skin file token is invalid.".to_string())
        );
        assert_eq!(
            coordinator.consume(&token).expect("consume token").bytes,
            png
        );
        assert_eq!(
            coordinator.consume(&token),
            Err("Dropped skin file is no longer available.".to_string())
        );
        drop(coordinator);
        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }

    #[test]
    fn enter_and_leave_do_not_cancel_an_issued_skin_drop_token() {
        let dir = test_dir("token-drag-lifecycle");
        let path = dir.join("player.png");
        let png = test_skin_png(64, 64);
        fs::write(&path, &png).expect("write png");
        let root_session = test_root_session(&dir);
        let coordinator = NativeSkinDropCoordinator::new();
        let generation = coordinator.begin_drop();
        let token = coordinator
            .publish(
                generation,
                NativeSkinFileAdmission::admit(&root_session, path.clone()).expect("admit skin"),
            )
            .expect("publish token");

        coordinator.begin_drag(false);
        coordinator.cancel_drag();

        assert_eq!(
            coordinator.consume(&token).expect("consume token").bytes,
            png
        );
        drop(coordinator);
        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }

    #[test]
    fn newer_failed_drop_revokes_the_previous_skin_drop_token() {
        let dir = test_dir("token-new-drop");
        let path = dir.join("player.png");
        fs::write(&path, test_skin_png(64, 64)).expect("write png");
        let root_session = test_root_session(&dir);
        let coordinator = NativeSkinDropCoordinator::new();
        let generation = coordinator.begin_drop();
        let token = coordinator
            .publish(
                generation,
                NativeSkinFileAdmission::admit(&root_session, path.clone()).expect("admit skin"),
            )
            .expect("publish token");

        coordinator.begin_drop();
        assert!(
            NativeSkinFileAdmission::admit(&root_session, dir.join("missing.png")).is_err()
        );

        assert_eq!(
            coordinator.consume(&token),
            Err("Dropped skin file is no longer available.".to_string())
        );
        drop(coordinator);
        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }

    #[test]
    fn expired_skin_drop_token_is_rejected_and_removed() {
        let dir = test_dir("expired-token");
        let path = dir.join("player.png");
        fs::write(&path, test_skin_png(64, 64)).expect("write png");
        let root_session = test_root_session(&dir);
        let coordinator = NativeSkinDropCoordinator::new();
        let generation = coordinator.begin_drop();
        let token = coordinator
            .publish(
                generation,
                NativeSkinFileAdmission::admit(&root_session, path.clone()).expect("admit skin"),
            )
            .expect("publish token");
        coordinator
            .shared
            .lock()
            .expect(SKIN_DROP_LOCK_INVARIANT)
            .pending
            .as_mut()
            .expect("pending token")
            .expires_at = Instant::now();

        assert_eq!(
            coordinator.consume(&token),
            Err("Dropped skin file expired. Drop it again.".to_string())
        );
        assert_eq!(
            coordinator.consume(&token),
            Err("Dropped skin file is no longer available.".to_string())
        );
        drop(coordinator);
        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }

    #[test]
    fn native_skin_read_rejects_non_png_content_and_oversized_input() {
        let dir = test_dir("validation");
        let root_session = test_root_session(&dir);
        let invalid = dir.join("invalid.png");
        fs::write(&invalid, b"not a png").expect("write invalid file");
        assert_eq!(
            NativeSkinFileAdmission::admit(&root_session, invalid.clone())
                .and_then(NativeSkinFileAdmission::read),
            Err("Choose a valid PNG skin file.".to_string())
        );

        let malformed = dir.join("malformed.png");
        fs::write(&malformed, b"\x89PNG\r\n\x1a\nmalformed").expect("write malformed png");
        assert_eq!(
            NativeSkinFileAdmission::admit(&root_session, malformed.clone())
                .and_then(NativeSkinFileAdmission::read),
            Err("Choose a valid PNG skin file.".to_string())
        );

        let bad_dimensions = dir.join("bad-dimensions.png");
        fs::write(&bad_dimensions, test_skin_png(32, 32)).expect("write bad dimensions png");
        assert_eq!(
            NativeSkinFileAdmission::admit(&root_session, bad_dimensions.clone())
                .and_then(NativeSkinFileAdmission::read),
            Err("Skin image must be 64x64 or 64x32.".to_string())
        );

        let oversized = dir.join("oversized.png");
        fs::write(&oversized, vec![0; (SKIN_FILE_MAX_BYTES + 1) as usize])
            .expect("write oversized file");
        assert_eq!(
            NativeSkinFileAdmission::admit(&root_session, oversized.clone())
                .and_then(NativeSkinFileAdmission::read),
            Err("Skin file is too large; choose a PNG under 256 KiB.".to_string())
        );

        drop(root_session);
        fs::remove_dir_all(dir).expect("cleanup dir");
    }
}
