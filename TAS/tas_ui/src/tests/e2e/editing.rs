//! Editing a stopped take through the timeline, then replaying it.

use super::*;

/// `take` with `bit` added over `ticks`.
pub fn with_bit(take: &[u8], bit: u8, ticks: std::ops::Range<usize>) -> Vec<u8> {
    let mut edited = take.to_vec();
    for i in ticks {
        edited[i] |= bit;
    }
    edited
}

/// Somewhere with no widget, to take focus away by clicking.
const NOWHERE: Pos2 = Pos2::new(400.0, 650.0);

/// Typed Start and End values commit once, on Enter or on focus loss, never
/// per keystroke: "1" and "10" on the way to 105 would each have swallowed
/// the R block at 80-100. Undo then walks back exactly.
#[test]
fn typed_start_and_end_commit_once_and_undo_exactly() {
    let mut h = Harness::with_standard_take();
    let original = h.input();
    let entries = h.app.history.list.len();
    h.click("timeline.block.R.130");
    let selected = h
        .app
        .editor
        .timeline
        .selected
        .map(|e| (e.bit, e.start, e.end));
    assert_eq!(selected, Some((RIGHT, 130, 170)));

    // Start 130 -> 105, typed, Enter.
    h.click("timeline.start");
    h.clear_field();
    for c in ["1", "0", "5"] {
        h.type_text(c);
        assert_eq!(h.input(), original, "applied before Enter");
        assert!(!h.app.editor.has_pending());
    }
    h.key(Key::Enter, Modifiers::NONE);
    h.frame();
    let moved = with_bit(&original, RIGHT, 105..130);
    assert_eq!(h.input(), moved);
    assert_eq!(h.app.history.list.len(), entries + 1);
    assert_eq!(
        h.app.history.list.entries().last().unwrap().label,
        "Set R 105-170t"
    );

    // End 170 -> 175, typed, committed by clicking elsewhere.
    h.click("timeline.end");
    h.clear_field();
    for c in ["1", "7", "5"] {
        h.type_text(c);
        assert_eq!(h.input(), moved, "applied before focus loss");
    }
    h.click_at(NOWHERE);
    h.frames(2);
    let stretched = with_bit(&moved, RIGHT, 170..175);
    assert_eq!(h.input(), stretched);
    assert_eq!(
        h.app.history.list.len(),
        entries + 2,
        "one entry per commit"
    );
    let limit = h
        .app
        .take
        .identity
        .stamps
        .as_ref()
        .unwrap()
        .trajectory_ticks;
    assert_eq!(limit, Some(105), "the edit cut the judged trajectory");

    h.click("transport.undo");
    h.frame();
    assert_eq!(h.input(), moved);
    h.click("transport.undo");
    h.frame();
    assert_eq!(h.input(), original);
    let limit = h
        .app
        .take
        .identity
        .stamps
        .as_ref()
        .unwrap()
        .trajectory_ticks;
    assert_eq!(limit, None, "the original take is whole again");
}

/// Delete L 110-120 through the editor.
pub fn delete_the_left_block(h: &mut Harness) {
    h.click("timeline.block.L.110");
    h.click("timeline.delete");
    h.frame();
    let limit = h
        .app
        .take
        .identity
        .stamps
        .as_ref()
        .unwrap()
        .trajectory_ticks;
    assert_eq!(limit, Some(110));
}

/// An edited take replays its new input, departing from the stale recorded
/// trajectory after the edit, without a false abort; the drift banner does
/// not call that drift either.
#[test]
fn an_edited_take_replays_past_its_edit() {
    let mut h = Harness::with_standard_take();
    delete_the_left_block(&mut h);
    h.play();
    assert!(!h.aborted(), "{}", h.log());
    assert_eq!(h.log_count("DRIFT"), 0, "{}", h.log());
    let s = h.state();
    let rec_gate = h.rec_gate();
    let live_gate = *h.game().live_gates.last().unwrap() as usize;
    let first_diff = (0..s.recorded_count as usize - rec_gate)
        .find(|&k| s.rec_coords[rec_gate + k] != s.play_coords[live_gate + k]);
    // Positions are captured before each tick's physics.
    assert_eq!(
        first_diff,
        Some(111 - rec_gate),
        "the replay departs after the edit"
    );
}

/// A replay that differs BEFORE the edit still aborts: the edit limits the
/// check, it does not switch it off.
#[test]
fn an_edited_take_still_aborts_on_a_difference_before_the_edit() {
    let mut h = Harness::with_standard_take();
    delete_the_left_block(&mut h);
    h.game().perturb_at = Some(20);
    h.play();
    assert_eq!(
        h.log_count("PLAY aborted: replay diverged at gate+20"),
        1,
        "{}",
        h.log()
    );
}
