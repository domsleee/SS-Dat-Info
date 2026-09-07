use crate::drift_scan::{coordinate_delta, DriftTracker};
use tas_shared::{TasMode, TasSharedState};

fn scan_and_banner(tracker: &mut DriftTracker, state: &TasSharedState) -> bool {
    tracker.scan(state);
    tracker.banner_visible(state)
}

#[test]
fn cont_splice_verdict_survives_real_dll_mode_transition() {
    for skip_play_poll in [false, true] {
        let mut tracker = DriftTracker::default();
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
            assert!(!scan_and_banner(&mut tracker, &state));
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
            scan_and_banner(&mut tracker, &state),
            "lost genuine mismatch; skipped PLAY poll={skip_play_poll}"
        );
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.5);
        state.mode = TasMode::Off as u32;
        assert!(!scan_and_banner(&mut tracker, &state));
    }
}

/// A captured CONT whose prefix replay has a transient (a jump wobble that
/// heals) well before the splice: the banner must stay clear through the
/// transient and judge the splice itself.
#[test]
fn captured_ui_break_at_4500_exercises_production_banner() {
    let meta: serde_json::Value =
        serde_json::from_str(include_str!("data/cont-splice-4500/capture.json")).unwrap();
    // `u32le count | input[count] | f32le xyz[count]`; only the XYZ block is read.
    let raw = include_bytes!("data/cont-splice-4500/recording.tasrec");
    // `f32le xyz` samples of the playback, starting at `play_tail_recording_start`.
    let play = include_bytes!("data/cont-splice-4500/play-tail.bin");
    let mut state = tas_shared::zeroed_boxed();
    state.recorded_count = u32::from_le_bytes(raw[..4].try_into().unwrap());
    assert_eq!(raw.len(), 4 + state.recorded_count as usize * 13);
    assert_eq!(
        state.recorded_count as u64,
        meta["recording_ticks"].as_u64().unwrap()
    );
    let splice = meta["splice_exclusive"].as_u64().unwrap() as usize;
    let start = meta["play_tail_recording_start"].as_u64().unwrap() as usize;
    let first = meta["first_transient_tick"].as_u64().unwrap() as usize;
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
    for (dst, src) in state.play_coords[start..]
        .iter_mut()
        .zip(play.chunks_exact(12))
    {
        for axis in 0..3 {
            dst[axis] = f32::from_le_bytes(src[axis * 4..axis * 4 + 4].try_into().unwrap());
        }
    }
    assert_eq!(
        &state.play_coords[start..start + 16],
        &state.rec_coords[start..start + 16]
    );
    state.arm_generation = 42;
    state.mode = TasMode::Play as u32;
    state.continue_from_frame = splice as u32;
    state.gate_align_rec = start as u32;
    state.gate_index = start as u32;
    let mut tracker = DriftTracker::default();
    for pos in (start + 1..splice).step_by(17) {
        state.playback_pos = pos as u32;
        assert!(
            !scan_and_banner(&mut tracker, &state),
            "transient banner at {pos}"
        );
    }
    assert_eq!(tracker.first_drift_tick, Some(first));
    assert!(tracker.max_dx > 0.0);
    assert!(tracker.max_dx < 1.0 && tracker.max_dz < 1.0);
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
    assert_eq!(state.play_coords[splice - 1], state.rec_coords[splice - 1]);
    state.playback_pos = splice as u32;
    state.mode = TasMode::Rec as u32;
    state.continue_from_frame = 0;
    state.segment_start_frame = splice as u32;
    assert!(!scan_and_banner(&mut tracker, &state));
    assert_eq!(
        tracker.splice_tick,
        Some(splice),
        "must actually judge the splice, not silently skip it"
    );
    assert_eq!(tracker.splice_dx, 0.0);
    assert_eq!(tracker.splice_dz, 0.0);
    // Negative control on the same real capture: endpoint divergence must warn.
    state.play_coords[splice - 1][0] += 0.5;
    assert!(scan_and_banner(&mut tracker, &state));
}
