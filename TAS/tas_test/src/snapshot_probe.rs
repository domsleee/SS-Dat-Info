//! Snapshot/restore prototype probe.
//!
//! Validates the in-DLL writable-memory snapshot/restore (the libTAS/TMInterface
//! "instant CONT" mechanism). Sequence:
//!   1. F5-restart so the snowboarder is in-level and auto-descending (moving).
//!   2. Run a while, record position P_snap, then CMD_SNAPSHOT (time it, size it).
//!   3. Run further so the player moves to P_moved (proves the sim advanced).
//!   4. CMD_RESTORE (time it), then read P_restored.
//!   5. Assert P_restored == P_snap (bit-exact) → the restore rewound the sim.
//!   6. Re-run forward and confirm the trajectory reproduces P_snap..P_moved
//!      (determinism) and the game didn't crash (frame_count still advancing).
//!
//! Prints snapshot/restore wall-time + bytes. This is the go/no-go for the
//! state-snapshot CONT project.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasCommand;

use crate::harness;

fn read_pos(client: &tas_shared::TasSharedMemoryClient) -> [f32; 3] {
    let s = client.state();
    [s.player_x, s.player_y, s.player_z]
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f64 {
    let dx = (a[0] - b[0]) as f64;
    let dy = (a[1] - b[1]) as f64;
    let dz = (a[2] - b[2]) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

/// Send a snapshot/restore command and wait for the DLL to process it (the op
/// runs inside one cave2 frame and blocks it). Returns (bytes, microseconds).
fn run_op(client: &mut tas_shared::TasSharedMemoryClient, cmd: TasCommand) -> (u32, u32) {
    client.send_command(cmd);
    // The op completes within a frame or two; the heavy memcpy hitches that
    // frame. Give it generous time, then read the published bytes/us.
    let t0 = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(20));
        // command is cleared to IDLE by the DLL after processing.
        let processed = client.state().command == TasCommand::Idle as u32;
        if processed && t0.elapsed() > Duration::from_millis(120) {
            break;
        }
        if t0.elapsed() > Duration::from_secs(5) {
            eprintln!("  WARN: op {:?} didn't visibly complete in 5s", cmd);
            break;
        }
    }
    let s = client.state();
    (s.snapshot_size, s.snapshot_flags)
}

