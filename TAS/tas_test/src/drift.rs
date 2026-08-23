//! Drift calculation utilities.
//!
//! Computes max absolute drift between REC and PLAY coordinate logs.

use tas_shared::TasSharedState;

/// Per-axis drift result.
///
/// Y (vertical) is compared alongside X/Z. It was historically omitted, which
/// left every drift-based mode blind to a regression that changed height while
/// preserving the ground track — jump arcs and terrain following are exactly
/// that shape, so a "zero drift" verdict used to be able to hide them.
#[derive(Debug, Clone, Default)]
pub struct DriftResult {
    pub max_drift_x: f64,
    pub max_drift_y: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_y: usize,
    pub max_drift_frame_z: usize,
}

impl DriftResult {
    pub fn is_zero(&self) -> bool {
        self.max_drift_x == 0.0 && self.max_drift_y == 0.0 && self.max_drift_z == 0.0
    }

    pub fn is_within(&self, epsilon: f64) -> bool {
        self.max_drift_x < epsilon && self.max_drift_y < epsilon && self.max_drift_z < epsilon
    }

    /// Largest drift across all three axes — for one-line reporting.
    pub fn max_axis(&self) -> f64 {
        self.max_drift_x.max(self.max_drift_y).max(self.max_drift_z)
    }
}

fn compute_drift_between(
    rec_coords: &[[f32; 3]],
    play_coords: &[[f32; 3]],
    rec_start: usize,
    play_start: usize,
    count: usize,
    normalize_to_start: bool,
) -> DriftResult {
    let mut result = DriftResult::default();
    let count = count
        .min(rec_coords.len().saturating_sub(rec_start))
        .min(play_coords.len().saturating_sub(play_start));
    if count == 0 {
        return result;
    }

    let rec_origin = rec_coords[rec_start];
    let play_origin = play_coords[play_start];

    for offset in 0..count {
        let rec_i = rec_start + offset;
        let play_i = play_start + offset;
        let rec_x = if normalize_to_start {
            rec_coords[rec_i][0] - rec_origin[0]
        } else {
            rec_coords[rec_i][0]
        };
        let rec_y = if normalize_to_start {
            rec_coords[rec_i][1] - rec_origin[1]
        } else {
            rec_coords[rec_i][1]
        };
        let rec_z = if normalize_to_start {
            rec_coords[rec_i][2] - rec_origin[2]
        } else {
            rec_coords[rec_i][2]
        };
        let play_x = if normalize_to_start {
            play_coords[play_i][0] - play_origin[0]
        } else {
            play_coords[play_i][0]
        };
        let play_y = if normalize_to_start {
            play_coords[play_i][1] - play_origin[1]
        } else {
            play_coords[play_i][1]
        };
        let play_z = if normalize_to_start {
            play_coords[play_i][2] - play_origin[2]
        } else {
            play_coords[play_i][2]
        };

        let dx = (rec_x as f64 - play_x as f64).abs();
        let dy = (rec_y as f64 - play_y as f64).abs();
        let dz = (rec_z as f64 - play_z as f64).abs();

        // NaN must never read as "no drift". Every comparison against NaN is
        // false, so a NaN delta would slide through the `>` tests below and leave
        // the maxima at 0.0 — i.e. `is_zero()` would report a clean run for a
        // simulation that had gone non-finite. Force it to infinity so it fails
        // every gate loudly instead.
        let sanitize = |d: f64| if d.is_nan() { f64::INFINITY } else { d };
        let (dx, dy, dz) = (sanitize(dx), sanitize(dy), sanitize(dz));

        if dx > result.max_drift_x {
            result.max_drift_x = dx;
            result.max_drift_frame_x = rec_i;
        }
        if dy > result.max_drift_y {
            result.max_drift_y = dy;
            result.max_drift_frame_y = rec_i;
        }
        if dz > result.max_drift_z {
            result.max_drift_z = dz;
            result.max_drift_frame_z = rec_i;
        }
    }

    result
}

/// Compute drift between rec_coords and play_coords over `count` ticks.
pub fn compute_drift(state: &TasSharedState, count: u32) -> DriftResult {
    compute_drift_between(
        &state.rec_coords,
        &state.play_coords,
        0,
        0,
        count as usize,
        false,
    )
}

