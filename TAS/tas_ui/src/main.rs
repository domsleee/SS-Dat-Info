use eframe::egui;
use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

struct TasApp {
    shared: Option<TasSharedMemoryClient>,
    connect_error: Option<String>,
}

impl TasApp {
    fn new() -> Self {
        let (shared, connect_error) = match TasSharedMemoryClient::open() {
            Ok(s) => (Some(s), None),
            Err(e) => (None, Some(e)),
        };
        Self {
            shared,
            connect_error,
        }
    }
}

impl eframe::App for TasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Supreme Snowboarding TAS");
            ui.separator();

            if let Some(ref err) = self.connect_error {
                ui.colored_label(egui::Color32::RED, format!("Not connected: {}", err));
                if ui.button("Retry connection").clicked() {
                    match TasSharedMemoryClient::open() {
                        Ok(s) => {
                            self.shared = Some(s);
                            self.connect_error = None;
                        }
                        Err(e) => self.connect_error = Some(e),
                    }
                }
                return;
            }

            if let Some(ref shared) = self.shared {
                let state = shared.state();

                // Hook status
                ui.label(format!("Version: {}", state.version));
                ui.horizontal(|ui| {
                    let hook_label = |ui: &mut egui::Ui, name: &str, hooked: u32| {
                        let color = if hooked == 1 {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };
                        ui.colored_label(color, name);
                    };
                    hook_label(ui, "Cave2", state.cave2_hooked);
                    hook_label(ui, "Cave1C", state.cave1c_hooked);
                    hook_label(ui, "Cave1D", state.cave1d_hooked);
                    hook_label(ui, "Cave5", state.cave5_hooked);
                });

                ui.separator();

                // Status
                let mode_str = match state.mode {
                    0 => "OFF",
                    1 => "REC",
                    2 => "PLAY",
                    _ => "???",
                };
                ui.label(format!("Mode: {} | Frames: {}", mode_str, state.frame_count));
                ui.label(format!(
                    "Position: ({:.1}, {:.1}, {:.1})",
                    state.player_x, state.player_y, state.player_z
                ));
                ui.label(format!(
                    "Max drift: X={:.6} Z={:.6}",
                    state.max_drift_x, state.max_drift_z
                ));
                ui.label(format!(
                    "BB3B10 calls: {} | Handler blocks: {}",
                    state.bb3b10_call_count, state.handler_block_count
                ));

                ui.separator();

                // Transport controls (Phase 3 will wire these up)
                ui.horizontal(|ui| {
                    ui.label("Transport:");
                    if ui.button("REC").clicked() {
                        // Will send CMD_ARM_REC in Phase 3
                    }
                    if ui.button("PLAY").clicked() {
                        // Will send CMD_ARM_PLAY in Phase 3
                    }
                    if ui.button("STOP").clicked() {
                        // Will send CMD_STOP in Phase 3
                    }
                });
            }
        });

        // Auto-refresh at ~30fps
        ctx.request_repaint_after(std::time::Duration::from_millis(33));
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Supreme TAS",
        options,
        Box::new(|_cc| Ok(Box::new(TasApp::new()))),
    )
}
