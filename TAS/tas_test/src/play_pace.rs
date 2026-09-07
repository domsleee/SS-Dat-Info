//! Playback-pace regression test: plain 1x PLAY must run at real time.
//!
//! The spatial-drift suite cannot see a PLAY that skips ahead: REC and PLAY
//! trace the same coordinates, just faster. At 1x one render cycle advances
//! one 0.01 s physics tick and `playback_pos` tracks 1:1 with sim time, so
//! PLAYing N frames must take ~N x 0.01 s of wall time. 1x PLAY is
//! clock-gated, so unlike most timing here the absolute pace is a sound
//! assertion. Too fast (ratio < LO) is the skip-ahead regression; too slow
//! (ratio > HI) is a stall.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING: &str = "FE-tremendous.tasrec";
/// Frames of PLAY to time: ~20 s of wall time, enough to swamp per-trial
/// startup jitter and well inside FE-tremendous's length.
const PACE_TARGET: u32 = 2000;
/// Native seconds per frame (1 render cycle = 1 physics tick = 0.01s at 1x).
const NATIVE_SECS_PER_FRAME: f64 = 0.01;
const TRIALS: u32 = 3;
const PLAY_TIMEOUT_SECS: u64 = 90;
/// Acceptable band for (measured wall) / (native expected). The good baseline
/// measures ~1.00; the band absorbs scheduler/render jitter while a 2x+
/// skip-ahead (ratio ~0.5) fails decisively.
const PACE_LO: f64 = 0.85;
const PACE_HI: f64 = 1.30;

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

pub fn run() -> bool {
    println!(
        "=== PLAY-PACE test (1× PLAY of {} frames must take ~native wall time) ===",
        PACE_TARGET
    );

    let path = match harness::fixture_path(RECORDING) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
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
    harness::ensure_exclusive_runtime_ownership(&mut client, "play-pace timing");

    let mut samples: Vec<f64> = Vec::with_capacity(TRIALS as usize);
    for t in 0..TRIALS {
        client.state_mut().playback_speed = 1.0;
        replay::write_to_shared(&mut client, &rec);
        if !harness::restart_inprocess(&mut client) {
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
