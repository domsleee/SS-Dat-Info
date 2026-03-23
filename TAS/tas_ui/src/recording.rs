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
    pub inject_mode: u32,
    pub force_fixed_tick: u32,
    pub force_direct: u32,
    pub input_source: u32,
    pub max_drift_x: f32,
    pub max_drift_z: f32,
    pub timestamp: String,
    pub notes: String,
    #[serde(default)]
    pub segments: Vec<Segment>,
}

pub struct RecordingFile;

impl RecordingFile {
    #[allow(dead_code)]
    pub fn save(state: &TasSharedState, path: &std::path::Path) -> Result<(), String> {
        Self::save_with_segments(state, path, &[])
    }

    pub fn save_with_segments(
        state: &TasSharedState,
        path: &std::path::Path,
        segments: &[Segment],
    ) -> Result<(), String> {
        let count = state.recorded_count as usize;
        if count == 0 {
            return Err("Nothing recorded".into());
        }

        let meta = RecordingMetadata {
            version: state.version,
            recorded_count: state.recorded_count,
            inject_mode: state.inject_mode,
            force_fixed_tick: state.force_fixed_tick,
            force_direct: state.force_direct,
            input_source: state.input_source,
            max_drift_x: state.max_drift_x,
            max_drift_z: state.max_drift_z,
            timestamp: chrono::Local::now().to_rfc3339(),
            notes: String::new(),
            segments: segments.to_vec(),
        };

        let meta_json = serde_json::to_string_pretty(&meta).map_err(|e| format!("{}", e))?;

        // File format: [4-byte meta_len][JSON metadata][input_log bytes][rec_coords floats]
        let meta_bytes = meta_json.as_bytes();
        let meta_len = meta_bytes.len() as u32;

        let mut data = Vec::new();
        data.extend_from_slice(&meta_len.to_le_bytes());
        data.extend_from_slice(meta_bytes);
        data.extend_from_slice(&state.input_log[..count]);

        // Write rec_coords as raw f32 bytes
        for i in 0..count {
            for j in 0..3 {
                data.extend_from_slice(&state.rec_coords[i][j].to_le_bytes());
            }
        }

        std::fs::write(path, &data).map_err(|e| format!("{}", e))
    }

    pub fn load(
        state: &mut TasSharedState,
        path: &std::path::Path,
    ) -> Result<(u32, Vec<Segment>), String> {
        let data = std::fs::read(path).map_err(|e| format!("{}", e))?;
        if data.len() < 4 {
            return Err("File too small".into());
        }

        let meta_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        if data.len() < 4 + meta_len {
            return Err("Truncated metadata".into());
        }

        let meta_json =
            std::str::from_utf8(&data[4..4 + meta_len]).map_err(|e| format!("{}", e))?;
        let meta: RecordingMetadata =
            serde_json::from_str(meta_json).map_err(|e| format!("{}", e))?;

        let count = meta.recorded_count as usize;
        if count > TAS_MAX_TICKS {
            return Err(format!("Recording too long: {} ticks", count));
        }

        let input_start = 4 + meta_len;
        let input_end = input_start + count;
        if data.len() < input_end {
            return Err("Truncated input log".into());
        }

        // Clear and load input log
        state.input_log[..count].copy_from_slice(&data[input_start..input_end]);
        for i in count..TAS_MAX_TICKS {
            state.input_log[i] = 0;
        }

        // Load rec_coords if present
        let coords_start = input_end;
        let coords_size = count * 3 * 4; // 3 floats * 4 bytes
        if data.len() >= coords_start + coords_size {
            let mut offset = coords_start;
            for i in 0..count {
                for j in 0..3 {
                    state.rec_coords[i][j] = f32::from_le_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]);
                    offset += 4;
                }
            }
        }

        state.recorded_count = meta.recorded_count;
        state.inject_mode = meta.inject_mode;
        state.force_fixed_tick = 0; // always force fft=0 (proven zero-drift config)
        state.force_direct = meta.force_direct;
        state.input_source = meta.input_source;

        let segments = meta.segments;
        Ok((meta.recorded_count, segments))
    }
}

/// Snapshot of input log + coords for undo
pub struct RecordingSnapshot {
    pub recorded_count: u32,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
}

