//! The hunt for the second hidden spawn state.
//!
//! A control run settled that the gate index is not the whole bucket: an
//! aligned replay diverged at the FIRST MOVING COORDINATE with its gate index
//! matched, while the legacy path replayed the same recording bit-exact. So
//! something in the physics state differs between restarts that position does
//! not show — position at the arm is bit-identical across restarts (36/36).
//!
//! Rather than guess the field, capture the whole player object and its
//! physics sub-object at the arm and at the gate, run many aligned replays
//! with no judging and no retries, and let the trajectory outcome partition
//! the captures: a dword whose values split cleanly between the replays that
//! matched and the ones that did not IS the hidden state, or sits next to it.
//!
//! Two capture points because they answer different questions. A difference
//! visible at the ARM can be rejected — or corrected — before a single tick
//! replays. One visible only at the GATE still names the field, and says the
//! state evolves during the countdown.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::cont::BucketVerdict;
use tas_shared::{
    TasCommand, TasMode, TasSharedMemoryClient, OBJSNAP_PHYSICS_DWORDS, OBJSNAP_PLAYER_DWORDS,
};

use crate::harness;
use crate::patterns;
use crate::replay;

/// Fresh-recording mode mirrors the acceptance flow, which is where the
/// hidden state was actually seen: a REC with Pico steering made in this
/// session, then an aligned PLAY of it. A file-loaded recording never showed
/// it in 24/24 — the recording's own restart may simply have landed the
/// common state. Recording fresh each time samples BOTH sides' restarts, and
/// gives the recording's own arm/gate snapshots to diff against.
const FRESH_STEER_PATTERN: &str = "LRL";
const FRESH_HOLD_TICKS: u32 = 150;
const FRESH_TAIL_TICKS: u32 = 50;

const RECORDING_REL: &str = "TAS/recordings/FE-10065.tasrec";
const GATE_TIMEOUT_SECS: u64 = 40;
/// Gate-relative ticks to judge. The failing case diverged at k=0, so this is
/// plenty, and it keeps each attempt short.
const JUDGE_TICKS: u32 = 64;

struct Attempt {
    live_gate: u32,
    rec_gate: u32,
    /// Physics-TICK count from the F5 press to the gate. first_moving is a
    /// CYCLE count; this is the tick count, and the two need not agree — the
    /// countdown was measured at 310 OR 311 ticks for the same cycle gate. If
    /// mismatched replays differ here while matched ones do not, the residual
    /// IS the sub-tick: one extra settle tick.
    gate_ticks: i64,
    gate_cycles: i64,
    /// The RECORDING's own snapshots, when recorded fresh this iteration.
    rec_arm_player: Vec<u32>,
    rec_arm_physics: Vec<u32>,
    rec_gate_player: Vec<u32>,
    rec_gate_physics: Vec<u32>,
    matched: bool,
    first_bad: Option<u32>,
    arm_player: Vec<u32>,
    arm_physics: Vec<u32>,
    gate_player: Vec<u32>,
    gate_physics: Vec<u32>,
    player_ok: u32,
    physics_ok: u32,
}

