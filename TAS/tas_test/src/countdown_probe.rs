//! countdown-probe: what is the spawn countdown actually counting?
//!
//! `bucket-predict` established that the bucket (which tick the boarder leaves
//! spawn on) is not determined by the spawn state, nor by the arm phase measured
//! to exact tick granularity. The information is sub-tick. Before instrumenting
//! the DLL to chase it, this asks a cheaper question that decides WHERE to look:
//!
//!     Is the countdown a fixed number of TICKS, or a fixed amount of WALL TIME?
//!
//! Those have different consequences:
//!
//!   * fixed TICKS  -> first_moving should be constant, and the variation must
//!                     come from where the recording starts relative to it.
//!   * fixed WALL   -> the tick it lands on depends on where the tick boundaries
//!                     fall inside that interval, i.e. on the sub-tick phase.
//!                     The observed 261/262/263 spread is then a rounding
//!                     artifact, and the phase is genuinely the thing to capture.
//!
//! Method: ignore REC entirely and watch the LIVE player position, which the DLL
//! publishes every frame regardless of mode. Restart, note the spawn bits, then
//! poll until the position changes bit-exactly, recording both wall-clock ms and
//! ticks elapsed. Whichever of the two is stable across restarts is what the
//! countdown is really counting.
//!
//! Polling resolution is one frame (~6-10ms), which is far too coarse to see a
//! sub-tick phase — but it does not need to. Distinguishing "constant ms, varying
//! ticks" from "constant ticks, varying ms" only needs single-tick resolution.

use std::thread;
use std::time::{Duration, Instant};

use crate::harness;

/// Give up on a restart that never moves — something is wrong with the run.
const MOVE_TIMEOUT_SECS: u64 = 15;

struct Sample {
    iter: u32,
    wall_ms: u128,
    ticks: u32,
    frames: u32,
}

pub fn run(iterations: u32) -> bool {
    println!("=== COUNTDOWN-PROBE: ticks or wall time? ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut samples: Vec<Sample> = Vec::new();

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed — skipping", iter);
            continue;
        }

        let spawn = {
            let s = client.state();
            [
                s.player_x.to_bits(),
                s.player_y.to_bits(),
                s.player_z.to_bits(),
            ]
        };
        let t0 = Instant::now();
        let tick0 = client.tick_count_volatile();
        let frame0 = client.frame_count_volatile();

        let deadline = Instant::now() + Duration::from_secs(MOVE_TIMEOUT_SECS);
        let mut moved = false;
        while Instant::now() < deadline {
            let s = client.state();
            let now = [
                s.player_x.to_bits(),
                s.player_y.to_bits(),
                s.player_z.to_bits(),
            ];
            if now != spawn {
                moved = true;
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
        if !moved {
            eprintln!("  iter {}: never moved within {}s", iter, MOVE_TIMEOUT_SECS);
            continue;
        }

        let wall_ms = t0.elapsed().as_millis();
        let ticks = client.tick_count_volatile().wrapping_sub(tick0);
        let frames = client.frame_count_volatile().wrapping_sub(frame0);
        println!(
            "  iter {:>2}: wall={:>5}ms  ticks={:<5} frames={:<5}  ({:.2} ticks/frame)",
            iter,
            wall_ms,
            ticks,
            frames,
            if frames > 0 {
                ticks as f64 / frames as f64
            } else {
                0.0
            }
        );
        samples.push(Sample {
            iter,
            wall_ms,
            ticks,
            frames,
        });
    }

    analyze(&samples)
}

fn spread<T: Copy + Into<f64>>(vals: &[T]) -> (f64, f64, f64) {
    let v: Vec<f64> = vals.iter().map(|x| (*x).into()).collect();
    let min = v.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = v.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    (min, max, mean)
}

fn analyze(samples: &[Sample]) -> bool {
    println!("\n========== ANALYSIS ==========");
    if samples.len() < 4 {
        println!("  Not enough samples to conclude.");
        return false;
    }
    let _ = samples[0].iter;

    let wall: Vec<f64> = samples.iter().map(|s| s.wall_ms as f64).collect();
    let ticks: Vec<f64> = samples.iter().map(|s| s.ticks as f64).collect();
    let frames: Vec<f64> = samples.iter().map(|s| s.frames as f64).collect();

    let (wmin, wmax, wmean) = spread(&wall);
    let (tmin, tmax, tmean) = spread(&ticks);
    let (fmin, fmax, fmean) = spread(&frames);

    println!("  samples: {}", samples.len());
    println!(
        "  wall ms : min {:.0}  max {:.0}  mean {:.1}  spread {:.0}ms",
        wmin,
        wmax,
        wmean,
        wmax - wmin
    );
    println!(
        "  ticks   : min {:.0}  max {:.0}  mean {:.1}  spread {:.0}",
        tmin,
        tmax,
        tmean,
        tmax - tmin
    );
    println!(
        "  frames  : min {:.0}  max {:.0}  mean {:.1}  spread {:.0}",
        fmin,
        fmax,
        fmean,
        fmax - fmin
    );

    // Compare like with like: express each spread as a fraction of its own mean,
    // so "3 ticks out of 300" and "30ms out of 3000ms" are directly comparable.
    let wall_rel = (wmax - wmin) / wmean.max(1.0);
    let tick_rel = (tmax - tmin) / tmean.max(1.0);
    println!(
        "\n  relative spread: wall {:.4}   ticks {:.4}",
        wall_rel, tick_rel
    );

    println!("\n========== CONCLUSION ==========");
    if tick_rel < wall_rel * 0.5 {
        println!("  TICK-COUNTED. The countdown spans a near-constant number of ticks");
        println!("  while wall time varies, so the boarder leaves on a fixed tick and the");
        println!("  observed first_moving spread comes from where RECORDING starts, not");
        println!("  from the countdown. Look at the arm alignment, not the clock phase.");
    } else if wall_rel < tick_rel * 0.5 {
        println!("  WALL-CLOCK. The countdown spans a near-constant real duration while the");
        println!("  tick it lands on varies — so which tick the boarder leaves on is decided");
        println!("  by where the tick boundaries fall inside that interval. That is the");
        println!("  sub-tick phase, and it is genuinely what has to be captured.");
    } else {
        println!("  INCONCLUSIVE — both vary comparably. Neither is a clean fixed quantity;");
        println!("  the countdown may be driven by something else again.");
    }
    true
}