impl RecordingSnapshot {
    pub fn from_state(state: &TasSharedState) -> Self {
        let count = state.recorded_count as usize;
        Self {
            recorded_count: state.recorded_count,
            input_log: state.input_log[..count].to_vec(),
            rec_coords: state.rec_coords[..count].to_vec(),
        }
    }

    pub fn restore_to(&self, state: &mut TasSharedState) {
        let count = self.recorded_count as usize;
        state.recorded_count = self.recorded_count;
        state.input_log[..count].copy_from_slice(&self.input_log);
        for i in count..TAS_MAX_TICKS {
            state.input_log[i] = 0;
        }
        state.rec_coords[..count].copy_from_slice(&self.rec_coords);
    }
}

pub struct UndoRing {
    snapshots: Vec<RecordingSnapshot>,
    capacity: usize,
}

impl UndoRing {
    pub fn new(capacity: usize) -> Self {
        Self {
            snapshots: Vec::new(),
            capacity,
        }
    }

    pub fn push(&mut self, state: &TasSharedState) {
        if state.recorded_count == 0 {
            return;
        }
        if self.snapshots.len() >= self.capacity {
            self.snapshots.remove(0);
        }
        self.snapshots.push(RecordingSnapshot::from_state(state));
    }

    pub fn pop(&mut self) -> Option<RecordingSnapshot> {
        self.snapshots.pop()
    }

    pub fn len(&self) -> usize {
        self.snapshots.len()
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

#[allow(dead_code)]
pub fn save_dialog(state: &TasSharedState, log: &mut Vec<String>) {
    save_dialog_with_segments(state, &[], log);
}

pub fn save_dialog_with_segments(
    state: &TasSharedState,
    segments: &[Segment],
    log: &mut Vec<String>,
) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Save TAS Recording")
        .add_filter("TAS Recording", &["tasrec"])
        .save_file()
    {
        match RecordingFile::save_with_segments(state, &path, segments) {
            Ok(()) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Saved recording to {}", ts, path.display()));
            }
            Err(e) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Save error: {}", ts, e));
            }
        }
    }
}

pub fn load_dialog(
    state: &mut TasSharedState,
    tracker: &mut SegmentTracker,
    log: &mut Vec<String>,
) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Load TAS Recording")
        .add_filter("TAS Recording", &["tasrec"])
        .pick_file()
    {
        match RecordingFile::load(state, &path) {
            Ok((count, segments)) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                let seg_count = segments.len();
                tracker.restore_from(segments);
                log.push(format!(
                    "[{}] Loaded {} ticks, {} segments from {}",
                    ts, count, seg_count,
                    path.display()
                ));
            }
            Err(e) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Load error: {}", ts, e));
            }
        }
    }
}