pub fn run(iterations: u32, rec: Option<&str>, fresh: bool) -> bool {
    if std::env::var("HS_VAR").is_ok() {
        return variance(iterations);
    }
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
    println!("=== Hidden spawn state hunt: {} ===", path);

    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    if !fresh {
        let loaded = match replay::load_tasrec(std::path::Path::new(&path)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("ERROR: {}", e);
                return false;
            }
        };
        replay::write_to_shared(&mut client, &loaded);
    }
    harness::focus_game();

    let mut attempts: Vec<Attempt> = Vec::new();
    for i in 1..=iterations {
        let mut rec_snaps: Option<[Vec<u32>; 4]> = None;
        if fresh {
            // REC: restart, arm, steer via Pico, stop. Mirrors acceptance.
            client.state_mut().playback_speed = 1.0;
            client.state_mut().gate_index = 0;
            if !harness::restart_and_stabilize_inprocess(&mut client) {
                println!("  iter {:>2}: REC restart failed", i);
                continue;
            }
            harness::arm_rec(&mut client);
            let steps = patterns::build_from_pattern(FRESH_STEER_PATTERN, FRESH_HOLD_TICKS, 0);
            harness::drive_pico_steps_keepalive(&steps, Some(FRESH_HOLD_TICKS as u64 * 10), 200);
            thread::sleep(Duration::from_millis(FRESH_TAIL_TICKS as u64 * 10));
            harness::stop(&mut client);
            let s = client.state();
            if s.recorded_count < 200 || s.gate_index == 0 {
                println!(
                    "  iter {:>2}: REC too short or never moved ({} ticks)",
                    i, s.recorded_count
                );
                continue;
            }
            rec_snaps = Some([
                s.objsnap_arm_player[..].to_vec(),
                s.objsnap_arm_physics[..].to_vec(),
                s.objsnap_gate_player[..].to_vec(),
                s.objsnap_gate_physics[..].to_vec(),
            ]);
        }
        let rec_gate = {
            let s = client.state();
            match tas_shared::cont::detect_first_moving(&s.rec_coords[..], s.recorded_count) {
                Some(f) => f,
                None => {
                    println!("  iter {:>2}: recording never moves", i);
                    continue;
                }
            }
        };
        match one_attempt(&mut client, rec_gate) {
            Some(mut a) => {
                if let Some([ap, ah, gp, gh]) = rec_snaps {
                    a.rec_arm_player = ap;
                    a.rec_arm_physics = ah;
                    a.rec_gate_player = gp;
                    a.rec_gate_physics = gh;
                }
                let rot = rotation_of(&a.gate_physics);
                println!(
                    "  attempt {:>2}: gate cyc={} (off {:+}) TICKS={} {} first_bad={:?} | rot [{:.5} {:.5} {:.5}]",
                    i,
                    a.gate_cycles,
                    a.live_gate as i64 - a.rec_gate as i64,
                    a.gate_ticks,
                    if a.matched { "MATCH   " } else { "MISMATCH" },
                    a.first_bad,
                    rot[0],
                    rot[1],
                    rot[2]
                );
                attempts.push(a);
            }
            None => println!("  attempt {:>2}: failed", i),
        }
    }
    harness::stop(&mut client);
    analyse(&attempts)
}

/// Raw-restart variance: what does the ARM snapshot look like across restarts
/// with nothing else going on? Whatever varies is the hidden spawn state.
fn variance(iterations: u32) -> bool {
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));
    harness::focus_game();

    let mut players: Vec<Vec<u32>> = Vec::new();
    let mut physics: Vec<Vec<u32>> = Vec::new();
    for i in 1..=iterations {
        client.state_mut().playback_speed = 1.0;
        if !harness::restart_and_stabilize_inprocess(&mut client) {
            println!("  iter {:>2}: restart failed", i);
            continue;
        }
        harness::arm_rec(&mut client);
        thread::sleep(Duration::from_millis(30));
        let s = client.state();
        let sp = [
            f32::from_bits(s.objsnap_arm_player[GameOff::X]),
            f32::from_bits(s.objsnap_arm_player[GameOff::Y]),
            f32::from_bits(s.objsnap_arm_player[GameOff::Z]),
        ];
        println!(
            "  iter {:>2}: spawn ({:.6}, {:.6}, {:.6}) player_ok={} physics_ok={}",
            i, sp[0], sp[1], sp[2], s.objsnap_player_ok, s.objsnap_physics_ok
        );
        players.push(s.objsnap_arm_player[..].to_vec());
        physics.push(s.objsnap_arm_physics[..].to_vec());
        harness::stop(&mut client);
        thread::sleep(Duration::from_millis(120));
    }
    harness::stop(&mut client);

    report_variance("player ", &players);
    report_variance("physics", &physics);
    true
}

/// Player object dword offsets of the known position floats (bytes/4).
struct GameOff;
impl GameOff {
    const X: usize = 0xF8 / 4;
    const Y: usize = 0xFC / 4;
    const Z: usize = 0x100 / 4;
}

