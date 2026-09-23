//! Gate-alignment helpers shared by tas_ui (the live transport) and tas_test
//! (the harness), so both judge a replay on exactly the same criteria.

/// Max restarts for an aligned-trajectory mismatch.
pub const START_MATCH_MAX_RETRIES: u32 = 30;

/// First frame index where `rec_coords` first differs (bit-exact) from
/// `rec_coords[0]` — when the recorded player leaves spawn. `None` if it
/// never moves within `recorded_count` (degenerate / positions not loaded).
pub fn detect_first_moving(rec_coords: &[[f32; 3]], recorded_count: u32) -> Option<u32> {
    if recorded_count == 0 || rec_coords.is_empty() {
        return None;
    }
    let n = (recorded_count as usize).min(rec_coords.len());
    if n < 2 {
        return None;
    }
    let start = rec_coords[0];
    for (j, c) in rec_coords.iter().copied().enumerate().take(n).skip(1) {
        if c[0].to_bits() != start[0].to_bits()
            || c[1].to_bits() != start[1].to_bits()
            || c[2].to_bits() != start[2].to_bits()
        {
            return Some(j as u32);
        }
    }
    None
}

/// Verdict from watching a gate-aligned replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketVerdict {
    /// Playback hasn't progressed far enough to judge — keep polling.
    KeepWaiting,
    /// A coordinate capture failed, so the trajectory cannot be trusted.
    WrongStart,
    /// No gate-relative trajectory to watch.
    NoSignal,
    /// The replay matched the recording bit for bit.
    Match,
    /// The replay differs from the recording at gate-relative frame `observed`.
    WrongBucket { observed: Option<u32> },
}

/// Rust mirror of gate_alignment.hpp's GATE_ALIGN_PRE_GATE_LEAD - keep
/// the two in lock-step. During an aligned replay, every recorded input
/// frame in [rec_gate - LEAD, rec_gate) is REPLACED by the gate mask
/// while the live gate is pending, so the window must stay as narrow as
/// the observed gate jitter (max four cycles) allows.
pub const GATE_ALIGN_PRE_GATE_LEAD: u32 = 8;

/// How many recorded input frames inside the pre-gate hold window
/// differ from the gate mask - i.e. transitions that an aligned replay
/// will NOT reproduce at their recorded position. Non-zero means the
/// recording exercises input timing the alignment cannot honor; the
/// armers surface it as a warning so a deterministic reroll loop or a
/// late divergence is attributable instead of mysterious.
pub fn pre_gate_hold_overwrites(input_log: &[u8], rec_gate: u32) -> u32 {
    let g = rec_gate as usize;
    if g == 0 || g >= input_log.len() {
        return 0;
    }
    let mask = input_log[g];
    let from = g.saturating_sub(GATE_ALIGN_PRE_GATE_LEAD as usize);
    input_log[from..g].iter().filter(|&&b| b != mask).count() as u32
}

/// Frames past the gate the aligned watcher checks bit-exact before
/// declaring Match (capped at the splice for CONT). A replay exact through
/// the settle can still veer off later; this catches that without watching
/// the whole run.
pub const BUCKET_VALIDATE_WINDOW: u32 = 1024;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_first_moving_finds_the_first_departure() {
        let mut c = vec![[1.0, 2.0, 3.0]; 10];
        c[5] = [1.0, 2.0, 3.5];
        assert_eq!(detect_first_moving(&c, 10), Some(5));
        assert_eq!(detect_first_moving(&[[1.0, 2.0, 3.0]; 10], 10), None);
        assert_eq!(detect_first_moving(&[], 0), None);
        // respects recorded_count bound
        let mut c2 = vec![[1.0, 2.0, 3.0]; 10];
        c2[5] = [9.0, 9.0, 9.0];
        assert_eq!(detect_first_moving(&c2, 3), None);
        assert_eq!(detect_first_moving(&c2, 6), Some(5));
    }
}
