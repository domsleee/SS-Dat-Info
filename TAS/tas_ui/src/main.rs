mod app;
mod cycle;
mod drift_scan;
mod editor;
mod game_connection;
mod history_runtime;
mod history_store;
mod host;
mod level;
mod panels;
mod pico;
mod probe;
mod recording;
mod relaunch;
mod script_watch;
mod session;
mod settings;
mod shortcuts;
mod start_line;
mod take_buffer;
mod transport_runtime;
mod ui_log;
mod view;
mod win32;
mod worker;

use eframe::egui;

use app::TasApp;

fn main() -> eframe::Result {
    // Reject unsafe automation setup before opening any user history/settings.
    if std::env::var_os("SSB_INSPECT_E2E_RECORDING").is_some()
        && std::env::var_os("SSB_INSPECT_DATA_DIR").is_none()
    {
        eprintln!("UI E2E setup requires an isolated SSB_INSPECT_DATA_DIR");
        std::process::exit(1);
    }
    // Single-instance guard via a named Windows mutex.
    let instance = single_instance::SingleInstance::new("SSBInspect").unwrap();
    if !instance.is_single() {
        eprintln!("SSB Inspect is already running.");
        win32::focus_window_titled("SSB Inspect");
        std::process::exit(0);
    }

    // 1100 px wide so the transport row, with `from:` visible beside the
    // 280 px history rail, does not clip Redo or the speed presets.
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 680.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SSB Inspect",
        options,
        Box::new(|cc| {
            // Force dark theme regardless of OS setting
            cc.egui_ctx.set_theme(egui::Theme::Dark);

            let mut visuals = egui::Visuals::dark();
            let dark_bg = egui::Color32::from_gray(24);
            let widget_bg = egui::Color32::from_gray(35);
            let widget_hover = egui::Color32::from_gray(45);
            let widget_active = egui::Color32::from_gray(40);
            let subtle_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_gray(50));

            // Panel and window backgrounds
            visuals.panel_fill = dark_bg;
            visuals.window_fill = dark_bg;
            visuals.extreme_bg_color = egui::Color32::from_gray(10);
            visuals.faint_bg_color = egui::Color32::from_gray(30);
            visuals.code_bg_color = egui::Color32::from_gray(30);

            // Non-interactive widgets (labels, separators)
            visuals.widgets.noninteractive.bg_fill = dark_bg;
            visuals.widgets.noninteractive.weak_bg_fill = dark_bg;
            visuals.widgets.noninteractive.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(40));

            // Inactive widgets (buttons, combo boxes, collapsing headers)
            visuals.widgets.inactive.bg_fill = widget_bg;
            visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.inactive.bg_stroke = subtle_stroke;

            // Hovered widgets
            visuals.widgets.hovered.bg_fill = widget_hover;
            visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_gray(38);
            visuals.widgets.hovered.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(70));

            // Active (pressed) widgets
            visuals.widgets.active.bg_fill = widget_active;
            visuals.widgets.active.weak_bg_fill = egui::Color32::from_gray(35);
            visuals.widgets.active.bg_stroke =
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(80));

            // Open widgets (dropdowns)
            visuals.widgets.open.bg_fill = egui::Color32::from_gray(30);
            visuals.widgets.open.weak_bg_fill = egui::Color32::from_gray(28);

            // Window decoration
            visuals.window_stroke = egui::Stroke::new(1.0_f32, egui::Color32::from_gray(40));

            cc.egui_ctx.set_visuals_of(egui::Theme::Dark, visuals);
            Ok(Box::new(TasApp::new()))
        }),
    )
}

#[cfg(test)]
mod tests {
    mod cont_splice;
    mod cycle;
    mod e2e;
    mod fake_game;
    mod relaunch;
    mod session;
    mod shortcuts;

