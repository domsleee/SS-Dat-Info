//! Pipeline liveness: a short REC, then a PLAY that must run to completion with
//! the player moving in both phases. Not F5-aligned, so drift is expected and
//! never asserted.

use crate::{drift, harness};

pub fn run() -> bool {
    println!("=== Smoke Test ===");
    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    println!("\n--- REC 3s ---");
    harness::arm_rec(&mut client);
    std::thread::sleep(std::time::Duration::from_secs(3));
    let rec_count = client.state().recorded_count;
    harness::stop(&mut client);
    println!("Recorded {} ticks", rec_count);

    println!("--- PLAY ---");
    harness::arm_play(&mut client);
    let play_ok = harness::wait_playback(&client, rec_count);
    harness::print_results(&client);

    let state = client.state();
    let played = state.playback_pos;
    // Assess PLAY only over frames that played: walking past playback_pos into
    // stale play_coords would manufacture movement on a short playback.
    let (rec_dx, rec_dy, rec_dz) = drift::compute_movement(&state.rec_coords, rec_count as usize);
    let (play_dx, play_dy, play_dz) =
        drift::compute_movement(&state.play_coords, rec_count.min(played) as usize);
    let rec_travel = rec_dx.max(rec_dy).max(rec_dz);
    let play_travel = play_dx.max(play_dy).max(play_dz);

    // No drift gate runs here, so check finiteness directly: `compute_movement`
    // compares with `>`, which is false for NaN.
    let finite = |coords: &[[f32; 3]], n: usize| {
        coords[..n.min(coords.len())]
            .iter()
            .all(|c| c[0].is_finite() && c[1].is_finite() && c[2].is_finite())
    };
    let coords_finite = finite(&state.rec_coords, rec_count as usize)
        && finite(&state.play_coords, rec_count.min(played) as usize);

    let recorded_ok = rec_count > 0;
    let complete_ok = play_ok && played >= rec_count;
    let rec_moved = rec_travel > 0.1;
    let play_moved = play_travel > 0.1;

    let verdict = |ok: bool| if ok { "PASS" } else { "FAIL" };
    println!("\n=== SMOKE CHECKS ===");
    println!(
        "  recorded ticks    : {} — {}",
        rec_count,
        verdict(recorded_ok)
    );
    println!(
        "  playback complete : {}/{} — {}",
        played,
        rec_count,
        verdict(complete_ok)
    );
    println!(
        "  REC movement      : max(dx,dy,dz)={:.4} (dx={:.4} dy={:.4} dz={:.4}) — {}",
        rec_travel,
        rec_dx,
        rec_dy,
        rec_dz,
        verdict(rec_moved)
    );
    println!(
        "  PLAY movement     : max(dx,dy,dz)={:.4} (dx={:.4} dy={:.4} dz={:.4}) — {}",
        play_travel,
        play_dx,
        play_dy,
        play_dz,
        verdict(play_moved)
    );
    println!(
        "  coords finite     : {} — {}",
        coords_finite,
        verdict(coords_finite)
    );
    println!("  (drift is not asserted — smoke is not F5-aligned by design)");

    let pass = recorded_ok && complete_ok && rec_moved && play_moved && coords_finite;
    if pass {
        println!("\n*** SMOKE TEST PASSED ***");
    } else {
        println!("\n*** SMOKE TEST FAILED ***");
    }
    pass
}