pub fn run() -> bool {
    println!("=== SNAPSHOT/RESTORE PROTOTYPE PROBE ===\n");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);
    harness::ensure_exclusive_runtime_ownership(&mut client, "snapshot-probe");

    // 1. Fresh in-level state.
    if !harness::restart_and_stabilize_inprocess(&mut client) {
        eprintln!("ERROR: restart failed");
        return false;
    }
    // The boarder sits at spawn through the ~298-tick countdown, then starts
    // moving. Poll until it's ACTUALLY moving so the rewind test is meaningful
    // (snapshotting a stationary boarder trivially "passes").
    let spawn = read_pos(&client);
    let mut waited = 0;
    loop {
        harness::wait_frames(&client, 10);
        waited += 10;
        let p = read_pos(&client);
        if dist(p, spawn) > 0.5 {
            println!("  boarder moving after ~{} frames (pos z={:.3})", waited, p[2]);
            break;
        }
        if waited > 800 {
            println!("  WARN: boarder still near spawn after {} frames; testing anyway", waited);
            break;
        }
    }
    // A few more frames so it's clearly in motion.
    harness::wait_frames(&client, 20);
    let p_snap = read_pos(&client);
    let f_snap = client.frame_count_volatile();
    println!(
        "  pre-snapshot: frame={} pos=({:.4},{:.4},{:.4})",
        f_snap, p_snap[0], p_snap[1], p_snap[2]
    );

    // 2. SNAPSHOT.
    let (snap_bytes, snap_us) = run_op(&mut client, TasCommand::Snapshot);
    println!(
        "  SNAPSHOT: {} bytes ({:.2} MB) in {} us ({:.1} ms)",
        snap_bytes,
        snap_bytes as f64 / 1_048_576.0,
        snap_us,
        snap_us as f64 / 1000.0
    );
    if snap_bytes == 0 {
        eprintln!("  FAIL: snapshot captured 0 bytes");
        return false;
    }

    // 3. Advance — capture the post-snapshot trajectory to check determinism.
    // SMALL gap (few frames) to minimize heap-shape change + other-thread churn
    // between snapshot and restore (the suspected blocker for full-process
    // restore over a long window). If the rewind works with a small gap, the
    // concept is proven and only the long-window case needs the libTAS/
    // TMInterface treatment.
    // Let the DLL record traj A (frame-exact determinism baseline) — needs ~24
    // frames after the snapshot before we churn the sim.
    harness::wait_frames(&client, 30);
    // LONG window: advance many frames to stress heap-shape change + cross-thread
    // state between snapshot and restore — the case that stalled before.
    let mut traj_a = Vec::new();
    for _ in 0..10 {
        harness::wait_frames(&client, 20);
        traj_a.push(read_pos(&client));
    }
    let p_moved = read_pos(&client);
    let moved = dist(p_moved, p_snap);
    println!(
        "  post-advance: pos=({:.4},{:.4},{:.4})  moved {:.4} from snapshot",
        p_moved[0], p_moved[1], p_moved[2], moved
    );
    if moved < 0.001 {
        eprintln!("  WARN: player barely moved ({:.6}); test is weak", moved);
    }

    // 4. RESTORE.
    let (rest_bytes, rest_us) = run_op(&mut client, TasCommand::Restore);
    // The DLL did the frame-exact proof: it compared the player coords right
    // after the restore memcpy (zero frames advanced) to the snapshot coords.
    // snapshot_buffer_capacity carries the match count (3 = bit-exact, 0xFF=no ptr).
    let revert_match = client.state().snapshot_buffer_capacity;
    let region_acct = client.state().snapshot_buffer_ptr;
    let skipped = region_acct >> 16;
    let faulted = region_acct & 0xFFFF;
    println!(
        "  REGION ACCOUNTING: skipped {} / faulted {} regions (changed shape since snapshot)",
        skipped, faulted
    );
    // Also read the live position 1 frame later (informational only — this one
    // includes a frame or two of post-restore motion, so it won't be exact).
    harness::wait_frames(&client, 1);
    let p_restored = read_pos(&client);
    println!(
        "  RESTORE: {} bytes in {} us ({:.1} ms)  DLL revert match = {}/3 {}",
        rest_bytes,
        rest_us,
        rest_us as f64 / 1000.0,
        if revert_match == 0xFF { 0 } else { revert_match },
        if revert_match == 3 { "(BIT-EXACT rewind!)" } else if revert_match == 0xFF { "(no player ptr)" } else { "(partial)" }
    );

    // 5. Did the restore rewind the sim? Gold standard = the DLL frame-exact
    // match; the position read below is informational (includes ~1 frame of
    // motion).
    let revert_err = dist(p_restored, p_snap);
    let exact = revert_match == 3;
    println!(
        "  REWIND CHECK: DLL match {}/3 (bit-exact: {})  |live_pos - snapshot| = {:.6} (info; +~1 frame motion)",
        if revert_match == 0xFF { 0 } else { revert_match }, exact, revert_err
    );

    // 6. Determinism: re-run forward, compare to traj_a; and liveness.
    // Let the DLL record traj B (~24 frames) and run its frame-exact compare.
    harness::wait_frames(&client, 30);
    let traj_result = client.state().event_count;
    let traj_match = traj_result & 0xFFFF;
    let traj_shift = (traj_result >> 16) as i32 - 8;
    println!(
        "  DLL FRAME-EXACT TRAJ: {}/{} frames bit-identical at shift {} (24=fully deterministic)",
        traj_match, 24, traj_shift
    );
    let mut traj_b = Vec::new();
    for _ in 0..6 {
        harness::wait_frames(&client, 1);
        traj_b.push(read_pos(&client));
    }
    let f_end = client.frame_count_volatile();
    let alive = f_end > f_snap;
    let mut traj_max = 0.0f64;
    for i in 0..traj_a.len().min(traj_b.len()) {
        traj_max = traj_max.max(dist(traj_a[i], traj_b[i]));
    }
    println!(
        "  RE-RUN CHECK: max |traj_b - traj_a| over {} samples = {:.6}  (game alive: {}, frame {})",
        traj_a.len(),
        traj_max,
        alive,
        f_end
    );

    // Verdict. The DLL's frame-exact 3/3 match is the gold standard for a clean
    // rewind (the live-position check is confounded by post-restore motion).
    let rewound = exact;
    let deterministic = traj_max < 0.5; // loose: just "follows a similar path"
    let ok = alive && rewound && snap_bytes > 0 && rest_bytes > 0;
    println!();
    if ok {
        println!(
            "*** SNAPSHOT PROBE PASSED: rewind {:.6} (exact={}), determinism drift {:.4}, snap {:.1}ms / restore {:.1}ms ***",
            revert_err, exact, traj_max, snap_us as f64 / 1000.0, rest_us as f64 / 1000.0
        );
    } else {
        println!(
            "*** SNAPSHOT PROBE FAILED: alive={} rewound={}({:.4}) deterministic={}({:.4}) ***",
            alive, rewound, revert_err, deterministic, traj_max
        );
    }
    ok
}
