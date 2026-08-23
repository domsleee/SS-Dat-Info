//! Does aligning the replay's input to its OWN gate make the bucket irrelevant?
//!
//! A replay normally applies `input_log[playback_pos]` — indexed from the ARM.
//! If this replay's countdown ends on a different index than the recording's
//! did, every input lands at the wrong offset against the race start and the
//! run diverges. That, and only that, is why a judged PLAY rerolls until it
//! matches the recording's first-moving frame.
//!
//! Predicting that index exactly turned out to be impossible from arm-time
//! state: the deciding event is tick batching during the restart, which is over
//! before the arm and leaves nothing behind to read. So instead of predicting
//! the gate, index the input FROM it:
//!
//!     recorded_index = playback_index - play_gate + rec_gate
//!
//! This runs replays with bucket matching OFF, so the gate lands wherever it
//! lands, and compares the trajectory AFTER each side's own gate:
//!
//!     rec_coords[rec_gate + k]  vs  play_coords[play_gate + k]
//!
//! If those agree while the raw indices disagree, the countdown's end tick has
//! stopped mattering — which is a better outcome than predicting it, because
//! there is then nothing left to get wrong.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::harness;
use crate::replay;

const RECORDING_REL: &str = "TAS/recordings/FE-10065.tasrec";
/// Compare the WHOLE overlap past each side's gate, not a window. The first
/// version compared 1200 of 7162 ticks, which cannot see the failure this
/// project has a documented history of: a bucket that tracks through the
/// settle and veers off hundreds of ticks later.
const COMPARE_TICKS: u32 = 6000;
/// Replayed fast. Judging during a catch-up was already measured bit-exact
/// (drift 0.0000 at 64x on this same recording), so this buys depth without
/// buying a new variable.
const COMPARE_SPEED: f32 = 16.0;
const PLAY_TIMEOUT_SECS: u64 = 180;
/// Arm delays swept across attempts, in milliseconds. The gate is measured
/// from the ARM, so delaying the arm moves the gate — which is the only way to
/// exercise offsets bigger than the +1 the first run happened to produce.
const ARM_DELAYS_MS: &[u64] = &[0, 8, 16, 24, 32, 40];

struct Attempt {
    aligned: bool,
    /// How many ticks were actually compared. A pass on three ticks is not a
    /// pass, so this is reported and asserted rather than assumed.
    compared: u32,
    play_gate: u32,
    rec_gate: u32,
    /// Max per-axis |play - rec| over COMPARE_TICKS past each side's own gate.
    gate_rel_drift: f32,
    /// The same comparison done the OLD way, index against index.
    index_drift: f32,
}

pub fn run(iterations: u32, rec: Option<&str>) -> bool {
    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."));
    let rel = rec.unwrap_or(RECORDING_REL);
    let candidates = [
        exe_dir.join("../../..").join(rel),
        PathBuf::from(rel),
        exe_dir.join("../../..").join("TAS/recordings").join(rel),
    ];
    let Some(path) = candidates.iter().find(|p| p.exists()) else {
        eprintln!("ERROR: couldn't locate {}", rel);
        return false;
    };
    let path = path.to_string_lossy().into_owned();
    println!("=== Gate-relative input alignment: {} ===", path);

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    let loaded = match replay::load_tasrec(std::path::Path::new(&path)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    let rec_gate = {
        let s = client.state();
        match tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count) {
            Some(f) => f,
            None => {
                eprintln!("ERROR: the recording never moves");
                return false;
            }
        }
    };
    println!("  recording first-moving = {}", rec_gate);
    // Does this recording have input BEFORE its gate? Alignment holds
    // input_log[rec_gate] through the countdown instead of replaying whatever
    // the recording had there, on the argument that pre-gate input is inert
    // because the boarder cannot move. If the recording DOES have pre-gate
    // input, zero drift is evidence for that argument rather than an untested
    // assumption — and if it does not, the argument is simply unexercised.
    {
        let s = client.state();
        let gate_mask = s.input_log[rec_gate as usize];
        let nonzero = s.input_log[..rec_gate as usize].iter().filter(|b| **b != 0).count();
        let differing = s.input_log[..rec_gate as usize]
            .iter()
            .filter(|b| **b != gate_mask)
            .count();
        println!(
            "  pre-gate input: {} of {} frames non-zero, {} differ from input_log[rec_gate]=0x{:02x}",
            nonzero, rec_gate, differing, gate_mask
        );
        if differing == 0 {
            println!("  (so holding the gate input changes nothing here — assumption UNEXERCISED)");
        } else {
            println!("  (so holding the gate input DOES differ from the recording — assumption under test)");
        }
    }
    harness::focus_game();

    let mut attempts = Vec::new();
    for i in 1..=iterations {
        // Alternate alignment, and sweep the arm delay so both arms see the
        // same spread of gates rather than whatever the machine felt like.
        let aligned = i % 2 == 0;
        let delay = ARM_DELAYS_MS[(i as usize / 2) % ARM_DELAYS_MS.len()];
        if let Some(a) = one_attempt(&mut client, rec_gate, aligned, delay) {
            println!(
                "  attempt {:>2} {:<9} d{:>2}ms play_gate={} (rec {}, off {:+}) | {} ticks | gate-rel {:.5} | index {:.5}",
                i,
                if aligned { "ALIGNED" } else { "baseline" },
                delay,
                a.play_gate,
                a.rec_gate,
                a.play_gate as i64 - a.rec_gate as i64,
                a.compared,
                a.gate_rel_drift,
                a.index_drift
            );
            attempts.push(a);
        } else {
            println!("  attempt {:>2}: failed", i);
        }
    }
    harness::stop(&mut client);
    report(&attempts, rec_gate)
}

