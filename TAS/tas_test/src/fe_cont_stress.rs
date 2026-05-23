//! Speed sweep for CONT splice reliability against FE-tremendous.
//!
//! Strict cont-reliability (cycles must hit bit-identical rec_coords)
//! can't pass for FE-tremendous at any speed — the recording was made
//! without rotation/velocity state captured, so no F5 bucket reproduces
//! the recorded trajectory at the post-countdown anchor frame. This
//! stress mode is the reference-comparison variant: take the FIRST
//! successful splice as the reference, then verify subsequent splices
//! produce the same prefix-replay trajectory by retrying F5 until they
//! match. This is what "reliable" actually means in practice — the
//! prefix replays the same way every time, regardless of how it
//! compares to the original recording's coords.
//!
//! For each speed in [1, 12, 32, 64, 128, 256] (configurable), runs
//! the reference cycle plus N verification cycles and reports whether
//! all N matched the reference within the retry budget.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
/// Frames of PLAY-prefix to compare against the reference. Set just below
/// SPLICE_FRAME so the verification window ends BEFORE the PLAY→REC mode
/// flip — once mode flips, playback_pos stops advancing and waiting for
/// it to hit SPLICE_FRAME would hang.
const VERIFY_FRAMES: u32 = 2199;
const DEFAULT_ITERATIONS_PER_SPEED: u32 = 20;
const REF_MATCH_RETRIES: u32 = 30;
const RESTART_TIMEOUT_SECS: u64 = 15;

#[derive(Clone, Copy)]
struct SpeedResult {
    speed: f32,
    reference_ok: bool,
    matched: u32,
    iterations: u32,
    /// Reference-splice wall-clock to reach VERIFY_FRAMES (seconds).
    ref_wall_secs: f64,
    /// Effective speedup of the reference splice vs 1× game time. At 1×
    /// the game advances 100 ticks/second, so VERIFY_FRAMES ticks would
    /// take VERIFY_FRAMES/100 seconds. Ratio is how much faster than that
    /// the actual catch-up was. Effective ≠ playback_speed setting once
    /// the per-frame tick cap or render-rate ceiling is hit.
    effective_x: f64,
}

