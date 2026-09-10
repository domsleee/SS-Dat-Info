//! Does aligning the replay's input to its OWN gate make the bucket irrelevant?
//!
//! A replay normally applies `input_log[playback_pos]` — indexed from the ARM.
//! If this replay's countdown ends on a different index than the recording's
//! did, every input lands at the wrong offset against the race start and the
//! run diverges. This tool isolates that indexing problem; it does not assume
//! the gate index fully describes the game's hidden spawn state.
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
//! stopped mattering for input indexing. Product PLAY still watches the first
//! gate-relative trajectory window and rerolls a differing hidden spawn state.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::harness;
use crate::replay;

const RECORDING: &str = "FE-10065.tasrec";
/// A post-finish dialog stops Supreme::Cycle while leaving TAS in PLAY. Treat a
/// stable playback position and stable, readable race clock as a terminal run
/// instead of burning the full timeout. The matched control establishes how
/// many meaningful ticks exist before that freeze.
const FINISH_STALL_SECS: u64 = 3;
/// Replayed fast: judging during a catch-up is bit-exact on this recording, so
/// this buys depth without a new variable.
const COMPARE_SPEED: f32 = 16.0;
const PLAY_TIMEOUT_SECS: u64 = 180;
/// Arm delays swept across attempts, in milliseconds. The gate is measured
/// from the ARM, so delaying the arm moves the gate — which is the only way to
/// exercise offsets bigger than the +1 the first run happened to produce.
const ARM_DELAYS_MS: &[u64] = &[0, 8, 16, 24, 32, 40];

struct Attempt {
    aligned: bool,
    /// Ticks that were NOT bit-identical. "Drift below a threshold" is not
    /// bit-exactness, and a printed 0.00000 is five decimals of a float, not
    /// proof — this compares raw bits.
    bit_mismatches: u32,
    /// The first gate-relative tick that differed, if any. A mismatch that
    /// starts thousands of ticks in is a different animal from one at k=0.
    first_mismatch: Option<u32>,
    /// Ticks the replay was expected to produce past its gate, and did.
    expected: u32,
    /// The on-screen race time at the end of the replay, in centiseconds.
    ///
    /// Position agreeing is not the same as the RUN agreeing. This is the
    /// number a TAS is actually judged on, it is read from the HUD rather than
    /// derived, and it would catch a replay that traced the right path while
    /// the race clock ran differently.
    race_time_cs: u32,
    /// True when the HUD clock stayed fixed after playback ended. An unfinished
    /// recording's clock keeps running, so a later poll is not a run result.
    race_time_final: bool,
    /// How many ticks were actually compared. A pass on three ticks is not a
    /// pass, so this is reported and asserted rather than assumed.
    compared: u32,
    play_gate: u32,
    rec_gate: u32,
    /// Max per-axis |play - rec| over the available recording past each gate.
    gate_rel_drift: f32,
    /// The same comparison done the OLD way, index against index.
    index_drift: f32,
}

