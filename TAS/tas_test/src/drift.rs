//! Drift calculation utilities.
//!
//! Computes max absolute drift between REC and PLAY coordinate logs.

use tas_shared::TasSharedState;

/// Per-axis drift result.
#[derive(Debug, Clone, Default)]
pub struct DriftResult {
    pub max_drift_x: f64,
    pub max_drift_z: f64,
    pub max_drift_frame_x: usize,
    pub max_drift_frame_z: usize,
}

impl DriftResult {
    pub fn is_zero(&self) -> bool {
        self.max_drift_x == 0.0 && self.max_drift_z == 0.0
    }

    pub fn is_within(&self, epsilon: f64) -> bool {
        self.max_drift_x < epsilon && self.max_drift_z < epsilon
    }
}

/// Compute drift between rec_coords and play_coords over `count` ticks.
pub fn compute_drift(state: &TasSharedState, count: u32) -> DriftResult {
    let mut result = DriftResult::default();
    let n = count as usize;

    for i in 0..n {
        let dx = (state.rec_coords[i][0] as f64 - state.play_coords[i][0] as f64).abs();
        let dz = (state.rec_coords[i][2] as f64 - state.play_coords[i][2] as f64).abs();

        if dx > result.max_drift_x {
            result.max_drift_x = dx;
            result.max_drift_frame_x = i;
        }
        if dz > result.max_drift_z {
            result.max_drift_z = dz;
            result.max_drift_frame_z = i;
        }
    }
    result
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
        if dx > max_dx { max_dx = dx; }
        if dy > max_dy { max_dy = dy; }
        if dz > max_dz { max_dz = dz; }
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
    for i in 0..count {
        if input_log[i] != 0 {
            return i as i32;
        }
    }
    -1
}
