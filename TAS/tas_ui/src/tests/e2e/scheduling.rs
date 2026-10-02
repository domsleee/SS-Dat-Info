//! Frame-ordering edges: things that land between, or inside, UI frames.

use super::*;

/// Type a new Start for the L block (110 -> 115) without committing it.
fn type_left_start(h: &mut Harness) {
    h.click("timeline.block.L.110");
    h.click("timeline.start");
    h.clear_field();
    h.type_text("115");
}

/// The standard take with L 110-120 shortened to 115-120.
fn shortened(take: &[u8]) -> Vec<u8> {
    let mut edited = take.to_vec();
    for byte in &mut edited[110..115] {
        *byte &= !LEFT;
    }
    edited
}

#[derive(Debug, Clone, Copy)]
enum Press {
    /// The PLAY button, on the frame after Enter.
    Button,
    /// F10 in the window, on the frame after Enter.
    Key,
    /// Enter and F10 in one frame.
    KeySameFrame,
    /// F10 in the game, on the frame after Enter.
    GlobalKey,
}

/// Commit a typed edit with Enter and press PLAY right after it: the take
/// must replay edited, judged only up to the edit.
fn edit_then_play(press: Press) {
    let mut h = Harness::with_standard_take();
    let edited = shortened(&h.input());
    type_left_start(&mut h);
    match press {
        Press::Button => {
            h.key(Key::Enter, Modifiers::NONE);
            h.click("transport.play");
        }
        Press::Key => {
            h.key(Key::Enter, Modifiers::NONE);
            h.key(Key::F10, Modifiers::NONE);
        }
        Press::KeySameFrame => {
            let mut events = key_events(Key::Enter, Modifiers::NONE);
            events.extend(key_events(Key::F10, Modifiers::NONE));
            h.frame_with(events, Modifiers::NONE);
        }
        Press::GlobalKey => {
            h.key(Key::Enter, Modifiers::NONE);
            h.world().keys_down = vec![crate::win32::VK_F10];
            h.world().foreground = Some(GAME_PID);
            h.frame();
            h.world().keys_down.clear();
        }
    }
    h.run_until("PLAY runs", 2000, |h| {
        !h.app.transport.is_running() && h.mode() == TasMode::Off
    });
    assert!(!h.aborted(), "{press:?}\n{}", h.log());
    assert_eq!(h.input(), edited, "{press:?}");
    assert_eq!(
        h.game().taken.last(),
        Some(&TasCommand::ArmPlay),
        "{press:?}"
    );
}

/// A script edit saved during a PLAY stops the run at once, while its
/// watcher is still judging, then applies; it does not wait out the cycle.
#[test]
fn an_edit_during_play_stops_the_run_at_once() {
    let mut h = Harness::with_standard_take();
    h.click("transport.play");
    h.run_until("PLAY replaying", 2000, |h| h.mode() == TasMode::Play);
    assert!(h.app.transport.is_running(), "the watcher is still judging");
    let edited = shortened(&h.input());
    let events = crate::panels::input_script::runs_from_log(&edited, edited.len() as u32);
    h.app.editor.pending = crate::editor::PendingEdit::Ready(crate::editor::Edit {
        events,
        commit: true,
        label: "Loaded inputs from script".into(),
    });
    h.run_until("edit applied", 2000, |h| {
        h.mode() == TasMode::Off && !h.app.editor.has_pending()
    });
    assert_eq!(h.input(), edited);
    assert_eq!(
        h.taken().last(),
        Some(&TasCommand::Stop),
        "stopped, not left to end on its own"
    );
}

/// A click is handled after the frame applied the pending edit.
#[test]
fn play_clicked_right_after_an_edit_replays_the_edit() {
    edit_then_play(Press::Button);
}

/// Shortcuts used to dispatch before the frame applied a pending edit, so the
/// PLAY armed with the old edit limit and the watcher aborted the edited
/// replay as a TAS bug; with Enter and F10 in one frame the edit even landed
/// mid-cycle, between the restart and the arm.
#[test]
fn play_keyed_right_after_an_edit_replays_the_edit() {
    for press in [Press::Key, Press::KeySameFrame, Press::GlobalKey] {
        edit_then_play(press);
    }
}

/// The finish can come before the UI has seen REC at all (a CONT spliced
/// just before the line, or a slow frame): the take still stops there and
/// is labelled with its race time.
#[test]
fn a_finish_before_the_ui_sees_rec_still_stops_the_take() {
    let mut h = Harness::new();
    h.game().finish_x = Some(fake_game::SPAWN[0] + 30.0);
    h.click("transport.rec");
    h.run_until("REC is armed", 100, |h| !h.app.transport.is_running());
    assert_eq!(h.mode(), TasMode::Rec);
    assert_eq!(
        h.app.session.last_mode,
        TasMode::Off as u32,
        "the UI has not seen REC"
    );
    let finishes = tas_shared::race_clock::race_finish(h.state()).map_or(0, |f| f.seq);
    h.run_game_until("the finish", |s| {
        tas_shared::race_clock::race_finish(s).is_some_and(|f| f.seq > finishes)
    });
    let finish = tas_shared::race_clock::race_finish(h.state()).unwrap();
    h.run_game_until("a few more REC ticks", |s| {
        s.recorded_count > finish.tick + 5
    });

    h.run_until("the take stops", 50, |h| {
        h.app.session.finish.at_tick.is_some() && h.app.session.active.is_none()
    });
    assert_eq!(h.app.session.finish.at_tick, Some(finish.tick));
    assert_eq!(h.log_count(&format!("Finished at tick {}", finish.tick)), 1);
    let cs = tas_shared::race_clock::hud_cs(finish.seconds, true);
    let entry = h.app.history.list.entries().last().unwrap();
    assert_eq!(entry.finish_time_cs, Some(cs));
    assert!(entry.label.starts_with("Finish 0:00."), "{}", entry.label);
}

/// A whole short PLAY can run between two UI frames (a stalled UI): the
/// cycle then judges it from what was captured, not as a refused arm.
#[test]
fn a_whole_play_between_two_frames_is_judged() {
    for edit in [false, true] {
        let mut h = Harness::with_standard_take();
        if edit {
            super::editing::delete_the_left_block(&mut h);
        }
        let generation = h.state().arm_generation;
        h.click("transport.play");
        h.run_until("the arm is sent", 200, |h| {
            h.state().arm_generation != generation
                || h.state().command == TasCommand::ArmPlay as u32
        });
        assert!(h.app.transport.is_running());
        h.run_game_until("the replay ends", |s| {
            s.arm_generation != generation && s.mode == TasMode::Off as u32
        });
        h.frames(2);
        assert!(!h.app.transport.is_running());
        assert!(!h.aborted(), "edit={edit}\n{}", h.log());
        if !edit {
            assert!(h.replay_matches_take());
        }
    }
}
