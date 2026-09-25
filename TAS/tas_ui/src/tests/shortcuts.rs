//! Tests for `crate::shortcuts`: in-window keys, global-key edges and the
//! button rule the keys share.

use super::*;
use crate::shortcuts::compute_global_key_edges;

/// Run handle_shortcuts with a simulated key press and return actions.
fn press_key(app: &mut TasApp, key: Key, modifiers: Modifiers) -> Vec<transport::Action> {
    let ctx = egui::Context::default();
    let input = RawInput {
        events: vec![Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers,
        }],
        ..Default::default()
    };

    let mut actions = Vec::new();
    let _ = ctx.run(input, |_ctx| {
        actions = app.handle_shortcuts(_ctx);
    });
    actions
}

fn action_has_command(actions: &[transport::Action], cmd: TasCommand) -> bool {
    actions
        .iter()
        .any(|a| matches!(a, transport::Action::Send(c) if *c == cmd))
}

fn action_has_restart_then(actions: &[transport::Action], cmd: TasCommand) -> bool {
    actions
        .iter()
        .any(|a| matches!(a, transport::Action::RestartThen(c) if *c == cmd))
}

fn action_has_log(actions: &[transport::Action], needle: &str) -> bool {
    actions
        .iter()
        .any(|a| matches!(a, transport::Action::Log(s) if s.contains(needle)))
}

// ===== Keyboard shortcuts =====

/// An app connected to an idle DLL in a ticking level holding `recorded`
/// ticks: the state in which the REC / PLAY / CONT buttons are enabled.
fn idle_in_level_app(recorded: u32) -> TasApp {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().game_in_game = 1;
    shared.state_mut().recorded_count = recorded;
    app.shared = Some(shared);
    app.cycle_advance_at = std::time::Instant::now();
    app
}

#[test]
fn shortcut_f9_arms_rec() {
    let mut app = idle_in_level_app(0);
    let actions = press_key(&mut app, Key::F9, Modifiers::NONE);
    assert!(action_has_restart_then(&actions, TasCommand::ArmRec));
    assert!(action_has_log(&actions, "F9"));
}

#[test]
fn shortcut_f10_arms_play() {
    let mut app = idle_in_level_app(100);
    let actions = press_key(&mut app, Key::F10, Modifiers::NONE);
    assert!(action_has_restart_then(&actions, TasCommand::ArmPlay));
    assert!(action_has_log(&actions, "F10"));
}

#[test]
fn shortcuts_are_inert_without_a_game() {
    let mut app = test_app();
    for key in [Key::F9, Key::F10, Key::F12] {
        let actions = press_key(&mut app, key, Modifiers::NONE);
        assert!(!action_has_restart_then(&actions, TasCommand::ArmRec));
        assert!(!action_has_restart_then(&actions, TasCommand::ArmPlay));
        assert!(!action_has_restart_then(&actions, TasCommand::ArmContinue));
        assert!(action_has_log(&actions, "not connected"));
    }
}

#[test]
fn shortcut_f11_stops() {
    let mut app = test_app();
    let actions = press_key(&mut app, Key::F11, Modifiers::NONE);
    assert!(action_has_command(&actions, TasCommand::Stop));
    assert!(action_has_log(&actions, "F11"));
}

#[test]
fn shortcut_f12_arms_continue() {
    let mut app = idle_in_level_app(100);
    app.continue_from_text = "50".into();
    let actions = press_key(&mut app, Key::F12, Modifiers::NONE);
    assert!(matches!(
        actions.first(),
        Some(transport::Action::SetContinueFrame(50))
    ));
    assert!(action_has_restart_then(&actions, TasCommand::ArmContinue));
    assert!(action_has_log(&actions, "F12"));
}

#[test]
fn shortcut_space_stops() {
    let mut app = test_app();
    let actions = press_key(&mut app, Key::Space, Modifiers::NONE);
    assert!(action_has_command(&actions, TasCommand::Stop));
    assert!(action_has_log(&actions, "Space"));
}

#[test]
fn shortcut_f5_without_pico_logs_not_connected() {
    let mut app = test_app();
    let actions = press_key(&mut app, Key::F5, Modifiers::NONE);
    assert!(action_has_log(&actions, "not connected"));
}

#[test]
fn shortcut_ctrl_z_undoes() {
    let mut app = test_app();
    let actions = press_key(&mut app, Key::Z, Modifiers::CTRL);
    // Must produce Undo action — not just "no panic"
    assert!(
        actions.iter().any(|a| matches!(a, transport::Action::Undo)),
        "Ctrl+Z must produce Undo action, got: {:?}",
        actions.len()
    );
    assert!(action_has_log(&actions, "Undo"), "Ctrl+Z must log Undo");
}

#[test]
fn shortcut_ctrl_y_redoes() {
    let mut app = test_app();
    let actions = press_key(&mut app, Key::Y, Modifiers::CTRL);
    assert!(
        actions.iter().any(|a| matches!(a, transport::Action::Redo)),
        "Ctrl+Y must produce Redo action"
    );
    assert!(action_has_log(&actions, "Redo"), "Ctrl+Y must log Redo");
}

#[test]
fn shortcut_shift_z_only_redoes() {
    for modifiers in [Modifiers::CTRL, Modifiers::MAC_CMD] {
        let mut app = test_app();
        let actions = press_key(
            &mut app,
            Key::Z,
            Modifiers {
                shift: true,
                ..modifiers
            },
        );
        assert_eq!(
            actions
                .iter()
                .filter(|a| matches!(a, transport::Action::Redo))
                .count(),
            1
        );
        assert!(!actions.iter().any(|a| matches!(a, transport::Action::Undo)));
    }
}

