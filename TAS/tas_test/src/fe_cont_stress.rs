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
//! the reference cycle plus N verification cycles and reports how many
//! match the reference on the FIRST attempt — i.e. the user experience
//! of pressing "cont" once with no F5 retries. Anything less than N/N
//! means the user will sometimes see a "wrong" trajectory after pressing
//! cont and have to restart.

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
/// Max bucket-mismatch rerolls per iteration when auto-reroll is enabled.
/// At ~50ms per reroll detection and ~15% miss rate, 5 rerolls gives a
/// theoretical (1 - 0.15^5) ≈ 99.99% success bound.
const MAX_REROLLS: u32 = 5;
const RESTART_TIMEOUT_SECS: u64 = 15;

#[derive(Clone, Copy)]
struct SpeedResult {
    speed: f32,
    reference_ok: bool,
    /// Iterations that matched the reference on the FIRST attempt
    /// (no reroll). This is what the user experiences pressing "cont"
    /// once with the auto-reroll feature disabled.
    one_shot_matched: u32,
    /// Iterations that matched the reference within MAX_REROLLS bucket
    /// rerolls. With auto-reroll enabled this is the user-visible
    /// reliability — they always see the correct trajectory, after at
    /// most a brief delay while bad buckets get re-rolled.
    eventual_matched: u32,
    /// Total bucket-mismatch rerolls used across all iterations.
    /// Divided by iterations = average reroll cost per cont.
    total_rerolls: u32,
    iterations: u32,
    /// Reference-splice wall-clock to reach VERIFY_FRAMES (seconds). A VALID
    /// relative measurement: the summary divides the 1× run's wall by this to
    /// report the effective speedup. (The old per-speed `effective_x` divided
    /// by VERIFY_FRAMES/100, which wrongly treated render-cycle frames as 0.01s
    /// physics ticks — catchup_speed.rs has the correct T_1x/T_Nx metric.)
    ref_wall_secs: f64,
    /// First-moving frame in the reference (bucket fingerprint).
    expected_first_moving: Option<usize>,
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
fn arm_continue_and_wait_for_verify(client: &mut tas_shared::TasSharedMemoryClient) -> (bool, f64) {
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

/// Find the first frame index where the player position differs from
/// `coords[0]`. Used as a "bucket fingerprint" — the F5-restart timing
/// determines whether the first post-restart frame computed 0 or 1 ticks,
/// which manifests as a 1-frame phase offset in when the countdown
/// completes and the player first moves. Comparing the observed first-
/// moving frame in a fresh run against the reference's first-moving
/// frame tells us if we're in the right bucket *long* before frame 2200.
fn first_moving_frame(coords: &[[f32; 3]]) -> Option<usize> {
    let spawn = coords[0];
    for (j, c) in coords.iter().enumerate().skip(1) {
        if c[0].to_bits() != spawn[0].to_bits()
            || c[1].to_bits() != spawn[1].to_bits()
            || c[2].to_bits() != spawn[2].to_bits()
        {
            return Some(j);
        }
    }
    None
}

fn matches_reference(
    client: &tas_shared::TasSharedMemoryClient,
    reference: &[[f32; 3]],
) -> Option<usize> {
    let state = client.state();
    for (j, r) in reference
        .iter()
        .copied()
        .enumerate()
        .take(VERIFY_FRAMES as usize)
    {
        let p = state.play_coords[j];
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
        return SpeedResult {
            speed,
            reference_ok: false,
            one_shot_matched: 0,
            eventual_matched: 0,
            total_rerolls: 0,
            iterations: DEFAULT_ITERATIONS_PER_SPEED,
            ref_wall_secs: 0.0,
            expected_first_moving: None,
        };
    }
    // CMD_RESTART zeros continue_from_frame; re-write so ARM_CONTINUE's
    // validity check sees the right value.
    client.state_mut().continue_from_frame = SPLICE_FRAME;
    let (verify_ok, ref_wall) = arm_continue_and_wait_for_verify(client);
    if !verify_ok {
        println!(
            "  Reference ARM_CONTINUE didn't reach frame {}",
            VERIFY_FRAMES
        );
        harness::stop(client);
        return SpeedResult {
            speed,
            reference_ok: false,
            one_shot_matched: 0,
            eventual_matched: 0,
            total_rerolls: 0,
            iterations: DEFAULT_ITERATIONS_PER_SPEED,
            ref_wall_secs: 0.0,
            expected_first_moving: None,
        };
    }
    println!(
        "  Reference splice: {:.2}s wall to reach {} frames (speedup vs 1× computed in the summary)",
        ref_wall, VERIFY_FRAMES
    );
    let reference = capture_play_prefix(client);
    println!(
        "  Reference: start=({:.4}, {:.4}, {:.4})  anchor[{}]=({:.4}, {:.4}, {:.4})",
        reference[0][0],
        reference[0][1],
        reference[0][2],
        VERIFY_FRAMES - 1,
        reference[(VERIFY_FRAMES - 1) as usize][0],
        reference[(VERIFY_FRAMES - 1) as usize][1],
        reference[(VERIFY_FRAMES - 1) as usize][2],
    );
    // Let prefix finish or just stop (we have what we need)
    harness::stop(client);
    thread::sleep(Duration::from_millis(200));

    // ---- Bucket-detection auto-reroll ----
    // Compute the reference's first-moving frame — this is the "bucket
    // fingerprint" we compare each iteration's first-moving frame against.
    // If they don't match, we're in a wrong bucket and need to re-roll
    // F5 before doing the expensive full bit-comparison.
    let expected_first_moving = first_moving_frame(&reference);
    println!(
        "  Reference first-moving frame: {}",
        match expected_first_moving {
            Some(f) => f.to_string(),
            None => "(never moves?)".to_string(),
        }
    );

    let mut one_shot_matched = 0u32;
    let mut eventual_matched = 0u32;
    let mut total_rerolls = 0u32;

    for i in 0..DEFAULT_ITERATIONS_PER_SPEED {
        let mut iter_matched = false;
        for attempt in 0..MAX_REROLLS {
            replay::write_to_shared(client, rec);
            client.state_mut().playback_speed = speed;
            client.state_mut().continue_from_frame = SPLICE_FRAME;

            if !wait_restart_complete(client) {
                println!("  Iter {}: restart timeout — skipping attempt", i + 1);
                break;
            }
            client.state_mut().continue_from_frame = SPLICE_FRAME;
            let (verify_ok, _wall) = arm_continue_and_wait_for_verify(client);
            if !verify_ok {
                println!("  Iter {}: didn't reach verify frame", i + 1);
                harness::stop(client);
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            // Bucket check: compare the observed first-moving frame
            // against the reference's. Cheap (just walks play_coords
            // until first non-spawn position) and discriminates buckets
            // long before any chaos accumulates.
            let observed_play = client.state().play_coords[..VERIFY_FRAMES as usize].to_vec();
            let observed_first_moving = first_moving_frame(&observed_play);

            if observed_first_moving != expected_first_moving {
                // Wrong bucket. Re-roll without doing the full comparison.
                if attempt == 0 {
                    // First attempt: print details so the run log is informative.
                    println!(
                        "  Iter {:>2}: REROLL   bucket mismatch (first_moving={:?} vs ref={:?})",
                        i + 1,
                        observed_first_moving,
                        expected_first_moving,
                    );
                }
                total_rerolls += 1;
                harness::stop(client);
                thread::sleep(Duration::from_millis(200));
                continue;
            }

            // Right bucket — verify full bit-identical match.
            match matches_reference(client, &reference) {
                None => {
                    let last_idx = (VERIFY_FRAMES - 1) as usize;
                    let p = client.state().play_coords[last_idx];
                    let label = if attempt == 0 { "MATCH   " } else { "MATCH(R)" };
                    println!(
                        "  Iter {:>2}: {} end=({:.4},{:.4},{:.4})  attempt={}",
                        i + 1,
                        label,
                        p[0],
                        p[1],
                        p[2],
                        attempt + 1,
                    );
                    if attempt == 0 {
                        one_shot_matched += 1;
                    }
                    eventual_matched += 1;
                    iter_matched = true;
                    harness::stop(client);
                    thread::sleep(Duration::from_millis(200));
                    break;
                }
                Some(diverge_frame) => {
                    // Same bucket fingerprint but still diverges. Shouldn't
                    // happen if the bucket hypothesis is complete — log it
                    // and count as a reroll.
                    println!(
                        "  Iter {:>2}: BUCKET-OK BUT DIVERGES@{}  attempt={} — unexpected",
                        i + 1,
                        diverge_frame,
                        attempt + 1,
                    );
                    total_rerolls += 1;
                    harness::stop(client);
                    thread::sleep(Duration::from_millis(200));
                }
            }
        }
        if !iter_matched {
            println!("  Iter {:>2}: FAILED after {} rerolls", i + 1, MAX_REROLLS);
        }
    }

    SpeedResult {
        speed,
        reference_ok: true,
        one_shot_matched,
        eventual_matched,
        total_rerolls,
        iterations: DEFAULT_ITERATIONS_PER_SPEED,
        ref_wall_secs: ref_wall,
        expected_first_moving,
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
        "{:>8}  {:>6}  {:>9}  {:>9}  {:>8}  {:>9}  {:>10}",
        "setting", "ref_ok", "one_shot", "eventual", "rerolls", "wall_sec", "vs_1x"
    );
    println!("{}", "-".repeat(78));
    // Honest relative speedup: the 1× run's wall to the same splice frame,
    // divided by this speed's wall. (catchup_speed.rs is the asserted version;
    // this column is informational across the whole sweep.)
    let base_wall = results
        .iter()
        .find(|r| (r.speed - 1.0).abs() < 0.01 && r.reference_ok && r.ref_wall_secs > 0.0)
        .map(|r| r.ref_wall_secs);
    let mut all_eventual_ok = true;
    for r in &results {
        let one_shot_pct = 100.0 * r.one_shot_matched as f64 / r.iterations as f64;
        let eventual_ok = r.reference_ok && r.eventual_matched == r.iterations;
        if !eventual_ok {
            all_eventual_ok = false;
        }
        let speedup = match base_wall {
            Some(b) if r.ref_wall_secs > 0.0 => format!("{:.1}×", b / r.ref_wall_secs),
            _ => "—".to_string(),
        };
        println!(
            "{:>8.1}  {:>6}  {:>2}/{:<2} {:>3.0}%  {:>4}/{:<3}  {:>7}  {:>8.2}s  {:>10}{}",
            r.speed,
            if r.reference_ok { "yes" } else { "NO" },
            r.one_shot_matched,
            r.iterations,
            one_shot_pct,
            r.eventual_matched,
            r.iterations,
            r.total_rerolls,
            r.ref_wall_secs,
            speedup,
            if eventual_ok { "  PASS" } else { "  FAIL" },
        );
    }
    println!();
    println!("one_shot = matched first try, no reroll (= what user sees pressing cont once)");
    println!(
        "eventual = matched within {} bucket-detection rerolls (= auto-reroll feature)",
        MAX_REROLLS
    );
    println!();
    all_eventual_ok
}
