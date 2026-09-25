//! Tests for `crate::cycle`: requests the transport cycle must refuse.

use super::*;

/// CONT with no recording loaded must not leave the app half-armed at
/// catch-up speed: with nothing to splice, the PLAY→REC transition that
/// calls clear_cont_catchup would never fire.
#[test]
fn cont_with_no_recording_does_not_arm_catchup() {
    let mut app = test_app();
    app.playback_speed = 1.0;
    app.cont_catchup_multiplier = 32.0;
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

// ===== Transport controller state =====

#[test]
fn cont_controller_initially_none() {
    let app = test_app();
    assert!(app.cycle.is_none());
}
