use eframe::egui;

use crate::ui_log::UiLog;

pub fn show(ui: &mut egui::Ui, log: &mut UiLog) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Log").strong());
        if ui.small_button("Copy").clicked() {
            ui.ctx().copy_text(log.lines().join("\n"));
        }
        if ui.small_button("Clear").clicked() {
            log.clear();
        }
    });

    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .auto_shrink(false)
        .show(ui, |ui| {
            for line in log.lines() {
                ui.label(egui::RichText::new(line).monospace().size(10.0));
            }
        });
}
