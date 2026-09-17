//! Steering-impact regression test: injected steering must MOVE the player,
//! and ONLY with a correct BB3B10 arg4.
//!
//! The game's input observer silently discards any injected key whose arg4
//! is wrong, so a stale value makes ALL injected steering a no-op; the drift
//! and CONT suites cannot see that because a straight-line replay is perfectly
//! reproducible. "arg4" is the hi dword of the 64-bit Kernel::Time every key
//! event is stamped with at dispatch (BB3B10(key, pressed, Time.lo, Time.hi)),
//! and the observer discards events whose Time predates the current race
//! context; the DLL stamps injections with the game's own
//! Kernel::Time::Current().
//!
//! Method: replay the same hard RIGHT twice via the `test_arg4_override` hook.
//!   Phase A: a deliberately WRONG Time.hi → injection discarded → boarder
//!            goes STRAIGHT.
//!   Phase B: override 0 → the live Kernel::Time::Current stamp → boarder
//!            TURNS, and the DLL reports arg4_source == TIME_CURRENT.

use crate::harness;
use std::thread;
use std::time::Duration;
use tas_shared::{input_bits, TasCommand, ARG4_SOURCE_TIME_CURRENT, TAS_MAX_TICKS};

/// Hold RIGHT from this tick (well past the ~183-frame spawn countdown so the
/// boarder is moving and steerable) ...
const STEER_START: usize = 360;
/// ... to this tick. ~200 ticks (2s) of sustained turn.
const STEER_END: usize = 560;
const TOTAL: u32 = 580;
/// A real sustained RIGHT swings the lateral (X) axis ~10 units (live runs:
/// 9.7, 10.4).
const TURN_MIN: f64 = 6.0;
/// A discarded injection leaves the boarder on its natural line — NOT zero
/// lateral swing (terrain drifts it ~2.3 on Forest Easy), so the "discarded"
/// judge is a CONTRAST: the wrong-stamp run must swing at least this much
/// less than the live-stamp run.
const CONTRAST_MIN: f64 = 5.0;
/// A Time.hi the game's clock can't be at (live values are small — machine
/// uptime in coarse units; 0xBADBAD is centuries away) — guaranteed "wrong".
const BOGUS_ARG4: u32 = 0x00BA_DBAD;

/// Replay a hard RIGHT once with the given arg4 override and return the lateral
/// (X) swing over the steered span plus the DLL-reported `arg4_source` for the
/// run's injections. `override_arg4 == 0` means "use the DLL's live
/// Kernel::Time::Current stamp".
fn replay_turn_swing(
    client: &mut tas_shared::TasSharedMemoryClient,
    override_arg4: u32,
) -> Option<(f64, u32)> {
    if !harness::restart_and_stabilize_inprocess(client) {
        eprintln!("ERROR: game not alive for restart");
        return None;
    }
    {
        let s = client.state_mut();
        s.test_arg4_override = override_arg4;
        s.arg4_source = 0; // cleared so this run's injections re-stamp it
        for i in 0..TOTAL as usize {
            s.input_log[i] = if (STEER_START..STEER_END).contains(&i) {
                input_bits::RIGHT
            } else {
                0
            };
        }
        s.recorded_count = TOTAL;
    }
    harness::arm_play(client);
    if !harness::wait_playback(client, STEER_END as u32) {
        eprintln!("ERROR: playback did not reach the steered span");
        harness::stop(client);
        client.state_mut().test_arg4_override = 0;
        return None;
    }
    thread::sleep(Duration::from_millis(200)); // settle the last captured coords
    let end = STEER_END.min(TAS_MAX_TICKS);
    let coords: Vec<[f32; 3]> = client.state().play_coords[..end].to_vec();
    let arg4_source = client.state().arg4_source;
    harness::stop(client);
    client.state_mut().test_arg4_override = 0; // always restore

    let end = end.min(coords.len());
    if end <= STEER_START + 50 {
        eprintln!(
            "  (replay covered too few steered frames: {})",
            end.saturating_sub(STEER_START)
        );
        return None;
    }
    let xs = &coords[STEER_START..end];
    let xmin = xs.iter().map(|c| c[0] as f64).fold(f64::INFINITY, f64::min);
    let xmax = xs
        .iter()
        .map(|c| c[0] as f64)
        .fold(f64::NEG_INFINITY, f64::max);
    Some((xmax - xmin, arg4_source))
}

pub fn run() -> bool {
    println!("=== STEER-IMPACT regression test (live) ===");
    println!(
        "  Invariant: replaying a hard RIGHT turns the boarder with a correct \
         BB3B10 arg4, and is silently discarded (straight) with a wrong one."
    );
    let mut client = harness::ensure_game_running();

    // Phase A: a deliberately-wrong Time.hi — injection must be discarded.
    println!(
        "  Phase A: forced WRONG arg4 (0x{:06X}) — expect NO turn ...",
        BOGUS_ARG4
    );
    let Some((wrong_swing, _)) = replay_turn_swing(&mut client, BOGUS_ARG4) else {
        println!("*** STEER-IMPACT INCONCLUSIVE (phase A did not produce a usable replay) ***");
        return false;
    };
    println!("    phase A lateral swing = {:.3}", wrong_swing);

    // Phase B: the live Kernel::Time::Current stamp — injection must steer.
    println!("  Phase B: live Time stamp (override 0) — expect a turn via Time::Current ...");
    let Some((live_swing, live_source)) = replay_turn_swing(&mut client, 0) else {
        println!("*** STEER-IMPACT INCONCLUSIVE (phase B did not produce a usable replay) ***");
        return false;
    };
    println!(
        "    phase B lateral swing = {:.3}, arg4_source = {} (1 = Time::Current)",
        live_swing, live_source
    );

    // Make sure we left the override cleared and the game stopped.
    client.send_command(TasCommand::Stop);

    let b_ok = live_swing >= TURN_MIN; // live Time -> turn
    let a_ok = wrong_swing <= live_swing - CONTRAST_MIN; // wrong Time -> discarded
    let src_ok = live_source == ARG4_SOURCE_TIME_CURRENT; // the proper path fired
    if a_ok && b_ok && src_ok {
        println!(
            "*** STEER-IMPACT OK: wrong Time → natural line ({:.2}, ≥{:.1} under the \
             live run), live Kernel::Time::Current → turn ({:.2} ≥ {:.1}). Injection \
             is Time-gated and steers via the game's own clock. ***",
            wrong_swing, CONTRAST_MIN, live_swing, TURN_MIN
        );
        return true;
    }
    if !b_ok {
        println!(
            "*** STEER-IMPACT FAILED: the live Time stamp did NOT steer the boarder (swing \
             {:.2} < {:.1}) — injected steering is dead (broken inject path / Time stamp) ***",
            live_swing, TURN_MIN
        );
    } else if !a_ok {
        println!(
            "*** STEER-IMPACT FAILED: a WRONG event Time still steered (swing {:.2} vs live \
             {:.2}, contrast < {:.1}) — injection is ignoring the Time stamp (the guard can't \
             detect a real-world stale stamp) ***",
            wrong_swing, live_swing, CONTRAST_MIN
        );
    }
    if !src_ok {
        println!(
            "*** STEER-IMPACT FAILED: arg4_source = {} (expected 1 = Kernel::Time::Current) — \
             the proper Time path didn't fire (export unresolved → calibrated fallback?) ***",
            live_source
        );
    }
    false
}
