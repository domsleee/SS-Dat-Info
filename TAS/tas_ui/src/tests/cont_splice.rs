use super::*;

fn scan_and_banner(app: &mut TasApp, state: &TasSharedState) -> bool {
    scan_drift(
        state,
        &mut app.cached_max_drift_x,
        &mut app.cached_max_drift_z,
        &mut app.cached_splice_drift_x,
        &mut app.cached_splice_drift_z,
        &mut app.cached_splice_tick,
        &mut app.last_drift_scan_count,
        &mut app.drift_aligned_bases,
        &mut app.drift_latch_arm_generation,
        &mut app.drift_first_tick,
    );
    should_show_drift_banner(
        state,
        app.drift_latch_arm_generation,
        app.drift_first_tick,
        app.cached_splice_tick,
        app.cached_splice_drift_x.max(app.cached_splice_drift_z),
        app.cached_max_drift_x.max(app.cached_max_drift_z),
    )
}

#[test]
fn cont_splice_verdict_survives_real_dll_mode_transition() {
    for skip_play_poll in [false, true] {
        let mut app = test_app();
        let mut state = tas_shared::zeroed_boxed();
        state.arm_generation = 42;
        state.mode = TasMode::Play as u32;
        state.continue_from_frame = 8;
        state.recorded_count = 12;
        state.gate_align_rec = 2;
        state.gate_index = 4;
        state.playback_pos = 9; // Last prefix sample has not been captured.
        state.play_coords[9][0] = 0.5;
        if !skip_play_poll {
            assert!(!scan_and_banner(&mut app, &state));
        }
        // cave2 captures live tick 9 (recording tick 7), increments pos,
        // then switches to REC and clears the requested splice marker.
        state.playback_pos = 10;
        state.mode = TasMode::Rec as u32;
        state.continue_from_frame = 0;
        state.segment_start_frame = 8;
        state.recorded_count = 9; // First resumed input already recorded.
        state.rec_coords[8][0] = 999.0; // Not reference data anymore.
        assert!(
            scan_and_banner(&mut app, &state),
            "lost genuine mismatch; skipped PLAY poll={skip_play_poll}"
        );
        assert_eq!(app.cached_splice_tick, Some(8));
        assert_eq!(app.cached_splice_drift_x, 0.5);
        state.mode = TasMode::Off as u32;
        assert!(!scan_and_banner(&mut app, &state));
    }
}

#[test]
fn captured_ui_break_at_4500_exercises_production_banner() {
    let meta: serde_json::Value =
        serde_json::from_str(include_str!("data/cont-splice-4500/capture.json")).unwrap();
    let raw = include_bytes!("data/cont-splice-4500/recording.tasrec");
    let play = include_bytes!("data/cont-splice-4500/play-tail.bin");
    let mut state = tas_shared::zeroed_boxed();
    state.recorded_count = u32::from_le_bytes(raw[..4].try_into().unwrap());
    assert_eq!(raw.len(), 4 + state.recorded_count as usize * 13);
    assert_eq!(
        state.recorded_count as u64,
        meta["recording_ticks"].as_u64().unwrap()
    );
    assert_eq!(meta["splice_exclusive"], 4500);
    assert_eq!(meta["play_tail_recording_start"], 945);
    assert_eq!(
        play.len(),
        meta["play_tail_samples"].as_u64().unwrap() as usize * 12
    );
    for (dst, src) in state
        .rec_coords
        .iter_mut()
        .zip(raw[4 + state.recorded_count as usize..].chunks_exact(12))
    {
        for axis in 0..3 {
            dst[axis] = f32::from_le_bytes(src[axis * 4..axis * 4 + 4].try_into().unwrap());
        }
    }
    // The saved artifact is a later playback tail, starting at rec tick
    // 945, NOT a zero-based prefix ending at 4500. Only compare its coverage.
    const START: usize = 945;
    for (dst, src) in state.play_coords[START..]
        .iter_mut()
        .zip(play.chunks_exact(12))
    {
        for axis in 0..3 {
            dst[axis] = f32::from_le_bytes(src[axis * 4..axis * 4 + 4].try_into().unwrap());
        }
    }
    assert_eq!(
        &state.play_coords[START..START + 16],
        &state.rec_coords[START..START + 16]
    );
    state.arm_generation = 42;
    state.mode = TasMode::Play as u32;
    state.continue_from_frame = 4500;
    state.gate_align_rec = START as u32;
    state.gate_index = START as u32;
    let mut app = test_app();
    for pos in (START + 1..4500).step_by(17) {
        state.playback_pos = pos as u32;
        assert!(
            !scan_and_banner(&mut app, &state),
            "transient banner at {pos}"
        );
    }
    assert_eq!(app.drift_first_tick, Some(1831));
    assert!(app.cached_max_drift_x > 0.0);
    assert!(app.cached_max_drift_x < 1.0 && app.cached_max_drift_z < 1.0);
    let first = 1831;
    assert!(
        (coordinate_delta(state.play_coords[first][0], state.rec_coords[first][0]) - 0.003112793)
            .abs()
            < 1e-6
    );
    assert!(
        (coordinate_delta(state.play_coords[first][2], state.rec_coords[first][2]) - 0.051513672)
            .abs()
            < 1e-6
    );
    assert_eq!(state.play_coords[4499], state.rec_coords[4499]);
    state.playback_pos = 4500;
    state.mode = TasMode::Rec as u32;
    state.continue_from_frame = 0;
    state.segment_start_frame = 4500;
    assert!(!scan_and_banner(&mut app, &state));
    assert_eq!(
        app.cached_splice_tick,
        Some(4500),
        "must actually judge the splice, not silently skip it"
    );
    assert_eq!(app.cached_splice_drift_x, 0.0);
    assert_eq!(app.cached_splice_drift_z, 0.0);
    // Negative control on the same real capture: endpoint divergence must warn.
    state.play_coords[4499][0] += 0.5;
    assert!(scan_and_banner(&mut app, &state));
}