    #[test]
    fn load_restore_save_and_edit_keep_take_identity() {
        use tas_shared::{TAS_CHARACTER_KEITH, TAS_CHARACTER_VINCENT};
        use tas_shared::{TAS_RENDERER_DIRECTX7, TAS_RENDERER_OPENGL};

        let mut app = test_app();
        // Live game runs Vincent/DirectX7.
        app.history.list.set_live_stamps(recording::IdentityStamps {
            renderer_id: Some(TAS_RENDERER_DIRECTX7),
            fpu_control_word: Some(0x007F),
            rider_character: Some(TAS_CHARACTER_VINCENT),
            rider_stance: Some(0),
            input_model: None,
            trajectory_ticks: None,
        });
        // A Keith/OpenGL file is loaded: the buffer and its history entry
        // carry the file's stamps, not the live game's.
        let mut file_state = tas_shared::zeroed_boxed();
        file_state.recorded_count = 4;
        let keith = recording::IdentityStamps {
            renderer_id: Some(TAS_RENDERER_OPENGL),
            fpu_control_word: Some(0x027F),
            rider_character: Some(TAS_CHARACTER_KEITH),
            rider_stance: Some(0),
            input_model: None,
            trajectory_ticks: None,
        };
        assert!(app.history.list.push_loaded_snapshot(
            &file_state,
            std::path::Path::new("keith.tasrec"),
            Some(keith.clone()),
        ));
        assert_eq!(app.history.list.entries().last().unwrap().stamps, keith);
        // Restore the entry as the UI does, then save a copy: the file must
        // keep saying Keith/OpenGL even though the game runs Vincent/DX7.
        app.adopt_selected_entry_stamps();
        assert_eq!(app.take.identity.stamps, Some(keith.clone()));
        let path =
            std::env::temp_dir().join(format!("identity_chain_{}.tasrec", std::process::id()));
        recording::RecordingFile::save(&file_state, &path, app.take.identity.stamps.as_ref())
            .unwrap();
        let meta = recording::RecordingFile::read_metadata(&path).unwrap();
        assert_eq!(meta.renderer.as_deref(), Some("OpenGL"));
        assert_eq!(meta.character.as_deref(), Some("Keith"));
        // An edit derived from the loaded take keeps its identity too.
        assert!(app.history.list.push_snapshot_data_with_session(
            recording::RecordingSnapshot::from_state(&file_state),
            "Edited inputs",
            0,
            0,
            app.take.identity.stamps.clone(),
        ));
        assert_eq!(app.history.list.entries().last().unwrap().stamps, keith);
        let _ = std::fs::remove_file(&path);
    }

    /// An input edit cuts the take's judged trajectory at its first changed
    /// tick; later edits only move it earlier. The history entry and the
    /// PLAY watcher both get it.
    #[test]
    fn an_input_edit_limits_the_judged_trajectory() {
        use input_script::InputEvent;
        let mut app = test_app();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        shared.state_mut().recorded_count = 100;
        shared.state_mut().input_model = tas_shared::TAS_INPUT_MODEL_HELD;
        shared.state_mut().input_log[30..60].fill(0x04);
        app.conn.shared = Some(shared);
        let up = |start, end| InputEvent {
            bit: 0x04,
            start,
            end,
        };
        let left = |start, end| InputEvent {
            bit: 0x01,
            start,
            end,
        };
        let mut edit = |events: Vec<InputEvent>| {
            app.editor.queue(editor::Edit {
                events,
                commit: true,
                label: "edit".into(),
            });
            app.apply_pending_input_edit();
            app.take
                .identity
                .stamps
                .as_ref()
                .and_then(|i| i.trajectory_ticks)
        };
        assert_eq!(edit(vec![up(30, 60), left(70, 80)]), Some(70));
        assert_eq!(edit(vec![up(40, 60), left(70, 80)]), Some(30));
        assert_eq!(edit(vec![up(40, 60), left(70, 90)]), Some(30));
        assert_eq!(
            app.take
                .identity
                .stamps
                .as_ref()
                .and_then(|i| i.input_model),
            Some(tas_shared::TAS_INPUT_MODEL_HELD),
            "the stamp keeps the buffer's model"
        );
        assert_eq!(
            app.history
                .list
                .entries()
                .last()
                .unwrap()
                .stamps
                .trajectory_ticks,
            Some(30)
        );
    }

