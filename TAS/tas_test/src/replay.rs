//! Replay mode: load a .tasrec file and play it back N times, checking drift each time.
//!
//! Usage: tas_test replay <path.tasrec> [--iterations N] [--verbose]

use serde::Deserialize;
use serde_json::Value;
use tas_shared::{TasSharedMemoryClient, TAS_MAX_TICKS};

use crate::drift;
use crate::harness;

#[derive(Deserialize)]
pub struct RecordingMetadata {
    pub recorded_count: u32,
    #[allow(dead_code)]
    pub inject_mode: u32,
    #[allow(dead_code)]
    pub force_fixed_tick: u32,
    #[allow(dead_code)]
    pub force_direct: u32,
    #[allow(dead_code)]
    pub input_source: u32,
    #[serde(default)]
    pub notes: String,
}

pub struct LoadedRecording {
    pub count: u32,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
    pub meta: RecordingMetadata,
    pub raw_meta: Value,
}

pub fn load_tasrec(path: &std::path::Path) -> Result<LoadedRecording, String> {
    let data = std::fs::read(path).map_err(|e| format!("read: {}", e))?;
    if data.len() < 4 {
        return Err("File too small".into());
    }

    let meta_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    if data.len() < 4 + meta_len {
        return Err("Truncated metadata".into());
    }

    let meta_json = std::str::from_utf8(&data[4..4 + meta_len]).map_err(|e| format!("{}", e))?;
    let raw_meta: Value = serde_json::from_str(meta_json).map_err(|e| format!("{}", e))?;
    let meta: RecordingMetadata =
        serde_json::from_value(raw_meta.clone()).map_err(|e| format!("{}", e))?;

    let count = meta.recorded_count as usize;
    if count > TAS_MAX_TICKS {
        return Err(format!("Recording too long: {} ticks", count));
    }

    let input_start = 4 + meta_len;
    let input_end = input_start + count;
    if data.len() < input_end {
        return Err("Truncated input log".into());
    }

    let input_log = data[input_start..input_end].to_vec();

    let coords_start = input_end;
    let coords_size = count * 3 * 4;
    let mut rec_coords = vec![[0.0f32; 3]; count];

    if data.len() >= coords_start + coords_size {
        let mut offset = coords_start;
        for coord in rec_coords.iter_mut() {
            for val in coord.iter_mut() {
                *val = f32::from_le_bytes([
                    data[offset],
                    data[offset + 1],
                    data[offset + 2],
                    data[offset + 3],
                ]);
                offset += 4;
            }
        }
    } else {
        return Err("Truncated rec_coords".into());
    }

    Ok(LoadedRecording {
        count: meta.recorded_count,
        input_log,
        rec_coords,
        meta,
        raw_meta,
    })
}

pub fn save_tasrec(
    path: &std::path::Path,
    meta: &Value,
    input_log: &[u8],
    rec_coords: &[[f32; 3]],
) -> Result<(), String> {
    if input_log.len() != rec_coords.len() {
        return Err(format!(
            "input/coord length mismatch: {} inputs vs {} coords",
            input_log.len(),
            rec_coords.len()
        ));
    }

    let meta_json = serde_json::to_string_pretty(meta).map_err(|e| format!("{}", e))?;
    let meta_bytes = meta_json.as_bytes();
    let meta_len = meta_bytes.len() as u32;

    let mut data =
        Vec::with_capacity(4 + meta_bytes.len() + input_log.len() + rec_coords.len() * 12);
    data.extend_from_slice(&meta_len.to_le_bytes());
    data.extend_from_slice(meta_bytes);
    data.extend_from_slice(input_log);
    for coord in rec_coords {
        for value in coord {
            data.extend_from_slice(&value.to_le_bytes());
        }
    }

    std::fs::write(path, data).map_err(|e| format!("{}", e))
}

/// Write loaded recording into shared memory state.
pub fn write_to_shared(client: &mut TasSharedMemoryClient, rec: &LoadedRecording) {
    let state = client.state_mut();
    let count = rec.count as usize;

    state.input_log[..count].copy_from_slice(&rec.input_log);
    for i in count..TAS_MAX_TICKS {
        state.input_log[i] = 0;
    }

    for (i, coord) in rec.rec_coords.iter().enumerate() {
        state.rec_coords[i] = *coord;
    }

    state.recorded_count = rec.count;
    // Force proven zero-drift config
    state.force_fixed_tick = 0;
}

