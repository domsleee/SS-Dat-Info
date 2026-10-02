//! Tests for `crate::relaunch`: the game process dying, being killed or being
//! replaced under a `tas_ui` that stays open, and the crash-recovery
//! checkpoint that has to survive all three.
//!
//! Each name is a scenario, and the interesting ones are the orderings: the
//! PID watcher and the memset are two halves of ONE relaunch and may arrive
//! either way round.

use super::*;

/// A recovery store in a fresh scratch directory holding a `ticks`-long REC
/// checkpoint.
fn checkpoint_store(tag: &str, ticks: u32) -> (std::path::PathBuf, recording::RecoveryStore) {
    let root = std::env::temp_dir().join(format!("ssb_inspect_{}_{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    let mut store =
        recording::RecoveryStore::new_with(root.clone(), std::time::Duration::ZERO).unwrap();
    let state = state_with_recorded_count(ticks);
    let snapshot = recording::RecordingSnapshot::from_state(&state);
    let session =
        recording::RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, ticks).unwrap();
    store
        .take_write_job(&snapshot, &session, true)
        .unwrap()
        .write()
        .unwrap();
    (root, store)
}

fn logged(app: &TasApp, needle: &str) -> bool {
    app.log_lines.lines().iter().any(|l| l.contains(needle))
}

#[test]
fn finalize_with_an_empty_buffer_recovers_the_checkpoint_into_history() {
    let (root, store) = checkpoint_store("finalize_keep", 300);
    let checkpoint = root.join("recovery_checkpoint.tasrec");
    assert!(checkpoint.exists());

    let mut app = test_app();
    app.history.recovery_store = Some(store);
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 300,
    });
    // The game died; the only buffer in sight is a fresh, zeroed mapping.
    let empty = recording::RecordingSnapshot::from_state(&state_with_recorded_count(0));
    app.finalize_recording_session(&empty, 0);

    // The empty snapshot is not the take — the checkpoint is, and it lands
    // in history right away (pinned, ⟲) instead of waiting for a restart
    // that a later take's checkpoint could pre-empt.
    assert_eq!(app.history.list.len(), 1);
    let recovered = &app.history.list.entries()[0];
    assert_eq!((recovered.end_tick, recovered.pinned), (300, true));
    assert_eq!(recovered.custom_name.as_deref(), Some("⟲"));
    // No history writer in the test app = nothing durable yet, so the
    // file stays until a confirmed persist.
    assert!(
        checkpoint.exists(),
        "the checkpoint is cleared only after a durable persist"
    );
    std::fs::remove_dir_all(&root).unwrap();
}

/// A UI reopened while the game still records the checkpoint's take leaves
/// it to that REC's STOP: recovering it now would pin a partial duplicate.
#[test]
fn a_checkpoint_of_the_take_still_recording_is_not_recovered() {
    let (root, store) = checkpoint_store("still_live", 300);
    let mut app = test_app();
    app.history.recovery_store = Some(store);
    let mut live = state_with_recorded_count(450);
    live.mode = TasMode::Rec as u32;
    assert!(app.history.pending_checkpoint_is_live(&live));
    live.input_log[100] = 0x02;
    assert!(
        !app.history.pending_checkpoint_is_live(&live),
        "another take"
    );
    live.input_log[100] = 0x01;
    live.mode = TasMode::Off as u32;
    assert!(
        !app.history.pending_checkpoint_is_live(&live),
        "a stopped game means the take is unsaved: recover it"
    );
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn recovery_drains_queued_checkpoint_writes_before_reading() {
    // Checkpoint A is on disk; checkpoint B (longer) is still queued in the
    // background writer when recovery runs.
    let (root, mut store) = checkpoint_store("recover_flush", 300);
    let app_state = state_with_recorded_count(500);
    let snapshot = recording::RecordingSnapshot::from_state(&app_state);
    let session =
        recording::RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 500).unwrap();
    let job = store.take_write_job(&snapshot, &session, true).unwrap();

    let mut app = test_app();
    app.history.recovery_store = Some(store);
    assert!(app.history.recovery_writer.submit(job));
    assert!(app
        .history
        .recover_pending_checkpoint_matching(None, &mut app.log_lines));
    assert_eq!(
        app.history.list.entries()[0].end_tick,
        500,
        "the newest checkpoint must be the one recovered"
    );
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn game_exit_mid_rec_captures_the_take_from_the_dead_mapping() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    let state = state_with_recorded_count(450);
    {
        let dst = shared.state_mut();
        dst.mode = TasMode::Rec as u32;
        dst.recorded_count = 450;
        dst.input_log[..450].copy_from_slice(&state.input_log[..450]);
        dst.rec_coords[..450].copy_from_slice(&state.rec_coords[..450]);
    }
    app.conn.shared = Some(shared);
    app.session.last_mode = TasMode::Rec as u32;
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 400,
    });

    app.disconnect_from_dead_game();

    assert!(app.conn.shared.is_none());
    assert_eq!(app.session.last_mode, TasMode::Off as u32);
    assert!(app.session.active.is_none());
    assert_eq!(app.history.list.len(), 1);
    assert_eq!(app.history.list.entries()[0].label, "Recorded 0:04.50");
}

