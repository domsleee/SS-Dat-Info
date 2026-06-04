//! cont-splice-frame: assert CONT splices at the EXACT requested frame.
//!
//! "Continue actually continues on the correct frame" — when you CONT at frame
//! N, the new recording must begin at frame N, not N±k. cave2 records the splice
//! frame in `segment_boundaries[].frame` at the splice moment, so it is NOT
//! blurred by the post-splice recording that inflates `recorded_count`. We run
//! at 0.25x by default (per the user's suggestion): the replay advances slowly
//! so the splice is frame-precise and any catch-up overshoot would show up.
//!
//! Asserts: splice boundary == N, mode==REC after splice, and zero drift over
//! the replayed prefix [0..N] (so it continued from the RIGHT frame's state).

use std::path::PathBuf;

use tas_shared::TasMode;

use crate::{drift, harness, replay};

const RECORDING_REL: &str = "TAS/recordings/FE-tremendous.tasrec";
const RETRIES: u32 = 40;

fn locate() -> Option<PathBuf> {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let candidates = [
        exe_dir.join("../../..").join(RECORDING_REL),
        PathBuf::from(RECORDING_REL),
        PathBuf::from("recordings/FE-tremendous.tasrec"),
    ];
    candidates.iter().find(|p| p.exists()).cloned()
}

pub fn run(splice_frame: u32, speed: f32) -> bool {
    println!(
        "=== CONT-SPLICE-FRAME: continue at frame {} @ {}x (splice must land EXACTLY on {}) ===",
        splice_frame, speed, splice_frame
    );
    let path = match locate() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: couldn't locate {}", RECORDING_REL);
            return false;
        }
    };
    let rec = match replay::load_tasrec(&path) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: load failed: {}", e);
            return false;
        }
    };
    if rec.count <= splice_frame {
        eprintln!("ERROR: recording has {} frames, need > {}", rec.count, splice_frame);
        return false;
    }
    println!("  Recording: {} ({} frames)", path.display(), rec.count);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "cont-splice-frame");
    replay::write_to_shared(&mut client, &rec);
    let rec_start = rec.rec_coords[0];
    client.state_mut().playback_speed = speed;

    let spliced =
        harness::restart_continue_and_splice_inprocess(&mut client, rec_start, splice_frame, RETRIES);
    if !spliced {
        println!("*** CONT-SPLICE-FRAME FAILED: never landed a CONT bucket ***");
        return false;
    }

    let s = client.state();
    let mode_rec = s.mode == TasMode::Rec as u32;
    let seg_count = s.segment_count;
    let recorded = s.recorded_count;
    let splice_boundary = if seg_count >= 1 && (seg_count as usize) <= s.segment_boundaries.len() {
        s.segment_boundaries[(seg_count - 1) as usize].frame
    } else {
        0
    };
    let d = drift::compute_drift(s, splice_frame);

    println!();
    println!("  requested splice frame: {}", splice_frame);
    println!(
        "  splice boundary frame:  {}   (segment_count={})",
        splice_boundary, seg_count
    );
    println!("  mode after splice:      {}", if mode_rec { "REC" } else { "NOT REC" });
    println!(
        "  recorded_count:         {}   (= splice frame + post-splice frames)",
        recorded
    );
    println!(
        "  prefix drift [0..{}):    X={:.9}  Z={:.9}",
        splice_frame, d.max_drift_x, d.max_drift_z
    );

    let exact = splice_boundary == splice_frame;
    let drift_ok = d.max_drift_x == 0.0 && d.max_drift_z == 0.0;
    if exact && mode_rec && drift_ok {
        println!(
            "\n*** CONT-SPLICE-FRAME PASSED: spliced at EXACT frame {} with zero drift ***",
            splice_frame
        );
        true
    } else {
        if !exact {
            println!(
                "  >> splice landed on frame {} but {} was requested (off by {})",
                splice_boundary,
                splice_frame,
                (splice_boundary as i64 - splice_frame as i64)
            );
        }
        println!(
            "\n*** CONT-SPLICE-FRAME FAILED: exact={} mode_rec={} drift_ok={} ***",
            exact, mode_rec, drift_ok
        );
        false
    }
}
