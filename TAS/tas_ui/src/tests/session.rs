//! Tests for `crate::session`: session start/finalize and the checkpoint rule.

use super::*;
use crate::history_runtime::checkpoint_clear_decision;

#[test]
fn checkpoint_survives_missing_or_failed_history() {
    // Durable flush authorizes deletion with no notice.
    assert_eq!(checkpoint_clear_decision(Some(Ok(()))), (true, None));
    // A failed flush keeps the checkpoint and says so.
    let (durable, notice) = checkpoint_clear_decision(Some(Err("disk full".into())));
    assert!(!durable);
    assert!(notice.unwrap().contains("keeping recovery checkpoint"));
    // A missing writer is NOT durability: the checkpoint files are the
    // only copy, so they stay, with an actionable notice.
    let (durable, notice) = checkpoint_clear_decision(None);
    assert!(!durable);
    assert!(notice.unwrap().contains("store unavailable"));
}

#[test]
fn fresh_recording_detaches_editor_but_continue_preserves_it() {
    let mut app = test_app();
    let path = crate::script_watch::ScriptWatch::fresh_path();
    assert_ne!(path, crate::script_watch::ScriptWatch::fresh_path());
    app.editor.script_watch = Some(crate::script_watch::ScriptWatch::new(path, String::new()));
    app.start_recording_session(10, 20);
    assert!(app.editor.script_watch.is_some());
    app.start_recording_session(0, 20);
    assert!(app.editor.script_watch.is_none());
}

#[test]
fn completed_rec_session_pushes_history_entry() {
    let mut app = test_app();
    let state = state_with_recorded_count(2303);
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 2303,
    });

    let snapshot = recording::RecordingSnapshot::from_state(&state);
    app.finalize_recording_session(&snapshot, 2303);

    assert_eq!(app.history.list.len(), 1);
    assert_eq!(app.history.list.entries()[0].label, "Recorded 0:23.03");
}

#[test]
fn completed_cont_session_pushes_history_entry() {
    let mut app = test_app();
    let state = state_with_recorded_count(5303);
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Continue,
        start_tick: 3000,
        max_recorded_count: 5303,
    });

    let snapshot = recording::RecordingSnapshot::from_state(&state);
    app.finalize_recording_session(&snapshot, 5303);

    assert_eq!(app.history.list.len(), 1);
    assert_eq!(
        app.history.list.entries()[0].label,
        "Continued from 0:30.00, total 0:53.03"
    );
}

#[test]
fn continue_session_uses_pending_continue_start_when_shared_resets_to_zero() {
    let mut app = test_app();
    app.session.pending = Some(crate::session::StartContext::Continue { splice: 3415 });
    app.start_recording_session(0, 3415);

    let session = app.session.active.expect("session should start");
    assert_eq!(session.kind, RecordingSessionKind::Continue);
    assert_eq!(session.start_tick, 3415);
    assert_eq!(session.max_recorded_count, 3415);
}

#[test]
fn play_stop_noop_does_not_push_history_entry() {
    let mut app = test_app();
    let state = state_with_recorded_count(100);

    let snapshot = recording::RecordingSnapshot::from_state(&state);
    app.finalize_recording_session(&snapshot, 100);
    assert_eq!(app.history.list.len(), 0);

    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 100,
        max_recorded_count: 100,
    });
    app.finalize_recording_session(&snapshot, 100);
    assert_eq!(app.history.list.len(), 0);
}

/// A cycle can be queued in the frame the DLL ended a REC that the UI has
/// not finalized yet: the next frame still saves that take, and the queued
/// CONT still records as a CONT.
#[test]
fn a_cycle_queued_before_a_rec_is_finalized_keeps_both() {
    let mut app = idle_in_level_app(100);
    app.session.last_mode = TasMode::Rec as u32;
    app.session.active = Some(crate::session::ActiveSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 100,
    });
    app.view.continue_from_frame = 50;
    app.queue_restart_then(TasCommand::ArmContinue);
    assert!(app.transport.is_running());
    app.track_mode_transitions();
    assert_eq!(app.history.list.len(), 1, "the ended take is saved");
    assert!(app.session.active.is_none());
    assert_eq!(
        app.session.pending,
        Some(crate::session::StartContext::Continue { splice: 50 })
    );
}
