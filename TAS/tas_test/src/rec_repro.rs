//! REC-vs-REC reproducibility ("scuffing catcher"): record the SAME driven
//! input pattern across two independent F5 restarts and compare the captured
//! input logs. Catches input scuffing — dropped presses, extra presses, and
//! systematic timing shifts between what was pressed and what got recorded.
//!
//! What this does NOT judge: trajectory equality. The two passes get their
//! real keys at different wall instants (OS delivery), so a ±few-tick jitter
//! per transition is physical, not a bug. The judge is therefore:
//!   - each pass captures the requested per-key edges and hold durations,
//!     and includes a stationary spawn followed by real movement;
//!   - same number of input transitions in both passes (nothing dropped or
//!     invented), and
//!   - each transition lands within TOL ticks of its counterpart after
//!     aligning both logs on their first transition (removes the arm→drive
//!     phase offset).

use crate::{harness, patterns};
use std::thread;
use std::time::Duration;
use tas_shared::input_bits;

/// Max per-transition timing skew after alignment. Real key delivery jitters
/// 1-3 ticks; a scuffed recorder shifts whole runs or drops edges.
const TOL_TICKS: i64 = 12;

/// (tick, mask) at each change of the input mask.
fn transitions(log: &[u8], count: u32) -> Vec<(u32, u8)> {
    let n = (count as usize).min(log.len());
    let mut out = Vec::new();
    let mut prev = 0u8;
    for (i, &m) in log.iter().enumerate().take(n) {
        if m != prev {
            out.push((i as u32, m));
            prev = m;
        }
    }
    out
}

fn record_pass(
    client: &mut tas_shared::TasSharedMemoryClient,
    steps: &[patterns::PatternStep],
    pass: u32,
) -> Option<(Vec<u8>, u32)> {
    println!("  Pass {}: restart + REC + drive pattern ...", pass);
    harness::focus_game();
    if !harness::restart_and_stabilize_inprocess(client) {
        eprintln!("ERROR: game not alive for restart (pass {})", pass);
        return None;
    }
    harness::arm_rec(client);
    if let Err(error) = harness::drive_pico_steps(steps) {
        eprintln!("{error}");
        harness::stop(client);
        return None;
    }
    thread::sleep(Duration::from_millis(300));
    let count = client.state().recorded_count;
    let log = client.state().input_log[..(count as usize).min(tas_shared::TAS_MAX_TICKS)].to_vec();
    harness::stop(client);
    println!("    recorded {} ticks", count);
    // Reuse this fresh capture for the spawn/countdown contract rather than
    // recording a separate neutral run solely for rec-start.
    let start = crate::rec_start::analyze_start(
        &client.state().rec_coords[..(count as usize).min(tas_shared::TAS_MAX_TICKS)],
        count as usize,
    );
    if !crate::rec_start::passes(&start) {
        eprintln!("FAIL: capture did not include a stationary spawn and subsequent motion");
        return None;
    }
    println!("    spawn/countdown and subsequent movement: PASS");
    Some((log, count))
}

pub fn run() -> bool {
    println!("=== REC-REPRO regression test (live) ===");
    println!(
        "  Invariant: recording the same driven input twice yields the same \
         transitions (none dropped/invented, timing within {} ticks).",
        TOL_TICKS
    );
    let mut client = harness::ensure_game_running();

    // Distinctive pattern: alternating taps + a long hold, ~8s total.
    let steps = patterns::build_from_explicit(&[
        (input_bits::LEFT, 60),
        (0x00, 40),
        (input_bits::RIGHT, 60),
        (0x00, 40),
        (input_bits::LEFT, 200),
        (0x00, 40),
        (input_bits::RIGHT, 60),
        (0x00, 300),
    ]);

    let Some((log_a, count_a)) = record_pass(&mut client, &steps, 1) else {
        println!("*** REC-REPRO INCONCLUSIVE (pass 1 failed) ***");
        return false;
    };
    let Some((log_b, count_b)) = record_pass(&mut client, &steps, 2) else {
        println!("*** REC-REPRO INCONCLUSIVE (pass 2 failed) ***");
        return false;
    };

    let ta = transitions(&log_a, count_a);
    for log in [&log_a, &log_b] {
        if let Err(error) = patterns::verify_capture(&steps, log, TOL_TICKS as u32) {
            eprintln!("FAIL: driven input was not captured: {error}");
            return false;
        }
    }
    let tb = transitions(&log_b, count_b);
    println!(
        "  pass1: {} transitions / {} ticks · pass2: {} transitions / {} ticks",
        ta.len(),
        count_a,
        tb.len(),
        count_b
    );

    if ta.is_empty() || tb.is_empty() {
        println!(
            "*** REC-REPRO FAILED: a pass recorded NO input transitions — keys \
             never reached the recorder (Pico/driver/cave1c gate problem) ***"
        );
        return false;
    }
    if ta.len() != tb.len() {
        println!(
            "*** REC-REPRO FAILED: transition count differs ({} vs {}) — \
             a press was dropped or invented between identical passes ***",
            ta.len(),
            tb.len()
        );
        return false;
    }

    // Align on the first transition of each pass, then compare pairwise.
    let base_a = ta[0].0 as i64;
    let base_b = tb[0].0 as i64;
    let mut worst: i64 = 0;
    for (i, ((tick_a, mask_a), (tick_b, mask_b))) in ta.iter().zip(tb.iter()).enumerate() {
        if mask_a != mask_b {
            println!(
                "*** REC-REPRO FAILED: transition {} differs in MASK \
                 ({:#04x} vs {:#04x}) — recorded keys don't match the driven input ***",
                i, mask_a, mask_b
            );
            return false;
        }
        let skew = ((*tick_a as i64 - base_a) - (*tick_b as i64 - base_b)).abs();
        worst = worst.max(skew);
        if skew > TOL_TICKS {
            println!(
                "*** REC-REPRO FAILED: transition {} skewed {} ticks between \
                 passes (> {}) — recorder timing is scuffed ***",
                i, skew, TOL_TICKS
            );
            return false;
        }
    }

    println!(
        "*** REC-REPRO OK: {} transitions match in order+mask, worst timing \
         skew {} ticks (≤ {}). Recording is reproducible across restarts. ***",
        ta.len(),
        worst,
        TOL_TICKS
    );
    true
}
