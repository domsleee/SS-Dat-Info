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
    #[serde(default)]
    pub force_fixed_tick: u32,
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
    pub max_drift_y: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_y: usize,
    pub max_drift_frame_z: usize,
    pub position_matched: bool,
    pub playback_complete: bool,
}

impl ReplayResult {
    /// Any non-zero drift on any axis (Y included).
    pub fn has_drift(&self) -> bool {
        self.max_drift_x > 0.0 || self.max_drift_y > 0.0 || self.max_drift_z > 0.0
    }
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
            "{:>4} {:>12} {:>12} {:>12} {:>7} {:>8}",
            "#", "max_drift_x", "max_drift_y", "max_drift_z", "pos_ok", "play_ok"
        );
        println!("{}", "-".repeat(70));

        let mut drift_count = 0;
        for r in &self.results {
            if r.has_drift() {
                drift_count += 1;
            }
            println!(
                "{:>4} {:>12.9} {:>12.9} {:>12.9} {:>7} {:>8}",
                r.iteration,
                r.max_drift_x,
                r.max_drift_y,
                r.max_drift_z,
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

    println!("Loaded: {} ticks, fft={}", rec.count, rec.meta.force_fixed_tick);
    if !rec.meta.notes.is_empty() {
        println!("Notes: {}", rec.meta.notes);
    }

    // Count steering transitions for info
    let transitions = drift::count_transitions(&rec.input_log, rec.count as usize);
    println!("Input transitions: {}", transitions);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Pre-flight: is the game even on the track this recording belongs to?
    // Without this a wrong-track run looks like a hang — the start matcher can
    // never reach a spawn that is on another map, so it burns its full retry
    // budget (~22s each) and the mode appears to stall for many minutes.
    // FAIL CLOSED. Waiting for a resolved level is not enough on its own: if it
    // never resolves, treating that as "no opinion" lets through exactly the
    // wrong-track replay this guard exists to stop, and the caller sees the
    // 10-minute pseudo-hang instead of an error. Unresolved means we CANNOT
    // confirm the track, and a guard that cannot confirm must refuse.
    //
    // TAS_TEST_LEVEL=any is the documented, explicit bypass (it already disables
    // the harness track check), so deliberate off-track work stays possible
    // without the guard silently deciding for you.
    let bypass = std::env::var("TAS_TEST_LEVEL")
        .map(|v| v.eq_ignore_ascii_case("any"))
        .unwrap_or(false);
    let resolved = {
        let start = std::time::Instant::now();
        loop {
            if let Some(id) = tas_shared::resolved_level_id(client.state()) {
                break Some(id);
            }
            if start.elapsed() > std::time::Duration::from_secs(12) {
                break None;
            }
            std::thread::sleep(std::time::Duration::from_millis(250));
        }
    };
    match resolved {
        Some(id) => {
            if let Err(msg) = tas_shared::level::check_recording_matches_live(path, id) {
                eprintln!("ERROR: {}", msg);
                std::process::exit(1);
            }
        }
        None if bypass => {
            eprintln!("  WARNING: level unresolved; track guard bypassed (TAS_TEST_LEVEL=any)");
        }
        None => {
            eprintln!(
                "ERROR: the track could not be identified within 12s, so this recording \
                 cannot be confirmed to belong here. Replaying on the wrong track never \
                 matches the recorded spawn and burns the entire retry budget. Navigate \
                 into a Time-Attack track, or set TAS_TEST_LEVEL=any to replay anyway."
            );
            std::process::exit(1);
        }
    }

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
            let m = harness::restart_play_and_match_inprocess(
                &mut client,
                target,
                harness::START_MATCH_RETRIES,
            );
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
            max_drift_y: d.max_drift_y,
            max_drift_z: d.max_drift_z,
            max_drift_frame_x: d.max_drift_frame_x,
            max_drift_frame_y: d.max_drift_frame_y,
            max_drift_frame_z: d.max_drift_frame_z,
            position_matched: matched,
            playback_complete: play_ok,
        };

        if verbose || !d.is_zero() {
            println!(
                "  Drift: X={:.9} (frame {}), Y={:.9} (frame {}), Z={:.9} (frame {})",
                d.max_drift_x,
                d.max_drift_frame_x,
                d.max_drift_y,
                d.max_drift_frame_y,
                d.max_drift_z,
                d.max_drift_frame_z
            );
            // Find first frame where any axis diverges (bit-level mismatch).
            // Reveals whether drift is sudden (rotation mismatch at start) or
            // gradual (some per-frame state slowly diverging).
            let played = rec.count.min(state.playback_pos) as usize;
            let mut first_div: Option<(usize, [f32; 3], [f32; 3])> = None;
            for i in 0..played {
                let p = state.play_coords[i];
                let r = state.rec_coords[i];
                if p[0].to_bits() != r[0].to_bits()
                    || p[1].to_bits() != r[1].to_bits()
                    || p[2].to_bits() != r[2].to_bits()
                {
                    first_div = Some((i, p, r));
                    break;
                }
            }
            if let Some((i, p, r)) = first_div {
                let dx = (p[0] as f64 - r[0] as f64).abs();
                let dy = (p[1] as f64 - r[1] as f64).abs();
                let dz = (p[2] as f64 - r[2] as f64).abs();
                println!(
                    "  First divergence at frame {}: rec=({:.6},{:.6},{:.6}) play=({:.6},{:.6},{:.6}) Δ=({:.6},{:.6},{:.6})",
                    i, r[0], r[1], r[2], p[0], p[1], p[2], dx, dy, dz
                );
            }
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
