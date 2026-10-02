//! Crash recovery and relaunch: the game process can die or be replaced while
//! the UI keeps its mapping.
//!
//! One relaunch produces two signals in either order — the Supreme.exe PID
//! changing (1 Hz sample) and `frame_count` going backwards (the new DLL's
//! memset). `expect_ring_restart` ties them to one reset. `mapping_is_old`
//! says whether the section still holds the dead DLL's bytes, which are ours
//! to capture and release; once zeroed, nothing in it is.

use tas_shared::TasMode;

use crate::recording;
use crate::TasApp;

impl TasApp {
    /// A fresh `TAS_Helper.dll` reused the section this process still maps, so
    /// `frame_count` restarted from 0. Nothing else notices (the version
    /// matches, the mapping never went away), so treat it as a reconnect: log
    /// cursor, cached game PID, session view and in-flight cycle all belong to
    /// the dead process.
    pub(crate) fn on_dll_reinitialised(&mut self, fc: u32) {
        let current_pid = self.host.game_pid();
        self.on_dll_reinitialised_with(fc, current_pid);
    }

    pub(crate) fn on_dll_reinitialised_with(&mut self, fc: u32, current_pid: Option<u32>) {
        let expected_for = self.conn.expect_ring_restart.take();
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
                self.conn.cycle_fc, fc
            ));
            self.conn.rebaseline(fc);
            self.conn.log_read_cursor = 0;
            return;
        }
        // A pending expectation for some OTHER process is stale (that
        // relaunch never showed a regression): this is a new one.
        let why = format!(
            "TAS_Helper.dll re-initialised its shared memory (frame counter {} → {}, \
             pid {:?}, expected {:?}): the game was restarted with tas_ui open — \
             resetting the session view",
            self.conn.cycle_fc, fc, current_pid, expected_for
        );
        // The counter went backwards: the section is the new DLL's already,
        // and its ring starts over.
        self.reset_session_view_for_new_game(&why, fc, false);
        self.conn.log_read_cursor = 0;
        // Bring the PID watcher up to date so it does not fire a second reset
        // a second later, on top of whatever the user started in between.
        self.conn.game_pid_seen = current_pid;
    }

    /// The frame counter cannot show a relaunch that happens before the old
    /// process ever ticked (0 → 0 at the main menu), so the health check also
    /// watches Supreme.exe's identity.
    pub(crate) fn on_game_process_changed(&mut self, old_pid: u32, new_pid: u32) {
        let fc = self
            .conn
            .shared
            .as_ref()
            .map_or(0, |shared| shared.frame_count_volatile());
        // The dead DLL's section is frozen at the counter we last saw; a
        // counter below it means the new DLL has already zeroed the section.
        // A counter that never moved proves nothing either way (the old
        // game sat at a menu): treat the section as unknown, never as ours.
        let old_never_ticked = self.conn.cycle_fc == 0;
        let mapping_is_old = !old_never_ticked && fc >= self.conn.cycle_fc;
        let stale_mode = self
            .conn
            .shared
            .as_ref()
            .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
        if mapping_is_old {
            // Relaunched between two samples: the dead DLL's record is still here.
            self.conn
                .report_game_exit(old_pid, false, &mut self.log_lines);
        }
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
            self.session.last_mode = stale_mode;
        }
        if mapping_is_old && !old_never_ticked {
            // Rewinding now would replay lines already shown; the memset that
            // follows restarts the ring, and only that regression, for this
            // process, is the second half of this relaunch.
            self.conn.expect_ring_restart = Some(new_pid);
        } else {
            // The new ring is already in place, or the old game never ticked
            // and its ring held only start-up lines. A 0 → 0 menu relaunch
            // shows no regression, so nothing else would rewind.
            self.conn.expect_ring_restart = None;
            self.conn.log_read_cursor = 0;
        }
    }

    /// One relaunch, one reset — whichever signal saw it first.
    fn reset_session_view_for_new_game(&mut self, why: &str, fc: u32, mapping_is_old: bool) {
        self.push_log(why);
        if self.session.active.is_some() {
            if mapping_is_old {
                // Capture before resetting anything: the dead DLL's buffer is
                // the complete take, the checkpoint lags by a debounce.
                self.capture_take_from_mapping(
                    "The game process that owned the recording in progress is gone",
                );
            } else {
                // The buffer belongs to the new game now, so the old take's
                // checkpoint is its only copy.
                let owner = self.session.active.take();
                self.push_log(
                    "The recording in progress was lost with the old game process; \
                     recovering its last checkpoint",
                );
                if self.history.recover_pending_checkpoint_matching(
                    owner.map(|o| o.owner()),
                    &mut self.log_lines,
                ) {
                    self.history
                        .clear_recovery_after_durable_persist(self.host.now(), &mut self.log_lines);
                }
            }
        }
        self.conn.reset_for_new_game(fc);
        self.transport.clear_catchup();
        self.session.pending = None;
        self.transport
            .forget_for_new_game(self.conn.shared.as_mut(), mapping_is_old);
        self.editor.detach();
        self.take.replace_identity(Default::default());
        self.session.last_mode = TasMode::Off as u32;
    }

    /// Finalize the take still sitting in the section into history, for when
    /// the process that owned it is gone but nothing has zeroed it yet.
    ///
    /// Waiting for a REC→OFF transition instead would lose it: that transition
    /// can only arrive from a fresh, empty mapping, which pushes an empty
    /// snapshot and then deletes the checkpoint. No-ops without a session, so
    /// callers on the relaunch path need no guard.
    fn capture_take_from_mapping(&mut self, why: &str) -> bool {
        if self.session.active.is_none() {
            return false;
        }
        let capture = self.conn.shared.as_ref().map(|shared| {
            let snapshot = recording::RecordingSnapshot::from_state(shared.state());
            let recorded = snapshot.recorded_count;
            (snapshot, recorded)
        });
        if let Some((snapshot, recorded)) = capture {
            self.push_log(&format!(
                "{why} — captured its {recorded} ticks from shared memory"
            ));
            self.finalize_recording_session(&snapshot, recorded);
            return true;
        }
        false
    }

    /// The 1 Hz identity sample. A new PID is a relaunch; no PID during a
    /// recording is the moment to capture it, because nothing can zero the
    /// section until a new game injects and waiting for the stale-frame
    /// disconnect (up to 5 s) would settle for the lagging checkpoint.
    pub(crate) fn on_game_pid_observed(&mut self, pid: Option<u32>) {
        match pid {
            Some(pid) => {
                if let Some(seen) = self.conn.game_pid_seen {
                    if seen != pid {
                        self.on_game_process_changed(seen, pid);
                    }
                }
                self.conn.game_pid_seen = Some(pid);
            }
            None => {
                let Some(dead) = self.conn.game_pid_seen else {
                    return;
                };
                let mut take_saved = false;
                if self.session.active.is_some() {
                    take_saved = self.capture_take_from_mapping("Game exited during a recording");
                    // The frozen section still reads REC; the memset or the
                    // disconnect must not turn that into a phantom session.
                    self.session.last_mode = self
                        .conn
                        .shared
                        .as_ref()
                        .map_or(TasMode::Off as u32, |shared| shared.mode_volatile());
                }
                self.conn
                    .report_game_exit(dead, take_saved, &mut self.log_lines);
            }
        }
    }

    pub(crate) fn disconnect_from_dead_game(&mut self) {
        self.push_log("Game process not found — disconnecting shared memory");
        self.capture_take_from_mapping("Game exited during a recording");
        self.conn.disconnect();
        self.editor.detach();
        self.transport.clear_catchup();
        self.reset_continue_runtime_state();
        self.session.last_mode = TasMode::Off as u32;
    }
}