    /// A UI opened on the take the DLL kept takes the stamps of the history
    /// entry holding it, edit limit and track included; another take gets
    /// none.
    #[test]
    fn a_reopened_ui_takes_the_stamps_of_the_entry_holding_the_take() {
        for same_take in [true, false] {
            let mut app = test_app();
            let mut entry_state = tas_shared::zeroed_boxed();
            entry_state.recorded_count = 100;
            entry_state.input_log[30..60].fill(0x04);
            app.history.list.set_live_level(Some("FE"));
            assert!(app.history.list.push_snapshot_data_with_session(
                recording::RecordingSnapshot::from_state(&entry_state),
                "Deleted D 60-70t",
                0,
                0,
                Some(recording::IdentityStamps {
                    input_model: Some(tas_shared::TAS_INPUT_MODEL_HELD),
                    trajectory_ticks: Some(60),
                    ..Default::default()
                }),
            ));
            let mut shared = TasSharedMemoryClient::new_test_mapping();
            shared.state_mut().recorded_count = 100;
            shared.state_mut().input_log[30..60].fill(0x04);
            if !same_take {
                shared.state_mut().input_log[80] = 0x01;
            }
            app.conn.shared = Some(shared);
            app.take.identity.stamps = None;
            app.adopt_buffer_identity();
            let limit = app
                .take
                .identity
                .stamps
                .as_ref()
                .and_then(|i| i.trajectory_ticks);
            if same_take {
                assert_eq!(limit, Some(60));
                assert_eq!(app.take.identity.level.as_deref(), Some("FE"));
            } else {
                assert_eq!(limit, None);
                assert!(app.take.identity.stamps.is_none());
            }
        }
    }

    /// At a CONT splice the replayed prefix becomes the new take's
    /// trajectory, so an edit before the splice leaves nothing stale.
    #[test]
    fn a_cont_splice_adopts_the_replayed_prefix() {
        let mut app = test_app();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        let s = shared.state_mut();
        s.recorded_count = 100;
        for t in 0..100 {
            s.rec_coords[t] = if t < 10 {
                [1.0, 2.0, 3.0]
            } else {
                [t as f32, 0.0, 0.0]
            };
        }
        // Live gate 12; the replay departs from the recording at gate+30.
        s.gate_index = 12;
        for k in 0..90 {
            let y = if k < 30 { 0.0 } else { 1.0 };
            s.play_coords[12 + k] = [(10 + k) as f32, y, 0.0];
        }
        // The splice fired: REC, and the DLL has cleared continue_from_frame.
        s.mode = TasMode::Rec as u32;
        s.continue_from_frame = 0;
        app.conn.shared = Some(shared);
        app.session.last_mode = TasMode::Play as u32;
        app.session.pending = Some(crate::session::StartContext::Continue { splice: 80 });
        app.track_mode_transitions();
        let s = app.conn.shared.as_ref().unwrap().state();
        assert_eq!(s.rec_coords[5], [1.0, 2.0, 3.0], "spawn untouched");
        assert_eq!(s.rec_coords[39], [39.0, 0.0, 0.0]);
        assert_eq!(s.rec_coords[40], [40.0, 1.0, 0.0]);
        assert_eq!(s.rec_coords[79], [79.0, 1.0, 0.0]);
        assert_eq!(
            s.rec_coords[80],
            [80.0, 0.0, 0.0],
            "REC's own part untouched"
        );
    }

    #[test]
    fn replacing_a_recording_detaches_its_script_and_queued_stop_edit() {
        let mut app = test_app();
        let old_path = crate::script_watch::ScriptWatch::fresh_path();
        std::fs::write(&old_path, "1-5 press left").unwrap();
        app.editor
            .watch_script(old_path.clone(), "1-5 press left".into());
        app.editor.pending = editor::PendingEdit::WaitingForStop(editor::Edit {
            events: Vec::new(),
            commit: true,
            label: "old script".into(),
        });
        assert!(app.stop_active_session_for_load());
        std::fs::write(&old_path, "2-9 press shift").unwrap();
        app.editor.poll_script(&mut app.log_lines);
        assert!(app.editor.script_watch.is_none());
        assert_eq!(app.editor.pending, editor::PendingEdit::None);
        std::fs::remove_file(old_path).unwrap();
    }

