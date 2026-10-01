//! History at runtime: the entry list, its background persistence (revisions
//! queued, confirmed and retried) and the crash-recovery checkpoint that must
//! survive until history has the take durably.

use std::time::{Duration, Instant};

use crate::recording::{self, RecordingHistory, RecordingSessionKind};
use crate::ui_log::UiLog;
use crate::{history_store, start_line};

/// The session a recovery checkpoint must belong to (see
/// [`HistoryRuntime::recover_pending_checkpoint_matching`]).
#[derive(Clone, Copy)]
pub(crate) struct CheckpointOwner {
    pub(crate) kind: RecordingSessionKind,
    pub(crate) start_tick: u32,
    pub(crate) max_recorded_count: u32,
}

/// A replacement of the whole recording buffer from history or a file. Each
/// stops a running take first and overwrites only once the DLL acknowledged
/// the STOP.
pub(crate) enum BufferOperation {
    Undo,
    Redo,
    /// The entry with this id; `fallback` is the row it was clicked as,
    /// used when the id no longer exists.
    Restore {
        entry_id: Option<u64>,
        fallback: usize,
    },
    /// A picked `.tasrec`, replayed once loaded.
    Load(std::path::PathBuf),
}

/// Whether the recovery checkpoint may be deleted: only when the recovered
/// take is durably committed to history. A missing history writer is NOT
/// durability — the checkpoint files are then the only copy in existence,
/// and deleting them loses the recovered take permanently.
pub(crate) fn checkpoint_clear_decision(
    flush: Option<Result<(), String>>,
) -> (bool, Option<String>) {
    match flush {
        Some(Ok(())) => (true, None),
        Some(Err(e)) => (
            false,
            Some(format!(
                "[history] persist failed — keeping recovery checkpoint: {}",
                e
            )),
        ),
        None => (
            false,
            Some(
                "[history] store unavailable — keeping recovery checkpoint (no history was written)"
                    .to_string(),
            ),
        ),
    }
}

pub(crate) struct HistoryRuntime {
    pub(crate) list: RecordingHistory,
    pub(crate) writer: Option<history_store::HistoryWriter>,
    /// Highest history revision the worker confirmed durably committed.
    last_persisted_revision: u64,
    /// Highest revision currently queued/in flight. Kept separate from durable
    /// state so a failed background write remains dirty and can be retried.
    last_queued_revision: u64,
    last_failed_revision: u64,
    retry_after: Option<Instant>,
    pub(crate) recovery_store: Option<recording::RecoveryStore>,
    /// Serialized off-thread writer for recovery checkpoints. One writer keeps
    /// writes ordered, so a late write can't land after `clear_pending` and
    /// resurrect a stale checkpoint as a duplicate "Recovered" entry.
    pub(crate) recovery_writer: recording::RecoveryWriter,
}

impl HistoryRuntime {
    pub(crate) fn new(cap: usize) -> Self {
        Self {
            list: RecordingHistory::new(cap),
            writer: None,
            last_persisted_revision: 0,
            last_queued_revision: 0,
            last_failed_revision: 0,
            retry_after: None,
            recovery_store: None,
            recovery_writer: recording::RecoveryWriter::new(),
        }
    }

    /// Open the file-per-entry store in `dir` and the recovery store.
    /// Returns the notices to log.
    pub(crate) fn open_stores(&mut self, dir: &std::path::Path) -> Vec<String> {
        let mut notices = Vec::new();
        self.writer = match history_store::HistoryWriter::open(dir.to_path_buf()) {
            Ok((writer, load)) => {
                // Entries load lazily: a restore reads its blob from here.
                self.list.set_blob_dir(dir.to_path_buf());
                if load.entries.is_empty() {
                    // Preserve the computed id floor even for an empty or
                    // damaged manifest so a new entry cannot reuse a blob id.
                    self.list.adopt_id_floor(load.next_entry_id);
                } else {
                    self.list
                        .apply_loaded(load.entries, load.current_entry_id, load.next_entry_id);
                }
                for warning in load.warnings {
                    notices.push(format!("History: {}", warning));
                }
                notices.push(format!("History store: {}", dir.display()));
                Some(writer)
            }
            Err(e) => {
                notices.push(format!("History store disabled: {}", e));
                None
            }
        };
        // One-time level backfill: tag pre-tagging entries by classifying
        // their snapshot's spawn position (only unambiguous spawns — shared
        // Alpine / FM-FH clusters stay untagged and remain visible on every
        // level). Idempotent: already-tagged entries are skipped, so this is
        // a no-op on every launch after the first.
        let backfilled = self.list.backfill_levels(start_line::level_code_from_spawn);
        if backfilled > 0 {
            notices.push(format!(
                "History: backfilled level tags on {} entries (by spawn position)",
                backfilled
            ));
        }
        self.recovery_store = recording::RecoveryStore::new().ok();
        notices
    }

