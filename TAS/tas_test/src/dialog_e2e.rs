//! END-TO-END validation of the two reported clock bugs, driven the way a USER
//! hits them — real finishes, a real (Pico HID) keypress on the save-replay
//! dialog, and the ACTUAL main menu afterwards:
//!
//!   "After finishing, if you don't press anything on the 'save replay?'
//!    screen, it will play in fast forward when you do press it, for a while."
//!   "The menu speed is sometimes slow and sometimes fast."
//!
//! One continuous session, driven with real input:
//!
//!   PRECHECK — unfinished-race quit confirmation hides the underlying menu,
//!              refuses stale commands, and restores navigation after No.
//!
//!   PHASE A — aligned PLAY of a finishing recording at 1x, idle at the save
//!             dialog, decline with physical RIGHT + Enter. No tick burst allowed.
//!   PHASE B — the TAS user's actual flow: CONT near the finish, the splice
//!             flips to REC, the run crosses the line RECORDING, the dialog
//!             appears in REC mode. Idle, physical RIGHT + Enter, no burst.
//!   PHASE C — quit to the REAL main menu (pause-menu recipe) and measure
//!             what the user sees: cont_suppress_input must read 0 (a stale
//!             flag here kills the keyboard), and the menu VIDEO must move at
//!             a plausible rate (video-rate's screen sampler).

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasMode;

use crate::harness;
use crate::replay;
use crate::video_rate;

const RECORDING: &str = "FE-decent-done.tasrec";
/// Splice close to the finish (race ends ~tick 6800) so the post-splice REC
/// coasts across the line with no live input, like a user redoing the ending.
const CONT_SPLICE_FRAME: u32 = 6700;
const DIALOG_IDLE_SECS: u64 = 12;
/// Native is 100 ticks/sec; the 100ms buckets read ~10-11. A backlog burst is
/// an order of magnitude out, so a generous ceiling still separates them.
const MAX_TICKS_PER_SEC: f64 = 140.0;

fn unfinished_quit_modal(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    const SCREEN: &str = "ID_ARCADE_IN_GAME_MENU";
    fn command(
        client: &mut tas_shared::TasSharedMemoryClient,
        kind: u32,
        target: &str,
    ) -> Option<u32> {
        let seq = tas_shared::menu_command_submit(client.state_mut(), kind, target, SCREEN).ok()?;
        crate::menu::wait_for_ack(client, seq)
    }
    println!("--- Unfinished race: confirmation must hide and protect the pause menu ---");
    if !harness::restart_and_stabilize_inprocess(client) || !harness::send_escape() {
        return false;
    }
    thread::sleep(Duration::from_millis(300));
    if command(
        client,
        tas_shared::TAS_MENU_CMD_ACTIVATE,
        "ID_IN_GAME_QUIT_GAME",
    ) != Some(tas_shared::TAS_MENU_RESULT_OK)
    {
        eprintln!("FAIL: could not open unfinished-race quit confirmation");
        return false;
    }
    thread::sleep(Duration::from_millis(300));
    let hidden = tas_shared::menu_doc(client.state()).is_none();
    let mut protected = true;
    for kind in [
        tas_shared::TAS_MENU_CMD_ACTIVATE,
        tas_shared::TAS_MENU_CMD_FOCUS,
        tas_shared::TAS_MENU_CMD_UP,
        tas_shared::TAS_MENU_CMD_DOWN,
        tas_shared::TAS_MENU_CMD_LEFT,
        tas_shared::TAS_MENU_CMD_RIGHT,
        tas_shared::TAS_MENU_CMD_TRIGGER,
    ] {
        // A previously read target must not act through the modal, even if
        // its expected underlying screen still matches.
        protected &= command(client, kind, "ID_IN_GAME_QUIT_GAME")
            == Some(tas_shared::TAS_MENU_RESULT_NO_MENU);
    }
    // This confirmation has the same physical Yes/No controls as the finish prompt.
    if !harness::dismiss_finish_prompt() {
        return false;
    }
    thread::sleep(Duration::from_millis(300));
    let restored = tas_shared::menu_doc(client.state()).is_some()
        && command(client, tas_shared::TAS_MENU_CMD_ACTIVATE, "ID_CONTINUE")
            == Some(tas_shared::TAS_MENU_RESULT_OK);
    println!("  hidden={hidden}, all commands refused={protected}, Continue restored={restored}");
    hidden && protected && restored
}

fn idle_profile_passes(idle_ticks: u32, resumed_ticks: u32, peak_rate: f64) -> bool {
    idle_ticks <= 1 && resumed_ticks > 0 && (0.0..=MAX_TICKS_PER_SEC).contains(&peak_rate)
}

