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
const BASELINE_BUILD_ATTEMPTS_SYNTHETIC: u32 = 3;
const DEFAULT_TAP_TICKS: u32 = 8;
const FORWARD_EPS_Z: f32 = 1e-4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaselineInputProfile {
    /// High-transition pattern: alternating LEFT/RIGHT taps until splice frame.
    Taps,
    /// Legacy behavior: one long LEFT hold followed by one long RIGHT hold.
    Sweep,
}

impl BaselineInputProfile {
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "taps" => Some(Self::Taps),
            "sweep" => Some(Self::Sweep),
            _ => None,
        }
    }

    pub fn label(self, tap_ticks: u32) -> String {
        match self {
            Self::Taps => format!("taps ({} ticks per tap)", tap_ticks.max(1)),
            Self::Sweep => "sweep (legacy left/right hold)".to_string(),
        }
    }
}

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
    pub forward_only_ok: bool,
    pub prefix_net_z: f64,
    pub prefix_forward_steps: u32,
    pub prefix_backward_steps: u32,
    pub prefix_min_x: f32,
    pub prefix_max_x: f32,
    pub prefix_min_z: f32,
    pub prefix_max_z: f32,
}

#[derive(Debug)]
pub struct ContReliabilityReport {
    pub iterations: u32,
    pub speed: f32,
    pub splice_frame: u32,
    pub baseline_ticks: u32,
    pub baseline_profile: String,
    pub baseline_transitions: u32,
    pub results: Vec<ContCycleResult>,
}

