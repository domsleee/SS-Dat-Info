//! The `.tasrec` file format: its JSON metadata header, the identity stamps it
//! carries, save/load, and the segment boundaries recorded alongside a take.

use super::RecoverySessionContext;
use serde::{Deserialize, Serialize};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

/// A segment boundary within a multi-segment recording.
#[derive(Clone, Serialize, Deserialize)]
pub struct Segment {
    pub name: String,
    pub start_tick: u32,
    pub end_tick: u32,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize)]
pub struct RecordingMetadata {
    pub version: u32,
    pub recorded_count: u32,
    pub timestamp: String,
    pub notes: String,
    #[serde(default)]
    pub segments: Vec<Segment>,
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
}

impl IdentityStamps {
    /// Capture the live DLL words, mapping its unknown sentinels to `None`
    /// (same rule `RecoverySessionContext::with_stamps` uses).
    pub fn from_live(state: &TasSharedState) -> Self {
        Self {
            renderer_id: (state.renderer_id != tas_shared::TAS_RENDERER_UNKNOWN)
                .then_some(state.renderer_id),
            fpu_control_word: (state.fpu_control_word != 0).then_some(state.fpu_control_word),
            rider_character: (state.rider_character != tas_shared::TAS_CHARACTER_UNKNOWN)
                .then_some(state.rider_character),
            rider_stance: (state.rider_stance != u32::MAX).then_some(state.rider_stance),
        }
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
    }
}
pub struct RecordingFile;

impl RecordingFile {
    pub fn save_with_segments(
        state: &TasSharedState,
        path: &std::path::Path,
        segments: &[Segment],
        identity: Option<&IdentityStamps>,
    ) -> Result<(), String> {
        let bytes = Self::encode_with_segments(state, segments, identity, None)?;
        tas_codec::save_atomic(path, &bytes)
    }

    pub(super) fn encode_with_segments(
        state: &TasSharedState,
        segments: &[Segment],
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
            notes: String::new(),
            segments: segments.to_vec(),
            recovery_session,
            renderer: (state.renderer_id != tas_shared::TAS_RENDERER_UNKNOWN)
                .then(|| tas_shared::renderer_name(state.renderer_id).to_string()),
            fpu_control_word: (state.fpu_control_word != 0).then_some(state.fpu_control_word),
            character: (state.rider_character != tas_shared::TAS_CHARACTER_UNKNOWN)
                .then(|| tas_shared::character_name(state.rider_character).to_string()),
            stance: (state.rider_stance != u32::MAX).then_some(state.rider_stance),
        };
        // A loaded or restored take carries its own identity: stamp the file
        // with the take's words, never the live game's. `None` halves stay
        // unknown rather than falling back to live.
        if let Some(identity) = identity {
            identity.apply_to_metadata(&mut meta);
        }

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

    pub fn load(
        state: &mut TasSharedState,
        path: &std::path::Path,
    ) -> Result<(u32, Vec<Segment>), String> {
        let data = tas_codec::read_bounded(path)?;
        let meta = Self::load_bytes(state, &data)?;
        Ok((meta.recorded_count, meta.segments))
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
        for i in count..TAS_MAX_TICKS {
            state.input_log[i] = 0;
        }

        // Zero the full coord buffer first: a legacy .tasrec with no coord
        // block must not inherit the previous recording's coords, which CONT,
        // drift analysis and re-saves all read.
        for i in 0..TAS_MAX_TICKS {
            state.rec_coords[i] = [0.0, 0.0, 0.0];
        }
        state.rec_coords[..count].copy_from_slice(&body.rec_coords);

        state.recorded_count = meta.recorded_count;

        Ok(meta)
    }
}

/// Tracks segment boundaries as the user records and continues.
pub struct SegmentTracker {
    pub segments: Vec<Segment>,
    current_start: Option<u32>,
    segment_counter: u32,
}

