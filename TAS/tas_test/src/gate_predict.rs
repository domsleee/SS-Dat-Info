//! Can the countdown gate be predicted EXACTLY, not just to +/-1?
//!
//! The tick model says `first_moving = K - (arm_tick - restart_tick)`, and it
//! measured K at 300 or 301 — a spread of one. One tick is not a prediction, it
//! is a coin flip narrowed, so this asks where that tick comes from.
//!
//! The suspicion: `tick_count` is OUR counter, incremented once per
//! Supreme::Cycle. The countdown is not compared against it. The game compares
//! its OWN 16-bit centisecond clock (SG+0x1D5334), and race_timer.hpp already
//! records that the two are not perfectly phase-locked — "(clock - cs)
//! occasionally lands +/-1 off (sub-tick rounding; the same wobble you see as
//! start_ts 477<->478)". If that is the same wobble, then the countdown is a
//! FIXED number of the game's units while being a wobbling number of ours, and
//! sampling the game's clock recovers exactly the phase our tick counter throws
//! away.
//!
//! So this samples both clocks at the same three moments — restart, arm, gate —
//! and puts the two models head to head on the same cycles:
//!
//!     tick model   first_moving = K_tick - (arm_tick - restart_tick)
//!     clock model  first_moving = (restart_clk + K_clk) - arm_clk
//!
//! A model is only useful here if its error is ZERO every time. Anything else
//! still needs the replay to settle it.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasSharedMemoryClient;

use crate::harness;

/// Arm offsets to sweep, in milliseconds after the restart completes. Spread
/// well past one tick (10ms) so a one-for-one trade shows up as a clear slope
/// rather than as noise.
const ARM_DELAYS_MS: &[u64] = &[0, 5, 10, 20, 30, 40];
const GATE_TIMEOUT_SECS: u64 = 20;

/// The game clock is 16 bits and wraps at 65536.
fn clk_diff(a: u32, b: u32) -> i64 {
    (a.wrapping_sub(b) & 0xFFFF) as i64
}

#[derive(Debug, Clone, Copy)]
struct Cycle {
    delay_ms: u64,
    restart_tick: u32,
    arm_tick: u32,
    gate_tick: u32,
    first_moving: u32,
    restart_clk: u32,
    arm_clk: u32,
    gate_clk: u32,
    arm_reset_tick: u32,
    gate_reset_tick: u32,
    arm_pos: [u32; 3],
    reset_qpc: u64,
    arm_qpc: u64,
    gate_qpc: u64,
    arm_secs: u64,
    gate_secs: u64,
    f5_press_tick: u32,
    f5_press_qpc: u64,
}

impl Cycle {
    /// Countdown length measured from OUR F5 hold finishing.
    fn k_tick(&self) -> i64 {
        self.gate_tick as i64 - self.restart_tick as i64
    }
    /// Countdown length measured from the LEVEL RESET — what the game
    /// actually starts counting from.
    fn k_reset(&self) -> i64 {
        self.gate_tick as i64 - self.arm_reset_tick as i64
    }
    /// Was the reset already final when this cycle armed? If not, it explains
    /// the past but cannot predict the future.
    fn reset_known_at_arm(&self) -> bool {
        self.arm_reset_tick != 0 && self.arm_reset_tick < self.gate_tick
    }
    fn arm_offset_reset(&self) -> i64 {
        self.arm_tick as i64 - self.arm_reset_tick as i64
    }
    /// Countdown length in the GAME's centiseconds.
    fn k_clk(&self) -> i64 {
        clk_diff(self.gate_clk, self.restart_clk)
    }
    fn arm_offset_tick(&self) -> i64 {
        self.arm_tick as i64 - self.restart_tick as i64
    }
    fn arm_offset_clk(&self) -> i64 {
        clk_diff(self.arm_clk, self.restart_clk)
    }
    /// Do the two clocks stay in lockstep from the arm to the gate? If not, no
    /// amount of phase information at the arm can survive the trip.
    fn drift_arm_to_gate(&self) -> i64 {
        clk_diff(self.gate_clk, self.arm_clk) - (self.gate_tick as i64 - self.arm_tick as i64)
    }
}