    /// Make sure the latest history is on disk (at exit).
    pub(crate) fn flush(&self) {
        if let Some(writer) = self.writer.as_ref() {
            let _ = writer.flush();
        }
    }

    pub(crate) fn persist_if_needed(&mut self, now: Instant, log: &mut UiLog) {
        let revision = self.list.revision();
        let Some(writer) = self.writer.as_ref() else {
            return;
        };

        for error in writer.take_errors() {
            log.push(format!("History persistence failed: {error}"));
        }

        self.last_persisted_revision = self.last_persisted_revision.max(writer.durable_revision());
        // Takes the writer has committed no longer need a resident copy.
        for (id, blob) in writer.take_durable_blobs() {
            self.list.mark_durable(id, blob);
        }
        let failed = writer.failed_revision();
        if failed == 0 {
            self.last_failed_revision = 0;
        } else if failed != self.last_failed_revision && failed > self.last_persisted_revision {
            self.last_failed_revision = failed;
            self.last_queued_revision = self.last_persisted_revision;
            self.retry_after = Some(now + Duration::from_secs(1));
            log.push(format!(
                "[history] background persist of revision {} failed; retrying",
                failed
            ));
        }

        if revision == self.last_persisted_revision || revision == self.last_queued_revision {
            return;
        }
        if self.retry_after.is_some_and(|deadline| now < deadline) {
            return;
        }
        // UI-thread cost is only the clone; the worker does serialize + disk.
        let entries = self.list.to_stored_entries();
        let current = self.list.current_entry_id();
        let next = self.list.next_entry_id();
        // Only mark the revision persisted if the job actually reached the
        // worker. If the writer thread is gone, leave the revision dirty so a
        // later frame retries instead of silently dropping the change.
        if !writer.persist(entries, current, next, revision) {
            return;
        }
        // The re-queue is a new attempt, so forget the handled failure. If the
        // worker fails again with the same revision before this thread sees
        // the cleared marker, a stale `last_failed_revision` would suppress
        // the retry.
        self.last_failed_revision = 0;
        self.last_queued_revision = revision;
        self.retry_after = None;
    }

    /// Clear the crash-recovery checkpoint, but only after the history it
    /// represents is durably committed (persist → flush → clear). If the flush
    /// fails the checkpoint is kept, so the next launch recovers it. The
    /// recovery writer is drained first so no in-flight write can recreate the
    /// checkpoint files after they are deleted.
    pub(crate) fn clear_recovery_after_durable_persist(&mut self, now: Instant, log: &mut UiLog) {
        self.persist_if_needed(now, log);
        let flush = self.writer.as_ref().map(|writer| {
            let result = writer.flush();
            if result.is_ok() {
                self.last_persisted_revision =
                    self.last_persisted_revision.max(writer.durable_revision());
            }
            result
        });
        let (durable, notice) = checkpoint_clear_decision(flush);
        if let Some(notice) = notice {
            log.push(notice);
        }
        // Drain in-flight recovery writes BEFORE clearing the files.
        if let Err(error) = self.recovery_writer.flush() {
            log.push(format!("Recovery flush failed: {error}"));
        }
        if durable {
            if let Some(store) = self.recovery_store.as_mut() {
                if let Err(error) = store.clear_pending() {
                    log.push(format!(
                        "Could not clear durable recovery checkpoint: {error}"
                    ));
                }
            }
        }
    }

