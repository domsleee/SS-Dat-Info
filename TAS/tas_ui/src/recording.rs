use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

const TAS_TICKS_PER_SECOND: u32 = 100;
const DEFAULT_RECOVERY_DEBOUNCE_MS: u64 = 250;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecordingSessionKind {
    Rec,
    Continue,
}

/// Format TAS tick durations as clock time (`m:ss.cc` or `h:mm:ss.cc`) at 100 Hz.
pub fn format_recording_duration(ticks: u32) -> String {
    let total_seconds = ticks / TAS_TICKS_PER_SECOND;
    let centiseconds = ticks % TAS_TICKS_PER_SECOND;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    if minutes >= 60 {
        let hours = minutes / 60;
        let rem_minutes = minutes % 60;
        format!(
            "{}:{:02}:{:02}.{:02}",
            hours, rem_minutes, seconds, centiseconds
        )
    } else {
        format!("{}:{:02}.{:02}", minutes, seconds, centiseconds)
    }
}

/// Find the first index after 0 where the recorded position diverges from
/// `rec_coords[0]`. This is the recording's "race-start" landmark: the
/// character holds the start coordinate during the pre-race countdown, then
/// moves on the first physics frame after the gate opens. Returns `None` if
/// `recorded_count` is 0 or no movement is recorded.
///
/// Bit-comparison of `f32` is intentional — we want exact equality, not
/// epsilon, so the first sub-millimetre coord change counts.
pub fn detect_first_moving(rec_coords: &[[f32; 3]], recorded_count: u32) -> Option<u32> {
    // Delegate to the shared implementation so tas_ui and the cont-reliability
    // harness use one source of truth for the F5-bucket fingerprint.
    tas_shared::cont::detect_first_moving(rec_coords, recorded_count)
}

