//! Two-segment CONT through product transport: splice a LEFT recording into
//! RIGHT steering, then require exact metadata and gate-relative zero drift.

use crate::{gates, harness, patterns};
use tas_shared::input_bits;

/// Past the countdown and alignment watch window, but inside the LEFT hold.
const SPLICE_FRAME: u32 = 500;

fn segment_structure_passes(state: &tas_shared::TasSharedState, count: u32) -> bool {
    let end = count as usize;
    let splice = SPLICE_FRAME as usize;
    count > SPLICE_FRAME
        && end <= state.input_log.len()
        && state.segment_count == 2
        && state.segment_boundaries[0].frame == 0
        && state.segment_boundaries[0].input_log_offset == 0
        && state.segment_boundaries[1].frame == SPLICE_FRAME
        && state.segment_boundaries[1].input_log_offset == SPLICE_FRAME
        && state.input_log[splice - 1] & input_bits::LEFT != 0
        && !state.input_log[..splice]
            .iter()
            .any(|m| m & input_bits::RIGHT != 0)
        && state.input_log[splice..end]
            .iter()
            .any(|m| m & input_bits::RIGHT != 0)
        && state.input_log[end - 1] == 0
}

pub fn run() -> bool {
    println!("=== Multi-Segment E2E Zero-Drift Test ===\n");
    let mut client = harness::ensure_game_running();
    harness::ensure_exclusive_runtime_ownership(&mut client, "multi-segment recording");
    client.state_mut().playback_speed = 1.0;
    harness::print_status(&client);

    println!("\n--- Phase 1: REC segment 0 (LEFT steering) ---");
    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: Game not alive for Phase 1");
        return false;
    }

    harness::arm_rec(&mut client);
    println!("  Recording with LEFT steering via Pico HID...");
    let seg0_steps = patterns::build_from_explicit(&[(0, 350), (input_bits::LEFT, 200), (0, 100)]);
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
    if let Err(error) = patterns::verify_capture(
        &seg0_steps,
        &client.state().input_log[..seg0_count as usize],
        12,
    ) {
        eprintln!("ERROR: Segment 0 input: {error}");
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
    client.state_mut().cont_resume_speed = 1.0;
    if harness::restart_continue_and_splice_inprocess(&mut client, rec_start, SPLICE_FRAME, 30)
        .is_none()
    {
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
    if !segment_structure_passes(state, total_count) {
        eprintln!("ERROR: Expected two segments at 0/{SPLICE_FRAME}, LEFT before the splice, RIGHT after it and a released tail");
        return false;
    }
    for i in 0..state.segment_count as usize {
        let b = &state.segment_boundaries[i];
        println!(
            "  Boundary[{}]: frame={} input_log_offset={}",
            i, b.frame, b.input_log_offset
        );
    }

    println!(
        "\n--- Phase 3: PLAY full recording ({} ticks) ---",
        total_count
    );
    let Some((rec_gate, play_gate)) = harness::restart_play_aligned_inprocess(&mut client) else {
        eprintln!("FAIL: Could not match position for PLAY");
        harness::stop(&mut client);
        return false;
    };
    let expected_end = play_gate.saturating_add(total_count.saturating_sub(rec_gate));
    if !harness::wait_playback(&client, expected_end) {
        eprintln!("FAIL: Playback did not complete normally");
        harness::stop(&mut client);
        return false;
    }

    let state = client.state();

    if SPLICE_FRAME < total_count {
        let sf = SPLICE_FRAME as usize;
        let rec = state.rec_coords[sf];
        let play_frame = (play_gate + SPLICE_FRAME - rec_gate) as usize;
        let play = state.play_coords[play_frame];
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
            play_frame, play[0], play[1], play[2]
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

    let assessment = gates::run_gates_aligned(state, total_count, rec_gate, play_gate);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splice_metadata_and_input_sides_are_required() {
        let mut state = tas_shared::zeroed_boxed();
        state.segment_count = 2;
        state.segment_boundaries[1].frame = SPLICE_FRAME;
        state.segment_boundaries[1].input_log_offset = SPLICE_FRAME;
        state.input_log[499] = input_bits::LEFT;
        state.input_log[500] = input_bits::RIGHT;
        assert!(segment_structure_passes(&state, 600));
        state.segment_count = 1;
        assert!(!segment_structure_passes(&state, 600));
        state.segment_count = 2;
        state.segment_boundaries[1].frame += 1;
        assert!(!segment_structure_passes(&state, 600));
        state.segment_boundaries[1].frame = SPLICE_FRAME;
        state.segment_boundaries[1].input_log_offset += 1;
        assert!(!segment_structure_passes(&state, 600));
        state.segment_boundaries[1].input_log_offset = SPLICE_FRAME;
        state.input_log[500] = 0;
        assert!(!segment_structure_passes(&state, 600));
        state.input_log[500] = input_bits::RIGHT;
        state.input_log[499] = input_bits::RIGHT;
        assert!(!segment_structure_passes(&state, 600));
        assert!(!segment_structure_passes(&state, 500));
        assert!(!segment_structure_passes(&state, u32::MAX));
    }
}
