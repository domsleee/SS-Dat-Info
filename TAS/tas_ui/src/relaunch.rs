//! Crash recovery and relaunch: the game process can die or be replaced while
//! the UI keeps its mapping.
//!
//! One relaunch produces two signals in either order — the Supreme.exe PID
//! changing (1 Hz sample) and `frame_count` going backwards (the new DLL's
//! memset). `expect_ring_restart` ties them to one reset. `mapping_is_old`
//! says whether the section still holds the dead DLL's bytes, which are ours
//! to capture and release; once zeroed, nothing in it is.

use tas_shared::TasMode;

use crate::recording::{self, RecordingSessionKind};
use crate::win32;
use crate::{ActiveRecordingSession, TasApp};

/// How long `cont_suppress_input` may stay set with the DLL idle in OFF and no
/// arm landing before it counts as abandoned. A live cycle's restart/settle
/// arms within a few seconds; the DLL's own menu-side retirement is 5 s.
pub(crate) const STALE_PROTECTION_GRACE: std::time::Duration = std::time::Duration::from_secs(10);

/// The checkpoint a session left behind: same kind and start tick, and no
/// longer than the session ever was.
#[derive(Clone, Copy)]
pub(crate) struct CheckpointOwner {
    kind: RecordingSessionKind,
    start_tick: u32,
    max_recorded_count: u32,
}

impl From<&ActiveRecordingSession> for CheckpointOwner {
    fn from(session: &ActiveRecordingSession) -> Self {
        Self {
            kind: session.kind,
            start_tick: session.start_tick,
            max_recorded_count: session.max_recorded_count,
        }
    }
}

impl TasApp {
    /// Bring a pending checkpoint back as a pinned history entry, tagged with
    /// the track / rider / physics it was recorded under. The caller clears the
    /// checkpoint only after the history flush is confirmed durable.
    pub(crate) fn recover_pending_checkpoint(&mut self) -> bool {
        self.recover_pending_checkpoint_matching(None)
    }