/// Compute drift between rec_coords and play_coords over the window `[start, count)`.
/// Gate-relative drift: rec_coords from rec_gate, play_coords from play_gate,
/// for `count` samples. When the two gates differ (the countdown landed on a
/// different tick), raw-index drift is meaningless but this is exactly zero on
/// a correct aligned replay.
pub fn compute_drift_gate_relative(
    state: &TasSharedState,
    rec_gate: u32,
    play_gate: u32,
    count: u32,
) -> DriftResult {
    compute_drift_between(
        &state.rec_coords,
        &state.play_coords,
        rec_gate as usize,
        play_gate as usize,
        count as usize,
        false,
    )
}

pub fn compute_drift_window(state: &TasSharedState, start: u32, count: u32) -> DriftResult {
    let end = count as usize;
    let start = (start as usize).min(end);
    compute_drift_between(
        &state.rec_coords,
        &state.play_coords,
        start,
        start,
        end - start,
        false,
    )
}

/// Compute drift between rec_coords and play_coords over the window `[start, count)`,
/// after normalizing both traces to the coordinate at `start`.
pub fn compute_normalized_drift_window(
    state: &TasSharedState,
    start: u32,
    count: u32,
) -> DriftResult {
    let end = count as usize;
    let start = (start as usize).min(end);
    compute_drift_between(
        &state.rec_coords,
        &state.play_coords,
        start,
        start,
        end - start,
        true,
    )
}

/// Compare recording and playback at equal offsets from their independently
/// observed gates. Drift frame indices are reported in recording coordinates.
pub fn compute_gate_relative_drift(
    state: &TasSharedState,
    rec_gate: u32,
    play_gate: u32,
    count: u32,
) -> DriftResult {
    compute_drift_between(
        &state.rec_coords,
        &state.play_coords,
        rec_gate as usize,
        play_gate as usize,
        count as usize,
        false,
    )
}

/// Compute max coordinate delta (movement) for a single coord log.
pub fn compute_movement(coords: &[[f32; 3]], count: usize) -> (f64, f64, f64) {
    if count < 2 {
        return (0.0, 0.0, 0.0);
    }
    let mut max_dx: f64 = 0.0;
    let mut max_dy: f64 = 0.0;
    let mut max_dz: f64 = 0.0;

    for i in 0..count {
        let dx = (coords[i][0] as f64 - coords[0][0] as f64).abs();
        let dy = (coords[i][1] as f64 - coords[0][1] as f64).abs();
        let dz = (coords[i][2] as f64 - coords[0][2] as f64).abs();
        if dx > max_dx {
            max_dx = dx;
        }
        if dy > max_dy {
            max_dy = dy;
        }
        if dz > max_dz {
            max_dz = dz;
        }
    }
    (max_dx, max_dy, max_dz)
}

/// Count input transitions (mask changes) in the input log.
pub fn count_transitions(input_log: &[u8], count: usize) -> u32 {
    if count < 2 {
        return 0;
    }
    let mut transitions = 0u32;
    for i in 1..count {
        if input_log[i] != input_log[i - 1] {
            transitions += 1;
        }
    }
    transitions
}

