use crate::history_store_v2::BlobRef;
use crate::ui_log::UiLog;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

const TAS_TICKS_PER_SECOND: u32 = 100;
// Crash-recovery checkpoint cadence while recording. Each checkpoint is a FULL
// rewrite of the recording so far (~13 bytes/tick), so a fixed interval makes
// total write volume ~quadratic in length. We instead save LESS often as the
// run grows — a phase table that trades a little more max-loss-on-crash for
// much less churn, hard-capped so a crash never loses more than the late
// interval. (250ms/4x-sec was the original wild value; real tools autosave on
// the order of seconds-to-minutes.)
//
// Phase     tick range        interval   max crash loss
// early     0..12_000  (<2m)  1.5s       1.5s
// mid       12_000..48_000    5s         5s
// late      48_000..cap (8m+) 10s        10s
// Worst case over a full ~11-min recording: ~170 writes / ~51 MiB (vs ~437 /
// ~177 MiB at a flat 1.5s), with crash loss bounded at 10s.
const RECOVERY_DEBOUNCE_EARLY_MS: u64 = 1_500;
const RECOVERY_DEBOUNCE_MID_MS: u64 = 5_000;
const RECOVERY_DEBOUNCE_LATE_MS: u64 = 10_000;
const RECOVERY_PHASE_MID_TICKS: u32 = 12_000; // ~2 min at 100 ticks/sec
const RECOVERY_PHASE_LATE_TICKS: u32 = 48_000; // ~8 min
/// Early-phase interval; also the value the production `RecoveryStore::new`
/// constructs with. A zero debounce (test-only) disables throttling entirely.
const DEFAULT_RECOVERY_DEBOUNCE_MS: u64 = RECOVERY_DEBOUNCE_EARLY_MS;
const MAX_TASREC_METADATA_BYTES: usize = 1024 * 1024;
const MAX_TASREC_BYTES: u64 = (4 + MAX_TASREC_METADATA_BYTES + TAS_MAX_TICKS * (1 + 3 * 4)) as u64;

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

/// The race time of a session that ended at the finish line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FinishStamp {
    /// Centiseconds (= game ticks at 100 Hz).
    pub cs: u32,
    /// `true` = the HUD timer's value; `false` = derived from the recording's
    /// start-line / finish-line crossings (within ~0.05 s of the game's timer).
    pub exact: bool,
}

/// Display form of a finish time: "3:50.57" when exact, "~3:50.62" when
/// geometry-derived, so the label never overstates its precision.
pub fn format_finish_time(cs: u32, exact: bool) -> String {
    if exact {
        format_recording_duration(cs)
    } else {
        format!("~{}", format_recording_duration(cs))
    }
}

/// Label for a session that ended by crossing the finish line, so the entry
/// reads "Finish 0:53.34" rather than by its length.
pub fn finished_session_label(finish: FinishStamp) -> String {
    format!("Finish {}", format_finish_time(finish.cs, finish.exact))
}

