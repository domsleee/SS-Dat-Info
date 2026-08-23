//! Bug #2 regression: a `continue_from_frame` that lands in shared memory during
//! a PLAIN PLAY must NOT hijack the replay into REC.
//!
//! Repro: arm a normal PLAY, then (simulating a stray UI/writer setting the
//! splice marker mid-replay) write `continue_from_frame` to a frame ahead of the
//! playhead. Without the fix, cave2's per-frame splice check flips PLAY→REC at
//! that frame and truncates the recording. With the `g_cave2_contArmed` gate, a
//! splice can only fire for a PLAY that was genuinely entered via ARM_CONTINUE,
//! so the plain replay runs to completion (PLAY→OFF) and NEVER enters REC.

use std::thread;
use std::time::{Duration, Instant};

use crate::{harness, replay};

const SPLICE_AT: u32 = 2000; // a frame well within the recording, ahead of the playhead
const REPLAY_SPEED: f32 = 32.0;

const MODE_PLAY: u32 = 2;
const MODE_REC: u32 = 1;

pub fn run() -> bool {
    println!("=== BUG #2 REGRESSION: CONT-during-PLAY must not start REC ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::stop_competing_tas_ui_writer();
    harness::stop(&mut client);
    thread::sleep(Duration::from_millis(100));

    // Load a real recording so the plain PLAY has frames to replay.
    let path = "TAS/recordings/FE-10065.tasrec";
    let candidates = [
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("../../..").join(path))),
        Some(std::path::PathBuf::from(path)),
        Some(std::path::PathBuf::from("recordings/FE-10065.tasrec")),
    ];
    let recpath = match candidates.into_iter().flatten().find(|p| p.exists()) {
        Some(p) => p,
        None => {
            eprintln!("ERROR: couldn't find {}", path);
            return false;
        }
    };
    let loaded = match replay::load_tasrec(&recpath) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("ERROR loading recording: {}", e);
            return false;
        }
    };
    replay::write_to_shared(&mut client, &loaded);
    client.state_mut().playback_speed = REPLAY_SPEED;
    println!("  Loaded {} ticks", loaded.count);

    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: F5 restart failed");
        return false;
    }

    // Arm a PLAIN PLAY (ARM_PLAY clears continue_from_frame + leaves the splice
    // gate closed).
    harness::arm_play(&mut client);

    // Wait until the replay is genuinely running but still BEFORE the splice
    // frame, then inject the stray marker — exactly the bug trigger.
    let t0 = Instant::now();
    loop {
        let pos = client.playback_pos_volatile();
        if pos >= 100 {
            break;
        }
        if client.state().mode != MODE_PLAY || t0.elapsed() > Duration::from_secs(10) {
            eprintln!(
                "  ERROR: replay didn't start (pos={}, mode={})",
                pos,
                client.state().mode
            );
            harness::stop(&mut client);
            return false;
        }
        thread::sleep(Duration::from_millis(2));
    }
    let pos_at_inject = client.playback_pos_volatile();
    println!(
        "  Replay running (pos={}); injecting stray continue_from_frame={} mid-PLAY",
        pos_at_inject, SPLICE_AT
    );
    client.state_mut().continue_from_frame = SPLICE_AT;

    // Watch the replay cross the splice frame. The bug = mode flips to REC at
    // SPLICE_AT. The fix = it stays PLAY past it (then naturally PLAY→OFF at end).
    let mut hijacked = false;
    let mut crossed = false;
    let t1 = Instant::now();
    loop {
        let (mode, pos) = {
            let s = client.state();
            (s.mode, s.playback_pos)
        };
        if mode == MODE_REC {
            hijacked = true;
            println!("  !!! HIJACKED: mode flipped to REC at pos={}", pos);
            break;
        }
        if pos > SPLICE_AT + 100 {
            crossed = true;
            break; // safely past the splice frame, still PLAY
        }
        if mode != MODE_PLAY {
            // PLAY ended (reached end of recording) without ever hitting REC.
            crossed = pos >= SPLICE_AT;
            break;
        }
        if t1.elapsed() > Duration::from_secs(20) {
            break;
        }
        thread::sleep(Duration::from_millis(2));
    }

    harness::stop(&mut client);

    println!();
    if hijacked {
        println!(
            "*** BUG #2 FAILED: plain PLAY was hijacked into REC by a stray splice marker ***"
        );
        false
    } else if crossed {
        println!(
            "*** BUG #2 PASSED: replay crossed frame {} still in PLAY — no REC hijack ***",
            SPLICE_AT
        );
        true
    } else {
        println!("*** BUG #2 INCONCLUSIVE: replay never reached the splice frame ***");
        false
    }
}
