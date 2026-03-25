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

pub fn show(ui: &mut egui::Ui, state: &TasSharedState) {
    let m = &state.rotation_matrix;

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
    draw_wireframe_body(&painter, center, scale, m);

    // Draw rotated coordinate axes: X=red, Y=green, Z=blue
    draw_axis(&painter, center, scale, m, [1.0, 0.0, 0.0], egui::Color32::from_rgb(220, 80, 80), "X");
    draw_axis(&painter, center, scale, m, [0.0, 1.0, 0.0], egui::Color32::from_rgb(80, 200, 80), "Y");
    draw_axis(&painter, center, scale, m, [0.0, 0.0, 1.0], egui::Color32::from_rgb(80, 130, 230), "Z");

    // Center dot
    painter.circle_filled(center, 2.5, egui::Color32::WHITE);

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

    /// Pitch-up rotation (lean back) must produce a POSITIVE pitch value.
    /// This verifies the fix: m[3].asin() without negation.
    ///
    /// A rotation of +30° about the X-axis (lean back):
    ///   R = [[1, 0, 0], [0, cos30, -sin30], [0, sin30, cos30]]
    /// Row-major: [1, 0, 0,  0, cos30, -sin30,  0, sin30, cos30]
    /// m[3] = 0 for pure X-rotation, so we use a combined rotation.
    ///
    /// Instead, use a Y-axis rotation of +30° (nose up in XZ plane):
    ///   R = [[cos30, 0, sin30], [0, 1, 0], [-sin30, 0, cos30]]
    /// Row-major: [cos30, 0, sin30,  0, 1, 0,  -sin30, 0, cos30]
    /// m[3] = 0 → pitch = 0, not helpful.
    ///
    /// Use a rotation that puts sin into m[3]:
    /// A rotation about Z by θ gives R = [[cosθ, -sinθ, 0], [sinθ, cosθ, 0], [0, 0, 1]]
    /// m[3] = sinθ → pitch = asin(sinθ) = θ
    #[test]
    fn lean_back_gives_positive_pitch() {
        let angle = std::f32::consts::FRAC_PI_6; // 30°
        let c = angle.cos();
        let s = angle.sin();
        // Z-rotation: m[3] = sin(30°) = 0.5
        let m: [f32; 9] = [c, -s, 0.0, s, c, 0.0, 0.0, 0.0, 1.0];
        let pitch = pitch_from_matrix(&m);
        // pitch should be positive (~30°), not negative
        assert!(
            pitch > 0.0,
            "Lean-back (positive m[3]) must give positive pitch, got {}",
            pitch
        );
        assert!((pitch - angle).abs() < 1e-5);
    }

    /// Lean forward (negative m[3]) must produce negative pitch.
    #[test]
    fn lean_forward_gives_negative_pitch() {
        let angle = std::f32::consts::FRAC_PI_6;
        let c = angle.cos();
        let s = angle.sin();
        // Z-rotation by -30°: m[3] = sin(-30°) = -0.5
        let m: [f32; 9] = [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0];
        let pitch = pitch_from_matrix(&m);
        assert!(
            pitch < 0.0,
            "Lean-forward (negative m[3]) must give negative pitch, got {}",
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
