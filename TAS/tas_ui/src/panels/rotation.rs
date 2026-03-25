use eframe::egui;
use tas_shared::TasSharedState;

/// Apply the 3x3 rotation matrix (row-major) to a 3D point.
fn rotate(m: &[f32; 9], p: [f32; 3]) -> [f32; 3] {
    [
        m[0] * p[0] + m[1] * p[1] + m[2] * p[2],
        m[3] * p[0] + m[4] * p[1] + m[5] * p[2],
        m[6] * p[0] + m[7] * p[1] + m[8] * p[2],
    ]
}

/// Project a 3D point to 2D using a fixed isometric-style camera.
/// Camera looks from above-right-front at a slight angle.
fn project(p: [f32; 3]) -> egui::Vec2 {
    // Isometric projection: X goes right+down, Y goes up, Z goes right+up
    let x2d = p[0] * 0.87 + p[2] * 0.5;
    let y2d = -p[1] * 0.9 + p[0] * 0.25 - p[2] * 0.43;
    egui::Vec2::new(x2d, y2d)
}

/// Extract yaw angle from rotation matrix (atan2 of r02/r00).
fn yaw_from_matrix(m: &[f32; 9]) -> f32 {
    m[2].atan2(m[0])
}

/// Extract pitch angle from rotation matrix.
fn pitch_from_matrix(m: &[f32; 9]) -> f32 {
    m[3].asin().clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2)
}

/// Extract roll angle from rotation matrix.
fn roll_from_matrix(m: &[f32; 9]) -> f32 {
    m[5].atan2(m[4])
}

/// Draw a 3D arrow (axis line with small arrowhead) and label.
fn draw_axis(
    painter: &egui::Painter,
    center: egui::Pos2,
    scale: f32,
    m: &[f32; 9],
    axis: [f32; 3],
    color: egui::Color32,
    label: &str,
) {
    let tip_3d = rotate(m, axis);
    let tip_2d = project(tip_3d) * scale;
    let end = center + tip_2d;

    // Main axis line
    painter.line_segment(
        [center, end],
        egui::Stroke::new(2.5, color),
    );

    // Arrowhead (two small lines)
    let dir = tip_2d.normalized();
    let perp = egui::Vec2::new(-dir.y, dir.x);
    let head_len = 6.0;
    let head_base = end - dir * head_len;
    painter.line_segment(
        [end, head_base + perp * 3.0],
        egui::Stroke::new(2.0, color),
    );
    painter.line_segment(
        [end, head_base - perp * 3.0],
        egui::Stroke::new(2.0, color),
    );

    // Label at tip
    let label_pos = end + tip_2d.normalized() * 10.0;
    painter.text(
        label_pos,
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::monospace(11.0),
        color,
    );
}

/// Draw a wireframe box rotated by the matrix to give spatial context.
fn draw_wireframe_body(
    painter: &egui::Painter,
    center: egui::Pos2,
    scale: f32,
    m: &[f32; 9],
) {
    // A flat elongated box representing the snowboard/rider orientation
    let hx = 0.15_f32; // narrow
    let hy = 0.1;      // thin
    let hz = 0.5;      // long (forward axis)

    let verts: [[f32; 3]; 8] = [
        [-hx, -hy, -hz], [ hx, -hy, -hz], [ hx,  hy, -hz], [-hx,  hy, -hz],
        [-hx, -hy,  hz], [ hx, -hy,  hz], [ hx,  hy,  hz], [-hx,  hy,  hz],
    ];
    let edges: [(usize, usize); 12] = [
        (0,1),(1,2),(2,3),(3,0), // back face
        (4,5),(5,6),(6,7),(7,4), // front face
        (0,4),(1,5),(2,6),(3,7), // connecting edges
    ];

    let projected: Vec<egui::Pos2> = verts.iter()
        .map(|v| {
            let r = rotate(m, *v);
            center + project(r) * scale
        })
        .collect();

    let wire_color = egui::Color32::from_rgba_premultiplied(180, 180, 220, 140);
    for (a, b) in &edges {
        painter.line_segment(
            [projected[*a], projected[*b]],
            egui::Stroke::new(1.0, wire_color),
        );
    }
}

