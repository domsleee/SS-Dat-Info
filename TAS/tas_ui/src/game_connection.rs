//! The game connection: the shared-memory mapping, the heartbeat that says
//! whether the race is ticking, the game process's identity and exit, the
//! DLL's log ring, and the live track / rider / physics read coherently.
//!
//! It observes; the app decides what a relaunch or a dead game means for the
//! take, the session and the cycle (`relaunch.rs`).

use std::time::{Duration, Instant};

use tas_shared::TasSharedMemoryClient;

use crate::host::Host;
use crate::recording;
use crate::ui_log::UiLog;

pub(crate) struct GameConnection {
    pub(crate) shared: Option<TasSharedMemoryClient>,
    pub(crate) connect_error: Option<String>,
    /// Last automatic `try_reconnect` while disconnected.
    last_reconnect_attempt: Instant,
    pub(crate) log_read_cursor: u32,
    /// Cached Supreme.exe PID. Resolved on first poll, invalidated when the
    /// shared-memory connection drops (game closed or restarted).
    pub(crate) game_pid_cached: Option<u32>,
    /// Supreme.exe PID seen by the last 1 Hz health check: a relaunch is
    /// noticed by identity even when the frame counter cannot show it (a game
    /// replaced at the main menu goes 0 → 0).
    pub(crate) game_pid_seen: Option<u32>,
    /// The PID watcher reset the session for a relaunch whose memset has not
    /// been seen yet: the coming frame-counter regression only restarts the
    /// ring, it must not reset (and cancel) whatever started in between.
    pub(crate) expect_ring_restart: Option<u32>,
    /// Why the last game process ended, when it crashed or vanished; shown
    /// until dismissed.
    pub(crate) game_exit_banner: Option<String>,
    /// The game process whose exit has been reported.
    game_exit_reported_pid: Option<u32>,
    /// The DLL's crash_seq already shown for the live game.
    crash_seq_seen: u32,
    last_frame_count: u32,
    stale_frame_ticks: u32,
    last_health_check: Instant,
    // Engine-activity tracker. game_in_game says a race is launched (it stays
    // 1 while paused); frame_count only advances while Supreme::Cycle runs,
    // so a recent advance means the race is really ticking. Sampled every
    // frame.
    pub(crate) cycle_fc: u32,
    // True once cycle_fc holds a real baseline sample, so the first sample
    // after launch or reconnect is not mistaken for an advance.
    cycle_fc_seeded: bool,
    pub(crate) cycle_advance_at: Instant,
    // The last track we were confidently on, used only to name a save: the
    // engine's post-run dialog stops the cycle, so the live level reads
    // unknown just as the user clicks Save. Never used to decide what may be
    // restored; that must be live.
    pub(crate) last_resolved_level: Option<String>,
    // The level_epoch we were last resolved in. "Unresolved" has two causes
    // that need opposite handling: a freeze (menu, pause, post-race dialog;
    // the level is still resident and the epoch unchanged) and a real context
    // change (level_scan saw the level path change or disappear and bumped the
    // epoch). Only the second should hide the history panel.
    last_resolved_epoch: Option<u32>,
}

/// The live game's identity, read in one pass.
pub(crate) struct LiveIdentity {
    pub(crate) physics: Option<String>,
    pub(crate) rider: Option<String>,
    pub(crate) stamps: recording::IdentityStamps,
    pub(crate) level: LiveLevel,
}

pub(crate) enum LiveLevel {
    /// The track (None = an id with no code).
    Resolved(Option<&'static str>),
    /// Unresolved in a new level context: hide what belongs to the old one.
    Changed,
    /// Unresolved but the same context (menu, pause, post-race dialog).
    Frozen,
}

impl GameConnection {
    pub(crate) fn new(now: Instant) -> Self {
        Self {
            shared: None,
            connect_error: None,
            last_reconnect_attempt: now,
            log_read_cursor: 0,
            game_pid_cached: None,
            game_pid_seen: None,
            expect_ring_restart: None,
            game_exit_banner: None,
            game_exit_reported_pid: None,
            crash_seq_seen: 0,
            last_frame_count: 0,
            stale_frame_ticks: 0,
            last_health_check: now,
            cycle_fc: 0,
            cycle_fc_seeded: false,
            // Ancient, not now(): "ticking" must be FALSE until a real
            // frame_count advance is observed.
            cycle_advance_at: now.checked_sub(Duration::from_secs(600)).unwrap_or(now),
            last_resolved_level: None,
            last_resolved_epoch: None,
        }
    }

    pub(crate) fn open(&mut self) {
        match TasSharedMemoryClient::open() {
            Ok(s) => self.shared = Some(s),
            Err(e) => self.connect_error = Some(e),
        }
    }

