//! restart-probe: measure the in-process F5 restart timeline directly.
//!
//! The CONT bug is that old recordings have a ~300-frame (3s) countdown but the
//! in-process F5 restart (added in d04104f) yields a first-moving frame of ~211.
//! Two possibilities:
//!   (1) the countdown is full (~300) but REC/PLAY ARMS ~89 frames into it, or
//!   (2) the injected F5 produces a genuinely SHORTER countdown.
//!
//! This probe ignores REC/PLAY entirely: it triggers CMD_RESTART (the same
//! in-process F5 path) and watches the live player position (updated every frame
//! regardless of mode) to time F5-press -> teleport-to-spawn -> first-motion.
//! That count IS the game's countdown after the injected F5. If it's ~300 the
//! fix is arm-timing; if it's ~211 the fix is the F5 injection itself.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasCommand;

use crate::harness;

/// How many frames past F5-press to watch.
const WATCH_FRAMES: u32 = 520;
/// A coord delta below this (per sample) counts as "stationary".
const MOVE_EPS: f32 = 1e-3;

fn dist(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn run() -> bool {
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "restart-probe");

    // Clean OFF state; let the player be wherever it is (mid-run is fine — the
    // teleport-to-spawn then becomes visible).
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(200));

    client.reset_restart_state();
    let press_frame = client.frame_count_volatile();
    let pre = {
        let s = client.state();
        (s.player_x, s.player_y, s.player_z)
    };
    println!("=== RESTART PROBE ===");
    println!(
        "  F5-press at frame {} (pre-restart pos = ({:.1},{:.1},{:.1}))",
        press_frame, pre.0, pre.1, pre.2
    );
    client.send_command(TasCommand::Restart);

    // (rel_frame, mode, restart_state, x, y, z)
    let mut samples: Vec<(u32, u8, u8, f32, f32, f32)> = Vec::new();
    let mut last_sampled: i64 = -100;
    let mut rs2_rel: Option<u32> = None;
    let t0 = Instant::now();
    loop {
        let (f, mode, rs, pos) = {
            let s = client.state();
            (
                s.frame_count,
                s.mode as u8,
                s.restart_state as u8,
                (s.player_x, s.player_y, s.player_z),
            )
        };
        let rel = f.wrapping_sub(press_frame);
        if rs == 2 && rs2_rel.is_none() {
            rs2_rel = Some(rel);
        }
        if rel as i64 - last_sampled >= 3 {
            samples.push((rel, mode, rs, pos.0, pos.1, pos.2));
            last_sampled = rel as i64;
        }
        if rel >= WATCH_FRAMES || t0.elapsed() > Duration::from_secs(12) {
            break;
        }
        thread::sleep(Duration::from_millis(6));
    }

    // ---- Analysis ----
    // 1. Find the teleport-to-spawn: the largest single-sample jump (mid-run pos
    //    -> spawn). Everything after the LAST big jump and before motion is the
    //    countdown.
    // 2. Spawn = the stable position right after the teleport.
    // 3. first-motion = first sample after spawn that moves > MOVE_EPS from spawn.
    let mut teleport_idx = 0usize;
    let mut max_jump = 0.0f32;
    for i in 1..samples.len() {
        let a = (samples[i - 1].3, samples[i - 1].4, samples[i - 1].5);
        let b = (samples[i].3, samples[i].4, samples[i].5);
        let d = dist(a, b);
        if d > max_jump {
            max_jump = d;
            teleport_idx = i;
        }
    }
    let spawn = (
        samples[teleport_idx].3,
        samples[teleport_idx].4,
        samples[teleport_idx].5,
    );
    let spawn_rel = samples[teleport_idx].0;
    // first motion after the teleport
    let mut first_move_rel: Option<u32> = None;
    for s in samples.iter().skip(teleport_idx + 1) {
        if dist((s.3, s.4, s.5), spawn) > MOVE_EPS {
            first_move_rel = Some(s.0);
            break;
        }
    }

    println!("\n  {:>6} {:>4} {:>3} {:>11} {:>11} {:>11}", "rel", "mode", "rs", "x", "y", "z");
    for (rel, mode, rs, x, y, z) in &samples {
        let mark = if *rel == spawn_rel {
            "  <- teleport/spawn"
        } else if Some(*rel) == first_move_rel {
            "  <- FIRST MOTION"
        } else {
            ""
        };
        println!("  {:>6} {:>4} {:>3} {:>11.3} {:>11.3} {:>11.3}{}", rel, mode, rs, x, y, z, mark);
    }

    println!("\n=== TIMELINE ===");
    println!("  F5-press:           rel 0");
    println!("  restart_state==2:   rel {:?}  (F5 released / restart 'done')", rs2_rel);
    println!("  teleport to spawn:  rel {}  (jump {:.2} to ({:.1},{:.1},{:.1}))", spawn_rel, max_jump, spawn.0, spawn.1, spawn.2);
    match first_move_rel {
        Some(fm) => {
            println!("  first motion:       rel {}", fm);
            let countdown_from_press = fm;
            let countdown_from_spawn = fm.saturating_sub(spawn_rel);
            println!();
            println!("  >>> countdown F5-press -> first-motion = {} frames (~{:.2}s)", countdown_from_press, countdown_from_press as f32 / 100.0);
            println!("  >>> countdown spawn   -> first-motion = {} frames (~{:.2}s)", countdown_from_spawn, countdown_from_spawn as f32 / 100.0);
            println!();
            if countdown_from_press >= 280 {
                println!("  VERDICT: countdown is ~FULL (~300). The 211 first-moving is an ARM-TIMING");
                println!("           problem — REC/PLAY captures play_coords[0] ~{} frames late. Fix the arm point.", countdown_from_press.saturating_sub(211));
            } else {
                println!("  VERDICT: countdown is SHORT (~{}). The injected F5 itself yields a shorter", countdown_from_press);
                println!("           restart than a real keypress. Fix the F5 injection (hold length / BB3B10).");
            }
        }
        None => {
            println!("  first motion:       NOT observed within {} frames", WATCH_FRAMES);
        }
    }

    first_move_rel.is_some()
}
