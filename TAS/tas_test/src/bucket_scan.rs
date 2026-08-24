//! bucket-scan: brute-force the game's memory for whatever decides the F5 bucket.
//!
//! Every hand-picked candidate has now been measured and refuted (see
//! `bucket_predict`): the spawn state is CONSTANT across buckets, the arm phase
//! does not determine the bucket even measured to exact tick granularity, and the
//! engine clock sampled at the arm is tick-quantised by construction because
//! cave5 publishes once per frame and frames are exactly one tick apart.
//!
//! So stop guessing which value it is. If the game decides the bucket at the F5
//! instant — a countdown start, a deadline, a seed, a phase — that decision is
//! SOMEWHERE IN ITS MEMORY during the countdown, and whatever it is must
//! partition restarts exactly the way first_moving does.
//!
//! Method, per iteration:
//!   restart -> wait into the middle of the countdown -> snapshot the module
//!   images -> ARM_REC -> record past the countdown -> first_moving
//!
//! Then test EVERY 4-byte aligned slot as a predictor, with the same two-sided
//! criterion used elsewhere:
//!   SOUND    - no single value of the slot spans two different first_moving
//!   COMPLETE - no single first_moving is split across two values of the slot
//!
//! A slot that is constant across all samples is useless (no information); one
//! that is different in every sample is equally useless (it is a clock or a
//! counter, not a bucket id). What we are looking for is a slot with exactly as
//! many distinct values as there are buckets, lining up one-to-one.
//!
//! Scanning module images rather than the whole heap is a deliberate bet, and a
//! reasonable one: the level path (SG+0x1D3304) and the race clock (SG+0x1D5334)
//! both live in module data, so this engine keeps its state there. If nothing
//! turns up, the heap is the next place to look and the same machinery applies.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::thread;
use std::time::Duration;

use crate::harness;
use crate::level_hunt;

/// Where in the countdown to snapshot. Far enough in that the restart has fully
/// settled, far enough from the end that we are not racing the gate.
const SNAPSHOT_DELAY_MS: u64 = 1200;
/// Record long enough to see the boarder leave spawn.
const RECORD_SECS: u64 = 5;
const MIN_TICKS: usize = 320;

struct Sample {
    iter: u32,
    first_moving: u32,
    mem: HashMap<(String, usize), u32>,
}

pub fn run(iterations: u32) -> bool {
    println!("=== BUCKET-SCAN: brute-force the memory for the F5 bucket decider ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let pid = match level_hunt::find_pid() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: could not find the game process");
            return false;
        }
    };

    let mut samples: Vec<Sample> = Vec::new();

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed — skipping", iter);
            continue;
        }
        // Snapshot mid-countdown: the decision is already made by now, and the
        // boarder has not moved, so nothing downstream has perturbed it.
        thread::sleep(Duration::from_millis(SNAPSHOT_DELAY_MS));
        let mem = level_hunt::capture_all(pid);
        if mem.is_empty() {
            eprintln!("  iter {}: memory capture failed — skipping", iter);
            continue;
        }

        harness::focus_game();
        harness::arm_rec(&mut client);
        thread::sleep(Duration::from_secs(RECORD_SECS));
        let count = client.state().recorded_count as usize;
        harness::stop(&mut client);

        let n = count.min(client.state().rec_coords.len());
        let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
        let fm = match tas_shared::cont::detect_first_moving(&coords, n as u32) {
            Some(f) if n >= MIN_TICKS => f,
            _ => {
                eprintln!(
                    "  iter {}: no usable first_moving (ticks={}) — skipping",
                    iter, n
                );
                continue;
            }
        };

        println!(
            "  iter {:>2}: fm={}  slots captured={}",
            iter,
            fm,
            mem.len()
        );
        samples.push(Sample {
            iter,
            first_moving: fm,
            mem,
        });
    }

    analyze(&samples)
}

