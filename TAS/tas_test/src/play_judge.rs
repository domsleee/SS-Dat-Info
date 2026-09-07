//! Judged-PLAY speed handover: does replaying the countdown fast change the
//! run, and does the handover land where it was asked to?
//!
//! A bucket-matched PLAY replays the spawn countdown only so the judge can see
//! which frame the boarder leaves the spawn on. Nothing in that stretch moves,
//! and at 1x it is ~2.6s — paid on the attempt that succeeds AND on every
//! reroll. So the countdown now replays at a catch-up speed and the DLL hands
//! back at `first_moving + 1`.
//!
//! That trades one risk for the win: the replayed prefix is now produced under a
//! different tick batching regime than the run the recording was made in. CONT
//! has judged during a 256x catch-up for a long time and still splices
//! bit-exact, which is the reason to believe this is safe — but CONT hands over
//! at a splice the DLL owns, and this is a different handover point, so it needs
//! its own evidence.
//!
//! So this runs the SAME judged PLAY at 1x (no handover) and at the fast speeds,
//! and compares what actually matters:
//!
//!   * accepted first_moving      — the fast path must land the same bucket
//!   * prefix drift               — the accepted replay must track the recording
//!     no worse than the 1x one does
//!   * handover position          — must be first_moving + 1, not a batch late
//!   * speed after the handover   — must be the resume speed, and must STAY
//!     there (the controller and the UI both re-assert catch-up speeds on a timer)
//!   * time to first movement     — the reason the feature exists
//!
//! Failure here means the handover is not exact, or judging at speed changed
//! the physics — both of which are reasons not to ship it.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::transport::{Arm, ArmConfig, BucketTarget, StepOutcome, TransportController};
use tas_shared::{TasCommand, TasSharedMemoryClient};

use crate::harness;
use crate::replay;

const RECORDING: &str = "FE-10065.tasrec";
/// Catch-up speeds to compare. 1.0 is the control (no handover at all).
const SPEEDS: &[f32] = &[1.0, 64.0];
const MAX_RETRIES: u32 = 30;
/// The handover must land ON the tick it was staged for. cave5 caps the
/// catch-up batch to do exactly that, so anything above 0 is that cap failing.
/// 1 tick of slack for a poll that samples the position a tick late.
const MAX_HANDOVER_OVERSHOOT: u32 = 1;
/// Per-attempt budget: restart handshake plus the replay to the judge point.
const ATTEMPT_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Clone)]
struct Run {
    retries: u32,
    /// Replay position when the DLL cleared the staged handover (None at 1x).
    handover_pos: Option<u32>,
    /// Playback speed sampled right after the handover fired.
    speed_at_handover: Option<f32>,
    /// Playback speed at the end of the cycle — catches either re-asserter
    /// dragging the run back up to the catch-up speed after the handover.
    speed_at_done: f32,
    /// Wall time from the cycle starting to the boarder actually moving.
    ms_to_movement: f64,
    ms_to_accept: f64,
    /// Max per-axis |play - rec| over the replayed prefix at accept time.
    prefix_drift: f32,
    observed_first_moving: Option<u32>,
}

pub fn run(iterations: u32) -> bool {
    let path = match harness::fixture_path(RECORDING) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    println!("=== Judged PLAY speed handover: {} ===", path.display());

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    let loaded = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    if loaded.count == 0 || loaded.rec_coords.is_empty() {
        eprintln!("ERROR: recording is empty");
        return false;
    }
    replay::write_to_shared(&mut client, &loaded);

    let start = loaded.rec_coords[0];
    let expected_start_bits = [start[0].to_bits(), start[1].to_bits(), start[2].to_bits()];
    let expected_fm = {
        let s = client.state();
        tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count)
    };
    let fm = match expected_fm {
        Some(f) => f,
        None => {
            eprintln!("ERROR: the recording never moves — nothing to judge");
            return false;
        }
    };
    println!("  {} ticks, first moving frame {}", loaded.count, fm);
    harness::focus_game();

    let mut all_ok = true;
    let mut by_speed: Vec<(f32, Vec<Run>)> = Vec::new();
    for &speed in SPEEDS {
        let runs = run_at(&mut client, speed, expected_start_bits, fm, iterations);
        all_ok &= report(speed, fm, &runs, iterations);
        by_speed.push((speed, runs));
    }

    // Cross-speed comparison: the whole point is that the fast path lands the
    // SAME bucket and tracks the recording no worse than the control.
    println!("\n--- 1x vs fast ---");
    let control = by_speed.iter().find(|(s, _)| *s == 1.0).map(|(_, r)| r);
    if let Some(control) = control {
        let control_drift = max_drift(control);
        let control_move = median(&control.iter().map(|r| r.ms_to_movement).collect::<Vec<_>>());
        for (speed, runs) in by_speed.iter().filter(|(s, _)| *s != 1.0) {
            let d = max_drift(runs);
            let m = median(&runs.iter().map(|r| r.ms_to_movement).collect::<Vec<_>>());
            println!(
                "  {}x: max prefix drift {:.4} (1x: {:.4}), median time-to-movement {:.0} ms (1x: {:.0} ms)",
                speed, d, control_drift, m, control_move
            );
            // Drift is compared against the control, not an absolute epsilon:
            // the landed bucket legitimately blips during the settle. The fast
            // path must not be worse than the slow one by more than that blip.
            if d > control_drift + tas_shared::cont::BUCKET_MATCH_EPSILON {
                println!("  FAIL: {}x drifts further than 1x does", speed);
                all_ok = false;
            }
            if m >= control_move {
                println!("  FAIL: {}x is not faster to first movement than 1x", speed);
                all_ok = false;
            }
        }
    }

    println!();
    if all_ok {
        println!("*** PLAY JUDGE HANDOVER PASSED ***");
    } else {
        println!("*** PLAY JUDGE HANDOVER FAILED (see lines above) ***");
    }
    harness::stop(&mut client);
    all_ok
}

