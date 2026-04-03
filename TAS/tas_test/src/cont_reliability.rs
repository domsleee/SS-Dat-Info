//! CONT reliability test: repeated long-offset continue splices.
//!
//! Validates that `ARM_CONTINUE` reliably reaches REC splice at large frame
//! offsets (default 2400) without introducing drift in the replayed prefix.

use crate::{drift, harness, patterns, replay};
use std::thread;
use std::time::Duration;
use tas_shared::{input_bits, TasMode, TasSharedMemoryClient, TAS_MAX_TICKS};

const BASELINE_TAIL_TICKS: u32 = 120;
const CONT_RESTART_RETRIES: u32 = 40;

#[derive(Debug)]
pub struct ContCycleResult {
    pub iteration: u32,
    pub spliced: bool,
    pub mode_rec_after_splice: bool,
    pub replay_coverage_ok: bool,
    pub playback_pos_at_splice: u32,
    pub splice_recorded_count: u32,
    pub max_drift_x: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_z: usize,
}

#[derive(Debug)]
pub struct ContReliabilityReport {
    pub iterations: u32,
    pub speed: f32,
    pub splice_frame: u32,
    pub baseline_ticks: u32,
    pub results: Vec<ContCycleResult>,
}

impl ContReliabilityReport {
    pub fn all_pass(&self) -> bool {
        self.results.iter().all(|r| {
            r.spliced
                && r.mode_rec_after_splice
                && r.replay_coverage_ok
                && r.max_drift_x == 0.0
                && r.max_drift_z == 0.0
        })
    }

    pub fn print_summary(&self) {
        println!("\n=== CONT RELIABILITY SUMMARY ===");
        println!(
            "Splice frame: {} | Catch-up speed: {}x | Iterations: {}",
            self.splice_frame, self.speed, self.iterations
        );
        println!("Baseline recording: {} ticks", self.baseline_ticks);
        println!();
        println!(
            "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>8} {:>8} {:>12} {:>12}",
            "#",
            "splice",
            "mode",
            "rec_cnt",
            "play_cnt",
            "cover",
            "frame_x",
            "frame_z",
            "max_drift_x",
            "max_drift_z"
        );
        println!("{}", "-".repeat(103));
        for r in &self.results {
            println!(
                "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>8} {:>8} {:>12.9} {:>12.9}",
                r.iteration,
                if r.spliced { "ok" } else { "FAIL" },
                if r.mode_rec_after_splice {
                    "REC"
                } else {
                    "bad"
                },
                r.splice_recorded_count,
                r.playback_pos_at_splice,
                if r.replay_coverage_ok { "ok" } else { "short" },
                r.max_drift_frame_x,
                r.max_drift_frame_z,
                r.max_drift_x,
                r.max_drift_z,
            );
        }
        println!();
        if self.all_pass() {
            println!(
                "*** CONT RELIABILITY PASSED: {}/{} splice cycles clean ***",
                self.results.len(),
                self.iterations
            );
        } else {
            let fail_count = self
                .results
                .iter()
                .filter(|r| {
                    !(r.spliced
                        && r.mode_rec_after_splice
                        && r.replay_coverage_ok
                        && r.max_drift_x == 0.0
                        && r.max_drift_z == 0.0)
                })
                .count();
            println!(
                "*** CONT RELIABILITY FAILED: {}/{} cycles had issues ***",
                fail_count, self.iterations
            );
        }
    }
}

fn build_baseline_steps(splice_frame: u32) -> Vec<patterns::PatternStep> {
    let left_ticks = (splice_frame / 2).max(1);
    let right_ticks = splice_frame.saturating_sub(left_ticks).max(1);
    patterns::build_from_explicit(&[
        ("LEFT", input_bits::LEFT, left_ticks),
        ("RIGHT", input_bits::RIGHT, right_ticks),
        ("TAIL", 0x00, BASELINE_TAIL_TICKS),
    ])
}

fn snapshot_baseline(
    client: &TasSharedMemoryClient,
    baseline_ticks: u32,
) -> (Vec<u8>, Vec<[f32; 3]>) {
    let count = baseline_ticks as usize;
    let state = client.state();
    (
        state.input_log[..count].to_vec(),
        state.rec_coords[..count].to_vec(),
    )
}

fn restore_baseline(
    client: &mut TasSharedMemoryClient,
    baseline_ticks: u32,
    baseline_input: &[u8],
    baseline_rec_coords: &[[f32; 3]],
) {
    let count = baseline_ticks as usize;
    let state = client.state_mut();
    state.input_log[..count].copy_from_slice(baseline_input);
    for i in count..TAS_MAX_TICKS {
        state.input_log[i] = 0;
        state.rec_coords[i] = [0.0, 0.0, 0.0];
    }
    for (i, coord) in baseline_rec_coords.iter().enumerate() {
        state.rec_coords[i] = *coord;
    }
    state.recorded_count = baseline_ticks;
    // Keep proven config explicit when restoring baseline data.
    state.force_fixed_tick = 0;
}