fn analyze(samples: &[Sample]) -> bool {
    println!("\n========== ANALYSIS ==========");
    if samples.len() < 6 {
        println!(
            "  Only {} usable samples — too few to trust any hit.",
            samples.len()
        );
        println!("  A slot can match by luck across a handful of restarts; the whole point");
        println!("  of a brute-force scan is that it will find spurious matches unless the");
        println!("  sample count makes them implausible. Re-run with more iterations.");
        return false;
    }

    let mut buckets: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for s in samples {
        buckets.entry(s.first_moving).or_default().push(s.iter);
    }
    println!("  {} samples", samples.len());
    println!("\n--- buckets observed ---");
    for (fm, iters) in &buckets {
        println!("  fm={:<5} x{:<3} {:?}", fm, iters.len(), iters);
    }
    if buckets.len() < 2 {
        println!("\n  Only one bucket occurred — nothing to discriminate. Re-run.");
        return false;
    }

    // Only slots present in EVERY sample can be judged; a slot that vanished in
    // one capture would otherwise look artificially consistent.
    let mut keys: BTreeSet<(String, usize)> = samples[0].mem.keys().cloned().collect();
    for s in &samples[1..] {
        keys.retain(|k| s.mem.contains_key(k));
    }
    println!("\n  {} slots present in every sample", keys.len());

    // A slot whose value is DIFFERENT IN EVERY SAMPLE is not automatically
    // noise, and treating it as such is how the first pass missed the point: a
    // countdown deadline or start stamped in QPC units is unique per restart by
    // construction. What would partition like the bucket is not the raw value
    // but its SUB-TICK REMAINDER — where it falls inside a 99,999-unit tick.
    //
    // So every varying slot is tested both raw and through a set of derived
    // keys, and the all-distinct ones are reported rather than silently dropped.
    const TICK_UNITS: u32 = 99_999;

    struct Derived {
        name: &'static str,
        f: fn(u32) -> u32,
    }
    let derived: Vec<Derived> = vec![
        Derived {
            name: "raw",
            f: |v| v,
        },
        Derived {
            name: "subtick/2",
            f: |v| (v % TICK_UNITS) * 2 / TICK_UNITS,
        },
        Derived {
            name: "subtick/3",
            f: |v| (v % TICK_UNITS) * 3 / TICK_UNITS,
        },
        Derived {
            name: "subtick/4",
            f: |v| (v % TICK_UNITS) * 4 / TICK_UNITS,
        },
        Derived {
            name: "subtick/6",
            f: |v| (v % TICK_UNITS) * 6 / TICK_UNITS,
        },
        Derived {
            name: "subtick/8",
            f: |v| (v % TICK_UNITS) * 8 / TICK_UNITS,
        },
        Derived {
            name: "subtick/16",
            f: |v| (v % TICK_UNITS) * 16 / TICK_UNITS,
        },
        // centisecond-domain variants, in case the value is not QPC at all
        Derived {
            name: "mod100/4",
            f: |v| (v % 100) * 4 / 100,
        },
        Derived {
            name: "mod1000/4",
            f: |v| (v % 1000) * 4 / 1000,
        },
    ];

    let mut hits: Vec<(String, usize, &'static str, usize)> = Vec::new();
    let mut constant = 0usize;
    let mut varying = 0usize;
    let mut all_distinct: Vec<(String, usize)> = Vec::new();

    for k in &keys {
        let vals: Vec<u32> = samples.iter().map(|s| s.mem[k]).collect();
        let distinct: BTreeSet<u32> = vals.iter().cloned().collect();
        if distinct.len() < 2 {
            constant += 1;
            continue;
        }
        varying += 1;
        if distinct.len() == samples.len() {
            all_distinct.push(k.clone());
        }

        for d in &derived {
            let keyed: Vec<u32> = vals.iter().map(|v| (d.f)(*v)).collect();
            let kd: BTreeSet<u32> = keyed.iter().cloned().collect();
            // A derived key with one value says nothing; one value per sample is
            // still just the raw counter wearing a hat.
            if kd.len() < 2 || kd.len() == samples.len() {
                continue;
            }
            let mut val_to_fms: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
            let mut fm_to_vals: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
            for (s, v) in samples.iter().zip(keyed.iter()) {
                val_to_fms.entry(*v).or_default().insert(s.first_moving);
                fm_to_vals.entry(s.first_moving).or_default().insert(*v);
            }
            let sound = val_to_fms.values().all(|f| f.len() == 1);
            let complete = fm_to_vals.values().all(|v| v.len() == 1);
            if sound && complete {
                hits.push((k.0.clone(), k.1, d.name, kd.len()));
            }
        }
    }

    println!(
        "  {} constant, {} varying ({} of them different in every sample)",
        constant,
        varying,
        all_distinct.len()
    );

    // Report the all-distinct slots explicitly with their values. These are the
    // clocks and counters, and one of them may be the countdown stamp.
    if !all_distinct.is_empty() {
        println!("\n--- slots that differ in EVERY sample (clock-like) ---");
        for k in all_distinct.iter().take(12) {
            let vals: Vec<String> = samples.iter().map(|s| format!("{}", s.mem[k])).collect();
            println!("  {}+{:#x}", k.0, k.1);
            println!("      values : {}", vals.join(" "));
            let rem: Vec<String> = samples
                .iter()
                .map(|s| format!("{}", s.mem[k] % TICK_UNITS))
                .collect();
            println!("      %tick  : {}", rem.join(" "));
            let fms: Vec<String> = samples
                .iter()
                .map(|s| format!("{}", s.first_moving))
                .collect();
            println!("      fm     : {}", fms.join(" "));
        }
    }

    // Also dump every varying slot that is NOT all-distinct — there were only a
    // handful, and seeing them beats guessing why none matched.
    println!("\n--- varying slots (not all-distinct), value -> fm ---");
    let mut shown = 0;
    for k in &keys {
        let vals: Vec<u32> = samples.iter().map(|s| s.mem[k]).collect();
        let distinct: BTreeSet<u32> = vals.iter().cloned().collect();
        if distinct.len() < 2 || distinct.len() == samples.len() {
            continue;
        }
        if shown >= 30 {
            break;
        }
        shown += 1;
        let mut m: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
        for (s, v) in samples.iter().zip(vals.iter()) {
            m.entry(*v).or_default().insert(s.first_moving);
        }
        let pairs: Vec<String> = m.iter().map(|(v, f)| format!("{}->{:?}", v, f)).collect();
        println!("  {}+{:#x}  {}", k.0, k.1, pairs.join(" "));
    }
    println!("\n========== RESULT ==========");
    if hits.is_empty() {
        println!("  NO SLOT in the module images predicts the bucket.");
        println!();
        println!("  That is a real result, not a failed run: if the decision were sitting in");
        println!("  a static variable during the countdown, this would have found it. Next");
        println!("  place to look is the heap, which needs region enumeration rather than");
        println!("  module images — same criterion, bigger haystack.");
        return false;
    }

    println!(
        "  {} (slot, derivation) pair(s) partition the restarts EXACTLY like first_moving:\n",
        hits.len()
    );
    hits.sort();
    for (module, off, deriv, n) in hits.iter().take(40) {
        println!("  {}+{:#x}  via {}  ({} distinct)", module, off, deriv, n);
    }
    if hits.len() > 40 {
        println!("  ... and {} more", hits.len() - 40);
    }
    println!();
    println!("  CAUTION: this scanned ~660k slots x 9 derivations, so some hits are");
    println!("  coincidence. Keep only what survives an independent set of restarts.");
    true
}

/// countdown-find: look for the countdown itself, INSIDE one countdown.
///
/// `bucket-scan` compares snapshots ACROSS restarts, which finds values that
/// differ per restart. It cannot see a value that counts down within a single
/// countdown and resets each time — and that is exactly what a 3..2..1..GO
/// timer is. If the game holds "time remaining", reading it at ARM gives
/// first_moving outright, with no replay and no prediction.
///
/// Method: snapshot early and late in ONE countdown and diff every slot. A
/// countdown in centiseconds falls by ~(dt_ms/10); in seconds-as-float by
/// ~dt_ms/1000. Report both directions — "elapsed" counting up is just as
/// usable as "remaining" counting down, as long as the scale matches. Slots
/// that move by an unrelated amount are the engine getting on with its work.
pub fn find_countdown(iterations: u32) -> bool {
    println!("=== COUNTDOWN-FIND: a value that moves WITHIN one countdown ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let pid = match level_hunt::find_pid() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: could not find the game process");
            return false;
        }
    };

    const EARLY_MS: u64 = 500;
    const LATE_MS: u64 = 2000;
    let dt_ms = LATE_MS - EARLY_MS;

    // Candidates must survive EVERY restart, or they are engine churn that
    // happened to move the right way once.
    let mut survivors: Option<BTreeSet<(String, usize)>> = None;

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed", iter);
            continue;
        }
        thread::sleep(Duration::from_millis(EARLY_MS));
        let a = level_hunt::capture_all(pid);
        thread::sleep(Duration::from_millis(dt_ms));
        let b = level_hunt::capture_all(pid);
        if a.is_empty() || b.is_empty() {
            eprintln!("  iter {}: capture failed", iter);
            continue;
        }

        // What a countdown would look like over dt_ms, in each plausible unit.
        let cs = (dt_ms as f64) / 10.0; // centiseconds
        let secs = (dt_ms as f64) / 1000.0; // seconds
        let mut found: BTreeSet<(String, usize)> = BTreeSet::new();

        for (k, va) in &a {
            let vb = match b.get(k) {
                Some(v) => *v,
                None => continue,
            };
            if vb == *va {
                continue;
            }
            // integer view, both directions, 15% tolerance
            let di = vb as f64 - *va as f64;
            let int_hit = (di.abs() - cs).abs() < cs * 0.15;
            // float view
            let fa = f32::from_bits(*va) as f64;
            let fb = f32::from_bits(vb) as f64;
            let df = fb - fa;
            let float_hit = fa.is_finite()
                && fb.is_finite()
                && fa.abs() < 1e6
                && fb.abs() < 1e6
                && (df.abs() - secs).abs() < secs * 0.15;
            if int_hit || float_hit {
                found.insert(k.clone());
            }
        }
        println!(
            "  iter {:>2}: {} slot(s) moved like a countdown",
            iter,
            found.len()
        );
        survivors = Some(match survivors {
            None => found,
            Some(prev) => prev.intersection(&found).cloned().collect(),
        });
        if let Some(sv) = &survivors {
            if sv.is_empty() {
                println!("      (no candidate has survived every restart so far)");
            }
        }
    }

    println!("\n========== RESULT ==========");
    let sv = survivors.unwrap_or_default();
    if sv.is_empty() {
        println!("  NOTHING in the module images moves at countdown rate on every restart.");
        println!("  Either the countdown lives on the heap, or the engine never stores");
        println!("  \"time remaining\" at all and simply compares a stored start against");
        println!("  the running clock each frame — in which case there is no value to read");
        println!("  and the phase is only knowable from the start stamp itself.");
        return false;
    }
    println!(
        "  {} slot(s) move at countdown rate on EVERY restart:",
        sv.len()
    );
    for k in sv.iter().take(30) {
        println!("    {}+{:#x}", k.0, k.1);
    }
    println!();
    println!("  Next: read one at ARM and check it equals the ticks left before the");
    println!("  boarder moves. That would be first_moving, known instantly.");
    true
}

