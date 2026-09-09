//! Physically pause a clean, product-aligned replay and require zero drift
//! across its complete gate-relative trajectory, including the resumed tail.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasMode;

use crate::{gates, harness, patterns};

const PAUSE_AT_FRAME: u32 = 1200;
const PAUSE_DURATION_SECS: u64 = 4;

fn pause_window_valid(
    position_before: u32,
    position_after: u32,
    cycles_before: u32,
    cycles_after: u32,
    expected_end: u32,
) -> bool {
    position_before >= PAUSE_AT_FRAME
        && position_after >= position_before
        && position_after - position_before <= 1
        && cycles_after.wrapping_sub(cycles_before) <= 1
        && expected_end.saturating_sub(position_after) >= 100
}

pub fn run() -> bool {
    println!("=== Pause / Resume Replay Test ===");
    let mut client = harness::ensure_game_running();
    harness::ensure_exclusive_runtime_ownership(&mut client, "pause/resume");
    if !harness::restart_and_stabilize_inprocess(&mut client) {
        return false;
    }
    harness::arm_rec(&mut client);
    let steps = patterns::with_neutral_tail(patterns::build_from_pattern("LRLR", 500, 0), 200);
    if let Err(error) = harness::drive_pico_steps(&steps) {
        eprintln!("ERROR: {error}");
        harness::stop(&mut client);
        return false;
    }
    thread::sleep(Duration::from_millis(200));
    harness::stop(&mut client);
    let rec_count = client.recorded_count_volatile();
    if let Err(error) =
        patterns::verify_capture(&steps, &client.state().input_log[..rec_count as usize], 12)
    {
        eprintln!("ERROR: Input capture: {error}");
        return false;
    }
    let Some((rec_gate, play_gate)) = harness::restart_play_aligned_inprocess(&mut client) else {
        eprintln!("ERROR: Product PLAY alignment failed before pause");
        return false;
    };
    let expected_end = play_gate.saturating_add(rec_count.saturating_sub(rec_gate));
    let deadline = Instant::now() + Duration::from_secs(30);
    while client.playback_pos_volatile() < PAUSE_AT_FRAME {
        if Instant::now() >= deadline || client.mode_volatile() != TasMode::Play as u32 {
            eprintln!("ERROR: Playback did not reach the pause point");
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }

    if !harness::send_escape() {
        return false;
    }
    thread::sleep(Duration::from_millis(200));
    let pos_at_pause = client.playback_pos_volatile();
    let fc_at_pause = client.frame_count_volatile();
    let prefix_end = rec_gate
        .saturating_add(pos_at_pause.saturating_sub(play_gate))
        .min(rec_count);
    let prefix = gates::run_gates_aligned(client.state(), prefix_end, rec_gate, play_gate);
    println!("Pre-pause trajectory, through replay frame {pos_at_pause}:");
    prefix.print_summary();

    thread::sleep(Duration::from_secs(PAUSE_DURATION_SECS));
    let pos_after_pause = client.playback_pos_volatile();
    let fc_after_pause = client.frame_count_volatile();
    let paused = pause_window_valid(
        pos_at_pause,
        pos_after_pause,
        fc_at_pause,
        fc_after_pause,
        expected_end,
    ) && client.mode_volatile() == TasMode::Play as u32;
    println!(
        "Pause hold: position {pos_at_pause}->{pos_after_pause}, cycles {fc_at_pause}->{fc_after_pause}, valid={paused}"
    );
    // Resume even if the prefix verdict failed, so diagnostics do not leave
    // the game parked in its pause menu.
    if !harness::send_escape() {
        return false;
    }
    if !prefix.all_pass() || !paused {
        eprintln!("ERROR: Invalid pre-resume baseline; cannot attribute drift to pause/resume");
        harness::stop(&mut client);
        return false;
    }
    if !harness::wait_playback(&client, expected_end) {
        eprintln!("ERROR: Playback did not complete after resume");
        return false;
    }
    let assessment = gates::run_gates_aligned(client.state(), rec_count, rec_gate, play_gate);
    assessment.print_summary();
    println!(
        "PAUSE/RESUME {}: full trajectory, including {} ticks after pause",
        if assessment.all_pass() {
            "PASSED"
        } else {
            "FAILED"
        },
        expected_end.saturating_sub(pos_after_pause)
    );
    assessment.all_pass()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pause_requires_frozen_playback_and_cycles_with_a_real_tail() {
        assert!(pause_window_valid(1200, 1200, 5000, 5000, 2200));
        assert!(pause_window_valid(1200, 1201, u32::MAX, 0, 2200));
        assert!(!pause_window_valid(1200, 1202, 5000, 5000, 2200));
        assert!(!pause_window_valid(1200, 1200, 5000, 5040, 2200));
        assert!(!pause_window_valid(1200, 1199, 5000, 5000, 2200));
        assert!(!pause_window_valid(1199, 1199, 5000, 5000, 2200));
        assert!(!pause_window_valid(2200, 2200, 5000, 5000, 2200));
    }
}
