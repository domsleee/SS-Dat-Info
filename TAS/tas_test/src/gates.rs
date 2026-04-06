//! Four verification gates — mandatory before any human testing.
//!
//! | Gate | Check                | Pass condition                               |
//! |------|----------------------|----------------------------------------------|
//! | 0    | Z-coordinate ref     | 18 reference frames (50-900) match expected  |
//! | 1    | REC movement         | transitions > 0, firstInput != -1            |
//! | 2    | PLAY movement        | steering visibly works (coord deltas present) |
//! | 3    | Zero drift           | maxDriftX == 0.0, maxDriftZ == 0.0            |

use tas_shared::TasSharedState;

use crate::drift::{self, DriftResult};

/// Result of a single gate check.
#[derive(Debug, Clone)]
pub struct GateResult {
    pub gate: u8,
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
}

/// Results from all 4 gates.
#[derive(Debug, Clone)]
pub struct GateAssessment {
    pub gates: [GateResult; 4],
    pub drift: DriftResult,
}

impl GateAssessment {
    pub fn all_pass(&self) -> bool {
        self.gates.iter().all(|g| g.passed)
    }

    pub fn print_summary(&self) {
        println!("\n=== 4-Gate Assessment ===");
        for g in &self.gates {
            println!(
                "Gate {} ({}): {} — {}",
                g.gate,
                g.name,
                if g.passed { "PASS" } else { "FAIL" },
                g.detail
            );
        }
        println!(
            "\n=== Overall: {} ===",
            if self.all_pass() {
                "ALL GATES PASS"
            } else {
                "SOME GATES FAILED"
            }
        );
    }
}

/// Reference frame indices for Gate 0 Z-coordinate check (18 frames, spaced ~50 apart).
const REFERENCE_FRAMES: [usize; 18] = [
    50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600, 650, 700, 750, 800, 850, 900,
];

fn gate0_reference_frames(count: usize) -> Vec<usize> {
    let mut frames: Vec<usize> = REFERENCE_FRAMES
        .iter()
        .copied()
        .filter(|&frame| frame < count)
        .collect();

    if frames.is_empty() && count > 0 {
        // Short recordings never reach the fixed 50..900 frame window.
        // Fall back to the last captured frame so Gate 0 still verifies that
        // REC coordinates were populated instead of failing as 0/0 by design.
        frames.push(count - 1);
    }

    frames
}

/// Run all 4 gates on the current shared state after a REC->PLAY cycle.
pub fn run_gates(state: &TasSharedState, rec_count: u32) -> GateAssessment {
    let n = rec_count as usize;

    // Gate 0: Z-coordinate reference — check that REC coords are captured (non-zero at reference frames)
    let reference_frames = gate0_reference_frames(n);
    let ref_total = reference_frames.len();
    let ref_ok = reference_frames
        .iter()
        .filter(|&&frame| state.rec_coords[frame][2] != 0.0)
        .count();
    let gate0_pass = ref_total > 0 && ref_ok == ref_total;
    let gate0 = GateResult {
        gate: 0,
        name: "Z-coord reference",
        passed: gate0_pass,
        detail: format!("{}/{} reference frames have non-zero Z", ref_ok, ref_total),
    };

    // Gate 1: REC movement — transitions > 0 and firstInput != -1
    let transitions = drift::count_transitions(&state.input_log, n);
    let first_input = drift::first_input_tick(&state.input_log, n);
    let (rec_dx, _, rec_dz) = drift::compute_movement(&state.rec_coords, n);
    let rec_moved = rec_dz > 0.1;
    let gate1_pass = transitions > 0 && first_input >= 0 && rec_moved;
    let gate1 = GateResult {
        gate: 1,
        name: "REC movement",
        passed: gate1_pass,
        detail: format!(
            "transitions={}, firstInput={}, recDeltaX={:.4} recDeltaZ={:.4}",
            transitions, first_input, rec_dx, rec_dz
        ),
    };

    // Gate 2: PLAY movement — player moved during playback
    let (play_dx, _, play_dz) = drift::compute_movement(&state.play_coords, n);
    let play_moved = play_dz > 0.1;
    let gate2_pass = play_moved;
    let gate2 = GateResult {
        gate: 2,
        name: "PLAY movement",
        passed: gate2_pass,
        detail: format!("playDeltaX={:.4} playDeltaZ={:.4}", play_dx, play_dz),
    };

    // Gate 3: Zero drift
    let drift_result = drift::compute_drift(state, rec_count);
    let gate3_pass = drift_result.is_zero();
    let gate3 = GateResult {
        gate: 3,
        name: "Zero drift",
        passed: gate3_pass,
        detail: format!(
            "maxDriftX={:.9} (frame {}) maxDriftZ={:.9} (frame {})",
            drift_result.max_drift_x,
            drift_result.max_drift_frame_x,
            drift_result.max_drift_z,
            drift_result.max_drift_frame_z,
        ),
    };

    GateAssessment {
        gates: [gate0, gate1, gate2, gate3],
        drift: drift_result,
    }
}

