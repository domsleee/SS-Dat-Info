//! Catch-up SPEED regression test (guards "64× CONT collapsed to native").
//!
//! Background: pressing CONT splices live REC in at frame N. The engine first
//! REPLAYS frames 0..N as fast as it can (the "catch-up"), then hands control
//! to live REC at the splice. The whole point of the speed setting is that the
//! catch-up is FAST — a regression where 64× collapsed back to ~native speed
//! made CONT painful to use, and NO existing test caught it (the spatial-drift
//! and bucket-match suites only assert *where* the player ends up, never *how
//! fast* the replay got there).
//!
//! Why this is measured, not derived: cave2 advances `playback_pos` by exactly
//! 1 per Supreme::Cycle (cave2.hpp:507) and cave5 scales how many physics ticks
//! run *inside* each cycle (cave5.hpp). Whether a higher speed therefore reaches
//! a given `playback_pos` in less WALL time is not obvious from the code — it
//! depends on the render/cycle rate ceiling. So the test measures the real
//! median time-to-splice at 1× and 64× and asserts on the RATIO, which is
//! immune to the harness's ~2× wall-clock environment (both speeds are inflated
//! by the same factor, so it cancels). Absolute wall time is NOT asserted.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
/// Stop timing just below SPLICE_FRAME: at the splice cave2 flips PLAY→REC and
/// playback_pos stops advancing, so waiting for it to reach SPLICE_FRAME hangs.
const VERIFY_FRAMES: u32 = 2199;
/// Trials per speed; the median rejects the occasional slow restart/scheduler
/// hiccup so the ratio is stable. 3 is enough for a median: the 1x leg measured
/// ±3% across 5 trials (21.98-21.99s), and the gate has ~4x headroom (32.6x
/// measured vs an 8x floor), so the extra 2 trials cost ~44s of the lane
/// without moving the verdict.
const TRIALS: u32 = 3;
const RESTART_TIMEOUT_SECS: u64 = 15;
const SPLICE_TIMEOUT_SECS: u64 = 120;
/// The 1× and 64× speeds whose time-to-splice ratio is the catch-up speedup.
const SLOW_SPEED: f32 = 1.0;
const FAST_SPEED: f32 = 64.0;
/// Minimum acceptable T_1x / T_64x. Measured 32.8× on the good 1d32308 baseline
/// (T_1x=21.99s, T_64x=0.67s, ±3% across 5 trials), so an 8× floor leaves ~4×
/// headroom for machine load / scheduler variance while a collapse back toward
/// native (ratio→1, the regression this guards) fails hard. The ratio is taken
/// to the SAME splice frame, which lands at a bit-identical position at both
/// speeds, so speed changes only time-to-splice — the ratio is the speedup.
const CATCHUP_MIN_RATIO: f64 = 8.0;

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

