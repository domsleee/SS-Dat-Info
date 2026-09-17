//! F5-aligned straight-line REC then PLAY, judged by the gate checks.

use crate::{gates, harness};

pub fn run() -> bool {
    println!("=== F5-Aligned Zero-Drift Test ===");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    println!("\n--- Phase 1: F5 + REC ---");
    if !harness::restart_and_stabilize(&client) {
        eprintln!("ERROR: Game not alive after F5");
        return false;
    }

    harness::arm_rec(&mut client);
    println!("Recording 5s (straight line)...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("Recorded {} ticks", rec_count);

    if rec_count == 0 {
        eprintln!("ERROR: No ticks recorded");
        return false;
    }

    let s = client.state();
    let last = (rec_count - 1) as usize;
    println!(
        "First REC coord: ({:.4}, {:.4}, {:.4})",
        s.rec_coords[0][0], s.rec_coords[0][1], s.rec_coords[0][2]
    );
    println!(
        "Last REC coord [{}]: ({:.4}, {:.4}, {:.4})",
        last, s.rec_coords[last][0], s.rec_coords[last][1], s.rec_coords[last][2]
    );
    let rec_start = s.rec_coords[0];

    println!("\n--- Phase 2: F5 + PLAY ---");
    if !harness::restart_play_and_match(&mut client, rec_start, harness::START_MATCH_RETRIES)
        || !harness::wait_playback(&client, rec_count)
    {
        eprintln!("FAIL: PLAY did not match and complete");
        harness::stop(&mut client);
        return false;
    }

    let s = client.state();
    let n = rec_count as usize;
    println!("\n--- Starting position comparison ---");
    println!(
        "  REC[0]: ({:.6}, {:.6}, {:.6})",
        s.rec_coords[0][0], s.rec_coords[0][1], s.rec_coords[0][2]
    );
    println!(
        "  PLAY[0]: ({:.6}, {:.6}, {:.6})",
        s.play_coords[0][0], s.play_coords[0][1], s.play_coords[0][2]
    );
    println!(
        "  Initial offset: dx={:.9} dy={:.9} dz={:.9}",
        (s.rec_coords[0][0] as f64 - s.play_coords[0][0] as f64).abs(),
        (s.rec_coords[0][1] as f64 - s.play_coords[0][1] as f64).abs(),
        (s.rec_coords[0][2] as f64 - s.play_coords[0][2] as f64).abs()
    );

    println!("\n--- Drift progression ---");
    for &frame in &[0, 5, 10, 50, 100, 200, 500, 1000, 1500] {
        if frame < n {
            let dx = (s.rec_coords[frame][0] as f64 - s.play_coords[frame][0] as f64).abs();
            let dz = (s.rec_coords[frame][2] as f64 - s.play_coords[frame][2] as f64).abs();
            println!("  [{}] dx={:.9} dz={:.9}", frame, dx, dz);
        }
    }

    harness::print_results(&client);

    let assessment = gates::run_gates_straight(client.state(), rec_count);
    assessment.print_summary();
    assessment.all_pass()
}
