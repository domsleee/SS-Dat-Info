//! In-memory snapshots of a take (input log + coordinates) used by history and
//! recovery, and the compact persisted form the history store writes.

use serde::{Deserialize, Serialize};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

/// Snapshot of input log + coords for history restore operations.
/// Pre-allocates max-size buffers to avoid per-push heap allocation.
#[derive(Clone)]
pub struct RecordingSnapshot {
    pub recorded_count: u32,
    pub(crate) input_log: Box<[u8; TAS_MAX_TICKS]>,
    pub(crate) rec_coords: Box<[[f32; 3]; TAS_MAX_TICKS]>,
}

impl RecordingSnapshot {
    pub fn from_state(state: &TasSharedState) -> Self {
        let mut snap = Self::new_empty();
        snap.capture_from(state);
        snap
    }

    /// Create an empty pre-allocated snapshot.
    pub(super) fn new_empty() -> Self {
        Self {
            recorded_count: 0,
            input_log: vec![0u8; TAS_MAX_TICKS]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            rec_coords: vec![[0.0f32; 3]; TAS_MAX_TICKS]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        }
    }

    /// Capture state into this (already-allocated) snapshot. No new allocations.
    /// `recorded_count` is clamped to `TAS_MAX_TICKS`: the source arrays are
    /// fixed-size, so a corrupt/over-range count from shared memory must never
    /// panic-slice this hot path (history + recovery snapshotting).
    fn capture_from(&mut self, state: &TasSharedState) {
        let count = (state.recorded_count as usize).min(TAS_MAX_TICKS);
        self.recorded_count = count as u32;
        self.input_log[..count].copy_from_slice(&state.input_log[..count]);
        self.rec_coords[..count].copy_from_slice(&state.rec_coords[..count]);
    }

    pub fn restore_to(&self, state: &mut TasSharedState) {
        let count = self.recorded_count as usize;
        state.recorded_count = self.recorded_count;
        state.input_log[..count].copy_from_slice(&self.input_log[..count]);
        for i in count..TAS_MAX_TICKS {
            state.input_log[i] = 0;
        }
        state.rec_coords[..count].copy_from_slice(&self.rec_coords[..count]);
    }

    pub(super) fn to_persisted(&self) -> PersistedSnapshot {
        let count = self.recorded_count as usize;
        PersistedSnapshot {
            recorded_count: self.recorded_count,
            input_log: self.input_log[..count].to_vec(),
            rec_coords: self.rec_coords[..count].to_vec(),
        }
    }

    pub(super) fn from_persisted(persisted: PersistedSnapshot) -> Result<Self, String> {
        persisted.validate()?;
        let count = persisted.recorded_count as usize;
        let mut snap = RecordingSnapshot::new_empty();
        snap.recorded_count = persisted.recorded_count;
        snap.input_log[..count].copy_from_slice(&persisted.input_log);
        snap.rec_coords[..count].copy_from_slice(&persisted.rec_coords);
        Ok(snap)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct PersistedSnapshot {
    pub recorded_count: u32,
    pub input_log: Vec<u8>,
    pub rec_coords: Vec<[f32; 3]>,
}

impl PersistedSnapshot {
    /// Validate the structural invariants a snapshot blob must satisfy before
    /// it can be restored. The v2 store calls this at load so a blob that
    /// deserializes but is semantically bogus (e.g. tampered/forged
    /// `recorded_count`) is treated as corrupt — NOT silently demoted to a
    /// marker by the bridge's `from_persisted(..).ok()`.
    pub fn validate(&self) -> Result<(), String> {
        let count = self.recorded_count as usize;
        if count > TAS_MAX_TICKS {
            return Err(format!(
                "persisted snapshot too large: {} ticks (max {})",
                count, TAS_MAX_TICKS
            ));
        }
        if self.input_log.len() != count {
            return Err(format!(
                "persisted input_log length mismatch: expected {}, got {}",
                count,
                self.input_log.len()
            ));
        }
        if self.rec_coords.len() != count {
            return Err(format!(
                "persisted rec_coords length mismatch: expected {}, got {}",
                count,
                self.rec_coords.len()
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_from_state_and_restore() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        state.rec_coords[0] = [10.0, 20.0, 30.0];
        state.rec_coords[1] = [40.0, 50.0, 60.0];

        let mut snap = RecordingSnapshot::new_empty();
        snap.capture_from(&state);
        assert_eq!(snap.recorded_count, 2);
        assert_eq!(snap.input_log[0], 0x01);
        assert_eq!(snap.input_log[1], 0x02);

        // Modify state
        state.recorded_count = 0;
        state.input_log[0] = 0xFF;

        // Restore
        snap.restore_to(&mut state);
        assert_eq!(state.recorded_count, 2);
        assert_eq!(state.input_log[0], 0x01);
        assert_eq!(state.input_log[1], 0x02);
        assert_eq!(state.rec_coords[0], [10.0, 20.0, 30.0]);
    }

    #[test]
    fn snapshot_restore_clears_trailing_data() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 5;
        for i in 0..5 {
            state.input_log[i] = 0xFF;
        }

        // Snapshot with 2 entries
        state.recorded_count = 2;
        state.input_log[0] = 0x01;
        state.input_log[1] = 0x02;
        let mut snap = RecordingSnapshot::new_empty();
        snap.capture_from(&state);

        // Set state to have trailing data
        state.recorded_count = 5;
        state.input_log[2] = 0xAA;
        state.input_log[3] = 0xBB;
        state.input_log[4] = 0xCC;

        // Restore should clear beyond count
        snap.restore_to(&mut state);
        assert_eq!(state.recorded_count, 2);
        assert_eq!(state.input_log[2], 0);
        assert_eq!(state.input_log[3], 0);
        assert_eq!(state.input_log[4], 0);
    }

    /// A `recorded_count` beyond the fixed buffer size (corrupt shared
    /// memory / a misbehaving DLL) must clamp, not panic-slice the hot path.
    #[test]
    fn from_state_clamps_overlong_recorded_count() {
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = TAS_MAX_TICKS as u32 + 50;
        let snap = RecordingSnapshot::from_state(&state);
        assert_eq!(
            snap.recorded_count as usize, TAS_MAX_TICKS,
            "clamped to max"
        );
    }
}
