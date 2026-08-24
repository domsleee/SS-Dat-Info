//! bucket-predict: can the F5 bucket be identified BEFORE the boarder moves?
//!
//! THE PROBLEM. The bucket judge is positional: it waits until the replay has
//! run `first_moving + BUCKET_MATCH_WINDOW` ticks (~314, ~3.1s) and then compares
//! which frame the boarder left spawn on. Most of that wait is structural, not
//! conservatism — the boarder is bit-identical for the whole countdown
//! (`rec-start` measures a stationary prefix of 250-262 frames), so position
//! carries ZERO bucket information until it moves. At ~5s per reroll and a mean
//! of 3.88 attempts, that is ~20s of lottery before a PLAY settles.
//!
//! THE QUESTION. Is some other observable already different at spawn? The bucket
//! is a sub-tick clock phase, and `f5_probe`'s docstring records the suspicion
//! that "the rotation/velocity state is also bucket-quantized" — bit-identical
//! spawn POSITIONS that still diverge later. If a field sampled seconds before
//! the boarder moves predicts which bucket you landed, the reroll decision
//! becomes ~50ms instead of ~3.1s.
//!
//! THE EXPERIMENT. Per iteration: restart, snapshot the early state, then RECORD
//! long enough to observe the bucket, and pair them.
//!
//!     restart -> snapshot(pos, rot[9], vel[3], race_time_cs, speed)
//!             -> ARM_REC -> record past the countdown -> first_moving
//!
//! Then check both directions, which is the part that matters:
//!   * do samples sharing an early fingerprint always share a first_moving?
//!     (no false confidence — the fingerprint never claims two buckets are one)
//!   * do samples sharing a first_moving always share the fingerprint?
//!     (no wasted rerolls — the fingerprint never splits one bucket into two)
//!
//! A field is only useful as a fast judge if BOTH hold. A field that is constant
//! across every sample is dead (no signal); a field that is unique per sample is
//! equally dead (it is noise, and would reroll forever).
//!
//! This is a measurement tool, not a pass/fail gate. It prints what each
//! candidate field can and cannot do, and says plainly if none of them work.

//! ==========================================================================
//! RESULTS (2026-08-22, FE Cloudy, 24 + 24 + 20 restarts)
//! ==========================================================================
//!
//! TWO HYPOTHESES TESTED, BOTH REFUTED. Recorded here so nobody re-runs them.
//!
//! 1. "The spawn state is bucket-quantized" (f5_probe's standing suspicion).
//!    NO. Across 24 restarts spanning 3 buckets (fm = 261 x3, 262 x9, 263 x12):
//!
//!    spawn position   1 distinct value  <- bit-identical every single time
//!    velocity         1 distinct value
//!    speed            1 distinct value
//!    race_time_cs     1 distinct value
//!    rotation matrix  4 distinct values <- UNSOUND: one key spans several
//!    buckets, so it would accept wrong ones
//!
//!    The spawn state is not merely a weak predictor, it is CONSTANT. There is
//!    nothing to read.
//!
//! 2. "The bucket is the ARM PHASE — the tick offset between restart and ARM."
//!    This looked extremely promising. transport::arm_settle_ms already encodes
//!    a linear model (base = (OBSERVED_AT_ZERO_SETTLE - fm) * MS_PER_FRAME), and
//!    the first pass fit it almost perfectly: fm + arm_phase = 304 in 23 of 24
//!    samples. The single outlier looked like polling jitter, because the arm
//!    tick was being sampled after arm_rec's 50ms sleep.
//!
//!    So the command was bracketed exactly — tick_count immediately before and
//!    after send_command. THE WINDOW CAME BACK 0 TICKS on all 20 samples, i.e.
//!    no measurement uncertainty at all. And the model still failed:
//!
//!    arm_phase=36 -> fm 263 (x11), and also 261, and also 262
//!    arm_phase=37 -> fm 261, 262 AND 263
//!    fm+arm_phase = 297, 298, 299, 300   (not constant)
//!
//!    Same arm phase, different bucket, with the phase measured exactly. The
//!    arm phase is strongly CORRELATED with the bucket but does not determine
//!    it.
//!
//! WHAT THAT LEAVES. The discriminating information is genuinely SUB-TICK, which
//! is what the field notes always said. Nothing observable at tick granularity —
//! position, velocity, rotation, the race timer, or the arm phase in ticks —
//! can capture it, and all of those are now measured rather than assumed.
//!
//! AND NOT prev_time EITHER, deductively: cave5 advances the accumulator by
//! `esi * tick_advance`, always a multiple of 0.01, so `prev_time mod 0.01` is
//! invariant by construction. Sampling the accumulator cannot reveal the phase.
//!
//! The only place the sub-tick phase can live is the `now` stamp the game reads
//! at the top of the cycle — Kernel::Time::Current via call [0x46d15c] at
//! EXE+0x25C4A, landing as a 64-bit value at [esp+0x84], which cave5's hook site
//! (EXE+0x25C81) is downstream of and could read. `(now - prev) * 100` BEFORE
//! __ftol truncates it is the fractional tick position; that number, captured at
//! ARM, is the last untested candidate for a fast judge.
//!
//! That needs a DLL change and a shared-state field (no spare capacity — it
//! means bumping TAS_SHARED_VERSION and the sizeof assert), on the
//! determinism-critical path. Not done here.
use std::collections::BTreeMap;
use std::thread;
use std::time::Duration;

