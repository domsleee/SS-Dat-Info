//! The `.tasrec` file format: its JSON metadata header, the identity stamps it
//! carries, and save/load.

use super::RecoverySessionContext;
use serde::{Deserialize, Serialize};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

#[derive(Default, Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub version: u32,
    pub recorded_count: u32,
    pub timestamp: String,
    pub notes: String,
    /// Renderer plugin the take was recorded under (`OpenGL`, `DirectX6`, ...).
    #[serde(default)]
    pub renderer: Option<String>,
    /// Raw x87 control word of the game thread at save time: 0x007F = 24-bit
    /// (DirectX 6/7), 0x027F = 53-bit (OpenGL/Software2). The physics round
    /// differently, so a replay under the other mode diverges.
    #[serde(default)]
    pub fpu_control_word: Option<u32>,
    /// Character the take was recorded as (`Keith`, `Vincent`, ...). The
    /// physics differ per character, so a replay as someone else diverges.
    #[serde(default)]
    pub character: Option<String>,
    /// The loadout's stance word at save time (0 / 1); the stance changes
    /// the trajectory too. `None` = unknown / pre-stamp file.
    #[serde(default)]
    pub stance: Option<u32>,
    /// TAS_INPUT_MODEL_* of the input: 1 = the held keys each tick's physics
    /// read. `None` = a file from before the held model, whose masks were
    /// injected a tick ahead.
    #[serde(default)]
    pub input_model: Option<u32>,
    /// Leading ticks whose recorded trajectory the input produced; an input
    /// edit cuts it at the first changed tick. `None` = the whole take.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trajectory_ticks: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery_session: Option<RecoverySessionContext>,
}

impl RecordingMetadata {
    /// Canonical rider stamp (see `tas_shared::rider_label`), `None` for
    /// files saved before the stamp existed.
    pub fn rider_label(&self) -> Option<String> {
        let character = self.character.as_deref()?;
        let stance = self.stance.unwrap_or(u32::MAX);
        let id = tas_shared::character_id_from_name(character);
        if id == tas_shared::TAS_CHARACTER_UNKNOWN {
            return None;
        }
        tas_shared::rider_label(id, stance)
    }

    /// Canonical physics-mode stamp (see `tas_shared::physics_mode_label`);
    /// `None` for files saved before the stamp existed.
    pub fn physics_label(&self) -> Option<String> {
        let id = self
            .renderer
            .as_deref()
            .map(tas_shared::renderer_id_from_name)
            .unwrap_or(tas_shared::TAS_RENDERER_UNKNOWN);
        tas_shared::physics_mode_label(id, self.fpu_control_word.unwrap_or(0))
    }
}

/// Raw identity stamps of the take a buffer/file/entry holds: the DLL words
/// the physics and rider labels derive from. `None` = unknown, and unknown
/// is carried as unknown — never backfilled from the live game (a restored
/// Keith take saved under a live Vincent session must keep saying Keith,
/// and a pre-stamp take must keep saying nothing).
#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct IdentityStamps {
    #[serde(default)]
    pub renderer_id: Option<u32>,
    #[serde(default)]
    pub fpu_control_word: Option<u32>,
    #[serde(default)]
    pub rider_character: Option<u32>,
    #[serde(default)]
    pub rider_stance: Option<u32>,
    /// TAS_INPUT_MODEL_* of the take's input; None = a take from before the
    /// held model, which is injected.
    #[serde(default)]
    pub input_model: Option<u32>,
    /// Leading ticks whose recorded trajectory the input produced (see
    /// `RecordingMetadata::trajectory_ticks`). `None` = the whole take.
    #[serde(default)]
    pub trajectory_ticks: Option<u32>,
}

impl IdentityStamps {
    /// Capture the live DLL words, mapping its unknown sentinels to `None`.
    pub fn from_live(state: &TasSharedState) -> Self {
        Self {
            renderer_id: (state.renderer_id != tas_shared::TAS_RENDERER_UNKNOWN)
                .then_some(state.renderer_id),
            fpu_control_word: (state.fpu_control_word != 0).then_some(state.fpu_control_word),
            rider_character: (state.rider_character != tas_shared::TAS_CHARACTER_UNKNOWN)
                .then_some(state.rider_character),
            rider_stance: (state.rider_stance != u32::MAX).then_some(state.rider_stance),
            input_model: Some(state.input_model),
            trajectory_ticks: None,
        }
    }

