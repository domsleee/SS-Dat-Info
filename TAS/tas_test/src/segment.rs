//! Two-segment CONT: REC a LEFT segment, F5-match a CONT into a RIGHT segment,
//! then PLAY the whole recording and require zero drift at the boundary.
//!
//! The CONT reuses the F5 that matched the recording's start, so no second
//! restart is needed between the match and the splice.

use crate::{gates, harness, patterns};
use tas_shared::input_bits;

/// Segment 0 is 200 ticks of LEFT plus a 100-tick neutral tail; the splice
/// lands at the end of the LEFT hold.
const SPLICE_FRAME: u32 = 200;

pub fn run() -> bool {
    println!("=== Multi-Segment E2E Zero-Drift Test ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    println!("\n--- Phase 1: REC segment 0 (LEFT steering) ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive for Phase 1");
        return false;
    }

    harness::arm_rec(&mut client);
    println!("  Recording with LEFT steering via Pico HID...");
    let seg0_steps =
        patterns::build_from_explicit(&[(input_bits::LEFT, SPLICE_FRAME), (0x00, 100)]);
    if let Err(error) = harness::drive_pico_steps(&seg0_steps) {
        eprintln!("{error}");
        harness::stop(&mut client);
        return false;
    }

    let seg0_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Segment 0 recorded: {} ticks", seg0_count);

    if seg0_count < SPLICE_FRAME {
        eprintln!("ERROR: Too few ticks in segment 0 (need >= {SPLICE_FRAME})");
        return false;
    }

    let rec_start = client.state().rec_coords[0];
    println!(
        "  REC start: ({:.4}, {:.4}, {:.4})",
        rec_start[0], rec_start[1], rec_start[2]
    );
    println!("  Segments before CONT: {}", client.state().segment_count);

    println!(
        "\n--- Phase 2: CONT from frame {} (RIGHT steering) ---",
        SPLICE_FRAME
    );
    if harness::restart_continue_and_splice(&mut client, rec_start, SPLICE_FRAME, 30).is_none() {
        eprintln!("ERROR: Could not position-match for CONT after retries");
        return false;
    }

    println!("  Recording segment 1 with RIGHT steering via Pico HID...");
    let seg1_steps = patterns::build_from_explicit(&[(input_bits::RIGHT, 200), (0x00, 100)]);
    if let Err(error) = harness::drive_pico_steps(&seg1_steps) {
        eprintln!("{error}");
        harness::stop(&mut client);
        return false;
    }

    let total_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("  Total recorded after CONT: {} ticks", total_count);
    println!("  Segment count: {}", client.state().segment_count);

    let state = client.state();
    for i in 0..state.segment_count as usize {
        let b = &state.segment_boundaries[i];
        println!(
            "  Boundary[{}]: frame={} input_log_offset={}",
            i, b.frame, b.input_log_offset
        );
    }

    let log = &state.input_log[..total_count as usize];
    let has_left = log.iter().any(|m| m & input_bits::LEFT != 0);
    let has_right = log.iter().any(|m| m & input_bits::RIGHT != 0);
    println!(
        "  Input log check: has_left={} has_right={} (both expected)",
        has_left, has_right
    );
    if !has_left || !has_right {
        eprintln!("ERROR: Input log missing expected LEFT or RIGHT inputs");
        return false;
    }

    println!(
        "\n--- Phase 3: PLAY full recording ({} ticks) ---",
        total_count
    );
    if !harness::restart_play_and_match(&mut client, rec_start, harness::START_MATCH_RETRIES) {
        eprintln!("FAIL: Could not match position for PLAY");
        harness::stop(&mut client);
        return false;
    }
    if !harness::wait_playback(&client, total_count) {
        eprintln!("FAIL: Playback did not complete normally");
        harness::stop(&mut client);
        return false;
    }

    harness::print_results(&client);
    let state = client.state();

    if SPLICE_FRAME < total_count {
        let sf = SPLICE_FRAME as usize;
        let rec = state.rec_coords[sf];
        let play = state.play_coords[sf];
        let boundary_drift: Vec<f64> = (0..3)
            .map(|axis| (rec[axis] as f64 - play[axis] as f64).abs())
            .collect();
        println!("\n--- Segment Boundary (frame {}) ---", SPLICE_FRAME);
        println!(
            "  REC[{}]:  ({:.6}, {:.6}, {:.6})",
            sf, rec[0], rec[1], rec[2]
        );
        println!(
            "  PLAY[{}]: ({:.6}, {:.6}, {:.6})",
            sf, play[0], play[1], play[2]
        );
        println!(
            "  Boundary drift: X={:.9} Y={:.9} Z={:.9}",
            boundary_drift[0], boundary_drift[1], boundary_drift[2]
        );
        if boundary_drift.iter().all(|d| *d == 0.0) {
            println!("  Boundary check: PASS (zero discontinuity)");
        } else {
            println!("  Boundary check: FAIL (drift at segment boundary)");
        }
    }

    let assessment = gates::run_gates(state, total_count);
    assessment.print_summary();

    println!("\n=== SEGMENT TEST VERDICT ===");
    if assessment.all_pass() {
        println!("*** MULTI-SEGMENT ZERO-DRIFT TEST PASSED ***");
        println!(
            "  {} ticks, {} segments, splice at frame {}",
            total_count, state.segment_count, SPLICE_FRAME
        );
    } else {
        println!("*** MULTI-SEGMENT ZERO-DRIFT TEST FAILED ***");
    }
    assessment.all_pass()
}