pub fn completed_session_label(
    kind: RecordingSessionKind,
    start_tick: u32,
    end_tick: u32,
) -> Option<String> {
    if end_tick <= start_tick {
        return None;
    }

    let segment_ticks = end_tick - start_tick;
    let segment = format_recording_duration(segment_ticks);

    let label = match kind {
        RecordingSessionKind::Rec => format!("Recorded {}", segment),
        RecordingSessionKind::Continue => format!(
            "Continued from {}, total {}",
            format_recording_duration(start_tick),
            format_recording_duration(end_tick)
        ),
    };
    Some(label)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySessionContext {
    pub kind: RecordingSessionKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub label: String,
}

impl RecoverySessionContext {
    pub fn from_ticks(kind: RecordingSessionKind, start_tick: u32, end_tick: u32) -> Option<Self> {
        let label = completed_session_label(kind, start_tick, end_tick)?;
        Some(Self {
            kind,
            start_tick,
            end_tick,
            label,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RecoveryMetadata {
    version: u32,
    saved_at: String,
    recorded_count: u32,
    segment_count: usize,
    session: RecoverySessionContext,
}

pub struct RecoveryCheckpoint {
    pub saved_at: String,
    pub session: RecoverySessionContext,
    pub snapshot: RecordingSnapshot,
    pub segments: Vec<Segment>,
}

impl RecoveryCheckpoint {
    pub fn label(&self) -> &str {
        &self.session.label
    }
}

pub struct RecoveryStore {
    root: PathBuf,
    metadata_path: PathBuf,
    recording_path: PathBuf,
    last_recorded_count: u32,
    last_write_at: Option<Instant>,
    debounce: Duration,
}

impl RecoveryStore {
    pub fn new() -> Result<Self, String> {
        let root = default_history_root_dir().join("recovery");
        Self::new_with(root, Duration::from_millis(DEFAULT_RECOVERY_DEBOUNCE_MS))
    }

    fn new_with(root: PathBuf, debounce: Duration) -> Result<Self, String> {
        std::fs::create_dir_all(&root)
            .map_err(|e| format!("failed to create recovery dir {}: {}", root.display(), e))?;

        Ok(Self {
            metadata_path: root.join("recovery_checkpoint.json"),
            recording_path: root.join("recovery_checkpoint.tasrec"),
            root,
            last_recorded_count: 0,
            last_write_at: None,
            debounce,
        })
    }

    #[cfg(test)]
    fn new_in_root(root: PathBuf, debounce: Duration) -> Result<Self, String> {
        Self::new_with(root, debounce)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn persist_if_needed(
        &mut self,
        state: &TasSharedState,
        segments: &[Segment],
        session: &RecoverySessionContext,
        force: bool,
    ) -> Result<bool, String> {
        let snapshot = RecordingSnapshot::from_state(state);
        self.persist_snapshot_if_needed(&snapshot, segments, session, force)
    }

    /// Decide whether a checkpoint is due (throttle/dedup), and if so return a
    /// self-contained [`RecoveryWriteJob`]. The job's ~12ms disk write can then
    /// run OFF the UI thread (so STOP / REC don't hitch). The throttle state is
    /// updated here as if the write happened.
    pub fn take_write_job(
        &mut self,
        snapshot: &RecordingSnapshot,
        segments: &[Segment],
        session: &RecoverySessionContext,
        force: bool,
    ) -> Option<RecoveryWriteJob> {
        if snapshot.recorded_count == 0 {
            return None;
        }
        if !force {
            if snapshot.recorded_count <= self.last_recorded_count {
                return None;
            }
            if let Some(last_write_at) = self.last_write_at {
                if last_write_at.elapsed() < self.debounce {
                    return None;
                }
            }
        }
        self.last_recorded_count = snapshot.recorded_count;
        self.last_write_at = Some(Instant::now());
        Some(RecoveryWriteJob {
            snapshot: snapshot.clone(),
            segments: segments.to_vec(),
            session: session.clone(),
            recording_path: self.recording_path.clone(),
            metadata_path: self.metadata_path.clone(),
        })
    }

    pub fn persist_snapshot_if_needed(
        &mut self,
        snapshot: &RecordingSnapshot,
        segments: &[Segment],
        session: &RecoverySessionContext,
        force: bool,
    ) -> Result<bool, String> {
        match self.take_write_job(snapshot, segments, session, force) {
            Some(job) => {
                job.write()?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

/// A self-contained recovery checkpoint write. Owns everything it needs so the
/// disk write (~12ms) can be moved onto a background thread, keeping STOP and
/// REC snappy on the UI thread.
pub struct RecoveryWriteJob {
    snapshot: RecordingSnapshot,
    segments: Vec<Segment>,
    session: RecoverySessionContext,
    recording_path: PathBuf,
    metadata_path: PathBuf,
}

impl RecoveryWriteJob {
    pub fn write(self) -> Result<(), String> {
        let mut state = tas_shared::zeroed_boxed();
        self.snapshot.restore_to(&mut state);

        let tmp_recording_path = temp_path_for(&self.recording_path);
        RecordingFile::save_with_segments(&state, &tmp_recording_path, &self.segments)?;
        atomic_replace_file(&tmp_recording_path, &self.recording_path)?;

        let metadata = RecoveryMetadata {
            version: 1,
            saved_at: chrono::Local::now().to_rfc3339(),
            recorded_count: self.snapshot.recorded_count,
            segment_count: self.segments.len(),
            session: self.session,
        };
        let metadata_json = serde_json::to_vec_pretty(&metadata)
            .map_err(|e| format!("failed to serialize recovery metadata: {}", e))?;

        let tmp_metadata_path = temp_path_for(&self.metadata_path);
        std::fs::write(&tmp_metadata_path, metadata_json).map_err(|e| {
            format!(
                "failed to write temp recovery metadata {}: {}",
                tmp_metadata_path.display(),
                e
            )
        })?;
        atomic_replace_file(&tmp_metadata_path, &self.metadata_path)?;
        Ok(())
    }
}

impl RecoveryStore {
    pub fn load_pending(&self) -> Result<Option<RecoveryCheckpoint>, String> {
        if !self.metadata_path.exists() || !self.recording_path.exists() {
            return Ok(None);
        }

        let metadata_json = std::fs::read_to_string(&self.metadata_path).map_err(|e| {
            format!(
                "failed to read recovery metadata {}: {}",
                self.metadata_path.display(),
                e
            )
        })?;
        let metadata: RecoveryMetadata = serde_json::from_str(&metadata_json)
            .map_err(|e| format!("failed to parse recovery metadata: {}", e))?;

        let mut state = tas_shared::zeroed_boxed();
        let (_, segments) = RecordingFile::load(&mut state, &self.recording_path).map_err(|e| {
            format!(
                "failed to load recovery recording {}: {}",
                self.recording_path.display(),
                e
            )
        })?;

        if state.recorded_count == 0 {
            return Ok(None);
        }

        Ok(Some(RecoveryCheckpoint {
            saved_at: metadata.saved_at,
            session: metadata.session,
            snapshot: RecordingSnapshot::from_state(&state),
            segments,
        }))
    }

    pub fn clear_pending(&mut self) -> Result<(), String> {
        remove_if_exists(&self.metadata_path)?;
        remove_if_exists(&self.recording_path)?;
        self.last_recorded_count = 0;
        self.last_write_at = None;
        Ok(())
    }
}

fn temp_path_for(path: &Path) -> PathBuf {
    let nonce = chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default();
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "recovery".to_string());
    path.with_file_name(format!("{}.{}.tmp", name, nonce))
}

fn atomic_replace_file(temp_path: &Path, final_path: &Path) -> Result<(), String> {
    if let Err(rename_err) = std::fs::rename(temp_path, final_path) {
        if final_path.exists() {
            std::fs::remove_file(final_path).map_err(|e| {
                format!(
                    "failed to replace {} after rename failure ({}): {}",
                    final_path.display(),
                    rename_err,
                    e
                )
            })?;
            std::fs::rename(temp_path, final_path).map_err(|e| {
                format!(
                    "failed to finalize replacement {}: {}",
                    final_path.display(),
                    e
                )
            })?;
            return Ok(());
        }
        return Err(format!(
            "failed to move {} to {}: {}",
            temp_path.display(),
            final_path.display(),
            rename_err
        ));
    }
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(path).map_err(|e| format!("failed to remove {}: {}", path.display(), e))
}

fn default_history_root_dir() -> PathBuf {
    user_home_dir()
        .map(|home| home.join(".ssb-inspector"))
        .unwrap_or_else(|| PathBuf::from(".ssb-inspector"))
}

fn user_home_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(PathBuf::from))
        .or_else(|| {
            let drive = std::env::var_os("HOMEDRIVE")?;
            let path = std::env::var_os("HOMEPATH")?;
            let mut buf = PathBuf::from(drive);
            buf.push(path);
            Some(buf)
        })
}

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

        // Load rec_coords if present. Crucially, zero the FULL coord
        // buffer first — loading a legacy/minimal .tasrec with no coord
        // block used to silently leave the previous recording's coords
        // in shared memory, which then corrupted CONT start-matching,
        // drift analysis, and any re-save of the loaded file.
        for i in 0..TAS_MAX_TICKS {
            state.rec_coords[i] = [0.0, 0.0, 0.0];
        }
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

/// Snapshot of input log + coords for history restore operations.
/// Pre-allocates max-size buffers to avoid per-push heap allocation.
#[derive(Clone)]
pub struct RecordingSnapshot {
    pub recorded_count: u32,
    pub(crate) input_log: Box<[u8; TAS_MAX_TICKS]>,
    pub(crate) rec_coords: Box<[[f32; 3]; TAS_MAX_TICKS]>,
}

impl RecordingSnapshot {
    pub fn from_state(state: &TasSharedState) -> Self {
        let mut snap = Self::new_empty();
        snap.capture_from(state);
        snap
    }

    /// Create an empty pre-allocated snapshot.
    fn new_empty() -> Self {
        Self {
            recorded_count: 0,
            input_log: vec![0u8; TAS_MAX_TICKS]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            rec_coords: vec![[0.0f32; 3]; TAS_MAX_TICKS]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    /// Capture state into this (already-allocated) snapshot. No new allocations.
    fn capture_from(&mut self, state: &TasSharedState) {
        let count = state.recorded_count as usize;
        self.recorded_count = state.recorded_count;
        self.input_log[..count].copy_from_slice(&state.input_log[..count]);
        self.rec_coords[..count].copy_from_slice(&state.rec_coords[..count]);
    }

    pub fn restore_to(&self, state: &mut TasSharedState) {
        let count = self.recorded_count as usize;
        state.recorded_count = self.recorded_count;
        state.input_log[..count].copy_from_slice(&self.input_log[..count]);
        for i in count..TAS_MAX_TICKS {
            state.input_log[i] = 0;
        }
        state.rec_coords[..count].copy_from_slice(&self.rec_coords[..count]);
    }

    fn to_persisted(&self) -> PersistedSnapshot {
        let count = self.recorded_count as usize;
        PersistedSnapshot {
            recorded_count: self.recorded_count,
            input_log: self.input_log[..count].to_vec(),
            rec_coords: self.rec_coords[..count].to_vec(),
        }
    }

    fn from_persisted(persisted: PersistedSnapshot) -> Result<Self, String> {
        let count = persisted.recorded_count as usize;
        if count > TAS_MAX_TICKS {
            return Err(format!(
                "persisted snapshot too large: {} ticks (max {})",
                count, TAS_MAX_TICKS
            ));
        }
        if persisted.input_log.len() != count {
            return Err(format!(
                "persisted input_log length mismatch: expected {}, got {}",
                count,
                persisted.input_log.len()
            ));
        }
        if persisted.rec_coords.len() != count {
            return Err(format!(
                "persisted rec_coords length mismatch: expected {}, got {}",
                count,
                persisted.rec_coords.len()
            ));
        }

        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = persisted.recorded_count;
        snap.input_log[..count].copy_from_slice(&persisted.input_log);
        snap.rec_coords[..count].copy_from_slice(&persisted.rec_coords);
        Ok(snap)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryEntryKind {
    Snapshot,
    SaveMarker,
    LoadSnapshot,
}

pub struct HistoryEntry {
    /// Stable, monotonic, never-reused id (the storage identity — NOT the
    /// positional index). Assigned by `RecordingHistory` on push.
    pub entry_id: u64,
    /// Pinned entries are exempt from cap-eviction and never GC'd — durable
    /// named checkpoints that survive across sessions.
    pub pinned: bool,
    /// User-given name (via rename). When set, it's shown instead of the
    /// auto-generated `label`. `None` = use the auto label/duration.
    pub custom_name: Option<String>,
    pub label: String,
    /// HH:MM:SS-of-day legacy display field. Kept for backward compat with
    /// existing persisted history files; new code should prefer
    /// `created_at` for any logic that needs a real date.
    pub timestamp: String,
    /// Full local-tz creation time. Used for date grouping in the panel.
    /// For legacy entries that were persisted without this field, the
    /// in-memory value is set to load-time `Local::now()` as a best-effort
    /// fallback so they cluster under "today" rather than scattering.
    pub created_at: chrono::DateTime<chrono::Local>,
    pub kind: HistoryEntryKind,
    /// Session start tick: 0 for REC entries (recording from the beginning)
    /// or for markers, the resume tick for CONT entries. Zero on legacy
    /// entries persisted before this field existed — the panel falls back
    /// to label parsing in that case.
    pub start_tick: u32,
    /// `recorded_count` at push time. Zero on legacy entries / markers.
    pub end_tick: u32,
    /// First tick where the recorded position diverged from `rec_coords[0]`
    /// — the "race-start" landmark. `None` for legacy entries, markers,
    /// and snapshots with no detected movement.
    pub first_moving: Option<u32>,
    snapshot: Option<RecordingSnapshot>,
}

impl HistoryEntry {
    fn from_snapshot(label: String, kind: HistoryEntryKind, snapshot: RecordingSnapshot) -> Self {
        let now = chrono::Local::now();
        let end_tick = snapshot.recorded_count;
        let first_moving = detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            timestamp: now.format("%H:%M:%S").to_string(),
            created_at: now,
            kind,
            // start_tick is overwritten by `with_session` for CONT entries
            // that know their resume point; REC entries leave it at 0.
            start_tick: 0,
            end_tick,
            first_moving,
            snapshot: Some(snapshot),
        }
    }

    fn marker(label: String, kind: HistoryEntryKind) -> Self {
        let now = chrono::Local::now();
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            timestamp: now.format("%H:%M:%S").to_string(),
            created_at: now,
            kind,
            start_tick: 0,
            end_tick: 0,
            first_moving: None,
            snapshot: None,
        }
    }

    fn with_session(mut self, start_tick: u32, end_tick: u32) -> Self {
        self.start_tick = start_tick;
        self.end_tick = end_tick;
        self
    }

    pub fn can_restore(&self) -> bool {
        self.snapshot.is_some()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PersistedSnapshot {
    pub recorded_count: u32,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
}

#[derive(Serialize, Deserialize)]
pub struct PersistedHistoryEntry {
    pub label: String,
    pub timestamp: String,
    /// ISO 8601 local datetime with offset, e.g. `2026-05-24T21:04:43+10:00`.
    /// Optional for backward compat — legacy entries persisted without this
    /// field get serialised as `""` and the loader falls back to "now" when
    /// constructing the in-memory `created_at`. Going forward, every new
    /// entry serialises a full timestamp here so day-grouping in the panel
    /// remains correct across multi-day sessions.
    #[serde(default)]
    pub created_at_iso: String,
    pub kind: HistoryEntryKind,
    pub snapshot: Option<PersistedSnapshot>,
    /// Session start tick. Legacy entries default to 0; the panel falls
    /// back to parsing `label` when this is 0 alongside a non-zero
    /// snapshot recorded_count.
    #[serde(default)]
    pub start_tick: u32,
    /// `recorded_count` at push time. Legacy entries default to 0.
    #[serde(default)]
    pub end_tick: u32,
    /// "Race-start" landmark tick — first index where rec_coords diverged
    /// from rec_coords[0]. Legacy entries default to None.
    #[serde(default)]
    pub first_moving: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct PersistedHistory {
    pub version: u32,
    pub saved_at: String,
    pub current_index: Option<usize>,
    pub entries: Vec<PersistedHistoryEntry>,
}

pub struct RecordingHistory {
    capacity: usize,
    entries: Vec<HistoryEntry>,
    current_index: Option<usize>,
    /// Next stable entry id to hand out. Authoritative + monotonic; never
    /// reused. Restored (and bumped past) on load.
    next_entry_id: u64,
    /// Bumped on every mutation (structural / metadata / cursor) so the app can
    /// cheaply detect "history changed, re-persist" without diffing.
    revision: u64,
}

impl RecordingHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            entries: Vec::with_capacity(capacity.max(1)),
            current_index: None,
            next_entry_id: 1,
            revision: 0,
        }
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_entry_id;
        self.next_entry_id = self.next_entry_id.checked_add(1).expect("entry_id overflow");
        id
    }

    /// Monotonic change counter — compare across frames to know if a persist is
    /// needed. Rename and pin bump this even though len/cursor are unchanged.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    fn bump(&mut self) {
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn next_entry_id(&self) -> u64 {
        self.next_entry_id
    }

    pub fn push_snapshot(&mut self, state: &TasSharedState, label: impl Into<String>) -> bool {
        self.push_snapshot_data(RecordingSnapshot::from_state(state), label)
    }

    pub fn push_snapshot_data(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
    ) -> bool {
        self.push_snapshot_entry(snapshot, label.into(), HistoryEntryKind::Snapshot, None)
    }

    /// Like `push_snapshot_data` but records the session's `start_tick` /
    /// `end_tick` on the entry so the panel can render "from <tick> ·
    /// <in-game time>" without parsing the label string.
    pub fn push_snapshot_data_with_session(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
        start_tick: u32,
        end_tick: u32,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            Some((start_tick, end_tick)),
        )
    }

    pub fn push_loaded_snapshot(&mut self, state: &TasSharedState, path: &Path) -> bool {
        let label = format!("Load: {}", short_file_label(path));
        self.push_snapshot_entry(
            RecordingSnapshot::from_state(state),
            label,
            HistoryEntryKind::LoadSnapshot,
            None,
        )
    }

    /// Record a save marker without changing the current restored state.
    pub fn push_save_marker(&mut self, state: &TasSharedState, path: &Path) {
        if self.current_index.is_none() && state.recorded_count > 0 {
            let _ = self.push_snapshot(state, "Current recording");
        }
        if self.entries.is_empty() {
            return;
        }
        let label = format!("Save: {}", short_file_label(path));
        let mut marker = HistoryEntry::marker(label, HistoryEntryKind::SaveMarker);
        marker.entry_id = self.alloc_id();
        if let Some(current) = self.current_index {
            // Save belongs to the current visible state without changing selection.
            let insert_at = (current + 1).min(self.entries.len());
            self.entries.insert(insert_at, marker);
            self.enforce_capacity();
        } else {
            self.entries.push(marker);
            self.enforce_capacity();
        }
        self.bump();
    }

    pub fn undo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let prev = (0..current)
            .rev()
            .find(|&i| self.entries[i].snapshot.is_some())?;
        self.current_index = Some(prev);
        self.bump();
        self.entries[prev].snapshot.as_ref()
    }

    pub fn redo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let next =
            ((current + 1)..self.entries.len()).find(|&i| self.entries[i].snapshot.is_some())?;
        self.current_index = Some(next);
        self.bump();
        self.entries[next].snapshot.as_ref()
    }

    pub fn restore_index(&mut self, index: usize) -> Option<&RecordingSnapshot> {
        if index >= self.entries.len() {
            return None;
        }
        self.entries[index].snapshot.as_ref()?;
        self.current_index = Some(index);
        self.bump();
        self.entries[index].snapshot.as_ref()
    }

    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    /// Clear the "current" pointer so no row is highlighted. Does not
    /// touch any entry data. Used by the panel when the user clicks
    /// empty space to deselect.
    pub fn clear_selection(&mut self) {
        self.current_index = None;
        self.bump();
    }

    // ===== v2 store bridge =====

    /// Convert the in-memory history to the store's entry list (row order).
    pub fn to_stored_entries(&self) -> Vec<crate::history_store_v2::StoredEntry> {
        self.entries
            .iter()
            .map(|e| crate::history_store_v2::StoredEntry {
                entry_id: e.entry_id,
                name: e.label.clone(),
                user_name: e.custom_name.clone(),
                pinned: e.pinned,
                kind: e.kind,
                start_tick: e.start_tick,
                end_tick: e.end_tick,
                first_moving: e.first_moving,
                created_at_iso: e.created_at.to_rfc3339(),
                snapshot: e.snapshot.as_ref().map(RecordingSnapshot::to_persisted),
            })
            .collect()
    }

    /// The stable id of the current (selected) entry, if any.
    pub fn current_entry_id(&self) -> Option<u64> {
        self.current_index.map(|i| self.entries[i].entry_id)
    }

    pub fn set_pinned(&mut self, entry_id: u64, pinned: bool) -> bool {
        let mut changed = false;
        let found = if let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) {
            if e.pinned != pinned {
                e.pinned = pinned;
                changed = true;
            }
            true
        } else {
            false
        };
        if changed {
            self.bump();
            if !pinned {
                // Unpinning can push the unpinned count back over the cap.
                self.enforce_capacity();
            }
        }
        found
    }

    /// Set (or clear, if blank) the user-given name of an entry. Leaves the
    /// auto `label` (and its duration) intact; the name is shown instead.
    pub fn rename(&mut self, entry_id: u64, name: impl Into<String>) -> bool {
        let name = name.into();
        let new = {
            let t = name.trim();
            if t.is_empty() {
                None
            } else {
                Some(t.to_string())
            }
        };
        if let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) {
            if e.custom_name != new {
                e.custom_name = new;
                self.bump();
            }
            true
        } else {
            false
        }
    }

    /// Change the soft cap (max unpinned entries) and trim immediately. Used
    /// by the config-panel setting.
    pub fn set_capacity(&mut self, capacity: usize) {
        let cap = capacity.max(1);
        if cap != self.capacity {
            self.capacity = cap;
            self.enforce_capacity();
            self.bump();
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_pinned(&self, entry_id: u64) -> bool {
        self.entries
            .iter()
            .find(|e| e.entry_id == entry_id)
            .map(|e| e.pinned)
            .unwrap_or(false)
    }

    /// Rebuild the in-memory history from a v2-store load. Unavailable entries
    /// (snapshot == None but kind expects one) come in inert (can't restore).
    pub fn apply_loaded(
        &mut self,
        loaded: Vec<crate::history_store_v2::LoadedEntry>,
        current_entry_id: Option<u64>,
        next_entry_id_floor: u64,
    ) {
        let mut entries = Vec::with_capacity(loaded.len());
        for le in loaded {
            let snapshot = le
                .snapshot
                .and_then(|ps| RecordingSnapshot::from_persisted(ps).ok());
            let created_at = chrono::DateTime::parse_from_rfc3339(&le.created_at_iso)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local))
                .unwrap_or_else(chrono::Local::now);
            entries.push(HistoryEntry {
                entry_id: le.entry_id,
                pinned: le.pinned,
                custom_name: le.user_name,
                label: le.name,
                timestamp: created_at.format("%H:%M:%S").to_string(),
                created_at,
                kind: le.kind,
                start_tick: le.start_tick,
                end_tick: le.end_tick,
                first_moving: le.first_moving,
                snapshot,
            });
        }
        let max_id_plus_1 = entries.iter().map(|e| e.entry_id + 1).max().unwrap_or(1);
        self.entries = entries;
        self.next_entry_id = self
            .next_entry_id
            .max(next_entry_id_floor)
            .max(max_id_plus_1);
        // Respect the store's already-resolved cursor exactly (it fell back to
        // the nearest available entry, or None). Do NOT silently jump to newest.
        self.current_index = current_entry_id
            .and_then(|id| self.entries.iter().position(|e| e.entry_id == id))
            .filter(|&i| self.entries[i].snapshot.is_some());
        // A lowered cap (e.g. settings changed between sessions) trims on load.
        self.enforce_capacity_preserving_none();
        self.bump();
    }

    pub fn can_undo(&self) -> bool {
        self.undo_depth() > 0
    }

    pub fn can_redo(&self) -> bool {
        self.redo_depth() > 0
    }

    pub fn undo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        (0..current)
            .filter(|&i| self.entries[i].snapshot.is_some())
            .count()
    }

    pub fn redo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        ((current + 1)..self.entries.len())
            .filter(|&i| self.entries[i].snapshot.is_some())
            .count()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }

    pub fn to_persisted(&self) -> PersistedHistory {
        let entries = self
            .entries
            .iter()
            .map(|entry| PersistedHistoryEntry {
                label: entry.label.clone(),
                timestamp: entry.timestamp.clone(),
                created_at_iso: entry.created_at.to_rfc3339(),
                kind: entry.kind,
                snapshot: entry.snapshot.as_ref().map(RecordingSnapshot::to_persisted),
                start_tick: entry.start_tick,
                end_tick: entry.end_tick,
                first_moving: entry.first_moving,
            })
            .collect();

        PersistedHistory {
            version: 2,
            saved_at: chrono::Local::now().to_rfc3339(),
            current_index: self.current_index,
            entries,
        }
    }

    pub fn apply_persisted(&mut self, persisted: PersistedHistory) -> Result<(), String> {
        let mut entries = Vec::with_capacity(persisted.entries.len());
        for entry in persisted.entries {
            let snapshot = match entry.snapshot {
                Some(snapshot) => Some(RecordingSnapshot::from_persisted(snapshot)?),
                None => None,
            };
            // Parse the persisted ISO timestamp into a chrono DateTime.
            // For legacy entries that pre-date the `created_at_iso` field,
            // fall back to today's date combined with the persisted
            // HH:MM:SS — that preserves the original time-of-day so the
            // row displays e.g. `21:04` rather than the load-time clock,
            // while still clustering all legacy entries under "today" in
            // the day-header grouping. If the timestamp is also unparsable,
            // last-resort is `Local::now()`.
            let created_at = chrono::DateTime::parse_from_rfc3339(&entry.created_at_iso)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local))
                .or_else(|| {
                    let today = chrono::Local::now().date_naive();
                    let hms = chrono::NaiveTime::parse_from_str(&entry.timestamp, "%H:%M:%S").ok()?;
                    today
                        .and_time(hms)
                        .and_local_timezone(chrono::Local)
                        .single()
                })
                .unwrap_or_else(chrono::Local::now);
            // Legacy entries (pre-fields) come in with end_tick=0 even
            // when a snapshot is present. Fall back to the snapshot's
            // recorded_count so the renderer can still compute durations
            // from structured data; start_tick stays 0 → renderer treats
            // it as "no resume point" and falls back to label parsing
            // for the context phrase.
            let mut end_tick = entry.end_tick;
            if end_tick == 0 {
                if let Some(snap) = snapshot.as_ref() {
                    end_tick = snap.recorded_count;
                }
            }
            entries.push(HistoryEntry {
                entry_id: self.alloc_id(),
                pinned: false,
                custom_name: None,
                label: entry.label,
                timestamp: entry.timestamp,
                created_at,
                kind: entry.kind,
                start_tick: entry.start_tick,
                end_tick,
                first_moving: entry.first_moving,
                snapshot,
            });
        }

        self.entries = entries;
        self.current_index = persisted
            .current_index
            .filter(|idx| *idx < self.entries.len());

        if let Some(idx) = self.current_index {
            if self.entries[idx].snapshot.is_none() {
                self.current_index = None;
            }
        }

        self.enforce_capacity();
        self.bump(); // mark dirty so the (revision-gated) writer persists it
        Ok(())
    }

    /// Soft cap: `capacity` bounds the number of UNPINNED entries. Pinned
    /// entries are never evicted, and the current entry is never evicted.
    /// Evicts the oldest unpinned, non-current entry until the unpinned count
    /// fits (so the effective total can exceed `capacity` if there are many
    /// pins — pins win).
    fn enforce_capacity(&mut self) {
        self.evict_to_cap();
        // After eviction (e.g. from a push), make sure something restorable is
        // selected if nothing is.
        if self.current_index.is_none() {
            self.current_index = self
                .entries
                .iter()
                .enumerate()
                .rev()
                .find_map(|(idx, entry)| entry.snapshot.as_ref().map(|_| idx));
        }
    }

    /// Enforce the cap but leave a `None` cursor as `None` (used on load, where
    /// the store already resolved the cursor and a deliberate `None` must stand).
    fn enforce_capacity_preserving_none(&mut self) {
        self.evict_to_cap();
    }

    /// Soft cap: `capacity` bounds the UNPINNED count. Pinned entries and the
    /// current entry are never evicted; evicts the oldest unpinned, non-current
    /// entry until the unpinned count fits (so total can exceed `capacity` when
    /// there are many pins — pins win).
    fn evict_to_cap(&mut self) {
        loop {
            let unpinned = self.entries.iter().filter(|e| !e.pinned).count();
            if unpinned <= self.capacity {
                break;
            }
            let cur = self.current_index;
            let victim = self
                .entries
                .iter()
                .enumerate()
                .find(|(i, e)| !e.pinned && cur != Some(*i))
                .map(|(i, _)| i);
            let Some(victim) = victim else {
                break; // only pinned and/or the current entry remain
            };
            self.entries.remove(victim);
            self.current_index = self
                .current_index
                .map(|idx| if idx > victim { idx - 1 } else { idx });
        }
    }

    fn push_snapshot_entry(
        &mut self,
        snapshot: RecordingSnapshot,
        label: String,
        kind: HistoryEntryKind,
        session: Option<(u32, u32)>,
    ) -> bool {
        if snapshot.recorded_count == 0 {
            return false;
        }

        let mut entry = HistoryEntry::from_snapshot(label, kind, snapshot);
        entry.entry_id = self.alloc_id();
        if let Some((start_tick, end_tick)) = session {
            entry = entry.with_session(start_tick, end_tick);
        }
        self.entries.push(entry);
        self.current_index = Some(self.entries.len() - 1);
        self.enforce_capacity();
        self.bump();
        true
    }
}