/// Wait until the engine freezes at the post-race dialog: frame_count stops
/// advancing for >2s. Returns false on the deadline.
fn wait_engine_frozen(client: &tas_shared::TasSharedMemoryClient, deadline_secs: u64) -> bool {
    let deadline = Instant::now() + Duration::from_secs(deadline_secs);
    let mut last_fc = client.state().frame_count;
    let mut stable_since = Instant::now();
    let mut last_beat = Instant::now();
    loop {
        if Instant::now() > deadline {
            let s = client.state();
            eprintln!(
                "ERROR: engine never froze at the dialog (fc={} pos={} mode={} rec={})",
                s.frame_count, s.playback_pos, s.mode, s.recorded_count
            );
            return false;
        }
        thread::sleep(Duration::from_millis(100));
        let state = client.state();
        if state.mode == TasMode::Off as u32
            && state.recorded_count > CONT_SPLICE_FRAME
            && state.playback_pos >= state.recorded_count
        {
            eprintln!("FAIL: fixture finished without the save prompt; use an isolated game copy with fresh Forest Easy scores, not your personal score table");
            return false;
        }
        let fc = client.state().frame_count;
        if fc != last_fc {
            last_fc = fc;
            stable_since = Instant::now();
        } else if stable_since.elapsed() > Duration::from_secs(2) {
            let state = client.state();
            let progress = if state.mode == TasMode::Rec as u32 {
                state.recorded_count
            } else {
                state.playback_pos
            };
            // A stalled restart or an early pause is not the finishing dialog.
            // This fixture crosses the line after the near-finish splice.
            if progress > CONT_SPLICE_FRAME
                && state.race_time_cs != u32::MAX
                && state.race_time_cs > 0
            {
                return true;
            }
            eprintln!(
                "FAIL: engine froze before a finishing run (progress={progress}, race_time={})",
                state.race_time_cs
            );
            return false;
        }
        if last_beat.elapsed() > Duration::from_secs(5) {
            last_beat = Instant::now();
            let s = client.state();
            println!(
                "  [beat] fc={} pos={} mode={} rec={} tc={}",
                s.frame_count, s.playback_pos, s.mode, s.recorded_count, s.tick_count
            );
        }
    }
}

/// Idle at the dialog, physically select No and confirm, profile the tick rate.
/// Returns (max_ticks_per_sec, ok).
fn idle_dismiss_profile(client: &tas_shared::TasSharedMemoryClient, label: &str) -> (f64, bool) {
    println!(
        "  [{}] idling {}s at the dialog (untouched)...",
        label, DIALOG_IDLE_SECS
    );
    let t0 = client.state().tick_count;
    thread::sleep(Duration::from_secs(DIALOG_IDLE_SECS));
    let idled = client.state().tick_count.wrapping_sub(t0);
    println!(
        "  [{}] during idle: {} ticks advanced (must be ~0)",
        label, idled
    );

    // Escape does not confirm the "Save attack player?" prompt. Selecting
    // No avoids writing a ghost; Enter remains physical HID, not PostMessage.
    if !harness::dismiss_finish_prompt() {
        eprintln!("FAIL: {label}: physical dialog dismissal failed");
        return (f64::INFINITY, false);
    }

    let mut prev = client.state().tick_count;
    let after_dismiss = prev;
    let mut max_bucket = 0u32;
    for _ in 0..30 {
        thread::sleep(Duration::from_millis(100));
        let tc = client.state().tick_count;
        max_bucket = max_bucket.max(tc.wrapping_sub(prev));
        prev = tc;
    }
    let rate = f64::from(max_bucket) * 10.0;
    let resumed_ticks = prev.wrapping_sub(after_dismiss);
    let resumed = resumed_ticks > 0;
    let ok = idle_profile_passes(idled, resumed_ticks, rate);
    if !resumed || idled > 1 {
        eprintln!("FAIL: {label}: idle ticks={idled}, resumed={resumed}");
        eprintln!(
            "Menu after dismissal: {:?}",
            tas_shared::menu_doc(client.state())
        );
    }
    println!(
        "  [{}] post-dismiss max rate {:.0} ticks/sec (ceiling {:.0}) -> {}",
        label,
        rate,
        MAX_TICKS_PER_SEC,
        if ok { "OK" } else { "BURST" }
    );
    (rate, ok)
}