    /// With `owner`, recovers the file only if it belongs to that session: a
    /// rejected finalize must not resurrect an older take's checkpoint as a
    /// pinned duplicate of an entry already in history.
    pub(crate) fn recover_pending_checkpoint_matching(
        &mut self,
        owner: Option<CheckpointOwner>,
    ) -> bool {
        if self.recovery_store.is_none() {
            return false;
        }
        // Drain queued checkpoint writes FIRST. The newest one may still be in
        // flight; recovering the older file and then clearing "after durable
        // persist" would let that newer write land only to be deleted.
        if let Err(error) = self.recovery_writer.flush() {
            self.push_log(&format!("Recovery flush failed: {error}"));
        }
        let Some(store) = self.recovery_store.as_ref() else {
            return false;
        };
        let cp = match store.load_pending() {
            Ok(Some(cp)) => cp,
            Ok(None) => return false,
            Err(err) => {
                self.push_log(&format!("Crash recovery check failed: {}", err));
                return false;
            }
        };
        if let Some(owner) = owner {
            let mine = cp.session.kind == owner.kind
                && cp.session.start_tick == owner.start_tick
                && cp.session.end_tick <= owner.max_recorded_count;
            if !mine {
                self.push_log(&format!(
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
        let cp_physics = tas_shared::physics_mode_label(
            cp.session
                .renderer_id
                .unwrap_or(tas_shared::TAS_RENDERER_UNKNOWN),
            cp.session.fpu_control_word.unwrap_or(0),
        );
        let cp_stamps = recording::IdentityStamps {
            renderer_id: cp.session.renderer_id,
            fpu_control_word: cp.session.fpu_control_word,
            rider_character: cp.session.rider_character,
            rider_stance: cp.session.rider_stance,
        };
        if !self.history.push_snapshot_data_with_session(
            cp.snapshot,
            session_label.clone(),
            start,
            end,
            Some(cp_stamps),
        ) {
            return false;
        }
        if let Some(id) = self.history.entries().last().map(|e| e.entry_id) {
            // Restore the track the checkpoint was RECORDED on. push_* stamps
            // from the live level, which may be None here (startup runs before
            // any level sync) — without this every recovered entry lands
            // untagged AND pinned, i.e. permanently floating at the top of
            // every track's history ("my favourited FE runs show on FM").
            self.history.set_level(id, cp_level);
            self.history.set_rider(id, cp_rider);
            self.history.set_physics(id, cp_physics);
            self.history.set_pinned(id, true);
            // Mark recovery with a compact ⟲ glyph and let the panel render
            // the duration via the normal parsed format.
            self.history.rename(id, "⟲".to_string());
        }
        self.push_log(&format!(
            "Recovered an unsaved recording ({}) → pinned in history",
            session_label
        ));
        true
    }

    /// A controller that died mid-cycle leaves `cont_suppress_input` set, so
    /// every live key but ESC is swallowed with nothing to explain why. Release
    /// it once it has stayed set for `STALE_PROTECTION_GRACE` with the DLL idle
    /// in OFF, no cycle of ours in flight and no arm landing.
    pub(crate) fn poll_stale_input_protection(&mut self) {
        // While a `tas_test` exists the flag may be its interlock, held through
        // a slow restart: never touch it. Asked lazily — the flag is almost
        // never set, and this is a process-list walk at 1 Hz.
        self.poll_stale_input_protection_with(|| win32::is_process_running("tas_test.exe"));
    }

    pub(crate) fn poll_stale_input_protection_with(
        &mut self,
        external_controller_alive: impl FnOnce() -> bool,
    ) {
        let observed = self.shared.as_ref().and_then(|shared| {
            let state = shared.state();
            (self.cont_controller.is_none()
                && state.mode == TasMode::Off as u32
                && state.cont_suppress_input != 0)
                .then_some(state.arm_generation)
        });
        let Some(arm_generation) = observed else {
            self.stale_protection_since = None;
            return;
        };
        if external_controller_alive() {
            self.stale_protection_since = None;
            return;
        }
        match self.stale_protection_since {
            Some((since, generation)) if generation == arm_generation => {
                if since.elapsed() < STALE_PROTECTION_GRACE {
                    return;
                }
                if let Some(shared) = self.shared.as_mut() {
                    shared.state_mut().cont_suppress_input = 0;
                }
                self.stale_protection_since = None;
                self.push_log(&format!(
                    "Cleared stale input protection (cont_suppress_input) left by a previous \
                     controller: live keys were being swallowed for {} s",
                    STALE_PROTECTION_GRACE.as_secs()
                ));
            }
            _ => self.stale_protection_since = Some((std::time::Instant::now(), arm_generation)),
        }
    }

    /// A fresh `TAS_Helper.dll` reused the section this process still maps, so
    /// `frame_count` restarted from 0. Nothing else notices (the version
    /// matches, the mapping never went away), so treat it as a reconnect: log
    /// cursor, cached game PID, session view and in-flight cycle all belong to
    /// the dead process.
    pub(crate) fn on_dll_reinitialised(&mut self, fc: u32) {
        let current_pid = win32::find_supreme_pid();
        self.on_dll_reinitialised_with(fc, current_pid);
    }

    pub(crate) fn on_dll_reinitialised_with(&mut self, fc: u32, current_pid: Option<u32>) {
        let expected_for = self.expect_ring_restart.take();
        // A snapshot that momentarily finds no process cannot contradict the
        // expectation; only a DIFFERENT process does.
        let same_relaunch =
            expected_for.is_some() && current_pid.is_none_or(|pid| Some(pid) == expected_for);
        if same_relaunch {
            // The PID watcher already reset the session for THIS relaunch;
            // the memset is the second half of the same event. Only the
            // ring and the heartbeat baseline start over — a cycle the user
            // started in between keeps running.
            self.push_log(&format!(
                "TAS_Helper.dll re-initialised its shared memory (frame counter {} → {})",
                self.cycle_fc, fc
            ));
            self.cycle_fc = fc;
            self.last_frame_count = fc;
            self.stale_frame_ticks = 0;
            self.log_read_cursor = 0;
            return;
        }
        // A pending expectation for some OTHER process is stale (that
        // relaunch never showed a regression): this is a new one.
        let why = format!(
            "TAS_Helper.dll re-initialised its shared memory (frame counter {} → {}, \
             pid {:?}, expected {:?}): the game was restarted with tas_ui open — \
             resetting the session view",
            self.cycle_fc, fc, current_pid, expected_for
        );
        // The counter went backwards: the section is the new DLL's already,
        // and its ring starts over.
        self.reset_session_view_for_new_game(&why, fc, false);
        self.log_read_cursor = 0;
        // Bring the PID watcher up to date so it does not fire a second reset
        // a second later, on top of whatever the user started in between.
        self.game_pid_seen = current_pid;
    }

    /// The frame counter cannot show a relaunch that happens before the old
    /// process ever ticked (0 → 0 at the main menu), so the health check also
    /// watches Supreme.exe's identity.
    pub(crate) fn on_game_process_changed(&mut self, old_pid: u32, new_pid: u32) {
        let fc = self
            .shared
            .as_ref()
            .map_or(0, |shared| shared.frame_count_volatile());
        // The dead DLL's section is frozen at the counter we last saw; a
        // counter below it means the new DLL has already zeroed the section.
        // A counter that never moved proves nothing either way (the old
        // game sat at a menu): treat the section as unknown, never as ours.
        let old_never_ticked = self.cycle_fc == 0;
        let mapping_is_old = !old_never_ticked && fc >= self.cycle_fc;
        let stale_mode = self
            .shared
            .as_ref()
            .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
        let why = format!(
            "Supreme.exe was replaced (pid {old_pid} → {new_pid}) with tas_ui still mapped: \
             resetting the session view"
        );
        self.reset_session_view_for_new_game(&why, fc, mapping_is_old);
        if mapping_is_old {
            // The dead DLL's mode word stays frozen (REC, say) until the
            // memset. Baseline on it, or the next poll reads it as a fresh
            // "REC started" and opens a phantom session for a take that was
            // just captured — which the memset then "loses" and recovers
            // from the checkpoint a second time.
            self.last_mode = stale_mode;
        }
        if mapping_is_old && !old_never_ticked {
            // Rewinding now would replay lines already shown; the memset that
            // follows restarts the ring, and only that regression, for this
            // process, is the second half of this relaunch.
            self.expect_ring_restart = Some(new_pid);
        } else {
            // The new ring is already in place, or the old game never ticked
            // and its ring held only start-up lines. A 0 → 0 menu relaunch
            // shows no regression, so nothing else would rewind.
            self.expect_ring_restart = None;
            self.log_read_cursor = 0;
        }
    }

    pub(crate) fn drain_dll_log(&mut self) {
        let Some(ref shared) = self.shared else {
            return;
        };
        use tas_shared::TasLogSeverity;
        // The ring's sequence only ever grows — until a fresh DLL zeroes the
        // section. A sequence below the cursor is that restart: rewind, or
        // the new DLL's first lines are skipped until it catches up.
        if shared.state().log_write_seq < self.log_read_cursor {
            self.log_read_cursor = 0;
            self.expect_ring_restart = None;
        }
        let (entries, new_cursor) = shared.state().read_log_entries(self.log_read_cursor);
        self.log_read_cursor = new_cursor;
        for (_, severity, text) in entries {
            let prefix = match severity {
                TasLogSeverity::Debug => "[DLL:DBG]",
                TasLogSeverity::Info => "[DLL]",
                TasLogSeverity::Warn => "[DLL:WARN]",
                TasLogSeverity::Error => "[DLL:ERR]",
            };
            self.log_lines.push(format!("{} {}", prefix, text));
        }
    }

    /// One relaunch, one reset — whichever signal saw it first.
    fn reset_session_view_for_new_game(&mut self, why: &str, fc: u32, mapping_is_old: bool) {
        self.push_log(why);
        if self.active_recording_session.is_some() {
            if mapping_is_old {
                // Capture before resetting anything: the dead DLL's buffer is
                // the complete take, the checkpoint lags by a debounce.
                self.capture_take_from_mapping(
                    "The game process that owned the recording in progress is gone",
                );
            } else {
                // The buffer belongs to the new game now, so the old take's
                // checkpoint is its only copy.
                let owner = self
                    .active_recording_session
                    .take()
                    .map(|session| CheckpointOwner::from(&session));
                self.push_log(
                    "The recording in progress was lost with the old game process; \
                     recovering its last checkpoint",
                );
                if self.recover_pending_checkpoint_matching(owner) {
                    self.clear_recovery_after_durable_persist();
                }
            }
        }
        self.cycle_fc = fc;
        self.last_frame_count = fc;
        self.stale_frame_ticks = 0;
        self.stale_protection_since = None;
        self.game_pid_cached = None;
        self.clear_cont_catchup();
        // Only a cycle of ours may release the interlock, and only while the
        // section is still the one it armed: a controller in another process
        // may already own the new game's restart.
        let owned_cycle = self.cont_controller.is_some();
        self.pending_session_kind = None;
        self.pending_continue_start_tick = None;
        self.cont_cycle_deadline = None;
        self.cont_controller = None;
        if owned_cycle && mapping_is_old {
            self.set_cont_suppress_input(false);
        }
        self.detach_input_editor();
        self.loaded_physics = None;
        self.loaded_rider = None;
        self.loaded_identity = None;
        self.last_mode = TasMode::Off as u32;
    }

    /// Finalize the take still sitting in the section into history, for when
    /// the process that owned it is gone but nothing has zeroed it yet.
    ///
    /// Waiting for a REC→OFF transition instead would lose it: that transition
    /// can only arrive from a fresh, empty mapping, which pushes an empty
    /// snapshot and then deletes the checkpoint. No-ops without a session, so
    /// callers on the relaunch path need no guard.
    fn capture_take_from_mapping(&mut self, why: &str) {
        if self.active_recording_session.is_none() {
            return;
        }
        let capture = self.shared.as_ref().map(|shared| {
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            let recorded = snapshot.recorded_count;
            (snapshot, recorded)
        });
        if let Some((snapshot, recorded)) = capture {
            self.push_log(&format!(
                "{why} — captured its {recorded} ticks from shared memory"
            ));
            self.finalize_recording_session(&snapshot, recorded);
        }
    }

    /// The 1 Hz identity sample. A new PID is a relaunch; no PID during a
    /// recording is the moment to capture it, because nothing can zero the
    /// section until a new game injects and waiting for the stale-frame
    /// disconnect (up to 5 s) would settle for the lagging checkpoint.
    pub(crate) fn on_game_pid_observed(&mut self, pid: Option<u32>) {
        match pid {
            Some(pid) => {
                if let Some(seen) = self.game_pid_seen {
                    if seen != pid {
                        self.on_game_process_changed(seen, pid);
                    }
                }
                self.game_pid_seen = Some(pid);
            }
            None => {
                if self.game_pid_seen.is_some() && self.active_recording_session.is_some() {
                    self.capture_take_from_mapping("Game exited during a recording");
                    // The frozen section still reads REC; the memset or the
                    // disconnect must not turn that into a phantom session.
                    self.last_mode = self
                        .shared
                        .as_ref()
                        .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
                }
            }
        }
    }

    pub(crate) fn disconnect_from_dead_game(&mut self) {
        self.push_log("Game process not found — disconnecting shared memory");
        self.capture_take_from_mapping("Game exited during a recording");
        self.shared = None;
        self.detach_input_editor();
        self.connect_error =
            Some("Supreme.exe has exited. Inject TAS_Helper.dll after restarting the game.".into());
        self.stale_frame_ticks = 0;
        self.stale_protection_since = None;
        self.log_read_cursor = 0;
        // A fresh Supreme.exe gets a different PID, so the global-shortcut
        // foreground gate would otherwise stay stale.
        self.game_pid_cached = None;
        self.game_pid_seen = None;
        self.expect_ring_restart = None;
        self.clear_cont_catchup();
        self.reset_continue_runtime_state();
        self.last_mode = TasMode::Off as u32;
    }
}