/// Transform a rotation matrix from game coordinates (Y-down) to display
/// coordinates (Y-up) via conjugation with S = diag(1, -1, 1).
/// This negates the off-diagonal Y elements: m[1], m[3], m[5], m[7].
fn flip_y(raw: &[f32; 9]) -> [f32; 9] {
    [
        raw[0], -raw[1], raw[2],
        -raw[3], raw[4], -raw[5],
        raw[6], -raw[7], raw[8],
    ]
}

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    let raw = &state.rotation_matrix;

    let has_data = raw.iter().any(|&v| v != 0.0);

    if !has_data {
        ui.label("No rotation data (DLL v5 required)");
        return;
    }

    // Game uses Y-down; flip to Y-up for all display purposes.
    let m = flip_y(raw);

    let yaw = yaw_from_matrix(&m);
    let pitch = pitch_from_matrix(&m);
    let roll = roll_from_matrix(&m);

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

    // 3D rotation visualization
    let size = 140.0;
    let (response, painter) = ui.allocate_painter(egui::Vec2::splat(size), egui::Sense::hover());
    let center = response.rect.center();
    let scale = size * 0.35;

    // Background circle (horizon reference)
    painter.circle_stroke(
        center,
        scale * 0.95,
        egui::Stroke::new(0.5, egui::Color32::from_gray(60)),
    );

    // Draw wireframe body showing board orientation
    draw_wireframe_body(&painter, center, scale, &m);

    // Draw rotated coordinate axes: X=red, Y=green, Z=blue
    draw_axis(&painter, center, scale, &m, [1.0, 0.0, 0.0], egui::Color32::from_rgb(220, 80, 80), "X");
    draw_axis(&painter, center, scale, &m, [0.0, 1.0, 0.0], egui::Color32::from_rgb(80, 200, 80), "Y");
    draw_axis(&painter, center, scale, &m, [0.0, 0.0, 1.0], egui::Color32::from_rgb(80, 130, 230), "Z");

    // Center dot
    painter.circle_filled(center, 2.5, egui::Color32::WHITE);

    // Raw matrix display (collapsible) — shows original game values
    ui.collapsing("Raw 3x3 Matrix", |ui| {
        egui::Grid::new("rotation_grid")
            .striped(true)
            .show(ui, |ui| {
                for row in 0..3 {
                    for col in 0..3 {
                        ui.label(format!("{:+.4}", raw[row * 3 + col]));
                    }
                    ui.end_row();
                }
            });
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Identity matrix → all Euler angles should be zero.
    #[test]
    fn identity_matrix_gives_zero_angles() {
        let identity: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        assert!((yaw_from_matrix(&identity)).abs() < 1e-6);
        assert!((pitch_from_matrix(&identity)).abs() < 1e-6);
        assert!((roll_from_matrix(&identity)).abs() < 1e-6);
    }

    /// flip_y negates exactly the off-diagonal Y elements.
    #[test]
    fn flip_y_negates_correct_elements() {
        let m: [f32; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let f = flip_y(&m);
        // m[0], m[2], m[4], m[6], m[8] unchanged
        assert_eq!(f[0], 1.0);
        assert_eq!(f[2], 3.0);
        assert_eq!(f[4], 5.0);
        assert_eq!(f[6], 7.0);
        assert_eq!(f[8], 9.0);
        // m[1], m[3], m[5], m[7] negated
        assert_eq!(f[1], -2.0);
        assert_eq!(f[3], -4.0);
        assert_eq!(f[5], -6.0);
        assert_eq!(f[7], -8.0);
    }

    /// flip_y of identity is still identity (off-diag Y elements are 0).
    #[test]
    fn flip_y_identity_stays_identity() {
        let identity: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let f = flip_y(&identity);
        assert_eq!(f, identity);
    }

    /// Full pipeline: game matrix with positive raw m[3] (front flip in Y-down
    /// game coords) should display as positive pitch after Y-flip.
    ///
    /// In game coords (Y-down), raw m[3] > 0 means a front flip.
    /// After flip_y, m[3] becomes negative → pitch_from_matrix gives negative.
    /// But we interpret negative display-pitch as "nose down" = front flip.
    ///
    /// Conversely, raw m[3] < 0 in game = back flip.
    /// After flip_y, m[3] becomes positive → positive pitch = "nose up" = back flip.
    #[test]
    fn front_flip_game_matrix_displays_correctly() {
        let angle = std::f32::consts::FRAC_PI_6; // 30°
        let c = angle.cos();
        let s = angle.sin();
        // Game matrix with positive m[3] (front flip in Y-down game)
        let game_m: [f32; 9] = [c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0];
        let display_m = flip_y(&game_m);
        let pitch = pitch_from_matrix(&display_m);
        // After Y-flip, m[3] is negated → pitch should be negative (nose-down = front flip)
        assert!(
            pitch < 0.0,
            "Front flip (positive game m[3]) after Y-flip must give negative display pitch, got {}",
            pitch
        );
    }

    /// Back flip: negative game m[3] → after Y-flip → positive pitch (nose-up).
    #[test]
    fn back_flip_game_matrix_displays_correctly() {
        let angle = std::f32::consts::FRAC_PI_6;
        let c = angle.cos();
        let s = angle.sin();
        // Game matrix with negative m[3] (back flip in Y-down game)
        let game_m: [f32; 9] = [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0];
        let display_m = flip_y(&game_m);
        let pitch = pitch_from_matrix(&display_m);
        assert!(
            pitch > 0.0,
            "Back flip (negative game m[3]) after Y-flip must give positive display pitch, got {}",
            pitch
        );
    }

    /// Rotate helper: basic 3D point rotation.
    #[test]
    fn rotate_identity_preserves_point() {
        let identity: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let p = [1.0, 2.0, 3.0];
        let r = rotate(&identity, p);
        assert!((r[0] - 1.0).abs() < 1e-6);
        assert!((r[1] - 2.0).abs() < 1e-6);
        assert!((r[2] - 3.0).abs() < 1e-6);
    }
}