fn one_attempt(
    client: &mut TasSharedMemoryClient,
    rec_gate: u32,
    aligned: bool,
    delay_ms: u64,
) -> Option<Attempt> {
    client.state_mut().playback_speed = COMPARE_SPEED;
    // The whole point: no bucket matching. The gate lands where it lands.
    client.state_mut().gate_align_rec = if aligned { rec_gate } else { 0 };
    client.state_mut().gate_index = 0;
    client.state_mut().gate_tick = 0;

    if !harness::restart_and_stabilize_inprocess(client) {
        return None;
    }
    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms));
    }
    harness::arm_play(client);

    // Wait until the replay is far enough past its gate to compare.
    let deadline = Instant::now() + Duration::from_secs(PLAY_TIMEOUT_SECS);
    loop {
        if Instant::now() > deadline {
            eprintln!("    timed out");
            client.send_command(TasCommand::Stop);
            return None;
        }
        let s = client.state();
        let g = s.gate_index;
        if g != 0 && s.playback_pos > g + COMPARE_TICKS {
            break;
        }
        if s.mode != TasMode::Play as u32 && s.playback_pos > 0 {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }

    let s = client.state();
    let play_gate = s.gate_index;
    if play_gate == 0 {
        client.send_command(TasCommand::Stop);
        return None;
    }
    let mut gate_rel = 0f32;
    let mut idx = 0f32;
    let mut compared = 0u32;
    for k in 0..COMPARE_TICKS {
        let pi = (play_gate + k) as usize;
        let ri = (rec_gate + k) as usize;
        if pi >= s.play_coords.len() || ri >= s.rec_coords.len() || ri >= s.recorded_count as usize {
            break;
        }
        if (pi as u32) >= s.playback_pos {
            break;
        }
        let p = s.play_coords[pi];
        let r = s.rec_coords[ri];
        gate_rel = gate_rel
            .max((p[0] - r[0]).abs())
            .max((p[1] - r[1]).abs())
            .max((p[2] - r[2]).abs());
        compared += 1;
        // Same window, compared index-against-index the old way.
        let r2 = s.rec_coords[pi.min(s.recorded_count as usize - 1)];
        idx = idx
            .max((p[0] - r2[0]).abs())
            .max((p[1] - r2[1]).abs())
            .max((p[2] - r2[2]).abs());
    }
    let a = Attempt {
        aligned,
        compared,
        play_gate,
        rec_gate,
        gate_rel_drift: gate_rel,
        index_drift: idx,
    };
    client.send_command(TasCommand::Stop);
    thread::sleep(Duration::from_millis(200));
    Some(a)
}

fn report(attempts: &[Attempt], rec_gate: u32) -> bool {
    println!("\n=== RESULT ===");
    let gates: Vec<u32> = attempts.iter().map(|a| a.play_gate).collect();
    let off: Vec<i64> = gates.iter().map(|g| *g as i64 - rec_gate as i64).collect();
    println!("  play gates seen: {:?} (offsets from the recording: {:?})", gates, off);

    for &aligned in &[false, true] {
        let v: Vec<&Attempt> = attempts.iter().filter(|a| a.aligned == aligned).collect();
        if v.is_empty() {
            continue;
        }
        let worst_rel = v.iter().map(|a| a.gate_rel_drift).fold(0.0, f32::max);
        let worst_idx = v.iter().map(|a| a.index_drift).fold(0.0, f32::max);
        println!(
            "  {:<9}: n={} worst gate-relative drift {:.5}, worst index drift {:.5}",
            if aligned { "ALIGNED" } else { "baseline" },
            v.len(),
            worst_rel,
            worst_idx
        );
    }

    // The claim to test: with alignment on, a replay whose gate does NOT match
    // the recording's still reproduces the run.
    let mismatched: Vec<&Attempt> = attempts
        .iter()
        .filter(|a| a.aligned && a.play_gate != a.rec_gate)
        .collect();
    if mismatched.is_empty() {
        println!("\n  No aligned attempt landed a DIFFERENT gate — nothing was proved.");
        println!("  (Re-run; the gate has to differ for the test to mean anything.)");
        return false;
    }
    // A pass on a handful of ticks is not a pass.
    let shallow = mismatched.iter().filter(|a| a.compared < 2000).count();
    if shallow > 0 {
        println!("  WARNING: {} mismatched attempts compared fewer than 2000 ticks", shallow);
    }
    let offsets: Vec<i64> = mismatched
        .iter()
        .map(|a| a.play_gate as i64 - a.rec_gate as i64)
        .collect();
    println!("  offsets exercised by aligned attempts: {:?}", offsets);
    let worst = mismatched.iter().map(|a| a.gate_rel_drift).fold(0.0, f32::max);
    println!(
        "\n  {} aligned attempts landed a different gate than the recording; worst drift {:.5}",
        mismatched.len(),
        worst
    );
    if worst < 0.001 && shallow == 0 {
        println!("  *** The gate index no longer matters. Alignment reproduces the run. ***");
        true
    } else {
        println!("  Alignment did NOT reproduce the run — something depends on the");
        println!("  absolute index, not just time since the race started.");
        false
    }
}
