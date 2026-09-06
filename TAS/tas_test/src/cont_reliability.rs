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
    /// F5 rerolls the bucket lottery needed before it landed (0 = first try).
    /// This is the "bucket repeats too often" signal, per iteration.
    pub rerolls: u32,
    /// Wall-clock ms from initiating the CONT (restart) to the splice — the
    /// "time to resume" the user waits through. Includes any rerolls.
    pub resume_ms: f64,
    pub mode_rec_after_splice: bool,
    pub replay_coverage_ok: bool,
    pub playback_pos_at_splice: u32,
    pub splice_recorded_count: u32,
    /// DLL-stamped frame_count (Cycle-callback counter) at CONT replay start and
    /// at the splice. The DELTA = game-frames the catch-up replay took to reach
    /// the splice; its run-to-run spread is the "resume off by a few frames"
    /// skew (Problem B). 0/0 if the DLL predates the stamps.
    pub replay_start_fc: u32,
    pub splice_fc: u32,
    pub max_drift_x: f64,
    pub max_drift_y: f64,
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
        // `.all()` is vacuously true on an empty set: zero completed splice
        // cycles would report a clean run. Require at least one.
        !self.results.is_empty()
            && self.results.iter().all(|r| {
                r.spliced
                    && r.mode_rec_after_splice
                    && r.replay_coverage_ok
                    && r.forward_only_ok
                    && r.max_drift_x == 0.0
                    && r.max_drift_y == 0.0
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

        // Problem A — the F5 bucket lottery (independent of resume timing):
        // how many rerolls each splice needed to land the recording's bucket.
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
        }

        // Problem B — resume TIMING (independent of the bucket lottery above):
        // game-frames the catch-up replay took to reach the splice
        // (cont_splice_fc - cont_replay_start_fc). The input index at the splice
        // is always exactly splice_frame, so any run-to-run variation in this
        // delta is the "resume off by a few frames" skew. Needs the DLL splice
        // stamps; shows n/a against an older DLL (both counters 0).
        let timed: Vec<u32> = self
            .results
            .iter()
            .filter(|r| r.spliced && (r.splice_fc != 0 || r.replay_start_fc != 0))
            .map(|r| r.splice_fc.saturating_sub(r.replay_start_fc))
            .collect();
        println!();
        if timed.is_empty() {
            println!(
                "Resume timing (replay game-frames): n/a — DLL splice stamps absent (rebuild + deploy the DLL)"
            );
        } else {
            let min = *timed.iter().min().unwrap();
            let max = *timed.iter().max().unwrap();
            let mean = timed.iter().sum::<u32>() as f64 / timed.len() as f64;
            let per: Vec<String> = self
                .results
                .iter()
                .map(|r| {
                    if r.spliced && (r.splice_fc != 0 || r.replay_start_fc != 0) {
                        r.splice_fc.saturating_sub(r.replay_start_fc).to_string()
                    } else {
                        "x".into()
                    }
                })
                .collect();
            println!(
                "Resume timing (replay game-frames to splice): min {} | max {} | spread {} | mean {:.1} | per-cycle [{}]",
                min, max, max - min, mean, per.join(" ")
            );
            if max != min {
                println!(
                    "  -> handover is NOT frame-stable: resume lands within a {}-frame window (Problem B reproduced)",
                    max - min
                );
            } else {
                println!(
                    "  -> replay→splice is frame-stable; any few-frame skew is downstream (UI 64x→0.5x handover), not the replay"
                );
            }
        }

        // Post-splice OVERSHOOT (the actual Problem B symptom): how far
        // recorded_count had already run past the splice frame by the time it
        // was read. Before the atomic-speed-drop fix this raced at the catch-up
        // rate (tens-to-hundreds of frames); with cont_resume_speed staged, the
        // DLL drops to 1x at the splice so it should collapse to ~0-2 frames.
        let overshoots: Vec<u32> = self
            .results
            .iter()
            .filter(|r| r.spliced)
            .map(|r| r.splice_recorded_count.saturating_sub(self.splice_frame))
            .collect();
        if !overshoots.is_empty() {
            let omin = *overshoots.iter().min().unwrap();
            let omax = *overshoots.iter().max().unwrap();
            let omean = overshoots.iter().sum::<u32>() as f64 / overshoots.len() as f64;
            println!(
                "Post-splice overshoot (rec_cnt − splice_frame): min {} | max {} | mean {:.1} | per-cycle [{}]",
                omin,
                omax,
                omean,
                overshoots.iter().map(|o| o.to_string()).collect::<Vec<_>>().join(" ")
            );
        }

        // Time to resume (restart→splice wall-clock) — the wait after pressing
        // CONT. min is the clean no-reroll catch-up; max includes reroll cycles.
        let resume_times: Vec<f64> = self
            .results
            .iter()
            .filter(|r| r.spliced)
            .map(|r| r.resume_ms)
            .collect();
        if !resume_times.is_empty() {
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
            let fail_count = self
                .results
                .iter()
                .filter(|r| {
                    !(r.spliced
                        && r.mode_rec_after_splice
                        && r.replay_coverage_ok
                        && r.forward_only_ok
                        && r.max_drift_x == 0.0
                        && r.max_drift_y == 0.0
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
                // Focus BEFORE the restart, not between restart and arm: the
                // CONT replay cycles arm immediately after their restart (no
                // focus call — see harness::run_cont_cycle), so any wall time
                // spent here between restart and ARM_REC shifts the baseline's
                // first-moving index under every replay's (a focus switch is
                // ~300ms ≈ 30 ticks) and the bucket judge then rerolls forever
                // with a systematic offset. Focus-first keeps restart→arm
                // identical in both paths regardless of who held focus.
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
            // Stage a resume speed so the DLL drops to it atomically at the
            // splice (Problem B fix). With this, the post-splice recorded_count
            // overshoot (rec_cnt - splice_frame) should collapse from the ~64x
            // poll-window to near-zero. 1.0 = a clean resume baseline.
            client.state_mut().cont_resume_speed = 1.0;
            // Time to resume: wall-clock from initiating the CONT (restart) to
            // the PLAY→REC splice. This is what the user waits through after
            // pressing CONT. Includes any F5 rerolls (reported separately).
            // Catch-up throughput probe: reset hook perf counters so we can read
            // cave2.calls (= physics ticks) vs cave5.calls (= render frames) over
            // the whole CONT cycle. Their ratio = ticks per render frame (is the
            // game hitting the CAVE5_PER_FRAME_TICK_CAP=64?), and cave5.calls/sec
            // = render FPS during catch-up. This is the experiment for "why 64 —
            // is catch-up render-bound or compute-bound?".
            client.reset_hook_perf_counters();
            let resume_t0 = std::time::Instant::now();
            let held_mask = match std::env::var("TAS_CONT_HOLD_KEYS").as_deref() {
                Ok("LEFT+SHIFT") => Some(input_bits::LEFT | input_bits::SHIFT),
                Ok("UP") => Some(input_bits::UP),
                Ok(other) => panic!("unsupported TAS_CONT_HOLD_KEYS: {other}"),
                Err(_) if std::env::var("TAS_CONT_HOLD_UP").as_deref() == Ok("1") => Some(input_bits::UP),
                Err(_) => None,
            };
            let splice_result = if let Some(held_mask) = held_mask {
                use std::sync::atomic::{AtomicBool, Ordering};
                let mut keys = harness::PicoKeys::open().expect("held-input test requires Pico HID");
                harness::focus_game();
                assert!(keys.send(held_mask), "Pico held-key press failed");
                println!("  Holding physical HID mask 0x{held_mask:02X} throughout restart and catch-up ({})", keys.port_name());
                // Refresh before the firmware's 500ms watchdog, including retries.
                // Scope exit joins the writer; PicoKeys::drop releases the key.
                let finished = AtomicBool::new(false);
                thread::scope(|scope| {
                    struct StopKeepalive<'a>(&'a AtomicBool);
                    impl Drop for StopKeepalive<'_> {
                        fn drop(&mut self) { self.0.store(true, Ordering::Release); }
                    }
                    let writer = scope.spawn(|| {
                        while !finished.load(Ordering::Acquire) {
                            if !keys.send(held_mask) { return false; }
                            thread::sleep(Duration::from_millis(50));
                        }
                        true
                    });
                    let stop_keepalive = StopKeepalive(&finished);
                    let result = harness::restart_continue_and_splice_inprocess(
                        &mut client, rec_start, splice_frame, CONT_RESTART_RETRIES,
                    );
                    drop(stop_keepalive);
                    assert!(writer.join().expect("Pico writer panicked"), "Pico keepalive failed");
                    result
                })
            } else {
                harness::restart_continue_and_splice_inprocess(
                    &mut client,
                    rec_start,
                    splice_frame,
                    CONT_RESTART_RETRIES,
                )
            };
            let resume_ms = resume_t0.elapsed().as_secs_f64() * 1000.0;
            {
                let st = client.state();
                let ticks = st.perf_cave2.calls;
                let render_frames = st.perf_cave5.calls;
                let cave2_cyc = st
                    .perf_cave2
                    .cycles_total
                    .checked_div(st.perf_cave2.calls)
                    .unwrap_or(0);
                let cave5_cyc = st
                    .perf_cave5
                    .cycles_total
                    .checked_div(st.perf_cave5.calls)
                    .unwrap_or(0);
                let secs = resume_ms / 1000.0;
                let render_fps = if secs > 0.0 {
                    render_frames as f64 / secs
                } else {
                    0.0
                };
                let ticks_per_frame = if render_frames > 0 {
                    ticks as f64 / render_frames as f64
                } else {
                    0.0
                };
                println!(
                    "  PERF: ticks(cave2)={} render_frames(cave5)={} | ticks/frame={:.1} (cap={}) | render_fps={:.0} | hook_cyc cave2={} cave5={}",
                    ticks, render_frames, ticks_per_frame, 64, render_fps, cave2_cyc, cave5_cyc
                );
            }
            let spliced = splice_result.is_some();
            // Reroll count: how many times the F5 bucket lottery missed before
            // landing (0 = first try). Surfaced per-iteration + aggregated.
            let rerolls = splice_result.unwrap_or(0);
            let mut mode_rec_after_splice = false;
            let mut replay_coverage_ok = false;
            let mut playback_pos_at_splice = 0;
            let mut splice_recorded_count = 0;
            let mut max_drift_x = f64::INFINITY;
            let mut max_drift_y = f64::INFINITY;
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
            let mut replay_start_fc = 0;
            let mut splice_fc = 0;

            if spliced {
                let state = client.state();
                mode_rec_after_splice = state.mode == TasMode::Rec as u32;
                splice_recorded_count = state.recorded_count;
                replay_start_fc = state.cont_replay_start_fc;
                splice_fc = state.cont_splice_fc;
                playback_pos_at_splice = state.playback_pos;
                // Gate-aligned CONT shifts the live play index relative to the
                // recording, so coverage and drift are gate-relative. Unaligned
                // CONT has gate_align_rec == 0 and this reduces to the old
                // raw-index comparison exactly.
                // Mirror the caller fallback: alignment is only in effect when
                // the gate fired AND the splice is past it, so a gate stamp
                // cannot be mistaken and the gate-relative length cannot
                // underflow.
                let aligned = state.gate_align_rec > 0
                    && state.gate_index > 0
                    && splice_frame > state.gate_align_rec;
                let (rec_gate, play_gate) = if aligned {
                    (state.gate_align_rec, state.gate_index)
                } else {
                    (0, 0)
                };
                // Play index the splice fired at, in gate-relative terms.
                let aligned_splice = if aligned {
                    play_gate + (splice_frame - rec_gate)
                } else {
                    splice_frame
                };
                replay_coverage_ok = playback_pos_at_splice >= aligned_splice;
                // Gate-relative prefix length: ticks of trajectory past the gate
                // that both sides share.
                let rel_count = (splice_frame.saturating_sub(rec_gate))
                    .min(playback_pos_at_splice.saturating_sub(play_gate));
                let assessed_prefix = splice_frame.min(playback_pos_at_splice);
                let d = if aligned {
                    drift::compute_drift_gate_relative(state, rec_gate, play_gate, rel_count)
                } else {
                    drift::compute_drift(state, assessed_prefix)
                };
                max_drift_x = d.max_drift_x;
                max_drift_y = d.max_drift_y;
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
                    "  Drift over replayed prefix [0..{}): X={:.9} (frame {}) Y={:.9} Z={:.9} (frame {})",
                    assessed_prefix,
                    max_drift_x,
                    max_drift_frame_x,
                    max_drift_y,
                    max_drift_z,
                    max_drift_frame_z
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
                rerolls,
                resume_ms,
                mode_rec_after_splice,
                replay_coverage_ok,
                playback_pos_at_splice,
                splice_recorded_count,
                replay_start_fc,
                splice_fc,
                max_drift_x,
                max_drift_y,
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
