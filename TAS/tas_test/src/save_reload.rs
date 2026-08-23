//! End-to-end save/reload/replay test.
//!
//! Validates that the full record-save-reload-replay workflow produces zero
//! drift across game restarts. This is the workflow tas_ui exercises in real
//! use, and the one no other test mode covers:
//!
//!   1. Record a fresh trajectory in this session (via Pico HID).
//!   2. Save the recording to a `.tasrec` file on disk.
//!   3. Kill the game (terminate Supreme.exe).
//!   4. Revive — fresh game, fresh DLL injection, fresh shared memory.
//!   5. Load the `.tasrec` from disk into shared memory.
//!   6. Replay it.
//!   7. Verify zero drift (rec_coords vs play_coords bit-identical).
//!
//! Pass criterion: exit code 0 + zero drift on the cross-session replay.
//!
//! Fails if anything in the persistence path corrupts state: incomplete
//! capture (e.g. rotation/velocity not preserved), reload bug, or any
//! mismatch between record-time and replay-time game state that can't be
//! reconstructed from the file alone.

use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

use serde_json::json;

use crate::drift;
use crate::harness;
use crate::patterns;
use crate::replay;

/// Record for long enough that the 1000-frame trajectory match exercises a
/// full window of post-stationary, post-initial-input motion — not just a
/// short blip. At 1x speed (100 ticks/s) we need ~11 seconds of game time
/// to fill 1100 ticks; pattern + tail yields ~1200 ticks so playback exits
/// match-verification with the recording still running.
const REC_DURATION_SECS: u64 = 13;
const PATTERN: &str = "LR";
const HOLD_TICKS: u32 = 500;
const TAIL_NEUTRAL_TICKS: u32 = 200;

pub fn run() -> bool {
    println!("=== Save / Reload / Replay End-to-End Test ===\n");

    // ---- Phase 1: Record fresh ----
    println!("--- Phase 1: Record fresh trajectory ---");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Use Pico F5 restart for REC. Both REC and PLAY then use the same
    // restart_and_stabilize timing (~16s), so the snowboarder slides the
    // same amount before rec_coords[0] / play_coords[0] is captured.
    // Position match alone is sufficient because at this slide endpoint
    // the rotation is determined by the F5 bucket (same as f5-probe shows
    // at F5 spawn — buckets are session-stable).
    if !harness::restart_and_stabilize(&client) {
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
    let rec_start = client.state().rec_coords[0];
    let rec_rotation_at_stop = client.state().rotation_matrix;
    let rec_velocity_at_stop = [
        client.state().velocity_x,
        client.state().velocity_y,
        client.state().velocity_z,
    ];
    println!(
        "  REC rotation[0..3] at stop: ({:.4}, {:.4}, {:.4})  velocity: ({:.6}, {:.6}, {:.6})",
        rec_rotation_at_stop[0],
        rec_rotation_at_stop[1],
        rec_rotation_at_stop[2],
        rec_velocity_at_stop[0],
        rec_velocity_at_stop[1],
        rec_velocity_at_stop[2]
    );
    harness::stop(&mut client);

    if rec_count < 100 {
        eprintln!("ERROR: Too few ticks recorded ({})", rec_count);
        return false;
    }

    let transitions = drift::count_transitions(&client.state().input_log, rec_count as usize);
    println!(
        "  Recorded {} ticks, {} transitions, start=({:.4}, {:.4}, {:.4})",
        rec_count, transitions, rec_start[0], rec_start[1], rec_start[2]
    );

    // ---- Phase 2: Save to disk ----
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
        "inject_mode": state.inject_mode,
        "force_fixed_tick": state.force_fixed_tick,
        "force_direct": state.force_direct,
        "input_source": state.input_source,
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

    // ---- Phase 3: Kill game, force fresh revive ----
    println!("\n--- Phase 3: Kill game (force fresh state) ---");
    drop(client);
    kill_game();
    thread::sleep(Duration::from_millis(800));

    // ---- Phase 4: Revive, reload from disk ----
    println!("\n--- Phase 4: Revive game + reload recording ---");
    let mut client = harness::ensure_game_running();
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

    replay::write_to_shared(&mut client, &loaded);

    let target = loaded.rec_coords[0];

    // ---- Phase 5: Replay against reloaded recording ----
    println!("\n--- Phase 5: Replay reloaded recording ---");
    // PLAY uses the same Pico F5 path as REC so slide timing matches.
    let matched =
        harness::restart_play_and_match(&mut client, target, harness::START_MATCH_RETRIES);
    if !matched {
        eprintln!(
            "ERROR: Position match failed after {} retries",
            harness::START_MATCH_RETRIES
        );
        return false;
    }
    let play_rotation_at_match = client.state().rotation_matrix;
    let play_velocity_at_match = [
        client.state().velocity_x,
        client.state().velocity_y,
        client.state().velocity_z,
    ];
    println!(
        "  PLAY rotation[0..3] at match: ({:.4}, {:.4}, {:.4})  velocity: ({:.6}, {:.6}, {:.6})",
        play_rotation_at_match[0],
        play_rotation_at_match[1],
        play_rotation_at_match[2],
        play_velocity_at_match[0],
        play_velocity_at_match[1],
        play_velocity_at_match[2]
    );

    let play_ok = harness::wait_playback(&client, loaded.count);
    if !play_ok {
        eprintln!("ERROR: Playback did not complete");
        return false;
    }

    let state = client.state();
    let drift_result = drift::compute_drift(state, loaded.count.min(state.playback_pos));

    println!(
        "\n  Drift: X={:.9} (frame {}) Z={:.9} (frame {})",
        drift_result.max_drift_x,
        drift_result.max_drift_frame_x,
        drift_result.max_drift_z,
        drift_result.max_drift_frame_z
    );

    // Find first divergence to characterize any failure mode.
    let played = loaded.count.min(state.playback_pos) as usize;
    let mut first_div: Option<usize> = None;
    for i in 0..played {
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

    let zero_drift = drift_result.is_zero();
    if zero_drift {
        println!("\n*** SAVE/RELOAD/REPLAY PASSED: zero drift across game restart ***");
    } else if let Some(i) = first_div {
        let p = state.play_coords[i];
        let r = state.rec_coords[i];
        println!(
            "\n  First divergence at frame {}: rec=({:.6}, {:.6}, {:.6}) play=({:.6}, {:.6}, {:.6})",
            i, r[0], r[1], r[2], p[0], p[1], p[2]
        );
        println!("\n*** SAVE/RELOAD/REPLAY FAILED: trajectory diverges across save+reload ***");
    } else {
        println!("\n*** SAVE/RELOAD/REPLAY FAILED: drift > 0 but no bit-divergence found (?) ***");
    }

    zero_drift
}

fn kill_game() {
    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-Process Supreme,tas_ui,Display_Config -ErrorAction SilentlyContinue | Stop-Process -Force",
        ])
        .output();
}

#[allow(dead_code)]
fn _ensure_output_dir(_p: &PathBuf) {}
