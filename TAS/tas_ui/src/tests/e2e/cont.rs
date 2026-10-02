//! CONT over an edited take, observed by the UI only after the splice.

use super::editing::delete_the_left_block;
use super::*;

/// The DLL clears the splice marker at the splice. A UI that first looks
/// some REC ticks later must still adopt the replayed prefix (gate-relative,
/// with the live gate unequal to the recording's), leave REC's own ticks
/// alone, save the whole take, and that take must then replay exactly.
#[test]
fn cont_over_an_edit_adopts_the_replayed_prefix() {
    let mut h = Harness::with_standard_take();
    let n = h.state().recorded_count as usize;
    let recorded = h.state().rec_coords[..n].to_vec();
    delete_the_left_block(&mut h);

    h.click("transport.from");
    h.clear_field();
    h.type_text("150");
    // The arm lands 4 ticks later than REC's did: the gates differ.
    h.game().delays.push((TasCommand::ArmContinue, 4));
    h.game().keyboard = Box::new(|i| if (160..190).contains(&i) { LEFT } else { 0 });
    h.click("transport.cont");
    h.run_until("the watcher approves the prefix", 1000, |h| {
        !h.app.transport.is_running()
    });
    assert!(!h.aborted(), "{}", h.log());
    assert_eq!(
        (h.mode(), h.state().continue_from_frame),
        (TasMode::Play, 150)
    );
    h.run_game_until("the splice, then REC", |s| {
        s.continue_from_frame == 0 && s.mode == TasMode::Rec as u32 && s.recorded_count >= 160
    });
    let rec_gate = h.rec_gate();
    let live_gate = *h.game().live_gates.last().unwrap() as usize;
    assert_ne!(rec_gate, live_gate);
    let own = h.state().rec_coords[150..160].to_vec();

    h.frame();
    let s = h.state();
    assert_eq!(s.rec_coords[..rec_gate], recorded[..rec_gate], "spawn kept");
    assert_eq!(
        s.rec_coords[rec_gate..150],
        s.play_coords[live_gate..live_gate + 150 - rec_gate],
        "the replayed prefix is the take's trajectory"
    );
    assert_ne!(
        s.rec_coords[111..150],
        recorded[111..150],
        "past the edit it moved"
    );
    assert_eq!(s.rec_coords[150..160], own[..], "REC's own ticks untouched");

    h.click("transport.stop");
    h.run_until("REC stops", 100, |h| h.app.session.active.is_none());
    let entry = h.app.history.list.entries().last().unwrap();
    assert_eq!(entry.start_tick, 150);
    assert!(entry.label.starts_with("Continued from"), "{}", entry.label);
    assert_eq!(entry.stamps.trajectory_ticks, None, "the new take is whole");
    let n = h.state().recorded_count as usize;
    let (input, coords) = (h.input(), h.state().rec_coords[..n].to_vec());
    let index = h.app.history.list.current_index().unwrap();
    let snapshot = h.app.history.list.restore_index(index).unwrap();
    assert_eq!(snapshot.input_log[..n], input[..]);
    assert_eq!(snapshot.rec_coords[..n], coords[..]);

    h.play();
    assert!(!h.aborted(), "{}", h.log());
    assert!(h.replay_matches_take());
    assert_eq!(h.app.view.drift.tracker.max_drift(), 0.0);
}

/// The frame that first sees REC adopts the replayed prefix and drops the
/// edit limit before the drift scan judges the splice. Scanned first, the
/// verdict would be skipped (the old limit is before the splice) or would
/// compare the replay with the stale trajectory.
#[test]
fn the_splice_is_judged_after_the_prefix_is_adopted() {
    let mut h = Harness::with_standard_take();
    delete_the_left_block(&mut h);
    h.click("transport.from");
    h.clear_field();
    h.type_text("150");
    h.game().delays.push((TasCommand::ArmContinue, 4));
    h.click("transport.cont");
    h.run_until("the watcher approves the prefix", 1000, |h| {
        !h.app.transport.is_running()
    });
    h.run_game_until("the splice, then REC", |s| {
        s.continue_from_frame == 0 && s.mode == TasMode::Rec as u32 && s.recorded_count >= 160
    });
    assert_eq!(h.log_count("CONT splice"), 0);
    h.frame();
    assert_eq!(
        h.log_count("CONT splice 150: X=0.000000000 Z=0.000000000"),
        1,
        "{}",
        h.log()
    );
    assert_eq!(h.log_count("CONT splice"), 1);
}