fn run_at(
    client: &mut TasSharedMemoryClient,
    speed: f32,
    expected_start_bits: [u32; 3],
    fm: u32,
    iterations: u32,
) -> Vec<Run> {
    println!("\n=== {}x catch-up ({} runs) ===", speed, iterations);
    let mut runs = Vec::new();
    for i in 1..=iterations {
        match one_cycle(client, speed, expected_start_bits, fm) {
            Some(r) => {
                println!(
                    "  run {}: retries={} handover@{:?} (want {}) speed {:?}->{} move {:.0} ms accept {:.0} ms drift {:.4} fm={:?}",
                    i, r.retries, r.handover_pos, fm + 1, r.speed_at_handover,
                    r.speed_at_done, r.ms_to_movement, r.ms_to_accept,
                    r.prefix_drift, r.observed_first_moving
                );
                runs.push(r);
            }
            None => println!("  run {}: aborted / timed out", i),
        }
        harness::stop(client);
        thread::sleep(Duration::from_millis(150));
    }
    runs
}

fn one_cycle(
    client: &mut TasSharedMemoryClient,
    speed: f32,
    expected_start_bits: [u32; 3],
    fm: u32,
) -> Option<Run> {
    // 1x is the control: no handover staged at all, so the run reproduces the
    // pre-handover behaviour exactly.
    let resume_speed = if speed > 1.0 { 1.0 } else { 0.0 };
    client.state_mut().playback_speed = speed;
    let cfg = ArmConfig {
        arm: Arm::Play,
        catchup_speed: speed,
        continue_from_frame: 0,
        // This command deliberately exercises the retired bucket-judge path as
        // a regression control, not the product's gate-aligned PLAY path.
        gate_align_rec: 0,
        target: Some(BucketTarget {
            expected_start_bits,
            expected_first_moving: Some(fm),
        }),
        max_retries: MAX_RETRIES,
        resume_speed,
        predict_bucket: true,
    };
    let mut c = TransportController::new(cfg);

    let t0 = Instant::now();
    let mut handover_pos = None;
    let mut speed_at_handover = None;
    let mut ms_to_movement = f64::NAN;
    let mut staged = false;
    // playback_pos still holds the PREVIOUS replay's final value until this
    // cycle's arm resets it, so "has the replay passed fm" must first watch it
    // come back down.
    let mut replay_started = false;
    let mut deadline = Instant::now() + Duration::from_secs(ATTEMPT_TIMEOUT_SECS);
    let mut retries = 0;

    loop {
        let outcome = c.step(client);

        // Sample the handover between steps. `speed_handoff_pos` going non-zero
        // then back to zero is the DLL firing it; catching the position and the
        // speed right there is the only way to tell an exact handover from one
        // that arrived a whole catch-up batch late.
        {
            let s = client.state();
            if s.speed_handoff_pos != 0 {
                staged = true;
            } else if staged && handover_pos.is_none() {
                handover_pos = Some(s.playback_pos);
                speed_at_handover = Some(s.playback_speed);
                ms_to_movement = t0.elapsed().as_secs_f64() * 1000.0;
            }
        }
        // At 1x nothing is staged, so time-to-movement is just when the replay
        // passes the recording's first moving frame.
        if resume_speed == 0.0 {
            let pos = client.playback_pos_volatile();
            if pos <= fm {
                replay_started = true;
            } else if replay_started && ms_to_movement.is_nan() {
                ms_to_movement = t0.elapsed().as_secs_f64() * 1000.0;
            }
        }

        match outcome {
            StepOutcome::InProgress => {
                if Instant::now() > deadline {
                    eprintln!("    stalled");
                    client.send_command(TasCommand::Stop);
                    return None;
                }
                thread::sleep(Duration::from_millis(5));
            }
            StepOutcome::Wait { ms } => thread::sleep(Duration::from_millis(ms)),
            StepOutcome::Reroll {
                attempt,
                suggested_delay_ms,
                ..
            } => {
                retries = attempt;
                // A reroll re-stages its own handover, so reset the sampler with
                // it — otherwise the next attempt's handover reads as "already
                // fired" and the position recorded would be the failed one's.
                staged = false;
                replay_started = false;
                handover_pos = None;
                speed_at_handover = None;
                ms_to_movement = f64::NAN;
                thread::sleep(Duration::from_millis(suggested_delay_ms));
                deadline = Instant::now() + Duration::from_secs(ATTEMPT_TIMEOUT_SECS);
            }
            StepOutcome::Done { retries_used, .. } => {
                let ms_to_accept = t0.elapsed().as_secs_f64() * 1000.0;
                let s = client.state();
                let end = (s.playback_pos as usize)
                    .min(s.play_coords.len())
                    .min(s.rec_coords.len())
                    .min(s.recorded_count as usize);
                let mut drift = 0.0f32;
                for k in 0..end {
                    let p = s.play_coords[k];
                    let r = s.rec_coords[k];
                    drift = drift
                        .max((p[0] - r[0]).abs())
                        .max((p[1] - r[1]).abs())
                        .max((p[2] - r[2]).abs());
                }
                let observed_first_moving =
                    tas_shared::cont::detect_first_moving(&s.play_coords[..], s.playback_pos);
                let speed_at_done = s.playback_speed;
                return Some(Run {
                    retries: retries.max(retries_used),
                    handover_pos,
                    speed_at_handover,
                    speed_at_done,
                    ms_to_movement,
                    ms_to_accept,
                    prefix_drift: drift,
                    observed_first_moving,
                });
            }
            StepOutcome::Aborted { reason } => {
                eprintln!("    aborted: {}", reason);
                return None;
            }
        }
    }
}

