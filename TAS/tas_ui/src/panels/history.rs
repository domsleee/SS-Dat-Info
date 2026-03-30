use eframe::egui;

use crate::recording::{HistoryEntryKind, RecordingHistory};

pub enum HistoryAction {
    Restore(usize),
}

pub fn show(ui: &mut egui::Ui, history: &RecordingHistory) -> Vec<HistoryAction> {
    let mut actions = Vec::new();

    if history.is_empty() {
        ui.label(
            egui::RichText::new(
                "No history yet. Record, continue, save, or load to create entries.",
            )
            .color(egui::Color32::from_rgb(140, 140, 140)),
        );
        return actions;
    }

    let current = history.current_index();
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, entry) in history.entries().iter().enumerate() {
                let is_current = current == Some(idx);
                let bullet = if is_current { "\u{25CF}" } else { "\u{25CB}" };
                let row = format!("{}  {}  {}", bullet, entry.timestamp, entry.label);

                if entry.can_restore() {
                    if ui.selectable_label(is_current, row).clicked() {
                        actions.push(HistoryAction::Restore(idx));
                    }
                } else {
                    let color = match entry.kind {
                        HistoryEntryKind::SaveMarker => egui::Color32::from_rgb(120, 170, 220),
                        _ => egui::Color32::from_rgb(130, 130, 130),
                    };
                    ui.label(egui::RichText::new(row).color(color));
                }
            }
        });

    actions
}