use crate::harness;

/// How long to record before reading first_moving. The countdown is ~250-300
/// ticks (~3s); 4.5s clears it with margin on a slow tick without wasting time.
const RECORD_SECS: u64 = 5;
/// Minimum ticks a recording must contain for its first_moving to be trusted.
const MIN_TICKS: usize = 320;

#[derive(Clone)]
struct Sample {
    iter: u32,
    /// Bit patterns, not floats: a bucket difference can be one ULP, and `==` on
    /// f32 would quietly call those equal.
    pos: [u32; 3],
    rot: [u32; 9],
    vel: [u32; 3],
    race_time_cs: u32,
    speed: u32,
    /// THE ARM-PHASE CANDIDATE. first_moving is a linear function of the arm
    /// delay (see transport::arm_settle_ms: base = (OBSERVED_AT_ZERO_SETTLE -
    /// fm) * MS_PER_FRAME), so the "bucket" is really the tick offset between
    /// the restart completing and the ARM landing. If that is true, this delta
    /// is knowable AT ARM TIME — before a single frame of replay.
    /// Ticks elapsed while the ARM command was in flight — the measurement
    /// uncertainty on the arm phase.
    arm_window: u32,
    /// The engine's own 64-bit elapsed-time delta at the ARM frame, {lo,hi},
    /// read by cave5 from [esp+0x40] before __ftol truncates it to a tick
    /// count. The sub-tick phase - the last candidate standing after every
    /// tick-resolution signal was ruled out by measurement.
    /// Engine clock at the moment the restart finished, same units as clk_lo.
    /// (clk_lo - clk_at_restart) is the arm delay at FULL resolution, where the
    /// tick-granular version of the same quantity was already shown not to
    /// determine the bucket.
    clk_at_restart: u32,
    units_per_tick: u32,
    clk_lo: u32,
    clk_hi: u32,
    ticks_restart_to_arm: u32,
    frames_restart_to_arm: u32,
    tick_at_arm_mod2: u32,
    tick_at_arm_mod3: u32,
    /// The bucket, learned the slow way — what a fast field would have to predict.
    first_moving: Option<u32>,
    ticks: usize,
}

fn bits3(v: [f32; 3]) -> [u32; 3] {
    [v[0].to_bits(), v[1].to_bits(), v[2].to_bits()]
}