    /// The model PLAY must replay this take with.
    pub fn input_model_or_injected(&self) -> u32 {
        self.input_model
            .unwrap_or(tas_shared::TAS_INPUT_MODEL_INJECTED)
    }

    /// Recover the stamps from a file's metadata header. Exact: the header
    /// stores the same names/words, so a load → save round trip preserves
    /// them bit-for-bit.
    pub fn from_metadata(meta: &RecordingMetadata) -> Self {
        Self {
            renderer_id: meta
                .renderer
                .as_deref()
                .map(tas_shared::renderer_id_from_name)
                .filter(|&id| id != tas_shared::TAS_RENDERER_UNKNOWN),
            fpu_control_word: meta.fpu_control_word,
            rider_character: meta
                .character
                .as_deref()
                .map(tas_shared::character_id_from_name)
                .filter(|&id| id != tas_shared::TAS_CHARACTER_UNKNOWN),
            rider_stance: meta.stance,
            input_model: meta.input_model,
            trajectory_ticks: meta.trajectory_ticks,
        }
    }

    /// Stamp a file header with these instead of the live DLL words. `None`
    /// fields write unknown — they never fall back to live.
    pub fn apply_to_metadata(&self, meta: &mut RecordingMetadata) {
        meta.renderer = self.renderer_id.and_then(|id| {
            (id != tas_shared::TAS_RENDERER_UNKNOWN)
                .then(|| tas_shared::renderer_name(id).to_string())
        });
        meta.fpu_control_word = self.fpu_control_word.filter(|&v| v != 0);
        meta.character = self.rider_character.and_then(|id| {
            (id != tas_shared::TAS_CHARACTER_UNKNOWN)
                .then(|| tas_shared::character_name(id).to_string())
        });
        meta.stance = self.rider_stance.filter(|&v| v != u32::MAX);
        meta.input_model = self.input_model;
        meta.trajectory_ticks = self.trajectory_ticks;
    }

    /// The watcher's limit for this take: u32::MAX = the whole trajectory.
    pub fn trajectory_limit(&self) -> u32 {
        self.trajectory_ticks.unwrap_or(u32::MAX)
    }
}
pub struct RecordingFile;

impl RecordingFile {
    pub fn save(
        state: &TasSharedState,
        path: &std::path::Path,
        identity: Option<&IdentityStamps>,
    ) -> Result<(), String> {
        let bytes = Self::encode(state, identity, None)?;
        tas_codec::save_atomic(path, &bytes)
    }

    pub(super) fn encode(
        state: &TasSharedState,
        identity: Option<&IdentityStamps>,
        recovery_session: Option<RecoverySessionContext>,
    ) -> Result<Vec<u8>, String> {
        let count = state.recorded_count as usize;
        if count == 0 {
            return Err("Nothing recorded".into());
        }
        if count > TAS_MAX_TICKS {
            return Err(format!("Recording too long: {} ticks", count));
        }

        let mut meta = RecordingMetadata {
            version: state.version,
            recorded_count: state.recorded_count,
            timestamp: chrono::Local::now().to_rfc3339(),
            recovery_session,
            ..Default::default()
        };
        // A loaded or restored take carries its own identity: stamp the file
        // with the take's words, never the live game's. `None` halves stay
        // unknown rather than falling back to live. A fresh take is stamped
        // with the live words.
        identity
            .cloned()
            .unwrap_or_else(|| IdentityStamps::from_live(state))
            .apply_to_metadata(&mut meta);

        let meta_json = serde_json::to_string_pretty(&meta).map_err(|e| format!("{}", e))?;
        tas_codec::encode(
            meta_json.as_bytes(),
            &state.input_log[..count],
            &state.rec_coords[..count],
        )
    }

    /// Parse only the JSON header of a `.tasrec` (same bounds as `load`).
    pub fn read_metadata(path: &std::path::Path) -> Result<RecordingMetadata, String> {
        let data = tas_codec::read_bounded(path)?;
        let meta_len = tas_codec::header_meta_len(&data)?;
        serde_json::from_slice::<RecordingMetadata>(&data[4..4 + meta_len])
            .map_err(|e| format!("{}", e))
    }

    /// Load a take; returns its header. Files from before segments were
    /// retired still load: serde skips their `segments` list.
    pub fn load(
        state: &mut TasSharedState,
        path: &std::path::Path,
    ) -> Result<RecordingMetadata, String> {
        Self::load_bytes(state, &tas_codec::read_bounded(path)?)
    }