pub fn run(
    iterations: u32,
    speed: f32,
    splice_frame: u32,
    source_tasrec: Option<&str>,
) -> ContReliabilityReport {
    let splice_frame = splice_frame.max(1);
    println!(
        "=== CONT Reliability Test: {}x splice at frame {} ({}x catch-up) ===\n",
        iterations, splice_frame, speed
    );
    if let Some(path) = source_tasrec {
        println!("Source baseline: {}", path);
    }

    let mut client = harness::connect();
    harness::print_status(&client);

    if !harness::check_liveness(&client) {
        eprintln!("ERROR: Cave 2 not firing");
        std::process::exit(1);
    }

    {
        let s = client.state();
        assert_eq!(s.force_fixed_tick, 0, "fft must be 0");
        assert_eq!(s.inject_mode, 6, "inject_mode must be 6");
        assert_eq!(s.force_direct, 2, "force_direct must be 2");
        println!(
            "Config OK: fft=0, inject_mode=6, force_direct=2 (Cave5={})",
            if s.cave5_hooked == 1 {
                "hooked"
            } else {
                "missing"
            }
        );
    }

    let (baseline_ticks, rec_start) = if let Some(path) = source_tasrec {
        println!("--- Baseline load from .tasrec ---");
        let loaded = match replay::load_tasrec(std::path::Path::new(path)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("ERROR: Failed to load {}: {}", path, e);
                std::process::exit(1);
            }
        };
        if loaded.count == 0 || loaded.rec_coords.is_empty() {
            eprintln!("ERROR: Baseline recording is empty");
            std::process::exit(1);
        }
        replay::write_to_shared(&mut client, &loaded);
        println!(
            "  Loaded: {} ticks, inject_mode={}, fft={}, force_direct={}",
            loaded.count,
            loaded.meta.inject_mode,
            loaded.meta.force_fixed_tick,
            loaded.meta.force_direct
        );
        if !loaded.meta.notes.is_empty() {
            println!("  Notes: {}", loaded.meta.notes);
        }
        let rec_start = loaded.rec_coords[0];
        println!(
            "  REC start: ({:.6}, {:.6}, {:.6})",
            rec_start[0], rec_start[1], rec_start[2]
        );
        (loaded.count, rec_start)
    } else {
        println!("--- Baseline REC build (single pass) ---");
        client.state_mut().playback_speed = 1.0;
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("ERROR: Game not alive for baseline REC (in-process restart)");
            std::process::exit(1);
        }
        harness::focus_game();
        harness::arm_rec(&mut client);
        let baseline_steps = build_baseline_steps(splice_frame);
        println!(
            "  Driving baseline pattern for {} ticks (LEFT/RIGHT + tail)",
            patterns::total_ticks(&baseline_steps)
        );
        harness::drive_pico_steps(&baseline_steps, None);
        thread::sleep(Duration::from_millis(200));
        let baseline_ticks = client.state().recorded_count;
        let rec_start = client.state().rec_coords[0];
        harness::stop(&mut client);
        println!("  Baseline recorded: {} ticks", baseline_ticks);
        println!(
            "  REC start: ({:.6}, {:.6}, {:.6})",
            rec_start[0], rec_start[1], rec_start[2]
        );
        (baseline_ticks, rec_start)
    };

    if baseline_ticks <= splice_frame {
        eprintln!(
            "ERROR: Baseline too short ({} <= splice frame {})",
            baseline_ticks, splice_frame
        );
        std::process::exit(1);
    }

    let (baseline_input, baseline_rec_coords) = snapshot_baseline(&client, baseline_ticks);
    let mut results = Vec::new();
    for i in 1..=iterations {
        println!("\n{}", "=".repeat(60));
        println!("  CONT cycle {}/{}", i, iterations);
        println!("{}", "=".repeat(60));

        restore_baseline(
            &mut client,
            baseline_ticks,
            &baseline_input,
            &baseline_rec_coords,
        );
        client.state_mut().playback_speed = speed;
        let spliced = harness::restart_continue_and_splice_inprocess(
            &mut client,
            rec_start,
            splice_frame,
            CONT_RESTART_RETRIES,
        );
        let mut mode_rec_after_splice = false;
        let mut replay_coverage_ok = false;
        let mut playback_pos_at_splice = 0;
        let mut splice_recorded_count = 0;
        let mut max_drift_x = f64::INFINITY;
        let mut max_drift_z = f64::INFINITY;
        let mut max_drift_frame_x = usize::MAX;
        let mut max_drift_frame_z = usize::MAX;

        if spliced {
            let state = client.state();
            mode_rec_after_splice = state.mode == TasMode::Rec as u32;
            splice_recorded_count = state.recorded_count;
            playback_pos_at_splice = state.playback_pos;
            replay_coverage_ok = playback_pos_at_splice >= splice_frame;
            let assessed_prefix = splice_frame.min(playback_pos_at_splice);
            let d = drift::compute_drift(state, assessed_prefix);
            max_drift_x = d.max_drift_x;
            max_drift_z = d.max_drift_z;
            max_drift_frame_x = d.max_drift_frame_x;
            max_drift_frame_z = d.max_drift_frame_z;
            println!(
                "  Drift over replayed prefix [0..{}): X={:.9} (frame {}) Z={:.9} (frame {})",
                assessed_prefix, max_drift_x, max_drift_frame_x, max_drift_z, max_drift_frame_z
            );
            if !replay_coverage_ok {
                println!(
                    "  Coverage shortfall: playback_pos={} < splice_frame={} (FAIL)",
                    playback_pos_at_splice, splice_frame
                );
            }
        } else {
            println!("  Splice failed before REC transition");
        }

        results.push(ContCycleResult {
            iteration: i,
            spliced,
            mode_rec_after_splice,
            replay_coverage_ok,
            playback_pos_at_splice,
            splice_recorded_count,
            max_drift_x,
            max_drift_z,
            max_drift_frame_x,
            max_drift_frame_z,
        });

        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(100));
    }

    client.state_mut().playback_speed = 1.0;
    println!("\nReset playback_speed to 1.0");

    let report = ContReliabilityReport {
        iterations,
        speed,
        splice_frame,
        baseline_ticks,
        results,
    };
    report.print_summary();
    report
}