pub fn run(iterations: u32) -> bool {
    println!("=== BUCKET-PREDICT: is the F5 bucket knowable before the boarder moves? ===\n");
    println!(
        "  {} iterations x (restart + {}s record) — roughly {} min\n",
        iterations,
        RECORD_SECS,
        (iterations as u64 * (RECORD_SECS + 3) / 60).max(1)
    );

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    // Calibrate the clock rate before assuming anything about units. cave5
    // publishes a raw engine counter; whether it is QPC 10MHz, 1MHz, or
    // something else decides what "one tick" is in these units, and guessing
    // would silently invalidate every phase computed below.
    let cal_a = client.state().clock_delta_lo;
    thread::sleep(Duration::from_millis(1000));
    let cal_b = client.state().clock_delta_lo;
    let units_per_sec = cal_b.wrapping_sub(cal_a);
    let units_per_tick = units_per_sec / 100; // the game ticks at 100/s
    println!(
        "  clock calibration: {} units/sec  =>  {} units per 10ms tick",
        units_per_sec, units_per_tick
    );

    let mut samples: Vec<Sample> = Vec::new();

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed — skipping", iter);
            continue;
        }

        // Snapshot BEFORE arming: this is the information a fast judge would
        // have available, seconds before the boarder moves.
        let (pos, rot, vel, race_time_cs, speed) = {
            let s = client.state();
            (
                bits3([s.player_x, s.player_y, s.player_z]),
                {
                    let mut r = [0u32; 9];
                    for (i, v) in s.rotation_matrix.iter().enumerate() {
                        r[i] = v.to_bits();
                    }
                    r
                },
                bits3([s.velocity_x, s.velocity_y, s.velocity_z]),
                s.race_time_cs,
                s.speed.to_bits(),
            )
        };

        let clk_at_restart = client.state().clock_delta_lo;
        let tick_after_restart = client.tick_count_volatile();
        let frame_after_restart = client.frame_count_volatile();
        harness::focus_game();
        // Bracket the command as tightly as a poller can: the DLL consumes it
        // somewhere inside [before, after], so `after - before` IS the
        // uncertainty in the arm phase. If that window is 0-1 ticks, a +/-1
        // residual in the fit is NOT measurement noise and the model is wrong;
        // if it is several ticks, the residual is mine, not the game's.
        let tick_before_cmd = client.tick_count_volatile();
        client.send_command(tas_shared::TasCommand::ArmRec);
        let tick_after_cmd = client.tick_count_volatile();
        let clk_lo = client.state().clock_delta_lo;
        let clk_hi = client.state().clock_delta_hi;
        let arm_window = tick_after_cmd.wrapping_sub(tick_before_cmd);
        harness::arm_rec(&mut client);
        let tick_at_arm = tick_before_cmd;
        let frame_at_arm = client.frame_count_volatile();
        let ticks_restart_to_arm = tick_at_arm.wrapping_sub(tick_after_restart);
        let frames_restart_to_arm = frame_at_arm.wrapping_sub(frame_after_restart);
        thread::sleep(Duration::from_secs(RECORD_SECS));
        let count = client.state().recorded_count as usize;
        harness::stop(&mut client);

        let n = count.min(client.state().rec_coords.len());
        let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
        let fm = tas_shared::cont::detect_first_moving(&coords, n as u32);

        println!(
            "  iter {:>2}: fm={:?}  arm_phase=+{} ticks  fine={:>9} units  subtick={:>7}",
            iter,
            fm,
            ticks_restart_to_arm,
            clk_lo.wrapping_sub(clk_at_restart),
            if units_per_tick > 0 {
                clk_lo.wrapping_sub(clk_at_restart) % units_per_tick
            } else {
                0
            }
        );
        let _ = (n, arm_window);

        samples.push(Sample {
            iter,
            pos,
            rot,
            vel,
            race_time_cs,
            speed,
            clk_at_restart,
            units_per_tick,
            clk_lo,
            clk_hi,
            arm_window,
            ticks_restart_to_arm,
            frames_restart_to_arm,
            tick_at_arm_mod2: tick_at_arm % 2,
            tick_at_arm_mod3: tick_at_arm % 3,
            first_moving: fm,
            ticks: n,
        });
    }

    analyze(&samples)
}

/// One candidate early field, reduced to a comparable key.
struct Candidate {
    name: &'static str,
    key: fn(&Sample) -> String,
}

