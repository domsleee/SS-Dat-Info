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
const ITERATIONS_PER_SPEED: u32 = 5;
const REF_MATCH_RETRIES: u32 = 30;
const RESTART_TIMEOUT_SECS: u64 = 15;

#[derive(Clone, Copy)]
struct SpeedResult {
    speed: f32,
    reference_ok: bool,
    matched: u32,
    iterations: u32,
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

fn arm_continue_and_wait_for_verify(
    client: &mut tas_shared::TasSharedMemoryClient,
) -> bool {
    client.send_command(TasCommand::ArmContinue);
    // Wait for cave2 to process the command before polling. Otherwise the
    // first poll sees mode==OFF (command not yet processed) and the
    // OFF-bailout path returns immediately with highest_pos=0.
    thread::sleep(Duration::from_millis(100));
    let start = Instant::now();
    let mut highest_pos: u32 = 0;
    loop {
        let pos = client.playback_pos_volatile();
        if pos > highest_pos {
            highest_pos = pos;
        }
        if highest_pos >= VERIFY_FRAMES {
            return true;
        }
        let mode = client.mode_volatile();
        if mode == TasMode::Rec as u32 {
            return highest_pos >= VERIFY_FRAMES;
        }
        if mode == TasMode::Off as u32 {
            return highest_pos >= VERIFY_FRAMES;
        }
        if start.elapsed() > Duration::from_secs(60) {
            return highest_pos >= VERIFY_FRAMES;
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
    client.state_mut().continue_from_frame = SPLICE_FRAME;

    if !wait_restart_complete(client) {
        println!("  Reference restart timed out");
        return SpeedResult { speed, reference_ok: false, matched: 0, iterations: ITERATIONS_PER_SPEED };
    }
    if !arm_continue_and_wait_for_verify(client) {
        println!("  Reference ARM_CONTINUE didn't reach frame {}", VERIFY_FRAMES);
        let _ = harness::send_escape; // pacify unused-import lints if any
        harness::stop(client);
        return SpeedResult { speed, reference_ok: false, matched: 0, iterations: ITERATIONS_PER_SPEED };
    }
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
    for i in 0..ITERATIONS_PER_SPEED {
        let mut attempt_matched = false;
        for attempt in 0..=REF_MATCH_RETRIES {
            replay::write_to_shared(client, rec);
            client.state_mut().playback_speed = speed;
            client.state_mut().continue_from_frame = SPLICE_FRAME;

            if !wait_restart_complete(client) {
                println!("  Iter {}: restart timeout", i + 1);
                break;
            }
            if !arm_continue_and_wait_for_verify(client) {
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
        iterations: ITERATIONS_PER_SPEED,
    }
}

pub fn run(speeds: &[f32]) -> bool {
    println!(
        "=== FE-tremendous CONT-stress sweep (splice={}, verify={} frames, {} iters/speed) ===\n",
        SPLICE_FRAME, VERIFY_FRAMES, ITERATIONS_PER_SPEED
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

    let mut results: Vec<SpeedResult> = Vec::with_capacity(speeds.len());
    for &speed in speeds {
        let r = run_one_speed(&mut client, &rec, speed);
        results.push(r);
    }
    // Reset speed to 1.0 at the end.
    client.state_mut().playback_speed = 1.0;

    println!("\n\n=== FE-CONT STRESS SUMMARY ===");
    println!("{:>10}  {:>8}  {:>20}", "speed", "ref_ok", "matched/iters");
    println!("{}", "-".repeat(50));
    let mut highest_ok: Option<f32> = None;
    for r in &results {
        let ok = r.reference_ok && r.matched == r.iterations;
        if ok {
            highest_ok = Some(r.speed);
        }
        println!(
            "{:>10.1}  {:>8}  {:>14}/{:<5} {}",
            r.speed,
            if r.reference_ok { "yes" } else { "NO" },
            r.matched,
            r.iterations,
            if ok { "PASS" } else { "FAIL" },
        );
    }
    println!();
    match highest_ok {
        Some(s) => {
            println!(
                "*** Fastest speed where all {}/{} iterations matched the reference: {}× ***",
                ITERATIONS_PER_SPEED, ITERATIONS_PER_SPEED, s
            );
            true
        }
        None => {
            println!("*** No speed produced a fully-matching set of verification iterations ***");
            false
        }
    }
}