/// Race time (centiseconds) of a finished run from the recording alone, for
/// when the DLL's HUD race-timer feed is empty: the in-game timer runs from
/// the START-LINE trigger to the finish line, so use the geometric start
/// crossing when the track has one; a recording that never crossed a start
/// line (unknown track) falls back to first movement, then to tick 0.
pub fn geometry_race_time_cs(
    finish_tick: u32,
    start_cross_tick: Option<u32>,
    first_moving: Option<u32>,
) -> u32 {
    let start = start_cross_tick.or(first_moving).unwrap_or(0);
    finish_tick.saturating_sub(start)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySessionContext {
    pub kind: RecordingSessionKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub label: String,
    /// The track this recording was made on, captured while RECORDING.
    ///
    /// A recovered checkpoint is pushed into history during app startup, before
    /// anything has read the live level — so it was always stamped untagged, and
    /// untagged entries show on EVERY track. Since recovered entries are also
    /// pinned, the visible result was "my favourites from Forest Easy are
    /// showing while I'm on Forest Medium", which is the exact bug report this
    /// whole line of work started from. The level is known when the checkpoint
    /// is written, so it is carried here rather than re-derived later.
    ///
    /// `serde(default)` so checkpoints written before this field still load.
    #[serde(default)]
    pub level: Option<String>,
    /// Physics and rider stamps captured while RECORDING (v43 fix): the
    /// checkpoint writer rebuilds a zeroed shared state, so without these the
    /// recovered .tasrec and history entry carried no renderer / precision /
    /// rider stamp and mismatch detection was silently off for them. All
    /// `serde(default)` so older checkpoints still load.
    #[serde(default)]
    pub fpu_control_word: Option<u32>,
    #[serde(default)]
    pub renderer_id: Option<u32>,
    #[serde(default)]
    pub rider_character: Option<u32>,
    #[serde(default)]
    pub rider_stance: Option<u32>,
}

impl RecoverySessionContext {
    pub fn from_ticks(kind: RecordingSessionKind, start_tick: u32, end_tick: u32) -> Option<Self> {
        let label = completed_session_label(kind, start_tick, end_tick)?;
        Some(Self {
            kind,
            start_tick,
            end_tick,
            label,
            level: None,
            fpu_control_word: None,
            renderer_id: None,
            rider_character: None,
            rider_stance: None,
        })
    }

    /// Carry the live physics / rider stamps: `(fpu_control_word, renderer_id,
    /// (rider_character, rider_stance))` as read coherently from the live
    /// shared state. Unknown halves stay `None`.
    pub fn with_stamps(mut self, live: Option<(u32, u32, (u32, u32))>) -> Self {
        if let Some((fpu, renderer, (character, stance))) = live {
            self.fpu_control_word = (fpu != 0).then_some(fpu);
            self.renderer_id = (renderer != tas_shared::TAS_RENDERER_UNKNOWN).then_some(renderer);
            self.rider_character =
                (character != tas_shared::TAS_CHARACTER_UNKNOWN).then_some(character);
            self.rider_stance = (stance != u32::MAX).then_some(stance);
        }
        self
    }

    /// Write the carried stamps into a rebuilt shared state so a save from it
    /// stamps the file exactly as a live save would.
    pub fn apply_stamps(&self, state: &mut tas_shared::TasSharedState) {
        if let Some(v) = self.fpu_control_word {
            state.fpu_control_word = v;
        }
        if let Some(v) = self.renderer_id {
            state.renderer_id = v;
        }
        if let Some(v) = self.rider_character {
            state.rider_character = v;
        }
        if let Some(v) = self.rider_stance {
            state.rider_stance = v;
        }
    }

    /// The rider stamp as the history entry shows it ("Keith · goofy").
    pub fn rider_label(&self) -> Option<String> {
        tas_shared::rider_label(
            self.rider_character
                .unwrap_or(tas_shared::TAS_CHARACTER_UNKNOWN),
            self.rider_stance.unwrap_or(u32::MAX),
        )
    }

    /// Record which track this is, for when the checkpoint comes back as a
    /// history entry. See the `level` field.
    pub fn with_level(mut self, level: Option<&str>) -> Self {
        self.level = level.map(str::to_string);
        self
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
    pub session: RecoverySessionContext,
    pub snapshot: RecordingSnapshot,
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
        let root = crate::settings::data_root_dir().join("recovery");
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

    /// Minimum spacing between checkpoint writes at the given recording length.
    /// Grows with length (see the phase-table constants) so a long run writes
    /// less often, while a crash still loses at most one late-phase interval.
    /// A zero base debounce (test-only) disables throttling entirely.
    fn effective_debounce(&self, recorded_count: u32) -> Duration {
        if self.debounce.is_zero() {
            return Duration::ZERO;
        }
        let ms = match recorded_count {
            0..RECOVERY_PHASE_MID_TICKS => RECOVERY_DEBOUNCE_EARLY_MS,
            RECOVERY_PHASE_MID_TICKS..RECOVERY_PHASE_LATE_TICKS => RECOVERY_DEBOUNCE_MID_MS,
            _ => RECOVERY_DEBOUNCE_LATE_MS,
        };
        Duration::from_millis(ms)
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
                if last_write_at.elapsed() < self.effective_debounce(snapshot.recorded_count) {
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
        self.session.apply_stamps(&mut state);

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

/// Serialized off-thread recovery-checkpoint writer. Replaces the old
/// fire-and-forget `thread::spawn` per write, which had no ordering and could
/// land AFTER `clear_pending()` — resurrecting a stale checkpoint into a
/// duplicate "Recovered" entry on the next launch. Jobs are processed in
/// submission order; within a coalesced batch only the newest job runs (older
/// ones are superseded on disk anyway). `flush()` is a drain barrier: it blocks
/// until every queued job has been written, so the caller can safely
/// `clear_pending()` afterwards with no in-flight write able to recreate the
/// files.
pub struct RecoveryWriter {
    tx: Option<Sender<RecoveryMsg>>,
    worker: Option<std::thread::JoinHandle<()>>,
}

enum RecoveryMsg {
    Write(RecoveryWriteJob),
    Flush(Sender<()>),
}

impl RecoveryWriter {
    pub fn new() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<RecoveryMsg>();
        let worker = std::thread::Builder::new()
            .name("recovery-writer".into())
            .spawn(move || {
                while let Ok(msg) = rx.recv() {
                    // Coalesce a burst: keep only the newest write (older
                    // checkpoints are superseded), then answer any flushes.
                    let mut latest: Option<RecoveryWriteJob> = None;
                    let mut acks: Vec<Sender<()>> = Vec::new();
                    let mut next = Some(msg);
                    while let Some(m) = next {
                        match m {
                            RecoveryMsg::Write(job) => latest = Some(job),
                            RecoveryMsg::Flush(ack) => acks.push(ack),
                        }
                        next = rx.try_recv().ok();
                    }
                    if let Some(job) = latest {
                        if let Err(e) = job.write() {
                            eprintln!("[recovery] checkpoint write failed: {}", e);
                        }
                    }
                    for ack in acks {
                        let _ = ack.send(());
                    }
                }
            })
            .ok();
        Self {
            tx: Some(tx),
            worker,
        }
    }

    pub fn submit(&self, job: RecoveryWriteJob) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(RecoveryMsg::Write(job));
        }
    }

    /// Block until every queued checkpoint write has hit disk. Call this before
    /// `RecoveryStore::clear_pending()` so no late write resurrects the files.
    pub fn flush(&self) {
        if let Some(tx) = &self.tx {
            let (a, r) = std::sync::mpsc::channel();
            if tx.send(RecoveryMsg::Flush(a)).is_ok() {
                let _ = r.recv();
            }
        }
    }
}

impl Default for RecoveryWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for RecoveryWriter {
    fn drop(&mut self) {
        self.flush();
        self.tx = None;
        if let Some(w) = self.worker.take() {
            let _ = w.join();
        }
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
        let _ = RecordingFile::load(&mut state, &self.recording_path).map_err(|e| {
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
            session: metadata.session,
            snapshot: RecordingSnapshot::from_state(&state),
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

/// Atomically replace `final_path` with `temp_path`. `std::fs::rename` performs
/// an atomic replace of an EXISTING destination (MoveFileExW with
/// MOVEFILE_REPLACE_EXISTING on Windows; `rename(2)` on Unix) — the destination
/// is always old-or-new, never missing. This mirrors the v2 store's manifest
/// publish (proven in production, replacing `manifest.json` every persist).
///
/// The previous remove-then-rename fallback opened a crash window where the
/// destination was briefly absent — exactly the wrong property for a
/// crash-recovery file — so it is gone. On failure the temp is cleaned up.
fn atomic_replace_file(temp_path: &Path, final_path: &Path) -> Result<(), String> {
    std::fs::rename(temp_path, final_path).map_err(|e| {
        let _ = std::fs::remove_file(temp_path);
        format!(
            "failed to replace {} with {}: {}",
            final_path.display(),
            temp_path.display(),
            e
        )
    })
}

fn write_file_atomically(path: &Path, data: &[u8]) -> Result<(), String> {
    let temp = temp_path_for(path);
    let write_result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)
            .map_err(|e| format!("failed to create {}: {}", temp.display(), e))?;
        file.write_all(data)
            .map_err(|e| format!("failed to write {}: {}", temp.display(), e))?;
        file.sync_all()
            .map_err(|e| format!("failed to flush {}: {}", temp.display(), e))?;
        drop(file);
        atomic_replace_file(&temp, path)
    })();
    if write_result.is_err() {
        let _ = std::fs::remove_file(&temp);
    }
    write_result
}

fn remove_if_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(path).map_err(|e| format!("failed to remove {}: {}", path.display(), e))
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
    pub force_fixed_tick: u32,
    pub max_drift_x: f32,
    pub max_drift_z: f32,
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

pub struct RecordingFile;

impl RecordingFile {
    pub fn save_with_segments(
        state: &TasSharedState,
        path: &std::path::Path,
        segments: &[Segment],
    ) -> Result<(), String> {
        let count = state.recorded_count as usize;
        if count == 0 {
            return Err("Nothing recorded".into());
        }
        if count > TAS_MAX_TICKS {
            return Err(format!("Recording too long: {} ticks", count));
        }

        let meta = RecordingMetadata {
            version: state.version,
            recorded_count: state.recorded_count,
            force_fixed_tick: state.force_fixed_tick,
            max_drift_x: state.max_drift_x,
            max_drift_z: state.max_drift_z,
            timestamp: chrono::Local::now().to_rfc3339(),
            notes: String::new(),
            segments: segments.to_vec(),
            renderer: (state.renderer_id != tas_shared::TAS_RENDERER_UNKNOWN)
                .then(|| tas_shared::renderer_name(state.renderer_id).to_string()),
            fpu_control_word: (state.fpu_control_word != 0).then_some(state.fpu_control_word),
            character: (state.rider_character != tas_shared::TAS_CHARACTER_UNKNOWN)
                .then(|| tas_shared::character_name(state.rider_character).to_string()),
            stance: (state.rider_stance != u32::MAX).then_some(state.rider_stance),
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

        write_file_atomically(path, &data)
    }

    /// Parse only the JSON header of a `.tasrec` (same bounds as `load`).
    pub fn read_metadata(path: &std::path::Path) -> Result<RecordingMetadata, String> {
        let file = File::open(path).map_err(|e| format!("{}", e))?;
        let mut head = Vec::new();
        file.take((4 + MAX_TASREC_METADATA_BYTES) as u64)
            .read_to_end(&mut head)
            .map_err(|e| format!("{}", e))?;
        if head.len() < 4 {
            return Err("File too small".into());
        }
        let meta_len = u32::from_le_bytes([head[0], head[1], head[2], head[3]]) as usize;
        if meta_len > MAX_TASREC_METADATA_BYTES {
            return Err(format!("Recording metadata too large: {} bytes", meta_len));
        }
        let end = 4usize
            .checked_add(meta_len)
            .ok_or_else(|| "Recording metadata length overflow".to_string())?;
        if head.len() < end {
            return Err("Truncated metadata".into());
        }
        serde_json::from_slice::<RecordingMetadata>(&head[4..end]).map_err(|e| format!("{}", e))
    }

    pub fn load(
        state: &mut TasSharedState,
        path: &std::path::Path,
    ) -> Result<(u32, Vec<Segment>), String> {
        let file = File::open(path).map_err(|e| format!("{}", e))?;
        let reported_len = file.metadata().map_err(|e| format!("{}", e))?.len();
        if reported_len > MAX_TASREC_BYTES {
            return Err(format!(
                "Recording file too large: {} bytes (maximum {})",
                reported_len, MAX_TASREC_BYTES
            ));
        }
        let mut data = Vec::with_capacity(reported_len as usize);
        file.take(MAX_TASREC_BYTES + 1)
            .read_to_end(&mut data)
            .map_err(|e| format!("{}", e))?;
        if data.len() as u64 > MAX_TASREC_BYTES {
            return Err(format!(
                "Recording file too large: more than {} bytes",
                MAX_TASREC_BYTES
            ));
        }
        if data.len() < 4 {
            return Err("File too small".into());
        }

        let meta_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        if meta_len > MAX_TASREC_METADATA_BYTES {
            return Err(format!("Recording metadata too large: {} bytes", meta_len));
        }
        let metadata_end = 4usize
            .checked_add(meta_len)
            .ok_or_else(|| "Recording metadata length overflow".to_string())?;
        if data.len() < metadata_end {
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

        let input_start = metadata_end;
        let input_end = input_start
            .checked_add(count)
            .ok_or_else(|| "Recording input length overflow".to_string())?;
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
        let coords_end = coords_start
            .checked_add(coords_size)
            .ok_or_else(|| "Recording coordinate length overflow".to_string())?;
        if data.len() >= coords_end {
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
        state.force_fixed_tick = 0; // always force fft=0 (proven zero-drift config)

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
    /// `recorded_count` is clamped to `TAS_MAX_TICKS`: the source arrays are
    /// fixed-size, so a corrupt/over-range count from shared memory must never
    /// panic-slice this hot path (history + recovery snapshotting).
    fn capture_from(&mut self, state: &TasSharedState) {
        let count = (state.recorded_count as usize).min(TAS_MAX_TICKS);
        self.recorded_count = count as u32;
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
        persisted.validate()?;
        let count = persisted.recorded_count as usize;
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

/// Where an entry's recording lives. Entries loaded from the v2 store start
/// `OnDisk`: the panel only needs the metadata, and a restore reads the blob
/// then. Keeping every snapshot resident cost a fixed 852 KB per entry (533
/// entries = 454 MB at startup, measured 2026-09-02) and made every persist
/// clone all of it on the UI thread.
pub(crate) enum SnapshotSlot {
    /// Marker entry (save/load landmark): nothing to restore.
    Marker,
    /// Resident. `on_disk` is set once the store holds this exact blob, which
    /// makes the resident copy droppable (`demote_resident_except`).
    Loaded {
        snapshot: RecordingSnapshot,
        on_disk: Option<BlobRef>,
    },
    /// Restorable; read from the store on demand.
    OnDisk(BlobRef),
    /// Blob missing/corrupt: visible but inert. The store keeps the blob
    /// reference itself (so a reappearing file recovers), nothing is needed here.
    Unavailable,
}

impl SnapshotSlot {
    fn restorable(&self) -> bool {
        matches!(self, SnapshotSlot::Loaded { .. } | SnapshotSlot::OnDisk(_))
    }

    fn loaded(&self) -> Option<&RecordingSnapshot> {
        match self {
            SnapshotSlot::Loaded { snapshot, .. } => Some(snapshot),
            _ => None,
        }
    }
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
    /// Full local-tz creation time. Used for date grouping in the panel.
    pub created_at: chrono::DateTime<chrono::Local>,
    pub kind: HistoryEntryKind,
    /// Session start tick: 0 for REC entries (recording from the beginning)
    /// or for markers, the resume tick for CONT entries.
    pub start_tick: u32,
    /// `recorded_count` at push time. Zero on markers.
    pub end_tick: u32,
    /// First tick where the recorded position diverged from `rec_coords[0]`
    /// — the "race-start" landmark. `None` for markers and snapshots with no
    /// detected movement.
    pub first_moving: Option<u32>,
    /// Race time in centiseconds when the session ended by crossing the
    /// finish line (the HUD timer the finish-line watch auto-stopped at).
    /// `None` = the session was stopped by hand. Drives the
    /// "Finish m:ss.cc" label and the flag in the panel.
    pub finish_time_cs: Option<u32>,
    /// `true` when `finish_time_cs` came from the HUD timer (exact); `false`
    /// when it was derived from the recording's start-line / finish-line
    /// crossings (within ~0.05 s of the game's timer - shown with a "~").
    pub finish_time_exact: bool,
    /// Level code (e.g. "FE") the entry was created on, from the DLL's live
    /// level_id at push time. `None` when the level was unknown (menu). Used by
    /// the panel's per-level filter.
    pub level: Option<String>,
    /// Physics-mode stamp at push time (`tas_shared::physics_mode_label`,
    /// e.g. `OpenGL/53-bit`). None = unknown / pre-stamp entry. Restoring an
    /// entry under a different mode replays different physics.
    pub physics: Option<String>,
    /// Rider stamp at push time (`tas_shared::rider_label`, e.g.
    /// `Vincent · goofy`). None = unknown / pre-stamp entry. Restoring an
    /// entry recorded as another character or stance replays different
    /// physics.
    pub rider: Option<String>,
    snapshot: SnapshotSlot,
}

impl HistoryEntry {
    fn from_snapshot(label: String, kind: HistoryEntryKind, snapshot: RecordingSnapshot) -> Self {
        let now = chrono::Local::now();
        let end_tick = snapshot.recorded_count;
        let first_moving =
            tas_shared::cont::detect_first_moving(snapshot.rec_coords.as_ref(), end_tick);
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            created_at: now,
            kind,
            // start_tick is overwritten by `with_session` for CONT entries
            // that know their resume point; REC entries leave it at 0.
            start_tick: 0,
            end_tick,
            first_moving,
            finish_time_cs: None, // set by push_completed_session for finished runs
            finish_time_exact: false,
            level: None,   // stamped from live_level by RecordingHistory on push
            physics: None, // stamped from live_physics by RecordingHistory on push
            rider: None,   // stamped from live_rider by RecordingHistory on push
            snapshot: SnapshotSlot::Loaded {
                snapshot,
                on_disk: None,
            },
        }
    }

    fn marker(label: String, kind: HistoryEntryKind) -> Self {
        let now = chrono::Local::now();
        Self {
            entry_id: 0, // assigned by RecordingHistory on push
            pinned: false,
            custom_name: None,
            label,
            created_at: now,
            kind,
            start_tick: 0,
            end_tick: 0,
            first_moving: None,
            finish_time_cs: None,
            finish_time_exact: false,
            level: None,   // stamped from live_level by RecordingHistory on push
            physics: None, // stamped from live_physics by RecordingHistory on push
            rider: None,   // stamped from live_rider by RecordingHistory on push
            snapshot: SnapshotSlot::Marker,
        }
    }

    fn with_session(mut self, start_tick: u32, end_tick: u32) -> Self {
        self.start_tick = start_tick;
        self.end_tick = end_tick;
        self
    }

    pub fn can_restore(&self) -> bool {
        self.snapshot.restorable()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PersistedSnapshot {
    pub recorded_count: u32,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
}

impl PersistedSnapshot {
    /// Validate the structural invariants a snapshot blob must satisfy before
    /// it can be restored. The v2 store calls this at load so a blob that
    /// deserializes but is semantically bogus (e.g. tampered/forged
    /// `recorded_count`) is treated as corrupt — NOT silently demoted to a
    /// marker by the bridge's `from_persisted(..).ok()`.
    pub fn validate(&self) -> Result<(), String> {
        let count = self.recorded_count as usize;
        if count > TAS_MAX_TICKS {
            return Err(format!(
                "persisted snapshot too large: {} ticks (max {})",
                count, TAS_MAX_TICKS
            ));
        }
        if self.input_log.len() != count {
            return Err(format!(
                "persisted input_log length mismatch: expected {}, got {}",
                count,
                self.input_log.len()
            ));
        }
        if self.rec_coords.len() != count {
            return Err(format!(
                "persisted rec_coords length mismatch: expected {}, got {}",
                count,
                self.rec_coords.len()
            ));
        }
        Ok(())
    }
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
    /// Current level code (e.g. "FE") from the DLL's live level_id; the app
    /// refreshes it every frame. Stamped onto each entry at push time so the
    /// panel can filter history per level. None = unknown/menu.
    live_level: Option<String>,
    /// Set between a level change and the scan publishing the new track, so
    /// "we don't know yet" is distinguishable from "we are on this track".
    level_resolving: bool,
    /// v2 store directory; where `OnDisk` entries are read from on restore.
    blob_dir: Option<PathBuf>,
    /// Problems hit while reading blobs on demand (the app logs and clears).
    warnings: Vec<String>,
    /// Live physics-mode stamp from the DLL (renderer + x87 precision),
    /// refreshed by the app every frame; stamped onto pushed entries.
    live_physics: Option<String>,
    /// Live rider stamp (character · stance), see `set_live_rider`.
    live_rider: Option<String>,
}

impl RecordingHistory {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            entries: Vec::with_capacity(capacity.max(1)),
            current_index: None,
            next_entry_id: 1,
            revision: 0,
            live_level: None,
            level_resolving: false,
            blob_dir: None,
            warnings: Vec::new(),
            live_physics: None,
            live_rider: None,
        }
    }

    /// Refresh the physics-mode stamp given to subsequently pushed entries.
    pub fn set_live_physics(&mut self, label: Option<String>) {
        if label.is_some() && self.live_physics != label {
            self.live_physics = label;
        }
    }

    pub fn live_physics(&self) -> Option<&str> {
        self.live_physics.as_deref()
    }

    /// Live rider stamp (character · stance). Like the physics stamp, an
    /// unknown live value never erases a known one.
    pub fn set_live_rider(&mut self, label: Option<String>) {
        if label.is_some() && self.live_rider != label {
            self.live_rider = label;
        }
    }

    pub fn live_rider(&self) -> Option<&str> {
        self.live_rider.as_deref()
    }

    /// Where lazily-loaded entries read their blobs from. Set before
    /// `apply_loaded`; without it an `OnDisk` entry cannot be restored.
    pub fn set_blob_dir(&mut self, dir: PathBuf) {
        self.blob_dir = Some(dir);
    }

    pub fn take_warnings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.warnings)
    }

    /// Make entry `index` resident (reading its blob if needed) and return it.
    /// A blob that fails to read turns the entry `Unavailable` and records a
    /// warning; the cursor is NOT moved here so a failed restore leaves the
    /// selection where it was.
    fn load_slot(&mut self, index: usize) -> Option<&RecordingSnapshot> {
        let blob = match &self.entries[index].snapshot {
            SnapshotSlot::Loaded { .. } => return self.entries[index].snapshot.loaded(),
            SnapshotSlot::OnDisk(blob) => *blob,
            SnapshotSlot::Marker | SnapshotSlot::Unavailable => return None,
        };
        let id = self.entries[index].entry_id;
        let loaded = self
            .blob_dir
            .as_deref()
            .ok_or_else(|| "history store directory unknown".to_string())
            .and_then(|dir| crate::history_store_v2::load_blob(dir, id, blob))
            .and_then(RecordingSnapshot::from_persisted);
        match loaded {
            Ok(snapshot) => {
                self.entries[index].snapshot = SnapshotSlot::Loaded {
                    snapshot,
                    on_disk: Some(blob),
                };
                self.entries[index].snapshot.loaded()
            }
            Err(e) => {
                self.warnings.push(format!(
                    "history entry {} ('{}') cannot be restored: {}",
                    id, self.entries[index].label, e
                ));
                self.entries[index].snapshot = SnapshotSlot::Unavailable;
                None
            }
        }
    }

    /// Drop resident copies the store already holds, except `keep` and the
    /// current entry. Restores are rare user actions and a blob re-reads in
    /// about a millisecond, so "at most the one you just used" is enough.
    fn demote_resident_except(&mut self, keep: usize) {
        let current = self.current_index;
        for (i, e) in self.entries.iter_mut().enumerate() {
            if i == keep || current == Some(i) {
                continue;
            }
            if let SnapshotSlot::Loaded {
                on_disk: Some(blob),
                ..
            } = &e.snapshot
            {
                let blob = *blob;
                e.snapshot = SnapshotSlot::OnDisk(blob);
            }
        }
    }

    /// The writer committed `id`'s blob: its resident copy is now droppable,
    /// and is dropped unless it is the current entry.
    pub fn mark_durable(&mut self, id: u64, blob: BlobRef) {
        let Some(i) = self.entries.iter().position(|e| e.entry_id == id) else {
            return;
        };
        if let SnapshotSlot::Loaded { on_disk, .. } = &mut self.entries[i].snapshot {
            if on_disk.is_none() {
                *on_disk = Some(blob);
            }
        }
        if self.current_index != Some(i) {
            if let SnapshotSlot::Loaded {
                on_disk: Some(blob),
                ..
            } = &self.entries[i].snapshot
            {
                let blob = *blob;
                self.entries[i].snapshot = SnapshotSlot::OnDisk(blob);
            }
        }
    }

    /// Refresh the level code stamped onto subsequently pushed entries and
    /// used by the panel's per-level view. STICKY: `None` (menu / unknown) is
    /// ignored so the panel keeps showing the track you were just on — you're
    /// almost always between restarts of the same level, and entries pushed
    /// at the menu (e.g. a save marker right after a run) still belong to it.
    /// Not a history mutation — does not bump the revision.
    pub fn set_live_level(&mut self, level: Option<&str>) {
        if let Some(code) = level {
            if self.live_level.as_deref() != Some(code) {
                self.live_level = Some(code.to_owned());
            }
            // A concrete reading ends any transition.
            self.level_resolving = false;
        }
    }

    /// We are in a level context whose track has not been identified yet.
    ///
    /// Driven by the DLL's `level_epoch` / `level_scan_epoch` pair: the engine's
    /// root object survives an F5 restart but is reallocated on quit-to-menu /
    /// menu-demo / track switch, so a root change is the only trustworthy "the
    /// level was swapped" event. Until a scan identifies the NEW track we must
    /// not keep asserting the old one — that filtered the panel to the wrong
    /// track and stamped entries pushed mid-load with it.
    ///
    /// A wrong tag is worse than no tag: an untagged entry is visibly untagged,
    /// a confidently mis-stamped one is indistinguishable from a correct one.
    ///
    /// Idempotent — called every frame while unresolved.
    pub fn enter_resolving(&mut self) {
        self.live_level = None;
        self.level_resolving = true;
    }

    /// True between a level change and the scan publishing the new track. The
    /// panel should say so rather than assert a track it cannot currently know.
    pub fn level_is_resolving(&self) -> bool {
        self.level_resolving
    }

    /// The level code new entries are currently stamped with (None = unknown).
    pub fn live_level(&self) -> Option<&str> {
        self.live_level.as_deref()
    }

    /// Backfill level tags on entries persisted before tagging existed, by
    /// classifying each snapshot's spawn position (rec_coords[0]). Only
    /// unambiguous spawns are tagged (the classifier refuses shared clusters),
    /// so a wrong tag can't hide an entry from its real level. Returns the
    /// number of entries tagged.
    pub fn backfill_levels<F>(&mut self, classify: F) -> usize
    where
        F: Fn(&[f32; 3]) -> Option<&'static str>,
    {
        let mut tagged = 0;
        for i in 0..self.entries.len() {
            if self.entries[i].level.is_some() {
                continue;
            }
            // Only the spawn coordinate is needed. For an on-disk entry read
            // the blob transiently rather than making it resident: this runs
            // once per untagged entry, and the tag is persisted afterwards.
            let spawn = match &self.entries[i].snapshot {
                SnapshotSlot::Loaded { snapshot, .. } => {
                    (snapshot.recorded_count > 0).then_some(snapshot.rec_coords[0])
                }
                SnapshotSlot::OnDisk(blob) => self.blob_dir.as_deref().and_then(|dir| {
                    crate::history_store_v2::load_blob(dir, self.entries[i].entry_id, *blob)
                        .ok()
                        .and_then(|ps| ps.rec_coords.first().copied())
                }),
                SnapshotSlot::Marker | SnapshotSlot::Unavailable => None,
            };
            let Some(spawn) = spawn else {
                continue;
            };
            if let Some(code) = classify(&spawn) {
                self.entries[i].level = Some(code.to_string());
                tagged += 1;
            }
        }
        if tagged > 0 {
            self.bump(); // persist the new tags
        }
        tagged
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_entry_id;
        self.next_entry_id = self
            .next_entry_id
            .checked_add(1)
            .expect("entry_id overflow");
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

    /// Raise the next-id allocator to at least `floor`. Used on startup so an
    /// EMPTY load (a valid empty manifest, or a corrupt manifest that preserved
    /// blobs) still advances past any id that already exists on disk — otherwise
    /// the next new entry restarts at 1 and collides with a preserved blob.
    pub fn adopt_id_floor(&mut self, floor: u64) {
        if floor > self.next_entry_id {
            self.next_entry_id = floor;
            self.bump();
        }
    }

    pub fn push_snapshot(&mut self, state: &TasSharedState, label: impl Into<String>) -> bool {
        self.push_snapshot_data(RecordingSnapshot::from_state(state), label)
    }

    pub fn push_snapshot_data(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            None,
            None,
        )
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
            None,
        )
    }

    /// A REC / CONT session that just ended. `finish` is the race time when
    /// the finish-line watch stopped it (the entry then shows the flag and
    /// "Finish m:ss.cc", whatever it is later renamed to); `None` for a
    /// session stopped by hand.
    pub fn push_completed_session(
        &mut self,
        snapshot: RecordingSnapshot,
        label: impl Into<String>,
        start_tick: u32,
        end_tick: u32,
        finish: Option<FinishStamp>,
    ) -> bool {
        self.push_snapshot_entry(
            snapshot,
            label.into(),
            HistoryEntryKind::Snapshot,
            Some((start_tick, end_tick)),
            finish,
        )
    }

    pub fn push_loaded_snapshot(&mut self, state: &TasSharedState, path: &Path) -> bool {
        let label = format!("Load: {}", short_file_label(path));
        self.push_snapshot_entry(
            RecordingSnapshot::from_state(state),
            label,
            HistoryEntryKind::LoadSnapshot,
            None,
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
        marker.level = self.live_level.clone();
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

    /// Whether entry `i` belongs to the track we are currently on — the SAME
    /// rule the history panel filters by (matching level, or untagged).
    ///
    /// undo/redo must honour it: they walk snapshots directly, so without this
    /// Ctrl+Z on Forest Medium happily restores a Forest Easy recording over the
    /// live buffer. That is the per-level guarantee failing in the one path that
    /// bypasses the list the user can actually see.
    pub fn entry_on_current_level(&self, i: usize) -> bool {
        if i >= self.entries.len() {
            return false;
        }
        // While RESOLVING we do not know what track we are on, so nothing
        // qualifies. Treating "no live level" as allow-all made the transition
        // window show AND restore every track — strictly worse than the bug this
        // began as, and hoisting the sync earlier only widened it.
        if self.level_resolving {
            return false;
        }
        match (self.live_level.as_deref(), self.entries[i].level.as_deref()) {
            (Some(want), Some(have)) => want == have,
            // Untagged: entries from before per-level tagging, and entries made
            // while the track was unknown. Deliberately allowed everywhere —
            // they carry no claim about where they belong, and hiding them would
            // strand the user's existing history. The guarantee is therefore
            // "no recording TAGGED with another track", not "nothing unknown".
            _ => true,
        }
    }

    pub fn undo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let prev = (0..current)
            .rev()
            .find(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))?;
        // Load BEFORE moving the cursor: an unreadable blob must not leave the
        // selection on an entry that just turned inert.
        self.load_slot(prev)?;
        self.current_index = Some(prev);
        self.demote_resident_except(prev);
        self.bump();
        self.entries[prev].snapshot.loaded()
    }

    pub fn redo(&mut self) -> Option<&RecordingSnapshot> {
        let current = self.current_index?;
        let next = ((current + 1)..self.entries.len())
            .find(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))?;
        self.load_slot(next)?;
        self.current_index = Some(next);
        self.demote_resident_except(next);
        self.bump();
        self.entries[next].snapshot.loaded()
    }

    pub fn restore_index(&mut self, index: usize) -> Option<&RecordingSnapshot> {
        if index >= self.entries.len() {
            return None;
        }
        // The same rule undo/redo and the row filter use. It was missing HERE —
        // in the one path the user actually clicks. The panel hides other-track
        // rows, so in practice a wrong row was hard to reach, but the index
        // arrives from the UI and was trusted without checking, which made the
        // whole per-level guarantee rest on a display filter. It does not any
        // more.
        if !self.entry_on_current_level(index) {
            return None;
        }
        self.load_slot(index)?;
        self.current_index = Some(index);
        self.demote_resident_except(index);
        self.bump();
        self.entries[index].snapshot.loaded()
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
                meta: crate::history_store_v2::EntryMeta {
                    entry_id: e.entry_id,
                    name: e.label.clone(),
                    user_name: e.custom_name.clone(),
                    pinned: e.pinned,
                    kind: e.kind,
                    start_tick: e.start_tick,
                    end_tick: e.end_tick,
                    first_moving: e.first_moving,
                    finish_time_cs: e.finish_time_cs,
                    finish_time_exact: e.finish_time_exact,
                    level: e.level.clone(),
                    physics: e.physics.clone(),
                    rider: e.rider.clone(),
                    created_at_iso: e.created_at.to_rfc3339(),
                },
                // Bytes travel only for snapshots the store does not have
                // yet. On-disk and durable entries send `None`; the store
                // keeps their blob reference (ids are never reused).
                snapshot: match &e.snapshot {
                    SnapshotSlot::Loaded {
                        snapshot,
                        on_disk: None,
                    } => Some(snapshot.to_persisted()),
                    _ => None,
                },
            })
            .collect()
    }

    /// The stable id of the current (selected) entry, if any.
    pub fn current_entry_id(&self) -> Option<u64> {
        self.current_index.map(|i| self.entries[i].entry_id)
    }

    /// Tag an entry with the track it belongs to.
    ///
    /// For recovered checkpoints, which carry their level from when they were
    /// RECORDED — at restore time (app startup) the live level is not known yet.
    /// A `None` level leaves the entry untagged, i.e. visible everywhere, which
    /// is the honest state when the checkpoint predates level stamping.
    pub fn set_level(&mut self, entry_id: u64, level: Option<String>) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.level == level {
            return false;
        }
        e.level = level;
        self.bump();
        true
    }

    /// Stamp the rider a recovered entry was recorded as (the checkpoint
    /// carries it; the live rider is unknown during startup). See set_level.
    pub fn set_rider(&mut self, entry_id: u64, rider: Option<String>) -> bool {
        let Some(e) = self.entries.iter_mut().find(|e| e.entry_id == entry_id) else {
            return false;
        };
        if e.rider == rider {
            return false;
        }
        e.rider = rider;
        self.bump();
        true
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
            let snapshot = match (le.snapshot, le.blob, le.available) {
                (Some(ps), blob, _) => match RecordingSnapshot::from_persisted(ps) {
                    Ok(snapshot) => SnapshotSlot::Loaded {
                        snapshot,
                        on_disk: blob,
                    },
                    Err(_) => {
                        if blob.is_some() {
                            SnapshotSlot::Unavailable
                        } else {
                            SnapshotSlot::Marker
                        }
                    }
                },
                (None, Some(blob), true) => SnapshotSlot::OnDisk(blob),
                (None, Some(_), false) => SnapshotSlot::Unavailable,
                (None, None, _) => SnapshotSlot::Marker,
            };
            let meta = le.meta;
            let created_at = chrono::DateTime::parse_from_rfc3339(&meta.created_at_iso)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Local))
                .unwrap_or_else(chrono::Local::now);
            entries.push(HistoryEntry {
                entry_id: meta.entry_id,
                pinned: meta.pinned,
                custom_name: meta.user_name,
                label: meta.name,
                created_at,
                kind: meta.kind,
                start_tick: meta.start_tick,
                end_tick: meta.end_tick,
                first_moving: meta.first_moving,
                finish_time_cs: meta.finish_time_cs,
                finish_time_exact: meta.finish_time_exact,
                level: meta.level,
                physics: meta.physics,
                rider: meta.rider,
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
            .filter(|&i| self.entries[i].can_restore());
        // A lowered cap (e.g. settings changed between sessions) trims on load.
        self.enforce_capacity_preserving_none();
        self.bump();
    }

    pub fn undo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        (0..current)
            .filter(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))
            .count()
    }

    pub fn redo_depth(&self) -> usize {
        let Some(current) = self.current_index else {
            return 0;
        };
        ((current + 1)..self.entries.len())
            .filter(|&i| self.entries[i].can_restore() && self.entry_on_current_level(i))
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
                .find_map(|(idx, entry)| entry.can_restore().then_some(idx));
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
        finish: Option<FinishStamp>,
    ) -> bool {
        if snapshot.recorded_count == 0 {
            return false;
        }

        let mut entry = HistoryEntry::from_snapshot(label, kind, snapshot);
        entry.entry_id = self.alloc_id();
        entry.level = self.live_level.clone();
        entry.physics = self.live_physics.clone();
        entry.rider = self.live_rider.clone();
        entry.finish_time_cs = finish.map(|f| f.cs);
        entry.finish_time_exact = finish.is_some_and(|f| f.exact);
        if let Some((start_tick, end_tick)) = session {
            entry = entry.with_session(start_tick, end_tick);
        }
        self.entries.push(entry);
        let newest = self.entries.len() - 1;
        self.current_index = Some(newest);
        // The previous take is no longer current: if the store already holds
        // its blob, its resident 852 KB copy can go.
        self.demote_resident_except(newest);
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

/// Default save name. The `<level>-<time>` convention (e.g. `FE-5876`) needs the
/// level id + finish time from the game (pending the game-awareness RE); until
/// then we default to a timestamp so saves still land somewhere sensible.
/// `level` is the track this RECORDING belongs to, supplied by the caller.
///
/// Deliberately not read live here. A recording belongs to the track it was
/// recorded on, not to whatever the game happens to be showing when the user
/// gets around to clicking Save — and by then the engine may well be sitting in
/// its post-run dialog, where the cycle is frozen and the level reads UNKNOWN.
/// Reading live would strand exactly the recording the user just finished as
/// untagged. `None` still degrades safely to a time-only name.
pub fn save_dialog_with_segments(
    state: &TasSharedState,
    segments: &[Segment],
    log: &mut UiLog,
    level: Option<&str>,
) -> Option<PathBuf> {
    // Default name: `<level>-<time>` (e.g. FE-5876). Mis-tagging is permanent —
    // the level filter keys off the saved name and folder — so an unknown level
    // degrades to a time-only name in the root folder, which is recoverable.
    // Time is the in-race duration (gate→end) in cs.
    let race_cs = crate::level::race_centiseconds(&state.rec_coords, state.recorded_count);
    let default_name = crate::level::default_recording_name(level, race_cs);
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Save TAS Recording")
        .set_directory(recordings_dir_for_level(level))
        .set_file_name(default_name)
        .add_filter("TAS Recording", &["tasrec"])
        .save_file()
    {
        match RecordingFile::save_with_segments(state, &path, segments) {
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
/// Split from the load so the caller can STOP an active recording between the
/// (cancellable) pick and the buffer-overwriting load — picking then loading
/// in one call would force a stop-before-dialog that a cancel would waste.
/// `level_id` selects the starting folder. None = the user cancelled.
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

    fn persist_checkpoint(
        store: &mut RecoveryStore,
        state: &TasSharedState,
        segments: &[Segment],
        session: &RecoverySessionContext,
        force: bool,
    ) -> Result<bool, String> {
        let snapshot = RecordingSnapshot::from_state(state);
        let Some(job) = store.take_write_job(&snapshot, segments, session, force) else {
            return Ok(false);
        };
        job.write()?;
        Ok(true)
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
        assert_eq!(history.undo_depth(), 0);
        assert_eq!(history.redo_depth(), 0);
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

        assert!(persist_checkpoint(&mut store, &state, &segments, &session, true).unwrap());
        let pending = store.load_pending().unwrap().expect("expected checkpoint");
        assert_eq!(pending.snapshot.recorded_count, 5);
        assert_eq!(pending.session.label, "Recorded 0:00.05");

        store.clear_pending().unwrap();
        assert!(store.load_pending().unwrap().is_none());
        let _ = std::fs::remove_dir_all(root);
    }

    /// #7: a `recorded_count` beyond the fixed buffer size (corrupt shared
    /// memory / a misbehaving DLL) must clamp, not panic-slice the hot path.
    #[test]
    fn from_state_clamps_overlong_recorded_count() {
        let mut state = zeroed_state();
        state.recorded_count = TAS_MAX_TICKS as u32 + 50;
        let snap = RecordingSnapshot::from_state(&state);
        assert_eq!(
            snap.recorded_count as usize, TAS_MAX_TICKS,
            "clamped to max"
        );
    }

    /// The checkpoint cadence stretches as the recording grows (early→mid→late),
    /// so a long run writes less often — while a zero base debounce (test mode)
    /// stays unthrottled.
    #[test]
    fn recovery_debounce_grows_with_length() {
        let prod = RecoveryStore::new_in_root(
            unique_temp_root("tas_ui_debounce_phases"),
            Duration::from_millis(RECOVERY_DEBOUNCE_EARLY_MS),
        )
        .unwrap();
        assert_eq!(prod.effective_debounce(0).as_millis(), 1_500);
        assert_eq!(prod.effective_debounce(11_999).as_millis(), 1_500);
        assert_eq!(prod.effective_debounce(12_000).as_millis(), 5_000);
        assert_eq!(prod.effective_debounce(47_999).as_millis(), 5_000);
        assert_eq!(prod.effective_debounce(48_000).as_millis(), 10_000);
        assert_eq!(prod.effective_debounce(65_536).as_millis(), 10_000);

        // Test-mode zero base stays unthrottled at every length.
        let test =
            RecoveryStore::new_in_root(unique_temp_root("tas_ui_debounce_zero"), Duration::ZERO)
                .unwrap();
        assert!(test.effective_debounce(0).is_zero());
        assert!(test.effective_debounce(60_000).is_zero());
    }

    /// #2: `adopt_id_floor` raises the allocator (never lowers it), so an empty
    /// load that nonetheless has a high stored next-id can't reissue old ids.
    #[test]
    fn adopt_id_floor_only_raises() {
        let mut h = RecordingHistory::new(8);
        h.adopt_id_floor(50);
        assert_eq!(h.next_entry_id(), 50);
        h.adopt_id_floor(10);
        assert_eq!(h.next_entry_id(), 50, "floor never lowers the allocator");
    }

    /// #3: the serialized recovery writer's `flush()` is a real drain barrier —
    /// after it returns the checkpoint is on disk, so a following `clear_pending`
    /// can't race an in-flight write that would resurrect the files.
    #[test]
    fn recovery_writer_flush_drains_before_clear() {
        let root = unique_temp_root("tas_ui_recovery_writer_drain");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = zeroed_state();
        state.recorded_count = 3;
        state.input_log[0] = 9;
        let snap = RecordingSnapshot::from_state(&state);
        let session = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 3).unwrap();
        let job = store
            .take_write_job(&snap, &[], &session, true)
            .expect("job");

        let writer = RecoveryWriter::new();
        writer.submit(job);
        writer.flush(); // blocks until the write lands

        assert!(
            store.load_pending().unwrap().is_some(),
            "checkpoint durable once flush returns"
        );
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

        assert!(persist_checkpoint(&mut store, &state, &[], &session3, true).unwrap());
        assert!(!persist_checkpoint(&mut store, &state, &[], &session3, false).unwrap());

        state.recorded_count = 4;
        state.input_log[3] = 0x08;
        let session4 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 4).unwrap();
        assert!(persist_checkpoint(&mut store, &state, &[], &session4, false).unwrap());

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
        assert!(persist_checkpoint(&mut store, &state, &[], &session2, true).unwrap());

        state.recorded_count = 6;
        for i in 2..6 {
            state.input_log[i] = 0x08;
        }
        let session6 = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 6).unwrap();
        assert!(persist_checkpoint(&mut store, &state, &[], &session6, true).unwrap());

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
        state.force_fixed_tick = 0;
        state.max_drift_x = 0.0;
        state.max_drift_z = 0.0;

        for i in 0..10 {
            state.input_log[i] = (i as u8) & 0x3F;
            state.rec_coords[i] = [i as f32 * 1.0, i as f32 * 2.0, i as f32 * 3.0];
        }

        let path = unique_temp_path("rec_rt", "tasrec");

        RecordingFile::save_with_segments(&state, &path, &[]).unwrap();

        let mut loaded = zeroed_state();
        let (count, segments) = RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(count, 10);
        assert!(segments.is_empty());
        assert_eq!(loaded.recorded_count, 10);
        assert_eq!(loaded.force_fixed_tick, 0); // always forced to 0 on load

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
        assert!(RecordingFile::save_with_segments(&state, &path, &[]).is_err());
    }

    #[test]
    fn lazy_history_entries_restore_from_store_and_stay_off_heap() {
        use crate::history_store_v2::{HistoryStoreV2, StoredEntry};
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_lazy_hist_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mk = |id: u64, count: u32| {
            let mut input_log = vec![0u8; count as usize];
            input_log[0] = id as u8;
            StoredEntry {
                meta: crate::history_store_v2::EntryMeta {
                    entry_id: id,
                    name: format!("take {}", id),
                    user_name: None,
                    pinned: false,
                    kind: HistoryEntryKind::Snapshot,
                    start_tick: 0,
                    end_tick: count,
                    first_moving: None,
                    finish_time_cs: None,
                    finish_time_exact: false,
                    level: None,
                    physics: None,
                    rider: None,
                    created_at_iso: "2026-09-02T00:00:00+10:00".to_string(),
                },
                snapshot: Some(PersistedSnapshot {
                    recorded_count: count,
                    input_log,
                    rec_coords: vec![[id as f32, 0.0, 0.0]; count as usize],
                }),
            }
        };
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store.persist(&[mk(1, 3), mk(2, 5)], Some(2), 3).unwrap();
        drop(store);

        let (_lazy, load) = HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        let mut history = RecordingHistory::new(8);
        history.set_blob_dir(dir.clone());
        history.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(history.len(), 2);
        assert!(history.entries().iter().all(|e| e.can_restore()));
        assert!(
            history
                .entries()
                .iter()
                .all(|e| matches!(e.snapshot, SnapshotSlot::OnDisk(_))),
            "nothing resident after a lazy load"
        );
        // Re-persisting sends no bytes for on-disk entries.
        assert!(history
            .to_stored_entries()
            .iter()
            .all(|e| e.snapshot.is_none()));

        // Restore reads the blob on demand.
        let snap = history.restore_index(0).expect("restorable");
        assert_eq!(snap.recorded_count, 3);
        assert_eq!(snap.input_log[0], 1);
        assert!(matches!(
            history.entries()[0].snapshot,
            SnapshotSlot::Loaded { .. }
        ));
        // Restoring another entry drops the previous resident copy to disk.
        history.restore_index(1).expect("restorable");
        assert!(matches!(
            history.entries()[0].snapshot,
            SnapshotSlot::OnDisk(_)
        ));
        assert!(matches!(
            history.entries()[1].snapshot,
            SnapshotSlot::Loaded { .. }
        ));

        // A corrupt blob makes the entry inert with a warning; the cursor
        // stays on the entry that was current.
        let p = dir.join("1.tasrec");
        let mut bytes = std::fs::read(&p).unwrap();
        bytes[4] ^= 0xFF;
        std::fs::write(&p, &bytes).unwrap();
        assert!(history.restore_index(0).is_none());
        assert!(!history.entries()[0].can_restore());
        assert_eq!(history.current_index(), Some(1));
        assert!(!history.take_warnings().is_empty());

        // A session take stays resident until the writer reports it durable,
        // then drops once it is no longer current.
        let mut state = zeroed_state();
        state.recorded_count = 4;
        state.input_log[0] = 9;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take 3".to_string(),
            0,
            4
        ));
        let stored = history.to_stored_entries();
        assert!(stored[2].snapshot.is_some(), "new take travels with bytes");
        let blob = crate::history_store_v2::BlobRef {
            size: 0,
            checksum: 0,
        };
        history.mark_durable(stored[2].meta.entry_id, blob);
        assert!(
            matches!(history.entries()[2].snapshot, SnapshotSlot::Loaded { .. }),
            "current entry stays resident"
        );
        history.restore_index(1).expect("restorable");
        assert!(matches!(
            history.entries()[2].snapshot,
            SnapshotSlot::OnDisk(_)
        ));
        assert!(history.to_stored_entries()[2].snapshot.is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The rider stamp travels with the file: a Keith take loaded while
    /// Vincent is on the board warns (the physics differ per character), so
    /// does the other stance; the same rider stays quiet; an unstamped file
    /// has no opinion.
    #[test]
    fn recording_file_stamps_and_checks_the_rider() {
        let path = unique_temp_path("rec_rider", "tasrec");
        let mut state = zeroed_state();
        state.recorded_count = 2;
        state.rider_character = tas_shared::TAS_CHARACTER_KEITH;
        state.rider_stance = 0;
        RecordingFile::save_with_segments(&state, &path, &[]).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        assert_eq!(meta.stance, Some(0));
        assert_eq!(meta.rider_label().as_deref(), Some("Keith · regular"));

        let mut tracker = SegmentTracker::new();
        let mut live = zeroed_state();
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
        let mut other_stance = zeroed_state();
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
        let mut same = zeroed_state();
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

        let mut unstamped = zeroed_state();
        unstamped.recorded_count = 1;
        RecordingFile::save_with_segments(&unstamped, &path, &[]).unwrap();
        assert_eq!(
            RecordingFile::read_metadata(&path).unwrap().rider_label(),
            None
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn history_entries_carry_the_rider_stamp_through_the_store() {
        let mut history = RecordingHistory::new(8);
        history.set_live_rider(Some("Vincent · regular".to_string()));
        let mut state = zeroed_state();
        state.recorded_count = 3;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take".to_string(),
            0,
            3
        ));
        assert_eq!(
            history.entries()[0].rider.as_deref(),
            Some("Vincent · regular")
        );
        history.set_live_rider(None);
        assert_eq!(
            history.live_rider(),
            Some("Vincent · regular"),
            "unknown never erases known"
        );

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.rider.as_deref(), Some("Vincent · regular"));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_rider_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) =
            crate::history_store_v2::HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store_v2::HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        assert_eq!(
            load.entries[0].meta.rider.as_deref(),
            Some("Vincent · regular")
        );
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(
            reloaded.entries()[0].rider.as_deref(),
            Some("Vincent · regular")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn recording_file_stamps_and_reports_physics_mode() {
        let path = unique_temp_path("rec_physics", "tasrec");
        let mut state = zeroed_state();
        state.recorded_count = 2;
        state.renderer_id = tas_shared::TAS_RENDERER_OPENGL;
        state.fpu_control_word = 0x027F;
        RecordingFile::save_with_segments(&state, &path, &[]).unwrap();
        let meta = RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.fpu_control_word, Some(0x027F));
        assert_eq!(meta.physics_label().as_deref(), Some("OpenGL/53-bit"));

        // Loading it into a DirectX/24-bit game warns; the same mode stays quiet.
        let mut tracker = SegmentTracker::new();
        let mut log = UiLog::default();
        let mut live = zeroed_state();
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
        let mut same = zeroed_state();
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
        let mut unstamped = zeroed_state();
        unstamped.recorded_count = 1;
        RecordingFile::save_with_segments(&unstamped, &path, &[]).unwrap();
        assert_eq!(
            RecordingFile::read_metadata(&path).unwrap().physics_label(),
            None
        );
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn finished_session_label_is_the_race_time() {
        let exact = |cs| FinishStamp { cs, exact: true };
        let approx = |cs| FinishStamp { cs, exact: false };
        assert_eq!(finished_session_label(exact(5334)), "Finish 0:53.34");
        assert_eq!(finished_session_label(exact(23_546)), "Finish 3:55.46");
        // Geometry-derived times are marked: the planes sit ~5 cs from the
        // engine's triggers (3:50.62 measured vs the game's 3:50.57).
        assert_eq!(finished_session_label(approx(23_062)), "Finish ~3:50.62");
        assert_eq!(format_finish_time(23_057, true), "3:50.57");
        assert_eq!(format_finish_time(23_062, false), "~3:50.62");
    }

    #[test]
    fn geometry_race_time_runs_from_the_start_line_not_first_movement() {
        // Forest Easy coast, 2026-09-02: first moved at 288, crossed the start
        // line at ~789, finish watch fired at 23846; the game said 3:50.57.
        assert_eq!(geometry_race_time_cs(23_846, Some(789), Some(288)), 23_057);
        // No start line known for the track: first movement is the best guess.
        assert_eq!(geometry_race_time_cs(23_846, None, Some(288)), 23_558);
        assert_eq!(geometry_race_time_cs(100, None, None), 100);
        assert_eq!(
            geometry_race_time_cs(50, Some(80), None),
            0,
            "never negative"
        );
    }

    #[test]
    fn finished_sessions_keep_their_race_time_through_the_store() {
        let mut history = RecordingHistory::new(8);
        let mut state = zeroed_state();
        state.recorded_count = 3;
        let stamp = FinishStamp {
            cs: 5334,
            exact: true,
        };
        assert!(history.push_completed_session(
            RecordingSnapshot::from_state(&state),
            finished_session_label(stamp),
            0,
            3,
            Some(stamp)
        ));
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "Recorded 0:00.03".to_string(),
            0,
            3
        ));
        assert_eq!(history.entries()[0].finish_time_cs, Some(5334));
        assert!(history.entries()[0].finish_time_exact);
        assert_eq!(history.entries()[0].label, "Finish 0:53.34");
        assert_eq!(
            history.entries()[1].finish_time_cs,
            None,
            "a hand-stopped take is not a finish"
        );

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.finish_time_cs, Some(5334));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_finish_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) =
            crate::history_store_v2::HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store_v2::HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        assert_eq!(load.entries[0].meta.finish_time_cs, Some(5334));
        assert_eq!(load.entries[1].meta.finish_time_cs, None);
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(reloaded.entries()[0].finish_time_cs, Some(5334));
        assert!(reloaded.entries()[0].finish_time_exact);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn history_entries_carry_the_physics_stamp_through_the_store() {
        let mut history = RecordingHistory::new(8);
        history.set_live_physics(Some("OpenGL/53-bit".to_string()));
        let mut state = zeroed_state();
        state.recorded_count = 3;
        assert!(history.push_snapshot_data_with_session(
            RecordingSnapshot::from_state(&state),
            "take".to_string(),
            0,
            3
        ));
        assert_eq!(
            history.entries()[0].physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        // An unknown live mode (DLL not attached / not sampled yet) never
        // erases a known one.
        history.set_live_physics(None);
        assert_eq!(history.live_physics(), Some("OpenGL/53-bit"));

        let stored = history.to_stored_entries();
        assert_eq!(stored[0].meta.physics.as_deref(), Some("OpenGL/53-bit"));
        let dir = std::env::temp_dir().join(format!(
            "tas_ui_physics_{}_{}",
            std::process::id(),
            chrono::Local::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut store, _) =
            crate::history_store_v2::HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&stored, history.current_entry_id(), history.next_entry_id())
            .unwrap();
        drop(store);
        let (_s, load) = crate::history_store_v2::HistoryStoreV2::open_lazy(dir.clone()).unwrap();
        assert_eq!(
            load.entries[0].meta.physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        let mut reloaded = RecordingHistory::new(8);
        reloaded.set_blob_dir(dir.clone());
        reloaded.apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
        assert_eq!(
            reloaded.entries()[0].physics.as_deref(),
            Some("OpenGL/53-bit")
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn recording_file_atomically_replaces_existing_save() {
        let path = unique_temp_path("rec_replace", "tasrec");
        let mut first = zeroed_state();
        first.recorded_count = 2;
        first.input_log[..2].copy_from_slice(&[1, 2]);
        RecordingFile::save_with_segments(&first, &path, &[]).unwrap();

        let mut second = zeroed_state();
        second.recorded_count = 3;
        second.input_log[..3].copy_from_slice(&[7, 8, 9]);
        RecordingFile::save_with_segments(&second, &path, &[]).unwrap();

        let mut loaded = zeroed_state();
        RecordingFile::load(&mut loaded, &path).unwrap();
        assert_eq!(loaded.recorded_count, 3);
        assert_eq!(&loaded.input_log[..3], &[7, 8, 9]);
        let _ = std::fs::remove_file(path);
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
    fn recording_file_rejects_oversized_file_before_reading_it() {
        let path = unique_temp_path("rec_oversized", "tasrec");
        let file = File::create(&path).unwrap();
        file.set_len(MAX_TASREC_BYTES + 1).unwrap();
        drop(file);
        let mut state = zeroed_state();
        let error = match RecordingFile::load(&mut state, &path) {
            Ok(_) => panic!("oversized recording unexpectedly loaded"),
            Err(error) => error,
        };
        assert!(error.contains("too large"), "unexpected error: {error}");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn recording_file_with_segments_round_trip() {
        let mut state = zeroed_state();
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
        assert_eq!(loaded.force_fixed_tick, 0); // fft always forced to 0 on load
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
            state.input_log[i] = (i + 1) as u8;
            state.rec_coords[i] = [i as f32, 0.0, i as f32 * 0.5];
        }
        state
    }

    #[test]
    fn backfill_levels_tags_unambiguous_spawns_idempotently() {
        let mut h = RecordingHistory::new(8);
        // FE-spawn recording (unique cluster → taggable).
        let mut fe = state_with_ticks(10);
        fe.rec_coords[0] = [519.2, -1401.6, 53.6];
        assert!(h.push_snapshot(&fe, "fe run"));
        // Alpine-spawn recording (shared cluster → must stay untagged).
        let mut alpine = state_with_ticks(10);
        alpine.rec_coords[0] = [642.9, -842.2, 96.5];
        assert!(h.push_snapshot(&alpine, "alpine run"));
        // Simulate pre-tagging entries.
        for e in &mut h.entries {
            e.level = None;
        }

        let rev = h.revision();
        assert_eq!(
            h.backfill_levels(crate::start_line::level_code_from_spawn),
            1
        );
        assert_eq!(h.entries()[0].level.as_deref(), Some("FE"));
        assert_eq!(h.entries()[1].level, None);
        assert!(h.revision() > rev, "tagging must mark history dirty");

        // Idempotent: second pass tags nothing, revision untouched.
        let rev = h.revision();
        assert_eq!(
            h.backfill_levels(crate::start_line::level_code_from_spawn),
            0
        );
        assert_eq!(h.revision(), rev);
    }

    /// The bug the user hit: quit FE, start LOADING Forest Medium, and the
    /// panel still says FE — so it filters to FE and, worse, stamps anything
    /// pushed during the load with FE too.
    ///
    /// The transition is: known(FE) -> unknown (menu + level load + up to ~1.5s
    /// for the DLL heap scan to publish) -> known(FM). Stickiness carries "FE"
    /// through that whole middle window, and nothing distinguishes "we are
    /// confidently on FE" from "we have no idea yet".
    ///
    /// A wrong tag is worse than no tag: an untagged entry can be spotted and
    /// fixed later, a confidently mis-stamped one cannot.
    #[test]
    fn level_change_does_not_carry_the_old_level_into_the_new_one() {
        let mut h = RecordingHistory::new(8);
        h.set_live_level(Some("FE"));
        assert_eq!(h.live_level(), Some("FE"));

        // Player quits to the menu and starts loading Forest Medium. The DLL
        // publishes 0xFFFFFFFF throughout: menu, teardown, load, and the first
        // ~1.5s of the new track before the scan resolves.
        h.enter_resolving();

        assert_ne!(
            h.live_level(),
            Some("FE"),
            "after the level changed underneath us, FE is no longer a fact — \
             continuing to assert it filters the panel to the wrong track and \
             mis-stamps anything pushed during the load"
        );
        assert!(
            h.level_is_resolving(),
            "the transition window should be an explicit 'resolving' state, not \
             silently rendered as the previous level"
        );

        // Anything pushed while resolving must NOT be stamped with the old level.
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "mid-load");
        assert_eq!(
            h.entries().last().and_then(|e| e.level.as_deref()),
            None,
            "an entry pushed mid-transition must be untagged, never tagged FE"
        );

        // Scan resolves: now we genuinely know.
        h.set_live_level(Some("FM"));
        assert_eq!(h.live_level(), Some("FM"));
        assert!(!h.level_is_resolving());
    }

    /// Clicking a history row must not reach across tracks either.
    ///
    /// undo/redo were guarded and the panel filters its rows, so `restore_index`
    /// looked safe — but it took an index straight from the UI and trusted it,
    /// which left the whole per-level guarantee resting on a DISPLAY filter. A
    /// row that is stale by one frame, or any future caller that indexes the
    /// unfiltered list, walks straight through.
    #[test]
    fn restore_index_does_not_cross_levels() {
        let mut h = RecordingHistory::new(8);

        h.set_live_level(Some("FE"));
        let mut fe = RecordingSnapshot::new_empty();
        fe.recorded_count = 10;
        h.push_snapshot_data(fe, "fe-run");

        h.set_live_level(Some("FM"));
        let mut fm = RecordingSnapshot::new_empty();
        fm.recorded_count = 20;
        h.push_snapshot_data(fm, "fm-run");

        // Index 0 is the FE entry; we are on FM.
        assert_eq!(h.entries()[0].level.as_deref(), Some("FE"));
        assert!(
            h.restore_index(0).is_none(),
            "restoring another track's entry by index must be refused, not just \
             hidden from the list"
        );
        // ...and the refusal must not have moved the selection.
        assert_ne!(h.current_index(), Some(0));

        // The FM entry restores fine.
        assert!(h.restore_index(1).is_some());

        // While RESOLVING nothing qualifies — we do not know where we are, and
        // guessing is what caused the original bug.
        h.enter_resolving();
        assert!(
            h.restore_index(1).is_none(),
            "must refuse every entry while the track is unknown"
        );
    }

    /// A RECOVERED recording must not haunt every other track.
    ///
    /// This is the reported bug, traced to its actual cause. Recovered
    /// checkpoints are pushed into history during app STARTUP, before anything
    /// has read the live level, so `push_*` stamped them untagged — and
    /// untagged means visible on every track. Recovery also pins them, and
    /// pinned entries float to the top of their day. So the user's Forest Easy
    /// favourites sat at the top of the list while they were on Forest Medium.
    ///
    /// In the real history this was 16 of the 20 untagged entries — every one
    /// of them pinned. The fix carries the level in the checkpoint, from when it
    /// was recorded; this pins the behaviour that fix must produce.
    #[test]
    fn a_recovered_entry_is_tagged_and_does_not_leak_across_tracks() {
        let mut h = RecordingHistory::new(8);

        // Startup: nothing knows the level yet. This is the real ordering — the
        // recovery push happens in the constructor, before any level sync.
        h.set_live_level(None);
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "⟲");
        let id = h.entries().last().unwrap().entry_id;
        assert_eq!(
            h.entries().last().unwrap().level,
            None,
            "precondition: the push itself cannot know the level"
        );

        // The checkpoint knew: it was recorded on FE.
        h.set_level(id, Some("FE".to_string()));
        h.set_pinned(id, true);

        // On Forest Medium it must be gone — pinning must not exempt it.
        h.set_live_level(Some("FM"));
        let idx = h.entries().iter().position(|e| e.entry_id == id).unwrap();
        assert!(
            !h.entry_on_current_level(idx),
            "a recovered FE recording must not show (or restore) on FM, pinned or not"
        );
        assert!(h.restore_index(idx).is_none());

        // Back on FE it is available again.
        h.set_live_level(Some("FE"));
        assert!(h.entry_on_current_level(idx));
        assert!(h.restore_index(idx).is_some());
    }

    /// A checkpoint written before the level was carried still loads, and stays
    /// visible everywhere — the honest state for one we genuinely cannot place.
    #[test]
    fn a_recovered_entry_with_no_recorded_level_stays_untagged() {
        let mut h = RecordingHistory::new(8);
        h.set_live_level(None);
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = 10;
        h.push_snapshot_data(snap, "⟲");
        let id = h.entries().last().unwrap().entry_id;

        assert!(!h.set_level(id, None), "no-op when there is nothing to set");
        h.set_live_level(Some("VH"));
        let idx = h.entries().iter().position(|e| e.entry_id == id).unwrap();
        assert!(h.entry_on_current_level(idx));
    }

    /// Untagged entries stay restorable everywhere, deliberately.
    ///
    /// They predate per-level tagging (or were made while the track was
    /// unknown), so they carry no claim about where they belong; hiding them
    /// would strand the user's existing history. The guarantee is "no recording
    /// TAGGED with another track", not "nothing unknown" — worth pinning down so
    /// the looseness is a decision rather than an oversight.
    #[test]
    fn untagged_entries_remain_restorable() {
        let mut h = RecordingHistory::new(8);

        // Pushed with no live level => untagged.
        h.set_live_level(None);
        let mut legacy = RecordingSnapshot::new_empty();
        legacy.recorded_count = 10;
        h.push_snapshot_data(legacy, "legacy-run");
        assert_eq!(h.entries()[0].level, None);

        h.set_live_level(Some("VH"));
        assert!(
            h.restore_index(0).is_some(),
            "an untagged entry must stay reachable on any track"
        );

        // But not while we do not know the track at all.
        h.enter_resolving();
        assert!(h.restore_index(0).is_none());
    }

    /// Ctrl+Z must not reach across tracks. undo/redo walk snapshots directly
    /// rather than the filtered list the user sees, so without an explicit check
    /// they restore another level's recording over the live buffer — the
    /// per-level guarantee failing in the one path that bypasses the panel.
    #[test]
    fn undo_does_not_cross_levels() {
        let mut h = RecordingHistory::new(8);

        h.set_live_level(Some("FE"));
        let mut fe = RecordingSnapshot::new_empty();
        fe.recorded_count = 10;
        h.push_snapshot_data(fe, "fe-run");

        h.set_live_level(Some("FM"));
        let mut fm = RecordingSnapshot::new_empty();
        fm.recorded_count = 20;
        h.push_snapshot_data(fm, "fm-run");

        // On FM with only one FM entry, there is nothing to undo TO. The FE
        // entry is on another track and must not be offered.
        assert_eq!(
            h.undo_depth(),
            0,
            "an FE entry must not be undo-reachable from FM"
        );
        assert!(
            h.undo().is_none(),
            "Ctrl+Z must not restore another track's recording"
        );

        // Back on FE it is reachable again.
        h.set_live_level(Some("FE"));
        assert_eq!(h.undo_depth(), 1);
    }

    #[test]
    fn set_live_level_is_sticky_across_unknown() {
        let mut h = RecordingHistory::new(8);
        assert_eq!(h.live_level(), None);
        h.set_live_level(Some("FE"));
        assert_eq!(h.live_level(), Some("FE"));
        // Menu / unknown must NOT clear the last-known level.
        h.set_live_level(None);
        assert_eq!(h.live_level(), Some("FE"));
        h.set_live_level(Some("AM"));
        assert_eq!(h.live_level(), Some("AM"));
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
    fn ids_never_reused_after_eviction_and_reload() {
        use crate::history_store_v2::HistoryStoreV2;
        let dir = std::env::temp_dir().join(format!(
            "ssb_idreuse_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(3);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        // cap 3 + 6 pushes ⇒ ids 1,2,3 were front-evicted; 4,5,6 remain.
        let evicted = [1u64, 2, 3];
        let next_before = h.next_entry_id();

        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), h.current_entry_id(), next_before)
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(3);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        // A new push after reload must get a FRESH id, never an evicted one —
        // even though the evicted ids are now "gaps" below the surviving set.
        h2.push_snapshot(&state_with_ticks(5), "new");
        let new_id = h2
            .entries()
            .iter()
            .find(|e| e.label == "new")
            .unwrap()
            .entry_id;
        assert!(
            new_id >= next_before,
            "id {} reused (next was {})",
            new_id,
            next_before
        );
        assert!(
            !evicted.contains(&new_id),
            "id {} reused an EVICTED id",
            new_id
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pinned_and_current_survive_lowered_cap_on_load() {
        use crate::history_store_v2::HistoryStoreV2;
        let dir = std::env::temp_dir().join(format!(
            "ssb_pinload_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut h = RecordingHistory::new(10);
        for i in 0..6 {
            h.push_snapshot(&state_with_ticks(5), format!("S{}", i));
        }
        let pin_id = h.entries()[0].entry_id; // oldest — would normally evict first
        assert!(h.set_pinned(pin_id, true));
        h.restore_index(1);
        let cur_id = h.current_entry_id();

        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur_id, h.next_entry_id())
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        // Load into a history with a cap FAR below the entry count.
        let mut h2 = RecordingHistory::new(2);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        assert!(
            h2.entries()
                .iter()
                .any(|e| e.entry_id == pin_id && e.pinned),
            "pinned entry must survive a lowered cap on load"
        );
        assert_eq!(
            h2.current_entry_id(),
            cur_id,
            "current entry must survive a lowered cap on load"
        );
        let _ = std::fs::remove_dir_all(&dir);
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

        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s2, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        assert_eq!(h2.len(), h.len());
        let b2 = h2.entries().iter().find(|e| e.entry_id == b_id).unwrap();
        assert_eq!(b2.custom_name.as_deref(), Some("B renamed"));
        assert!(b2.pinned);
        assert_eq!(h2.current_entry_id(), cur);
        let max_loaded_id = h2.entries().iter().map(|e| e.entry_id).max().unwrap();
        assert!(
            h2.next_entry_id() > max_loaded_id,
            "next id ({}) must be past EVERY loaded id ({}), incl. the save marker",
            h2.next_entry_id(),
            max_loaded_id
        );
        // snapshot content survives the round-trip
        let a2 = h2.entries().iter().find(|e| e.label == "A").unwrap();
        assert!(a2.can_restore());
        let _ = std::fs::remove_dir_all(&dir);
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

        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(
                &h.to_stored_entries(),
                h.current_entry_id(),
                h.next_entry_id(),
            )
            .unwrap();
        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        let mut h2 = RecordingHistory::new(16);
        h2.apply_loaded(res.entries, res.current_entry_id, res.next_entry_id);

        let e = h2.entries().iter().find(|e| e.entry_id == id).unwrap();
        assert_eq!(e.custom_name.as_deref(), Some("my best run"));
        assert_eq!(
            e.label, "Recorded 0:05",
            "auto label preserved alongside name"
        );

        // Blank rename clears the custom name.
        assert!(h2.rename(id, "   "));
        assert_eq!(
            h2.entries()
                .iter()
                .find(|e| e.entry_id == id)
                .unwrap()
                .custom_name,
            None
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn recovery_checkpoint_becomes_pinned_history_entry() {
        let dir = std::env::temp_dir().join(format!(
            "ssb_recov_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let mut store = RecoveryStore::new_with(dir.clone(), Duration::from_millis(0)).unwrap();
        let state = state_with_ticks(406); // a 4.06s in-progress recording
        let snap = RecordingSnapshot::from_state(&state);
        let session =
            RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 406).unwrap();
        store
            .take_write_job(&snap, &[], &session, true)
            .unwrap()
            .write()
            .unwrap();

        // Mirror startup: load the checkpoint and recover it into history.
        let cp = store.load_pending().unwrap().expect("checkpoint present");
        let mut history = RecordingHistory::new(16);
        assert!(history.push_snapshot_data_with_session(
            cp.snapshot,
            cp.session.label.clone(),
            cp.session.start_tick,
            cp.session.end_tick,
        ));
        let id = history.entries().last().unwrap().entry_id;
        history.set_pinned(id, true);
        history.rename(id, format!("Recovered · {}", cp.session.label));

        let e = &history.entries()[0];
        assert!(e.pinned, "recovered entry is pinned");
        assert!(e.custom_name.as_deref().unwrap().starts_with("Recovered"));
        assert!(e.can_restore(), "recovered snapshot is restorable");

        // DURABILITY: the recovered entry must round-trip through the v2 store —
        // recovery is pointless if it only lives in memory. (Startup persists +
        // flushes before clearing the checkpoint; prove the persisted form here.)
        let v2dir = std::env::temp_dir().join(format!(
            "ssb_recov_v2_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut hstore, _) =
            crate::history_store_v2::HistoryStoreV2::open_eager(v2dir.clone()).unwrap();
        hstore
            .persist(
                &history.to_stored_entries(),
                history.current_entry_id(),
                history.next_entry_id(),
            )
            .unwrap();
        let (_hs, hres) =
            crate::history_store_v2::HistoryStoreV2::open_eager(v2dir.clone()).unwrap();
        let reloaded = &hres.entries[0];
        assert!(reloaded.meta.pinned, "recovered entry persisted as pinned");
        assert!(reloaded
            .meta
            .user_name
            .as_deref()
            .unwrap()
            .starts_with("Recovered"));
        assert!(
            reloaded.snapshot.is_some(),
            "recovered snapshot persisted to disk (eager open returns bytes)"
        );
        let _ = std::fs::remove_dir_all(&v2dir);

        // Clearing the checkpoint means it won't be recovered again.
        store.clear_pending().unwrap();
        assert!(
            store.load_pending().unwrap().is_none(),
            "checkpoint cleared"
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
        let (mut store, _) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
        store
            .persist(&h.to_stored_entries(), cur, h.next_entry_id())
            .unwrap();

        let (_s, res) = HistoryStoreV2::open_eager(dir.clone()).unwrap();
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
        assert_eq!(history.undo_depth(), 0);

        // Redo back to 15
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 15);

        // Redo back to 25
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 25);

        // Can't redo further
        assert!(history.redo().is_none());
        assert_eq!(history.redo_depth(), 0);
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
        assert!(history.undo_depth() > 0);
        let snap = history.undo().unwrap();
        assert_eq!(snap.recorded_count, 10);

        // And redo back to the loaded file
        assert!(history.redo_depth() > 0);
        let snap = history.redo().unwrap();
        assert_eq!(snap.recorded_count, 50);
    }
}