#[test]
fn dll_reinitialisation_resets_the_session_view() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().frame_count = 500_000;
    app.conn.shared = Some(shared);
    app.conn.game_pid_cached = Some(1234);
    app.conn.log_read_cursor = 40;
    app.session.last_mode = TasMode::Rec as u32;
    app.check_game_health(); // seeds the baseline
    app.check_game_health(); // unchanged counter: nothing happens
    assert_eq!(app.conn.game_pid_cached, Some(1234));

    // A fresh DLL zeroed the section and started counting again.
    app.conn.shared.as_mut().unwrap().state_mut().frame_count = 7;
    app.check_game_health();

    assert!(logged(&app, "re-initialised its shared memory"));
    assert_eq!(app.conn.game_pid_cached, None);
    assert_eq!(app.conn.log_read_cursor, 0);
    assert_eq!(app.session.last_mode, TasMode::Off as u32);
    assert_eq!(app.conn.cycle_fc, 7);
    // A later advance is still recognised from the new baseline.
    app.conn.shared.as_mut().unwrap().state_mut().frame_count = 8;
    app.check_game_health();
    assert_eq!(app.conn.cycle_fc, 8);
}

/// A mapping that still holds a dead DLL's take: `ticks` of REC, seeded
/// heartbeat, the app mid-session.
fn app_recording_in_dead_mapping(ticks: usize) -> TasApp {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    let state = state_with_recorded_count(ticks as u32);
    {
        let dst = shared.state_mut();
        dst.mode = TasMode::Rec as u32;
        dst.recorded_count = ticks as u32;
        dst.frame_count = 700_000;
        dst.input_log[..ticks].copy_from_slice(&state.input_log[..ticks]);
        dst.rec_coords[..ticks].copy_from_slice(&state.rec_coords[..ticks]);
    }
    app.conn.shared = Some(shared);
    app.session.last_mode = TasMode::Rec as u32;
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 400,
    });
    app
}

#[test]
fn game_process_change_captures_the_take_still_in_the_mapping() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health(); // seed the heartbeat at 700_000
    app.on_game_process_changed(11, 22);
    assert_eq!(app.history.list.len(), 1);
    assert_eq!(app.history.list.entries()[0].label, "Recorded 0:04.50");
    assert!(app.session.active.is_none());
    assert_eq!(app.conn.expect_ring_restart, Some(22));
    assert_eq!(
        app.session.last_mode,
        TasMode::Rec as u32,
        "the dead mapping's frozen REC must not read as a fresh REC start"
    );
    assert!(logged(&app, "captured its 450 ticks"));
}

#[test]
fn expected_ring_restart_does_not_reset_the_session_twice() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health(); // seed the heartbeat at 700_000
    app.on_game_process_changed(11, 22);
    // The user restored a take and started work before the memset.
    app.take.identity.physics = Some("OpenGL/53-bit".into());
    app.conn.log_read_cursor = 40;
    app.on_dll_reinitialised_with(3, Some(22));
    assert_eq!(app.conn.expect_ring_restart, None);
    assert_eq!(app.conn.cycle_fc, 3);
    assert_eq!(
        app.conn.log_read_cursor, 0,
        "the ring restarted with the memset"
    );
    assert_eq!(
        app.take.identity.physics.as_deref(),
        Some("OpenGL/53-bit"),
        "the second signal of one relaunch must not reset again"
    );
    assert_eq!(app.history.list.len(), 1, "the take was captured once");
}

