//! Tests for `crate::session`: session start/finalize and the checkpoint rule.

use super::*;
use crate::session::checkpoint_clear_decision;

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
    app.script_watch = Some(crate::script_watch::ScriptWatch::new(path, String::new()));
    app.start_recording_session(10, 20);
    assert!(app.script_watch.is_some());
    app.start_recording_session(0, 20);
    assert!(app.script_watch.is_none());
}

#[test]
fn completed_rec_session_pushes_history_entry() {
    let mut app = test_app();
    let state = state_with_recorded_count(2303);
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 0,
        max_recorded_count: 2303,
    });

    let snapshot = recording::RecordingSnapshot::from_state(&state);
    app.finalize_recording_session(&snapshot, 2303);

    assert_eq!(app.history.len(), 1);
    assert_eq!(app.history.entries()[0].label, "Recorded 0:23.03");
}

#[test]
fn completed_cont_session_pushes_history_entry() {
    let mut app = test_app();
    let state = state_with_recorded_count(5303);
    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Continue,
        start_tick: 3000,
        max_recorded_count: 5303,
    });

    let snapshot = recording::RecordingSnapshot::from_state(&state);
    app.finalize_recording_session(&snapshot, 5303);

    assert_eq!(app.history.len(), 1);
    assert_eq!(
        app.history.entries()[0].label,
        "Continued from 0:30.00, total 0:53.03"
    );
}

#[test]
fn continue_session_uses_pending_continue_start_when_shared_resets_to_zero() {
    let mut app = test_app();
    app.pending_session_kind = Some(RecordingSessionKind::Continue);
    app.pending_continue_start_tick = Some(3415);
    app.start_recording_session(0, 3415);

    let session = app.active_recording_session.expect("session should start");
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
    assert_eq!(app.history.len(), 0);

    app.active_recording_session = Some(ActiveRecordingSession {
        kind: RecordingSessionKind::Rec,
        start_tick: 100,
        max_recorded_count: 100,
    });
    app.finalize_recording_session(&snapshot, 100);
    assert_eq!(app.history.len(), 0);
}