    use super::*;
    use crate::app::stop_is_acknowledged;
    use crate::panels::{input_script, timeline, transport};
    use crate::transport_runtime::normalize_playback_speed;
    use egui::{Event, Key, Modifiers, RawInput};
    use recording::RecordingSessionKind;
    use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient, TasSharedState};

    #[test]
    fn recording_buffer_overwrite_requires_stop_acknowledgement() {
        assert!(!stop_is_acknowledged(TasMode::Play as u32, false));
        assert!(!stop_is_acknowledged(TasMode::Play as u32, true));
        assert!(!stop_is_acknowledged(TasMode::Off as u32, false));
        assert!(stop_is_acknowledged(TasMode::Off as u32, true));
    }

    /// Test constructor: creates TasApp without shared memory or Pico.
    fn test_app() -> TasApp {
        let mut app = TasApp::blank(settings::Settings {
            history_cap: 64,
            cont_catchup_speed: 12.0,
            ..Default::default()
        });
        app.conn.connect_error = Some("Test mode: no DLL".into());
        app
    }

    /// An app connected to an idle DLL in a ticking level holding `recorded`
    /// ticks: the state in which the REC / PLAY / CONT buttons are enabled.
    fn idle_in_level_app(recorded: u32) -> TasApp {
        let mut app = test_app();
        let mut shared = TasSharedMemoryClient::new_test_mapping();
        shared.state_mut().game_in_game = 1;
        shared.state_mut().recorded_count = recorded;
        app.conn.shared = Some(shared);
        app.conn.cycle_advance_at = std::time::Instant::now();
        app
    }

    // ===== App startup without DLL =====

    #[test]
    fn app_starts_without_dll() {
        let app = test_app();
        assert!(app.conn.shared.is_none());
        assert!(app.conn.connect_error.is_some());
        assert_eq!(app.transport.playback_speed, 1.0);
        assert_eq!(app.view.timeline, timeline::TimelineView::default());
        assert!(!app.pico.connected);
    }

    #[test]
    fn restoring_while_idle_cancels_pending_arm() {
        use tas_shared::transport::{ArmConfig, TransportController};
        use transport_runtime::{ArmRequest, TransportState};
        for command in [
            TasCommand::ArmRec,
            TasCommand::ArmPlay,
            TasCommand::ArmContinue,
        ] {
            let mut app = test_app();
            let request = ArmRequest {
                command,
                arming_allowed: true,
                recorded: 500,
                continue_from_frame: 400,
                gate: 299,
                input_model: tas_shared::TAS_INPUT_MODEL_INJECTED,
                trajectory_ticks: u32::MAX,
            };
            app.transport.state = TransportState::Running {
                controller: TransportController::new(ArmConfig {
                    arm: request.arm(),
                    speed: 256.0,
                    continue_from_frame: 400,
                    gate_align_rec: 299,
                    max_retries: 1,
                    input_model: tas_shared::TAS_INPUT_MODEL_INJECTED,
                    trajectory_ticks: u32::MAX,
                }),
                deadline: std::time::Instant::now(),
                request,
            };
            app.transport.resume_speed = Some(1.0);
            app.transport.playback_speed = 256.0;
            app.session.pending = Some(crate::session::StartContext::Continue { splice: 400 });
            assert!(app.stop_active_session_for_load());
            assert!(!app.transport.is_running());
            assert!(app.session.pending.is_none());
            assert!(app.transport.resume_speed.is_none());
            assert_eq!(app.transport.playback_speed, 1.0);
        }
    }

    // ===== Speed edge values =====

    #[test]
    fn playback_speed_defaults_to_1x() {
        let app = test_app();
        assert!((app.transport.playback_speed - 1.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_rejects_catchup_values() {
        assert!((normalize_playback_speed(12.0) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(f32::NAN) - 1.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_keeps_valid_presets() {
        assert!((normalize_playback_speed(0.25) - 0.25).abs() < 0.001);
        assert!((normalize_playback_speed(0.5) - 0.5).abs() < 0.001);
        assert!((normalize_playback_speed(2.0) - 2.0).abs() < 0.001);
    }

    #[test]
    fn playback_speed_normalizer_resets_dropped_presets() {
        // 0.01x, 0.05x and 4x are not preset buttons — any persisted setting
        // at those values should fall back to 1x. (0.25x is a valid preset,
        // covered by playback_speed_normalizer_keeps_valid_presets.)
        assert!((normalize_playback_speed(0.01) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(0.05) - 1.0).abs() < 0.001);
        assert!((normalize_playback_speed(4.0) - 1.0).abs() < 0.001);
    }

    fn state_with_recorded_count(recorded_count: u32) -> Box<TasSharedState> {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = recorded_count;
        for i in 0..(recorded_count as usize).min(tas_shared::TAS_MAX_TICKS) {
            state.input_log[i] = 0x01;
            state.rec_coords[i] = [i as f32, 0.0, i as f32];
        }
        state
    }

    fn finish(app: &mut TasApp, seq: u32, tick: u32, mode: TasMode, valid: bool, seconds: f32) {
        let state = app.conn.shared.as_mut().unwrap().state_mut();
        state.race_finish_tick = tick;
        state.race_finish_mode = mode as u32;
        state.race_finish_valid = valid as u32;
        state.race_finish_time_bits = seconds.to_bits();
        state.fpu_control_word = 0x027F;
        // The DLL's seqlock: even when stable, two steps per finish.
        state
            .race_finish_seq
            .store(seq * 2, std::sync::atomic::Ordering::Release);
    }

    #[test]
    fn only_a_rec_finish_after_rec_began_stops_the_take() {
        let mut app = test_app();
        app.conn.shared = Some(TasSharedMemoryClient::new_test_mapping());
        // A finish from before REC began (a CONT prefix's replay).
        finish(&mut app, 1, 900, TasMode::Rec, true, 9.0);
        app.session.finish.seen_seq = 1;
        app.watch_finish_line();
        assert_eq!(app.session.finish.at_tick, None);
        // A replay's finish.
        finish(&mut app, 2, 950, TasMode::Play, true, 9.5);
        app.watch_finish_line();
        assert_eq!(app.session.finish.at_tick, None);

        finish(&mut app, 3, 5843, TasMode::Rec, true, 58.4262);
        app.watch_finish_line();
        assert_eq!(app.session.finish.at_tick, Some(5843));
        assert_eq!(app.session.finish.hud_cs, Some(5842));
        assert!(app
            .log_lines
            .lines()
            .iter()
            .any(|l| l.contains("Finished at tick 5843") && l.contains("0:58.42")));
        assert_eq!(
            app.conn.shared.as_ref().unwrap().state().command,
            TasCommand::Stop as u32
        );

        // Latched: a later finish doesn't move it.
        finish(&mut app, 4, 7000, TasMode::Rec, true, 70.0);
        app.watch_finish_line();
        assert_eq!(app.session.finish.at_tick, Some(5843));
    }

    #[test]
    fn a_finish_that_missed_a_checkpoint_has_no_official_time() {
        let mut app = test_app();
        app.conn.shared = Some(TasSharedMemoryClient::new_test_mapping());
        finish(&mut app, 1, 4000, TasMode::Rec, false, 40.0);
        app.watch_finish_line();
        assert_eq!(app.session.finish.at_tick, Some(4000));
        assert_eq!(app.session.finish.hud_cs, None);
        assert!(app
            .log_lines
            .lines()
            .iter()
            .any(|l| l.contains("a checkpoint was missed")));
    }
}
