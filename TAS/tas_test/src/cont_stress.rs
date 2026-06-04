//! cont-stress: the user's exact CONT use case, measuring reroll counts.
//!
//! Replicates pressing F12 (CONT) repeatedly while on a slow play speed: catch
//! up FAST (`--catchup`, default 64x) to the splice, then record at the slow
//! play speed (`--record-speed`, default 0.5x). Runs N iterations and reports
//! the reroll distribution, asserting every CONT splices, zero prefix drift, and
//! the median reroll count stays reasonable (the F5 lottery should not churn
//! dozens of restarts per CONT — that's the user's actual complaint).
//!
//! The reroll count comes from the shared transport controller, so this guards
//! the SHARED bucket-judge + jitter that tas_ui also runs: if a change makes the
//! lottery churn (as a non-blocking jitter did), the median explodes here.

use std::path::PathBuf;

use tas_shared::TasMode;

use crate::{drift, harness, replay};

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
const RETRIES: u32 = 30;

fn locate() -> Option<PathBuf> {
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

pub fn run(
    iterations: u32,
    splice_frame: u32,
    catchup: f32,
    record_speed: f32,
    max_median_rerolls: u32,
    file: Option<&str>,
    restart: &str,
) -> bool {
    let pico = restart.eq_ignore_ascii_case("pico");
    println!(
        "=== CONT-STRESS: {} iterations — catch up @ {}x → record @ {}x, splice {}, restart={} ===",
        iterations,
        catchup,
        record_speed,
        splice_frame,
        if pico { "PICO-F5" } else { "in-process" }
    );
    println!("  (replicates the user's F12 flow: slow play speed, fast catch-up)");

    // Reroll behaviour is recording-specific (the bucket the recording lives in
    // may be easy or hard to re-land), so use the user's actual tasrec when
    // given; otherwise fall back to FE-tremendous.
    let path = match file {
        Some(f) => {
            let p = PathBuf::from(f);
            if !p.exists() {
                eprintln!("ERROR: --file {} not found", f);
                return false;
            }
            p
        }
        None => match locate() {
            Some(p) => p,
            None => {
                eprintln!("ERROR: couldn't locate {} (pass --file <path>)", RECORDING_REL);
                return false;
            }
        },
    };
    let rec = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: load failed: {}", e);
            return false;
        }
    };
    if rec.count <= splice_frame {
        eprintln!("ERROR: recording has {} frames, need > {}", rec.count, splice_frame);
        return false;
    }
    println!("  Recording: {} ({} frames)", path.display(), rec.count);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "cont-stress");
    replay::write_to_shared(&mut client, &rec);
    let rec_start = rec.rec_coords[0];

    let mut reroll_counts: Vec<u32> = Vec::new();
    let mut wall_secs: Vec<f64> = Vec::new();
    let mut all_spliced = true;
    let mut worst_drift: f64 = 0.0;

    for i in 0..iterations {
        println!("\n--- iteration {}/{} ---", i + 1, iterations);
        // Slow play speed, fast catch-up — exactly the user's setup.
        client.state_mut().playback_speed = catchup;
        // Wall-clock "time to continue": from the CONT request to the splice
        // landing (the lag the user actually feels on F12).
        let t0 = std::time::Instant::now();
        // A/B the restart mechanism via the SAME judge loop (isolates the
        // restart as the only variable): Pico-F5 keypress vs in-process F5.
        let result = if pico {
            harness::restart_continue_and_splice(&mut client, rec_start, splice_frame, RETRIES)
        } else {
            harness::restart_continue_and_splice_inprocess_loop(
                &mut client,
                rec_start,
                splice_frame,
                RETRIES,
            )
        };
        let elapsed = t0.elapsed().as_secs_f64();
        match result {
            Some(rerolls) => {
                reroll_counts.push(rerolls);
                wall_secs.push(elapsed);
                let d = {
                    let s = client.state();
                    if s.mode != TasMode::Rec as u32 {
                        all_spliced = false;
                    }
                    drift::compute_drift(s, splice_frame)
                };
                let it_drift = d.max_drift_x.max(d.max_drift_z);
                worst_drift = worst_drift.max(it_drift);
                // Drop to the slow record speed and record a short burst, like
                // the user does after the catch-up.
                client.state_mut().playback_speed = record_speed;
                std::thread::sleep(std::time::Duration::from_millis(800));
                harness::stop(&mut client);
                println!(
                    "  iteration {}: {} reroll(s), {:.2}s to continue, prefix drift X={:.6} Z={:.6}",
                    i + 1,
                    rerolls,
                    elapsed,
                    d.max_drift_x,
                    d.max_drift_z
                );
            }
            None => {
                all_spliced = false;
                println!("  iteration {}: *** FAILED to splice ***", i + 1);
            }
        }
    }

    let n = reroll_counts.len();
    let mut sorted = reroll_counts.clone();
    sorted.sort_unstable();
    let median = if n > 0 { sorted[n / 2] } else { u32::MAX };
    let max = sorted.last().copied().unwrap_or(u32::MAX);
    let mean = if n > 0 {
        sorted.iter().sum::<u32>() as f64 / n as f64
    } else {
        0.0
    };

    let mut sorted_wall = wall_secs.clone();
    sorted_wall.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let wn = sorted_wall.len();
    let wall_median = if wn > 0 { sorted_wall[wn / 2] } else { 0.0 };
    let wall_max = sorted_wall.last().copied().unwrap_or(0.0);
    let wall_mean = if wn > 0 {
        sorted_wall.iter().sum::<f64>() / wn as f64
    } else {
        0.0
    };

    println!("\n=== CONT-STRESS RESULTS ===");
    println!("  reroll counts:  {:?}", reroll_counts);
    println!("  rerolls:        median={} mean={:.1} max={}", median, mean, max);
    println!(
        "  wall-clock (s): {}",
        wall_secs
            .iter()
            .map(|s| format!("{:.2}", s))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "  time-to-cont:   median={:.2}s mean={:.2}s max={:.2}s",
        wall_median, wall_mean, wall_max
    );
    println!("  all spliced:    {}", all_spliced);
    println!("  worst drift:    {:.9}", worst_drift);

    let rerolls_ok = median <= max_median_rerolls;
    let drift_ok = worst_drift == 0.0;
    if all_spliced && rerolls_ok && drift_ok {
        println!(
            "\n*** CONT-STRESS PASSED: median {} rerolls (<= {}), all spliced, zero drift ***",
            median, max_median_rerolls
        );
        true
    } else {
        if !rerolls_ok {
            println!(
                "  >> median {} rerolls exceeds budget {} — the F5 lottery is churning",
                median, max_median_rerolls
            );
        }
        println!(
            "\n*** CONT-STRESS FAILED: all_spliced={} median_rerolls_ok={} drift_ok={} ***",
            all_spliced, rerolls_ok, drift_ok
        );
        false
    }
}