fn wait_restart_complete(client: &mut tas_shared::TasSharedMemoryClient) -> bool {
    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    let start = Instant::now();
    loop {
        if client.restart_state() == 2 {
            client.reset_restart_state();
            return true;
        }
        if start.elapsed() > Duration::from_secs(RESTART_TIMEOUT_SECS) {
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// Returns (success, wall_clock_seconds from ARM_CONTINUE send to reaching
/// VERIFY_FRAMES).
fn arm_continue_and_wait_for_verify(
    client: &mut tas_shared::TasSharedMemoryClient,
) -> (bool, f64) {
    // Reset playback_pos before ArmContinue — a previous splice leaves
    // playback_pos at 2200 (the splice frame), and if the polling loop
    // reads that stale value it spuriously sees "verify frame already
    // reached" and returns 0.0s wall.
    client.state_mut().playback_pos = 0;
    let start = Instant::now();
    client.send_command(TasCommand::ArmContinue);
    // Wait long enough for cave2 to process the command (mode flips to
    // PLAY) before starting to poll. Don't just sleep — wait for mode.
    let mode_wait_start = Instant::now();
    while client.mode_volatile() != TasMode::Play as u32 {
        if mode_wait_start.elapsed() > Duration::from_secs(2) {
            // Command didn't switch mode to PLAY — bail.
            return (false, start.elapsed().as_secs_f64());
        }
        thread::sleep(Duration::from_millis(5));
    }
    let mut highest_pos: u32 = 0;
    let last_diag = Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        if pos > highest_pos {
            highest_pos = pos;
        }
        if highest_pos >= VERIFY_FRAMES {
            return (true, start.elapsed().as_secs_f64());
        }
        let mode = client.mode_volatile();
        let _ = last_diag;
        if mode == TasMode::Rec as u32 {
            return (highest_pos >= VERIFY_FRAMES, start.elapsed().as_secs_f64());
        }
        if mode == TasMode::Off as u32 {
            return (highest_pos >= VERIFY_FRAMES, start.elapsed().as_secs_f64());
        }
        if start.elapsed() > Duration::from_secs(60) {
            return (highest_pos >= VERIFY_FRAMES, start.elapsed().as_secs_f64());
        }
        thread::sleep(Duration::from_millis(15));
    }
}

fn capture_play_prefix(client: &tas_shared::TasSharedMemoryClient) -> Vec<[f32; 3]> {
    client.state().play_coords[..VERIFY_FRAMES as usize].to_vec()
}

fn matches_reference(client: &tas_shared::TasSharedMemoryClient, reference: &[[f32; 3]]) -> Option<usize> {
    let state = client.state();
    for j in 0..VERIFY_FRAMES as usize {
        let p = state.play_coords[j];
        let r = reference[j];
        if p[0].to_bits() != r[0].to_bits()
            || p[1].to_bits() != r[1].to_bits()
            || p[2].to_bits() != r[2].to_bits()
        {
            return Some(j);
        }
    }
    None
}

fn run_one_speed(
    client: &mut tas_shared::TasSharedMemoryClient,
    rec: &replay::LoadedRecording,
    speed: f32,
) -> SpeedResult {
    println!("\n\n========================================");
    println!("  Speed sweep: {}×", speed);
    println!("========================================");

    client.state_mut().playback_speed = speed;
    client.state_mut().continue_from_frame = SPLICE_FRAME;

    // ---- Reference splice ----
    println!("  Capturing reference splice...");
    replay::write_to_shared(client, rec);
    if !wait_restart_complete(client) {
        println!("  Reference restart timed out");
        return SpeedResult { speed, reference_ok: false, matched: 0, iterations: DEFAULT_ITERATIONS_PER_SPEED, ref_wall_secs: 0.0, effective_x: 0.0 };
    }
    // CMD_RESTART zeros continue_from_frame; re-write so ARM_CONTINUE's
    // validity check sees the right value.
    client.state_mut().continue_from_frame = SPLICE_FRAME;
    let (verify_ok, ref_wall) = arm_continue_and_wait_for_verify(client);
    if !verify_ok {
        println!("  Reference ARM_CONTINUE didn't reach frame {}", VERIFY_FRAMES);
        harness::stop(client);
        return SpeedResult { speed, reference_ok: false, matched: 0, iterations: DEFAULT_ITERATIONS_PER_SPEED, ref_wall_secs: 0.0, effective_x: 0.0 };
    }
    // Effective speedup: at 1× the game runs ~100 ticks/sec, so the
    // 1×-equivalent wall time for VERIFY_FRAMES ticks is VERIFY_FRAMES/100.
    // The actual prefix took ref_wall seconds.
    let baseline_wall = (VERIFY_FRAMES as f64) / 100.0;
    let effective_x = if ref_wall > 0.0 { baseline_wall / ref_wall } else { 0.0 };
    println!(
        "  Reference splice: {:.2}s wall to reach {} ticks (1× baseline {:.2}s → effective {:.2}×)",
        ref_wall, VERIFY_FRAMES, baseline_wall, effective_x
    );
    let reference = capture_play_prefix(client);
    println!(
        "  Reference: start=({:.4}, {:.4}, {:.4})  anchor[{}]=({:.4}, {:.4}, {:.4})",
        reference[0][0], reference[0][1], reference[0][2],
        VERIFY_FRAMES - 1,
        reference[(VERIFY_FRAMES - 1) as usize][0],
        reference[(VERIFY_FRAMES - 1) as usize][1],
        reference[(VERIFY_FRAMES - 1) as usize][2],
    );
    // Let prefix finish or just stop (we have what we need)
    harness::stop(client);
    thread::sleep(Duration::from_millis(200));

    // ---- Verification iterations ----
    let mut matched = 0u32;
    for i in 0..DEFAULT_ITERATIONS_PER_SPEED {
        let mut attempt_matched = false;
        for attempt in 0..=REF_MATCH_RETRIES {
            replay::write_to_shared(client, rec);
            client.state_mut().playback_speed = speed;
            client.state_mut().continue_from_frame = SPLICE_FRAME;

            if !wait_restart_complete(client) {
                println!("  Iter {}: restart timeout", i + 1);
                break;
            }
            // Re-set continue_from_frame after restart — CMD_RESTART zeros it.
            client.state_mut().continue_from_frame = SPLICE_FRAME;
            let (verify_ok, _wall) = arm_continue_and_wait_for_verify(client);
            if !verify_ok {
                println!("  Iter {}: didn't reach verify frame", i + 1);
                harness::stop(client);
                continue;
            }
            match matches_reference(client, &reference) {
                None => {
                    println!(
                        "  Iter {} matched on attempt {} ({} frames bit-identical)",
                        i + 1,
                        attempt + 1,
                        VERIFY_FRAMES
                    );
                    attempt_matched = true;
                    harness::stop(client);
                    thread::sleep(Duration::from_millis(200));
                    break;
                }
                Some(diverge_frame) => {
                    println!(
                        "  Iter {} attempt {}/{}: diverges at frame {}",
                        i + 1,
                        attempt + 1,
                        REF_MATCH_RETRIES,
                        diverge_frame
                    );
                    harness::stop(client);
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }
        if attempt_matched {
            matched += 1;
        } else {
            println!("  Iter {}: gave up after {} retries", i + 1, REF_MATCH_RETRIES);
        }
    }

    SpeedResult {
        speed,
        reference_ok: true,
        matched,
        iterations: DEFAULT_ITERATIONS_PER_SPEED,
        ref_wall_secs: ref_wall,
        effective_x,
    }
}

pub fn run(speeds: &[f32]) -> bool {
    println!(
        "=== FE-tremendous CONT-stress sweep (splice={}, verify={} frames, {} iters/speed) ===\n",
        SPLICE_FRAME, VERIFY_FRAMES, DEFAULT_ITERATIONS_PER_SPEED
    );

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-tremendous.tasrec"),
    ];
    let path = match candidates.iter().find(|p| p.exists()) {
        Some(p) => p.clone(),
        None => {
            eprintln!("ERROR: Couldn't locate {}", RECORDING_REL);
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

    // tas_ui (if running) writes playback_speed = 1.0 every frame from its
    // in-memory state, which clobbers our scripted speed setting and
    // collapses the test back to 1× rate. Kill it before doing any
    // speed-sensitive work.
    harness::ensure_exclusive_runtime_ownership(&mut client, "fe-cont-stress speed scaling");

    let mut results: Vec<SpeedResult> = Vec::with_capacity(speeds.len());
    for &speed in speeds {
        let r = run_one_speed(&mut client, &rec, speed);
        results.push(r);
    }
    // Reset speed to 1.0 at the end.
    client.state_mut().playback_speed = 1.0;

    println!("\n\n=== FE-CONT STRESS SUMMARY ===");
    println!(
        "{:>8}  {:>6}  {:>8}  {:>10}  {:>10}  {}",
        "setting", "ref_ok", "matched", "wall_sec", "effective", ""
    );
    println!("{}", "-".repeat(70));
    let mut highest_ok: Option<f32> = None;
    for r in &results {
        let ok = r.reference_ok && r.matched == r.iterations;
        if ok {
            highest_ok = Some(r.speed);
        }
        println!(
            "{:>8.1}  {:>6}  {:>4}/{:<3}  {:>9.2}s  {:>9.2}×  {}",
            r.speed,
            if r.reference_ok { "yes" } else { "NO" },
            r.matched,
            r.iterations,
            r.ref_wall_secs,
            r.effective_x,
            if ok { "PASS" } else { "FAIL" },
        );
    }
    println!();
    match highest_ok {
        Some(s) => {
            println!(
                "*** Fastest speed where all {}/{} iterations matched the reference: {}× ***",
                DEFAULT_ITERATIONS_PER_SPEED, DEFAULT_ITERATIONS_PER_SPEED, s
            );
            true
        }
        None => {
            println!("*** No speed produced a fully-matching set of verification iterations ***");
            false
        }
    }
}