pub fn run() -> bool {
    // Enter uses a reserved firmware command; an older firmware must never
    // interpret it as a combination of game keys.
    harness::stop_competing_tas_ui_writer();
    if let Err(error) = crate::pico::discover(true) {
        eprintln!("ERROR: Finish-dialog firmware preflight: {error}");
        return false;
    }
    let path = match harness::fixture_path(RECORDING) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    println!(
        "=== Save-dialog + menu-speed END TO END: {} ===",
        path.display()
    );

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(200));

    if !unfinished_quit_modal(&mut client) {
        eprintln!("FAIL: unfinished-race modal protection");
        return false;
    }

    let loaded = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    println!("  {} ticks loaded", loaded.count);
    harness::focus_game();

    println!("--- PHASE A: aligned PLAY at 1x through the finish ---");
    client.state_mut().playback_speed = 1.0;
    if harness::restart_play_aligned_inprocess(&mut client).is_none() {
        eprintln!("ERROR: aligned PLAY failed to arm");
        return false;
    }
    if !wait_engine_frozen(&client, 240) {
        return false;
    }
    println!(
        "  finish reached: pos={} race_time_cs={:#x}",
        client.state().playback_pos,
        client.state().race_time_cs
    );
    let (_rate_a, pass_a) = idle_dismiss_profile(&client, "A/PLAY");
    if !pass_a {
        return false;
    }

    println!(
        "--- PHASE B: CONT from {} -> splice -> REC crosses the finish ---",
        CONT_SPLICE_FRAME
    );
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(300));
    // Phase A's run is done; reload so recorded_count/input are pristine
    // (a splice truncates them).
    replay::write_to_shared(&mut client, &loaded);
    let target = client.state().rec_coords[0];
    client.state_mut().cont_resume_speed = 1.0; // ride the ending at 1x, like a user
    client.state_mut().playback_speed = 64.0;
    if harness::restart_continue_and_splice_inprocess(&mut client, target, CONT_SPLICE_FRAME, 30)
        .is_none()
    {
        eprintln!("ERROR: CONT cycle failed");
        return false;
    }
    let mode_after = client.state().mode;
    println!(
        "  splice done: mode={} ({}) recorded_count={}",
        mode_after,
        if mode_after == TasMode::Rec as u32 {
            "REC"
        } else {
            "not REC!"
        },
        client.state().recorded_count
    );
    if !wait_engine_frozen(&client, 120) {
        return false;
    }
    let rec_at_dialog = client.state().recorded_count;
    let pass_b_rec =
        client.state().mode == TasMode::Rec as u32 && rec_at_dialog > CONT_SPLICE_FRAME;
    println!(
        "  dialog reached IN REC: recorded_count={} (> splice {}) -> {}",
        rec_at_dialog,
        CONT_SPLICE_FRAME,
        if pass_b_rec { "OK" } else { "WRONG MODE/COUNT" }
    );
    let (_rate_b, pass_b_burst) = idle_dismiss_profile(&client, "B/REC");

    println!("--- PHASE C: quit to the main menu, measure what the user sees ---");
    // Entering the pause menu needs a physical key; every step after goes
    // through the menu protocol — read the document, activate by published
    // id, wait for the ack, verify the destination page. No blind cursor
    // counting: a wrong screen fails here instead of silently measuring it.
    if !harness::send_escape() {
        eprintln!("ERROR: opening the pause menu failed; Phase C would measure the wrong screen");
        return false;
    }
    let arcade = match crate::menu::activate_and_wait(
        &mut client,
        "Return To Menu",
        Some("ID_ARCADE_MENU"),
        Duration::from_secs(10),
    ) {
        Ok(screen) => screen,
        Err(e) => {
            eprintln!("ERROR: pause-menu navigation failed: {}", e);
            return false;
        }
    };
    println!("  pause menu -> {}", arcade);
    // Finish-menu Return To Menu goes directly to Arcade. Its back button
    // has no label, so select the published id rather than inventing a Yes.
    match crate::menu::activate_and_wait(
        &mut client,
        "ID_BACK",
        Some("ID_MAIN_MENU"),
        Duration::from_secs(10),
    ) {
        Ok(screen) => println!("  arcade menu -> {}", screen),
        Err(e) => {
            eprintln!("ERROR: arcade back navigation failed: {}", e);
            return false;
        }
    }

    // The user-visible thing itself: the menu video's on-screen motion.
    println!("  menu video (screen sampler):");
    let pass_c_video = video_rate::run(Some(6), None);

    // A CONT that completed normally never leaves the suppress flag set, and
    // a stale one is retired at the menu; a set flag here = dead keyboard.
    let suppress = client.state().cont_suppress_input;
    let pass_c_flag = suppress == 0;
    println!(
        "  cont_suppress_input at the menu: {} -> {}",
        suppress,
        if pass_c_flag {
            "clear OK"
        } else {
            "STALE (keyboard dead)"
        }
    );

    harness::stop(&mut client); // retire the frozen-armed REC left by the quit

    println!();
    println!(
        "  A: PLAY-dialog no burst: {} | B: REC-dialog reached: {} + no burst: {} | C: video: {} flag: {}",
        pass_a, pass_b_rec, pass_b_burst, pass_c_video, pass_c_flag
    );
    let ok = pass_a && pass_b_rec && pass_b_burst && pass_c_video && pass_c_flag;
    if ok {
        println!(
            "\n*** DIALOG-E2E PASSED: save-dialog and menu behave at native speed end to end ***"
        );
    } else {
        println!("\n*** DIALOG-E2E FAILED ***");
    }
    ok
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_frozen_game_or_failed_idle_cannot_pass_as_no_burst() {
        assert!(idle_profile_passes(0, 300, 110.0));
        assert!(!idle_profile_passes(0, 0, 0.0));
        assert!(!idle_profile_passes(30, 300, 110.0));
        assert!(!idle_profile_passes(0, 300, 150.0));
        assert!(!idle_profile_passes(0, 300, f64::NAN));
    }
}
