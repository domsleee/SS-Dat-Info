//! Cave hook performance benchmark runner (SSB-296).
//!
//! Method:
//! - Measure fixed frame windows (not fixed time) to reduce jitter.
//! - Instrumentation source is in-DLL per-hook `__rdtsc` counters.
//! - Run multiple repeats and report aggregate means.

use crate::harness;
use std::thread;
use std::time::{Duration, Instant};
use tas_shared::{input_bits, TasHookPerfCounter, TasMode, TasSharedMemoryClient};

const DEFAULT_REPEATS: u32 = 3;
const DEFAULT_MEASURE_FRAMES: u32 = 600;
const FRAME_WAIT_TIMEOUT_SECS: u64 = 60;

#[derive(Debug, Clone, Copy)]
pub struct BenchmarkConfig {
    pub repeats: u32,
    pub measure_frames: u32,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            repeats: DEFAULT_REPEATS,
            measure_frames: DEFAULT_MEASURE_FRAMES,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct HookMetrics {
    pub calls: u64,
    pub cycles_total: u64,
    pub cycles_max: u64,
    pub avg_cycles_per_call: f64,
    pub cycles_per_frame: f64,
}

impl HookMetrics {
    fn from_counter(counter: TasHookPerfCounter, frames: u32) -> Self {
        let calls = counter.calls;
        let cycles_total = counter.cycles_total;
        let avg_cycles_per_call = if calls == 0 {
            0.0
        } else {
            cycles_total as f64 / calls as f64
        };
        let cycles_per_frame = if frames == 0 {
            0.0
        } else {
            cycles_total as f64 / frames as f64
        };
        Self {
            calls,
            cycles_total,
            cycles_max: counter.cycles_max,
            avg_cycles_per_call,
            cycles_per_frame,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ScenarioSample {
    pub frames: u32,
    pub elapsed_secs: f64,
    pub fps: f64,
    pub cave2: HookMetrics,
    pub cave5: HookMetrics,
    pub cave1c_down: HookMetrics,
    pub cave1c_up: HookMetrics,
    pub cave1d: HookMetrics,
    pub replay_capture: HookMetrics,
}

#[derive(Debug, Clone, Default)]
pub struct ScenarioAggregate {
    pub name: &'static str,
    pub samples: Vec<ScenarioSample>,
    pub mean: ScenarioSample,
}

#[derive(Debug, Clone, Default)]
pub struct BenchmarkReport {
    pub config: BenchmarkConfig,
    pub idle_off: ScenarioAggregate,
    pub rec_neutral: ScenarioAggregate,
    pub play_neutral: ScenarioAggregate,
    pub play_transitions: ScenarioAggregate,
}

impl BenchmarkReport {
    pub fn print_summary(&self) {
        println!("\n=== CAVE BENCHMARK SUMMARY (MEAN OF REPEATS) ===");
        println!(
            "Config: repeats={} measure_frames={}",
            self.config.repeats, self.config.measure_frames
        );

        print_aggregate(&self.idle_off);
        print_aggregate(&self.rec_neutral);
        print_aggregate(&self.play_neutral);
        print_aggregate(&self.play_transitions);
    }
}

fn print_aggregate(agg: &ScenarioAggregate) {
    println!("\n--- {} ---", agg.name);
    println!(
        "Frames={} Elapsed={:.3}s FPS={:.2}",
        agg.mean.frames, agg.mean.elapsed_secs, agg.mean.fps
    );
    print_hook("Cave2", agg.mean.cave2);
    print_hook("Cave5", agg.mean.cave5);
    print_hook("1C_down", agg.mean.cave1c_down);
    print_hook("1C_up", agg.mean.cave1c_up);
    print_hook("1D", agg.mean.cave1d);
    print_hook("ReplayCap", agg.mean.replay_capture);
}

fn print_hook(name: &str, m: HookMetrics) {
    println!(
        "  {:9} calls={:6} avg={:10.1} cyc/call max={:10} cyc/frame={:10.1}",
        name, m.calls, m.avg_cycles_per_call, m.cycles_max, m.cycles_per_frame
    );
}

fn aggregate(name: &'static str, samples: Vec<ScenarioSample>) -> ScenarioAggregate {
    let mut out = ScenarioAggregate {
        name,
        samples,
        mean: ScenarioSample::default(),
    };
    let n = out.samples.len() as f64;
    if n == 0.0 {
        return out;
    }
    for s in &out.samples {
        out.mean.frames += s.frames;
        out.mean.elapsed_secs += s.elapsed_secs;
        out.mean.fps += s.fps;
        add_hook(&mut out.mean.cave2, s.cave2);
        add_hook(&mut out.mean.cave5, s.cave5);
        add_hook(&mut out.mean.cave1c_down, s.cave1c_down);
        add_hook(&mut out.mean.cave1c_up, s.cave1c_up);
        add_hook(&mut out.mean.cave1d, s.cave1d);
        add_hook(&mut out.mean.replay_capture, s.replay_capture);
    }
    out.mean.frames = (out.mean.frames as f64 / n).round() as u32;
    out.mean.elapsed_secs /= n;
    out.mean.fps /= n;
    div_hook(&mut out.mean.cave2, n);
    div_hook(&mut out.mean.cave5, n);
    div_hook(&mut out.mean.cave1c_down, n);
    div_hook(&mut out.mean.cave1c_up, n);
    div_hook(&mut out.mean.cave1d, n);
    div_hook(&mut out.mean.replay_capture, n);
    out
}

fn add_hook(dst: &mut HookMetrics, src: HookMetrics) {
    dst.calls += src.calls;
    dst.cycles_total += src.cycles_total;
    dst.cycles_max = dst.cycles_max.max(src.cycles_max);
    dst.avg_cycles_per_call += src.avg_cycles_per_call;
    dst.cycles_per_frame += src.cycles_per_frame;
}

fn div_hook(h: &mut HookMetrics, n: f64) {
    h.calls = (h.calls as f64 / n).round() as u64;
    h.cycles_total = (h.cycles_total as f64 / n).round() as u64;
    h.avg_cycles_per_call /= n;
    h.cycles_per_frame /= n;
}

fn wait_for_frame_delta(client: &TasSharedMemoryClient, delta: u32) -> Result<u32, String> {
    let start = client.frame_count_volatile();
    let deadline = Instant::now() + Duration::from_secs(FRAME_WAIT_TIMEOUT_SECS);
    loop {
        let now = client.frame_count_volatile();
        let progressed = now.saturating_sub(start);
        if progressed >= delta {
            return Ok(progressed);
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "timeout waiting for {} frames (only advanced {})",
                delta, progressed
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn sample_window(
    client: &mut TasSharedMemoryClient,
    frames: u32,
) -> Result<ScenarioSample, String> {
    client.reset_hook_perf_counters();
    let t0 = Instant::now();
    let advanced = wait_for_frame_delta(client, frames)?;
    let elapsed = t0.elapsed().as_secs_f64();
    let fps = if elapsed > 0.0 {
        advanced as f64 / elapsed
    } else {
        0.0
    };
    let s = client.state();
    Ok(ScenarioSample {
        frames: advanced,
        elapsed_secs: elapsed,
        fps,
        cave2: HookMetrics::from_counter(s.perf_cave2, advanced),
        cave5: HookMetrics::from_counter(s.perf_cave5, advanced),
        cave1c_down: HookMetrics::from_counter(s.perf_cave1c_down, advanced),
        cave1c_up: HookMetrics::from_counter(s.perf_cave1c_up, advanced),
        cave1d: HookMetrics::from_counter(s.perf_cave1d, advanced),
        replay_capture: HookMetrics::from_counter(s.perf_replay_capture, advanced),
    })
}

fn run_idle_off(client: &mut TasSharedMemoryClient, frames: u32) -> Result<ScenarioSample, String> {
    harness::stop(client);
    thread::sleep(Duration::from_millis(100));
    sample_window(client, frames)
}

fn run_rec_neutral(
    client: &mut TasSharedMemoryClient,
    frames: u32,
) -> Result<ScenarioSample, String> {
    harness::stop(client);
    thread::sleep(Duration::from_millis(100));
    harness::arm_rec(client);
    let sample = sample_window(client, frames)?;
    harness::stop(client);
    Ok(sample)
}

fn run_play_current_recording(
    client: &mut TasSharedMemoryClient,
    frames: u32,
) -> Result<ScenarioSample, String> {
    harness::stop(client);
    thread::sleep(Duration::from_millis(100));
    harness::arm_play(client);
    let sample = sample_window(client, frames)?;
    if client.mode_volatile() == TasMode::Play as u32 {
        harness::stop(client);
    }
    Ok(sample)
}

fn run_play_high_transitions(
    client: &mut TasSharedMemoryClient,
    frames: u32,
) -> Result<ScenarioSample, String> {
    let len = frames as usize + 4;
    let mut synthetic = vec![0u8; len];
    for (i, slot) in synthetic.iter_mut().enumerate() {
        *slot = if i % 2 == 0 {
            input_bits::LEFT
        } else {
            input_bits::RIGHT
        };
    }
    harness::write_synthetic_input(client, &synthetic);
    client.state_mut().continue_from_frame = 0;
    run_play_current_recording(client, frames)
}

fn ensure_race_stable(client: &mut TasSharedMemoryClient) -> bool {
    if harness::restart_and_stabilize_inprocess(client) {
        return true;
    }
    harness::restart_and_stabilize(client)
}

pub fn run(config: BenchmarkConfig) -> Result<BenchmarkReport, String> {
    println!("=== Cave Hook Benchmark (SSB-296) ===");
    println!(
        "Config: repeats={} measure_frames={}\n",
        config.repeats, config.measure_frames
    );

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut idle_samples = Vec::new();
    let mut rec_samples = Vec::new();
    let mut play_samples = Vec::new();
    let mut play_transition_samples = Vec::new();

    for i in 0..config.repeats {
        println!("\n=== Repeat {}/{} ===", i + 1, config.repeats);
        if !ensure_race_stable(&mut client) {
            return Err("Failed to stabilize race before benchmark repeat".into());
        }

        let idle = run_idle_off(&mut client, config.measure_frames)?;
        println!(
            "  idle-off: fps={:.2} cave2_avg={:.1} cave5_avg={:.1}",
            idle.fps, idle.cave2.avg_cycles_per_call, idle.cave5.avg_cycles_per_call
        );
        idle_samples.push(idle);

        let rec = run_rec_neutral(&mut client, config.measure_frames)?;
        println!(
            "  rec-neutral: fps={:.2} cave2_avg={:.1} cave5_avg={:.1}",
            rec.fps, rec.cave2.avg_cycles_per_call, rec.cave5.avg_cycles_per_call
        );
        rec_samples.push(rec);

        let play = run_play_current_recording(&mut client, config.measure_frames)?;
        println!(
            "  play-neutral: fps={:.2} cave2_avg={:.1} cave5_avg={:.1}",
            play.fps, play.cave2.avg_cycles_per_call, play.cave5.avg_cycles_per_call
        );
        play_samples.push(play);

        let play_transitions = run_play_high_transitions(&mut client, config.measure_frames)?;
        println!(
            "  play-transitions: fps={:.2} cave2_avg={:.1} cave5_avg={:.1} cave1d_calls={}",
            play_transitions.fps,
            play_transitions.cave2.avg_cycles_per_call,
            play_transitions.cave5.avg_cycles_per_call,
            play_transitions.cave1d.calls
        );
        play_transition_samples.push(play_transitions);
    }

    let report = BenchmarkReport {
        config,
        idle_off: aggregate("IDLE_OFF", idle_samples),
        rec_neutral: aggregate("REC_NEUTRAL", rec_samples),
        play_neutral: aggregate("PLAY_NEUTRAL", play_samples),
        play_transitions: aggregate("PLAY_HIGH_TRANSITIONS", play_transition_samples),
    };
    report.print_summary();
    Ok(report)
}