/// Every dword that is not identical across all captures, as hex + float.
fn report_variance(label: &str, caps: &[Vec<u32>]) {
    if caps.len() < 2 {
        println!("  {}: need >=2 captures", label);
        return;
    }
    let n = caps.iter().map(|c| c.len()).min().unwrap_or(0);
    let mut varying = 0usize;
    let mut lines: Vec<String> = Vec::new();
    for d in 0..n {
        let mut vals: Vec<u32> = caps.iter().map(|c| c[d]).collect();
        vals.sort_unstable();
        vals.dedup();
        if vals.len() > 1 {
            varying += 1;
            let hex: Vec<String> = vals.iter().take(6).map(|v| format!("{:08x}", v)).collect();
            let flt: Vec<String> = vals
                .iter()
                .take(6)
                .map(|v| format!("{:.5}", f32::from_bits(*v)))
                .collect();
            lines.push(format!(
                "    +0x{:03x}: {} distinct {:?} ({:?})",
                d * 4,
                vals.len(),
                hex,
                flt
            ));
        }
    }
    println!(
        "  {}: {} of {} dwords vary across {} restarts",
        label,
        varying,
        n,
        caps.len()
    );
    for l in lines.iter().take(60) {
        println!("{}", l);
    }
}

fn rotation_of(physics: &[u32]) -> [f32; 3] {
    let base = 0x1B4 / 4;
    if physics.len() < base + 9 {
        return [f32::NAN; 3];
    }
    [
        f32::from_bits(physics[base]),
        f32::from_bits(physics[base + 1]),
        f32::from_bits(physics[base + 2]),
    ]
}

fn one_attempt(client: &mut TasSharedMemoryClient, rec_gate: u32) -> Option<Attempt> {
    client.state_mut().playback_speed = 1.0;
    client.state_mut().gate_index = 0;
    client.state_mut().gate_tick = 0;
    if !harness::restart_and_stabilize_inprocess(client) {
        return None;
    }
    // After the restart — STOP clears alignment on purpose.
    client.state_mut().gate_align_rec = rec_gate;
    harness::arm_play(client);

    let deadline = Instant::now() + Duration::from_secs(GATE_TIMEOUT_SECS);
    loop {
        if Instant::now() > deadline {
            eprintln!("    timed out");
            client.send_command(TasCommand::Stop);
            return None;
        }
        let s = client.state();
        let g = s.gate_index;
        if g != 0 && s.playback_pos > g + JUDGE_TICKS + 2 {
            break;
        }
        if s.mode != TasMode::Play as u32 && s.playback_pos > 0 {
            break;
        }
        thread::sleep(Duration::from_millis(5));
    }

    let s = client.state();
    let live_gate = s.gate_index;
    if live_gate == 0 {
        client.send_command(TasCommand::Stop);
        return None;
    }
    let verdict = tas_shared::transport::judge_gate_aligned_play(
        &s.play_coords[..],
        &s.rec_coords[..],
        s.recorded_count,
        s.playback_pos,
        live_gate,
        rec_gate,
        s.capture_ok != 0,
    );
    let (matched, first_bad) = match verdict {
        BucketVerdict::Match => (true, None),
        BucketVerdict::WrongBucket { observed } => (false, observed),
        other => {
            eprintln!("    inconclusive verdict {:?}", other);
            client.send_command(TasCommand::Stop);
            return None;
        }
    };
    let a = Attempt {
        live_gate,
        rec_gate,
        gate_ticks: s.gate_tick as i64 - s.f5_press_tick as i64,
        gate_cycles: s.gate_seq as i64 - s.press_seq as i64,
        rec_arm_player: Vec::new(),
        rec_arm_physics: Vec::new(),
        rec_gate_player: Vec::new(),
        rec_gate_physics: Vec::new(),
        matched,
        first_bad,
        arm_player: s.objsnap_arm_player[..].to_vec(),
        arm_physics: s.objsnap_arm_physics[..].to_vec(),
        gate_player: s.objsnap_gate_player[..].to_vec(),
        gate_physics: s.objsnap_gate_physics[..].to_vec(),
        player_ok: s.objsnap_player_ok,
        physics_ok: s.objsnap_physics_ok,
    };
    client.send_command(TasCommand::Stop);
    // Ride-free: the next restart needs a visible teleport only for the
    // reset detector, which this hunt does not use.
    thread::sleep(Duration::from_millis(200));
    Some(a)
}

