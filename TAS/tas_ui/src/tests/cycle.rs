//! Tests for `crate::cycle`: requests the transport cycle must refuse.

use super::*;

/// CONT with no recording loaded must not leave the app half-armed at
/// catch-up speed: with nothing to splice, the PLAY→REC transition that
/// calls clear_cont_catchup would never fire.
#[test]
fn cont_with_no_recording_does_not_arm_catchup() {
    let mut app = test_app();
    app.playback_speed = 1.0;
    app.settings.cont_catchup_speed = 32.0;
    // No shared memory, so recorded_count reads as 0.
    app.queue_restart_then(TasCommand::ArmContinue);
    assert!(
        app.resume_speed.is_none(),
        "CONT without recording must not engage catchup speed"
    );
    assert!(
        (app.playback_speed - 1.0).abs() < 0.001,
        "playback_speed must remain at pre-CONT value, got {}",
        app.playback_speed
    );
    assert!(
        app.cycle.is_none(),
        "CONT without recording must not arm a controller"
    );
}

/// CONT from frame 0, or with nothing recorded, records a fresh take: it
/// arms REC, not a catch-up CONT.
#[test]
fn cont_from_zero_records_a_fresh_take() {
    for (recorded, from) in [(100, 0), (0, 0), (0, 40)] {
        let mut app = idle_in_level_app(recorded);
        app.playback_speed = 1.0;
        app.settings.cont_catchup_speed = 32.0;
        app.continue_from_frame = from;
        app.queue_restart_then(TasCommand::ArmContinue);
        assert!(
            app.cycle.is_some(),
            "recorded={recorded} from={from}: nothing armed"
        );
        assert_eq!(app.pending_session_kind, Some(RecordingSessionKind::Rec));
        assert!(app.resume_speed.is_none(), "a fresh take must not catch up");
        assert!((app.playback_speed - 1.0).abs() < 0.001);
    }
}

/// A finish that comes after the cycle starts counts even if the UI only
/// sees REC later (a CONT spliced just before the line): the baseline is
/// taken when the cycle is queued.
#[test]
fn a_cycle_takes_the_finish_baseline_when_it_starts() {
    let mut app = idle_in_level_app(100);
    // Two finishes before (seqlock: two steps each).
    app.shared
        .as_mut()
        .unwrap()
        .state_mut()
        .race_finish_seq
        .store(4, std::sync::atomic::Ordering::Release);
    app.continue_from_frame = 50;
    app.finish_seq_seen = 0;
    app.queue_restart_then(TasCommand::ArmContinue);
    assert!(app.cycle.is_some());
    assert_eq!(app.finish_seq_seen, 2);
    assert!(app.finish_baseline_armed);
    // The controller finishes (and is dropped) before the UI sees REC.
    app.cycle = None;
    // A finish in REC comes before the UI's first REC poll.
    let state = app.shared.as_mut().unwrap().state_mut();
    state.race_finish_tick = 60;
    state.race_finish_mode = TasMode::Rec as u32;
    state.race_finish_valid = 1;
    state.race_finish_time_bits = 10.0f32.to_bits();
    state
        .race_finish_seq
        .store(6, std::sync::atomic::Ordering::Release);
    state.mode = TasMode::Rec as u32;
    state.recorded_count = 61;
    app.track_mode_transitions();
    assert_eq!(app.finish_seq_seen, 2, "REC keeps the cycle's baseline");
    assert!(!app.finish_baseline_armed);
    app.watch_finish_line();
    assert_eq!(app.finished_at_tick, Some(60));
}

/// A UI opened on a buffer the DLL kept replays it in the DLL's model;
/// a loaded take's own stamp wins.
#[test]
fn the_buffer_model_falls_back_to_the_dll_when_nothing_was_loaded() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().input_model = tas_shared::TAS_INPUT_MODEL_HELD;
    app.shared = Some(shared);
    app.loaded_identity = None;
    assert_eq!(app.buffer_input_model(), tas_shared::TAS_INPUT_MODEL_HELD);
    app.loaded_identity = Some(crate::recording::IdentityStamps::default());
    assert_eq!(
        app.buffer_input_model(),
        tas_shared::TAS_INPUT_MODEL_INJECTED,
        "a loaded take without a stamp is injected"
    );
}

/// A take from another track spawns elsewhere and can never match: PLAY and
/// CONT refuse it up front instead of arming and reporting a divergence.
/// REC is unaffected, and an unknown track on either side is allowed.
#[test]
fn play_and_cont_refuse_a_take_from_another_track() {
    let arm = |take: Option<&str>, live: u32, command: TasCommand| {
        let mut app = idle_in_level_app(100);
        app.shared.as_mut().unwrap().state_mut().level_id = live;
        app.continue_from_frame = 50;
        app.loaded_level = take.map(Into::into);
        app.queue_restart_then(command);
        app.cycle.is_some()
    };
    let (fe, ve) = (0, 6);
    assert!(!arm(Some("FE"), ve, TasCommand::ArmPlay));
    assert!(!arm(Some("FE"), ve, TasCommand::ArmContinue));
    assert!(arm(Some("FE"), ve, TasCommand::ArmRec), "REC records here");
    assert!(arm(Some("FE"), fe, TasCommand::ArmPlay));
    assert!(
        arm(Some("FE"), u32::MAX, TasCommand::ArmPlay),
        "unknown track"
    );
}

/// Another rider or x87 precision can never match either: refused, with
/// the screen or setting that fixes it. Matching or unknown stamps arm.
#[test]
fn play_refuses_another_rider_or_precision() {
    let arm = |take_rider: &str, take_cw: u32, live_cw: u32| {
        let mut app = idle_in_level_app(100);
        let state = app.shared.as_mut().unwrap().state_mut();
        state.level_id = u32::MAX;
        state.fpu_control_word = live_cw;
        app.history.set_live_rider(Some("Vincent · regular".into()));
        app.loaded_rider = Some(take_rider.into());
        app.loaded_identity = Some(crate::recording::IdentityStamps {
            fpu_control_word: Some(take_cw),
            ..Default::default()
        });
        app.queue_restart_then(TasCommand::ArmPlay);
        app.cycle.is_some()
    };
    assert!(!arm("Keith · regular", 0x027F, 0x027F), "another character");
    assert!(!arm("Vincent · goofy", 0x027F, 0x027F), "another stance");
    assert!(
        !arm("Vincent · regular", 0x027F, 0x007F),
        "53-bit take, 24-bit game"
    );
    assert!(arm("Vincent · regular", 0x027F, 0x027F));
    assert!(
        arm("Vincent · regular", 0x027F, 0),
        "live precision not sampled yet"
    );
}