    /// Open the mapping again. True when it is newly connected.
    pub(crate) fn try_reconnect(&mut self) -> bool {
        match TasSharedMemoryClient::open() {
            Ok(s) => {
                self.shared = Some(s);
                self.connect_error = None;
                // A fresh mapping = a fresh frame_count stream: force the
                // heartbeat to re-baseline instead of treating the first
                // sample as an advance (transport gate false-positive).
                self.cycle_fc_seeded = false;
                true
            }
            Err(e) => {
                self.connect_error = Some(e);
                false
            }
        }
    }

    /// While disconnected, whether the 2 s automatic retry is due (and
    /// starts it).
    pub(crate) fn reconnect_due(&mut self, now: Instant) -> bool {
        if now.saturating_duration_since(self.last_reconnect_attempt) < Duration::from_secs(2) {
            return false;
        }
        self.last_reconnect_attempt = now;
        true
    }

    /// Drop the mapping of a game that is gone.
    pub(crate) fn disconnect(&mut self) {
        self.shared = None;
        self.connect_error =
            Some("Supreme.exe has exited. Inject TAS_Helper.dll after restarting the game.".into());
        self.stale_frame_ticks = 0;
        self.log_read_cursor = 0;
        // A fresh Supreme.exe gets a different PID, so the global-shortcut
        // foreground gate would otherwise stay stale.
        self.game_pid_cached = None;
        self.game_pid_seen = None;
        self.expect_ring_restart = None;
    }

    /// Sample cycle activity (cheap, every frame): game_in_game stays 1 in
    /// the pause menu, so a fresh frame_count advance is the "race is
    /// ticking" signal. Returns the counter when it went backwards: a fresh
    /// DLL zeroed the section.
    pub(crate) fn sample_activity(&mut self, now: Instant) -> Option<u32> {
        let fc = self.shared.as_ref()?.frame_count_volatile();
        if !self.cycle_fc_seeded {
            // Baseline only: a frozen menu has a nonzero frame_count too,
            // so the first sample is not evidence of ticking.
            self.cycle_fc_seeded = true;
            self.cycle_fc = fc;
        } else if fc < self.cycle_fc {
            // The counter only increments (once per cycle), so going
            // backwards means a fresh DLL zeroed the section.
            return Some(fc);
        } else if fc != self.cycle_fc {
            self.cycle_fc = fc;
            self.cycle_advance_at = now;
        }
        None
    }

    /// Whether the 1 Hz health check is due (and starts it).
    pub(crate) fn health_check_due(&mut self, now: Instant) -> bool {
        if now.saturating_duration_since(self.last_health_check) < Duration::from_secs(1) {
            return false;
        }
        self.last_health_check = now;
        true
    }

    /// One 1 Hz sample of the frame counter. True when it has been frozen
    /// for another 5 s: time to re-test whether the game is alive. Re-tested
    /// every 5 stale seconds, not once: the cycle also freezes at the menu
    /// while the game is alive, and the game may exit later while still
    /// stale.
    pub(crate) fn stale_check_due(&mut self) -> bool {
        let Some(shared) = self.shared.as_ref() else {
            return false;
        };
        let current_frame = shared.frame_count_volatile();
        if current_frame == self.last_frame_count {
            self.stale_frame_ticks += 1;
            self.stale_frame_ticks.is_multiple_of(5)
        } else {
            self.last_frame_count = current_frame;
            self.stale_frame_ticks = 0;
            false
        }
    }

    /// The fresh DLL's memset of a relaunch already handled: only the ring
    /// and the heartbeat baseline start over.
    pub(crate) fn rebaseline(&mut self, fc: u32) {
        self.cycle_fc = fc;
        self.last_frame_count = fc;
        self.stale_frame_ticks = 0;
    }

    /// The heartbeat and the cached PID belong to the dead game.
    pub(crate) fn reset_for_new_game(&mut self, fc: u32) {
        self.rebaseline(fc);
        self.game_pid_cached = None;
    }

    /// The menu gate: connected, in a race, and that race's cycle ticked
    /// within the last 400 ms. Arming drives an F5 restart, and in the pause
    /// menu or a dialog the race is still launched (`game_in_game` = 1) but the
    /// cycle is frozen and the game won't take F5, so an arm would leave a
    /// half-armed cycle.
    ///
    /// The button greying, the F9/F10/F12 refusals and `queue_restart_then`
    /// all read it here so they cannot drift apart.
    pub(crate) fn arming_allowed(&self, now: Instant) -> bool {
        self.shared
            .as_ref()
            .is_some_and(|shared| shared.state().game_in_game != 0 && self.ticking_within(now, 400))
    }

    /// Whether the race cycle advanced within the last `ms`.
    pub(crate) fn ticking_within(&self, now: Instant, ms: u64) -> bool {
        now.saturating_duration_since(self.cycle_advance_at) < Duration::from_millis(ms)
    }