    pub(super) fn load_bytes(
        state: &mut TasSharedState,
        data: &[u8],
    ) -> Result<RecordingMetadata, String> {
        let meta_len = tas_codec::header_meta_len(data)?;
        let meta: RecordingMetadata =
            serde_json::from_slice(&data[4..4 + meta_len]).map_err(|e| format!("{}", e))?;

        let count = meta.recorded_count as usize;
        if count > TAS_MAX_TICKS {
            return Err(format!("Recording too long: {} ticks", count));
        }
        // Legacy files may end after the input log: coordinates stay zeroed.
        let body = tas_codec::decode_body(data, meta_len, count, false)?;

        // Clear and load input log
        state.input_log[..count].copy_from_slice(&body.input_log);
        state.input_log[count..].fill(0);

        // Zero the full coord buffer first: a legacy .tasrec with no coord
        // block must not inherit the previous recording's coords, which CONT,
        // drift analysis and re-saves all read.
        state.rec_coords.fill([0.0; 3]);
        state.rec_coords[..count].copy_from_slice(&body.rec_coords);

        state.recorded_count = meta.recorded_count;

        Ok(meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recording::load_recording_path;
    use crate::recording::test_support::*;
    use crate::ui_log::UiLog;
    use std::fs::File;
    use tas_codec::MAX_TASREC_BYTES;

    #[test]
    fn truncated_load_does_not_replace_the_current_recording() {
        let path = unique_temp_path("truncated_coords", "tasrec");
        let mut original = tas_shared::zeroed_boxed();
        original.recorded_count = 2;
        original.input_log[0] = 3;
        original.rec_coords[0] = [1.0, 2.0, 3.0];
        RecordingFile::save(&original, &path, None).unwrap();
        let mut data = std::fs::read(&path).unwrap();
        data.pop();
        std::fs::write(&path, data).unwrap();
        original.input_log[0] = 9;
        assert!(RecordingFile::load(&mut original, &path).is_err());
        assert_eq!(original.recorded_count, 2);
        assert_eq!(original.input_log[0], 9);
        assert_eq!(original.rec_coords[0], [1.0, 2.0, 3.0]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recording_file_round_trip() {
        let mut state = tas_shared::zeroed_boxed();
        state.version = 4;
        state.recorded_count = 10;

        for i in 0..10 {
            state.input_log[i] = (i as u8) & 0x3F;
            state.rec_coords[i] = [i as f32 * 1.0, i as f32 * 2.0, i as f32 * 3.0];
        }

        let path = unique_temp_path("rec_rt", "tasrec");

        RecordingFile::save(&state, &path, None).unwrap();

        let mut loaded = tas_shared::zeroed_boxed();
        let count = RecordingFile::load(&mut loaded, &path)
            .unwrap()
            .recorded_count;
        assert_eq!(count, 10);
        assert_eq!(loaded.recorded_count, 10);

        for i in 0..10 {
            assert_eq!(loaded.input_log[i], (i as u8) & 0x3F);
            assert_eq!(
                loaded.rec_coords[i],
                [i as f32, i as f32 * 2.0, i as f32 * 3.0]
            );
        }
        // Trailing data should be zeroed
        assert_eq!(loaded.input_log[10], 0);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn recording_file_save_empty_errors() {
        let state = tas_shared::zeroed_boxed();
        let path = unique_temp_path("rec_empty", "tasrec");
        assert!(RecordingFile::save(&state, &path, None).is_err());
    }

    /// The rider stamp travels with the file: a Keith take loaded while
    /// Vincent is on the board warns (the physics differ per character), so
    /// does the other stance; the same rider stays quiet; an unstamped file
    /// has no opinion.
    #[test]
    fn recording_file_stamps_and_checks_the_rider() {
        let path = unique_temp_path("rec_rider", "tasrec");
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        state.rider_stance = 0;
        RecordingFile::save(&state, &path, None).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        assert_eq!(meta.stance, Some(0));
        assert_eq!(meta.rider_label().as_deref(), Some("Keith · regular"));

        let mut live = tas_shared::zeroed_boxed();
        live.rider_character = tas_shared::TAS_CHARACTER_VINCENT;
        live.rider_stance = 0;
        let mut log = UiLog::default();
        assert!(load_recording_path(&mut live, &mut log, &path).is_some());
        assert!(log.lines().iter().any(|l| l.contains("WARNING")
            && l.contains("Keith · regular")
            && l.contains("Vincent · regular")));
        let mut other_stance = tas_shared::zeroed_boxed();
        other_stance.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        other_stance.rider_stance = 1;
        let mut log2 = UiLog::default();
        assert!(load_recording_path(&mut other_stance, &mut log2, &path).is_some());
        assert!(log2
            .lines()
            .iter()
            .any(|l| l.contains("WARNING") && l.contains("Keith · goofy")));
        let mut same = tas_shared::zeroed_boxed();
        same.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        same.rider_stance = 0;
        let mut quiet = UiLog::default();
        assert!(load_recording_path(&mut same, &mut quiet, &path).is_some());
        assert!(!quiet.lines().iter().any(|l| l.contains("WARNING")));

        let mut unstamped = tas_shared::zeroed_boxed();
        unstamped.recorded_count = 1;
        RecordingFile::save(&unstamped, &path, None).unwrap();
        assert_eq!(
            RecordingFile::read_metadata(&path).unwrap().rider_label(),
            None
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn the_input_model_is_stamped_and_absent_means_injected() {
        let path = unique_temp_path("rec_model", "tasrec");
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.input_model = tas_shared::TAS_INPUT_MODEL_HELD;
        RecordingFile::save(&state, &path, None).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.input_model, Some(tas_shared::TAS_INPUT_MODEL_HELD));
        let stamps = IdentityStamps::from_metadata(&meta);
        assert_eq!(
            stamps.input_model_or_injected(),
            tas_shared::TAS_INPUT_MODEL_HELD
        );

        // A loaded take re-saved keeps its own model, not the live one.
        state.input_model = tas_shared::TAS_INPUT_MODEL_INJECTED;
        RecordingFile::save(&state, &path, Some(&stamps)).unwrap();
        let resaved = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(resaved.input_model, Some(tas_shared::TAS_INPUT_MODEL_HELD));

        // A file from before the field replays as injected.
        let legacy: RecordingMetadata =
            serde_json::from_str(r#"{"version":53,"recorded_count":2,"timestamp":"t","notes":""}"#)
                .unwrap();
        assert_eq!(
            IdentityStamps::from_metadata(&legacy).input_model_or_injected(),
            tas_shared::TAS_INPUT_MODEL_INJECTED
        );
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn recording_file_stamps_and_reports_physics_mode() {
        let path = unique_temp_path("rec_physics", "tasrec");
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.renderer_id = tas_shared::TAS_RENDERER_OPENGL;
        state.fpu_control_word = 0x027F;
        RecordingFile::save(&state, &path, None).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.fpu_control_word, Some(0x027F));
        assert_eq!(meta.physics_label().as_deref(), Some("OpenGL/53-bit"));

        // Loading it into a DirectX/24-bit game warns; the same mode stays quiet.
        let mut log = UiLog::default();
        let mut live = tas_shared::zeroed_boxed();
        live.renderer_id = tas_shared::TAS_RENDERER_DIRECTX6;
        live.fpu_control_word = 0x007F;
        assert!(load_recording_path(&mut live, &mut log, &path).is_some());
        assert!(log.lines().iter().any(|l| l.contains("WARNING")
            && l.contains("OpenGL/53-bit")
            && l.contains("DirectX6/24-bit")));
        let mut same = tas_shared::zeroed_boxed();
        same.renderer_id = tas_shared::TAS_RENDERER_OPENGL;
        same.fpu_control_word = 0x027F;
        let mut quiet = UiLog::default();
        assert!(load_recording_path(&mut same, &mut quiet, &path).is_some());
        assert!(!quiet.lines().iter().any(|l| l.contains("WARNING")));

        // A file saved before the stamp existed (or before the DLL sampled
        // the game thread) has no opinion.
        let mut unstamped = tas_shared::zeroed_boxed();
        unstamped.recorded_count = 1;
        RecordingFile::save(&unstamped, &path, None).unwrap();
        assert_eq!(
            RecordingFile::read_metadata(&path).unwrap().physics_label(),
            None
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recording_file_atomically_replaces_existing_save() {
        let path = unique_temp_path("rec_replace", "tasrec");
        let mut first = tas_shared::zeroed_boxed();
        first.recorded_count = 2;
        first.input_log[..2].copy_from_slice(&[1, 2]);
        RecordingFile::save(&first, &path, None).unwrap();

        let mut second = tas_shared::zeroed_boxed();
        second.recorded_count = 3;
        second.input_log[..3].copy_from_slice(&[7, 8, 9]);
        RecordingFile::save(&second, &path, None).unwrap();

        let mut loaded = tas_shared::zeroed_boxed();
        RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(loaded.recorded_count, 3);
        assert_eq!(&loaded.input_log[..3], &[7, 8, 9]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recording_file_load_truncated_errors() {
        let path = unique_temp_path("rec_trunc", "tasrec");
        std::fs::write(&path, [0u8; 2]).unwrap(); // Too small
        let mut state = tas_shared::zeroed_boxed();
        assert!(RecordingFile::load(&mut state, &path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn recording_file_rejects_oversized_file_before_reading_it() {
        let path = unique_temp_path("rec_oversized", "tasrec");
        let file = File::create(&path).unwrap();
        file.set_len(MAX_TASREC_BYTES + 1).unwrap();
        drop(file);
        let mut state = tas_shared::zeroed_boxed();
        let error = match RecordingFile::load(&mut state, &path) {
            Ok(_) => panic!("oversized recording unexpectedly loaded"),
            Err(error) => error,
        };
        assert!(error.contains("too large"), "unexpected error: {error}");
        let _ = std::fs::remove_file(path);
    }

    /// Files from before segments were retired still load, and a new save
    /// no longer writes the list.
    #[test]
    fn a_legacy_segments_list_loads_and_is_not_written() {
        let header = r#"{"version":56,"recorded_count":2,"timestamp":"","notes":"",
            "segments":[{"name":"Segment 1","start_tick":0,"end_tick":2,"timestamp":""}]}"#;
        let bytes =
            tas_codec::encode(header.as_bytes(), &[0x04, 0x05], &[[1.0, 2.0, 3.0]; 2]).unwrap();
        let path = unique_temp_path("legacy_segments", "tasrec");
        std::fs::write(&path, bytes).unwrap();
        let mut state = tas_shared::zeroed_boxed();
        assert_eq!(
            RecordingFile::load(&mut state, &path)
                .unwrap()
                .recorded_count,
            2
        );
        assert_eq!(state.input_log[..2], [0x04, 0x05]);

        RecordingFile::save(&state, &path, None).unwrap();
        let data = std::fs::read(&path).unwrap();
        let meta_len = tas_codec::header_meta_len(&data).unwrap();
        let header = std::str::from_utf8(&data[4..4 + meta_len]).unwrap();
        assert!(!header.contains("segments"), "{header}");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn save_uses_take_identity_not_live_state() {
        let path = unique_temp_path("rec_identity", "tasrec");
        // Live game runs Vincent/DirectX7; the take in the buffer is a
        // Keith/OpenGL recording whose stance is unknown.
        let mut live = state_with_ticks(8);
        live.renderer_id = tas_shared::TAS_RENDERER_DIRECTX7;
        live.fpu_control_word = 0x007F;
        live.rider_character = tas_shared::TAS_CHARACTER_VINCENT;
        live.rider_stance = 0;
        let identity = IdentityStamps {
            renderer_id: Some(tas_shared::TAS_RENDERER_OPENGL),
            fpu_control_word: Some(0x027F),
            rider_character: Some(tas_shared::TAS_CHARACTER_KEITH),
            rider_stance: None,
            input_model: None,
            trajectory_ticks: Some(5),
        };
        RecordingFile::save(&live, &path, Some(&identity)).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(
            meta.trajectory_ticks,
            Some(5),
            "an edited take stays edited"
        );
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.fpu_control_word, Some(0x027F));
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        assert_eq!(
            meta.stance, None,
            "unknown stance stays unknown, not live 0"
        );
        assert_eq!(meta.physics_label().as_deref(), Some("OpenGL/53-bit"));
        // The saved header recovers the same stamps: load → save is exact.
        assert_eq!(IdentityStamps::from_metadata(&meta), identity);
        // Without an override the live words are stamped.
        let live_path = unique_temp_path("rec_identity_live", "tasrec");
        RecordingFile::save(&live, &live_path, None).unwrap();
        let live_meta = RecordingFile::read_metadata(&live_path).unwrap();
        assert_eq!(live_meta.renderer.as_deref(), Some("DirectX7"));
        assert_eq!(live_meta.character.as_deref(), Some("Vincent"));
    }
}