impl SegmentTracker {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            current_start: None,
            segment_counter: 0,
        }
    }

    /// Call when REC starts (initial or continue).
    pub fn on_rec_start(&mut self, from_tick: u32) {
        self.current_start = Some(from_tick);
    }

    /// Call when recording stops. Finalizes the current segment.
    pub fn on_rec_stop(&mut self, end_tick: u32) {
        if let Some(start) = self.current_start.take() {
            if end_tick > start {
                self.segment_counter += 1;
                self.segments.push(Segment {
                    name: format!("Segment {}", self.segment_counter),
                    start_tick: start,
                    end_tick,
                    timestamp: chrono::Local::now().to_rfc3339(),
                });
            }
        }
    }

    /// Restore segments from a loaded file.
    pub fn restore_from(&mut self, segments: Vec<Segment>) {
        self.segment_counter = segments.len() as u32;
        self.segments = segments;
        self.current_start = None;
    }

    /// Reset all segments (e.g., when starting a brand-new recording).
    pub fn clear(&mut self) {
        self.segments.clear();
        self.current_start = None;
        self.segment_counter = 0;
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
    fn segment_tracker_new_is_empty() {
        let tracker = SegmentTracker::new();
        assert!(tracker.segments.is_empty());
        assert!(tracker.current_start.is_none());
        assert_eq!(tracker.segment_counter, 0);
    }

    #[test]
    fn segment_tracker_rec_start_stop() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_start(0);
        tracker.on_rec_stop(100);

        assert_eq!(tracker.segments.len(), 1);
        assert_eq!(tracker.segments[0].name, "Segment 1");
        assert_eq!(tracker.segments[0].start_tick, 0);
        assert_eq!(tracker.segments[0].end_tick, 100);
        assert!(!tracker.segments[0].timestamp.is_empty());
    }

    #[test]
    fn segment_tracker_multi_segment() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_start(0);
        tracker.on_rec_stop(100);
        tracker.on_rec_start(100);
        tracker.on_rec_stop(250);
        tracker.on_rec_start(250);
        tracker.on_rec_stop(400);

        assert_eq!(tracker.segments.len(), 3);
        assert_eq!(tracker.segments[0].name, "Segment 1");
        assert_eq!(tracker.segments[1].name, "Segment 2");
        assert_eq!(tracker.segments[2].name, "Segment 3");
        assert_eq!(tracker.segments[2].start_tick, 250);
        assert_eq!(tracker.segments[2].end_tick, 400);
    }

    #[test]
    fn segment_tracker_stop_without_start_is_noop() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_stop(100);
        assert!(tracker.segments.is_empty());
    }

    #[test]
    fn segment_tracker_zero_length_segment_ignored() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_start(50);
        tracker.on_rec_stop(50); // end == start, not >
        assert!(tracker.segments.is_empty());
    }

    #[test]
    fn segment_tracker_clear_resets_all() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_start(0);
        tracker.on_rec_stop(100);
        tracker.on_rec_start(100);
        tracker.on_rec_stop(200);

        assert_eq!(tracker.segments.len(), 2);
        assert_eq!(tracker.segment_counter, 2);

        tracker.clear();
        assert!(tracker.segments.is_empty());
        assert!(tracker.current_start.is_none());
        assert_eq!(tracker.segment_counter, 0);

        // New segments after clear start from 1 again
        tracker.on_rec_start(0);
        tracker.on_rec_stop(50);
        assert_eq!(tracker.segments[0].name, "Segment 1");
    }

    #[test]
    fn segment_tracker_double_start_overwrites() {
        let mut tracker = SegmentTracker::new();
        tracker.on_rec_start(0);
        tracker.on_rec_start(50); // overwrite without stop
        tracker.on_rec_stop(100);

        assert_eq!(tracker.segments.len(), 1);
        assert_eq!(tracker.segments[0].start_tick, 50); // used the second start
    }

    #[test]
    fn truncated_load_does_not_replace_the_current_recording() {
        let path = unique_temp_path("truncated_coords", "tasrec");
        let mut original = tas_shared::zeroed_boxed();
        original.recorded_count = 2;
        original.input_log[0] = 3;
        original.rec_coords[0] = [1.0, 2.0, 3.0];
        RecordingFile::save_with_segments(&original, &path, &[], None).unwrap();
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

        RecordingFile::save_with_segments(&state, &path, &[], None).unwrap();

        let mut loaded = tas_shared::zeroed_boxed();
        let (count, segments) = RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(count, 10);
        assert!(segments.is_empty());
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
        assert!(RecordingFile::save_with_segments(&state, &path, &[], None).is_err());
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
        RecordingFile::save_with_segments(&state, &path, &[], None).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        assert_eq!(meta.stance, Some(0));
        assert_eq!(meta.rider_label().as_deref(), Some("Keith · regular"));

        let mut tracker = SegmentTracker::new();
        let mut live = tas_shared::zeroed_boxed();
        live.rider_character = tas_shared::TAS_CHARACTER_VINCENT;
        live.rider_stance = 0;
        let mut log = UiLog::default();
        assert!(load_recording_path(
            &mut live,
            &mut tracker,
            &mut log,
            &path
        ));
        assert!(log.lines().iter().any(|l| l.contains("WARNING")
            && l.contains("Keith · regular")
            && l.contains("Vincent · regular")));
        let mut other_stance = tas_shared::zeroed_boxed();
        other_stance.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        other_stance.rider_stance = 1;
        let mut log2 = UiLog::default();
        assert!(load_recording_path(
            &mut other_stance,
            &mut tracker,
            &mut log2,
            &path
        ));
        assert!(log2
            .lines()
            .iter()
            .any(|l| l.contains("WARNING") && l.contains("Keith · goofy")));
        let mut same = tas_shared::zeroed_boxed();
        same.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        same.rider_stance = 0;
        let mut quiet = UiLog::default();
        assert!(load_recording_path(
            &mut same,
            &mut tracker,
            &mut quiet,
            &path
        ));
        assert!(!quiet.lines().iter().any(|l| l.contains("WARNING")));

        let mut unstamped = tas_shared::zeroed_boxed();
        unstamped.recorded_count = 1;
        RecordingFile::save_with_segments(&unstamped, &path, &[], None).unwrap();
        assert_eq!(
            RecordingFile::read_metadata(&path).unwrap().rider_label(),
            None
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recording_file_stamps_and_reports_physics_mode() {
        let path = unique_temp_path("rec_physics", "tasrec");
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.renderer_id = tas_shared::TAS_RENDERER_OPENGL;
        state.fpu_control_word = 0x027F;
        RecordingFile::save_with_segments(&state, &path, &[], None).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.fpu_control_word, Some(0x027F));
        assert_eq!(meta.physics_label().as_deref(), Some("OpenGL/53-bit"));

        // Loading it into a DirectX/24-bit game warns; the same mode stays quiet.
        let mut tracker = SegmentTracker::new();
        let mut log = UiLog::default();
        let mut live = tas_shared::zeroed_boxed();
        live.renderer_id = tas_shared::TAS_RENDERER_DIRECTX6;
        live.fpu_control_word = 0x007F;
        assert!(load_recording_path(
            &mut live,
            &mut tracker,
            &mut log,
            &path
        ));
        assert!(log.lines().iter().any(|l| l.contains("WARNING")
            && l.contains("OpenGL/53-bit")
            && l.contains("DirectX6/24-bit")));
        let mut same = tas_shared::zeroed_boxed();
        same.renderer_id = tas_shared::TAS_RENDERER_OPENGL;
        same.fpu_control_word = 0x027F;
        let mut quiet = UiLog::default();
        assert!(load_recording_path(
            &mut same,
            &mut tracker,
            &mut quiet,
            &path
        ));
        assert!(!quiet.lines().iter().any(|l| l.contains("WARNING")));

        // A file saved before the stamp existed (or before the DLL sampled
        // the game thread) has no opinion.
        let mut unstamped = tas_shared::zeroed_boxed();
        unstamped.recorded_count = 1;
        RecordingFile::save_with_segments(&unstamped, &path, &[], None).unwrap();
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
        RecordingFile::save_with_segments(&first, &path, &[], None).unwrap();

        let mut second = tas_shared::zeroed_boxed();
        second.recorded_count = 3;
        second.input_log[..3].copy_from_slice(&[7, 8, 9]);
        RecordingFile::save_with_segments(&second, &path, &[], None).unwrap();

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

    #[test]
    fn recording_file_with_segments_round_trip() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 5;
        for i in 0..5 {
            state.input_log[i] = 0x04;
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 2.0];
        }

        let segments = vec![
            Segment {
                name: "Seg A".into(),
                start_tick: 0,
                end_tick: 3,
                timestamp: "2026-01-01T00:00:00Z".into(),
            },
            Segment {
                name: "Seg B".into(),
                start_tick: 3,
                end_tick: 5,
                timestamp: "2026-01-01T00:00:01Z".into(),
            },
        ];

        let path = unique_temp_path("rec_segments", "tasrec");
        RecordingFile::save_with_segments(&state, &path, &segments, None).unwrap();

        // Use RecordingFile::load() for a real round-trip (not manual JSON parse)
        let mut loaded = tas_shared::zeroed_boxed();
        let (count, loaded_segments) = RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(count, 5);
        assert_eq!(loaded_segments.len(), 2);
        assert_eq!(loaded_segments[0].name, "Seg A");
        assert_eq!(loaded_segments[0].start_tick, 0);
        assert_eq!(loaded_segments[0].end_tick, 3);
        assert_eq!(loaded_segments[1].name, "Seg B");
        assert_eq!(loaded_segments[1].start_tick, 3);
        assert_eq!(loaded_segments[1].end_tick, 5);
        assert_eq!(loaded.recorded_count, 5);
        for i in 0..5 {
            assert_eq!(loaded.input_log[i], 0x04);
            assert_eq!(loaded.rec_coords[i], [i as f32, 0.0, i as f32 * 2.0]);
        }
        // Trailing data should be zeroed
        assert_eq!(loaded.input_log[5], 0);

        // Also verify segment metadata is preserved in the file
        let data = std::fs::read(&path).unwrap();
        let meta_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let meta_json = std::str::from_utf8(&data[4..4 + meta_len]).unwrap();
        let meta: RecordingMetadata = serde_json::from_str(meta_json).unwrap();
        assert_eq!(meta.segments.len(), 2);
        assert_eq!(meta.segments[0].name, "Seg A");
        assert_eq!(meta.segments[1].name, "Seg B");

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
        };
        RecordingFile::save_with_segments(&live, &path, &[], Some(&identity)).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
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
        RecordingFile::save_with_segments(&live, &live_path, &[], None).unwrap();
        let live_meta = RecordingFile::read_metadata(&live_path).unwrap();
        assert_eq!(live_meta.renderer.as_deref(), Some("DirectX7"));
        assert_eq!(live_meta.character.as_deref(), Some("Vincent"));
    }
}
