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

pub fn run(splice_frame: u32, catchup_speed: f32, record_speed: f32) -> bool {
    println!(
        "=== CONT-SPLICE-FRAME: catch up @ {}x → record @ {}x (splice must land EXACTLY on {}) ===",
        catchup_speed, record_speed, splice_frame
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
        eprintln!(
            "ERROR: recording has {} frames, need > {}",
            rec.count, splice_frame
        );
        return false;
    }
    println!("  Recording: {} ({} frames)", path.display(), rec.count);

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "cont-splice-frame");
    replay::write_to_shared(&mut client, &rec);
    let rec_start = rec.rec_coords[0];

    // Realistic CONT profile: catch up to the splice FAST (e.g. 64x), then drop
    // to a SLOW record speed (e.g. 0.25x) for the precise continued recording.
    // The controller reads playback_speed at construction, so set the catch-up
    // speed before arming.
    client.state_mut().playback_speed = catchup_speed;

    let spliced = harness::restart_continue_and_splice_inprocess(
        &mut client,
        rec_start,
        splice_frame,
        RETRIES,
    );
    if spliced.is_none() {
        println!("*** CONT-SPLICE-FRAME FAILED: never landed a CONT bucket ***");
        return false;
    }

    // Capture the just-spliced state BEFORE recording/stopping: the controller
    // returns once mode==REC at the splice, so the splice frame and REC mode are
    // observable here. (Reading after the stop below would always show OFF.)
    let mode_rec = client.state().mode == TasMode::Rec as u32;
    let seg_count = client.state().segment_count;
    let splice_boundary = {
        let s = client.state();
        if seg_count >= 1 && (seg_count as usize) <= s.segment_boundaries.len() {
            s.segment_boundaries[(seg_count - 1) as usize].frame
        } else {
            0
        }
    };
    let recorded_at_splice = client.state().recorded_count;
    // Gate-aligned CONT shifts the live play index; capture the gate fields
    // now, because harness::stop() below clears gate_align_rec. Unaligned CONT
    // has gate_align_rec == 0 and this reduces to raw-index drift.
    let (splice_aligned, splice_rec_gate, splice_play_gate) = {
        let s = client.state();
        let aligned = s.gate_align_rec > 0 && s.gate_index > 0 && splice_frame > s.gate_align_rec;
        (aligned, s.gate_align_rec, s.gate_index)
    };

    // Now record at the SLOW speed for a short burst, exercising the
    // catch-up→record speed transition. The splice boundary (captured at the
    // splice moment) must be unaffected, and recording must advance.
    println!(
        "  Spliced (mode={}) — dropping to {}x and recording a short burst...",
        if mode_rec { "REC" } else { "NOT REC" },
        record_speed
    );
    client.state_mut().playback_speed = record_speed;
    std::thread::sleep(std::time::Duration::from_millis(1500));
    let recorded_after_burst = client.state().recorded_count;
    harness::stop(&mut client);

    let recorded = recorded_after_burst;
    let burst_frames = recorded_after_burst.saturating_sub(recorded_at_splice);
    let d = if splice_aligned {
        let rel = splice_frame.saturating_sub(splice_rec_gate);
        drift::compute_drift_gate_relative(client.state(), splice_rec_gate, splice_play_gate, rel)
    } else {
        drift::compute_drift(client.state(), splice_frame)
    };

    println!();
    println!("  requested splice frame: {}", splice_frame);
    println!(
        "  splice boundary frame:  {}   (segment_count={})",
        splice_boundary, seg_count
    );
    println!(
        "  mode after splice:      {}",
        if mode_rec { "REC" } else { "NOT REC" }
    );
    println!(
        "  recorded_count:         {}   (= splice frame + post-splice frames)",
        recorded
    );
    println!(
        "  recorded during burst:  {} frames @ {}x   (slow-record after catch-up)",
        burst_frames, record_speed
    );
    println!(
        "  prefix drift{} [0..{}):   X={:.9}  Y={:.9}  Z={:.9}",
        if splice_aligned {
            " (gate-relative)"
        } else {
            ""
        },
        splice_frame,
        d.max_drift_x,
        d.max_drift_y,
        d.max_drift_z
    );

    let exact = splice_boundary == splice_frame;
    let drift_ok = d.is_zero();
    let recorded_advanced = burst_frames > 0;
    if exact && mode_rec && drift_ok && recorded_advanced {
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
            "\n*** CONT-SPLICE-FRAME FAILED: exact={} mode_rec={} drift_ok={} recorded_advanced={} ***",
            exact, mode_rec, drift_ok, recorded_advanced
        );
        false
    }
}