#[test]
fn a_regression_from_a_different_process_is_a_new_relaunch() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health();
    app.on_game_process_changed(11, 22);
    assert_eq!(app.conn.expect_ring_restart, Some(22));
    // A later relaunch (pid 33) whose memset arrives before its PID poll:
    // the stale expectation for 22 must not turn it into a light reset.
    app.take.identity.physics = Some("OpenGL/53-bit".into());
    app.on_dll_reinitialised_with(0, Some(33));
    assert_eq!(app.conn.expect_ring_restart, None);
    assert_eq!(
        app.conn.game_pid_seen,
        Some(33),
        "the PID watcher is brought up to date"
    );
    assert_eq!(
        app.take.identity.physics, None,
        "a full reset for the new relaunch"
    );
}

#[test]
fn relaunch_reset_after_the_memset_recovers_the_checkpoint_not_the_new_buffer() {
    let (root, store) = checkpoint_store("reset_after_memset", 300);
    // The section already belongs to the new game, which has 450 ticks of
    // someone else's take in it; our counter was 700_000.
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health();
    app.history.recovery_store = Some(store);
    app.conn.shared.as_mut().unwrap().state_mut().frame_count = 5;
    app.on_game_process_changed(11, 22);
    assert_eq!(app.history.list.len(), 1);
    let recovered = &app.history.list.entries()[0];
    assert_eq!(
        (recovered.end_tick, recovered.pinned),
        (300, true),
        "the checkpoint, never the new game's buffer"
    );
    assert_eq!(
        app.conn.expect_ring_restart, None,
        "the memset already happened"
    );
    assert_eq!(app.conn.log_read_cursor, 0);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn menu_relaunch_rewinds_the_log_cursor_immediately() {
    // The old game never ticked (main menu): its ring held only the DLL's
    // start-up lines and no counter regression will ever come.
    let mut app = test_app();
    let shared = TasSharedMemoryClient::new_test_mapping();
    app.conn.shared = Some(shared);
    app.check_game_health(); // seeds cycle_fc = 0
    app.conn.log_read_cursor = 15;
    app.on_game_process_changed(11, 22);
    assert_eq!(app.conn.log_read_cursor, 0);
    assert_eq!(app.conn.expect_ring_restart, None);
}

#[test]
fn relaunch_reset_never_releases_a_foreign_interlock() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().cont_suppress_input = 1;
    app.conn.shared = Some(shared);
    app.on_game_process_changed(11, 22);
    assert_eq!(
        app.conn
            .shared
            .as_ref()
            .unwrap()
            .state()
            .cont_suppress_input,
        1,
        "no cycle of ours was running, so the flag is someone else's"
    );
}

#[test]
fn log_cursor_rewinds_when_the_ring_sequence_drops() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    // A fresh DLL wrote 2 lines; our cursor is from the dead one.
    {
        let state = shared.state_mut();
        for (seq, text) in ["first line", "second line"].iter().enumerate() {
            let entry = &mut state.log_ring[seq];
            entry.severity = tas_shared::TasLogSeverity::Info as u32;
            let bytes = text.as_bytes();
            entry.text[..bytes.len()].copy_from_slice(bytes);
            entry.text[bytes.len()] = 0;
            entry.sequence = seq as u32 + 1;
        }
        state.log_write_seq = 2;
    }
    app.conn.shared = Some(shared);
    app.conn.log_read_cursor = 40;
    app.conn.drain_dll_log(&mut app.log_lines);
    assert_eq!(app.conn.log_read_cursor, 2);
    assert!(logged(&app, "second line"));
}

#[test]
fn a_vanished_game_pid_captures_the_take_at_once() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health(); // seed the heartbeat
    app.conn.game_pid_seen = Some(11);
    app.on_game_pid_observed(None);
    assert_eq!(
        app.history.list.len(),
        1,
        "captured from the frozen section"
    );
    assert_eq!(app.history.list.entries()[0].label, "Recorded 0:04.50");
    assert!(app.session.active.is_none());
    assert_eq!(
        app.session.last_mode,
        TasMode::Rec as u32,
        "no phantom session from the frozen REC"
    );
    assert_eq!(
        app.conn.game_pid_seen,
        Some(11),
        "a relaunch is still noticed later"
    );
    // The disconnect that follows finds nothing left to capture.
    app.disconnect_from_dead_game();
    assert_eq!(app.history.list.len(), 1);
}

#[test]
fn a_vanished_pid_with_no_recording_does_nothing() {
    let mut app = test_app();
    app.conn.shared = Some(TasSharedMemoryClient::new_test_mapping());
    app.conn.game_pid_seen = Some(11);
    app.on_game_pid_observed(None);
    assert_eq!(app.history.list.len(), 0);
    assert_eq!(app.conn.game_pid_seen, Some(11));
}

