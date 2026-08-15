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
        "  Prefix drift over frames 0..{}: X={:.9} (frame {}) Y={:.9} Z={:.9} (frame {})",
        check_end,
        drift_result.max_drift_x,
        drift_result.max_drift_frame_x,
        drift_result.max_drift_y,
        drift_result.max_drift_z,
        drift_result.max_drift_frame_z
    );

    // The prefix window above is NOT the test. `restart_play_and_match_inprocess`
    // verifies the first MATCH_VERIFY_FRAMES (=ZERO_DRIFT_FRAMES) before it
    // returns, so by the time we wait for PAUSE_AT_FRAME the replay is already
    // past it — the pause lands at ~1089 with the window ending at 1000. Checking
    // [0,1000) therefore only re-checks frames that played BEFORE the pause, and
    // is structurally incapable of seeing a pause-induced divergence. The real
    // assertion is the window AFTER the pause point.
    // Anchor on the position AFTER the hold, not before it: that is the frame
    // playback actually resumes from. If the pause worked the two are identical
    // (nothing advanced); if it did not, the later value is the honest boundary
    // and the run fails on `actually_paused` regardless.
    let resume_anchor = pos_at_pause.max(pos_after_pause);
    let post_pause_drift = drift::compute_drift_window(state, resume_anchor, rec_count);
    let post_pause_frames = rec_count.saturating_sub(resume_anchor);
    println!(
        "  POST-PAUSE drift over frames {}..{} ({} frames — the window the pause can affect): \
         X={:.9} Y={:.9} Z={:.9}",
        resume_anchor,
        rec_count,
        post_pause_frames,
        post_pause_drift.max_drift_x,
        post_pause_drift.max_drift_y,
        post_pause_drift.max_drift_z
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

    // The drift check is only meaningful if a pause actually happened. Without
    // it the replay ran start-to-finish uninterrupted, which trivially yields
    // zero drift while testing nothing — a false green. `actually_paused` was
    // already measured above (frame_count stalled during the hold); gate on it.
    // Completion matters too: the drift window is only the first 1000 frames, so
    // a replay that renders that prefix and then stalls or drops out of PLAY
    // before rec_count would otherwise pass on a prefix that proves nothing about
    // the pause it was supposed to survive.
    let played_all = client.state().playback_pos >= rec_count;
    // A post-pause window that is empty means the pause landed at/after the end
    // of playback — there is nothing it could have perturbed, so the run proves
    // nothing and must not pass.
    let post_pause_window_ok = post_pause_frames > 0;
    let pass = drift_result.is_zero()
        && post_pause_drift.is_zero()
        && post_pause_window_ok
        && actually_paused
        && play_ok
        && played_all;
    if !post_pause_window_ok {
        println!(
            "\n*** PAUSE/RESUME REPLAY FAILED: the pause landed at frame {} but the recording is \
             only {} frames — there is no post-pause window to assess, so this run proves nothing \
             about pause/resume. ***",
            resume_anchor, rec_count
        );
    } else if !actually_paused {
        println!(
            "\n*** PAUSE/RESUME REPLAY FAILED: the game never paused — Escape did not reach the \
             pause handler (frame_count advanced {} during the {}s hold). The replay ran \
             uninterrupted, so the zero-drift result below proves nothing. This mode needs the \
             Pico HID; keybd_event/PostMessage do not reach the pause handler. ***",
            fc_delta, PAUSE_DURATION_SECS
        );
    } else if !play_ok || !played_all {
        println!(
            "\n*** PAUSE/RESUME REPLAY FAILED: playback did not complete after the resume \
             (playback_pos={} of {}). The pre-pause prefix cannot stand in for a failed resume. ***",
            client.state().playback_pos,
            rec_count
        );
    } else if pass {
        println!(
            "\n*** PAUSE/RESUME REPLAY PASSED: zero drift across pause+resume — prefix {} frames \
             AND {} frames after the pause at {} ***",
            check_end, post_pause_frames, resume_anchor
        );
    } else if !post_pause_drift.is_zero() {
        println!(
            "\n*** PAUSE/RESUME REPLAY FAILED: trajectory diverges AFTER the pause (frames {}..{}): \
             X={:.9} Y={:.9} Z={:.9} — this is the pause/resume regression. ***",
            resume_anchor,
            rec_count,
            post_pause_drift.max_drift_x,
            post_pause_drift.max_drift_y,
            post_pause_drift.max_drift_z
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