fn analyze(samples: &[Sample]) -> bool {
    println!("\n========== ANALYSIS ==========");

    let usable: Vec<&Sample> = samples
        .iter()
        .filter(|s| s.first_moving.is_some() && s.ticks >= MIN_TICKS)
        .collect();

    println!(
        "  {} samples, {} usable (first_moving detected and >= {} ticks)",
        samples.len(),
        usable.len(),
        MIN_TICKS
    );
    if usable.len() < 4 {
        println!("\n  NOT ENOUGH DATA to conclude anything. Re-run with more iterations,");
        println!("  or fix whatever made the recordings short.");
        return false;
    }

    // The thing to predict.
    let mut buckets: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for s in &usable {
        buckets
            .entry(s.first_moving.unwrap())
            .or_default()
            .push(s.iter);
    }
    println!("\n--- The buckets actually observed (first_moving) ---");
    for (fm, iters) in &buckets {
        println!("  fm={:<5} x{:<3} iters {:?}", fm, iters.len(), iters);
    }
    if buckets.len() < 2 {
        println!("\n  ONLY ONE BUCKET OCCURRED. Nothing to discriminate — this run cannot");
        println!("  tell a working predictor from a constant. Re-run; the lottery needs to");
        println!("  actually roll differently for this experiment to mean anything.");
        return false;
    }

    let candidates: Vec<Candidate> = vec![
        Candidate {
            name: "spawn position",
            key: |s| format!("{:?}", s.pos),
        },
        Candidate {
            name: "rotation matrix",
            key: |s| format!("{:?}", s.rot),
        },
        Candidate {
            name: "velocity",
            key: |s| format!("{:?}", s.vel),
        },
        Candidate {
            name: "speed",
            key: |s| format!("{}", s.speed),
        },
        Candidate {
            name: "race_time_cs",
            key: |s| format!("{}", s.race_time_cs),
        },
        // The fine arm delay, bucketed at a few resolutions. Raw units are
        // noise (unique per sample); the question is whether some coarser slice
        // of the SUB-TICK REMAINDER lines up with the bucket.
        Candidate {
            name: "subtick/8th",
            key: |s| {
                let d = s.clk_lo.wrapping_sub(s.clk_at_restart);
                let t = s.units_per_tick.max(1);
                format!("{}", (d % t) * 8 / t)
            },
        },
        Candidate {
            name: "subtick/4th",
            key: |s| {
                let d = s.clk_lo.wrapping_sub(s.clk_at_restart);
                let t = s.units_per_tick.max(1);
                format!("{}", (d % t) * 4 / t)
            },
        },
        Candidate {
            name: "subtick/half",
            key: |s| {
                let d = s.clk_lo.wrapping_sub(s.clk_at_restart);
                let t = s.units_per_tick.max(1);
                format!("{}", (d % t) * 2 / t)
            },
        },
        Candidate {
            name: "fine delay /tick",
            key: |s| {
                let d = s.clk_lo.wrapping_sub(s.clk_at_restart);
                let t = s.units_per_tick.max(1);
                format!("{}", d / t)
            },
        },
        Candidate {
            name: "fm+arm_phase (const?)",
            key: |s| format!("{}", s.first_moving.unwrap_or(0) + s.ticks_restart_to_arm),
        },
        Candidate {
            name: "arm phase (ticks)",
            key: |s| format!("{}", s.ticks_restart_to_arm),
        },
        Candidate {
            name: "arm phase (frames)",
            key: |s| format!("{}", s.frames_restart_to_arm),
        },
        Candidate {
            name: "tick_at_arm % 2",
            key: |s| format!("{}", s.tick_at_arm_mod2),
        },
        Candidate {
            name: "tick_at_arm % 3",
            key: |s| format!("{}", s.tick_at_arm_mod3),
        },
        Candidate {
            name: "pos+rot+vel",
            key: |s| format!("{:?}{:?}{:?}", s.pos, s.rot, s.vel),
        },
    ];

    println!("\n--- Can any early field predict the bucket? ---");
    let mut any_useful = false;
    for c in &candidates {
        let mut by_key: BTreeMap<String, Vec<u32>> = BTreeMap::new();
        for s in &usable {
            by_key
                .entry((c.key)(s))
                .or_default()
                .push(s.first_moving.unwrap());
        }
        let distinct_keys = by_key.len();

        // Sound: one fingerprint never covers two different buckets.
        let sound = by_key.values().all(|fms| {
            let first = fms[0];
            fms.iter().all(|f| *f == first)
        });
        // Complete: one bucket is never split across fingerprints.
        let mut fm_to_keys: BTreeMap<u32, std::collections::BTreeSet<String>> = BTreeMap::new();
        for (k, fms) in &by_key {
            for f in fms {
                fm_to_keys.entry(*f).or_default().insert(k.clone());
            }
        }
        let complete = fm_to_keys.values().all(|ks| ks.len() == 1);

        let verdict = if distinct_keys == 1 {
            "DEAD — constant across every sample, carries no information".to_string()
        } else if distinct_keys == usable.len() && buckets.len() < usable.len() {
            "DEAD — unique per sample, it is noise not a bucket id".to_string()
        } else if sound && complete {
            any_useful = true;
            "*** PREDICTS THE BUCKET — sound AND complete ***".to_string()
        } else if sound {
            "partial — never confuses two buckets, but splits one bucket across keys \
             (would reroll good buckets)"
                .to_string()
        } else {
            "UNSOUND — one key covers multiple buckets (would accept wrong ones)".to_string()
        };

        println!(
            "  {:<16} {:>2} distinct value(s) across {} samples — {}",
            c.name,
            distinct_keys,
            usable.len(),
            verdict
        );
    }

    println!("\n========== CONCLUSION ==========");
    if any_useful {
        println!("  At least one field sampled BEFORE the countdown predicts the bucket.");
        println!("  That is the fast judge: compare it at spawn and reroll in ~50ms instead");
        println!("  of replaying ~3.1s. Next: store it in the recording and judge on it.");
    } else {
        println!("  NONE of these fields predicts the bucket.");
        println!("  The early physics state does not carry the phase, so a fast judge needs");
        println!("  a different signal — the per-frame tick schedule (the clock phase itself)");
        println!("  rather than the state it eventually produces.");
    }
    any_useful
}