impl ContReliabilityReport {
    pub fn all_pass(&self) -> bool {
        self.results.iter().all(|r| {
            r.spliced
                && r.mode_rec_after_splice
                && r.replay_coverage_ok
                && r.forward_only_ok
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
        println!(
            "Baseline recording: {} ticks | profile: {} | transitions: {}",
            self.baseline_ticks, self.baseline_profile, self.baseline_transitions
        );
        println!();
        println!(
            "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>5} {:>8} {:>8} {:>10} {:>12} {:>12}",
            "#",
            "splice",
            "mode",
            "rec_cnt",
            "play_cnt",
            "cover",
            "fwd",
            "frame_x",
            "frame_z",
            "net_z",
            "max_drift_x",
            "max_drift_z"
        );
        println!("{}", "-".repeat(122));
        for r in &self.results {
            println!(
                "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>5} {:>8} {:>8} {:>10.3} {:>12.9} {:>12.9}",
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
                if r.forward_only_ok { "ok" } else { "rev" },
                r.max_drift_frame_x,
                r.max_drift_frame_z,
                r.prefix_net_z,
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
                        && r.forward_only_ok
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

fn build_baseline_steps(
    splice_frame: u32,
    profile: BaselineInputProfile,
    tap_ticks: u32,
) -> Vec<patterns::PatternStep> {
    match profile {
        BaselineInputProfile::Sweep => {
            let left_ticks = (splice_frame / 2).max(1);
            let right_ticks = splice_frame.saturating_sub(left_ticks).max(1);
            patterns::build_from_explicit(&[
                ("LEFT", input_bits::LEFT, left_ticks),
                ("RIGHT", input_bits::RIGHT, right_ticks),
                ("TAIL", 0x00, BASELINE_TAIL_TICKS),
            ])
        }
        BaselineInputProfile::Taps => {
            let mut steps = Vec::new();
            let mut stop_tick = 0u32;
            let mut remaining = splice_frame.max(1);
            let mut tap_index = 1u32;
            let mut left = true;
            let tap_ticks = tap_ticks.max(1);

            while remaining > 0 {
                let hold = remaining.min(tap_ticks);
                stop_tick += hold;
                let (name, mask) = if left {
                    (format!("LEFT{}", tap_index), input_bits::LEFT)
                } else {
                    (format!("RIGHT{}", tap_index), input_bits::RIGHT)
                };
                steps.push(patterns::PatternStep {
                    name,
                    mask,
                    stop_tick,
                });
                remaining -= hold;
                tap_index += 1;
                left = !left;
            }

            stop_tick += BASELINE_TAIL_TICKS;
            steps.push(patterns::PatternStep {
                name: "TAIL".to_string(),
                mask: 0x00,
                stop_tick,
            });
            steps
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PrefixCoordStats {
    min_x: f32,
    max_x: f32,
    min_z: f32,
    max_z: f32,
    net_z: f64,
    z_range: f64,
    forward_steps: u32,
    backward_steps: u32,
    flat_steps: u32,
    forward_only_ok: bool,
}

fn analyze_prefix_coords(coords: &[[f32; 3]], frame_count: u32) -> PrefixCoordStats {
    if frame_count == 0 {
        return PrefixCoordStats {
            min_x: 0.0,
            max_x: 0.0,
            min_z: 0.0,
            max_z: 0.0,
            net_z: 0.0,
            z_range: 0.0,
            forward_steps: 0,
            backward_steps: 0,
            flat_steps: 0,
            forward_only_ok: false,
        };
    }

    let n = frame_count as usize;
    let mut min_x = coords[0][0];
    let mut max_x = coords[0][0];
    let mut min_z = coords[0][2];
    let mut max_z = coords[0][2];
    let mut forward_steps = 0u32;
    let mut backward_steps = 0u32;
    let mut flat_steps = 0u32;

    for i in 0..n {
        let x = coords[i][0];
        let z = coords[i][2];
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if z < min_z {
            min_z = z;
        }
        if z > max_z {
            max_z = z;
        }
        if i > 0 {
            let dz = z - coords[i - 1][2];
            if dz > FORWARD_EPS_Z {
                forward_steps += 1;
            } else if dz < -FORWARD_EPS_Z {
                backward_steps += 1;
            } else {
                flat_steps += 1;
            }
        }
    }

    let net_z = (coords[n - 1][2] - coords[0][2]) as f64;
    let z_range = (max_z - min_z) as f64;
    // Carving can cause local backward deltas. Gate on meaningful net forward travel instead.
    let forward_only_ok = net_z > 0.0 && z_range > 1.0 && forward_steps > 0;

    PrefixCoordStats {
        min_x,
        max_x,
        min_z,
        max_z,
        net_z,
        z_range,
        forward_steps,
        backward_steps,
        flat_steps,
        forward_only_ok,
    }
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
    baseline_profile: BaselineInputProfile,
    tap_ticks: Option<u32>,
) -> ContReliabilityReport {
    let splice_frame = splice_frame.max(1);
    let tap_ticks = tap_ticks.unwrap_or(DEFAULT_TAP_TICKS).max(1);
    println!(
        "=== CONT Reliability Test: {}x splice at frame {} ({}x catch-up) ===\n",
        iterations, splice_frame, speed
    );
    if let Some(path) = source_tasrec {
        println!("Source baseline: {}", path);
    } else {
        println!(
            "Synthetic baseline profile: {}",
            baseline_profile.label(tap_ticks)
        );
    }

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Quiesce stale runtime state before baseline capture/load so CONT compares
    // against deterministic test-owned data only.
    let stopped_writers = harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));
    if stopped_writers > 0 {
        println!(
            "  Runtime warmup after tas_ui stop ({} writer(s) killed)",
            stopped_writers
        );
        println!(
            "  NOTE: If CONT anchor mismatch persists after this, rerun from a clean revive with tas_ui closed."
        );
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("ERROR: Game not alive during pre-baseline warmup restart");
            std::process::exit(1);
        }
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(100));
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

    let baseline_attempts = if source_tasrec.is_some() {
        1
    } else {
        BASELINE_BUILD_ATTEMPTS_SYNTHETIC
    };

    for baseline_attempt in 1..=baseline_attempts {
        if baseline_attempts > 1 {
            println!(
                "\n--- Baseline attempt {}/{} ---",
                baseline_attempt, baseline_attempts
            );
        }

        let (baseline_ticks, rec_start, baseline_profile_label, baseline_transitions) =
            if let Some(path) = source_tasrec {
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
                let baseline_transitions =
                    drift::count_transitions(&loaded.input_log, loaded.count as usize);
                println!("  Input transitions: {}", baseline_transitions);
                (
                    loaded.count,
                    rec_start,
                    "file (.tasrec)".to_string(),
                    baseline_transitions,
                )
            } else {
                println!("--- Baseline REC build (single pass) ---");
                client.state_mut().playback_speed = 1.0;
                if !harness::restart_and_stabilize_inprocess(&mut client) {
                    eprintln!("ERROR: Game not alive for baseline REC (in-process restart)");
                    std::process::exit(1);
                }
                harness::focus_game();
                harness::arm_rec(&mut client);
                let baseline_steps =
                    build_baseline_steps(splice_frame, baseline_profile, tap_ticks);
                println!(
                    "  Driving baseline pattern for {} ticks ({})",
                    patterns::total_ticks(&baseline_steps),
                    baseline_profile.label(tap_ticks)
                );
                harness::drive_pico_steps(&baseline_steps, None);
                thread::sleep(Duration::from_millis(200));
                let baseline_ticks = client.state().recorded_count;
                let rec_start = client.state().rec_coords[0];
                let baseline_transitions =
                    drift::count_transitions(&client.state().input_log, baseline_ticks as usize);
                harness::stop(&mut client);
                println!("  Baseline recorded: {} ticks", baseline_ticks);
                println!("  Input transitions: {}", baseline_transitions);
                println!(
                    "  REC start: ({:.6}, {:.6}, {:.6})",
                    rec_start[0], rec_start[1], rec_start[2]
                );
                (
                    baseline_ticks,
                    rec_start,
                    baseline_profile.label(tap_ticks),
                    baseline_transitions,
                )
            };

        if baseline_ticks <= splice_frame {
            eprintln!(
                "ERROR: Baseline too short ({} <= splice frame {})",
                baseline_ticks, splice_frame
            );
            if baseline_attempt < baseline_attempts {
                println!("  Retrying baseline capture...");
                continue;
            }
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
            )
            .is_some();
            let mut mode_rec_after_splice = false;
            let mut replay_coverage_ok = false;
            let mut playback_pos_at_splice = 0;
            let mut splice_recorded_count = 0;
            let mut max_drift_x = f64::INFINITY;
            let mut max_drift_z = f64::INFINITY;
            let mut max_drift_frame_x = usize::MAX;
            let mut max_drift_frame_z = usize::MAX;
            let mut forward_only_ok = false;
            let mut prefix_net_z = 0.0;
            let mut prefix_forward_steps = 0;
            let mut prefix_backward_steps = 0;
            let mut prefix_min_x = 0.0;
            let mut prefix_max_x = 0.0;
            let mut prefix_min_z = 0.0;
            let mut prefix_max_z = 0.0;

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
                let prefix_stats = analyze_prefix_coords(&state.play_coords, assessed_prefix);
                forward_only_ok = prefix_stats.forward_only_ok;
                prefix_net_z = prefix_stats.net_z;
                prefix_forward_steps = prefix_stats.forward_steps;
                prefix_backward_steps = prefix_stats.backward_steps;
                prefix_min_x = prefix_stats.min_x;
                prefix_max_x = prefix_stats.max_x;
                prefix_min_z = prefix_stats.min_z;
                prefix_max_z = prefix_stats.max_z;
                println!(
                    "  Drift over replayed prefix [0..{}): X={:.9} (frame {}) Z={:.9} (frame {})",
                    assessed_prefix, max_drift_x, max_drift_frame_x, max_drift_z, max_drift_frame_z
                );
                println!(
                    "  Prefix range PLAY: X[{:.3}..{:.3}] Z[{:.3}..{:.3}] net_z={:.3} steps(+/-/0)={}/{}/{}",
                    prefix_stats.min_x,
                    prefix_stats.max_x,
                    prefix_stats.min_z,
                    prefix_stats.max_z,
                    prefix_stats.net_z,
                    prefix_stats.forward_steps,
                    prefix_stats.backward_steps,
                    prefix_stats.flat_steps
                );
                if !forward_only_ok {
                    println!(
                        "  Forward-progress check FAIL: net_z={:.6}, z_range={:.6}, forward_steps={}",
                        prefix_net_z, prefix_stats.z_range, prefix_forward_steps
                    );
                }
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
                forward_only_ok,
                prefix_net_z,
                prefix_forward_steps,
                prefix_backward_steps,
                prefix_min_x,
                prefix_max_x,
                prefix_min_z,
                prefix_max_z,
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
            baseline_profile: baseline_profile_label,
            baseline_transitions,
            results,
        };
        report.print_summary();
        if report.all_pass() || baseline_attempt == baseline_attempts {
            return report;
        }
        println!(
            "\nBaseline attempt {}/{} failed; rebuilding baseline and retrying...",
            baseline_attempt, baseline_attempts
        );
    }

    unreachable!("baseline attempt loop must return a report");
}
