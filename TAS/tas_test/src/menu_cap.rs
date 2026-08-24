//! Validate the menu present-cap engagement — the "menu is sometimes slow and
//! sometimes fast" report.
//!
//! The menu video decodes at most one frame per Paint, gated at 40 ms of
//! accumulated menu time, so its speed is a step function of the present rate:
//! capped at 20 fps it runs native in every state; uncapped it dips to half
//! speed on a fresh menu and runs ~3x after a level round-trip. The cap
//! (frame_limit) engages when the engine cycle is frozen — BUT it is disabled
//! while `cont_suppress_input` is set, because a present-Sleep inside a CONT's
//! F5 reload could shift the spawn lottery. A judged cycle that never tore
//! down leaves that flag set forever, permanently disabling the cap: exactly
//! the reported symptom.
//!
//! The fix under test: a cycle frozen for >5s with the flag still up treats
//! the flag as stale and lets the cap engage anyway (no CONT reload freeze
//! lasts that long), plus the root-change auto-stop clears the flag.
//!
//! Method — the pause menu freezes the cycle exactly like the main menu, and
//! `present_count` measures the present rate without screen capture:
//!   1. in-game idle: presents unthrottled (cycle live)     — baseline
//!   2. ESC pause, flag clear: cap must engage (~20/s)      — normal path
//!   3. flag SET while paused: cap disengages (CONT guard)  — by design
//!   4. still paused >5s with flag set: cap re-engages      — THE FIX
//!   5. ESC resume, flag cleared.

use std::thread;
use std::time::Duration;

use crate::harness;

const CAP_FPS: f64 = 20.0;

fn presents_per_sec(client: &tas_shared::TasSharedMemoryClient, secs: f64) -> f64 {
    let p0 = client.state().present_count;
    thread::sleep(Duration::from_millis((secs * 1000.0) as u64));
    let p1 = client.state().present_count;
    f64::from(p1.wrapping_sub(p0)) / secs
}

pub fn run() -> bool {
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    client.state_mut().cont_suppress_input = 0;
    thread::sleep(Duration::from_millis(200));

    println!("=== Menu present-cap engagement (pause menu == frozen cycle) ===");
    println!("  cap value: {} fps", client.state().menu_fps_cap);

    // 1. Baseline: in-game, cycle live, no throttle.
    let live = presents_per_sec(&client, 2.0);
    println!("  in-game (cycle live):            {:.1} presents/s", live);

    // 2. Pause. The cycle freezes; after 250ms staleness the cap engages.
    if !harness::send_escape() {
        eprintln!("ERROR: could not send ESC");
        return false;
    }
    thread::sleep(Duration::from_millis(600));
    let paused_capped = presents_per_sec(&client, 2.0);
    println!(
        "  paused, flag clear:              {:.1} presents/s (cap should hold ~{:.0})",
        paused_capped, CAP_FPS
    );

    // 3. Set the CONT flag: the cap must disengage (the reload guard, by design).
    client.state_mut().cont_suppress_input = 1;
    thread::sleep(Duration::from_millis(300));
    let paused_suppressed = presents_per_sec(&client, 2.0);
    println!(
        "  paused, flag SET (<5s frozen):   {:.1} presents/s (guard disengages the cap)",
        paused_suppressed
    );

    // 4. Keep the flag set. Once the cycle has been frozen >5s the flag is
    //    stale by definition and the cap must re-engage — THE FIX.
    thread::sleep(Duration::from_millis(2500));
    let paused_stale = presents_per_sec(&client, 2.0);
    println!(
        "  paused, flag SET (>5s frozen):   {:.1} presents/s (stale override re-engages)",
        paused_stale
    );

    // 5. Resume and clean up.
    client.state_mut().cont_suppress_input = 0;
    harness::send_escape();
    thread::sleep(Duration::from_millis(400));

    let cap_engaged = paused_capped < CAP_FPS * 1.5;
    let guard_disengaged = paused_suppressed > CAP_FPS * 1.8 || live < CAP_FPS * 1.8;
    let stale_reengaged = paused_stale < CAP_FPS * 1.5;
    println!();
    println!(
        "  cap engages when frozen: {} | CONT guard disengages: {} | stale flag overridden: {}",
        cap_engaged, guard_disengaged, stale_reengaged
    );
    if cap_engaged && stale_reengaged {
        println!("\n*** MENU-CAP PASSED: a stale suppress flag can no longer uncap the menu ***");
        true
    } else {
        println!("\n*** MENU-CAP FAILED ***");
        false
    }
}