#[test]
fn rejected_finalize_recovers_only_its_own_checkpoint() {
    // An older take's checkpoint (300 ticks) whose clear was refused.
    let (root, store) = checkpoint_store("finalize_owner", 300);
    let checkpoint = root.join("recovery_checkpoint.tasrec");
    let mut app = test_app();
    app.history.recovery_store = Some(store);
    let empty = recording::RecordingSnapshot::from_state(&state_with_recorded_count(0));

    // F9 then F11 before the first tick: a 0-tick session. Not its file.
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 0,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(
        app.history.list.len(),
        0,
        "another take's checkpoint is not resurrected"
    );
    assert!(
        checkpoint.exists(),
        "...and stays on disk for the next launch"
    );

    // A CONT session from 100: different start, not its file either.
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Continue,
        start_tick: 100,
        max_recorded_count: 400,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(app.history.list.len(), 0);

    // The session the checkpoint came from: recovered.
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 350,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(app.history.list.len(), 1);
    assert_eq!(app.history.list.entries()[0].end_tick, 300);
    std::fs::remove_dir_all(&root).unwrap();
}

fn app_with_dead_game(pid: u32) -> TasApp {
    let mut app = test_app();
    app.conn.shared = Some(TasSharedMemoryClient::new_test_mapping());
    app.conn.game_pid_seen = Some(pid);
    app
}

#[test]
fn a_crashed_game_raises_the_banner_with_the_fault_and_the_call() {
    let mut app = app_with_dead_game(11);
    {
        let state = app.conn.shared.as_mut().unwrap().state_mut();
        state.crash_pid = 11;
        state.crash_code = 0xC000_0005;
        state.crash_address = 0x1000_0010;
        state.crash_module[..10].copy_from_slice(b"Kernel.dll");
        state.crash_module_offset = 0x10;
        state.crash_game_call = tas_shared::TAS_GAME_CALL_TIME_CURRENT;
        state
            .crash_seq
            .store(1, std::sync::atomic::Ordering::Release);
    }
    app.on_game_pid_observed(None);
    let banner = app.conn.game_exit_banner.clone().expect("banner");
    assert!(
        banner.contains("access violation") && banner.contains("Kernel::Time::Current"),
        "{banner}"
    );
}

#[test]
fn a_killed_game_raises_the_no_record_banner_once() {
    let mut app = app_with_dead_game(11);
    app.on_game_pid_observed(None);
    let banner = app.conn.game_exit_banner.take().expect("banner");
    assert!(banner.contains("without a crash record"), "{banner}");
    app.on_game_pid_observed(None);
    assert!(
        app.conn.game_exit_banner.is_none(),
        "reported once per process"
    );
}

#[test]
fn a_normally_closed_game_raises_no_banner() {
    let mut app = app_with_dead_game(11);
    app.conn
        .shared
        .as_mut()
        .unwrap()
        .state_mut()
        .game_exit_clean = 1;
    app.on_game_pid_observed(None);
    assert!(app.conn.game_exit_banner.is_none());
    assert!(logged(&app, "closed normally"));
}

#[test]
fn a_crash_during_a_recording_says_the_take_was_saved() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health();
    app.conn.game_pid_seen = Some(11);
    app.on_game_pid_observed(None);
    let banner = app.conn.game_exit_banner.clone().expect("banner");
    assert!(banner.contains("saved to history"), "{banner}");
}

#[test]
fn a_fault_behind_the_games_own_dialog_raises_the_banner_while_it_lives() {
    let mut app = app_with_dead_game(11);
    app.conn.poll_crash_record(11, &mut app.log_lines);
    assert!(app.conn.game_exit_banner.is_none(), "no record yet");
    {
        let state = app.conn.shared.as_mut().unwrap().state_mut();
        state.crash_pid = 11;
        state.crash_code = 0xC000_0005;
        state.crash_game_call = tas_shared::TAS_GAME_CALL_MENU_TRIGGER;
        state
            .crash_seq
            .store(1, std::sync::atomic::Ordering::Release);
    }
    app.conn.poll_crash_record(11, &mut app.log_lines);
    let banner = app.conn.game_exit_banner.take().expect("banner");
    assert!(banner.contains("UI_Menu::Trigger"), "{banner}");
    app.conn.poll_crash_record(11, &mut app.log_lines);
    assert!(app.conn.game_exit_banner.is_none(), "shown once per record");
}