pub fn run(iterations: u32) -> bool {
    println!("=== Is the countdown gate predictable EXACTLY? ===");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));
    harness::focus_game();

    let mut cycles: Vec<Cycle> = Vec::new();
    for round in 1..=iterations {
        for &delay_ms in ARM_DELAYS_MS {
            match one_cycle(&mut client, delay_ms) {
                Some(c) => {
                    println!(
                        "  r{} d{:>2}ms | tick R={} A={} G={} K={} off={} fm={} | clk R={} A={} G={} K={} off={} drift={}",
                        round, delay_ms,
                        c.restart_tick, c.arm_tick, c.gate_tick, c.k_tick(), c.arm_offset_tick(), c.first_moving,
                        c.restart_clk, c.arm_clk, c.gate_clk, c.k_clk(), c.arm_offset_clk(), c.drift_arm_to_gate()
                    );
                    println!(
                        "         armPos ({:.4}, {:.4}, {:.4})  armQPCdelta={} accumAtArm={} accumAtGate={}",
                        f32::from_bits(c.arm_pos[0]),
                        f32::from_bits(c.arm_pos[1]),
                        f32::from_bits(c.arm_pos[2]),
                        c.arm_qpc, c.arm_secs, c.gate_secs
                    );
                    cycles.push(c);
                }
                None => println!("  r{} d{:>2}ms: no gate within budget", round, delay_ms),
            }
            harness::stop(&mut client);
            thread::sleep(Duration::from_millis(120));
        }
    }

    if cycles.len() < 4 {
        println!("\nnot enough cycles completed to conclude anything");
        return false;
    }
    analyse(&cycles)
}

fn one_cycle(client: &mut TasSharedMemoryClient, delay_ms: u64) -> Option<Cycle> {
    {
        let s = client.state_mut();
        s.gate_tick = 0;
        s.gate_index = 0;
        s.gate_clk = 0;
        s.restart_done_tick = 0;
        s.arm_consumed_tick = 0;
    }
    if !harness::restart_and_stabilize_inprocess(client) {
        eprintln!("    restart failed");
        return None;
    }
    if delay_ms > 0 {
        thread::sleep(Duration::from_millis(delay_ms));
    }
    harness::arm_rec(client);

    let deadline = Instant::now() + Duration::from_secs(GATE_TIMEOUT_SECS);
    while Instant::now() < deadline {
        let s = client.state();
        if s.gate_tick != 0 && s.restart_done_tick != 0 && s.arm_consumed_tick != 0 {
            // Ride for a moment before this cycle ends. The reset is detected as
            // the position changing, so the boarder has to be somewhere else
            // first; stopping at the gate leaves it AT the spawn and the next
            // restart's teleport is invisible.
            thread::sleep(Duration::from_millis(600));
            return Some(Cycle {
                delay_ms,
                restart_tick: s.restart_done_tick,
                arm_tick: s.arm_consumed_tick,
                gate_tick: s.gate_tick,
                first_moving: s.gate_index,
                restart_clk: s.restart_clk,
                arm_clk: s.arm_clk,
                gate_clk: s.gate_clk,
                arm_reset_tick: s.arm_reset_tick,
                gate_reset_tick: s.gate_reset_tick,
                arm_pos: [s.arm_pos_x, s.arm_pos_y, s.arm_pos_z],
                reset_qpc: ((s.reset_qpc_hi as u64) << 32) | s.reset_qpc_lo as u64,
                arm_qpc: ((s.arm_qpc_hi as u64) << 32) | s.arm_qpc_lo as u64,
                gate_qpc: ((s.gate_qpc_hi as u64) << 32) | s.gate_qpc_lo as u64,
                arm_secs: ((s.arm_secs_hi as u64) << 32) | s.arm_secs_lo as u64,
                gate_secs: ((s.gate_secs_hi as u64) << 32) | s.gate_secs_lo as u64,
                f5_press_tick: s.f5_press_tick,
                f5_press_qpc: ((s.f5_press_qpc_hi as u64) << 32) | s.f5_press_qpc_lo as u64,
            });
        }
        thread::sleep(Duration::from_millis(2));
    }
    None
}