/// countdown-find-heap: hunt the heap for a value that tracks the countdown.
///
/// A first pass with two snapshots and a +/-15% rate filter returned 14k-16k
/// "hits" per restart across 96MB of heap — which is noise, not signal. Over
/// that many slots, plenty of unrelated values happen to move by roughly the
/// right amount once.
///
/// The filter that actually discriminates is LINEARITY. A countdown or an
/// elapsed timer advances by the same amount per unit time; engine churn does
/// not. So sample four points across the countdown and demand:
///   * strictly monotonic (all three deltas the same sign),
///   * uniform (each delta within 20% of the mean delta),
///   * the right SCALE (mean delta matches centiseconds or seconds per step),
///   * and it must hold on EVERY restart, at the same offset within its region.
///
/// Anything surviving all four is a clock. Whether it is THE countdown is the
/// next question, but at least it is a timer and not a coincidence.
pub fn find_countdown_heap(iterations: u32) -> bool {
    println!("=== COUNTDOWN-FIND (HEAP, linearity-filtered) ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let pid = match level_hunt::find_pid() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: could not find the game process");
            return false;
        }
    };

    // Four samples inside the ~3s countdown, evenly spaced.
    const STEP_MS: u64 = 500;
    const FIRST_MS: u64 = 400;
    const STEPS: usize = 4;
    let cs_per_step = (STEP_MS as f64) / 10.0;
    let secs_per_step = (STEP_MS as f64) / 1000.0;

    // (region_base, offset) -> survives so far
    let mut survivors: Option<BTreeSet<(usize, usize)>> = None;

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed", iter);
            continue;
        }
        thread::sleep(Duration::from_millis(FIRST_MS));
        let mut snaps: Vec<Vec<level_hunt::RegionSnap>> = Vec::new();
        snaps.push(level_hunt::capture_heap(pid));
        for _ in 1..STEPS {
            thread::sleep(Duration::from_millis(STEP_MS));
            snaps.push(level_hunt::capture_heap(pid));
        }

        let mut found: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut shown = 0usize;
        for ra in &snaps[0] {
            // Every later snapshot must have the same region, same size, or the
            // slot cannot be followed across the whole window.
            let mut series: Vec<&level_hunt::RegionSnap> = vec![ra];
            let mut ok = true;
            for later in &snaps[1..] {
                match later
                    .iter()
                    .find(|r| r.base == ra.base && r.data.len() == ra.data.len())
                {
                    Some(r) => series.push(r),
                    None => {
                        ok = false;
                        break;
                    }
                }
            }
            if !ok {
                continue;
            }
            let n = ra.data.len() / 4 * 4;
            let mut off = 0usize;
            while off + 4 <= n {
                let vals: Vec<u32> = series
                    .iter()
                    .map(|r| {
                        u32::from_le_bytes([
                            r.data[off],
                            r.data[off + 1],
                            r.data[off + 2],
                            r.data[off + 3],
                        ])
                    })
                    .collect();

                // integer view
                let di: Vec<f64> = vals.windows(2).map(|w| w[1] as f64 - w[0] as f64).collect();
                let int_ok = linear_at(&di, cs_per_step);
                // float view
                let fv: Vec<f64> = vals.iter().map(|v| f32::from_bits(*v) as f64).collect();
                let float_ok = fv.iter().all(|x| x.is_finite() && x.abs() < 1e6) && {
                    let df: Vec<f64> = fv.windows(2).map(|w| w[1] - w[0]).collect();
                    linear_at(&df, secs_per_step)
                };

                if int_ok || float_ok {
                    found.insert((ra.base, off));
                    if shown < 10 && survivors.is_none() {
                        println!(
                            "      {:#x}: {:?}  (f32 {:?})",
                            ra.base + off,
                            vals,
                            fv.iter()
                                .map(|x| (x * 1000.0).round() / 1000.0)
                                .collect::<Vec<f64>>()
                        );
                        shown += 1;
                    }
                }
                off += 4;
            }
        }
        println!(
            "  iter {:>2}: {} linear timer-like slot(s)",
            iter,
            found.len()
        );
        survivors = Some(match survivors {
            None => found,
            Some(prev) => prev.intersection(&found).cloned().collect(),
        });
        if let Some(sv) = &survivors {
            println!("      surviving across restarts: {}", sv.len());
        }
    }

    println!("\n========== RESULT ==========");
    let sv = survivors.unwrap_or_default();
    if sv.is_empty() {
        println!("  NO heap slot advances linearly with the countdown on every restart.");
        println!();
        println!("  Combined with the module-image scan, the engine keeps no countdown");
        println!("  value anywhere that survives a restart at a stable address. There is");
        println!("  nothing for a fast judge to read.");
        return false;
    }
    println!("  {} slot(s) advance linearly on EVERY restart:", sv.len());
    for (base, off) in sv.iter().take(20) {
        println!("    {:#x} (region {:#x} + {:#x})", base + off, base, off);
    }
    println!();
    println!("  Heap addresses are not stable across runs, so these are a shape, not");
    println!("  an anchor. Next: check whether one, read at ARM, equals the ticks left");
    println!("  before the boarder moves.");
    true
}