/// Send ARM_CONTINUE and time how long the catch-up takes to reach
/// VERIFY_FRAMES. Returns (wall_secs, end_coord) or None if it never got there.
fn time_to_splice(client: &mut tas_shared::TasSharedMemoryClient) -> Option<(f64, [f32; 3])> {
    // A previous splice leaves playback_pos at the splice frame; clear it so the
    // poll loop doesn't read a stale "already there" value and return 0s.
    client.state_mut().playback_pos = 0;
    let start = Instant::now();
    client.send_command(TasCommand::ArmContinue);

    // Wait for cave2 to actually enter PLAY before polling progress.
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
        if highest >= VERIFY_FRAMES {
            let wall = start.elapsed().as_secs_f64();
            let end = client.state().play_coords[(VERIFY_FRAMES - 1) as usize];
            return Some((wall, end));
        }
        let mode = client.mode_volatile();
        if mode == TasMode::Rec as u32 || mode == TasMode::Off as u32 {
            // Mode flipped before we observed VERIFY_FRAMES — missed it.
            return None;
        }
        if start.elapsed() > Duration::from_secs(SPLICE_TIMEOUT_SECS) {
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

struct SpeedTiming {
    speed: f32,
    median_secs: f64,
    samples: usize,
    end_coord: Option<[f32; 3]>,
}

fn measure_speed(
    client: &mut tas_shared::TasSharedMemoryClient,
    rec: &replay::LoadedRecording,
    speed: f32,
) -> SpeedTiming {
    println!("\n  --- {speed:.0}× ---");
    let mut samples: Vec<f64> = Vec::with_capacity(TRIALS as usize);
    let mut end_coord = None;
    for t in 0..TRIALS {
        client.state_mut().playback_speed = speed;
        replay::write_to_shared(client, rec);
        if !restart(client) {
            println!("    trial {}: restart timed out — skipped", t + 1);
            harness::stop(client);
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        // CMD_RESTART zeros continue_from_frame and we re-assert speed in case
        // restart touched it; tas_ui (which clobbers speed each frame) was
        // already killed by ensure_exclusive_runtime_ownership.
        client.state_mut().continue_from_frame = SPLICE_FRAME;
        client.state_mut().playback_speed = speed;
        match time_to_splice(client) {
            Some((w, end)) => {
                println!(
                    "    trial {}: {:.3}s  end=({:.3},{:.3},{:.3})",
                    t + 1,
                    w,
                    end[0],
                    end[1],
                    end[2]
                );
                samples.push(w);
                end_coord = Some(end);
            }
            None => println!(
                "    trial {}: did not reach splice frame {}",
                t + 1,
                VERIFY_FRAMES
            ),
        }
        harness::stop(client);
        thread::sleep(Duration::from_millis(200));
    }
    let median_secs = median(&mut samples);
    println!(
        "    median = {:.3}s  ({} samples)",
        median_secs,
        samples.len()
    );
    SpeedTiming {
        speed,
        median_secs,
        samples: samples.len(),
        end_coord,
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
        "=== CATCH-UP SPEED test (median T_{:.0}x / T_{:.0}x to splice frame {}) ===",
        SLOW_SPEED, FAST_SPEED, VERIFY_FRAMES
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

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    // tas_ui writes playback_speed=1.0 every frame from its own state, which
    // would collapse the 64× run back to 1×. Take exclusive ownership first.
    harness::ensure_exclusive_runtime_ownership(&mut client, "catchup-speed scaling");

    let slow = measure_speed(&mut client, &rec, SLOW_SPEED);
    let fast = measure_speed(&mut client, &rec, FAST_SPEED);

    // Restore a sane speed for whatever runs next / the user.
    client.state_mut().playback_speed = 1.0;

    println!("\n=== CATCH-UP SPEED SUMMARY ===");
    println!(
        "  T_{:.0}x = {:.3}s ({} samples)",
        slow.speed, slow.median_secs, slow.samples
    );
    println!(
        "  T_{:.0}x = {:.3}s ({} samples)",
        fast.speed, fast.median_secs, fast.samples
    );

    // Cross-speed sim-invariance: the splice should land at the SAME physical
    // point regardless of speed (speed must change only how fast we get there,
    // not where). Informational — flags if 64× is silently skipping sim.
    if let (Some(a), Some(b)) = (slow.end_coord, fast.end_coord) {
        let d = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
        println!(
            "  splice end-coord delta 1x vs 64x = {:.4}  (1x=({:.3},{:.3},{:.3}) 64x=({:.3},{:.3},{:.3}))",
            d, a[0], a[1], a[2], b[0], b[1], b[2]
        );
    }

    if slow.samples == 0 || fast.samples == 0 {
        println!("*** CATCH-UP SPEED INCONCLUSIVE: a speed produced no samples — splice path not working. ***");
        return false;
    }
    if fast.median_secs <= 0.0 {
        println!("*** CATCH-UP SPEED INCONCLUSIVE: 64× median is zero — timing resolution too coarse. ***");
        return false;
    }

    let ratio = slow.median_secs / fast.median_secs;
    println!(
        "  effective catch-up = {:.2}× faster than 1× (floor {:.1}×)",
        ratio, CATCHUP_MIN_RATIO
    );

    if ratio >= CATCHUP_MIN_RATIO {
        println!(
            "*** CATCH-UP SPEED OK: 64× catch-up is {:.1}× faster than 1× (>= {:.1}×). ***",
            ratio, CATCHUP_MIN_RATIO
        );
        true
    } else {
        println!(
            "*** CATCH-UP SPEED FAILED: 64× catch-up is only {:.1}× faster than 1× (< {:.1}×) — \
             the speed scaling collapsed back toward native. CONT will feel slow. ***",
            ratio, CATCHUP_MIN_RATIO
        );
        false
    }
}