/// For one capture (a slice per attempt), find dwords whose values partition
/// the attempts by outcome.
fn partition(
    label: &str,
    attempts: &[Attempt],
    pick: impl Fn(&Attempt) -> &[u32],
    ndwords: usize,
) -> Vec<usize> {
    let good: Vec<&Attempt> = attempts.iter().filter(|a| a.matched).collect();
    let bad: Vec<&Attempt> = attempts.iter().filter(|a| !a.matched).collect();
    let mut clean: Vec<usize> = Vec::new();
    let mut varying = 0usize;
    for d in 0..ndwords {
        let mut gv: Vec<u32> = good.iter().map(|a| pick(a)[d]).collect();
        let mut bv: Vec<u32> = bad.iter().map(|a| pick(a)[d]).collect();
        gv.sort_unstable();
        gv.dedup();
        bv.sort_unstable();
        bv.dedup();
        let all_same =
            gv.len() + bv.len() <= 1 || (gv.len() == 1 && bv.len() == 1 && gv[0] == bv[0]);
        if all_same {
            continue;
        }
        varying += 1;
        // Disjoint value sets between outcomes = a clean partition.
        let disjoint = gv.iter().all(|v| !bv.contains(v));
        if disjoint && !gv.is_empty() && !bv.is_empty() {
            clean.push(d);
        }
    }
    println!(
        "  {}: {} dwords vary across attempts, {} partition cleanly by outcome",
        label,
        varying,
        clean.len()
    );
    for &d in &clean {
        let gv: Vec<String> = good
            .iter()
            .map(|a| format!("{:08x}", pick(a)[d]))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let bv: Vec<String> = bad
            .iter()
            .map(|a| format!("{:08x}", pick(a)[d]))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let gf: Vec<String> = gv
            .iter()
            .map(|h| format!("{:.6}", f32::from_bits(u32::from_str_radix(h, 16).unwrap())))
            .collect();
        let bf: Vec<String> = bv
            .iter()
            .map(|h| format!("{:.6}", f32::from_bits(u32::from_str_radix(h, 16).unwrap())))
            .collect();
        println!(
            "    +0x{:03x}: MATCH {:?} ({:?})  MISMATCH {:?} ({:?})",
            d * 4,
            gv,
            gf,
            bv,
            bf
        );
    }
    clean
}

fn pair_diff(label: &str, attempts: &[Attempt], pick: impl Fn(&Attempt) -> (&[u32], &[u32])) {
    let n = attempts
        .iter()
        .map(|a| pick(a).0.len().min(pick(a).1.len()))
        .min()
        .unwrap_or(0);
    let mut hits: Vec<(usize, usize, usize)> = Vec::new(); // (dword, bad_diffs, good_diffs)
    for d in 0..n {
        let mut bad_diff = 0usize;
        let mut good_diff = 0usize;
        for a in attempts {
            let (p, r) = pick(a);
            if p[d] != r[d] {
                if a.matched {
                    good_diff += 1
                } else {
                    bad_diff += 1
                }
            }
        }
        if bad_diff > 0 || good_diff > 0 {
            hits.push((d, bad_diff, good_diff));
        }
    }
    let n_bad = attempts.iter().filter(|a| !a.matched).count();
    let n_good = attempts.len() - n_bad;
    let discriminating: Vec<_> = hits
        .iter()
        .filter(|(_, b, g)| *b == n_bad && *g == 0)
        .collect();
    println!(
        "  {}: {} dwords differ somewhere; {} differ in EVERY mismatch and NO match",
        label,
        hits.len(),
        discriminating.len()
    );
    for (d, _, _) in discriminating.iter().take(40) {
        let ex = attempts.iter().find(|a| !a.matched).map(|a| {
            let (p, r) = pick(a);
            (p[*d], r[*d])
        });
        if let Some((pv, rv)) = ex {
            println!(
                "    +0x{:03x}: play {:08x} ({:.6}) vs rec {:08x} ({:.6})",
                d * 4,
                pv,
                f32::from_bits(pv),
                rv,
                f32::from_bits(rv)
            );
        }
    }
    let _ = n_good;
}

