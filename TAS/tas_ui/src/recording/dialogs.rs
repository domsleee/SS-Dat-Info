//! Recording folders and the Save / Load file dialogs.

use super::{IdentityStamps, RecordingFile, Segment, SegmentTracker};
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

/// Show the Save dialog and write the recording. `level` is the track the
/// recording was made on, supplied by the caller rather than read live: by
/// the time the user clicks Save the game may be in its post-run dialog,
/// where the level reads unknown.
pub fn save_dialog_with_segments(
    state: &TasSharedState,
    segments: &[Segment],
    log: &mut UiLog,
    level: Option<&str>,
    identity: Option<&IdentityStamps>,
) -> Option<PathBuf> {
    // Default name: `<level>-<race cs>` (e.g. FE-5876). The level filter keys
    // off the saved name and folder, so an unknown level falls back to a
    // time-only name in the root folder rather than guessing.
    let race_cs = crate::level::race_centiseconds(&state.rec_coords, state.recorded_count);
    let default_name = crate::level::default_recording_name(level, race_cs);
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Save TAS Recording")
        .set_directory(recordings_dir_for_level(level))
        .set_file_name(default_name)
        .add_filter("TAS Recording", &["tasrec"])
        .save_file()
    {
        match RecordingFile::save_with_segments(state, &path, segments, identity) {
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
    let level = crate::level::level_code_from_id(level_id);
    rfd::FileDialog::new()
        .set_title("Load TAS Recording")
        .set_directory(load_dir_for_level(level))
        .add_filter("TAS Recording", &["tasrec"])
        .pick_file()
}

/// Load a previously-picked recording into shared state. The caller must have
/// already stopped any active REC/PLAY (the DLL must be OFF) — this overwrites
/// the whole input/coords buffer.
pub fn load_recording_path(
    state: &mut TasSharedState,
    tracker: &mut SegmentTracker,
    log: &mut UiLog,
    path: &std::path::Path,
) -> bool {
    match RecordingFile::load(state, path) {
        Ok((count, segments)) => {
            let seg_count = segments.len();
            tracker.restore_from(segments);
            // The take carries the physics mode it was recorded under; the
            // live one comes from the DLL. Different renderers round the sim
            // differently (24-bit DirectX vs 53-bit OpenGL), so say so now
            // rather than letting the replay drift "mysteriously".
            if let Ok(meta) = RecordingFile::read_metadata(path) {
                let live =
                    tas_shared::physics_mode_label(state.renderer_id, state.fpu_control_word);
                if let (Some(stamp), Some(live)) = (meta.physics_label(), live) {
                    if stamp != live {
                        log.push(format!(
                            "WARNING: recording was made under {} but the game is running {}: \
                             the physics round differently, this replay will not be bit-exact",
                            stamp, live
                        ));
                    }
                }
                // Same for who is riding: a Keith take does not line up under
                // Vincent, and the stance changes the trajectory as well.
                let live_rider = tas_shared::rider_label(state.rider_character, state.rider_stance);
                if let (Some(stamp), Some(live)) = (meta.rider_label(), live_rider) {
                    if stamp != live {
                        log.push(format!(
                            "WARNING: recording was made as {} but the rider is {}: \
                             a different character or stance has different physics, \
                             this replay will not line up",
                            stamp, live
                        ));
                    }
                }
            }
            log.push(format!(
                "Loaded {} ticks, {} segments from {}",
                count,
                seg_count,
                path.display()
            ));
            true
        }
        Err(e) => {
            log.push(format!("Load error: {}", e));
            false
        }
    }
}
