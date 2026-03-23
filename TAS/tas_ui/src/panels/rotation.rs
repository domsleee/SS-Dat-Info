use eframe::egui;
use tas_shared::TasSharedState;

/// Extract yaw angle from rotation matrix (atan2 of r02/r00).
fn yaw_from_matrix(m: &[f32; 9]) -> f32 {
    m[2].atan2(m[0]) // atan2(r02, r00) in left-handed coords
}

/// Extract pitch angle from rotation matrix.
fn pitch_from_matrix(m: &[f32; 9]) -> f32 {
    // pitch = asin(-r10) for XYZ euler
    (-m[3]).asin().clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2)
}

/// Extract roll angle from rotation matrix.
fn roll_from_matrix(m: &[f32; 9]) -> f32 {
    m[5].atan2(m[4]) // atan2(r12, r11)
}

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    let m = &state.rotation_matrix;

    // Check if rotation data is populated (all zeros = no data)
    let has_data = m.iter().any(|&v| v != 0.0);

    if !has_data {
        ui.label("No rotation data (DLL v5 required)");
        return;
    }

    let yaw = yaw_from_matrix(m);
    let pitch = pitch_from_matrix(m);
    let roll = roll_from_matrix(m);

    let yaw_deg = yaw.to_degrees();
    let pitch_deg = pitch.to_degrees();
    let roll_deg = roll.to_degrees();

    // Euler angles display
    ui.horizontal(|ui| {
        ui.label(format!(
            "Yaw: {:.1}  Pitch: {:.1}  Roll: {:.1}",
            yaw_deg, pitch_deg, roll_deg
        ));
    });

    // Compass visual
    let size = 80.0;
    let (response, painter) = ui.allocate_painter(egui::Vec2::splat(size), egui::Sense::hover());
    let center = response.rect.center();
    let radius = size * 0.4;

    // Background circle
    painter.circle_stroke(center, radius, egui::Stroke::new(1.0, egui::Color32::GRAY));

    // Cardinal direction labels
    let label_r = radius + 8.0;
    for (label, angle) in [("N", 0.0f32), ("E", 90.0), ("S", 180.0), ("W", -90.0)] {
        let a = angle.to_radians();
        let pos = center + egui::Vec2::new(a.sin(), -a.cos()) * label_r;
        painter.text(
            pos,
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::monospace(9.0),
            egui::Color32::DARK_GRAY,
        );
    }

    // Yaw arrow (pointing in current heading direction)
    let arrow_len = radius * 0.85;
    let arrow_end = center + egui::Vec2::new(yaw.sin(), -yaw.cos()) * arrow_len;
    painter.arrow(
        center,
        arrow_end - center,
        egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 200, 100)),
    );

    // Pitch indicator (small bar on right side)
    let pitch_bar_x = response.rect.right() + 4.0;
    let pitch_center_y = center.y;
    let pitch_height = size * 0.3;
    let pitch_offset = (pitch_deg / 90.0) * pitch_height;
    painter.line_segment(
        [
            egui::Pos2::new(pitch_bar_x, pitch_center_y - pitch_height),
            egui::Pos2::new(pitch_bar_x, pitch_center_y + pitch_height),
        ],
        egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
    );
    painter.circle_filled(
        egui::Pos2::new(pitch_bar_x, pitch_center_y - pitch_offset),
        3.0,
        egui::Color32::from_rgb(100, 149, 237),
    );

    // Raw matrix display (collapsible)
    ui.collapsing("Raw 3x3 Matrix", |ui| {
        egui::Grid::new("rotation_grid")
            .striped(true)
            .show(ui, |ui| {
                for row in 0..3 {
                    for col in 0..3 {
                        ui.label(format!("{:+.4}", m[row * 3 + col]));
                    }
                    ui.end_row();
                }
            });
    });
}
