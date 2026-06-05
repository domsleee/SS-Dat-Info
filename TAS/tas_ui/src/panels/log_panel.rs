use eframe::egui;

pub fn show(ui: &mut egui::Ui, log_lines: &mut Vec<String>) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Log").strong());
        if ui.small_button("Copy").clicked() {
            ui.ctx().copy_text(log_lines.join("\n"));
        }
        if ui.small_button("Clear").clicked() {
            log_lines.clear();
        }
    });

    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .auto_shrink(false)
        .show(ui, |ui| {
            for line in log_lines.iter() {
                ui.label(egui::RichText::new(line).monospace().size(10.0));
            }
        });
}