/// `rec` is a path, or the name of a committed recording under
/// `TAS/recordings/`; the default is FE-10065.
pub fn run(iterations: u32, rec: Option<&str>) -> bool {
    let compare_speed = COMPARE_SPEED;
    let name = rec.unwrap_or(RECORDING);
    let path = match Some(PathBuf::from(name))
        .filter(|p| p.is_file())
        .ok_or(())
        .or_else(|()| harness::fixture_path(name))
    {
        Ok(p) => p,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    println!(
        "=== Gate-relative input alignment: {} at {}x ===",
        path.display(),
        compare_speed
    );

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    let loaded = match replay::load_tasrec(&path) {
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
        let nonzero = s.input_log[..rec_gate as usize]
            .iter()
            .filter(|b| **b != 0)
            .count();
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
    let mut failed_attempts = 0u32;
    for i in 1..=iterations {
        // Alternate alignment, and sweep the arm delay so both arms see the
        // same spread of gates rather than whatever the machine felt like.
        let aligned = i % 2 == 0;
        let delay = ARM_DELAYS_MS[(i as usize / 2) % ARM_DELAYS_MS.len()];
        if let Some(a) = one_attempt(&mut client, rec_gate, aligned, delay, compare_speed) {
            println!(
                "  attempt {:>2} {:<9} d{:>2}ms play_gate={} (rec {}, off {:+}) | {}/{} ticks | {} bad from {:?} | race {} | gate-rel {:.5} | index {:.5}",
                i,
                if aligned { "ALIGNED" } else { "baseline" },
                delay,
                a.play_gate,
                a.rec_gate,
                a.play_gate as i64 - a.rec_gate as i64,
                a.compared,
                a.expected,
                a.bit_mismatches,
                a.first_mismatch,
                a.race_time_cs,
                a.gate_rel_drift,
                a.index_drift
            );
            attempts.push(a);
        } else {
            println!("  attempt {:>2}: failed", i);
            failed_attempts += 1;
        }
    }
    // A matched, unaligned baseline is the control. Natural gate jitter can
    // omit one from a small requested sample, so collect bounded extra controls
    // rather than turning a sound feature into a coin-flip test result.
    if !attempts
        .iter()
        .any(|a| !a.aligned && a.play_gate == rec_gate)
    {
        println!("  no matched baseline yet; collecting up to 12 control attempts");
        for extra in 0..12u32 {
            let delay = ARM_DELAYS_MS[extra as usize % ARM_DELAYS_MS.len()];
            match one_attempt(&mut client, rec_gate, false, delay, compare_speed) {
                Some(a) => {
                    println!(
                        "  control {:>2} d{:>2}ms play_gate={} (off {:+}) | {}/{} ticks | race {}",
                        extra + 1,
                        delay,
                        a.play_gate,
                        a.play_gate as i64 - rec_gate as i64,
                        a.compared,
                        a.expected,
                        a.race_time_cs
                    );
                    let matched = a.play_gate == rec_gate;
                    attempts.push(a);
                    if matched {
                        break;
                    }
                }
                None => {
                    println!("  control {:>2}: failed", extra + 1);
                    failed_attempts += 1;
                }
            }
        }
    }
    harness::stop(&mut client);
    report(&attempts, rec_gate, failed_attempts)
}

fn one_attempt(
    client: &mut TasSharedMemoryClient,
    rec_gate: u32,
    aligned: bool,
    delay_ms: u64,
    compare_speed: f32,
) -> Option<Attempt> {
    client.state_mut().playback_speed = compare_speed;
    client.state_mut().gate_index = 0;
    client.state_mut().gate_tick = 0;

    if !harness::restart_and_stabilize_inprocess(client) {
        return None;
    }
    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms));
    }
    // The in-process F5 resets game speed. Reassert after the restart, as the
    // product transport controller does on every step.
    client.state_mut().playback_speed = compare_speed;
    // AFTER the restart: the restart sends a STOP, and STOP clears alignment on
    // purpose so a stale value cannot re-index someone else's replay.
    client.state_mut().gate_align_rec = if aligned { rec_gate } else { 0 };
    harness::arm_play(client);

    let expected = client.state().recorded_count.saturating_sub(rec_gate);
    // Wait until the replay reaches its shifted endpoint, ends, or freezes at a
    // finish dialog with a stable real race time.
    let deadline = Instant::now() + Duration::from_secs(PLAY_TIMEOUT_SECS);
    let mut last_pos = 0u32;
    let mut last_progress = Instant::now();
    loop {
        // Mirror TransportController: F5 and game code may reset this field, so
        // a catch-up owner reasserts it for the lifetime of the replay.
        client.state_mut().playback_speed = compare_speed;
        if Instant::now() > deadline {
            eprintln!("    timed out");
            client.send_command(TasCommand::Stop);
            return None;
        }
        let pos = client.playback_pos_volatile();
        let mode = client.mode_volatile();
        let s = client.state();
        let g = s.gate_index;
        if pos != last_pos {
            last_pos = pos;
            last_progress = Instant::now();
        }
        if g != 0 && pos >= g.saturating_add(expected) {
            break;
        }
        if mode != TasMode::Play as u32 && pos > 0 {
            break;
        }
        if g != 0
            && pos > g
            && s.race_time_cs != u32::MAX
            && last_progress.elapsed() >= Duration::from_secs(FINISH_STALL_SECS)
        {
            println!(
                "    terminal finish stall at playback_pos={} race_time_cs={}",
                pos, s.race_time_cs
            );
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
    // What the aligned replay OWES us: every tick of the recording past its
    // gate. Anything less is a short replay, which is the failure the
    // arm-relative endpoint used to cause and a fixed window cannot see.
    let expected = expected
        .min(s.rec_coords.len().saturating_sub(rec_gate as usize) as u32)
        .min(s.play_coords.len().saturating_sub(play_gate as usize) as u32);
    let mut gate_rel = 0f32;
    let mut idx = 0f32;
    let mut compared = 0u32;
    let mut bit_mismatches = 0u32;
    let mut first_mismatch: Option<u32> = None;
    for k in 0..expected {
        let pi = (play_gate + k) as usize;
        let ri = (rec_gate + k) as usize;
        if pi >= s.play_coords.len() || ri >= s.rec_coords.len() || ri >= s.recorded_count as usize
        {
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
        if p[0].to_bits() != r[0].to_bits()
            || p[1].to_bits() != r[1].to_bits()
            || p[2].to_bits() != r[2].to_bits()
        {
            bit_mismatches += 1;
            if first_mismatch.is_none() {
                first_mismatch = Some(k);
            }
        }
        if !p[0].is_finite() || !p[1].is_finite() || !p[2].is_finite() {
            bit_mismatches += 1;
        }
        // Same window, compared index-against-index the old way.
        let r2 = s.rec_coords[pi.min(s.recorded_count as usize - 1)];
        idx = idx
            .max((p[0] - r2[0]).abs())
            .max((p[1] - r2[1]).abs())
            .max((p[2] - r2[2]).abs());
    }
    let race_time_cs = s.race_time_cs;
    thread::sleep(Duration::from_millis(200));
    let race_after = unsafe { std::ptr::read_volatile(&client.state().race_time_cs as *const u32) };
    let a = Attempt {
        aligned,
        race_time_cs,
        race_time_final: race_time_cs != u32::MAX && race_after == race_time_cs,
        bit_mismatches,
        first_mismatch,
        expected,
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

fn report(attempts: &[Attempt], rec_gate: u32, failed_attempts: u32) -> bool {
    println!("\n=== RESULT ===");
    let gates: Vec<u32> = attempts.iter().map(|a| a.play_gate).collect();
    let off: Vec<i64> = gates.iter().map(|g| *g as i64 - rec_gate as i64).collect();
    println!(
        "  play gates seen: {:?} (offsets from the recording: {:?})",
        gates, off
    );
    if failed_attempts > 0 {
        println!(
            "  FAIL: {} attempted run(s) produced no result",
            failed_attempts
        );
    }

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
    let bits = mismatched.iter().map(|a| a.bit_mismatches).sum::<u32>();
    println!(
        "  bit-level mismatches across mismatched-gate attempts: {}",
        bits
    );
    let offsets: Vec<i64> = mismatched
        .iter()
        .map(|a| a.play_gate as i64 - a.rec_gate as i64)
        .collect();
    println!("  offsets exercised by aligned attempts: {:?}", offsets);
    // THE CONTROL. Zero is the wrong bar: a replay that matches the gate
    // exactly is not bit-identical to the recording all the way to the end —
    // the last ~289 ticks diverge for it too, which is a pre-existing property
    // of the tail (the race finishes in there) and nothing to do with
    // alignment. The question is whether an aligned replay with a MISMATCHED
    // gate does as well as a baseline replay with a MATCHED one.
    let control: Vec<&Attempt> = attempts
        .iter()
        .filter(|a| !a.aligned && a.play_gate == a.rec_gate)
        .collect();
    if control.is_empty() {
        println!("\n  No matched-gate baseline ran — no control to judge against.");
        return false;
    }
    let ctl = control
        .iter()
        .copied()
        .max_by_key(|a| a.compared)
        .expect("control is non-empty");
    let ctl_bits = ctl.bit_mismatches;
    let ctl_drift = ctl.gate_rel_drift;
    let ctl_first = ctl.first_mismatch;
    let short = mismatched
        .iter()
        .filter(|a| a.compared < ctl.compared)
        .count();
    if short > 0 {
        println!(
            "  FAIL: {} aligned run(s) covered fewer gate-relative ticks than the matched control ({})",
            short, ctl.compared
        );
    }
    println!(
        "  control (baseline, gate matched): {} bit-mismatches from {:?}, drift {:.5}",
        ctl_bits, ctl_first, ctl_drift
    );

    // Race time, aligned against the control. A trajectory can agree while the
    // clock does not.
    let ctl_race: Vec<u32> = control.iter().map(|a| a.race_time_cs).collect();
    let ali_race: Vec<u32> = mismatched.iter().map(|a| a.race_time_cs).collect();
    println!(
        "  race time cs — control {:?}, aligned {:?}",
        ctl_race, ali_race
    );
    let race_ok = !ctl.race_time_final
        || mismatched.iter().all(|a| {
            a.race_time_final && a.race_time_cs != u32::MAX && a.race_time_cs == ctl.race_time_cs
        });
    if !ctl.race_time_final {
        println!("  race time is still running at the recording endpoint; not a finish verdict");
    }
    if !race_ok {
        println!("  FAIL: an aligned replay finished on a different race time");
    }

    let worst = mismatched
        .iter()
        .map(|a| a.gate_rel_drift)
        .fold(0.0, f32::max);
    let worst_bits = mismatched
        .iter()
        .map(|a| a.bit_mismatches)
        .max()
        .unwrap_or(u32::MAX);
    let worst_first = mismatched.iter().filter_map(|a| a.first_mismatch).min();
    let first_ok = mismatched
        .iter()
        .all(|a| match (a.first_mismatch, ctl_first) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(aligned), Some(control)) => aligned >= control,
        });
    println!(
        "\n  {} aligned attempts landed a different gate than the recording; worst drift {:.5}",
        mismatched.len(),
        worst
    );
    // No worse than the control, on every axis.
    let as_good = worst_bits <= ctl_bits
        && worst <= ctl_drift * 1.0001
        && first_ok
        && race_ok
        && short == 0
        && failed_attempts == 0;
    println!(
        "  aligned, gate mismatched:         {} bit-mismatches from {:?}, drift {:.5}",
        worst_bits, worst_first, worst
    );
    if as_good {
        println!("  *** Gate-offset input indexing matched the control in this sample. ***");
        println!("  Hidden spawn state is a separate variable; product PLAY must still watch it.");
        true
    } else {
        println!("  Alignment did NOT reproduce the run — something depends on the");
        println!("  absolute index, not just time since the race started.");
        false
    }
}
