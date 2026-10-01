//! Diagnostics for hand-driven sessions: compare the replay in shared memory
//! with its recording over the whole length, gate-relative and bit for bit,
//! and print the take's model, gates and last finish. Reads only.

use tas_shared::race_clock::{hud_cs, race_finish};
use tas_shared::{TasSharedMemoryClient, TAS_INPUT_MODEL_HELD};

use crate::drift;

pub fn run() -> bool {
    let client = match TasSharedMemoryClient::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    let s = client.state();
    let count = s.recorded_count as usize;
    let model = if s.input_model == TAS_INPUT_MODEL_HELD {
        "held"
    } else {
        "injected"
    };
    let rec_gate = tas_shared::align::detect_first_moving(&s.rec_coords[..], s.recorded_count);
    let play_gate = s.gate_index;
    println!(
        "take: {count} ticks, model {model}, recording gate {rec_gate:?}, replay gate {play_gate}, replayed to {} (mode {})",
        s.playback_pos, s.mode
    );
    let mut per_bit = [0u32; 8];
    for mask in &s.input_log[..count] {
        for (bit, n) in per_bit.iter_mut().enumerate() {
            *n += u32::from(mask >> bit & 1);
        }
    }
    println!("ticks held per input bit (0..7): {per_bit:?}");
    if let Some(f) = race_finish(s) {
        println!(
            "last finish: #{} tick {} mode {} valid {} time {} ({} cs on the HUD)",
            f.seq,
            f.tick,
            f.mode,
            f.valid,
            f.seconds,
            hud_cs(
                f.seconds,
                tas_shared::race_clock::extended_precision(s.fpu_control_word)
            )
        );
    }
    let Some(rec_gate) = rec_gate else {
        println!("the recording never moves: nothing to compare");
        return true;
    };
    if play_gate == 0 {
        println!("no replay gate yet: nothing to compare");
        return true;
    }
    let replayed = s.playback_pos.saturating_sub(play_gate);
    let n = replayed.min((count as u32).saturating_sub(rec_gate));
    match drift::bit_mismatches(&s.rec_coords, &s.play_coords, rec_gate, play_gate, n) {
        (_, None) => {
            println!("replay == recording, bit for bit, over {n} ticks from the gate");
            true
        }
        (differ, Some(i)) => {
            println!(
                "{differ} of {n} ticks differ; first at gate+{i}: recorded {:?} replayed {:?}",
                s.rec_coords[(rec_gate + i) as usize],
                s.play_coords[(play_gate + i) as usize]
            );
            false
        }
    }
}
