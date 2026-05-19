//! Pause/resume replay test.
//!
//! Records a fresh trajectory, then replays it with an Escape pause inserted
//! mid-playback: pause for several seconds, resume (game fast-forwards as it
//! consumes the accumulated wall-clock time), wait for playback to finish,
//! then verify the first 1000 frames of rec_coords vs play_coords are bit-
//! identical.
//!
//! Catches the regression the user reported originally: pause+resume with
//! cave hooks loaded was causing a visible speedup on resume that should
//! NOT affect deterministic playback. If physics actually re-runs the
//! catch-up ticks correctly under the recorded inputs, drift stays zero;
//! if cave5's tick handling or the time-advance constant gets perturbed
//! across the pause boundary, the snowboarder ends up off the recorded
//! trajectory after resume and this test fails.
//!
//! Pass criterion: exit code 0, "*** PAUSE/RESUME REPLAY PASSED ***" line.

use std::thread;
use std::time::Duration;

use crate::drift;
use crate::harness;
use crate::patterns;

const REC_DURATION_SECS: u64 = 25;
const PATTERN: &str = "LRLR";
const HOLD_TICKS: u32 = 500;
const TAIL_NEUTRAL_TICKS: u32 = 200;

/// How long (in game ticks at 1x speed) into playback to insert the pause.
const PAUSE_AT_FRAME: u32 = 500;
/// Wall-clock duration of the pause. Game is stopped during this time.
const PAUSE_DURATION_SECS: u64 = 4;
/// Number of leading frames the rec/play coords must agree on (matches the
/// matcher's `MATCH_VERIFY_FRAMES`).
const ZERO_DRIFT_FRAMES: u32 = 1000;

pub fn run() -> bool {
    println!("=== Pause / Resume Replay Test ===\n");

    // ---- Phase 1: Record a fresh trajectory ----
    println!("--- Phase 1: Record ---");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: Game not alive for REC");
        return false;
    }

    harness::arm_rec(&mut client);

    let mut steps = patterns::build_from_pattern(PATTERN, HOLD_TICKS, 0);
    let last_stop = patterns::total_ticks(&steps);
    steps.push(patterns::PatternStep {
        name: "TAIL".into(),
        mask: 0x00,
        stop_tick: last_stop + TAIL_NEUTRAL_TICKS,
    });
    println!(
        "  Driving Pico HID: {} hold={} + {}t tail ({} total ticks)",
        PATTERN,
        HOLD_TICKS,
        TAIL_NEUTRAL_TICKS,
        patterns::total_ticks(&steps)
    );
    harness::drive_pico_steps(&steps, Some(REC_DURATION_SECS * 1000));
    thread::sleep(Duration::from_millis(200));

    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);

    if rec_count < ZERO_DRIFT_FRAMES {
        eprintln!(
            "ERROR: Recording too short ({} < {}) — pattern needs to produce enough frames to verify",
            rec_count, ZERO_DRIFT_FRAMES
        );
        return false;
    }
    println!("  Recorded {} ticks", rec_count);

    // ---- Phase 2: Start playback ----
    println!("\n--- Phase 2: Start playback ---");
    let target = client.state().rec_coords[0];
    let matched = harness::restart_play_and_match_inprocess(
        &mut client,
        target,
        harness::START_MATCH_RETRIES,
    );
    if !matched {
        eprintln!("ERROR: Playback position match failed");
        return false;
    }
    println!("  Playback started");

    // ---- Phase 3: Wait for playback to reach the pause frame ----
    println!(
        "\n--- Phase 3: Wait for playback to reach frame {} ---",
        PAUSE_AT_FRAME
    );
    let wait_start = std::time::Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        if pos >= PAUSE_AT_FRAME {
            println!("  Reached frame {}", pos);
            break;
        }
        if wait_start.elapsed() > Duration::from_secs(60) {
            eprintln!("ERROR: Playback didn't reach pause frame within 60s (pos={})", pos);
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }

    // ---- Phase 4: Pause ----
    println!("\n--- Phase 4: Press Escape (pause) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Couldn't send Escape");
        return false;
    }
    thread::sleep(Duration::from_millis(200));
    let pos_at_pause = client.playback_pos_volatile();
    let fc_at_pause = client.frame_count_volatile();
    println!(
        "  Paused at playback_pos={} frame_count={}",
        pos_at_pause, fc_at_pause
    );

    println!("\n--- Phase 5: Hold pause for {}s ---", PAUSE_DURATION_SECS);
    thread::sleep(Duration::from_secs(PAUSE_DURATION_SECS));
    let pos_after_pause = client.playback_pos_volatile();
    let fc_after_pause = client.frame_count_volatile();
    let fc_delta = fc_after_pause as i64 - fc_at_pause as i64;
    let actually_paused = fc_delta < (PAUSE_DURATION_SECS as i64 * 10);
    println!(
        "  After pause: playback_pos={} (delta {}) frame_count={} (delta {}) — game {}",
        pos_after_pause,
        pos_after_pause as i64 - pos_at_pause as i64,
        fc_after_pause,
        fc_delta,
        if actually_paused {
            "actually paused"
        } else {
            "did NOT pause (simulated Escape didn't reach pause handler — game kept running)"
        }
    );

    // ---- Phase 6: Resume ----
    println!("\n--- Phase 6: Press Escape (resume — game may fast-forward) ---");
    if !harness::send_escape() {
        eprintln!("ERROR: Couldn't send Escape to resume");
        return false;
    }

    // ---- Phase 7: Wait for playback to complete ----
    println!("\n--- Phase 7: Wait for playback complete ---");
    let play_ok = harness::wait_playback(&client, rec_count);
    if !play_ok {
        eprintln!("ERROR: Playback did not complete normally");
        // Continue to drift check anyway so we report what we have.
    }

    // ---- Phase 8: Verify zero drift over first 1000 frames ----
    println!(
        "\n--- Phase 8: Verify zero drift over first {} frames ---",
        ZERO_DRIFT_FRAMES
    );
    let state = client.state();
    let check_end = ZERO_DRIFT_FRAMES.min(rec_count);
    let drift_result = drift::compute_drift(state, check_end);

    println!(
        "  Drift over frames 0..{}: X={:.9} (frame {}) Z={:.9} (frame {})",
        check_end,
        drift_result.max_drift_x,
        drift_result.max_drift_frame_x,
        drift_result.max_drift_z,
        drift_result.max_drift_frame_z
    );

    let mut first_div: Option<usize> = None;
    for i in 0..check_end as usize {
        let p = state.play_coords[i];
        let r = state.rec_coords[i];
        if p[0].to_bits() != r[0].to_bits()
            || p[1].to_bits() != r[1].to_bits()
            || p[2].to_bits() != r[2].to_bits()
        {
            first_div = Some(i);
            break;
        }
    }

    let pass = drift_result.is_zero();
    if pass {
        println!(
            "\n*** PAUSE/RESUME REPLAY PASSED: zero drift across pause+resume for {} frames ***",
            check_end
        );
    } else if let Some(i) = first_div {
        let p = state.play_coords[i];
        let r = state.rec_coords[i];
        println!(
            "\n  First divergence at frame {}: rec=({:.6},{:.6},{:.6}) play=({:.6},{:.6},{:.6})",
            i, r[0], r[1], r[2], p[0], p[1], p[2]
        );
        println!(
            "\n*** PAUSE/RESUME REPLAY FAILED: trajectory diverges across pause+resume ***"
        );
    } else {
        println!("\n*** PAUSE/RESUME REPLAY FAILED: drift > 0 but no bit-divergence found ***");
    }

    pass
}
