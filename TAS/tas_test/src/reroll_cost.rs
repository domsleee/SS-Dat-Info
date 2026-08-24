//! What does a reroll cost, when the bucket was already determined wrong?
//!
//! `play-judge` cannot answer this. On FE-10065 the bucket lands first or second
//! try, and a single reroll is always attempt ONE — the attempt that teaches the
//! controller the countdown length. The predictive path only starts working on
//! attempt two, so the measurement that matters never happens there.
//!
//! So this forces it: judge against a first-moving frame the restart CANNOT
//! produce (the recording's, moved well outside the countdown's ±1 rounding).
//! Every attempt is then a genuine reroll, attempt 1 teaches K, and attempts
//! 2..N are all determined-wrong before they replay anything.
//!
//! Run twice — prediction on and off — and the difference is the whole point of
//! the feature: does a reroll still have to replay ~300 ticks of a stationary
//! boarder to find out what was already knowable when it armed?
//!
//! Measured at 1x deliberately. A catch-up would hide the win behind the speed
//! handover, which is exactly the workaround this is meant to replace.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::transport::{Arm, ArmConfig, BucketTarget, StepOutcome, TransportController};
use tas_shared::{TasCommand, TasSharedMemoryClient};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-10065.tasrec";
/// How far to move the target away from what the restart can actually produce.
/// Well outside COUNTDOWN_K_JITTER, so no attempt can accidentally match and
/// end the run early.
const UNREACHABLE_OFFSET: u32 = 6;
/// Rerolls to force per arm of the comparison.
const REROLLS: u32 = 6;

struct Outcome {
    rerolls: u32,
    predictive: u32,
    elapsed_ms: f64,
}

pub fn run() -> bool {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-10065.tasrec"),
    ];
    let path = match candidates.iter().find(|p| p.exists()) {
        Some(p) => p.to_string_lossy().into_owned(),
        None => {
            eprintln!("ERROR: couldn't locate {}", RECORDING_REL);
            return false;
        }
    };
    println!("=== Cost of a determined-wrong reroll ===");

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    let loaded = match replay::load_tasrec(std::path::Path::new(&path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    let start = loaded.rec_coords[0];
    let start_bits = [start[0].to_bits(), start[1].to_bits(), start[2].to_bits()];
    let real_fm = {
        let s = client.state();
        match tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count) {
            Some(f) => f,
            None => {
                eprintln!("ERROR: recording never moves");
                return false;
            }
        }
    };
    // Unreachable on purpose: the countdown rounds by one tick, so a target this
    // far out can never be produced and every attempt is a real reroll.
    let target_fm = real_fm + UNREACHABLE_OFFSET;
    println!(
        "  recording first-moving {}, judging against {} (unreachable by {} ticks)",
        real_fm, target_fm, UNREACHABLE_OFFSET
    );
    harness::focus_game();

    let with = force_rerolls(&mut client, start_bits, target_fm, true);
    let without = force_rerolls(&mut client, start_bits, target_fm, false);
    harness::stop(&mut client);

    let (w, wo) = match (with, without) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            println!("\nboth arms must complete to compare");
            return false;
        }
    };

    let per_w = w.elapsed_ms / w.rerolls.max(1) as f64;
    let per_wo = wo.elapsed_ms / wo.rerolls.max(1) as f64;
    println!("\n=== RESULT ===");
    println!(
        "  prediction ON : {} rerolls ({} predictive) in {:.0} ms -> {:.0} ms each",
        w.rerolls, w.predictive, w.elapsed_ms, per_w
    );
    println!(
        "  prediction OFF: {} rerolls ({} predictive) in {:.0} ms -> {:.0} ms each",
        wo.rerolls, wo.predictive, wo.elapsed_ms, per_wo
    );

    let mut ok = true;
    // Attempt 1 always replays — it is what teaches K — so the most that can be
    // predicted is one fewer than the rerolls performed.
    if w.predictive == 0 {
        println!("  FAIL: prediction was on but never fired");
        ok = false;
    }
    if wo.predictive != 0 {
        println!("  FAIL: prediction fired while switched off");
        ok = false;
    }
    if per_w >= per_wo {
        println!("  FAIL: predicted rerolls are not cheaper than replayed ones");
        ok = false;
    }
    if ok {
        println!(
            "\n*** REROLL COST PASSED: {:.0} ms -> {:.0} ms per reroll ({:.1}x) ***",
            per_wo,
            per_w,
            per_wo / per_w.max(0.001)
        );
    } else {
        println!("\n*** REROLL COST FAILED ***");
    }
    ok
}

fn force_rerolls(
    client: &mut TasSharedMemoryClient,
    start_bits: [u32; 3],
    target_fm: u32,
    predict: bool,
) -> Option<Outcome> {
    println!(
        "\n--- prediction {} ---",
        if predict { "ON" } else { "OFF" }
    );
    client.state_mut().playback_speed = 1.0;
    let cfg = ArmConfig {
        arm: Arm::Play,
        catchup_speed: 1.0,
        continue_from_frame: 0,
        // Research command: preserve the old unaligned bucket-reroll behavior.
        gate_align_rec: 0,
        target: Some(BucketTarget {
            expected_start_bits: start_bits,
            expected_first_moving: Some(target_fm),
        }),
        max_retries: REROLLS,
        resume_speed: 0.0,
        predict_bucket: predict,
    };
    let mut c = TransportController::new(cfg);

    let t0 = Instant::now();
    let mut rerolls = 0u32;
    let deadline = Instant::now() + Duration::from_secs(180);
    loop {
        if Instant::now() > deadline {
            eprintln!("    timed out in {}", c.phase_name());
            client.send_command(TasCommand::Stop);
            return None;
        }
        match c.step(client) {
            StepOutcome::InProgress => thread::sleep(Duration::from_millis(3)),
            StepOutcome::Wait { ms } => thread::sleep(Duration::from_millis(ms)),
            StepOutcome::Reroll {
                attempt,
                suggested_delay_ms,
                ..
            } => {
                rerolls = attempt;
                println!(
                    "    reroll {} at {:.0} ms (predictive so far: {})",
                    attempt,
                    t0.elapsed().as_secs_f64() * 1000.0,
                    c.predictive_rejects()
                );
                thread::sleep(Duration::from_millis(suggested_delay_ms));
            }
            // Exhausting the retries is the expected end: the target cannot be hit.
            StepOutcome::Aborted { .. } => {
                let o = Outcome {
                    rerolls: rerolls.max(1),
                    predictive: c.predictive_rejects(),
                    elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
                };
                client.send_command(TasCommand::Stop);
                thread::sleep(Duration::from_millis(150));
                return Some(o);
            }
            StepOutcome::Done { .. } => {
                eprintln!("    unexpectedly matched an unreachable target");
                client.send_command(TasCommand::Stop);
                return None;
            }
        }
    }
}