/// True if `deltas` are all the same sign, uniform to 20%, and of magnitude
/// `expect` to 20%. Two of those alone are not enough: a value that jumps once
/// and sits still is not a timer, and neither is a fast counter that happens to
/// average out.
fn linear_at(deltas: &[f64], expect: f64) -> bool {
    if deltas.is_empty() || expect <= 0.0 {
        return false;
    }
    let first = deltas[0];
    if first == 0.0 {
        return false;
    }
    if !deltas
        .iter()
        .all(|d| (*d > 0.0) == (first > 0.0) && *d != 0.0)
    {
        return false;
    }
    let mean = deltas.iter().map(|d| d.abs()).sum::<f64>() / deltas.len() as f64;
    if (mean - expect).abs() > expect * 0.20 {
        return false;
    }
    deltas.iter().all(|d| (d.abs() - mean).abs() <= mean * 0.20)
}

/// bucket-scan-heap: does ANY heap slot partition restarts the way the bucket does?
///
/// The module-image version answered "no" for static data. The heap is 96MB, so
/// holding a snapshot per restart is not an option; instead candidates are
/// pruned incrementally:
///
///   restart 1+2  -> keep only slots whose value CHANGED between them. Most of
///                   the heap is static across a restart, so this is the cut
///                   that makes the rest affordable.
///   restart 3..N -> append each value; drop any slot once it exceeds
///                   MAX_DISTINCT values. A clock takes a new value every
///                   restart and dies immediately; a bucket id cannot, because
///                   there are only a handful of buckets.
///
/// Then the surviving slots get the same two-sided test as everywhere else:
/// sound (one value never spans two buckets) and complete (one bucket never
/// spans two values).
pub fn bucket_scan_heap(iterations: u32) -> bool {
    println!("=== BUCKET-SCAN (HEAP) ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    let pid = match level_hunt::find_pid() {
        Some(p) => p,
        None => {
            eprintln!("ERROR: could not find the game process");
            return false;
        }
    };

    // A bucket id cannot take more values than there are buckets; observed runs
    // show 3-4. Allowing a little slack costs nothing and avoids discarding the
    // answer if a run happens to span more.
    const MAX_DISTINCT: usize = 6;
    const SNAPSHOT_DELAY_MS: u64 = 1200;
    const RECORD_SECS: u64 = 5;

    let mut cands: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut fms: Vec<u32> = Vec::new();
    let mut first: Option<Vec<level_hunt::RegionSnap>> = None;

    for iter in 1..=iterations {
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            eprintln!("  iter {}: restart failed", iter);
            continue;
        }
        thread::sleep(Duration::from_millis(SNAPSHOT_DELAY_MS));
        // tick_count at the snapshot, so the arm delay can be subtracted out.
        //
        // WHY THIS MATTERS: capture_heap reads 96MB and takes on the order of a
        // second, and that delay varies. first_moving is measured from ARM, so it
        // absorbs that jitter — correlating against it would be correlating
        // against this tool's own scheduling. What the game decides at the restart
        // is the total from restart to gate; fm + arm_phase recovers exactly that
        // and is independent of when we happened to arm.
        let tick_at_snap = client.tick_count_volatile();
        let snap = level_hunt::capture_heap(pid);

        harness::focus_game();
        harness::arm_rec(&mut client);
        let arm_phase = client.tick_count_volatile().wrapping_sub(tick_at_snap);
        thread::sleep(Duration::from_secs(RECORD_SECS));
        let count = client.state().recorded_count as usize;
        harness::stop(&mut client);
        let n = count.min(client.state().rec_coords.len());
        let coords: Vec<[f32; 3]> = client.state().rec_coords[..n].to_vec();
        let fm = match tas_shared::cont::detect_first_moving(&coords, n as u32) {
            Some(f) if n >= 320 => f + arm_phase,
            _ => {
                eprintln!("  iter {}: unusable recording — skipping", iter);
                continue;
            }
        };

        // Three distinct phases, driven by how many restarts have been recorded.
        // The earlier version keyed off an Option and re-entered the seed branch
        // every iteration, so values were never appended and every candidate was
        // discarded by the length check at the end — a silent no-op that looked
        // like a clean negative result.
        if fms.is_empty() {
            first = Some(snap);
            fms.push(fm);
            println!(
                "  iter {:>2}: total={} (holding restart 1 for the diff)",
                iter, fm
            );
        } else if cands.is_empty() {
            let prev = first.take().expect("restart 1 snapshot must be held");
            for ra in &prev {
                let rb = match snap
                    .iter()
                    .find(|r| r.base == ra.base && r.data.len() == ra.data.len())
                {
                    Some(r) => r,
                    None => continue,
                };
                let n4 = ra.data.len() / 4 * 4;
                let mut off = 0usize;
                while off + 4 <= n4 {
                    let va = u32::from_le_bytes([
                        ra.data[off],
                        ra.data[off + 1],
                        ra.data[off + 2],
                        ra.data[off + 3],
                    ]);
                    let vb = u32::from_le_bytes([
                        rb.data[off],
                        rb.data[off + 1],
                        rb.data[off + 2],
                        rb.data[off + 3],
                    ]);
                    if va != vb {
                        cands.insert((ra.base, off), vec![va, vb]);
                    }
                    off += 4;
                }
            }
            fms.push(fm);
            println!(
                "  iter {:>2}: total={}  {} varying slots seeded",
                iter,
                fm,
                cands.len()
            );
        } else {
            let mut drop: Vec<(usize, usize)> = Vec::new();
            for (k, hist) in cands.iter_mut() {
                let r = match snap
                    .iter()
                    .find(|r| r.base == k.0 && k.1 + 4 <= r.data.len())
                {
                    Some(r) => r,
                    None => {
                        drop.push(*k);
                        continue;
                    }
                };
                let v = u32::from_le_bytes([
                    r.data[k.1],
                    r.data[k.1 + 1],
                    r.data[k.1 + 2],
                    r.data[k.1 + 3],
                ]);
                hist.push(v);
                let d: BTreeSet<u32> = hist.iter().cloned().collect();
                if d.len() > MAX_DISTINCT {
                    drop.push(*k);
                }
            }
            for k in drop {
                cands.remove(&k);
            }
            fms.push(fm);
            println!(
                "  iter {:>2}: total={}  {} candidates left",
                iter,
                fm,
                cands.len()
            );
        }
    }

    println!("\n========== ANALYSIS ==========");
    println!("  {} restarts, {} candidate slots", fms.len(), cands.len());
    if fms.len() < 6 {
        println!("  Too few restarts to trust a hit.");
        return false;
    }
    let distinct_fm: BTreeSet<u32> = fms.iter().cloned().collect();
    println!("  restart->gate totals observed: {:?}", distinct_fm);
    if distinct_fm.len() < 2 {
        println!("  Only one bucket occurred — nothing to discriminate.");
        return false;
    }

    let mut hits: Vec<((usize, usize), usize)> = Vec::new();
    for (k, hist) in &cands {
        if hist.len() != fms.len() {
            continue; // slot vanished partway; cannot be judged
        }
        let mut val_to_fms: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
        let mut fm_to_vals: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
        for (v, fm) in hist.iter().zip(fms.iter()) {
            val_to_fms.entry(*v).or_default().insert(*fm);
            fm_to_vals.entry(*fm).or_default().insert(*v);
        }
        let sound = val_to_fms.values().all(|f| f.len() == 1);
        let complete = fm_to_vals.values().all(|v| v.len() == 1);
        if sound && complete {
            hits.push((*k, val_to_fms.len()));
        }
    }

    println!("\n========== RESULT ==========");
    if hits.is_empty() {
        println!("  NO heap slot partitions the restarts like the restart->gate total.");
        println!();
        println!("  With the module images already ruled out, nothing the game stores");
        println!("  ANYWHERE identifies the bucket during the countdown. The bucket is not");
        println!("  a value the engine holds — it is an emergent consequence of when the");
        println!("  restart landed relative to the tick, which nothing records.");
        return false;
    }
    println!(
        "  {} heap slot(s) partition the restarts EXACTLY like first_moving:",
        hits.len()
    );
    for ((base, off), n) in hits.iter().take(20) {
        let hist = &cands[&(*base, *off)];
        println!(
            "    {:#x}  ({} distinct)  values {:?}  fms {:?}",
            base + off,
            n,
            hist,
            fms
        );
    }
    println!();
    println!("  Heap addresses shift between game launches, so a shipped judge would");
    println!("  need to re-find this rather than hardcode it. Verify on a fresh set of");
    println!("  restarts before believing it.");
    true
}
