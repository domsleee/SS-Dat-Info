//! Gate alignment: a replay's input is indexed from the frame the boarder
//! first moves (the gate) rather than from the arm, so the countdown's length
//! does not matter. tas_ui and tas_test share these helpers so both judge a
//! replay on exactly the same criteria.

/// Max restarts for an aligned-trajectory mismatch.
pub const ALIGN_MAX_RETRIES: u32 = 30;

/// Frames past the gate the watcher checks bit-exact before `Matched` (capped
/// at the splice for CONT). A replay exact through the settle can still veer
/// off later; this catches that without watching the whole run.
pub const ALIGN_VERIFY_FRAMES: u32 = 1024;

/// Rust mirror of gate_alignment.hpp's GATE_ALIGN_PRE_GATE_LEAD; keep the two
/// in lock-step. While the live gate is pending, every recorded input frame in
/// [rec_gate - LEAD, rec_gate) is replaced by the gate mask, so the window is
/// kept as narrow as the measured gate jitter (at most four cycles) allows.
pub const GATE_ALIGN_PRE_GATE_LEAD: u32 = 8;

/// First frame index where `rec_coords` differs (bit-exact) from
/// `rec_coords[0]`: when the recorded player leaves the spawn. `None` if it
/// never moves within `recorded_count`.
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

/// How many recorded input frames inside the pre-gate hold window differ from
/// the gate mask, i.e. transitions an aligned replay will NOT reproduce at
/// their recorded position. The armers warn when this is non-zero, so a reroll
/// loop or a late divergence has a visible cause.
pub fn pre_gate_hold_overwrites(input_log: &[u8], rec_gate: u32) -> u32 {
    let g = rec_gate as usize;
    if g == 0 || g >= input_log.len() {
        return 0;
    }
    let mask = input_log[g];
    let from = g.saturating_sub(GATE_ALIGN_PRE_GATE_LEAD as usize);
    input_log[from..g].iter().filter(|&&b| b != mask).count() as u32
}

/// Verdict from watching a gate-aligned replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignVerdict {
    /// Not far enough past the gate to decide yet.
    Pending,
    /// A coordinate capture failed, so the trajectory cannot be trusted.
    CaptureFailed,
    /// The recording has no trajectory past its gate to compare against.
    MissingTrajectory,
    /// The checked prefix matched the recording bit for bit. The prefix is
    /// bounded by the recording's length, ALIGN_VERIFY_FRAMES and, for CONT,
    /// the splice.
    Matched,
    /// The replay differs from the recording at gate-relative frame `at`.
    Diverged { at: Option<u32> },
}

/// Compare an aligned replay with its recording, frame for frame from each
/// side's own gate. The cycle cave feeds the same recorded input from the same gate, so
/// the coordinates must be bit-exact; a hidden difference in the spawn state
/// shows up here and is rerolled rather than becoming the run.
///
/// `max_depth_rel` caps the check at a CONT splice (0 = uncapped): the DLL
/// parks playback at the splice until approved, so the verdict must be
/// decidable from the prefix that exists by then.
#[allow(clippy::too_many_arguments)] // reads many independent shared-state fields
pub fn check_aligned_trajectory(
    play_coords: &[[f32; 3]],
    rec_coords: &[[f32; 3]],
    recorded_count: u32,
    playback_pos: u32,
    live_gate: u32,
    rec_gate: u32,
    capture_ok: bool,
    max_depth_rel: u32,
) -> AlignVerdict {
    if !capture_ok {
        return AlignVerdict::CaptureFailed;
    }
    if live_gate == 0 || playback_pos <= live_gate {
        return AlignVerdict::Pending;
    }

    let rec_end = (recorded_count as usize).min(rec_coords.len());
    let rec_gate = rec_gate as usize;
    let live_gate = live_gate as usize;
    if rec_gate >= rec_end || live_gate >= play_coords.len() {
        return AlignVerdict::MissingTrajectory;
    }

    let depth_cap = if max_depth_rel == 0 {
        usize::MAX
    } else {
        max_depth_rel as usize
    };
    let depth = (rec_end - rec_gate)
        .min(ALIGN_VERIFY_FRAMES as usize)
        .min(depth_cap);
    if depth == 0 {
        return AlignVerdict::MissingTrajectory;
    }
    let available = (playback_pos as usize)
        .saturating_sub(live_gate)
        .min(play_coords.len().saturating_sub(live_gate))
        .min(depth);

    for k in 0..available {
        let p = play_coords[live_gate + k];
        let r = rec_coords[rec_gate + k];
        if !p.iter().all(|v| v.is_finite())
            || !r.iter().all(|v| v.is_finite())
            || p[0].to_bits() != r[0].to_bits()
            || p[1].to_bits() != r[1].to_bits()
            || p[2].to_bits() != r[2].to_bits()
        {
            return AlignVerdict::Diverged { at: Some(k as u32) };
        }
    }

    if available == depth {
        AlignVerdict::Matched
    } else {
        AlignVerdict::Pending
    }
}

/// A recording and a replay that agree frame for frame from their own gates
/// for `depth` frames, and are zero elsewhere.
#[cfg(test)]
pub(crate) fn aligned_trajectory(
    rec_gate: usize,
    live_gate: usize,
    depth: usize,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let len = (rec_gate.max(live_gate) + depth + 16).max(400);
    let mut rec = vec![[0.0; 3]; len];
    let mut play = vec![[0.0; 3]; len];
    for k in 0..depth {
        let point = [k as f32 + 1.25, k as f32 * 0.5 + 2.0, -(k as f32)];
        rec[rec_gate + k] = point;
        play[live_gate + k] = point;
    }
    (play, rec)
}

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

    #[test]
    fn matches_a_shifted_gate_relative_trajectory() {
        // recorded_count 400, rec_gate 299 -> full depth is 101; the
        // watcher must see ALL of it before declaring Matched.
        let (play, rec) = aligned_trajectory(299, 297, 101);
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 398, 297, 299, true, 0),
            AlignVerdict::Matched
        );
    }

    #[test]
    fn checks_past_the_settle_window() {
        // Exact through the settle, divergent at gate-relative frame 80.
        let (mut play, rec) = aligned_trajectory(299, 297, 101);
        play[297 + 80][2] += 0.5;
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 398, 297, 299, true, 0),
            AlignVerdict::Diverged { at: Some(80) }
        );
    }

    #[test]
    fn cont_depth_is_capped_at_the_splice() {
        // A CONT splicing 70 frames past the gate only ever shows the
        // watcher 70 frames (the DLL parks there).
        let (play, rec) = aligned_trajectory(299, 297, 70);
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 367, 297, 299, true, 70),
            AlignVerdict::Matched
        );
    }

    #[test]
    fn rejects_a_one_bit_hidden_state_difference() {
        let (mut play, rec) = aligned_trajectory(299, 297, 101);
        play[297][0] = f32::from_bits(play[297][0].to_bits() + 1);
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 298, 297, 299, true, 0),
            AlignVerdict::Diverged { at: Some(0) }
        );
    }

    #[test]
    fn waits_for_the_full_window_and_a_clean_capture() {
        let (play, rec) = aligned_trajectory(299, 297, 101);
        // One frame short of the full 101-frame depth: keep waiting.
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 397, 297, 299, true, 0),
            AlignVerdict::Pending
        );
        assert_eq!(
            check_aligned_trajectory(&play, &rec, 400, 398, 297, 299, false, 0),
            AlignVerdict::CaptureFailed
        );
    }
}
