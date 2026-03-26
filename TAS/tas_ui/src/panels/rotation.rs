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

/// Extract Euler angles from a 3x3 row-major rotation matrix.
///
/// Matches the SS-Dat-Info reference implementation (Three.js XYZ convention):
/// 1. Extract intrinsic XYZ Euler angles from the raw game matrix
/// 2. Negate pitch (X) and yaw (Y) for game→display coordinate correction
///
/// The game uses a left-handed coordinate system where X and Y axes are
/// inverted relative to standard right-handed rendering coordinates.
/// Reference: https://domsleee.github.io/SS-Dat-Info/ (threejs/src/script.ts)

fn pitch_from_matrix(m: &[f32; 9]) -> f32 {
    // Three.js XYZ: euler.x = atan2(-m23, m33) = atan2(-m[5], m[8])
    // Negate for display correction (reference negates euler.x)
    -((-m[5]).atan2(m[8]))
}

fn yaw_from_matrix(m: &[f32; 9]) -> f32 {
    // Three.js XYZ: euler.y = asin(m13) = asin(m[2])
    // Negate for display correction (reference negates euler.y)
    -(m[2].clamp(-1.0, 1.0).asin())
}

fn roll_from_matrix(m: &[f32; 9]) -> f32 {
    // Three.js XYZ: euler.z = atan2(-m12, m11) = atan2(-m[1], m[0])
    // Roll is NOT negated in the reference
    (-m[1]).atan2(m[0])
}

