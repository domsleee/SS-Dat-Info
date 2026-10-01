//! Restore, undo and load while a take is running: each stops it first and
//! overwrites the buffer only once the DLL has acknowledged the STOP.

use super::editing::delete_the_left_block;
use super::*;
use crate::recording::RecordingFile;

/// Click PLAY and run until the replay is well under way.
fn start_play(h: &mut Harness) {
    h.click("transport.play");
    h.run_until("PLAY replays", 500, |h| {
        h.mode() == TasMode::Play && h.state().playback_pos > 100
    });
}

/// A history restore during PLAY waits for the (late) STOP before it
/// overwrites the take; a STOP later than the wait refuses the restore and
/// leaves the buffer alone.
#[test]
fn a_restore_during_play_overwrites_only_after_the_stop() {
    let mut h = Harness::with_standard_take();
    let take = h.input();
    let recorded = h.app.history.list.entries().last().unwrap().entry_id;
    delete_the_left_block(&mut h);
    let edited = h.input();
    let edit = h.app.history.list.entries().last().unwrap().entry_id;

    start_play(&mut h);
    h.game().delays = vec![(TasCommand::Stop, 10)];
    h.click(&format!("history.row.{recorded}"));
    assert_eq!(
        h.game().logs_at_stop.last(),
        Some(&edited),
        "overwritten before STOP"
    );
    assert_eq!(h.input(), take);
    assert_eq!(h.log_count("History restore: "), 1, "{}", h.log());

    start_play(&mut h);
    h.game().delays = vec![(TasCommand::Stop, 40)];
    h.click(&format!("history.row.{edit}"));
    let refusal = "Load/restore refused: Stop was not acknowledged; recording buffer unchanged";
    assert_eq!(h.log_count(refusal), 1, "{}", h.log());
    h.run_until("the late STOP lands", 100, |h| h.mode() == TasMode::Off);
    h.frames(5);
    assert_eq!(h.input(), take);
}

/// The track can change while a restore waits for its STOP: the restore
/// checks the entry against the track after the STOP, not before.
#[test]
fn a_restore_rechecks_the_track_after_its_stop() {
    let mut h = Harness::with_standard_take();
    let recorded = h.app.history.list.entries().last().unwrap().entry_id;
    delete_the_left_block(&mut h);
    let edited = h.input();
    start_play(&mut h);
    h.game().on_stop = Some(Box::new(|g| g.set_level(VE)));
    h.click(&format!("history.row.{recorded}"));
    let refusal = "History restore refused: that entry is not for the current track (VE)";
    assert_eq!(h.log_count(refusal), 1, "{}", h.log());
    assert_eq!(h.input(), edited);
}

/// Ctrl+Z during REC stops it, keeps the take in history, then undoes to
/// the take before it.
#[test]
fn undo_during_rec_keeps_the_take_and_restores_the_one_before() {
    let mut h = Harness::with_standard_take();
    let first = h.input();
    h.game().keyboard = Box::new(|i| if (90..140).contains(&i) { LEFT } else { 0 });
    h.click("transport.rec");
    h.run_until("REC records", 500, |h| {
        h.mode() == TasMode::Rec && h.state().recorded_count > 150
    });
    h.game().delays = vec![(TasCommand::Stop, 5)];
    let entries = h.app.history.list.len();
    h.key(Key::Z, Modifiers::CTRL);
    assert_eq!(h.input(), first);
    assert_eq!(h.app.history.list.len(), entries + 1);
    let second = h.game().logs_at_stop.last().unwrap().clone();
    assert_eq!(second[100], LEFT, "the second take was stopped");
    let entry = h.app.history.list.entries().last().unwrap();
    assert!(entry.label.starts_with("Recorded"), "{}", entry.label);
    let snapshot = h.app.history.list.restore_index(entries).unwrap();
    assert_eq!(snapshot.input_log[..second.len()], second[..]);
}

/// Save stamps the take's identity and edit limit into the file; loading it
/// back during another take's PLAY (late STOP) restores them, and the PLAY
/// it re-arms honours the edit.
#[test]
fn identity_and_edit_limit_survive_save_and_a_load_during_play() {
    let mut h = Harness::with_standard_take();
    delete_the_left_block(&mut h);
    let edited = h.input();
    let stamps = h.app.take.identity.stamps.clone().unwrap();
    let path = h.world().dir.join("FE-e2e.tasrec");
    h.world().save_path = Some(path.clone());
    h.key(Key::S, Modifiers::CTRL);
    let meta = RecordingFile::read_metadata(&path).unwrap();
    assert_eq!(meta.trajectory_ticks, Some(110));
    assert_eq!(meta.character.as_deref(), Some("Vincent"));
    assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
    assert_eq!(meta.fpu_control_word, Some(0x027F));
    assert_eq!(meta.input_model, Some(tas_shared::TAS_INPUT_MODEL_HELD));

    h.record(|i| if (70..90).contains(&i) { UP } else { 0 }, 200);
    assert_eq!(
        h.app
            .take
            .identity
            .stamps
            .as_ref()
            .unwrap()
            .trajectory_ticks,
        None
    );
    start_play(&mut h);
    h.game().delays = vec![(TasCommand::Stop, 5)];
    h.world().load_path = Some(path);
    h.key(Key::O, Modifiers::CTRL);
    assert_eq!(h.input(), edited);
    assert_eq!(h.app.take.identity.stamps.as_ref(), Some(&stamps));
    assert_eq!(h.app.take.identity.level.as_deref(), Some("FE"));
    let entry = h.app.history.list.entries().last().unwrap();
    assert_eq!(entry.label, "Load: FE-e2e.tasrec");
    assert_eq!(entry.stamps, stamps);

    h.run_until("the loaded take replays", 2000, |h| {
        !h.app.transport.is_running() && h.mode() == TasMode::Off
    });
    assert!(!h.aborted(), "{}", h.log());
    assert_eq!(h.log_count("In-process restart → ArmPlay"), 2);
}