pub fn dump_diagnostics(state: &TasSharedState, ui_drift: (f32, f32), log: &mut Vec<String>) {
    let ts = chrono::Local::now();
    let filename = format!("tas_dump_{}.txt", ts.format("%Y%m%d_%H%M%S"));

    if let Some(path) = rfd::FileDialog::new()
        .set_title("Dump Diagnostics")
        .set_file_name(&filename)
        .add_filter("Text", &["txt"])
        .save_file()
    {
        let mut out = String::new();
        out.push_str(&format!("TAS Diagnostics Dump - {}\n", ts.to_rfc3339()));
        out.push_str(&format!("Version: {}\n", state.version));
        out.push_str(&format!("Mode: {}\n", state.mode_str()));
        out.push_str(&format!("Recorded: {} ticks\n", state.recorded_count));
        out.push_str(&format!("Playback pos: {}\n", state.playback_pos));
        out.push_str(&format!("Frame count: {}\n", state.frame_count));
        out.push_str(&format!(
            "Position: ({:.6}, {:.6}, {:.6})\n",
            state.player_x, state.player_y, state.player_z
        ));
        out.push_str(&format!(
            "Max drift (UI): X={:.9} Z={:.9}\n",
            ui_drift.0, ui_drift.1
        ));
        out.push_str(&format!(
            "Max drift (DLL): X={:.9} Z={:.9}\n",
            state.max_drift_x, state.max_drift_z
        ));
        out.push_str(&format!("BB3B10 calls: {}\n", state.bb3b10_call_count));
        out.push_str(&format!("Handler blocks: {}\n", state.handler_block_count));
        out.push_str(&format!("BB3B10 blocks: {}\n", state.bb3b10_block_count));
        out.push_str(&format!("Events: {}\n", state.event_count));
        out.push_str(&format!(
            "Config: inject_mode={} fft={} force_direct={} input_source={} speed={:.2}\n",
            state.inject_mode, state.force_fixed_tick, state.force_direct, state.input_source,
            state.playback_speed
        ));
        out.push_str(&format!(
            "Hooks: cave2={} cave1c={} cave1d={} cave5={} replay={}\n",
            state.cave2_hooked,
            state.cave1c_hooked,
            state.cave1d_hooked,
            state.cave5_hooked,
            state.replay_capture_hooked
        ));
        out.push_str(&format!(
            "Pointers: replay=0x{:08X} player=0x{:08X}\n\n",
            state.replay_ptr, state.player_ptr
        ));

        // Input transitions
        let count = state.recorded_count as usize;
        if count > 0 {
            out.push_str("Input transitions:\n");
            let mut prev: u8 = 0;
            for i in 0..count {
                let mask = state.input_log[i];
                if mask != prev {
                    let mut bits = String::new();
                    for &(bit, _, name) in tas_shared::input_bits::ALL {
                        if mask & bit != 0 {
                            if !bits.is_empty() {
                                bits.push('+');
                            }
                            bits.push_str(name);
                        }
                    }
                    if bits.is_empty() {
                        bits = "NONE".to_string();
                    }
                    out.push_str(&format!("  tick {}: {} (0x{:02X})\n", i, bits, mask));
                    prev = mask;
                }
            }
        }

        match std::fs::write(&path, &out) {
            Ok(()) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Dumped diagnostics to {}", ts, path.display()));
            }
            Err(e) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Dump error: {}", ts, e));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static REC_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    fn unique_temp_path(prefix: &str, ext: &str) -> std::path::PathBuf {
        let id = REC_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{}_{}_{}.{}", prefix, std::process::id(), id, ext))
    }

    // ===== UndoRing =====

    #[test]
    fn undo_ring_new_is_empty() {
        let ring = UndoRing::new(3);
        assert_eq!(ring.len(), 0);
        assert!(ring.snapshots.is_empty());
    }

    #[test]
    fn undo_ring_push_and_pop() {
        let ring = &mut UndoRing::new(5);
        let mut state = zeroed_state();
        state.recorded_count = 3;
        state.input_log[0] = 0x04; // UP
        state.input_log[1] = 0x04;
        state.input_log[2] = 0x00;
        state.rec_coords[0] = [1.0, 2.0, 3.0];

        ring.push(&state);
        assert_eq!(ring.len(), 1);

        let snap = ring.pop().unwrap();
        assert_eq!(snap.recorded_count, 3);
        assert_eq!(snap.input_log, vec![0x04, 0x04, 0x00]);
        assert_eq!(snap.rec_coords[0], [1.0, 2.0, 3.0]);
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn undo_ring_pop_empty_returns_none() {
        let mut ring = UndoRing::new(3);
        assert!(ring.pop().is_none());
    }

    #[test]
    fn undo_ring_skip_empty_recording() {
        let mut ring = UndoRing::new(5);
        let state = zeroed_state(); // recorded_count = 0
        ring.push(&state);
        assert_eq!(ring.len(), 0);
    }

    #[test]
    fn undo_ring_overflow_drops_oldest() {
        let mut ring = UndoRing::new(2);
        let mut state = zeroed_state();

        // Push snapshot A
        state.recorded_count = 1;
        state.input_log[0] = 0x01; // LEFT
        ring.push(&state);

        // Push snapshot B
        state.input_log[0] = 0x02; // RIGHT
        ring.push(&state);

        assert_eq!(ring.len(), 2);

        // Push snapshot C — should evict A
        state.input_log[0] = 0x04; // UP
        ring.push(&state);

        assert_eq!(ring.len(), 2);

        // Pop should return C (LIFO)
        let snap = ring.pop().unwrap();
        assert_eq!(snap.input_log[0], 0x04);

        // Next pop should return B (A was evicted)
        let snap = ring.pop().unwrap();
        assert_eq!(snap.input_log[0], 0x02);

        assert!(ring.pop().is_none());
    }

    #[test]
    fn undo_ring_lifo_order() {
        let mut ring = UndoRing::new(10);
        let mut state = zeroed_state();
        state.recorded_count = 1;

        for i in 0..5u8 {
            state.input_log[0] = i;
            ring.push(&state);
        }
        assert_eq!(ring.len(), 5);

        // Pop should return 4, 3, 2, 1, 0
        for expected in (0..5u8).rev() {
            let snap = ring.pop().unwrap();
            assert_eq!(snap.input_log[0], expected);
        }
    }

    // ===== RecordingSnapshot =====

    #[test]
    fn snapshot_from_state_and_restore() {
        let mut state = zeroed_state();
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        state.rec_coords[0] = [10.0, 20.0, 30.0];
        state.rec_coords[1] = [40.0, 50.0, 60.0];

        let snap = RecordingSnapshot::from_state(&state);
        assert_eq!(snap.recorded_count, 2);
        assert_eq!(snap.input_log.len(), 2);

        // Modify state
        state.recorded_count = 0;
        state.input_log[0] = 0xFF;

        // Restore
        snap.restore_to(&mut state);
        assert_eq!(state.recorded_count, 2);
        assert_eq!(state.input_log[0], 0x01);
        assert_eq!(state.input_log[1], 0x02);
        assert_eq!(state.rec_coords[0], [10.0, 20.0, 30.0]);
    }

    #[test]
    fn snapshot_restore_clears_trailing_data() {
        let mut state = zeroed_state();
        state.recorded_count = 5;
        for i in 0..5 {
            state.input_log[i] = 0xFF;
        }

        // Snapshot with 5 entries
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        let snap = RecordingSnapshot::from_state(&state);

        // Set state to have trailing data
        state.recorded_count = 5;
        state.input_log[2] = 0xAA;
        state.input_log[3] = 0xBB;
        state.input_log[4] = 0xCC;

        // Restore should clear beyond count
        snap.restore_to(&mut state);
        assert_eq!(state.recorded_count, 2);
        assert_eq!(state.input_log[2], 0);
        assert_eq!(state.input_log[3], 0);
        assert_eq!(state.input_log[4], 0);
    }

    // ===== SegmentTracker =====

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

    // ===== RecordingFile save/load round-trip =====

    #[test]
    fn recording_file_round_trip() {
        let mut state = zeroed_state();
        state.version = 4;
        state.recorded_count = 10;
        state.inject_mode = 6;
        state.force_fixed_tick = 0;
        state.force_direct = 2;
        state.input_source = 1;
        state.max_drift_x = 0.0;
        state.max_drift_z = 0.0;

        for i in 0..10 {
            state.input_log[i] = (i as u8) & 0x3F;
            state.rec_coords[i] = [i as f32 * 1.0, i as f32 * 2.0, i as f32 * 3.0];
        }

        let path = unique_temp_path("rec_rt", "tasrec");

        RecordingFile::save(&state, &path).unwrap();

        let mut loaded = zeroed_state();
        let (count, segments) = RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(count, 10);
        assert!(segments.is_empty());
        assert_eq!(loaded.recorded_count, 10);
        assert_eq!(loaded.inject_mode, 6);
        assert_eq!(loaded.force_fixed_tick, 0); // always forced to 0 on load
        assert_eq!(loaded.force_direct, 2);

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
        let state = zeroed_state();
        let path = unique_temp_path("rec_empty", "tasrec");
        assert!(RecordingFile::save(&state, &path).is_err());
    }

    #[test]
    fn recording_file_load_truncated_errors() {
        let path = unique_temp_path("rec_trunc", "tasrec");
        std::fs::write(&path, [0u8; 2]).unwrap(); // Too small
        let mut state = zeroed_state();
        assert!(RecordingFile::load(&mut state, &path).is_err());
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn recording_file_with_segments_round_trip() {
        let mut state = zeroed_state();
        state.recorded_count = 5;
        state.inject_mode = 6;
        state.force_direct = 2;
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
        RecordingFile::save_with_segments(&state, &path, &segments).unwrap();

        // Use RecordingFile::load() for a real round-trip (not manual JSON parse)
        let mut loaded = zeroed_state();
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
        assert_eq!(loaded.inject_mode, 6);
        assert_eq!(loaded.force_fixed_tick, 0); // fft always forced to 0 on load
        assert_eq!(loaded.force_direct, 2);
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
}
