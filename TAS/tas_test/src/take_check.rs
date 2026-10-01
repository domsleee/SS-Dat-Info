//! Diagnostics for hand-driven sessions: compare the replay in shared memory
//! with its recording over the whole length, gate-relative and bit for bit,
//! and print the take's model, gates and last finish. Reads only.

use tas_shared::race_clock::{hud_cs, race_finish};
use tas_shared::{TasSharedMemoryClient, TAS_INPUT_MODEL_HELD};

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
    let replayed = (s.playback_pos as usize).saturating_sub(play_gate as usize);
    let n = replayed.min(count.saturating_sub(rec_gate as usize));
    let recorded = &s.rec_coords[rec_gate as usize..rec_gate as usize + n];
    let played = &s.play_coords[play_gate as usize..play_gate as usize + n];
    let mut first = None;
    let mut differ = 0;
    for (i, (r, p)) in recorded.iter().zip(played).enumerate() {
        if r.map(f32::to_bits) != p.map(f32::to_bits) {
            differ += 1;
            first.get_or_insert(i);
        }
    }
    match first {
        None => {
            println!("replay == recording, bit for bit, over {n} ticks from the gate");
            true
        }
        Some(i) => {
            println!(
                "{differ} of {n} ticks differ; first at gate+{i}: recorded {:?} replayed {:?}",
                recorded[i], played[i]
            );
            false
        }
    }
}
