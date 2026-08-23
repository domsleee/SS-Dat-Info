//! Reconstruct the game's sub-tick residual, and use it to predict the gate.
//!
//! Everything observable at tick resolution has been ruled out with samples:
//! the player position at the arm is bit-identical across cycles that go on to
//! produce different gates, the clock-vs-tick phase is identical, and the
//! engine clock is sampled once per frame at a tick boundary so it carries no
//! sub-tick information either. The deciding quantity lives inside the game's
//! own tick accumulator.
//!
//! It does not have to be found in memory. The game's rule is
//!
//!     ticks  = floor((now - prev) / tick_len)
//!     prev  += ticks * tick_len
//!
//! and cave2's trace records `now` (the engine's absolute 10MHz clock) and
//! `tick_count` every frame — so the ticks the game emitted are the difference
//! of consecutive tick_counts, and the residual `now - prev` follows by
//! recurrence:
//!
//!     residual_{n+1} = (now_{n+1} - now_n) + residual_n - ticks_n * tick_len
//!
//! The initial residual is unknown, but it is pinned by observation: every
//! frame asserts `ticks_n <= residual_n / tick_len < ticks_n + 1`, and a few
//! hundred such constraints leave only a narrow band. This solves for it, then
//! checks whether the residual at the reset predicts which tick the boarder
//! leaves on.

use std::thread;
use std::time::{Duration, Instant};


use crate::harness;

const GATE_TIMEOUT_SECS: u64 = 20;

struct Frame {
    tick: u32,
    pos: [u32; 3],
    now: u64,
    physics: u32,
}

struct Run {
    press: u32,
    gate: u32,
    speed: f32,
    press_seq: u32,
    arm_seq: u32,
    gate_seq: u32,
    first_moving: u32,
    frames: Vec<Frame>,
}

pub fn run(iterations: u32) -> bool {
    println!("=== Reconstructing the sub-tick residual ===");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));
    harness::focus_game();

    let mut runs: Vec<Run> = Vec::new();
    for run in 1..=iterations {
        {
            let s = client.state_mut();
            s.gate_tick = 0;
            s.trace_count = 0;
            s.f5_press_tick = 0;
        }
        // Alternate 1x and 2x. THE decisive test. If the countdown is a fixed
        // number of SIMULATION ticks, gate-reset is unchanged by speed and the
        // gate is predictable from state at the arm. If it is a fixed amount of
        // REAL time, gate-reset scales with speed — and then the tick it lands
        // on depends on frame pacing that has not happened yet, so no arm-time
        // prediction can ever be exact, however much state we read.
        let speed: f32 = if run % 2 == 0 { 2.0 } else { 1.0 };
        client.state_mut().playback_speed = speed;
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  run {}: restart failed", run);
            continue;
        }
        harness::arm_rec(&mut client);
        let deadline = Instant::now() + Duration::from_secs(GATE_TIMEOUT_SECS);
        while Instant::now() < deadline {
            if client.state().gate_tick != 0 {
                break;
            }
            thread::sleep(Duration::from_millis(2));
        }
        // Ride briefly so the NEXT restart has a visible teleport.
        thread::sleep(Duration::from_millis(600));

        let s = client.state();
        let n = (s.trace_count as usize).min(tas_shared::TRACE_FRAMES);
        let frames: Vec<Frame> = (0..n)
            .map(|i| Frame {
                tick: s.trace[i][0],
                pos: [s.trace[i][1], s.trace[i][2], s.trace[i][3]],
                now: ((s.trace[i][5] as u64) << 32) | s.trace[i][4] as u64,
                physics: s.trace[i][6],
            })
            .collect();
        let r = Run {
            press: s.f5_press_tick,
            gate: s.gate_tick,
            speed,
            press_seq: s.press_seq,
            arm_seq: s.arm_seq,
            gate_seq: s.gate_seq,
            first_moving: s.gate_index,
            frames,
        };
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(150));
        if r.frames.len() > 100 && r.gate != 0 {
            runs.push(r);
        }
    }

    if runs.len() < 4 {
        println!("not enough runs captured");
        return false;
    }
    analyse(&runs)
}

/// The reset is the first frame at the SPAWN, and the spawn is whatever the
/// boarder was holding just before it left. Deriving it from the gate rather
/// than from "the first position change" matters: when the previous cycle ended
/// at the spawn there is no teleport, and the first change in the trace is the
/// GATE — which is how six of ten runs once reported a reset equal to their own
/// gate.
fn find_reset(r: &Run) -> Option<usize> {
    let gate_idx = r.frames.iter().position(|f| f.tick == r.gate)?;
    if gate_idx == 0 {
        return None;
    }
    let spawn = r.frames[gate_idx - 1].pos;
    r.frames[..gate_idx].iter().position(|f| f.pos == spawn)
}