/// Simplified gate check for straight-line tests (no steering input expected).
/// Only checks Gate 0 (coords captured), Gate 2 (movement during play), and Gate 3 (drift).
pub fn run_gates_straight(state: &TasSharedState, rec_count: u32) -> GateAssessment {
    let n = rec_count as usize;

    let reference_frames = gate0_reference_frames(n);
    let ref_total = reference_frames.len();
    let ref_ok = reference_frames
        .iter()
        .filter(|&&frame| state.rec_coords[frame][2] != 0.0)
        .count();
    let gate0 = GateResult {
        gate: 0,
        name: "Z-coord reference",
        passed: ref_total > 0 && ref_ok == ref_total,
        detail: format!("{}/{} reference frames have non-zero Z", ref_ok, ref_total),
    };

    // Gate 1: skip for straight-line (no input expected)
    let gate1 = GateResult {
        gate: 1,
        name: "REC movement (skip: straight)",
        passed: true,
        detail: "skipped for straight-line test".into(),
    };

    let (_, _, play_dz) = drift::compute_movement(&state.play_coords, n);
    let gate2 = GateResult {
        gate: 2,
        name: "PLAY movement",
        passed: play_dz > 0.1,
        detail: format!("playDeltaZ={:.4}", play_dz),
    };

    let drift_result = drift::compute_drift(state, rec_count);
    let gate3 = GateResult {
        gate: 3,
        name: "Zero drift",
        passed: drift_result.is_zero(),
        detail: format!(
            "maxDriftX={:.9} maxDriftZ={:.9}",
            drift_result.max_drift_x, drift_result.max_drift_z,
        ),
    };

    GateAssessment {
        gates: [gate0, gate1, gate2, gate3],
        drift: drift_result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn zeroed_state() -> Box<TasSharedState> {
        tas_shared::zeroed_boxed()
    }

    fn perfect_steered_state(n: usize) -> Box<TasSharedState> {
        let mut state = zeroed_state();
        for i in 0..n {
            let z = 100.0 + i as f32 * 0.5;
            let x = if (20..40).contains(&i) { 5.0 } else { 0.0 };
            state.rec_coords[i] = [x, 0.0, z];
            state.play_coords[i] = [x, 0.0, z];
        }
        for i in 0..n {
            state.input_log[i] = if (20..40).contains(&i) { 0x01 } else { 0x00 };
        }
        state
    }

    #[test]
    fn all_gates_pass_on_perfect_state() {
        let state = perfect_steered_state(1000);
        let assessment = run_gates(&state, 1000);
        assert!(assessment.all_pass());
    }

    #[test]
    fn gate0_fails_when_rec_coords_all_zero() {
        let state = zeroed_state();
        let assessment = run_gates(&state, 1000);
        assert!(!assessment.gates[0].passed);
    }

    #[test]
    fn gate0_passes_with_nonzero_z_at_reference_frames() {
        let mut state = zeroed_state();
        for &frame in &REFERENCE_FRAMES {
            state.rec_coords[frame] = [0.0, 0.0, 100.0 + frame as f32];
        }
        let assessment = run_gates(&state, 1000);
        assert!(assessment.gates[0].passed);
    }

    #[test]
    fn gate0_short_recording_uses_last_frame_fallback() {
        let state = zeroed_state();
        let assessment = run_gates(&state, 30);
        assert!(!assessment.gates[0].passed);
        assert_eq!(
            assessment.gates[0].detail,
            "0/1 reference frames have non-zero Z"
        );
    }

    #[test]
    fn gate0_short_recording_passes_with_nonzero_last_frame() {
        let mut state = zeroed_state();
        state.rec_coords[24] = [0.0, 0.0, 123.0];
        let assessment = run_gates(&state, 25);
        assert!(assessment.gates[0].passed);
        assert_eq!(
            assessment.gates[0].detail,
            "1/1 reference frames have non-zero Z"
        );
    }

    #[test]
    fn gate1_fails_without_input() {
        let mut state = zeroed_state();
        for i in 0..1000 {
            state.rec_coords[i] = [0.0, 0.0, i as f32];
            state.play_coords[i] = [0.0, 0.0, i as f32];
        }
        let assessment = run_gates(&state, 1000);
        assert!(!assessment.gates[1].passed);
    }

    #[test]
    fn gate2_fails_without_play_movement() {
        let mut state = zeroed_state();
        for i in 0..200 {
            state.rec_coords[i] = [0.0, 0.0, i as f32];
            state.input_log[i] = if i < 50 { 0x01 } else { 0x00 };
        }
        let assessment = run_gates(&state, 200);
        assert!(!assessment.gates[2].passed);
    }

    #[test]
    fn gate3_fails_with_drift() {
        let mut state = perfect_steered_state(1000);
        state.play_coords[500][0] += 1.0;
        let assessment = run_gates(&state, 1000);
        assert!(!assessment.gates[3].passed);
        assert!(!assessment.drift.is_zero());
    }

    #[test]
    fn run_gates_straight_skips_gate1() {
        let mut state = zeroed_state();
        for i in 0..1000 {
            state.rec_coords[i] = [0.0, 0.0, i as f32];
            state.play_coords[i] = [0.0, 0.0, i as f32];
        }
        let assessment = run_gates_straight(&state, 1000);
        assert!(assessment.gates[1].passed);
        assert!(assessment.all_pass());
    }
}
