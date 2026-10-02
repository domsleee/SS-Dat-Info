//! Test helpers shared by the recording modules' unit tests.

use std::sync::atomic::{AtomicU32, Ordering};
use tas_shared::{TasSharedState, TAS_MAX_TICKS};

static REC_COUNTER: AtomicU32 = AtomicU32::new(0);

pub(super) fn unique_temp_path(prefix: &str, ext: &str) -> std::path::PathBuf {
    let id = REC_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{}_{}_{}.{}", prefix, std::process::id(), id, ext))
}

pub(super) fn unique_temp_root(prefix: &str) -> std::path::PathBuf {
    let id = REC_COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), id));
    std::fs::create_dir_all(&root).unwrap();
    root
}

pub(super) fn one_tick_state(mask: u8) -> Box<TasSharedState> {
    let mut state = tas_shared::zeroed_boxed();
    state.recorded_count = 1;
    state.input_log[0] = mask;
    state.rec_coords[0] = [mask as f32, 0.0, mask as f32];
    state
}

/// Helper: create state with N recorded ticks and distinct input per tick.
pub(super) fn state_with_ticks(n: u32) -> Box<TasSharedState> {
    let mut state = tas_shared::zeroed_boxed();
    state.recorded_count = n;
    for i in 0..(n as usize).min(TAS_MAX_TICKS) {
        state.input_log[i] = (i + 1) as u8;
        state.rec_coords[i] = [i as f32, 0.0, i as f32 * 0.5];
    }
    state
}