// ===== Timeline zoom (keyboard +/-) =====

#[test]
fn zoom_in_shrinks_window() {
    let mut app = test_app();
    app.timeline_view = timeline::TimelineView {
        start: 100,
        end: 1100,
    };
    press_key(&mut app, Key::Plus, Modifiers::NONE);
    assert!(app.timeline_view.end - app.timeline_view.start < 1000);
}

#[test]
fn zoom_out_grows_window() {
    let mut app = test_app();
    app.timeline_view = timeline::TimelineView {
        start: 100,
        end: 1100,
    };
    press_key(&mut app, Key::Minus, Modifiers::NONE);
    assert!(app.timeline_view.end - app.timeline_view.start > 1000);
}

#[test]
fn zoom_in_clamps_to_min_window() {
    let mut app = test_app();
    // 60-tick window is already the minimum; zooming in must not go below.
    app.timeline_view = timeline::TimelineView {
        start: 500,
        end: 560,
    };
    press_key(&mut app, Key::Plus, Modifiers::NONE);
    assert!(app.timeline_view.end - app.timeline_view.start >= 60);
}

/// The edge detector fires once per false→true transition, so holding
/// F9 in-game arms REC once rather than every frame.
#[test]
fn global_shortcut_edges_fire_only_on_press() {
    let mut prev = [false; 4];
    // First call: F9 pressed → edge for slot 0 only.
    let edges = compute_global_key_edges([true, false, false, false], &mut prev);
    assert_eq!(edges, [true, false, false, false]);
    assert_eq!(prev, [true, false, false, false]);
    // Hold: F9 still pressed → no edge.
    let edges = compute_global_key_edges([true, false, false, false], &mut prev);
    assert_eq!(edges, [false, false, false, false]);
    // Release: F9 released → no edge (we fire on press, not release).
    let edges = compute_global_key_edges([false, false, false, false], &mut prev);
    assert_eq!(edges, [false, false, false, false]);
    assert_eq!(prev, [false, false, false, false]);
    // Re-press: F9 pressed again → edge fires.
    let edges = compute_global_key_edges([true, false, false, false], &mut prev);
    assert_eq!(edges, [true, false, false, false]);
}

/// A key held across many frames (repeated identical reads) produces no
/// further edges.
#[test]
fn global_shortcut_held_does_not_repeat() {
    let mut prev = [false; 4];
    compute_global_key_edges([true, true, true, true], &mut prev);
    for _ in 0..100 {
        let edges = compute_global_key_edges([true, true, true, true], &mut prev);
        assert_eq!(edges, [false, false, false, false]);
    }
}

fn only_log(actions: &[transport::Action]) -> String {
    match actions {
        [transport::Action::Log(line)] => line.clone(),
        other => panic!("expected a single log line, got {}", other.len()),
    }
}

#[test]
fn shortcuts_follow_the_button_rule() {
    let mut app = test_app();
    let mut shared = TasSharedMemoryClient::new_test_mapping();
    shared.state_mut().game_in_game = 1;
    shared.state_mut().mode = TasMode::Rec as u32;
    shared.state_mut().recorded_count = 100;
    app.shared = Some(shared);
    app.cycle_advance_at = std::time::Instant::now(); // the level is ticking

    // F9 during a REC: the button is grey, so the key refuses too.
    let line = only_log(&app.shortcut_arm("Shortcut: F9 REC", TasCommand::ArmRec));
    assert!(
        line.contains("ignored") && line.contains("STOP first"),
        "{line}"
    );

    app.shared.as_mut().unwrap().state_mut().mode = TasMode::Off as u32;
    app.shared.as_mut().unwrap().state_mut().recorded_count = 0;
    let line = only_log(&app.shortcut_arm("Shortcut: F10 PLAY", TasCommand::ArmPlay));
    assert!(line.contains("nothing is recorded"), "{line}");

    // F12 with a typo in From: refused, the typo stays for the user to see.
    app.shared.as_mut().unwrap().state_mut().recorded_count = 100;
    app.continue_from_text = "abc".into();
    app.continue_from_frame = 40;
    let line = only_log(&app.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue));
    assert!(line.contains("\"abc\""), "{line}");
    assert_eq!(
        (app.continue_from_text.as_str(), app.continue_from_frame),
        ("abc", 40)
    );

    // F12 with a frame past the end: clamped, displayed, and armed.
    app.continue_from_text = "500".into();
    let actions = app.shortcut_arm("Shortcut: F12 CONT", TasCommand::ArmContinue);
    assert!(matches!(
        actions.as_slice(),
        [
            transport::Action::SetContinueFrame(100),
            transport::Action::RestartThen(TasCommand::ArmContinue),
            transport::Action::Log(_)
        ]
    ));
    assert_eq!(
        (app.continue_from_text.as_str(), app.continue_from_frame),
        ("100", 100)
    );

    // At a menu nothing arms.
    app.cycle_advance_at = std::time::Instant::now() - std::time::Duration::from_secs(5);
    let line = only_log(&app.shortcut_arm("Global F9 (in-game): REC", TasCommand::ArmRec));
    assert!(line.contains("enter a level first"), "{line}");
}