/// Create a display-space rotation matrix from the game's raw matrix.
/// The game uses inverted X/Y axes relative to standard rendering.
/// This applies M_display = S * M where S = diag(-1, -1, 1),
/// matching the reference position transform: x=-x, y=-y, z=z.
fn display_matrix(m: &[f32; 9]) -> [f32; 9] {
    [
        -m[0], -m[1], -m[2],
        -m[3], -m[4], -m[5],
         m[6],  m[7],  m[8],
    ]
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

    // 3D rotation visualization using display-corrected matrix
    let dm = display_matrix(m);
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

    // Draw wireframe body showing board orientation (display-corrected)
    draw_wireframe_body(&painter, center, scale, &dm);

    // Draw rotated coordinate axes: X=red, Y=green, Z=blue (display-corrected)
    draw_axis(&painter, center, scale, &dm, [1.0, 0.0, 0.0], egui::Color32::from_rgb(220, 80, 80), "X");
    draw_axis(&painter, center, scale, &dm, [0.0, 1.0, 0.0], egui::Color32::from_rgb(80, 200, 80), "Y");
    draw_axis(&painter, center, scale, &dm, [0.0, 0.0, 1.0], egui::Color32::from_rgb(80, 130, 230), "Z");

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

    /// Identity matrix -> all Euler angles should be zero.
    #[test]
    fn identity_matrix_gives_zero_angles() {
        let identity: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        assert!((yaw_from_matrix(&identity)).abs() < 1e-6);
        assert!((pitch_from_matrix(&identity)).abs() < 1e-6);
        assert!((roll_from_matrix(&identity)).abs() < 1e-6);
    }

    /// Pure pitch rotation (X-axis, 30 deg) in left-handed convention.
    ///
    /// R_x(30 deg) left-handed:
    ///   [1   0      0   ]
    ///   [0   cos30  sin30]
    ///   [0  -sin30  cos30]
    ///
    /// XYZ extraction: euler.x = atan2(-sin30, cos30) = -30 deg
    /// After negate: pitch = +30 deg (positive = nose up in display)
    #[test]
    fn pure_pitch_30_degrees() {
        let angle = std::f32::consts::FRAC_PI_6;
        let c = angle.cos();
        let s = angle.sin();
        let m: [f32; 9] = [1.0, 0.0, 0.0, 0.0, c, s, 0.0, -s, c];
        let pitch = pitch_from_matrix(&m);
        assert!((pitch - angle).abs() < 1e-5, "Expected ~30 deg pitch, got {}", pitch.to_degrees());
        assert!((yaw_from_matrix(&m)).abs() < 1e-5);
        assert!((roll_from_matrix(&m)).abs() < 1e-5);
    }

    /// Front flip = negative pitch (nose down) in display coordinates.
    /// In the game's left-handed system, negative X rotation tilts nose down.
    ///
    /// R_x(-30 deg): m[5] = -sin30 = -0.5
    /// XYZ extraction: euler.x = atan2(0.5, cos30) = +30 deg
    /// After negate: pitch = -30 deg (negative = nose down in display)
    #[test]
    fn front_flip_gives_negative_pitch() {
        let angle = std::f32::consts::FRAC_PI_6;
        let c = angle.cos();
        let s = angle.sin();
        // R_x(-30 deg) left-handed
        let m: [f32; 9] = [1.0, 0.0, 0.0, 0.0, c, -s, 0.0, s, c];
        let pitch = pitch_from_matrix(&m);
        assert!(
            pitch < 0.0,
            "Front flip (negative X rotation) must give negative pitch, got {}",
            pitch.to_degrees()
        );
        assert!((pitch + angle).abs() < 1e-5);
    }

    /// Pure yaw rotation (Y-axis, 45 deg) in left-handed convention.
    ///
    /// R_y(45 deg) left-handed:
    ///   [cos45  0  -sin45]
    ///   [  0    1    0   ]
    ///   [sin45  0   cos45]
    ///
    /// XYZ extraction: euler.y = asin(-sin45) = -45 deg
    /// After negate: yaw = +45 deg
    #[test]
    fn pure_yaw_45_degrees() {
        let angle = std::f32::consts::FRAC_PI_4;
        let c = angle.cos();
        let s = angle.sin();
        let m: [f32; 9] = [c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c];
        let yaw = yaw_from_matrix(&m);
        assert!((yaw - angle).abs() < 1e-5, "Expected ~45 deg yaw, got {}", yaw.to_degrees());
        assert!((pitch_from_matrix(&m)).abs() < 1e-5);
        assert!((roll_from_matrix(&m)).abs() < 1e-5);
    }

    /// Pure roll rotation (Z-axis, 20 deg) in left-handed convention.
    ///
    /// R_z(20 deg) left-handed:
    ///   [cos20   sin20  0]
    ///   [-sin20  cos20  0]
    ///   [  0       0    1]
    ///
    /// XYZ extraction: euler.z = atan2(-sin20, cos20) = -20 deg
    /// Roll is NOT negated, so display shows -20 deg for left-handed +20 deg.
    #[test]
    fn pure_roll_20_degrees() {
        let angle = 20.0_f32.to_radians();
        let c = angle.cos();
        let s = angle.sin();
        let m: [f32; 9] = [c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0];
        let roll = roll_from_matrix(&m);
        // In XYZ extraction, left-handed R_z(+20) maps to display roll = -20
        assert!((roll + angle).abs() < 1e-5, "Expected ~-20 deg roll, got {}", roll.to_degrees());
        assert!((yaw_from_matrix(&m)).abs() < 1e-5);
        assert!((pitch_from_matrix(&m)).abs() < 1e-5);
    }

    /// Rotate helper: identity preserves point.
    #[test]
    fn rotate_identity_preserves_point() {
        let identity: [f32; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let p = [1.0, 2.0, 3.0];
        let r = rotate(&identity, p);
        assert!((r[0] - 1.0).abs() < 1e-6);
        assert!((r[1] - 2.0).abs() < 1e-6);
        assert!((r[2] - 3.0).abs() < 1e-6);
    }

    /// Display matrix negates X and Y rows.
    #[test]
    fn display_matrix_negates_xy() {
        let m: [f32; 9] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
        let dm = display_matrix(&m);
        assert_eq!(dm, [-1.0, -2.0, -3.0, -4.0, -5.0, -6.0, 7.0, 8.0, 9.0]);
    }

    /// Verify against SS-Dat-Info reference for real game data.
    /// FE-Decent.dat frame 1500: player mid-flip past vertical.
    #[test]
    fn matches_reference_frame_1500() {
        let m: [f32; 9] = [
            0.9973129, 0.07208475, -0.013_067_5,
            0.017486196, -0.40744606, -0.91306186,
            -0.07114214, 0.9103799, -0.4076117,
        ];
        let pitch = pitch_from_matrix(&m).to_degrees();
        let yaw = yaw_from_matrix(&m).to_degrees();
        let roll = roll_from_matrix(&m).to_degrees();
        // Reference: pitch=-114.06, yaw=0.75, roll=-4.13
        assert!((pitch - (-114.06)).abs() < 0.5, "pitch={}", pitch);
        assert!((yaw - 0.75).abs() < 0.5, "yaw={}", yaw);
        assert!((roll - (-4.13)).abs() < 0.5, "roll={}", roll);
    }

    /// Verify against SS-Dat-Info reference for another real frame.
    /// FE-Decent.dat frame 3500: player pitched forward on slope.
    #[test]
    fn matches_reference_frame_3500() {
        let m: [f32; 9] = [
            0.97618407, 0.12827694, -0.17495605,
            0.035068795, 0.70254636, 0.71077335,
            0.21409057, -0.69998115, 0.681_316_1,
        ];
        let pitch = pitch_from_matrix(&m).to_degrees();
        let yaw = yaw_from_matrix(&m).to_degrees();
        let roll = roll_from_matrix(&m).to_degrees();
        // Reference: pitch=46.21, yaw=10.08, roll=-7.49
        assert!((pitch - 46.21).abs() < 0.5, "pitch={}", pitch);
        assert!((yaw - 10.08).abs() < 0.5, "yaw={}", yaw);
        assert!((roll - (-7.49)).abs() < 0.5, "roll={}", roll);
    }
}
