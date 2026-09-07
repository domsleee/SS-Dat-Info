//! CONT reliability test: repeated long-offset continue splices.
//!
//! Validates that `ARM_CONTINUE` reliably reaches the REC splice at a large
//! frame offset without introducing drift in the replayed prefix.

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
    /// One long LEFT hold followed by one long RIGHT hold.
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
            Self::Sweep => "sweep (left/right hold)".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ContCycleResult {
    pub iteration: u32,
    pub spliced: bool,
    /// F5 rerolls needed before the bucket landed (0 = first try).
    pub rerolls: u32,
    /// Wall-clock ms from initiating the CONT (restart) to the splice,
    /// including any rerolls: what the user waits through.
    pub resume_ms: f64,
    pub mode_rec_after_splice: bool,
    pub replay_coverage_ok: bool,
    pub playback_pos_at_splice: u32,
    pub splice_recorded_count: u32,
    pub max_drift_x: f64,
    pub max_drift_y: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_z: usize,
    pub forward_only_ok: bool,
    pub prefix_net_z: f64,
}

impl ContCycleResult {
    fn is_clean(&self) -> bool {
        self.spliced
            && self.mode_rec_after_splice
            && self.replay_coverage_ok
            && self.forward_only_ok
            && self.max_drift_x == 0.0
            && self.max_drift_y == 0.0
            && self.max_drift_z == 0.0
    }
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
        // `.all()` is vacuously true on an empty set; require at least one cycle.
        !self.results.is_empty() && self.results.iter().all(ContCycleResult::is_clean)
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
            "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>5} {:>8} {:>8} {:>10} {:>12} {:>12} {:>12}",
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
            "max_drift_y",
            "max_drift_z"
        );
        println!("{}", "-".repeat(135));
        for r in &self.results {
            println!(
                "{:>3} {:>6} {:>6} {:>8} {:>8} {:>6} {:>5} {:>8} {:>8} {:>10.3} {:>12.9} {:>12.9} {:>12.9}",
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
                r.max_drift_y,
                r.max_drift_z,
            );
        }

        let spliced: Vec<&ContCycleResult> = self.results.iter().filter(|r| r.spliced).collect();
        if !spliced.is_empty() {
            let total: u32 = spliced.iter().map(|r| r.rerolls).sum();
            let max = spliced.iter().map(|r| r.rerolls).max().unwrap_or(0);
            let first_try = spliced.iter().filter(|r| r.rerolls == 0).count();
            let rerolls_list: Vec<String> = self
                .results
                .iter()
                .map(|r| {
                    if r.spliced {
                        r.rerolls.to_string()
                    } else {
                        "x".into()
                    }
                })
                .collect();
            println!();
            println!(
                "Bucket lottery (rerolls): first-try {}/{} | mean {:.1} | worst {} | per-cycle [{}]",
                first_try,
                spliced.len(),
                total as f64 / spliced.len() as f64,
                max,
                rerolls_list.join(" ")
            );
            let resume_times: Vec<f64> = spliced.iter().map(|r| r.resume_ms).collect();
            let rmin = resume_times.iter().cloned().fold(f64::INFINITY, f64::min);
            let rmax = resume_times.iter().cloned().fold(0.0_f64, f64::max);
            let rmean = resume_times.iter().sum::<f64>() / resume_times.len() as f64;
            println!(
                "Time to resume (restart→splice): min {:.0} ms | mean {:.0} ms | max {:.0} ms",
                rmin, rmean, rmax
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
            let fail_count = self.results.iter().filter(|r| !r.is_clean()).count();
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
                (input_bits::LEFT, left_ticks),
                (input_bits::RIGHT, right_ticks),
                (0x00, BASELINE_TAIL_TICKS),
            ])
        }
        BaselineInputProfile::Taps => {
            let mut holds = Vec::new();
            let mut remaining = splice_frame.max(1);
            let mut left = true;
            let tap_ticks = tap_ticks.max(1);
            while remaining > 0 {
                let hold = remaining.min(tap_ticks);
                let mask = if left {
                    input_bits::LEFT
                } else {
                    input_bits::RIGHT
                };
                holds.push((mask, hold));
                remaining -= hold;
                left = !left;
            }
            holds.push((0x00, BASELINE_TAIL_TICKS));
            patterns::build_from_explicit(&holds)
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
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_z = min_z.min(z);
        max_z = max_z.max(z);
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
    // Carving causes local backward deltas, so gate on net forward travel.
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
    state.force_fixed_tick = 0;
}