fn analyse(attempts: &[Attempt]) -> bool {
    let n_good = attempts.iter().filter(|a| a.matched).count();
    let n_bad = attempts.len() - n_good;
    println!(
        "\n=== ANALYSIS: {} attempts, {} matched, {} mismatched ===",
        attempts.len(),
        n_good,
        n_bad
    );
    if n_bad == 0 {
        println!(
            "  No mismatch observed — nothing to partition. Run more, or on a harder recording."
        );
        return false;
    }
    if n_good == 0 {
        println!("  No match observed — every replay diverged. That is a different problem.");
        return false;
    }

    // Gate offsets, so a partition that is just "the gate index" is recognisable.
    let mut offs: Vec<(i64, bool)> = attempts
        .iter()
        .map(|a| (a.live_gate as i64 - a.rec_gate as i64, a.matched))
        .collect();
    offs.sort();
    println!("  (gate offset, matched): {:?}", offs);

    // THE sub-tick test: does the TICK count to the gate split by outcome even
    // when the CYCLE count (gate index) matches?
    let mut good_ticks: Vec<i64> = attempts
        .iter()
        .filter(|a| a.matched)
        .map(|a| a.gate_ticks)
        .collect();
    let mut bad_ticks: Vec<i64> = attempts
        .iter()
        .filter(|a| !a.matched)
        .map(|a| a.gate_ticks)
        .collect();
    good_ticks.sort_unstable();
    good_ticks.dedup();
    bad_ticks.sort_unstable();
    bad_ticks.dedup();
    println!(
        "  gate TICK counts — matched {:?}, mismatched {:?}",
        good_ticks, bad_ticks
    );
    let mut good_cyc: Vec<i64> = attempts
        .iter()
        .filter(|a| a.matched)
        .map(|a| a.gate_cycles)
        .collect();
    let mut bad_cyc: Vec<i64> = attempts
        .iter()
        .filter(|a| !a.matched)
        .map(|a| a.gate_cycles)
        .collect();
    good_cyc.sort_unstable();
    good_cyc.dedup();
    bad_cyc.sort_unstable();
    bad_cyc.dedup();
    println!(
        "  gate CYCLE counts — matched {:?}, mismatched {:?}",
        good_cyc, bad_cyc
    );
    let ticks_split = good_ticks.iter().all(|t| !bad_ticks.contains(t)) && !bad_ticks.is_empty();
    if ticks_split {
        println!("  => the TICK count partitions the outcome: the residual IS the sub-tick.");
    }

    // Fresh mode: diff each PLAY snapshot against ITS recording's snapshot.
    // A dword that differs in every mismatched pair and no matched pair is
    // the hidden state (or sits next to it). One that differs in every pair
    // regardless is per-run noise — a pointer, a timer — and is ignored.
    if attempts.iter().all(|a| !a.rec_arm_physics.is_empty()) {
        println!("\n-- PLAY vs its own RECORDING --");
        pair_diff("arm  player ", attempts, |a| {
            (&a.arm_player, &a.rec_arm_player)
        });
        pair_diff("arm  physics", attempts, |a| {
            (&a.arm_physics, &a.rec_arm_physics)
        });
        pair_diff("gate player ", attempts, |a| {
            (&a.gate_player, &a.rec_gate_player)
        });
        pair_diff("gate physics", attempts, |a| {
            (&a.gate_physics, &a.rec_gate_physics)
        });
    }

    println!("\n-- at the ARM --");
    let ap = partition(
        "player ",
        attempts,
        |a| &a.arm_player,
        OBJSNAP_PLAYER_DWORDS,
    );
    let ah = partition(
        "physics",
        attempts,
        |a| &a.arm_physics,
        OBJSNAP_PHYSICS_DWORDS,
    );
    println!("\n-- at the GATE --");
    let gp = partition(
        "player ",
        attempts,
        |a| &a.gate_player,
        OBJSNAP_PLAYER_DWORDS,
    );
    let gh = partition(
        "physics",
        attempts,
        |a| &a.gate_physics,
        OBJSNAP_PHYSICS_DWORDS,
    );

    println!("\n=== VERDICT ===");
    if !ap.is_empty() || !ah.is_empty() {
        println!("  The hidden state is VISIBLE AT THE ARM — it can be rejected or fixed before replaying.");
        true
    } else if !gp.is_empty() || !gh.is_empty() {
        println!(
            "  The hidden state is visible only at the GATE — it evolves during the countdown."
        );
        true
    } else {
        println!(
            "  Nothing in these two objects partitions by outcome. The state lives elsewhere."
        );
        false
    }
}
