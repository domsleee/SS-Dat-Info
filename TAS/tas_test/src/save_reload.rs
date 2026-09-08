//! End-to-end save/reload/replay test: the workflow tas_ui exercises in real
//! use, and the one no other mode covers.
//!
//!   1. Record a fresh trajectory in this session (via Pico HID).
//!   2. Save the recording to a `.tasrec` file on disk.
//!   3. Kill the game.
//!   4. Revive — fresh game, fresh DLL injection, fresh shared memory.
//!   5. Load the `.tasrec` from disk into shared memory and replay it.
//!   6. Verify zero drift (rec_coords vs play_coords bit-identical).
//!
//! Fails if anything in the persistence path loses state the replay needs.

use std::thread;
use std::time::Duration;

use serde_json::json;

use crate::drift;
use crate::gates;
use crate::harness;
use crate::patterns;
use crate::replay;

/// Long enough that the 1000-frame trajectory match covers real post-input
/// motion: pattern + tail is ~1200 ticks, so the match window closes with the
/// recording still running.
const PATTERN: &str = "LR";
const HOLD_TICKS: u32 = 500;
const TAIL_NEUTRAL_TICKS: u32 = 200;

pub fn run() -> bool {
    println!("=== Save / Reload / Replay End-to-End Test ===\n");

    println!("--- Phase 1: Record fresh trajectory ---");
    let mut client = harness::ensure_game_running();
    harness::ensure_exclusive_runtime_ownership(&mut client, "save/reload recording");
    harness::print_status(&client);

    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: Game not alive for REC");
        return false;
    }

    harness::arm_rec(&mut client);

    let steps = patterns::with_neutral_tail(
        patterns::build_from_pattern(PATTERN, HOLD_TICKS, 0),
        TAIL_NEUTRAL_TICKS,
    );
    println!(
        "  Driving Pico HID: {} hold={} + {}t tail ({} total ticks)",
        PATTERN,
        HOLD_TICKS,
        TAIL_NEUTRAL_TICKS,
        patterns::total_ticks(&steps)
    );
    if let Err(error) = harness::drive_pico_steps(&steps) {
        eprintln!("{error}");
        harness::stop(&mut client);
        return false;
    }
    thread::sleep(Duration::from_millis(200));

    let rec_count = client.state().recorded_count;
    let rec_start = client.state().rec_coords[0];
    harness::stop(&mut client);

    if rec_count < 100 {
        eprintln!("ERROR: Too few ticks recorded ({})", rec_count);
        return false;
    }
    if let Err(error) =
        patterns::verify_capture(&steps, &client.state().input_log[..rec_count as usize], 12)
    {
        eprintln!("ERROR: Input capture: {error}");
        return false;
    }

    let transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);
    println!(
        "  Recorded {} ticks, {} transitions, start=({:.4}, {:.4}, {:.4})",
        rec_count, transitions, rec_start[0], rec_start[1], rec_start[2]
    );

    let out_dir = crate::output_dir();
    let tasrec_path = out_dir.join("save_reload_test.tasrec");
    println!(
        "\n--- Phase 2: Save recording to {} ---",
        tasrec_path.display()
    );

    let state = client.state();
    let count = state.recorded_count as usize;
    let input_log: Vec<u8> = state.input_log[..count].to_vec();
    let rec_coords: Vec<[f32; 3]> = state.rec_coords[..count].to_vec();

    let meta_value = json!({
        "version": state.version,
        "recorded_count": state.recorded_count,
        "force_fixed_tick": state.force_fixed_tick,
        "max_drift_x": 0.0_f32,
        "max_drift_z": 0.0_f32,
        "timestamp": "save_reload_test",
        "notes": "synthetic recording for save/reload e2e test",
        "segments": [],
    });

    if let Err(e) = replay::save_tasrec(&tasrec_path, &meta_value, &input_log, &rec_coords) {
        eprintln!("ERROR: Failed to save .tasrec: {}", e);
        return false;
    }
    println!("  Saved {} ticks to disk", count);

    println!("\n--- Phase 3: Kill game (force fresh state) ---");
    drop(client);
    harness::kill_game();
    thread::sleep(Duration::from_millis(800));

    println!("\n--- Phase 4: Revive game + reload recording ---");
    let mut client = harness::ensure_game_running();
    // Revival may start a new UI that owns COM7 and the command channel.
    harness::ensure_exclusive_runtime_ownership(&mut client, "save/reload replay");
    harness::print_status(&client);

    let loaded = match replay::load_tasrec(&tasrec_path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: Failed to reload .tasrec: {}", e);
            return false;
        }
    };
    println!(
        "  Reloaded {} ticks from {}",
        loaded.count,
        tasrec_path.display()
    );
    if loaded.count as usize != count
        || loaded.input_log != input_log
        || loaded.rec_coords != rec_coords
    {
        eprintln!("ERROR: Saved recording did not round-trip exactly");
        return false;
    }

    replay::write_to_shared(&mut client, &loaded);

    println!("\n--- Phase 5: Replay reloaded recording ---");
    let Some((rec_gate, play_gate)) = harness::restart_play_aligned_inprocess(&mut client) else {
        eprintln!("ERROR: Reloaded recording failed product PLAY alignment");
        return false;
    };
    let expected_end = play_gate.saturating_add(loaded.count.saturating_sub(rec_gate));
    let play_ok = harness::wait_playback(&client, expected_end);
    if !play_ok {
        eprintln!("ERROR: Playback did not complete");
        return false;
    }

    let state = client.state();
    let assessment = gates::run_gates_aligned(state, loaded.count, rec_gate, play_gate);
    assessment.print_summary();
    let zero_drift = assessment.all_pass();
    println!(
        "\n*** SAVE/RELOAD/REPLAY {}: complete gate-relative comparison across game restart ***",
        if zero_drift { "PASSED" } else { "FAILED" }
    );

    zero_drift
}