    /// The live game's identity: renderer + x87 precision and rider
    /// (character · stance), stamped onto every pushed history entry and
    /// compared on restore/load, and the track.
    ///
    /// One coherent seqlock read: id and path together, or nothing. Asking
    /// "resolved?" and then reading `level_id` separately could hand back "yes"
    /// plus the previous track. Unknown matches nothing — never "assume we are
    /// still where we were".
    pub(crate) fn live_identity(&mut self) -> Option<LiveIdentity> {
        let shared = self.shared.as_ref()?;
        // id and epoch from one seqlock window: a separate epoch read could
        // pair the old track's id with the new epoch across a switch.
        let level = match tas_shared::resolved_level_id_with_epoch(shared.state()) {
            Some((id, epoch)) => {
                let code = tas_shared::level::code_from_id(id);
                if let Some(c) = code {
                    self.last_resolved_level = Some(c.to_string());
                }
                self.last_resolved_epoch = Some(epoch);
                LiveLevel::Resolved(code)
            }
            None => {
                // A freeze (menu, pause, post-race dialog) also reads as
                // unresolved but leaves level_epoch unchanged, and the history
                // panel must stay. Only a moved epoch hides the rows until the
                // new track is identified. A plain epoch read is fine here:
                // this branch stamps nothing, so a torn read costs one frame.
                let epoch_now = shared.state().level_epoch;
                if self.last_resolved_epoch != Some(epoch_now) {
                    LiveLevel::Changed
                } else {
                    LiveLevel::Frozen
                }
            }
        };
        Some(LiveIdentity {
            physics: shared.physics_mode(),
            rider: shared.rider(),
            stamps: recording::IdentityStamps::from_live(shared.state()),
            level,
        })
    }

    /// The live track's code, read coherently. None = unknown.
    pub(crate) fn live_level_code(&self) -> Option<&'static str> {
        self.shared
            .as_ref()
            .and_then(|s| tas_shared::resolved_level_id(s.state()))
            .and_then(tas_shared::level::code_from_id)
    }

    /// The track a recording being saved right now belongs to: the live level if
    /// we have it, else the last one we were confidently on. See
    /// [`recording::save_dialog`] for why this is not read live.
    pub(crate) fn level_for_save(&self) -> Option<&str> {
        crate::level::level_for_save(
            self.shared
                .as_ref()
                .and_then(|s| crate::level::resolved_level_code(s.state())),
            self.last_resolved_level.as_deref(),
        )
    }

    pub(crate) fn live_finish_seq(&self) -> u32 {
        self.shared
            .as_ref()
            .and_then(|s| tas_shared::race_clock::race_finish(s.state()))
            .map_or(0, |f| f.seq)
    }

    /// Supreme.exe's PID for the global-shortcut foreground gate, cached. A
    /// stale cached PID (game restarted without dropping shared memory) just
    /// fails to match, which is harmless.
    pub(crate) fn game_pid_for_shortcuts(&mut self, host: &dyn Host) -> Option<u32> {
        self.game_pid_cached.or_else(|| {
            let resolved = host.game_pid();
            self.game_pid_cached = resolved;
            resolved
        })
    }

    pub(crate) fn drain_dll_log(&mut self, log: &mut UiLog) {
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
            log.push(format!("{} {}", prefix, text));
        }
    }

    /// The game's own error handler can catch a fault and hold the process
    /// open behind its dialog, so a new crash record is shown while the game
    /// is still alive.
    pub(crate) fn poll_crash_record(&mut self, pid: u32, log: &mut UiLog) {
        let Some(shared) = self.shared.as_ref() else {
            return;
        };
        let seq = shared
            .state()
            .crash_seq
            .load(std::sync::atomic::Ordering::Acquire);
        if seq == self.crash_seq_seen {
            return;
        }
        self.crash_seq_seen = seq;
        if let Some(record) = tas_shared::crash::crash_record(shared.state(), pid) {
            let banner =
                format!("The game faulted: {record}. It may be showing its own error dialog.");
            log.push(banner.as_str());
            self.game_exit_banner = Some(banner);
        }
    }

    /// Say how game process `pid` ended, from the crash record its DLL left
    /// in the mapping: a crash or an unexplained exit raises the banner.
    pub(crate) fn report_game_exit(&mut self, pid: u32, take_saved: bool, log: &mut UiLog) {
        if self.game_exit_reported_pid == Some(pid) {
            return;
        }
        self.game_exit_reported_pid = Some(pid);
        let Some(shared) = self.shared.as_ref() else {
            return;
        };
        let saved = if take_saved {
            " The take in progress was saved to history."
        } else {
            ""
        };
        let banner = match tas_shared::crash::game_exit(shared.state(), pid) {
            tas_shared::crash::GameExit::Clean => {
                log.push("The game closed normally");
                return;
            }
            tas_shared::crash::GameExit::Crashed(record) => {
                format!("The game crashed: {record}.{saved}")
            }
            tas_shared::crash::GameExit::Unexplained => format!(
                "The game closed without a crash record: it was killed, or it crashed \
                 where the TAS could not see.{saved}"
            ),
        };
        log.push(banner.as_str());
        self.game_exit_banner = Some(banner);
    }
}