/// Judge one completed CONT cycle. Gate-aligned CONT shifts the live play
/// index relative to the recording, so coverage and drift are gate-relative;
/// unaligned CONT has `gate_align_rec == 0` and this reduces to the raw-index
/// comparison.
fn assess_splice(
    client: &TasSharedMemoryClient,
    iteration: u32,
    rerolls: u32,
    resume_ms: f64,
    splice_frame: u32,
) -> ContCycleResult {
    let state = client.state();
    let playback_pos_at_splice = state.playback_pos;
    // Alignment is only in effect when the gate fired AND the splice is past
    // it, mirroring the controller's own fallback.
    let aligned =
        state.gate_align_rec > 0 && state.gate_index > 0 && splice_frame > state.gate_align_rec;
    let (rec_gate, play_gate) = if aligned {
        (state.gate_align_rec, state.gate_index)
    } else {
        (0, 0)
    };
    let aligned_splice = if aligned {
        play_gate + (splice_frame - rec_gate)
    } else {
        splice_frame
    };
    let replay_coverage_ok = playback_pos_at_splice >= aligned_splice;
    let rel_count = (splice_frame.saturating_sub(rec_gate))
        .min(playback_pos_at_splice.saturating_sub(play_gate));
    let assessed_prefix = splice_frame.min(playback_pos_at_splice);
    let d = if aligned {
        drift::compute_gate_relative_drift(state, rec_gate, play_gate, rel_count)
    } else {
        drift::compute_drift(state, assessed_prefix)
    };
    let prefix_stats = analyze_prefix_coords(&state.play_coords, assessed_prefix);
    println!(
        "  Drift over replayed prefix [0..{}): X={:.9} (frame {}) Y={:.9} Z={:.9} (frame {})",
        assessed_prefix,
        d.max_drift_x,
        d.max_drift_frame_x,
        d.max_drift_y,
        d.max_drift_z,
        d.max_drift_frame_z
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
    if !prefix_stats.forward_only_ok {
        println!(
            "  Forward-progress check FAIL: net_z={:.6}, z_range={:.6}, forward_steps={}",
            prefix_stats.net_z, prefix_stats.z_range, prefix_stats.forward_steps
        );
    }
    if !replay_coverage_ok {
        println!(
            "  Coverage shortfall: playback_pos={} < splice_frame={} (FAIL)",
            playback_pos_at_splice, splice_frame
        );
    }
    ContCycleResult {
        iteration,
        spliced: true,
        rerolls,
        resume_ms,
        mode_rec_after_splice: state.mode == TasMode::Rec as u32,
        replay_coverage_ok,
        playback_pos_at_splice,
        splice_recorded_count: state.recorded_count,
        max_drift_x: d.max_drift_x,
        max_drift_y: d.max_drift_y,
        max_drift_z: d.max_drift_z,
        max_drift_frame_x: d.max_drift_frame_x,
        max_drift_frame_z: d.max_drift_frame_z,
        forward_only_ok: prefix_stats.forward_only_ok,
        prefix_net_z: prefix_stats.net_z,
    }
}

fn failed_splice(iteration: u32, resume_ms: f64) -> ContCycleResult {
    ContCycleResult {
        iteration,
        spliced: false,
        rerolls: 0,
        resume_ms,
        mode_rec_after_splice: false,
        replay_coverage_ok: false,
        playback_pos_at_splice: 0,
        splice_recorded_count: 0,
        max_drift_x: f64::INFINITY,
        max_drift_y: f64::INFINITY,
        max_drift_z: f64::INFINITY,
        max_drift_frame_x: usize::MAX,
        max_drift_frame_z: usize::MAX,
        forward_only_ok: false,
        prefix_net_z: 0.0,
    }
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
    harness::ensure_exclusive_runtime_ownership(&mut client, "CONT anchor mismatch");
    harness::assert_proven_config(&client);

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
                    "  Loaded: {} ticks, fft={}",
                    loaded.count, loaded.meta.force_fixed_tick
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
                // Focus BEFORE the restart: the CONT cycles arm immediately
                // after their restart, so wall time spent between restart and
                // ARM_REC here would shift the baseline's first-moving index
                // under every replay's and the bucket judge would reroll forever.
                harness::focus_game();
                if !harness::restart_and_stabilize_inprocess(&mut client) {
                    eprintln!("ERROR: Game not alive for baseline REC (in-process restart)");
                    std::process::exit(1);
                }
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
            // The DLL drops to this speed atomically at the splice, so the
            // post-splice recorded_count overshoot stays near zero.
            client.state_mut().cont_resume_speed = 1.0;
            let resume_t0 = std::time::Instant::now();
            let splice_result = harness::restart_continue_and_splice_inprocess(
                &mut client,
                rec_start,
                splice_frame,
                CONT_RESTART_RETRIES,
            );
            let resume_ms = resume_t0.elapsed().as_secs_f64() * 1000.0;
            results.push(match splice_result {
                Some(rerolls) => assess_splice(&client, i, rerolls, resume_ms, splice_frame),
                None => {
                    println!("  Splice failed before REC transition");
                    failed_splice(i, resume_ms)
                }
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
