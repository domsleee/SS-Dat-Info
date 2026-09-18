//! Tests for `crate::relaunch`: the game process dying, being killed or being
//! replaced under a `tas_ui` that stays open, and the crash-recovery
//! checkpoint that has to survive all three.
//!
//! Each name is a scenario, and the interesting ones are the orderings: the
//! PID watcher and the memset are two halves of ONE relaunch and may arrive
//! either way round.

use super::*;
use crate::relaunch::STALE_PROTECTION_GRACE;

fn scratch_recovery_root(tag: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("ssb_inspect_{}_{}", tag, std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    root
}

fn write_checkpoint(store: &mut recording::RecoveryStore, ticks: u32) {
    let state = state_with_recorded_count(ticks);
    let snapshot = recording::RecordingSnapshot::from_state(&state);
    let session =
        recording::RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, ticks).unwrap();
    store
        .take_write_job(&snapshot, &[], &session, true)
        .unwrap()
        .write()
        .unwrap();
}

#[test]
fn finalize_with_an_empty_buffer_recovers_the_checkpoint_into_history() {
    let root = scratch_recovery_root("finalize_keep");
    let mut store =
        recording::RecoveryStore::new_in_root(root.clone(), std::time::Duration::ZERO).unwrap();
    write_checkpoint(&mut store, 300);
    let checkpoint = root.join("recovery_checkpoint.tasrec");
    assert!(checkpoint.exists());

    let mut app = test_app();
    app.recovery_store = Some(store);
    app.active_recording_session = Some(ActiveRecordingSession {
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
    assert_eq!(app.history.len(), 1);
    let recovered = &app.history.entries()[0];
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

#[test]
fn recovery_drains_queued_checkpoint_writes_before_reading() {
    let root = scratch_recovery_root("recover_flush");
    let mut store =
        recording::RecoveryStore::new_in_root(root.clone(), std::time::Duration::ZERO).unwrap();
    // Checkpoint A is on disk; checkpoint B (longer) is still queued in the
    // background writer when recovery runs.
    write_checkpoint(&mut store, 300);
    let app_state = state_with_recorded_count(500);
    let snapshot = recording::RecordingSnapshot::from_state(&app_state);
    let session =
        recording::RecoverySessionContext::from_ticks(RecordingSessionKind::Rec, 0, 500).unwrap();
    let job = store
        .take_write_job(&snapshot, &[], &session, true)
        .unwrap();

    let mut app = test_app();
    app.recovery_store = Some(store);
    assert!(app.recovery_writer.submit(job));
    assert!(app.recover_pending_checkpoint());
    assert_eq!(
        app.history.entries()[0].end_tick,
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
    app.shared = Some(shared);
    app.last_mode = TasMode::Rec as u32;
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 400,
    });

    app.disconnect_from_dead_game();

    assert!(app.shared.is_none());
    assert_eq!(app.last_mode, TasMode::Off as u32);
    assert!(app.active_recording_session.is_none());
    assert_eq!(app.history.len(), 1);
    assert_eq!(app.history.entries()[0].label, "Recorded 0:04.50");
}

#[test]
fn dll_reinitialisation_resets_the_session_view() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().frame_count = 500_000;
    app.shared = Some(shared);
    app.game_pid_cached = Some(1234);
    app.log_read_cursor = 40;
    app.last_mode = TasMode::Rec as u32;
    app.check_game_health(); // seeds the baseline
    app.check_game_health(); // unchanged counter: nothing happens
    assert_eq!(app.game_pid_cached, Some(1234));

    // A fresh DLL zeroed the section and started counting again.
    app.shared.as_mut().unwrap().state_mut().frame_count = 7;
    app.check_game_health();

    assert!(app
        .log_lines
        .lines()
        .iter()
        .any(|l| l.contains("re-initialised its shared memory")));
    assert_eq!(app.game_pid_cached, None);
    assert_eq!(app.log_read_cursor, 0);
    assert_eq!(app.last_mode, TasMode::Off as u32);
    assert_eq!(app.cycle_fc, 7);
    // A later advance is still recognised from the new baseline.
    app.shared.as_mut().unwrap().state_mut().frame_count = 8;
    app.check_game_health();
    assert_eq!(app.cycle_fc, 8);
}

#[test]
fn stale_input_protection_is_released_only_after_the_grace_period() {
    let suppressed = |app: &TasApp| app.shared.as_ref().unwrap().state().cont_suppress_input;
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().cont_suppress_input = 1;
    shared.state_mut().mode = TasMode::Rec as u32;
    app.shared = Some(shared);
    app.poll_stale_input_protection();
    assert_eq!(suppressed(&app), 1, "a running REC keeps its protection");
    assert!(app.stale_protection_since.is_none());

    // Idle in OFF with the flag set: the countdown starts, nothing clears yet
    // — this is exactly what a live controller's restart/settle looks like.
    app.shared.as_mut().unwrap().state_mut().mode = TasMode::Off as u32;
    app.poll_stale_input_protection();
    app.poll_stale_input_protection();
    assert_eq!(
        suppressed(&app),
        1,
        "a fresh OFF-mode hold is not stale yet"
    );
    let (since, generation) = app.stale_protection_since.unwrap();

    // An arm landing (generation bump) restarts the countdown.
    app.shared.as_mut().unwrap().state_mut().arm_generation = generation + 1;
    app.stale_protection_since = Some((since - STALE_PROTECTION_GRACE * 2, generation));
    app.poll_stale_input_protection();
    assert_eq!(suppressed(&app), 1, "a new arm means a live controller");
    assert_eq!(app.stale_protection_since.unwrap().1, generation + 1);

    // Held past the grace period with no arm: abandoned — release it.
    app.stale_protection_since = Some((
        std::time::Instant::now() - STALE_PROTECTION_GRACE * 2,
        generation + 1,
    ));
    app.poll_stale_input_protection();
    assert_eq!(suppressed(&app), 0);
    assert!(app.stale_protection_since.is_none());
    assert!(app
        .log_lines
        .lines()
        .iter()
        .any(|l| l.contains("Cleared stale input protection")));
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
    app.shared = Some(shared);
    app.last_mode = TasMode::Rec as u32;
    app.active_recording_session = Some(ActiveRecordingSession {
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
    assert_eq!(app.history.len(), 1);
    assert_eq!(app.history.entries()[0].label, "Recorded 0:04.50");
    assert!(app.active_recording_session.is_none());
    assert_eq!(app.expect_ring_restart, Some(22));
    assert_eq!(
        app.last_mode,
        TasMode::Rec as u32,
        "the dead mapping's frozen REC must not read as a fresh REC start"
    );
    assert!(app
        .log_lines
        .lines()
        .iter()
        .any(|l| l.contains("captured its 450 ticks")));
}

#[test]
fn expected_ring_restart_does_not_reset_the_session_twice() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health(); // seed the heartbeat at 700_000
    app.on_game_process_changed(11, 22);
    // The user restored a take and started work before the memset.
    app.loaded_physics = Some("OpenGL/53-bit".into());
    app.log_read_cursor = 40;
    app.on_dll_reinitialised_with(3, Some(22));
    assert_eq!(app.expect_ring_restart, None);
    assert_eq!(app.cycle_fc, 3);
    assert_eq!(app.log_read_cursor, 0, "the ring restarted with the memset");
    assert_eq!(
        app.loaded_physics.as_deref(),
        Some("OpenGL/53-bit"),
        "the second signal of one relaunch must not reset again"
    );
    assert_eq!(app.history.len(), 1, "the take was captured once");
}

#[test]
fn a_regression_from_a_different_process_is_a_new_relaunch() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health();
    app.on_game_process_changed(11, 22);
    assert_eq!(app.expect_ring_restart, Some(22));
    // A later relaunch (pid 33) whose memset arrives before its PID poll:
    // the stale expectation for 22 must not turn it into a light reset.
    app.loaded_physics = Some("OpenGL/53-bit".into());
    app.on_dll_reinitialised_with(0, Some(33));
    assert_eq!(app.expect_ring_restart, None);
    assert_eq!(
        app.game_pid_seen,
        Some(33),
        "the PID watcher is brought up to date"
    );
    assert_eq!(
        app.loaded_physics, None,
        "a full reset for the new relaunch"
    );
}

#[test]
fn relaunch_reset_after_the_memset_recovers_the_checkpoint_not_the_new_buffer() {
    let root = scratch_recovery_root("reset_after_memset");
    let mut store =
        recording::RecoveryStore::new_in_root(root.clone(), std::time::Duration::ZERO).unwrap();
    write_checkpoint(&mut store, 300);
    // The section already belongs to the new game, which has 450 ticks of
    // someone else's take in it; our counter was 700_000.
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health();
    app.recovery_store = Some(store);
    app.shared.as_mut().unwrap().state_mut().frame_count = 5;
    app.on_game_process_changed(11, 22);
    assert_eq!(app.history.len(), 1);
    let recovered = &app.history.entries()[0];
    assert_eq!(
        (recovered.end_tick, recovered.pinned),
        (300, true),
        "the checkpoint, never the new game's buffer"
    );
    assert_eq!(app.expect_ring_restart, None, "the memset already happened");
    assert_eq!(app.log_read_cursor, 0);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn menu_relaunch_rewinds_the_log_cursor_immediately() {
    // The old game never ticked (main menu): its ring held only the DLL's
    // start-up lines and no counter regression will ever come.
    let mut app = test_app();
    let shared = TasSharedMemoryClient::new_test_mapping();
    app.shared = Some(shared);
    app.check_game_health(); // seeds cycle_fc = 0
    app.log_read_cursor = 15;
    app.on_game_process_changed(11, 22);
    assert_eq!(app.log_read_cursor, 0);
    assert_eq!(app.expect_ring_restart, None);
}

#[test]
fn relaunch_reset_never_releases_a_foreign_interlock() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().cont_suppress_input = 1;
    app.shared = Some(shared);
    app.on_game_process_changed(11, 22);
    assert_eq!(
        app.shared.as_ref().unwrap().state().cont_suppress_input,
        1,
        "no cycle of ours was running, so the flag is someone else's"
    );
}

#[test]
fn a_live_harness_process_blocks_the_stale_release() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().cont_suppress_input = 1;
    app.shared = Some(shared);
    app.stale_protection_since = Some((std::time::Instant::now() - STALE_PROTECTION_GRACE * 2, 0));
    app.poll_stale_input_protection_with(|| true);
    assert_eq!(app.shared.as_ref().unwrap().state().cont_suppress_input, 1);
    assert!(
        app.stale_protection_since.is_none(),
        "no countdown while it lives"
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
    app.shared = Some(shared);
    app.log_read_cursor = 40;
    app.drain_dll_log();
    assert_eq!(app.log_read_cursor, 2);
    assert!(app
        .log_lines
        .lines()
        .iter()
        .any(|l| l.contains("second line")));
}

#[test]
fn a_vanished_game_pid_captures_the_take_at_once() {
    let mut app = app_recording_in_dead_mapping(450);
    app.check_game_health(); // seed the heartbeat
    app.game_pid_seen = Some(11);
    app.on_game_pid_observed(None);
    assert_eq!(app.history.len(), 1, "captured from the frozen section");
    assert_eq!(app.history.entries()[0].label, "Recorded 0:04.50");
    assert!(app.active_recording_session.is_none());
    assert_eq!(
        app.last_mode,
        TasMode::Rec as u32,
        "no phantom session from the frozen REC"
    );
    assert_eq!(
        app.game_pid_seen,
        Some(11),
        "a relaunch is still noticed later"
    );
    // The disconnect that follows finds nothing left to capture.
    app.disconnect_from_dead_game();
    assert_eq!(app.history.len(), 1);
}

#[test]
fn a_vanished_pid_with_no_recording_does_nothing() {
    let mut app = test_app();
    app.shared = Some(TasSharedMemoryClient::new_test_mapping());
    app.game_pid_seen = Some(11);
    app.on_game_pid_observed(None);
    assert_eq!(app.history.len(), 0);
    assert_eq!(app.game_pid_seen, Some(11));
}

#[test]
fn rejected_finalize_recovers_only_its_own_checkpoint() {
    let root = scratch_recovery_root("finalize_owner");
    let mut store =
        recording::RecoveryStore::new_in_root(root.clone(), std::time::Duration::ZERO).unwrap();
    // An older take's checkpoint (300 ticks) whose clear was refused.
    write_checkpoint(&mut store, 300);
    let checkpoint = root.join("recovery_checkpoint.tasrec");
    let mut app = test_app();
    app.recovery_store = Some(store);
    let empty = recording::RecordingSnapshot::from_state(&state_with_recorded_count(0));

    // F9 then F11 before the first tick: a 0-tick session. Not its file.
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 0,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(
        app.history.len(),
        0,
        "another take's checkpoint is not resurrected"
    );
    assert!(
        checkpoint.exists(),
        "...and stays on disk for the next launch"
    );

    // A CONT session from 100: different start, not its file either.
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Continue,
        start_tick: 100,
        max_recorded_count: 400,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(app.history.len(), 0);

    // The session the checkpoint came from: recovered.
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 350,
    });
    app.finalize_recording_session(&empty, 0);
    assert_eq!(app.history.len(), 1);
    assert_eq!(app.history.entries()[0].end_tick, 300);
    std::fs::remove_dir_all(&root).unwrap();
}