    /// Queue a checkpoint of the take being recorded, when one is due.
    pub(crate) fn persist_recovery_snapshot_if_needed(
        &mut self,
        snapshot: &recording::RecordingSnapshot,
        session: &recording::RecoverySessionContext,
        force: bool,
        log: &mut UiLog,
    ) {
        for error in self.recovery_writer.take_errors() {
            log.push(format!("Recovery persistence failed: {error}"));
            if let Some(store) = self.recovery_store.as_mut() {
                store.retry_failed_write();
            }
        }
        // Decide on the UI thread (the interval grows with the take, see
        // recording/recovery.rs) but write off it so REC
        // never hitches. STOP does not write a checkpoint: finalize moves the
        // take into durable history and then clears it. Best-effort: a failed
        // write only leaves a slightly staler recovery file.
        let job = match self.recovery_store.as_mut() {
            Some(store) => store.take_write_job(snapshot, session, force),
            None => None,
        };
        if let Some(job) = job {
            if !self.recovery_writer.submit(job) {
                log.push("Recovery writer unavailable; checkpoint was not queued");
                if let Some(store) = self.recovery_store.as_mut() {
                    store.retry_failed_write();
                }
            }
        }
    }

    /// Bring a pending checkpoint back as a pinned history entry, tagged with
    /// the track / rider / physics it was recorded under. The caller clears the
    /// checkpoint only after the history flush is confirmed durable.
    ///
    /// With `owner`, recovers the file only if it belongs to that session
    /// (same kind and start tick, and no longer than the session ever was): a
    /// rejected finalize must not resurrect an older take's checkpoint as a
    /// pinned duplicate of an entry already in history.
    pub(crate) fn recover_pending_checkpoint_matching(
        &mut self,
        owner: Option<CheckpointOwner>,
        log: &mut UiLog,
    ) -> bool {
        if self.recovery_store.is_none() {
            return false;
        }
        // Drain queued checkpoint writes FIRST. The newest one may still be in
        // flight; recovering the older file and then clearing "after durable
        // persist" would let that newer write land only to be deleted.
        if let Err(error) = self.recovery_writer.flush() {
            log.push(format!("Recovery flush failed: {error}"));
        }
        let Some(store) = self.recovery_store.as_ref() else {
            return false;
        };
        let cp = match store.load_pending() {
            Ok(Some(cp)) => cp,
            Ok(None) => return false,
            Err(err) => {
                log.push(format!("Crash recovery check failed: {}", err));
                return false;
            }
        };
        if let Some(owner) = owner {
            let mine = cp.session.kind == owner.kind
                && cp.session.start_tick == owner.start_tick
                && cp.session.end_tick <= owner.max_recorded_count;
            if !mine {
                log.push(format!(
                    "Recovery checkpoint ({}) belongs to another session - kept on disk for the next launch",
                    cp.session.label
                ));
                return false;
            }
        }
        let session_label = cp.session.label.clone();
        let (start, end) = (cp.session.start_tick, cp.session.end_tick);
        let cp_level = cp.session.level.clone();
        let cp_rider = cp.session.rider_label();
        let cp_physics = cp.session.physics_label();
        if !self.list.push_snapshot_data_with_session(
            cp.snapshot,
            session_label.clone(),
            start,
            end,
            Some(cp.session.stamps),
        ) {
            return false;
        }
        if let Some(id) = self.list.entries().last().map(|e| e.entry_id) {
            // Restore the track the checkpoint was RECORDED on. push_* stamps
            // from the live level, which may be None here (startup runs before
            // any level sync); an untagged pinned entry would float at the top
            // of every track's history.
            self.list.set_level(id, cp_level);
            self.list.set_rider(id, cp_rider);
            self.list.set_physics(id, cp_physics);
            self.list.set_pinned(id, true);
            // Mark recovery with a compact ⟲ glyph and let the panel render
            // the duration via the normal parsed format.
            self.list.rename(id, "⟲".to_string());
        }
        log.push(format!(
            "Recovered an unsaved recording ({}) → pinned in history",
            session_label
        ));
        true
    }
}