fn report(speed: f32, fm: u32, runs: &[Run], iterations: u32) -> bool {
    if runs.len() < iterations as usize {
        println!(
            "  {}x FAILED: only {}/{} runs completed",
            speed,
            runs.len(),
            iterations
        );
        return false;
    }
    let mut ok = true;

    // The bucket itself: every accepted replay must have left the spawn on the
    // recording's frame. This is the judge's own criterion, re-checked here
    // against the final state so a judge that accepted at speed and a replay
    // that then went somewhere else can't both hide.
    for r in runs {
        if r.observed_first_moving != Some(fm) {
            println!(
                "  {}x FAILED: accepted a replay whose first-moving is {:?}, not {}",
                speed, r.observed_first_moving, fm
            );
            ok = false;
        }
    }

    if speed > 1.0 {
        for r in runs {
            match r.handover_pos {
                None => {
                    println!("  {}x FAILED: no handover fired", speed);
                    ok = false;
                }
                Some(pos) => {
                    let overshoot = pos.saturating_sub(fm + 1);
                    if overshoot > MAX_HANDOVER_OVERSHOOT {
                        println!(
                            "  {}x FAILED: handover fired at {} — {} ticks past {} (cave5 batch cap not landing on it)",
                            speed, pos, overshoot, fm + 1
                        );
                        ok = false;
                    }
                }
            }
            if let Some(s) = r.speed_at_handover {
                if (s - 1.0).abs() > 0.001 {
                    println!("  {}x FAILED: speed after handover is {}, not 1", speed, s);
                    ok = false;
                }
            }
            // The controller re-asserts the catch-up speed on every step and the
            // UI syncs it every frame. If either one ignores the handover, the
            // run is back at catch-up speed by the time the cycle finishes.
            if (r.speed_at_done - 1.0).abs() > 0.001 {
                println!(
                    "  {}x FAILED: speed back at {} by Done — a re-asserter undid the handover",
                    speed, r.speed_at_done
                );
                ok = false;
            }
        }
    }

    let retries: Vec<f64> = runs.iter().map(|r| r.retries as f64).collect();
    println!(
        "  {}x: {} runs, mean retries {:.2}, median time-to-movement {:.0} ms, median accept {:.0} ms, max drift {:.4}",
        speed,
        runs.len(),
        retries.iter().sum::<f64>() / retries.len() as f64,
        median(&runs.iter().map(|r| r.ms_to_movement).collect::<Vec<_>>()),
        median(&runs.iter().map(|r| r.ms_to_accept).collect::<Vec<_>>()),
        max_drift(runs),
    );
    if ok {
        println!("  {}x PASSED", speed);
    }
    ok
}

fn max_drift(runs: &[Run]) -> f32 {
    runs.iter().map(|r| r.prefix_drift).fold(0.0, f32::max)
}

fn median(v: &[f64]) -> f64 {
    let mut v: Vec<f64> = v.iter().copied().filter(|x| !x.is_nan()).collect();
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}