fn analyse(runs: &[Run]) -> bool {
    // 1. Is the countdown simulation time or real time? Everything else depends
    //    on the answer: real time would mean the gate depends on frame pacing
    //    that has not happened at arm time, and no prediction could be exact.
    // THE UNIT TEST, in the literal sense. first_moving is a cave2 CYCLE
    // ordinal; tick_count is batched. Measured in cycles, does the countdown
    // collapse to one value?
    // The TEARDOWN, as a reset signal that cannot be missed. Position cannot
    // see a reset when the boarder was already at the spawn — but the level
    // reload writes a null/zero player state on the way through, and that
    // happens whatever the boarder was doing. If the gate is a fixed number of
    // cycles from the LAST teardown frame, that is the reference.
    println!("-- teardown (zero position) as the reset reference --");
    {
        let mut ds: Vec<i64> = Vec::new();
        for (i, r) in runs.iter().enumerate() {
            let zero = [0u32, 0, 0];
            let last_zero = r.frames.iter().rposition(|f| f.pos == zero);
            let gi = r.frames.iter().position(|f| f.tick == r.gate);
            match (last_zero, gi) {
                (Some(z), Some(g)) if g > z => {
                    let d = g as i64 - z as i64;
                    println!("  run {:>2}: teardownF={} gateF={} cycles={}", i + 1, z, g, d);
                    ds.push(d);
                }
                (None, _) => println!("  run {:>2}: no teardown frame seen", i + 1),
                _ => println!("  run {:>2}: teardown/gate ordering unusable", i + 1),
            }
        }
        ds.sort_unstable();
        let n = ds.len();
        ds.dedup();
        println!("  distinct cycles from teardown to gate: {:?} (over {} runs)", ds, n);
        if ds.len() == 1 && n > 1 {
            println!("  *** CONSTANT: the teardown frame predicts the gate exactly ***");
        }
    }

    println!("-- countdown measured in cave2 CYCLES --");
    let mut kp: Vec<i64> = runs.iter().map(|r| r.gate_seq as i64 - r.press_seq as i64).collect();
    let mut ka: Vec<i64> = runs.iter().map(|r| r.gate_seq as i64 - r.arm_seq as i64).collect();
    let mismatch = runs.iter().filter(|r| (r.gate_seq as i64 - r.arm_seq as i64) != r.first_moving as i64).count();
    kp.sort_unstable(); ka.sort_unstable();
    println!("  gate_seq - press_seq : {:?}", kp);
    println!("  gate_seq - arm_seq   : {:?}", ka);
    println!("  first_moving         : {:?}", runs.iter().map(|r| r.first_moving).collect::<Vec<_>>());
    println!("  (gate_seq-arm_seq) != first_moving on {}/{}", mismatch, runs.len());
    kp.dedup();
    if kp.len() == 1 {
        println!("  *** CONSTANT in cycles: the 310/311 split was tick_count aliasing ***");
    } else {
        println!("  still {} distinct values in cycles", kp.len());
    }

    println!("-- countdown length by simulation speed --");
    for sp in [1.0f32, 2.0] {
        let mut v: Vec<i64> = runs
            .iter()
            .filter(|r| r.speed == sp)
            .filter_map(|r| find_reset(r).map(|i| r.gate as i64 - r.frames[i].tick as i64))
            .collect();
        if v.is_empty() {
            continue;
        }
        v.sort_unstable();
        println!("  SPEED {}x: gate-reset {:?}", sp, v);
    }
    println!("  (unchanged => fixed SIM TICKS; scaled => fixed REAL TIME)");

    // 2. Every candidate reference, side by side. The countdown is tick-based,
    //    so SOME reference makes it constant — read off which column that is
    //    rather than testing one guess per run.
    //
    //    nDup matters: several cave2 frames can carry the same tick_count, so
    //    "the tick of the reset frame" is ambiguous, and which frame within the
    //    tick the reset lands on may be the whole residual.
    println!();
    println!("  run spd | resetF resetT posInTick | gateF gateT | dFrames dTicks");
    let mut by_dframes: Vec<i64> = Vec::new();
    let mut by_dticks: Vec<i64> = Vec::new();
    for (i, r) in runs.iter().enumerate() {
        let Some(ri) = find_reset(r) else {
            println!("  {:>3}     | no reset visible", i + 1);
            continue;
        };
        let Some(gi) = r.frames.iter().position(|f| f.tick == r.gate) else {
            println!("  {:>3}     | no gate frame", i + 1);
            continue;
        };
        let rt = r.frames[ri].tick;
        let first_of_tick = r.frames.iter().position(|f| f.tick == rt).unwrap_or(ri);
        let ndup = r.frames.iter().filter(|f| f.tick == rt).count();
        let df = gi as i64 - ri as i64;
        let dt = r.gate as i64 - rt as i64;
        println!(
            "  {:>3} {:>2}x | {:>6} {:>6} {:>4}of{:<3} | {:>5} {:>6} | {:>7} {:>6}",
            i + 1, r.speed, ri, rt, ri - first_of_tick + 1, ndup, gi, r.gate, df, dt
        );
        by_dframes.push(df);
        by_dticks.push(dt);
    }

    by_dframes.sort_unstable();
    by_dticks.sort_unstable();
    by_dframes.dedup();
    by_dticks.dedup();
    println!();
    // Where does the physics pointer change? Position cannot see a reset when
    // the boarder was already AT the spawn; a reallocated sub-object can.
    println!();
    println!("  run | physChangeF physChangeT | gateT | gate-physChange");
    let mut by_phys: Vec<i64> = Vec::new();
    for (i, r) in runs.iter().enumerate() {
        let mut at = None;
        for n in 1..r.frames.len() {
            if r.frames[n].physics != r.frames[n - 1].physics {
                at = Some(n);
            }
        }
        match at {
            Some(n) => {
                let d = r.gate as i64 - r.frames[n].tick as i64;
                println!(
                    "  {:>3} | {:>11} {:>11} | {:>5} | {}",
                    i + 1, n, r.frames[n].tick, r.gate, d
                );
                by_phys.push(d);
            }
            None => println!("  {:>3} | physics pointer never changed", i + 1),
        }
    }
    by_phys.sort_unstable();
    by_phys.dedup();
    println!("  distinct gate-physChange: {:?}", by_phys);

    println!("  distinct dFrames: {:?}", by_dframes);
    println!("  distinct dTicks : {:?}", by_dticks);
    let exact = by_dticks.len() == 1 || by_dframes.len() == 1;
    if exact {
        println!("  => one of them is CONSTANT: that reference predicts the gate exactly.");
    } else {
        println!("  => neither is constant yet.");
    }
    exact
}