fn short_file_label(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
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
pub fn save_dialog(state: &TasSharedState, log: &mut Vec<String>) -> Option<PathBuf> {
    save_dialog_with_segments(state, &[], log)
}

pub fn save_dialog_with_segments(
    state: &TasSharedState,
    segments: &[Segment],
    log: &mut Vec<String>,
) -> Option<PathBuf> {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Save TAS Recording")
        .add_filter("TAS Recording", &["tasrec"])
        .save_file()
    {
        match RecordingFile::save_with_segments(state, &path, segments) {
            Ok(()) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Saved recording to {}", ts, path.display()));
                return Some(path);
            }
            Err(e) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Save error: {}", ts, e));
            }
        }
    }
    None
}

pub fn load_dialog(
    state: &mut TasSharedState,
    tracker: &mut SegmentTracker,
    log: &mut Vec<String>,
) -> Option<PathBuf> {
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
                    ts,
                    count,
                    seg_count,
                    path.display()
                ));
                return Some(path);
            }
            Err(e) => {
                let ts = chrono::Local::now().format("%H:%M:%S");
                log.push(format!("[{}] Load error: {}", ts, e));
            }
        }
    }
    None
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
            state.inject_mode,
            state.force_fixed_tick,
            state.force_direct,
            state.input_source,
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
    use std::time::Duration;

    static REC_COUNTER: AtomicU32 = AtomicU32::new(0);

    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    fn unique_temp_path(prefix: &str, ext: &str) -> std::path::PathBuf {
        let id = REC_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("{}_{}_{}.{}", prefix, std::process::id(), id, ext))
    }

    fn unique_temp_root(prefix: &str) -> std::path::PathBuf {
        let id = REC_COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), id));
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn detect_first_moving_finds_first_changed_coord() {
        let mut coords = vec![[1.0, 2.0, 3.0]; 10];
        coords[5] = [1.0, 2.0, 3.5];
        assert_eq!(detect_first_moving(&coords, 10), Some(5));
    }

    #[test]
    fn detect_first_moving_returns_none_when_static() {
        let coords = vec![[1.0, 2.0, 3.0]; 10];
        assert_eq!(detect_first_moving(&coords, 10), None);
    }

    #[test]
    fn detect_first_moving_returns_none_for_empty_or_single_tick() {
        assert_eq!(detect_first_moving(&[], 0), None);
        assert_eq!(detect_first_moving(&[[0.0; 3]], 1), None);
        let coords = vec![[1.0, 2.0, 3.0]; 10];
        assert_eq!(detect_first_moving(&coords, 0), None);
    }

    #[test]
    fn detect_first_moving_respects_recorded_count_bound() {
        // Movement at index 5, but recorded_count limits the scan to 3.
        let mut coords = vec![[1.0, 2.0, 3.0]; 10];
        coords[5] = [9.0, 9.0, 9.0];
        assert_eq!(detect_first_moving(&coords, 3), None);
        // Bumping the count to 6 finds it.
        assert_eq!(detect_first_moving(&coords, 6), Some(5));
    }

    #[test]
    fn format_recording_duration_uses_clock_format() {
        assert_eq!(format_recording_duration(0), "0:00.00");
        assert_eq!(format_recording_duration(2303), "0:23.03");
        assert_eq!(format_recording_duration(5303), "0:53.03");
        assert_eq!(format_recording_duration(65536), "10:55.36");
        assert_eq!(format_recording_duration(372300), "1:02:03.00");
    }

    #[test]
    fn completed_session_label_formats_rec_and_continue() {
        assert_eq!(
            completed_session_label(RecordingSessionKind::Rec, 0, 2303).as_deref(),
            Some("Recorded 0:23.03")
        );
        assert_eq!(
            completed_session_label(RecordingSessionKind::Continue, 3000, 5303).as_deref(),
            Some("Continued from 0:30.00, total 0:53.03")
        );
        assert!(
            completed_session_label(RecordingSessionKind::Rec, 100, 100).is_none(),
            "zero-length sessions should not create labels"
        );
    }

    // ===== RecordingHistory =====

    fn one_tick_state(mask: u8) -> Box<TasSharedState> {
        let mut state = zeroed_state();
        state.recorded_count = 1;
        state.input_log[0] = mask;
        state.rec_coords[0] = [mask as f32, 0.0, mask as f32];
        state
    }

    #[test]
    fn history_new_is_empty() {
        let history = RecordingHistory::new(4);
        assert_eq!(history.len(), 0);
        assert!(!history.can_undo());
        assert!(!history.can_redo());
    }

    #[test]
    fn history_push_undo_redo() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        let c = one_tick_state(0x04);

        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        assert!(history.push_snapshot(&c, "C"));
        assert_eq!(history.current_index(), Some(2));
        assert_eq!(history.undo_depth(), 2);
        assert_eq!(history.redo_depth(), 0);

        let snap = history.undo().unwrap();
        assert_eq!(snap.input_log[0], 0x02);
        assert_eq!(history.current_index(), Some(1));
        assert_eq!(history.undo_depth(), 1);
        assert_eq!(history.redo_depth(), 1);

        let snap = history.redo().unwrap();
        assert_eq!(snap.input_log[0], 0x04);
        assert_eq!(history.current_index(), Some(2));
    }

    #[test]
    fn history_branch_keeps_previous_states_append_only() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        let c = one_tick_state(0x04);
        let d = one_tick_state(0x08);

        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        assert!(history.push_snapshot(&c, "C"));
        let _ = history.undo(); // now on B
        assert_eq!(history.current_index(), Some(1));
        assert_eq!(history.redo_depth(), 1);

        assert!(history.push_snapshot(&d, "D"));
        assert_eq!(history.len(), 4);
        assert_eq!(history.current_index(), Some(3));
        assert_eq!(history.entries()[2].label, "C");
        assert_eq!(history.entries()[3].label, "D");
        assert_eq!(history.redo_depth(), 0);
    }

    #[test]
    fn history_save_marker_keeps_current_cursor() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        let before = history.current_index();

        history.push_save_marker(&b, Path::new("C:\\temp\\run.tasrec"));
        assert_eq!(history.len(), 3);
        assert_eq!(history.current_index(), before);
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert!(!history.entries()[2].can_restore());
    }

    #[test]
    fn history_load_snapshot_becomes_current() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let loaded = one_tick_state(0x20);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_loaded_snapshot(&loaded, Path::new("C:\\temp\\loaded.tasrec")));
        let current = history.current_index().unwrap();
        assert_eq!(
            history.entries()[current].kind,
            HistoryEntryKind::LoadSnapshot
        );
        assert!(history.entries()[current].label.contains("loaded.tasrec"));
        assert_eq!(history.undo_depth(), 1);
    }

    #[test]
    fn history_restore_index_skips_non_restorable_entries() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        history.push_save_marker(&b, Path::new("run.tasrec"));

        // Save marker cannot be restored directly.
        assert!(history.restore_index(2).is_none());
        let snap = history.restore_index(0).unwrap();
        assert_eq!(snap.input_log[0], 0x01);
        assert_eq!(history.current_index(), Some(0));
    }

    #[test]
    fn history_capacity_eviction_preserves_recent_entries() {
        let mut history = RecordingHistory::new(2);
        assert!(history.push_snapshot(&one_tick_state(0x01), "A"));
        assert!(history.push_snapshot(&one_tick_state(0x02), "B"));
        assert!(history.push_snapshot(&one_tick_state(0x04), "C"));
        assert_eq!(history.len(), 2);
        assert_eq!(history.entries()[0].label, "B");
        assert_eq!(history.entries()[1].label, "C");
        assert_eq!(history.current_index(), Some(1));
    }

    #[test]
    fn history_persist_round_trip_preserves_snapshots() {
        let mut history = RecordingHistory::new(8);
        let a = one_tick_state(0x01);
        let b = one_tick_state(0x02);
        assert!(history.push_snapshot(&a, "A"));
        assert!(history.push_snapshot(&b, "B"));
        history.push_save_marker(&b, Path::new("C:\\temp\\run.tasrec"));
        let _ = history.undo();

        let persisted = history.to_persisted();
        let mut restored = RecordingHistory::new(8);
        restored.apply_persisted(persisted).unwrap();

        assert_eq!(restored.len(), history.len());
        assert_eq!(restored.current_index(), history.current_index());
        assert_eq!(restored.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert!(!restored.entries()[2].can_restore());
        assert_eq!(restored.restore_index(1).unwrap().input_log[0], 0x02);
    }

    #[test]
    fn history_apply_persisted_rejects_invalid_snapshot_lengths() {
        let mut history = RecordingHistory::new(8);
        let bad = PersistedHistory {
            version: 2,
            saved_at: chrono::Local::now().to_rfc3339(),
            current_index: Some(0),
            entries: vec![PersistedHistoryEntry {
                label: "bad".to_string(),
                timestamp: "00:00:00".to_string(),
                created_at_iso: String::new(),
                kind: HistoryEntryKind::Snapshot,
                snapshot: Some(PersistedSnapshot {
                    recorded_count: 2,
                    input_log: vec![1], // invalid: expected len=2
                    rec_coords: vec![[0.0, 0.0, 0.0], [1.0, 1.0, 1.0]],
                }),
                start_tick: 0,
                end_tick: 2,
                first_moving: None,
            }],
        };

        let err = history.apply_persisted(bad).unwrap_err();
        assert!(err.contains("input_log length mismatch"));
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

        let mut snap = RecordingSnapshot::new_empty();
        snap.capture_from(&state);
        assert_eq!(snap.recorded_count, 2);
        assert_eq!(snap.input_log[0], 0x01);
        assert_eq!(snap.input_log[1], 0x02);

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

        // Snapshot with 2 entries
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        let mut snap = RecordingSnapshot::new_empty();
        snap.capture_from(&state);

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

    // ===== RecoveryStore =====

    // Is the 3.6s history persist a debug-build artifact? Run debug vs release:
    //   cargo test -p tas_ui measure_history_serialize -- --ignored --nocapture
    //   cargo test --release -p tas_ui measure_history_serialize -- --ignored --nocapture
    #[test]
    #[ignore]
    fn measure_history_serialize() {
        use std::time::Instant;
        let mut history = RecordingHistory::new(256);
        let mut state = zeroed_state();
        for i in 0..600usize {
            state.rec_coords[i] = [i as f32, 1.0, 2.0];
            state.input_log[i] = (i % 4) as u8;
        }
        for k in 0..190u32 {
            state.recorded_count = 600 + k; // vary to defeat dedup
            let snap = RecordingSnapshot::from_state(&state);
            history.push_snapshot_data_with_session(snap, format!("entry {}", k), 0, 600 + k);
        }
        let payload = history.to_persisted();

        let t = Instant::now();
        let json = serde_json::to_string_pretty(&payload).unwrap();
        let json_dt = t.elapsed();

        let t = Instant::now();
        let bin = bincode::serialize(&payload).unwrap();
        let bin_dt = t.elapsed();

        // Load side (startup recovery parses the whole file).
        let t = Instant::now();
        let _: PersistedHistory = serde_json::from_str(&json).unwrap();
        let json_load = t.elapsed();
        let t = Instant::now();
        let _: PersistedHistory = bincode::deserialize(&bin).unwrap();
        let bin_load = t.elapsed();
        println!(
            "\n    LOAD:  JSON {:?}   bincode {:?}   ({:.1}x faster)",
            json_load,
            bin_load,
            json_load.as_secs_f64() / bin_load.as_secs_f64()
        );

        println!(
            "\n>>> {} entries\n    JSON:    {} MB  serialize {:?}\n    bincode: {} MB  serialize {:?}\n    binary is {:.1}x smaller, {:.1}x faster to serialize\n",
            history.len(),
            json.len() / 1_000_000,
            json_dt,
            bin.len() / 1_000_000,
            bin_dt,
            json.len() as f64 / bin.len() as f64,
            json_dt.as_secs_f64() / bin_dt.as_secs_f64(),
        );
    }

    // Measurement (not a pass/fail gate). Run with:
    //   cargo test -p tas_ui measure_rec_frame_cost -- --ignored --nocapture
    #[test]
    #[ignore]
    fn measure_rec_frame_cost() {
        use std::time::Instant;
        let mut state = zeroed_state();
        state.recorded_count = 60000;
        for i in 0..60000usize {
            state.rec_coords[i] = [i as f32 * 0.1, 1.0, 2.0];
            state.input_log[i] = (i % 4) as u8;
        }

        // 1) Per-frame snapshot cost (built EVERY frame during REC).
        let n = 500u32;
        let t = Instant::now();
        for _ in 0..n {
            let s = RecordingSnapshot::from_state(&state);
            std::hint::black_box(&s);
        }
        let per_snap = t.elapsed() / n;
        println!("\n>>> from_state (per-frame snapshot): {:?} per call", per_snap);

        // 2) Disk-write cost (throttled, fires ~every debounce during REC).
        let root = unique_temp_root("measure_rec_cost");
        let mut store = RecoveryStore::new_in_root(root, Duration::ZERO).unwrap();
        let snap = RecordingSnapshot::from_state(&state);
        let session =
            RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 60000).unwrap();
        let n2 = 20u32;
        let t = Instant::now();
        for _ in 0..n2 {
            store
                .persist_snapshot_if_needed(&snap, &[], &session, true)
                .unwrap();
        }
        let per_write = t.elapsed() / n2;
        println!(">>> persist (forced disk write): {:?} per write\n", per_write);
    }

    #[test]
    fn recovery_store_persists_and_loads_checkpoint() {
        let root = unique_temp_root("tas_ui_recovery_store_roundtrip");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = zeroed_state();
        state.recorded_count = 5;
        for i in 0..5 {
            state.input_log[i] = (i as u8) + 1;
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 1.5];
        }
        let segments = vec![Segment {
            name: "Segment 1".to_string(),
            start_tick: 0,
            end_tick: 5,
            timestamp: "2026-01-01T00:00:00Z".to_string(),
        }];
        let session = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 5).unwrap();

        assert!(store
            .persist_if_needed(&state, &segments, &session, true)
            .unwrap());
        let pending = store.load_pending().unwrap().expect("expected checkpoint");
        assert_eq!(pending.snapshot.recorded_count, 5);
        assert_eq!(pending.session.label, "Recorded 0:00.05");
        assert_eq!(pending.segments.len(), 1);
        assert_eq!(pending.segments[0].end_tick, 5);

        store.clear_pending().unwrap();
        assert!(store.load_pending().unwrap().is_none());
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn recovery_store_skips_unchanged_non_forced_writes() {
        let root = unique_temp_root("tas_ui_recovery_store_skip");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = zeroed_state();
        state.recorded_count = 3;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        state.input_log[2] = 0x04;
        let session3 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 3).unwrap();

        assert!(store
            .persist_if_needed(&state, &[], &session3, true)
            .unwrap());
        assert!(!store
            .persist_if_needed(&state, &[], &session3, false)
            .unwrap());

        state.recorded_count = 4;
        state.input_log[3] = 0x08;
        let session4 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 4).unwrap();
        assert!(store
            .persist_if_needed(&state, &[], &session4, false)
            .unwrap());

        let pending = store.load_pending().unwrap().expect("expected checkpoint");
        assert_eq!(pending.snapshot.recorded_count, 4);
        assert_eq!(pending.session.label, "Recorded 0:00.04");

        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn recovery_store_replaces_checkpoint_atomically() {
        let root = unique_temp_root("tas_ui_recovery_store_atomic");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = zeroed_state();
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        let session2 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 2).unwrap();
        assert!(store
            .persist_if_needed(&state, &[], &session2, true)
            .unwrap());

        state.recorded_count = 6;
        for i in 2..6 {
            state.input_log[i] = 0x08;
        }
        let session6 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 6).unwrap();
        assert!(store
            .persist_if_needed(&state, &[], &session6, true)
            .unwrap());

        let metadata_path = root.join("recovery_checkpoint.json");
        let metadata_json = std::fs::read_to_string(metadata_path).unwrap();
        let metadata: serde_json::Value = serde_json::from_str(&metadata_json).unwrap();
        assert_eq!(metadata["recorded_count"], 6);

        let tmp_files = std::fs::read_dir(&root)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().contains(".tmp"))
            .count();
        assert_eq!(tmp_files, 0, "temp files should not remain after replace");

        let _ = std::fs::remove_dir_all(root);
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

    // ===== History E2E tests (SSB-283) =====

    /// Helper: create state with N recorded ticks and distinct input per tick.
    fn state_with_ticks(n: u32) -> Box<TasSharedState> {
        let mut state = zeroed_state();
        state.recorded_count = n;
        for i in 0..(n as usize).min(TAS_MAX_TICKS) {
            state.input_log[i] = (i + 1) as u8 ;
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 0.5];
        }
        state
    }

    // ===== Phase 2a: entry_id, pin, soft cap, store bridge =====

    #[test]
    fn entry_ids_unique_and_monotonic() {
        let mut h = RecordingHistory::new(16);
        for i in 0..5 {
            assert!(h.push_snapshot(&state_with_ticks(10 + i), format!("S{}", i)));
        }
        let ids: Vec<u64> = h.entries().iter().map(|e| e.entry_id).collect();
        let mut uniq = ids.clone();
        uniq.sort();
        uniq.dedup();
        assert_eq!(uniq.len(), ids.len(), "ids unique");
        assert!(ids.windows(2).all(|w| w[0] < w[1]), "ids monotonic");
    }

    #[test]
    fn soft_cap_evicts_oldest_unpinned() {
        let mut h = RecordingHistory::new(3);
        for i in 0..5 {
            h.push_snapshot(&state_with_ticks(10 + i), format!("S{}", i));
        }
        assert_eq!(h.len(), 3, "capped to 3 unpinned");
        let labels: Vec<&str> = h.entries().iter().map(|e| e.label.as_str()).collect();
        assert_eq!(labels, vec!["S2", "S3", "S4"], "oldest unpinned evicted");
    }

    #[test]
    fn pinned_survive_eviction() {
        let mut h = RecordingHistory::new(3);
        h.push_snapshot(&state_with_ticks(10), "A");
        let a_id = h.entries()[0].entry_id;
        assert!(h.set_pinned(a_id, true));
        for i in 0..5 {
            h.push_snapshot(&state_with_ticks(20 + i), format!("U{}", i));
        }
        assert!(
            h.entries().iter().any(|e| e.entry_id == a_id),
            "pinned A survived"
        );
        assert_eq!(
            h.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "unpinned still capped"
        );
    }

    #[test]
    fn current_entry_never_evicted() {
        let mut h = RecordingHistory::new(3);
        for lbl in ["A", "B", "C"] {
            h.push_snapshot(&state_with_ticks(10), lbl);
        }
        h.restore_index(0); // select the oldest
        let a_id = h.entries()[0].entry_id;
        h.capacity = 2; // tighten below the count
        h.enforce_capacity();
        assert!(
            h.entries().iter().any(|e| e.entry_id == a_id),
            "current (oldest) entry not evicted"
        );
    }

    #[test]
    fn set_capacity_trims_immediately() {
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        assert_eq!(h.len(), 6);
        h.set_capacity(3);
        assert_eq!(
            h.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "lowering the cap trims unpinned immediately"
        );
    }

    #[test]
    fn all_pinned_over_cap_keeps_all() {
        let mut h = RecordingHistory::new(4);
        for i in 0..4 {
            h.push_snapshot(&state_with_ticks(10), format!("P{}", i));
        }
        let ids: Vec<u64> = h.entries().iter().map(|e| e.entry_id).collect();
        for id in &ids {
            h.set_pinned(*id, true);
        }
        h.capacity = 2;
        h.enforce_capacity();
        assert_eq!(h.len(), 4, "all-pinned kept despite cap 2");
    }

    #[test]
    fn rename_and_pin_bump_revision_only_on_change() {
        let mut h = RecordingHistory::new(8);
        h.push_snapshot(&state_with_ticks(5), "A");
        let id = h.entries()[0].entry_id;
        let r0 = h.revision();
        assert!(h.rename(id, "A2"));
        assert!(h.revision() > r0);
        let r1 = h.revision();
        assert!(h.set_pinned(id, true));
        assert!(h.revision() > r1);
        // no-op rename + pin must NOT bump
        let r2 = h.revision();
        h.rename(id, "A2");
        h.set_pinned(id, true);
        assert_eq!(h.revision(), r2, "no-op meta change doesn't bump");
    }

    #[test]
    fn bridge_roundtrips_through_v2_store() {
        use crate::history_store_v2::HistoryStoreV2;
        let dir = std::env::temp_dir().join(format!(
            "ssb_bridge_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));

        let mut h = RecordingHistory::new(16);
        h.push_snapshot(&state_with_ticks(5), "A");
        h.push_snapshot(&state_with_ticks(7), "B");
        h.push_save_marker(&state_with_ticks(7), Path::new("run.tasrec"));
        let b_id = h.entries()[1].entry_id;
        h.set_pinned(b_id, true);
        h.rename(b_id, "B renamed");
        h.restore_index(1);
        let cur = h.current_entry_id();

        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s2, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        assert_eq!(h2.len(), h.len());
        let b2 = h2.entries().iter().find(|e| e.entry_id == b_id).unwrap();
        assert_eq!(b2.custom_name.as_deref(), Some("B renamed"));
        assert!(b2.pinned);
        assert_eq!(h2.current_entry_id(), cur);
        assert!(h2.next_entry_id() > b_id, "next id continues past loaded max");
        // snapshot content survives the round-trip
        let a2 = h2.entries().iter().find(|e| e.label == "A").unwrap();
        assert!(a2.can_restore());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn apply_persisted_bumps_revision() {
        // Migration relies on this: apply_persisted -> revision bumps ->
        // the revision-gated writer persists the migrated history.
        let persisted = {
            let mut src = RecordingHistory::new(8);
            src.push_snapshot(&state_with_ticks(5), "A");
            src.to_persisted()
        };
        let mut h = RecordingHistory::new(8);
        let r0 = h.revision();
        h.apply_persisted(persisted).unwrap();
        assert!(h.revision() > r0, "apply_persisted must bump revision");
    }

    #[test]
    fn custom_name_roundtrips_and_clears() {
        use crate::history_store_v2::HistoryStoreV2;
        let dir = std::env::temp_dir().join(format!(
            "ssb_name_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(16);
        h.push_snapshot(&state_with_ticks(5), "Recorded 0:05");
        let id = h.entries()[0].entry_id;
        assert!(h.rename(id, "  my best run  ")); // trims whitespace

        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), h.current_entry_id(), h.next_entry_id())
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        let e = h2.entries().iter().find(|e| e.entry_id == id).unwrap();
        assert_eq!(e.custom_name.as_deref(), Some("my best run"));
        assert_eq!(e.label, "Recorded 0:05", "auto label preserved alongside name");

        // Blank rename clears the custom name.
        assert!(h2.rename(id, "   "));
        assert_eq!(
            h2.entries().iter().find(|e| e.entry_id == id).unwrap().custom_name,
            None
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn push_save_marker_bumps_revision() {
        let mut h = RecordingHistory::new(8);
        h.push_snapshot(&state_with_ticks(5), "A");
        let r = h.revision();
        h.push_save_marker(&state_with_ticks(5), Path::new("x.tasrec"));
        assert!(h.revision() > r, "save marker must bump revision");
    }

    #[test]
    fn unpin_triggers_eviction() {
        let mut h = RecordingHistory::new(2);
        h.push_snapshot(&state_with_ticks(5), "A");
        let a = h.entries()[0].entry_id;
        h.set_pinned(a, true);
        h.push_snapshot(&state_with_ticks(5), "B");
        h.push_snapshot(&state_with_ticks(5), "C"); // current=C; A pinned + B,C unpinned
        assert_eq!(h.len(), 3);
        h.set_pinned(a, false); // now unpinned A,B,C = 3 > cap 2 -> evict oldest (A)
        assert!(
            !h.entries().iter().any(|e| e.entry_id == a),
            "unpinned-over-cap A evicted"
        );
        assert_eq!(h.entries().iter().filter(|e| !e.pinned).count(), 2);
    }

    #[test]
    fn apply_loaded_lowered_cap_trims() {
        use crate::history_store_v2::HistoryStoreV2;
        let dir = std::env::temp_dir().join(format!(
            "ssb_captrim_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        let cur = h.current_entry_id();
        let (mut store, _) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s, res) = HistoryStoreV2::open_in(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(3); // smaller cap
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);
        assert_eq!(
            h2.entries().iter().filter(|e| !e.pinned).count(),
            3,
            "lowered cap trims unpinned on load"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn history_append_only_after_undo_preserves_all_entries() {
        // Board requirement: pressing from a previous state must NOT discard later entries.
        let mut history = RecordingHistory::new(16);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);
        let s4 = state_with_ticks(40);

        assert!(history.push_snapshot(&s1, "Rec 10"));
        assert!(history.push_snapshot(&s2, "Rec 20"));
        assert!(history.push_snapshot(&s3, "Rec 30"));
        assert_eq!(history.len(), 3);

        // Undo to s2
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 20);
        assert_eq!(history.current_index(), Some(1));

        // Push new entry from this undo point — must NOT truncate s3
        assert!(history.push_snapshot(&s4, "Rec 40"));
        assert_eq!(history.len(), 4); // [s1, s2, s3, s4] — all preserved
        assert_eq!(history.entries()[0].label, "Rec 10");
        assert_eq!(history.entries()[1].label, "Rec 20");
        assert_eq!(history.entries()[2].label, "Rec 30"); // NOT deleted
        assert_eq!(history.entries()[3].label, "Rec 40");
        assert_eq!(history.current_index(), Some(3));
    }

    #[test]
    fn history_double_undo_then_push_preserves_all() {
        let mut history = RecordingHistory::new(16);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);
        let s4 = state_with_ticks(40);
        let s5 = state_with_ticks(50);

        for (s, label) in [(&s1, "A"), (&s2, "B"), (&s3, "C"), (&s4, "D")] {
            assert!(history.push_snapshot(s, label));
        }
        assert_eq!(history.len(), 4);

        // Undo twice: D -> C -> B
        history.undo().unwrap();
        history.undo().unwrap();
        assert_eq!(history.current_index(), Some(1)); // on B

        // Push E — all of [A, B, C, D, E] should exist
        assert!(history.push_snapshot(&s5, "E"));
        assert_eq!(history.len(), 5);
        assert_eq!(history.entries()[2].label, "C"); // preserved
        assert_eq!(history.entries()[3].label, "D"); // preserved
        assert_eq!(history.entries()[4].label, "E"); // new
    }

    #[test]
    fn history_undo_redo_full_cycle_restores_correct_data() {
        let mut history = RecordingHistory::new(8);
        let s1 = state_with_ticks(5);
        let s2 = state_with_ticks(15);
        let s3 = state_with_ticks(25);

        assert!(history.push_snapshot(&s1, "5 ticks"));
        assert!(history.push_snapshot(&s2, "15 ticks"));
        assert!(history.push_snapshot(&s3, "25 ticks"));

        // Undo to 15
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 15);

        // Undo to 5
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 5);

        // Can't undo further
        assert!(history.undo().is_none());
        assert!(!history.can_undo());

        // Redo back to 15
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 15);

        // Redo back to 25
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 25);

        // Can't redo further
        assert!(history.redo().is_none());
        assert!(!history.can_redo());
    }

    #[test]
    fn history_save_marker_after_undo_does_not_shift_selection() {
        let mut history = RecordingHistory::new(16);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);

        assert!(history.push_snapshot(&s1, "A"));
        assert!(history.push_snapshot(&s2, "B"));
        assert!(history.push_snapshot(&s3, "C"));

        // Undo to B
        history.undo().unwrap();
        assert_eq!(history.current_index(), Some(1));

        // Save marker should not change current_index
        history.push_save_marker(&s2, Path::new("test.tasrec"));
        assert_eq!(history.current_index(), Some(1)); // still on B
        assert_eq!(history.len(), 4); // A, B, SaveMarker, C
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert!(!history.entries()[2].can_restore());
    }

    #[test]
    fn history_capacity_eviction_under_undo_keeps_valid_cursor() {
        let mut history = RecordingHistory::new(3);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);
        let s4 = state_with_ticks(40);

        assert!(history.push_snapshot(&s1, "A"));
        assert!(history.push_snapshot(&s2, "B"));
        assert!(history.push_snapshot(&s3, "C"));
        assert_eq!(history.len(), 3);
        assert_eq!(history.current_index(), Some(2)); // on C

        // Undo to B
        history.undo().unwrap();
        assert_eq!(history.current_index(), Some(1)); // on B

        // Push D — eviction should happen (capacity=3, will have 4 before eviction)
        assert!(history.push_snapshot(&s4, "D"));
        assert_eq!(history.len(), 3); // A evicted
                                      // current_index should be valid and point to D
        let idx = history.current_index().unwrap();
        assert_eq!(history.entries()[idx].label, "D");
    }

    #[test]
    fn history_zero_recorded_count_is_rejected() {
        let mut history = RecordingHistory::new(8);
        let empty = zeroed_state(); // recorded_count = 0
        assert!(!history.push_snapshot(&empty, "Empty"));
        assert_eq!(history.len(), 0);
    }

    #[test]
    fn history_persist_round_trip_after_undo_preserves_all() {
        let mut history = RecordingHistory::new(16);
        let s1 = state_with_ticks(10);
        let s2 = state_with_ticks(20);
        let s3 = state_with_ticks(30);

        assert!(history.push_snapshot(&s1, "A"));
        assert!(history.push_snapshot(&s2, "B"));
        assert!(history.push_snapshot(&s3, "C"));
        history.undo().unwrap(); // cursor on B

        // Persist and restore
        let persisted = history.to_persisted();
        assert_eq!(persisted.entries.len(), 3); // all three persisted
        assert_eq!(persisted.current_index, Some(1)); // cursor on B

        let mut restored = RecordingHistory::new(16);
        restored.apply_persisted(persisted).unwrap();
        assert_eq!(restored.len(), 3);
        assert_eq!(restored.current_index(), Some(1));
        assert_eq!(restored.entries()[0].label, "A");
        assert_eq!(restored.entries()[1].label, "B");
        assert_eq!(restored.entries()[2].label, "C"); // C preserved through round-trip
        assert!(restored.can_undo());
        assert!(restored.can_redo());
    }

    #[test]
    fn history_restore_index_validates_bounds() {
        let mut history = RecordingHistory::new(4);
        let s1 = state_with_ticks(10);
        assert!(history.push_snapshot(&s1, "A"));

        assert!(history.restore_index(99).is_none()); // out of bounds
        assert_eq!(history.current_index(), Some(0)); // unchanged
    }

    #[test]
    fn history_mixed_workflow_rec_save_undo_load_continue() {
        // Simulates a real user workflow:
        // 1. Record 10 ticks
        // 2. Record 20 ticks
        // 3. Save file
        // 4. Undo to 10 ticks
        // 5. Record 15 ticks (continue)
        // 6. Load a file
        // All entries should be preserved.
        let mut history = RecordingHistory::new(32);
        let s10 = state_with_ticks(10);
        let s20 = state_with_ticks(20);
        let s15 = state_with_ticks(15);
        let s_loaded = state_with_ticks(50);

        // Step 1-2: Two recording sessions
        assert!(history.push_snapshot(&s10, "Recorded 0:00.10"));
        assert!(history.push_snapshot(&s20, "Recorded 0:00.20"));

        // Step 3: Save
        history.push_save_marker(&s20, Path::new("run.tasrec"));

        // Step 4: Undo to 10 ticks
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 10);

        // Step 5: Continue from undo point — appends, doesn't truncate
        assert!(history.push_snapshot(&s15, "Continued from 0:00.10, total 0:00.15"));

        // Step 6: Load a file
        assert!(history.push_loaded_snapshot(&s_loaded, Path::new("other.tasrec")));

        // Trace: [s10, s20] → save marker at idx 2 → [s10, s20, SaveMarker]
        // undo → current=0 → push s15 → [s10, s20, SaveMarker, s15] current=3
        // push loaded → [s10, s20, SaveMarker, s15, loaded] current=4
        assert_eq!(history.len(), 5);
        assert_eq!(history.entries()[0].label, "Recorded 0:00.10");
        assert_eq!(history.entries()[1].label, "Recorded 0:00.20");
        assert_eq!(history.entries()[2].kind, HistoryEntryKind::SaveMarker);
        assert_eq!(
            history.entries()[3].label,
            "Continued from 0:00.10, total 0:00.15"
        );
        assert!(history.entries()[4].label.contains("other.tasrec"));
        assert_eq!(history.entries()[4].kind, HistoryEntryKind::LoadSnapshot);

        // Can undo all the way back: s15(3), s20(1), s10(0) = 3 restorable before current(4)
        assert_eq!(history.undo_depth(), 3);
    }

    #[test]
    fn format_recording_duration_boundary_values() {
        // 0 ticks
        assert_eq!(format_recording_duration(0), "0:00.00");
        // 1 tick = 0.01s
        assert_eq!(format_recording_duration(1), "0:00.01");
        // 99 ticks = 0.99s
        assert_eq!(format_recording_duration(99), "0:00.99");
        // 100 ticks = 1.00s
        assert_eq!(format_recording_duration(100), "0:01.00");
        // 5999 ticks = 59.99s (just under 1 minute)
        assert_eq!(format_recording_duration(5999), "0:59.99");
        // 6000 ticks = 1:00.00
        assert_eq!(format_recording_duration(6000), "1:00.00");
        // 359999 ticks = 59:59.99 (just under 1 hour)
        assert_eq!(format_recording_duration(359999), "59:59.99");
        // 360000 ticks = 1:00:00.00
        assert_eq!(format_recording_duration(360000), "1:00:00.00");
        // Large value: 65536 ticks (the original bug case)
        assert_eq!(format_recording_duration(65536), "10:55.36");
    }

    #[test]
    fn completed_session_label_zero_length_is_none() {
        assert!(completed_session_label(RecordingSessionKind::Rec, 50, 50).is_none());
        assert!(completed_session_label(RecordingSessionKind::Rec, 50, 49).is_none());
        assert!(completed_session_label(RecordingSessionKind::Continue, 100, 100).is_none());
    }

    #[test]
    fn completed_session_label_continue_shows_start_and_total() {
        let label = completed_session_label(RecordingSessionKind::Continue, 6000, 12000).unwrap();
        assert_eq!(label, "Continued from 1:00.00, total 2:00.00");
    }

    #[test]
    fn history_load_snapshot_is_undoable() {
        let mut history = RecordingHistory::new(8);
        let s1 = state_with_ticks(10);
        let s_loaded = state_with_ticks(50);

        assert!(history.push_snapshot(&s1, "Recording"));
        assert!(history.push_loaded_snapshot(&s_loaded, Path::new("loaded.tasrec")));

        // Should be able to undo back to the recording
        assert!(history.can_undo());
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 10);

        // And redo back to the loaded file
        assert!(history.can_redo());
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 50);
    }

    #[test]
    fn history_multiple_undo_push_cycles_never_lose_data() {
        // Stress test: repeatedly undo and push, verify entry count only grows
        let mut history = RecordingHistory::new(64);
        let states: Vec<_> = (1..=10).map(|n| state_with_ticks(n * 5)).collect();

        // Push 5 entries
        for (i, s) in states[..5].iter().enumerate() {
            assert!(history.push_snapshot(s, format!("S{}", i)));
        }
        assert_eq!(history.len(), 5);

        // Undo 3 times, push new
        history.undo().unwrap();
        history.undo().unwrap();
        history.undo().unwrap();
        assert!(history.push_snapshot(&states[5], "S5"));
        assert_eq!(history.len(), 6); // all 5 + new one

        // Undo 2 times, push another
        history.undo().unwrap();
        history.undo().unwrap();
        assert!(history.push_snapshot(&states[6], "S6"));
        assert_eq!(history.len(), 7); // none lost

        // Verify first entry is still intact
        assert_eq!(history.entries()[0].label, "S0");
    }
}