/// Find the first tick where input is non-zero (-1 if none).
pub fn first_input_tick(input_log: &[u8], count: usize) -> i32 {
    for (i, &byte) in input_log[..count].iter().enumerate() {
        if byte != 0 {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    // ========== compute_drift ==========

    #[test]
    fn drift_zero_when_coords_identical() {
        let mut state = zeroed_state();
        for i in 0..100 {
            state.rec_coords[i] = [1.0, 2.0, 3.0 + i as f32];
            state.play_coords[i] = [1.0, 2.0, 3.0 + i as f32];
        }
        let d = compute_drift(&state, 100);
        assert!(d.is_zero());
        assert_eq!(d.max_drift_frame_x, 0);
        assert_eq!(d.max_drift_frame_z, 0);
    }

    #[test]
    fn drift_detects_x_difference() {
        let mut state = zeroed_state();
        for i in 0..50 {
            state.rec_coords[i] = [10.0, 0.0, 0.0];
            state.play_coords[i] = [10.0, 0.0, 0.0];
        }
        // Introduce drift at frame 30
        state.play_coords[30][0] = 15.0;

        let d = compute_drift(&state, 50);
        assert!(!d.is_zero());
        assert_eq!(d.max_drift_x, 5.0);
        assert_eq!(d.max_drift_frame_x, 30);
        assert_eq!(d.max_drift_z, 0.0);
    }

    #[test]
    fn drift_detects_z_difference() {
        let mut state = zeroed_state();
        state.rec_coords[10] = [0.0, 0.0, 100.0];
        state.play_coords[10] = [0.0, 0.0, 100.5];

        let d = compute_drift(&state, 20);
        assert_eq!(d.max_drift_z as f32, 0.5);
        assert_eq!(d.max_drift_frame_z, 10);
    }

    #[test]
    fn drift_count_zero() {
        let state = zeroed_state();
        let d = compute_drift(&state, 0);
        assert!(d.is_zero());
    }

    #[test]
    fn drift_is_within() {
        let d = DriftResult {
            max_drift_x: 0.001,
            max_drift_y: 0.0015,
            max_drift_z: 0.002,
            ..Default::default()
        };
        assert!(d.is_within(0.01));
        assert!(!d.is_within(0.001));
    }

    // ========== Y axis (vertical) ==========

    #[test]
    fn drift_detects_y_difference() {
        let mut state = zeroed_state();
        for i in 0..50 {
            state.rec_coords[i] = [10.0, 5.0, 20.0];
            state.play_coords[i] = [10.0, 5.0, 20.0];
        }
        // A pure height change: ground track (X/Z) is untouched.
        state.play_coords[30][1] = 8.0;

        let d = compute_drift(&state, 50);
        assert!(
            !d.is_zero(),
            "a vertical-only divergence must not report zero drift"
        );
        assert_eq!(d.max_drift_y, 3.0);
        assert_eq!(d.max_drift_frame_y, 30);
        // Ground track really is identical — this is the case the old X/Z-only
        // oracle silently passed.
        assert_eq!(d.max_drift_x, 0.0);
        assert_eq!(d.max_drift_z, 0.0);
    }

    #[test]
    fn drift_is_within_catches_y_alone() {
        let d = DriftResult {
            max_drift_y: 0.5,
            ..Default::default()
        };
        assert!(!d.is_within(0.01));
        assert!(!d.is_zero());
    }

    #[test]
    fn normalized_drift_window_cancels_constant_y_offset() {
        let mut state = zeroed_state();
        for i in 0..6 {
            state.rec_coords[i] = [0.0, 100.0 + i as f32, 0.0];
            // Same vertical *shape*, shifted by a constant 40 units.
            state.play_coords[i] = [0.0, 140.0 + i as f32, 0.0];
        }
        let raw = compute_drift_window(&state, 2, 6);
        let normalized = compute_normalized_drift_window(&state, 2, 6);
        assert_eq!(raw.max_drift_y, 40.0);
        assert!(normalized.is_zero());
    }

    #[test]
    fn nan_coordinate_is_not_zero_drift() {
        let mut state = zeroed_state();
        for i in 0..20 {
            state.rec_coords[i] = [1.0, 2.0, 3.0];
            state.play_coords[i] = [1.0, 2.0, 3.0];
        }
        // A non-finite simulation must fail, not report a clean run. Every
        // comparison against NaN is false, so without the guard the maxima stay
        // 0.0 and is_zero() returns true.
        state.play_coords[7][1] = f32::NAN;

        let d = compute_drift(&state, 20);
        assert!(!d.is_zero(), "NaN must not read as zero drift");
        assert!(!d.is_within(f64::MAX));
        assert!(d.max_drift_y.is_infinite());
    }

    #[test]
    fn max_axis_reports_largest_of_three() {
        let d = DriftResult {
            max_drift_x: 1.0,
            max_drift_y: 7.0,
            max_drift_z: 3.0,
            ..Default::default()
        };
        assert_eq!(d.max_axis(), 7.0);
    }

    #[test]
    fn drift_window_skips_prefix_before_start() {
        let mut state = zeroed_state();
        state.rec_coords[0] = [10.0, 0.0, 20.0];
        state.play_coords[0] = [999.0, 0.0, 999.0];
        for i in 1..5 {
            state.rec_coords[i] = [i as f32, 0.0, (i * 10) as f32];
            state.play_coords[i] = state.rec_coords[i];
        }

        let d = compute_drift_window(&state, 1, 5);
        assert!(d.is_zero());
    }

    #[test]
    fn normalized_drift_window_cancels_constant_offset_after_start() {
        let mut state = zeroed_state();
        for i in 0..6 {
            state.rec_coords[i] = [100.0 + i as f32, 0.0, 200.0 + (i * 2) as f32];
            state.play_coords[i] = [0.0, 0.0, 0.0];
        }
        for i in 2..6 {
            state.play_coords[i] = [
                state.rec_coords[i][0] + 5.0,
                0.0,
                state.rec_coords[i][2] - 7.0,
            ];
        }

        let raw = compute_drift_window(&state, 2, 6);
        let normalized = compute_normalized_drift_window(&state, 2, 6);
        assert_eq!(raw.max_drift_x, 5.0);
        assert_eq!(raw.max_drift_z, 7.0);
        assert!(normalized.is_zero());
    }

    #[test]
    fn normalized_drift_window_keeps_shape_mismatch() {
        let mut state = zeroed_state();
        for i in 0..5 {
            state.rec_coords[i] = [i as f32, 0.0, (i * 10) as f32];
            state.play_coords[i] = state.rec_coords[i];
        }
        state.play_coords[4][0] += 3.0;
        state.play_coords[4][2] -= 4.0;

        let d = compute_normalized_drift_window(&state, 1, 5);
        assert_eq!(d.max_drift_x, 3.0);
        assert_eq!(d.max_drift_z, 4.0);
    }

    #[test]
    fn gate_relative_drift_compares_equal_offsets_not_equal_indices() {
        let mut state = zeroed_state();
        let rec_gate = 3usize;
        let play_gate = 5usize;
        for offset in 0..6 {
            let coord = [10.0 + offset as f32, 20.0, 30.0 + offset as f32];
            state.rec_coords[rec_gate + offset] = coord;
            state.play_coords[play_gate + offset] = coord;
        }
        assert!(!compute_drift(&state, 11).is_zero());
        assert!(
            compute_gate_relative_drift(&state, rec_gate as u32, play_gate as u32, 6).is_zero()
        );

        state.play_coords[play_gate + 4][1] += 0.25;
        let d = compute_gate_relative_drift(&state, rec_gate as u32, play_gate as u32, 6);
        assert_eq!(d.max_drift_y, 0.25);
        assert_eq!(d.max_drift_frame_y, rec_gate + 4);
    }

    // ========== compute_movement ==========

    #[test]
    fn movement_zero_with_single_coord() {
        let coords = [[1.0, 2.0, 3.0]];
        let (dx, dy, dz) = compute_movement(&coords, 1);
        assert_eq!(dx, 0.0);
        assert_eq!(dy, 0.0);
        assert_eq!(dz, 0.0);
    }

    #[test]
    fn movement_zero_with_empty() {
        let coords: &[[f32; 3]] = &[];
        let (dx, dy, dz) = compute_movement(coords, 0);
        assert_eq!(dx, 0.0);
        assert_eq!(dy, 0.0);
        assert_eq!(dz, 0.0);
    }

    #[test]
    fn movement_measures_from_first_coord() {
        let coords = [[0.0, 0.0, 0.0], [5.0, 3.0, 10.0], [2.0, 1.0, 20.0]];
        let (dx, dy, dz) = compute_movement(&coords, 3);
        assert_eq!(dx, 5.0);
        assert_eq!(dy, 3.0);
        assert_eq!(dz, 20.0);
    }

    // ========== count_transitions ==========

    #[test]
    fn transitions_empty() {
        assert_eq!(count_transitions(&[], 0), 0);
    }

    #[test]
    fn transitions_single() {
        assert_eq!(count_transitions(&[0x01], 1), 0);
    }

    #[test]
    fn transitions_no_change() {
        assert_eq!(count_transitions(&[0x01, 0x01, 0x01], 3), 0);
    }

    #[test]
    fn transitions_every_tick() {
        // 0→1→0→1 = 3 transitions
        assert_eq!(count_transitions(&[0, 1, 0, 1], 4), 3);
    }

    #[test]
    fn transitions_typical_lr() {
        // L held 3 ticks, then R held 3 ticks = 1 transition
        let log = [0x01, 0x01, 0x01, 0x02, 0x02, 0x02];
        assert_eq!(count_transitions(&log, 6), 1);
    }

    // ========== first_input_tick ==========

    #[test]
    fn first_input_none() {
        assert_eq!(first_input_tick(&[0, 0, 0], 3), -1);
    }

    #[test]
    fn first_input_empty() {
        assert_eq!(first_input_tick(&[], 0), -1);
    }

    #[test]
    fn first_input_immediate() {
        assert_eq!(first_input_tick(&[0x01, 0, 0], 3), 0);
    }

    #[test]
    fn first_input_delayed() {
        assert_eq!(first_input_tick(&[0, 0, 0, 0x04, 0x04], 5), 3);
    }
}
