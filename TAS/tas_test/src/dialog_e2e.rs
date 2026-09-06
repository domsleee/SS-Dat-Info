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
//!   PHASE A — aligned PLAY of a finishing recording at 1x, idle at the save
//!             dialog, dismiss with a REAL Pico Escape. No tick burst allowed.
//!   PHASE B — the TAS user's actual flow: CONT near the finish, the splice
//!             flips to REC, the run crosses the line RECORDING, the dialog
//!             appears in REC mode. Idle, real Pico Escape, no burst.
//!   PHASE C — quit to the REAL main menu (pause-menu recipe) and measure
//!             what the user sees: cont_suppress_input must read 0 (a stale
//!             flag here kills the keyboard), and the menu VIDEO must move at
//!             a plausible rate (video-rate's screen sampler).

use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasMode;

use crate::harness;
use crate::replay;
use crate::video_rate;

const RECORDING_REL: &str = "TAS/recordings/FE-decent-done.tasrec";
/// Splice close to the finish (race ends ~tick 6800) so the post-splice REC
/// coasts across the line with no live input, like a user redoing the ending.
const CONT_SPLICE_FRAME: u32 = 6700;
const DIALOG_IDLE_SECS: u64 = 12;
/// Native is 100 ticks/sec; the 100ms buckets read ~10-11. A burst (the old
/// bug: 1219 ticks in the first 100ms) is an order of magnitude out, so a
/// generous ceiling still separates them cleanly.
const MAX_TICKS_PER_SEC: f64 = 140.0;

fn locate_recording() -> Option<String> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
    ];
    candidates
        .iter()
        .find(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
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
        let fc = client.state().frame_count;
        if fc != last_fc {
            last_fc = fc;
            stable_since = Instant::now();
        } else if stable_since.elapsed() > Duration::from_secs(2) {
            return true;
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

/// Idle at the dialog, dismiss with a REAL Pico Escape, profile the tick rate.
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

    // THE REAL KEYPRESS: hardware HID Escape from the Pico, exactly what a
    // user's keyboard sends. (The Pico mask has no Enter; any key resumes the
    // engine and would replay the backlog if the drain were broken, so Escape
    // exercises the same bug path the report describes.)
    if !harness::send_escape() {
        eprintln!(
            "  [{}] WARNING: Pico Escape failed — falling back to PostMessage Enter",
            label
        );
        harness::dismiss_save_dialog_pub();
    }

    let mut prev = client.state().tick_count;
    let mut max_bucket = 0u32;
    for _ in 0..30 {
        thread::sleep(Duration::from_millis(100));
        let tc = client.state().tick_count;
        max_bucket = max_bucket.max(tc.wrapping_sub(prev));
        prev = tc;
    }
    let rate = f64::from(max_bucket) * 10.0;
    let ok = rate <= MAX_TICKS_PER_SEC;
    println!(
        "  [{}] post-dismiss max rate {:.0} ticks/sec (ceiling {:.0}) -> {}",
        label,
        rate,
        MAX_TICKS_PER_SEC,
        if ok { "OK" } else { "BURST" }
    );
    (rate, ok)
}

fn run_keys(keys: &str, delay_ms: u32) -> bool {
    // Resolve relative to the exe (target/release/tas_test.exe -> repo root)
    // instead of a hard-coded checkout path, and REPORT failure: a silently
    // failed quit sequence turns Phase C into measuring the wrong screen.
    let script = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.join(r"..\..\..\TAS\tools\keys.ps1")))
        .filter(|p| p.exists())
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| r"TAS\tools\keys.ps1".to_string());
    match Command::new("powershell")
        .args([
            "-NoProfile",
            "-File",
            &script,
            "-Keys",
            keys,
            "-DelayMs",
            &delay_ms.to_string(),
        ])
        .output()
    {
        Ok(o) if o.status.success() => true,
        Ok(o) => {
            eprintln!("  WARNING: keys.ps1 exited {} for {:?}", o.status, keys);
            false
        }
        Err(e) => {
            eprintln!("  WARNING: keys.ps1 failed to run: {}", e);
            false
        }
    }
}

pub fn run() -> bool {
    let Some(path) = locate_recording() else {
        eprintln!("ERROR: couldn't locate {}", RECORDING_REL);
        return false;
    };
    println!("=== Save-dialog + menu-speed END TO END: {} ===", path);

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(200));

    let loaded = match replay::load_tasrec(std::path::Path::new(&path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    println!("  {} ticks loaded", loaded.count);
    harness::focus_game();

    // ---------------- PHASE A: PLAY finish + real-key dismiss ----------------
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

    // ---------------- PHASE B: CONT -> REC rides through the finish ----------
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

    // ---------------- PHASE C: the REAL main menu ----------------------------
    println!("--- PHASE C: quit to the main menu, measure what the user sees ---");
    // Pause menu: DOWN x4 = Return To Menu; "Are you sure?" = LEFT then ENTER.
    if !run_keys("ESC,DOWN,DOWN,DOWN,DOWN,ENTER,LEFT,ENTER", 700) {
        eprintln!(
            "ERROR: quit-to-menu key sequence failed; Phase C would measure the wrong screen"
        );
        return false;
    }
    thread::sleep(Duration::from_secs(3));

    // The user-visible thing itself: the menu video's on-screen motion.
    println!("  menu video (screen sampler):");
    let pass_c_video = video_rate::run_region(Some(6), None);

    // After >9s at the menu any stale suppress flag must have been retired by
    // the level-scan worker — and a CONT that completed normally never leaves
    // it set at all. A set flag here = dead keyboard.
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
