//! Playback-pace regression test (guards "PLAY skipped ahead / ran fast").
//!
//! Background: a regression made plain 1× PLAY run FASTER than real wall-clock
//! time (it "skipped ahead"). The spatial-drift suite never caught it — REC and
//! PLAY traced the same coordinates, just faster — so drift stayed zero while
//! the replay no longer matched the user's real-time experience.
//!
//! The observable: at 1× the game's tick clock paces physics to real time —
//! one render cycle advances one 0.01s physics tick, and `playback_pos` (which
//! advances once per cycle, cave2.hpp:507) tracks 1:1 with sim time. So PLAYing
//! N frames at 1× must take ~N×0.01s of WALL time. The catch-up probe confirmed
//! this empirically: 2199 frames replayed in 21.986s = 0.009998 s/frame, i.e.
//! exactly native. (The harness's ~2× wall-clock inflation is a REC-phase
//! artifact and does NOT touch 1× PLAY, which is clock-gated — so unlike most
//! timing here, absolute pace IS a sound assertion for plain PLAY.)
//!
//! A skip-ahead regression (more physics ticks per cycle at 1×, or the clock
//! gate removed) finishes the window in well under N×0.01s → ratio < LO. A
//! stall finishes too slowly → ratio > HI.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
/// Frames of PLAY to time. At native 0.01s/tick this is ~20s of wall time —
/// long enough to swamp per-trial startup jitter, short enough to keep the test
/// brief and well inside FE-tremendous's length.
const PACE_TARGET: u32 = 2000;
/// Native seconds per frame (1 render cycle = 1 physics tick = 0.01s at 1×).
const NATIVE_SECS_PER_FRAME: f64 = 0.01;
const TRIALS: u32 = 3;
const RESTART_TIMEOUT_SECS: u64 = 15;
const PLAY_TIMEOUT_SECS: u64 = 90;
/// Acceptable band for (measured wall) / (native expected). Below LO = ran
/// faster than real time (the skip-ahead regression); above HI = stalled. The
/// good baseline measures ~1.00; the band absorbs scheduler/render jitter while
/// a 2×+ skip-ahead (ratio ~0.5) fails decisively.
const PACE_LO: f64 = 0.85;
const PACE_HI: f64 = 1.30;

fn restart(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    let start = Instant::now();
    loop {
        if client.restart_state() == 2 {
            client.reset_restart_state();
            return true;
        }
        if start.elapsed() > Duration::from_secs(RESTART_TIMEOUT_SECS) {
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// Arm plain PLAY and time how long playback_pos takes to reach PACE_TARGET.
fn time_play_window(client: &mut tas_shared::TasSharedMemoryClient) -> Option<f64> {
    client.state_mut().playback_pos = 0;
    // Plain PLAY, not CONT — ensure no splice marker is set.
    client.state_mut().continue_from_frame = 0;
    let start = Instant::now();
    client.send_command(TasCommand::ArmPlay);

    let mode_wait = Instant::now();
    while client.mode_volatile() != TasMode::Play as u32 {
        if mode_wait.elapsed() > Duration::from_secs(2) {
            return None;
        }
        thread::sleep(Duration::from_millis(5));
    }

    let mut highest: u32 = 0;
    loop {
        let pos = client.playback_pos_volatile();
        if pos > highest {
            highest = pos;
        }
        if highest >= PACE_TARGET {
            return Some(start.elapsed().as_secs_f64());
        }
        // PLAY runs the whole recording then flips to OFF; if it ended before
        // our window the recording is shorter than PACE_TARGET (config error).
        if client.mode_volatile() == TasMode::Off as u32 {
            return None;
        }
        if start.elapsed() > Duration::from_secs(PLAY_TIMEOUT_SECS) {
            return None;
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.0
    }
}

fn locate_recording() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-tremendous.tasrec"),
    ];
    candidates.iter().find(|p| p.exists()).cloned()
}

pub fn run() -> bool {
    println!(
        "=== PLAY-PACE test (1× PLAY of {} frames must take ~native wall time) ===",
        PACE_TARGET
    );

    let path = match locate_recording() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: couldn't locate {}", RECORDING_REL);
            return false;
        }
    };
    println!("  Recording: {}", path.display());
    let rec = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: load failed: {}", e);
            return false;
        }
    };
    if (rec.count as u32) < PACE_TARGET {
        eprintln!(
            "ERROR: recording has {} frames, need >= {}",
            rec.count, PACE_TARGET
        );
        return false;
    }

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    // tas_ui writes playback_speed=1.0 each frame — that happens to be what we
    // want here, but take exclusive ownership anyway so nothing else perturbs
    // the run (and to match the other timing tests' preconditions).
    harness::ensure_exclusive_runtime_ownership(&mut client, "play-pace timing");

    let mut samples: Vec<f64> = Vec::with_capacity(TRIALS as usize);
    for t in 0..TRIALS {
        client.state_mut().playback_speed = 1.0;
        replay::write_to_shared(&mut client, &rec);
        if !restart(&mut client) {
            println!("  trial {}: restart timed out — skipped", t + 1);
            harness::stop(&mut client);
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        client.state_mut().playback_speed = 1.0;
        match time_play_window(&mut client) {
            Some(w) => {
                println!("  trial {}: {:.3}s for {} frames", t + 1, w, PACE_TARGET);
                samples.push(w);
            }
            None => println!("  trial {}: PLAY didn't reach frame {}", t + 1, PACE_TARGET),
        }
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(200));
    }

    if samples.is_empty() {
        println!("*** PLAY-PACE INCONCLUSIVE: no successful PLAY windows — playback path not working. ***");
        return false;
    }

    let measured = median(&mut samples);
    let expected = PACE_TARGET as f64 * NATIVE_SECS_PER_FRAME;
    let ratio = measured / expected;
    println!("\n=== PLAY-PACE SUMMARY ===");
    println!("  measured median = {:.3}s   native expected = {:.3}s   ratio = {:.3} (band [{:.2}, {:.2}])", measured, expected, ratio, PACE_LO, PACE_HI);

    if ratio < PACE_LO {
        println!(
            "*** PLAY-PACE FAILED: 1× PLAY ran {:.0}% of native time — it SKIPPED AHEAD (faster than real time). ***",
            ratio * 100.0
        );
        false
    } else if ratio > PACE_HI {
        println!(
            "*** PLAY-PACE FAILED: 1× PLAY took {:.0}% of native time — it STALLED (slower than real time). ***",
            ratio * 100.0
        );
        false
    } else {
        println!(
            "*** PLAY-PACE OK: 1× PLAY ran at {:.0}% of native wall time (within band). ***",
            ratio * 100.0
        );
        true
    }
}
