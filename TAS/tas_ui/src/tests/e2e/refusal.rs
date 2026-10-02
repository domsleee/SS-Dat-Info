//! PLAY and CONT refuse a take that cannot match the live game, on every
//! path that arms, and send nothing.

use super::*;
use crate::win32::{VK_F10, VK_F12};
use tas_shared::{TAS_CHARACTER_KEITH, TAS_CHARACTER_UNKNOWN, TAS_CHARACTER_VINCENT};

#[derive(Debug, Clone, Copy)]
enum Path {
    Button,
    Key,
    /// F10 / F12 polled globally while the game has focus.
    GlobalKey,
}

fn arm(h: &mut Harness, play: bool, path: Path) {
    match path {
        Path::Button => h.click(if play {
            "transport.play"
        } else {
            "transport.cont"
        }),
        Path::Key => h.key(if play { Key::F10 } else { Key::F12 }, Modifiers::NONE),
        Path::GlobalKey => {
            let mut world = h.world();
            world.keys_down = vec![if play { VK_F10 } else { VK_F12 }];
            world.foreground = Some(GAME_PID);
            drop(world);
            h.frame();
            h.world().keys_down.clear();
        }
    }
    h.frames(2);
}

/// What an arm would have sent: commands taken, ownership requested, a
/// command left in the slot, a controller.
fn sent(h: &Harness) -> (usize, u32, u32, bool) {
    let s = h.state();
    (
        h.taken().len(),
        s.owner_request_seq
            .load(std::sync::atomic::Ordering::Acquire),
        s.command,
        h.app.transport.is_running(),
    )
}

type Change = fn(&mut FakeGame);

/// A take from another track, rider or x87 precision is refused by the
/// PLAY and CONT buttons, F10 / F12 in the window and F10 / F12 in the game,
/// before anything reaches the DLL. Unknown on the live side still arms.
#[test]
fn a_take_that_cannot_match_is_refused_on_every_path() {
    let mut h = Harness::with_standard_take();
    h.click("transport.from");
    h.clear_field();
    h.type_text("150");
    h.frame();
    let cases: [(Change, Change, &str); 3] = [
        (
            |g| g.set_level(VE),
            |g| g.set_level(FE),
            "refused: this take was recorded on FE but the game is on VE",
        ),
        (
            |g| g.set_rider(TAS_CHARACTER_KEITH, 0),
            |g| g.set_rider(TAS_CHARACTER_VINCENT, 0),
            "refused: this take was recorded as Vincent · regular but the rider is Keith · regular",
        ),
        (
            |g| g.set_fpu_control_word(0x007F),
            |g| g.set_fpu_control_word(0x027F),
            "refused: this take was recorded at 53-bit x87 precision but the game runs at 24-bit",
        ),
    ];
    for (change, restore, refusal) in cases {
        change(&mut h.game());
        h.frames(2);
        for play in [true, false] {
            for path in [Path::Button, Path::Key, Path::GlobalKey] {
                let before = sent(&h);
                let refusals = h.log_count(refusal);
                arm(&mut h, play, path);
                let case = format!("{refusal} / play={play} / {path:?}");
                assert_eq!(sent(&h), before, "{case}: sent something");
                assert_eq!(h.log_count(refusal), refusals + 1, "{case}\n{}", h.log());
            }
        }
        restore(&mut h.game());
        h.frames(2);
    }

    // Unknown cannot prove a mismatch.
    let mut game = h.game();
    game.unresolve_level();
    game.set_rider(TAS_CHARACTER_UNKNOWN, u32::MAX);
    game.set_fpu_control_word(0);
    drop(game);
    h.frames(2);
    h.play();
    assert!(h.taken().ends_with(&[
        TasCommand::StopForRestart,
        TasCommand::Restart,
        TasCommand::ArmPlay
    ]));
    assert!(!h.aborted(), "{}", h.log());
    assert!(h.replay_matches_take());
}