fn analyse(cycles: &[Cycle]) -> bool {
    println!("\n=== ANALYSIS ({} cycles) ===", cycles.len());

    let mut kt: Vec<i64> = cycles.iter().map(|c| c.k_tick()).collect();
    kt.sort_unstable();
    println!("\n-- countdown length in OUR ticks (gate_tick - restart_tick) --");
    print_histogram(&kt);

    // The model that matters: measured from the level reset rather than from
    // our F5 hold finishing.
    let mut kr: Vec<i64> = cycles.iter().map(|c| c.k_reset()).collect();
    kr.sort_unstable();
    println!("\n-- countdown length from the LEVEL RESET (gate_tick - reset_tick) --");
    print_histogram(&kr);
    let stale = cycles.iter().filter(|c| !c.reset_known_at_arm()).count();
    println!(
        "  reset already final at the arm on {}/{} cycles",
        cycles.len() - stale,
        cycles.len()
    );

    let mut kc: Vec<i64> = cycles.iter().map(|c| c.k_clk()).collect();
    kc.sort_unstable();
    println!("\n-- countdown length in the GAME's centiseconds (gate_clk - restart_clk) --");
    print_histogram(&kc);

    let mut drift: Vec<i64> = cycles.iter().map(|c| c.drift_arm_to_gate()).collect();
    drift.sort_unstable();
    println!("\n-- clock-vs-tick drift from the arm to the gate --");
    print_histogram(&drift);
    println!("  (non-zero means phase measured at the arm cannot survive to the gate)");

    // Head to head. Both use the median countdown length in their own units;
    // the question is only which one predicts the SAME first_moving every time.
    let kt_med = kt[kt.len() / 2];
    let kc_med = kc[kc.len() / 2];

    let mut err_tick: Vec<i64> = cycles
        .iter()
        .map(|c| c.first_moving as i64 - (kt_med - c.arm_offset_tick()))
        .collect();
    err_tick.sort_unstable();
    println!(
        "\n-- TICK model error: fm - (K_tick - arm_offset_tick), K={} --",
        kt_med
    );
    print_histogram(&err_tick);

    let mut err_clk: Vec<i64> = cycles
        .iter()
        .map(|c| c.first_moving as i64 - (kc_med - c.arm_offset_clk()))
        .collect();
    err_clk.sort_unstable();
    println!(
        "\n-- CLOCK model error: fm - (K_clk - arm_offset_clk), K={} --",
        kc_med
    );
    print_histogram(&err_clk);

    // THE QUESTION: does the settle frame the arm landed on determine the
    // countdown length? If every distinct arm position carries exactly one K,
    // then the position IS the missing phase and the gate is exactly
    // predictable from state that already exists when the cycle arms.
    println!("\n-- countdown length grouped by the position the arm captured --");
    let mut groups: Vec<([u32; 3], Vec<i64>)> = Vec::new();
    for c in cycles {
        match groups.iter_mut().find(|(p, _)| *p == c.arm_pos) {
            Some((_, ks)) => ks.push(c.k_tick()),
            None => groups.push((c.arm_pos, vec![c.k_tick()])),
        }
    }
    let mut ambiguous = 0usize;
    for (pos, ks) in &mut groups {
        ks.sort_unstable();
        let distinct = {
            let mut d = ks.clone();
            d.dedup();
            d
        };
        if distinct.len() > 1 {
            ambiguous += 1;
        }
        println!(
            "  z={:>12.5} n={:<3} K={:?}{}",
            f32::from_bits(pos[2]),
            ks.len(),
            distinct,
            if distinct.len() > 1 {
                "   <-- AMBIGUOUS"
            } else {
                ""
            }
        );
    }
    println!(
        "  {} distinct arm positions, {} of them ambiguous about K",
        groups.len(),
        ambiguous
    );
    if ambiguous == 0 && groups.len() > 1 {
        println!("  => the position the arm captured DETERMINES the countdown length.");
    }

    // The sub-tick test. If the countdown's start stamp is the QPC at the reset
    // frame, gate_qpc - reset_qpc is a constant 3 seconds and the residual
    // below is far smaller than one tick (~99,999 units at 10MHz).
    // THE REFERENCE-POINT TEST. The countdown is 3.10000s = 310.003 ticks, and
    // gate - restart_done measured 300/301 — a gap of ~10, which is exactly
    // RESTART_F5_HOLD_FRAMES. If the level resets on the PRESS, measuring from
    // there should collapse the spread that measuring from the release created.
    println!(
        "
-- countdown length from the F5 PRESS (gate_tick - f5_press_tick) --"
    );
    let mut kp: Vec<i64> = cycles
        .iter()
        .map(|c| c.gate_tick as i64 - c.f5_press_tick as i64)
        .collect();
    kp.sort_unstable();
    print_histogram(&kp);
    let mut errp: Vec<i64> = {
        let med = kp[kp.len() / 2];
        cycles
            .iter()
            .map(|c| c.first_moving as i64 - (med - (c.arm_tick as i64 - c.f5_press_tick as i64)))
            .collect()
    };
    errp.sort_unstable();
    println!("-- F5-PRESS model error: fm - (K_press - (arm_tick - press_tick)) --");
    print_histogram(&errp);
    let worst_press = errp.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
    println!(
        "  worst |error| {}, exact on {}/{}",
        worst_press,
        errp.iter().filter(|e| **e == 0).count(),
        cycles.len()
    );
    if worst_press == 0 {
        println!("  => EXACT. The countdown starts at the F5 press.");
    }

    // THE QPC MODEL. Stop counting ticks and do the arithmetic the game does:
    // the gate is the first tick at or after press + 3.1s, and ticks are ~99,999
    // apart at 10MHz. Every term is a measured QPC value, so if the countdown
    // duration is genuinely fixed this is exact by construction.
    println!(
        "
-- QPC: press -> gate, and the measured tick length --"
    );
    let mut span: Vec<i64> = cycles
        .iter()
        .map(|c| c.gate_qpc as i64 - c.f5_press_qpc as i64)
        .collect();
    span.sort_unstable();
    if span.last().copied().unwrap_or(0) == 0 {
        println!("  no QPC captured");
    } else {
        println!(
            "  press->gate  min={} max={} spread={} ({:.4} ticks)",
            span[0],
            span[span.len() - 1],
            span[span.len() - 1] - span[0],
            (span[span.len() - 1] - span[0]) as f64 / 99_999.0
        );
        let mut tl: Vec<f64> = cycles
            .iter()
            .filter(|c| c.first_moving > 0)
            .map(|c| (c.gate_qpc as i64 - c.arm_qpc as i64) as f64 / c.first_moving as f64)
            .collect();
        tl.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!(
            "  tick length  min={:.3} max={:.3} median={:.3} QPC units",
            tl[0],
            tl[tl.len() - 1],
            tl[tl.len() / 2]
        );

        // Predict fm purely from QPC measured at the arm.
        let dur = span[span.len() / 2];
        let tick = tl[tl.len() / 2];
        let mut errq: Vec<i64> = cycles
            .iter()
            .map(|c| {
                let remaining = (c.f5_press_qpc as i64 + dur) - c.arm_qpc as i64;
                let predicted = (remaining as f64 / tick).ceil() as i64;
                c.first_moving as i64 - predicted
            })
            .collect();
        errq.sort_unstable();
        println!(
            "-- QPC model error: fm - ceil((press_qpc + {} - arm_qpc) / {:.1}) --",
            dur, tick
        );
        print_histogram(&errq);
        let worst = errq.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
        println!(
            "  worst |error| {}, exact on {}/{}",
            worst,
            errq.iter().filter(|e| **e == 0).count(),
            cycles.len()
        );
        if worst == 0 {
            println!("  => EXACT, from QPC measured at the arm.");
        }
    }

    // THE SUB-TICK TEST. Seconds accumulated since the reset, at the gate.
    // If the countdown trips at a fixed elapsed time, this is constant; and the
    // FRACTIONAL part at the arm is then exactly what decides 300 vs 301.
    println!(
        "
-- engine SECONDS since the reset, at the gate --"
    );
    let mut gs: Vec<i64> = cycles.iter().map(|c| c.gate_secs as i64).collect();
    gs.sort_unstable();
    if gs.last().copied().unwrap_or(0) == 0 {
        println!("  all zero — the accumulator never ran");
    } else {
        println!(
            "  min={} max={} spread={} ({:.3} ticks of 99999)",
            gs[0],
            gs[gs.len() - 1],
            gs[gs.len() - 1] - gs[0],
            (gs[gs.len() - 1] - gs[0]) as f64 / 99_999.0
        );
        for k in [300i64, 301] {
            let mut v: Vec<i64> = cycles
                .iter()
                .filter(|c| c.k_tick() == k)
                .map(|c| (c.arm_secs as i64).rem_euclid(99_999))
                .collect();
            if v.is_empty() {
                continue;
            }
            v.sort_unstable();
            println!(
                "  K={}: sub-tick remainder at the arm  min={} max={} n={}",
                k,
                v[0],
                v[v.len() - 1],
                v.len()
            );
        }
        println!("  (if the two K groups' fraction ranges do not overlap, the gate is");
        println!("   an exact function of the accumulator at the arm)");
    }

    println!(
        "
-- QPC from the reset frame to the gate frame (10MHz) --"
    );
    let mut span: Vec<i64> = cycles
        .iter()
        .map(|c| c.gate_qpc as i64 - c.reset_qpc as i64)
        .collect();
    span.sort_unstable();
    if span.first() == Some(&0) && span.last() == Some(&0) {
        println!("  all zero — the engine clock was not published at these frames");
    } else {
        let lo = span[0];
        let hi = span[span.len() - 1];
        println!(
            "  min={} max={} spread={} ({:.3} ticks)",
            lo,
            hi,
            hi - lo,
            (hi - lo) as f64 / 99_999.0
        );
        println!(
            "  median={} ({:.4} seconds)",
            span[span.len() / 2],
            span[span.len() / 2] as f64 / 10_000_000.0
        );
        // Grouped by the countdown length it produced: if the QPC span is what
        // decides, the two groups must separate cleanly.
        for k in [300i64, 301] {
            let mut v: Vec<i64> = cycles
                .iter()
                .filter(|c| c.k_tick() == k)
                .map(|c| (c.arm_qpc as i64 - c.reset_qpc as i64).rem_euclid(99_999))
                .collect();
            if v.is_empty() {
                continue;
            }
            v.sort_unstable();
            println!(
                "  K={}: sub-tick phase of (arm-reset) min={} max={} n={}",
                k,
                v[0],
                v[v.len() - 1],
                v.len()
            );
        }
    }

    // Does the sub-tick phase at the RESET decide 310 vs 311? The gate is at a
    // fixed real time, so the tick it lands on is decided by where the tick
    // accumulator's fraction sat when the countdown began.
    println!(
        "
-- sub-tick phase at the reset, grouped by the countdown length --"
    );
    {
        let tick = 99_999.007f64;
        let mut any = false;
        let mut ks: Vec<i64> = cycles.iter().map(|c| c.k_reset()).collect();
        ks.sort_unstable();
        ks.dedup();
        for k in ks {
            let mut v: Vec<f64> = cycles
                .iter()
                .filter(|c| c.k_reset() == k && c.reset_qpc != 0)
                .map(|c| (c.reset_qpc as f64 % tick) / tick)
                .collect();
            if v.is_empty() {
                continue;
            }
            any = true;
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!(
                "  K_reset={:>4}: phase min={:.4} max={:.4} n={}",
                k,
                v[0],
                v[v.len() - 1],
                v.len()
            );
        }
        if !any {
            println!("  no reset QPC captured");
        } else {
            println!("  (non-overlapping ranges => the phase decides the gate)");
        }
    }

    let kr_med = kr[kr.len() / 2];
    let mut err_reset: Vec<i64> = cycles
        .iter()
        .map(|c| c.first_moving as i64 - (kr_med - c.arm_offset_reset()))
        .collect();
    err_reset.sort_unstable();
    println!(
        "\n-- RESET model error: fm - (K_reset - (arm_tick - reset_tick)), K={} --",
        kr_med
    );
    print_histogram(&err_reset);

    let worst_reset = err_reset.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
    let exact_reset = err_reset.iter().filter(|e| **e == 0).count();
    let worst_tick = err_tick.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
    let worst_clk = err_clk.iter().map(|e| e.abs()).max().unwrap_or(i64::MAX);
    let exact_tick = err_tick.iter().filter(|e| **e == 0).count();
    let exact_clk = err_clk.iter().filter(|e| **e == 0).count();

    println!("\n=== VERDICT ===");
    println!(
        "  tick model : worst |error| {}, exact on {}/{}",
        worst_tick,
        exact_tick,
        cycles.len()
    );
    println!(
        "  clock model: worst |error| {}, exact on {}/{}",
        worst_clk,
        exact_clk,
        cycles.len()
    );
    println!(
        "  reset model: worst |error| {}, exact on {}/{}",
        worst_reset,
        exact_reset,
        cycles.len()
    );
    if worst_reset == 0 && stale == 0 {
        println!("\n  The LEVEL RESET predicts first_moving EXACTLY, and it is already");
        println!("  known when the cycle arms. The +/-1 was the gap between our F5 hold");
        println!("  finishing and the game actually resetting the level.");
        return true;
    }
    if worst_reset == 0 {
        println!(
            "\n  The level reset predicts first_moving exactly, but on {} cycles it",
            stale
        );
        println!("  was not yet final at the arm — so the arm has to wait for it.");
        return false;
    }
    if worst_clk == 0 {
        println!("\n  The game's own clock predicts first_moving EXACTLY.");
        println!("  The +/-1 was our tick counter's phase, not the countdown's.");
        true
    } else {
        println!("\n  Neither model is exact yet.");
        if kc.first() == kc.last() {
            println!(
                "  But the countdown IS a fixed {} game-centiseconds — the residual is",
                kc_med
            );
            println!("  in converting that to a replay index, i.e. the clock-vs-tick drift above.");
        } else {
            println!("  The countdown is not even a fixed number of the game's own units,");
            println!("  so the gate is not a pure function of the clock at the restart.");
        }
        false
    }
}

fn print_histogram(sorted: &[i64]) {
    let mut i = 0;
    while i < sorted.len() {
        let v = sorted[i];
        let n = sorted[i..].iter().take_while(|x| **x == v).count();
        println!("  {:>6}: {} {}", v, "#".repeat(n.min(60)), n);
        i += n;
    }
}
