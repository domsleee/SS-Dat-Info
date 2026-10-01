//! Recording folders and the Save / Load file dialogs.

use super::file::RecordingMetadata;
use super::{IdentityStamps, RecordingFile};
use crate::ui_log::UiLog;
use std::path::PathBuf;
use tas_shared::TasSharedState;

/// Per-game recordings folder: `<data_root>/recordings` (created on demand).
/// `<data_root>` is the game folder when deployed (see
/// `default_history_root_dir`), so each game install's recordings stay separate.
pub fn recordings_dir() -> PathBuf {
    let dir = crate::settings::data_root_dir().join("recordings");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Per-LEVEL recordings folder: `recordings/<code>` (e.g. `recordings/FE`),
/// created on demand. Unknown level → the flat recordings root.
pub fn recordings_dir_for_level(level: Option<&str>) -> PathBuf {
    match level {
        Some(code) => {
            let dir = recordings_dir().join(code);
            let _ = std::fs::create_dir_all(&dir);
            dir
        }
        None => recordings_dir(),
    }
}

/// Where the Load dialog should open: the current level's folder when it
/// already holds recordings, else the flat root (where pre-per-level saves
/// live — don't hide them behind an empty subfolder).
fn load_dir_for_level(level: Option<&str>) -> PathBuf {
    if let Some(code) = level {
        let dir = recordings_dir().join(code);
        let has_recs = std::fs::read_dir(&dir)
            .map(|entries| {
                entries.flatten().any(|e| {
                    e.path()
                        .extension()
                        .is_some_and(|ext| ext.eq_ignore_ascii_case("tasrec"))
                })
            })
            .unwrap_or(false);
        if has_recs {
            return dir;
        }
    }
    recordings_dir()
}

/// The Save dialog, in `level`'s folder. None = the user cancelled.
pub fn pick_save_path(level: Option<&str>, default_name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Save TAS Recording")
        .set_directory(recordings_dir_for_level(level))
        .set_file_name(default_name)
        .add_filter("TAS Recording", &["tasrec"])
        .save_file()
}

/// Show the Save dialog and write the recording. `level` is the track the
/// recording was made on, supplied by the caller rather than read live: by
/// the time the user clicks Save the game may be in its post-run dialog,
/// where the level reads unknown.
pub fn save_dialog(
    host: &dyn crate::host::Host,
    state: &TasSharedState,
    log: &mut UiLog,
    level: Option<&str>,
    identity: Option<&IdentityStamps>,
) -> Option<PathBuf> {
    // Default name: `<level>-<race cs>` (e.g. FE-5876). The level filter keys
    // off the saved name and folder, so an unknown level falls back to a
    // time-only name in the root folder rather than guessing.
    let race_cs = crate::level::race_centiseconds(&state.rec_coords, state.recorded_count);
    let default_name = crate::level::default_recording_name(level, race_cs);
    if let Some(path) = host.pick_save_path(level, &default_name) {
        match RecordingFile::save(state, &path, identity) {
            Ok(()) => {
                log.push(format!("Saved recording to {}", path.display()));
                return Some(path);
            }
            Err(e) => {
                log.push(format!("Save error: {}", e));
            }
        }
    }
    None
}

/// Show the Load file dialog and return the chosen path WITHOUT loading it.
/// Split from the load so the caller can stop an active recording after the
/// user commits to a file, not before a dialog they might cancel. `level_id` selects the starting folder. None = the user cancelled.
pub fn pick_recording_path(level_id: u32) -> Option<PathBuf> {
    let level = tas_shared::level::code_from_id(level_id);
    rfd::FileDialog::new()
        .set_title("Load TAS Recording")
        .set_directory(load_dir_for_level(level))
        .add_filter("TAS Recording", &["tasrec"])
        .pick_file()
}

/// Load a previously-picked recording into shared state and return its
/// header, `None` when it failed to load. The caller must have already
/// stopped any active REC/PLAY (the DLL must be OFF) — this overwrites the
/// whole input/coords buffer.
pub fn load_recording_path(
    state: &mut TasSharedState,
    log: &mut UiLog,
    path: &std::path::Path,
) -> Option<RecordingMetadata> {
    match RecordingFile::load(state, path) {
        Ok(meta) => {
            // The take carries the physics mode and rider it was recorded
            // under; the live ones come from the DLL. Say so now rather than
            // letting the replay drift "mysteriously".
            let live_physics =
                tas_shared::physics_mode_label(state.renderer_id, state.fpu_control_word);
            let live_rider = tas_shared::rider_label(state.rider_character, state.rider_stance);
            warn_identity_mismatch(
                log,
                (meta.physics_label().as_deref(), live_physics.as_deref()),
                (meta.rider_label().as_deref(), live_rider.as_deref()),
            );
            log.push(format!(
                "Loaded {} ticks from {}",
                meta.recorded_count,
                path.display()
            ));
            Some(meta)
        }
        Err(e) => {
            log.push(format!("Load error: {}", e));
            None
        }
    }
}

/// Warn when the take's physics mode or rider differs from the live game's,
/// each given as `(take, live)`; an unknown side has no opinion. Renderers
/// round the sim differently (24-bit DirectX vs 53-bit OpenGL), a Keith take
/// does not line up under Vincent, and the stance changes the trajectory too.
pub fn warn_identity_mismatch(
    log: &mut UiLog,
    physics: (Option<&str>, Option<&str>),
    rider: (Option<&str>, Option<&str>),
) {
    if let (Some(take), Some(live)) = physics {
        if take != live {
            log.push(format!(
                "WARNING: this take was recorded under {take} but the game is running {live}: \
                 the physics round differently, a replay will not be bit-exact"
            ));
        }
    }
    if let (Some(take), Some(live)) = rider {
        if take != live {
            log.push(format!(
                "WARNING: this take was recorded as {take} but the rider is {live}: \
                 a different character or stance has different physics, a replay will not \
                 line up"
            ));
        }
    }
}
