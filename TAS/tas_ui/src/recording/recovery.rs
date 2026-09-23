//! Crash-recovery checkpoints: the on-disk store, its length-scaled write
//! debounce, and the background writer that keeps checkpoint writes off the UI thread.

use super::{
    completed_session_label, IdentityStamps, RecordingFile, RecordingSessionKind,
    RecordingSnapshot, Segment,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// Crash-recovery checkpoint cadence while recording. Each checkpoint rewrites
// the whole recording (~13 bytes/tick), so a fixed interval makes total write
// volume quadratic in length. The interval therefore grows with the run,
// capped so a crash never loses more than the late interval.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoverySessionContext {
    pub kind: RecordingSessionKind,
    pub start_tick: u32,
    pub end_tick: u32,
    pub label: String,
    /// The track this recording was made on, captured while recording.
    ///
    /// A recovered checkpoint is pushed into history at startup, before the
    /// live level is known, and an untagged entry would show on every track.
    /// The level is known when the checkpoint is written, so it is carried here.
    #[serde(default)]
    pub level: Option<String>,
    /// Physics and rider stamps captured while recording. The checkpoint
    /// writer rebuilds a zeroed shared state, so without these the recovered
    /// take would carry no stamps and mismatch detection would be off for it.
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
    pub(crate) fn new_in_root(root: PathBuf, debounce: Duration) -> Result<Self, String> {
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
}

impl RecoveryWriteJob {
    pub fn write(self) -> Result<(), String> {
        if self.session.end_tick != self.snapshot.recorded_count
            || self.session.start_tick >= self.session.end_tick
        {
            return Err("Recovery session does not match its recording".into());
        }
        let mut state = tas_shared::zeroed_boxed();
        self.snapshot.restore_to(&mut state);
        self.session.apply_stamps(&mut state);

        let identity = IdentityStamps {
            renderer_id: self.session.renderer_id,
            fpu_control_word: self.session.fpu_control_word,
            rider_character: self.session.rider_character,
            rider_stance: self.session.rider_stance,
        };
        let bytes = RecordingFile::encode_with_segments(
            &state,
            &self.segments,
            Some(&identity),
            Some(self.session),
        )?;
        tas_codec::save_atomic(&self.recording_path, &bytes)
    }
}

/// Recovery and history share the same coalescing and flush/error contract.
pub struct RecoveryWriter {
    inner: Result<crate::worker::CoalescingWriter<RecoveryWriteJob>, String>,
}

impl RecoveryWriter {
    pub fn new() -> Self {
        Self {
            inner: crate::worker::CoalescingWriter::spawn("recovery", RecoveryWriteJob::write),
        }
    }
    pub fn submit(&self, job: RecoveryWriteJob) -> bool {
        self.inner.as_ref().is_ok_and(|writer| writer.submit(job))
    }
    pub fn flush(&self) -> Result<(), String> {
        self.inner.as_ref().map_err(Clone::clone)?.flush()
    }
    pub fn take_errors(&self) -> Vec<String> {
        match &self.inner {
            Ok(writer) => writer.take_errors(),
            Err(error) => vec![error.clone()],
        }
    }
}
impl Default for RecoveryWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl RecoveryStore {
    pub fn load_pending(&self) -> Result<Option<RecoveryCheckpoint>, String> {
        if !self.recording_path.exists() {
            if !self.metadata_path.exists() {
                return Ok(None);
            }
            // A sidecar from an older build with no recording beside it:
            // nothing is recoverable from it, and leaving it would fail this
            // check on every launch forever. Report once, then clear it.
            remove_if_exists(&self.metadata_path)?;
            return Err(
                "Recovery metadata from an older build had no recording beside it; removed".into(),
            );
        }
        let data = tas_codec::read_bounded(&self.recording_path)?;
        let mut state = tas_shared::zeroed_boxed();
        let meta = RecordingFile::load_bytes(&mut state, &data)?;
        if state.recorded_count == 0 {
            return Ok(None);
        }
        // Old sidecars cannot be bound to these bytes, even when counts match.
        // Recover the take, but never adopt possibly unrelated track/session stamps.
        let session = match meta.recovery_session {
            Some(session) => {
                if session.end_tick != state.recorded_count
                    || session.start_tick >= session.end_tick
                {
                    return Err("Recovery session does not match its recording".into());
                }
                session
            }
            None => {
                let mut session = RecoverySessionContext::from_ticks(
                    RecordingSessionKind::Rec,
                    0,
                    state.recorded_count,
                )
                .ok_or("Empty recovery recording")?;
                session.label = "Recovered recording (legacy session unknown)".into();
                let identity = IdentityStamps::from_metadata(&meta);
                session.renderer_id = identity.renderer_id;
                session.fpu_control_word = identity.fpu_control_word;
                session.rider_character = identity.rider_character;
                session.rider_stance = identity.rider_stance;
                session
            }
        };
        Ok(Some(RecoveryCheckpoint {
            session,
            snapshot: RecordingSnapshot::from_state(&state),
        }))
    }

    pub fn retry_failed_write(&mut self) {
        self.last_recorded_count = 0;
        self.last_write_at = None;
    }

    pub fn clear_pending(&mut self) -> Result<(), String> {
        remove_if_exists(&self.metadata_path)?;
        remove_if_exists(&self.recording_path)?;
        self.last_recorded_count = 0;
        self.last_write_at = None;
        Ok(())
    }
}

fn remove_if_exists(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(path).map_err(|e| format!("failed to remove {}: {}", path.display(), e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recording::test_support::*;
    use crate::recording::RecordingHistory;
    use tas_shared::TasSharedState;

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
    fn recovery_store_persists_and_loads_checkpoint() {
        let root = unique_temp_root("tas_ui_recovery_store_roundtrip");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = tas_shared::zeroed_boxed();
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

    #[test]
    fn recovery_ignores_unbound_legacy_sidecar_and_recovers_first_recording() {
        let root = unique_temp_root("recovery_legacy_pair");
        let store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.input_log[0] = 8;
        RecordingFile::save_with_segments(&state, &store.recording_path, &[], None).unwrap();
        for sidecar in [None, Some(r#"{"session":{"level":"FE"}}"#)] {
            if let Some(json) = sidecar {
                std::fs::write(&store.metadata_path, json).unwrap();
            }
            let loaded = store.load_pending().unwrap().unwrap();
            assert_eq!(loaded.snapshot.input_log[0], 8);
            assert_eq!(loaded.session.level, None);
            assert!(loaded.session.label.contains("legacy session unknown"));
        }
        let _ = std::fs::remove_dir_all(root);
    }

    #[cfg(windows)]
    #[test]
    fn failed_checkpoint_publish_preserves_old_complete_session() {
        use std::os::windows::fs::OpenOptionsExt;
        let root = unique_temp_root("recovery_failed_publish");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.input_log[0] = 1;
        let old = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 2)
            .unwrap()
            .with_level(Some("FE"));
        persist_checkpoint(&mut store, &state, &[], &old, true).unwrap();
        let lock = std::fs::OpenOptions::new()
            .read(true)
            .share_mode(1)
            .open(&store.recording_path)
            .unwrap();
        state.input_log[0] = 8;
        let new = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 2)
            .unwrap()
            .with_level(Some("AM"));
        let writer = RecoveryWriter::new();
        assert!(writer.submit(
            store
                .take_write_job(&RecordingSnapshot::from_state(&state), &[], &new, true)
                .unwrap()
        ));
        assert!(writer.flush().is_err());
        assert!(!writer.take_errors().is_empty());
        drop(lock);
        let loaded = store.load_pending().unwrap().unwrap();
        assert_eq!(loaded.snapshot.input_log[0], 1);
        assert_eq!(loaded.session.level.as_deref(), Some("FE"));
        store.retry_failed_write();
        persist_checkpoint(&mut store, &state, &[], &new, false).unwrap();
        let loaded = store.load_pending().unwrap().unwrap();
        assert_eq!(loaded.snapshot.input_log[0], 8);
        assert_eq!(loaded.session.level.as_deref(), Some("AM"));
        let _ = std::fs::remove_dir_all(root);
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

    /// The serialized recovery writer's `flush()` is a real drain barrier:
    /// after it returns the checkpoint is on disk, so a following `clear_pending`
    /// can't race an in-flight write that would resurrect the files.
    #[test]
    fn recovery_writer_flush_drains_before_clear() {
        let root = unique_temp_root("tas_ui_recovery_writer_drain");
        let mut store = RecoveryStore::new_in_root(root.clone(), Duration::ZERO).unwrap();
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 3;
        state.input_log[0] = 9;
        let snap = RecordingSnapshot::from_state(&state);
        let session = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 3).unwrap();
        let job = store
            .take_write_job(&snap, &[], &session, true)
            .expect("job");

        let writer = RecoveryWriter::new();
        writer.submit(job);
        writer.flush().unwrap(); // blocks until the write lands

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
        let mut state = tas_shared::zeroed_boxed();
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
        let mut state = tas_shared::zeroed_boxed();
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

        let metadata =
            RecordingFile::read_metadata(&root.join("recovery_checkpoint.tasrec")).unwrap();
        assert_eq!(metadata.recorded_count, 6);
        assert_eq!(metadata.recovery_session.unwrap().end_tick, 6);
        assert!(!root.join("recovery_checkpoint.json").exists());

        let tmp_files = std::fs::read_dir(&root)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().contains(".tmp"))
            .count();
        assert_eq!(tmp_files, 0, "temp files should not remain after replace");

        let _ = std::fs::remove_dir_all(root);
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
        let session = RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 406)
            .unwrap()
            .with_level(Some("FE"))
            .with_stamps(Some((
                0x027F,
                tas_shared::TAS_RENDERER_OPENGL,
                (tas_shared::TAS_CHARACTER_KEITH, 0),
            )));
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
            None,
        ));
        let id = history.entries().last().unwrap().entry_id;
        // Mirror startup: restore every stamp the checkpoint carries.
        history.set_level(id, cp.session.level.clone());
        history.set_rider(id, cp.session.rider_label());
        history.set_physics(
            id,
            tas_shared::physics_mode_label(
                cp.session
                    .renderer_id
                    .unwrap_or(tas_shared::TAS_RENDERER_UNKNOWN),
                cp.session.fpu_control_word.unwrap_or(0),
            ),
        );
        history.set_stamps(
            id,
            IdentityStamps {
                renderer_id: cp.session.renderer_id,
                fpu_control_word: cp.session.fpu_control_word,
                rider_character: cp.session.rider_character,
                rider_stance: cp.session.rider_stance,
            },
        );
        history.set_pinned(id, true);
        history.rename(id, format!("Recovered · {}", cp.session.label));

        let e = &history.entries()[0];
        assert!(e.pinned, "recovered entry is pinned");
        assert!(e.custom_name.as_deref().unwrap().starts_with("Recovered"));
        assert_eq!(e.level.as_deref(), Some("FE"));
        assert_eq!(e.physics.as_deref(), Some("OpenGL/53-bit"));
        assert_eq!(e.rider.as_deref(), Some("Keith · regular"));
        assert_eq!(
            e.stamps,
            IdentityStamps {
                renderer_id: Some(tas_shared::TAS_RENDERER_OPENGL),
                fpu_control_word: Some(0x027F),
                rider_character: Some(tas_shared::TAS_CHARACTER_KEITH),
                rider_stance: Some(0),
            },
            "all three checkpoint stamps survive recovery"
        );

        // The recovered entry must round-trip through the v2 store: startup
        // persists and flushes it before clearing the checkpoint.
        let v2dir = std::env::temp_dir().join(format!(
            "ssb_recov_v2_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0)
        ));
        let (mut hstore, _) =
            crate::history_store::HistoryStore::open_eager(v2dir.clone()).unwrap();
        hstore
            .persist(
                &history.to_stored_entries(),
                history.current_entry_id(),
                history.next_entry_id(),
            )
            .unwrap();
        let (_hs, hres) = crate::history_store::HistoryStore::open_eager(v2dir.clone()).unwrap();
        let reloaded = &hres.entries[0];
        assert!(reloaded.meta.pinned, "recovered entry persisted as pinned");
        assert!(reloaded
            .meta
            .user_name
            .as_deref()
            .unwrap()
            .starts_with("Recovered"));
        assert_eq!(
            reloaded.meta.physics.as_deref(),
            Some("OpenGL/53-bit"),
            "physics stamp persisted"
        );
        assert_eq!(
            reloaded.meta.rider.as_deref(),
            Some("Keith · regular"),
            "rider stamp persisted"
        );
        assert_eq!(
            reloaded.meta.stamps.rider_character,
            Some(tas_shared::TAS_CHARACTER_KEITH),
            "raw stamps persisted"
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
}