pub struct ReplayResult {
    pub iteration: u32,
    pub max_drift_x: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_z: usize,
    pub position_matched: bool,
    pub playback_complete: bool,
}

pub struct ReplayReport {
    pub results: Vec<ReplayResult>,
    pub file_path: String,
    pub recorded_count: u32,
}

impl ReplayReport {
    pub fn print_summary(&self) {
        println!("\n=== REPLAY SUMMARY ===");
        println!("File: {}", self.file_path);
        println!("Recording: {} ticks", self.recorded_count);
        println!("Iterations: {}", self.results.len());
        println!();
        println!(
            "{:>4} {:>12} {:>12} {:>8} {:>8} {:>7} {:>8}",
            "#", "max_drift_x", "max_drift_z", "frame_x", "frame_z", "pos_ok", "play_ok"
        );
        println!("{}", "-".repeat(70));

        let mut drift_count = 0;
        for r in &self.results {
            let has_drift = r.max_drift_x > 0.0 || r.max_drift_z > 0.0;
            if has_drift {
                drift_count += 1;
            }
            println!(
                "{:>4} {:>12.9} {:>12.9} {:>8} {:>8} {:>7} {:>8}",
                r.iteration,
                r.max_drift_x,
                r.max_drift_z,
                r.max_drift_frame_x,
                r.max_drift_frame_z,
                if r.position_matched { "yes" } else { "NO" },
                if r.playback_complete { "yes" } else { "NO" },
            );
        }

        println!();
        if drift_count == 0 {
            println!(
                "Result: ZERO DRIFT in all {} iterations",
                self.results.len()
            );
        } else {
            println!(
                "Result: DRIFT in {}/{} iterations",
                drift_count,
                self.results.len()
            );
        }
    }
}

pub fn run(path: &str, iterations: u32, verbose: bool, no_match: bool) -> ReplayReport {
    println!("=== Replay Drift Test ===");
    println!("File: {}", path);
    println!("Iterations: {}", iterations);

    let rec = match load_tasrec(std::path::Path::new(path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: Failed to load {}: {}", path, e);
            std::process::exit(1);
        }
    };

    println!(
        "Loaded: {} ticks, inject_mode={}, fft={}, force_direct={}",
        rec.count, rec.meta.inject_mode, rec.meta.force_fixed_tick, rec.meta.force_direct
    );
    if !rec.meta.notes.is_empty() {
        println!("Notes: {}", rec.meta.notes);
    }

    // Count steering transitions for info
    let transitions = drift::count_transitions(&rec.input_log, rec.count as usize);
    println!("Input transitions: {}", transitions);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Write recording data into shared memory
    write_to_shared(&mut client, &rec);

    let target = rec.rec_coords[0];
    println!(
        "Target start position: ({:.6}, {:.6}, {:.6})",
        target[0], target[1], target[2]
    );

    let mut results = Vec::new();

    for i in 1..=iterations {
        println!("\n--- Iteration {}/{} ---", i, iterations);

        // Focus game for full framerate playback
        harness::focus_game();

        let matched = if no_match {
            // Just restart and immediately arm play (no position matching)
            harness::restart_and_stabilize(&client);
            harness::arm_play(&mut client);
            println!("  Position matching skipped (--no-match)");
            false
        } else {
            let m = harness::restart_play_and_match_inprocess(&mut client, target, 20);
            if !m {
                println!("  WARNING: Position match failed");
            }
            m
        };

        let play_ok = harness::wait_playback(&client, rec.count);

        let state = client.state();
        let d = drift::compute_drift(state, rec.count.min(state.playback_pos));

        let r = ReplayResult {
            iteration: i,
            max_drift_x: d.max_drift_x,
            max_drift_z: d.max_drift_z,
            max_drift_frame_x: d.max_drift_frame_x,
            max_drift_frame_z: d.max_drift_frame_z,
            position_matched: matched,
            playback_complete: play_ok,
        };

        if verbose || d.max_drift_x > 0.0 || d.max_drift_z > 0.0 {
            println!(
                "  Drift: X={:.9} (frame {}), Z={:.9} (frame {})",
                d.max_drift_x, d.max_drift_frame_x, d.max_drift_z, d.max_drift_frame_z
            );
        } else {
            println!("  Zero drift");
        }

        results.push(r);

        // Re-write recording data (playback may have modified shared state)
        write_to_shared(&mut client, &rec);
    }

    let report = ReplayReport {
        results,
        file_path: path.to_string(),
        recorded_count: rec.count,
    };

    report.print_summary();
    report
}
