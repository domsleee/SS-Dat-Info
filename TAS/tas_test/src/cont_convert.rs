//! A CONT from a take recorded before the held model (DESIGN.md "REC
//! observes, PLAY writes"). The splice converts the replayed prefix to held
//! keys; the new take, prefix and recorded suffix alike, must then replay bit
//! for bit under the held model over its whole length, not just the
//! watcher's window. LEFT, jump and shift are held on the real keyboard
//! through the splice: REC must start from them, and once they are released
//! nothing may stay held in the observer or the key buffer.

use std::thread;
use std::time::Duration;

use tas_shared::{TasMode, TasSharedMemoryClient, TAS_INPUT_MODEL_HELD, TAS_INPUT_MODEL_INJECTED};

use crate::command_edges::{cleanup, load_fixture};
use crate::gamemem::GameMemory;
use crate::{harness, patterns};

/// LEFT + jump + shift: an arrow and both modifiers with alias codes.
const HELD: u8 = 0x01 | 0x10 | 0x20;

const SPLICE: u32 = 2200;
const CATCHUP_SPEED: f32 = 12.0;

pub fn run() -> bool {
    println!("=== CONT-CONVERT: a CONT from an injected take replays exactly as a held take ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = convert_and_replay(&mut client);
    cleanup(&mut client);
    match result {
        Ok(summary) => {
            println!("  {summary}");
            println!("\n*** CONT-CONVERT PASSED ***");
            true
        }
        Err(e) => {
            eprintln!("\n*** CONT-CONVERT FAILED: {e} ***");
            false
        }
    }
}

fn convert_and_replay(client: &mut TasSharedMemoryClient) -> Result<String, String> {
    harness::stop(client);
    load_fixture(client)?;
    if client.state().input_model != TAS_INPUT_MODEL_INJECTED {
        return Err("the fixture should load as an injected take".into());
    }
    let legacy: Vec<u8> = client.state().input_log[..SPLICE as usize].to_vec();
    let memory = GameMemory::attach().ok_or("cannot read the game's memory")?;
    client.state_mut().playback_speed = CATCHUP_SPEED;
    client.state_mut().cont_resume_speed = 1.0;
    // Hold the keys through the restart and catch-up (PLAY blocks them) and
    // into the REC, then steer and release.
    let keys = thread::spawn(|| {
        let steps =
            patterns::build_from_explicit(&[(HELD, 700), (0x00, 30), (0x02, 40), (0x00, 60)]);
        harness::drive_pico_steps(&steps)
    });
    let spliced = harness::restart_continue_and_splice_inprocess(client, SPLICE, 0);
    let driven = keys.join().map_err(|_| "the Pico thread panicked")?;
    spliced.ok_or("the CONT did not splice")?;
    driven?;
    thread::sleep(Duration::from_millis(200));
    if client.state().input_model != TAS_INPUT_MODEL_HELD {
        return Err("the splice left the take in the injected model".into());
    }
    let first_rec = client.state().input_log[SPLICE as usize];
    if first_rec & HELD != HELD {
        return Err(format!(
            "the first REC tick holds {first_rec:#04x}, not the physical keys {HELD:#04x}"
        ));
    }
    let observer = memory.observer_tas_mask()?;
    let buffer = memory.held_tas_mask()?;
    if observer != 0 || buffer != 0 {
        return Err(format!(
            "keys still held after the physical release: observer {observer:#04x}, key buffer {buffer:#04x}"
        ));
    }
    let count = client.state().recorded_count;
    harness::stop(client);
    if client.mode_volatile() != TasMode::Off as u32 {
        return Err("REC did not stop".into());
    }
    if count <= SPLICE + 100 {
        return Err(format!("only {count} ticks after a splice at {SPLICE}"));
    }
    let converted: Vec<u8> = client.state().input_log[..SPLICE as usize].to_vec();
    let staggered = (1..SPLICE as usize)
        .filter(|&i| converted[i] != legacy[i - 1])
        .count();
    let suffix_keys = client.state().input_log[SPLICE as usize..count as usize]
        .iter()
        .filter(|&&m| m != 0)
        .count();
    if suffix_keys == 0 {
        return Err("the recorded suffix holds no keys (did the Pico reach the game?)".into());
    }
    let rec: Vec<[f32; 3]> = client.state().rec_coords[..count as usize].to_vec();

    // Replay the new take, whole, under the held model.
    client.state_mut().playback_speed = 8.0;
    let (rec_gate, play_gate) = harness::restart_play_aligned_inprocess(client)
        .ok_or("aligned PLAY of the new take failed")?;
    if client.state().input_model != TAS_INPUT_MODEL_HELD {
        return Err("the new take was not replayed as a held take".into());
    }
    let expected_end = play_gate + (count - rec_gate);
    if !harness::wait_playback(client, expected_end) {
        return Err("the replay did not finish".into());
    }
    client.state_mut().playback_speed = 1.0;
    let play = &client.state().play_coords;
    let mut mismatches = 0;
    let mut first = None;
    let recorded = &rec[rec_gate as usize..count as usize];
    let replayed = &play[play_gate as usize..];
    for (i, (r, p)) in recorded.iter().zip(replayed).enumerate() {
        if r.map(f32::to_bits) != p.map(f32::to_bits) {
            mismatches += 1;
            first.get_or_insert(i + rec_gate as usize);
        }
    }
    if mismatches > 0 {
        return Err(format!(
            "{mismatches} of {} ticks differ; first at recorded tick {:?} (splice {SPLICE})",
            count - rec_gate,
            first
        ));
    }
    Ok(format!(
        "{} ticks from the gate (prefix to {SPLICE}, suffix {} ticks with {suffix_keys} keyed) replayed bit for bit; conversion changed {staggered} prefix tick(s) beyond the one-tick shift",
        count - rec_gate,
        count - SPLICE
    ))
}
