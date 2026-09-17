//! Catch-up SPEED regression test: the CONT catch-up at 64x must stay much
//! faster than 1x.
//!
//! CONT replays frames 0..N as fast as it can before handing over to live REC.
//! The drift and bucket-match suites only assert WHERE the player ends up, so
//! a catch-up that collapses back to native speed passes them. Whether a higher
//! speed reaches a given `playback_pos` in less wall time depends on the
//! render/cycle rate ceiling, so it is measured: median time-to-splice at 1x
//! and 64x, asserted as a RATIO, which cancels the harness's wall-clock
//! environment. Absolute wall time is not asserted.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING: &str = "FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
/// Stop timing just below SPLICE_FRAME: at the splice cave2 flips PLAY→REC and
/// playback_pos stops advancing, so waiting for it to reach SPLICE_FRAME hangs.
const VERIFY_FRAMES: u32 = 2199;
/// Trials per speed; the median rejects the occasional slow restart. The 1x
/// leg varies about 3% and the gate has ~4x headroom, so 3 is enough.
const TRIALS: u32 = 3;
const SPLICE_TIMEOUT_SECS: u64 = 120;
/// The 1x and 64x speeds whose time-to-splice ratio is the catch-up speedup.
const SLOW_SPEED: f32 = 1.0;
const FAST_SPEED: f32 = 64.0;
/// Minimum acceptable T_1x / T_64x. A healthy build measures ~30x, so 8x
/// leaves headroom for machine load while a collapse toward native (ratio
/// near 1) fails hard. The splice lands at a bit-identical position at both
/// speeds, so speed changes only time-to-splice.
const CATCHUP_MIN_RATIO: f64 = 8.0;

/// Send ARM_CONTINUE and time how long the catch-up takes to reach
/// VERIFY_FRAMES. Returns (wall_secs, end_coord) or None if it never got there.
fn time_to_splice(client: &mut tas_shared::TasSharedMemoryClient) -> Option<(f64, [f32; 3])> {
    // An earlier splice leaves playback_pos at the splice frame, which would
    // read as "already there".
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
        if !harness::restart_inprocess(client) {
            println!("    trial {}: restart timed out — skipped", t + 1);
            harness::stop(client);
            thread::sleep(Duration::from_millis(200));
            continue;
        }
        // CMD_RESTART zeros continue_from_frame and may touch the speed.
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
    let median_secs =
        crate::timing::complete_median(&mut samples, TRIALS as usize).unwrap_or(f64::NAN);
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

pub fn run() -> bool {
    println!(
        "=== CATCH-UP SPEED test (median T_{:.0}x / T_{:.0}x to splice frame {}) ===",
        SLOW_SPEED, FAST_SPEED, VERIFY_FRAMES
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

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    // tas_ui writes playback_speed=1.0 every frame, which would collapse the
    // 64x run back to 1x.
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

    // Informational: the splice should land at the same point at both speeds.
    if let (Some(a), Some(b)) = (slow.end_coord, fast.end_coord) {
        let d = ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt();
        println!(
            "  splice end-coord delta 1x vs 64x = {:.4}  (1x=({:.3},{:.3},{:.3}) 64x=({:.3},{:.3},{:.3}))",
            d, a[0], a[1], a[2], b[0], b[1], b[2]
        );
    }

    if slow.samples != TRIALS as usize || fast.samples != TRIALS as usize {
        println!("*** CATCH-UP SPEED INCONCLUSIVE: not all planned trials completed — splice path not working. ***");
        return false;
    }
    if !slow.median_secs.is_finite() || !fast.median_secs.is_finite() || fast.median_secs <= 0.0 {
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
