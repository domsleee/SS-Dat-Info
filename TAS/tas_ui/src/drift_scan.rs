//! Incremental replay-vs-recording comparison behind the DRIFT banner, the
//! debug drift graph and the session-log drift lines.

use tas_shared::{TasMode, TasSharedState};

/// The gate-aligned comparison window, latched per arm generation.
///
/// Gate-aligned replays are correct when `play[live_gate+k] == rec[rec_gate+k]`,
/// so the bases come from the DLL's gate fields while both are set and stay
/// latched afterwards: the DLL clears both the moment playback completes, and
/// re-scanning raw indices at that point reported the alignment shift itself as
/// drift at the end of every bit-exact replay whose gate landed elsewhere
/// (Time Attack ghosts move it ~11 ticks). A new `arm_generation` drops the
/// latch; nothing else does.
#[derive(Default)]
pub struct DriftWindow {
    bases: Option<(usize, usize)>,
    latch_arm_generation: u32,
}

impl DriftWindow {
    /// `(play_base, rec_base)`, raw indices when no alignment is latched.
    pub fn bases(&self) -> (usize, usize) {
        self.bases.unwrap_or((0, 0))
    }

    #[cfg(test)]
    pub fn aligned_bases(&self) -> Option<(usize, usize)> {
        self.bases
    }

    /// Number of comparable pairs right now and whether a new arm or a base
    /// change invalidated everything scanned so far. Zero pairs outside PLAY:
    /// OFF retains the previous playback buffer while a history restore may
    /// replace the recording under it, and REC is not a playback verdict.
    pub fn update(&mut self, state: &TasSharedState) -> (usize, bool) {
        // A fast replay can start and finish between two UI polls without the
        // position ever falling back below the old gate, so the arm counter,
        // not the playback position, is what drops the previous attempt.
        let mut forced_reset = false;
        if state.arm_generation != self.latch_arm_generation {
            self.latch_arm_generation = state.arm_generation;
            self.bases = None;
            forced_reset = true;
        }
        if state.mode != TasMode::Play as u32 {
            return (0, forced_reset);
        }

        let live_gate = state.gate_index as usize;
        let rec_gate = state.gate_align_rec as usize;
        if live_gate > 0 && rec_gate > 0 {
            let next = Some((live_gate, rec_gate));
            if self.bases != next {
                self.bases = next;
                forced_reset = true;
            }
        } else if rec_gate > 0 && self.bases.is_none() {
            // This arm expects gate alignment but the live gate has not fired
            // yet: raw-index comparisons during the variable spawn countdown
            // say nothing about replay determinism.
            return (0, forced_reset);
        }
        let (play_base, rec_base) = self.bases();
        let count = (state.playback_pos as usize)
            .saturating_sub(play_base)
            .min((state.recorded_count as usize).saturating_sub(rec_base))
            .min(state.play_coords.len().saturating_sub(play_base))
            .min(state.rec_coords.len().saturating_sub(rec_base));
        (count, forced_reset)
    }
}

/// Running maxima and the splice verdict of the current PLAY / CONT.
///
/// `max_dx` / `max_dz` are the historical diagnostic: the largest difference
/// seen anywhere in the prefix, kept latched even after a transient heals.
/// The banner must never verdict on those for a CONT; it uses the splice-tick
/// drift, which is zero / `None` until playback has covered the splice.
#[derive(Default)]
pub struct DriftTracker {
    pub window: DriftWindow,
    pub max_dx: f32,
    pub max_dz: f32,
    pub splice_dx: f32,
    pub splice_dz: f32,
    /// The splice whose agreement `splice_dx` / `splice_dz` describe.
    pub splice_tick: Option<usize>,
    /// First recording tick with a non-zero X/Z difference in this PLAY.
    pub first_drift_tick: Option<usize>,
    last_count: usize,
}

impl DriftTracker {
    fn clear_measurements(&mut self) {
        self.max_dx = 0.0;
        self.max_dz = 0.0;
        self.splice_dx = 0.0;
        self.splice_dz = 0.0;
        self.splice_tick = None;
        self.first_drift_tick = None;
        self.last_count = 0;
    }

    /// Compare the pairs not scanned yet and update the maxima and the splice
    /// verdict. Returns `(pairs compared so far, everything was reset)`.
    ///
    /// Only X and Z are compared. The banner is a live "still on the recorded
    /// line" hint calibrated on the ground track (the captured
    /// `cont-splice-4500` fixture); `tas_test/src/drift.rs` compares Y as well
    /// because the certification harness must not pass a height-only
    /// regression on an identical ground track. The harness is the gate, this
    /// scan is the hint, so the two deliberately differ.
    pub fn scan(&mut self, state: &TasSharedState) -> (usize, bool) {
        let (count, forced_reset) = self.window.update(state);
        if forced_reset {
            self.clear_measurements();
        }
        if state.mode != TasMode::Play as u32 {
            if state.mode == TasMode::Rec as u32 {
                // cave2 freezes playback_pos at the splice and starts writing
                // REC at `splice`, so the last untouched reference pair is
                // `splice - 1`. REC can publish this verdict even when the
                // whole catch-up PLAY happened between two UI polls.
                self.splice_tick = None;
                self.splice_dx = 0.0;
                self.splice_dz = 0.0;
                let splice = state.segment_start_frame as usize;
                let played = state.playback_pos as usize;
                if splice > 0
                    && splice <= state.recorded_count as usize
                    && splice <= state.rec_coords.len()
                    && played > 0
                    && played <= state.play_coords.len()
                {
                    let play = state.play_coords[played - 1];
                    let rec = state.rec_coords[splice - 1];
                    self.splice_dx = coordinate_delta(play[0], rec[0]);
                    self.splice_dz = coordinate_delta(play[2], rec[2]);
                    self.splice_tick = Some(splice);
                }
            }
            return (self.last_count, forced_reset);
        }
        let (play_base, rec_base) = self.window.bases();
        let reset = forced_reset || count < self.last_count;
        if reset {
            self.clear_measurements();
        }
        for i in self.last_count..count {
            let rec_tick = rec_base + i;
            let play = state.play_coords[play_base + i];
            let rec = state.rec_coords[rec_tick];
            for (axis, max) in [(0, &mut self.max_dx), (2, &mut self.max_dz)] {
                let d = (play[axis] - rec[axis]).abs();
                if d > 0.0 && self.first_drift_tick.is_none() {
                    self.first_drift_tick = Some(rec_tick);
                }
                if d > *max {
                    *max = d;
                }
            }
        }
        self.last_count = count;
        // `splice` is an exclusive prefix endpoint, not a captured playback
        // tick; a parked PLAY may reach it before the watcher approves the
        // REC switch.
        self.splice_dx = 0.0;
        self.splice_dz = 0.0;
        self.splice_tick = None;
        let splice = state.continue_from_frame as usize;
        if splice > rec_base && count >= splice - rec_base {
            let i = splice - rec_base - 1;
            let play = state.play_coords[play_base + i];
            let rec = state.rec_coords[rec_base + i];
            self.splice_dx = coordinate_delta(play[0], rec[0]);
            self.splice_dz = coordinate_delta(play[2], rec[2]);
            self.splice_tick = Some(splice);
        }
        (count, reset)
    }

    pub fn max_drift(&self) -> f32 {
        self.max_dx.max(self.max_dz)
    }

    pub fn splice_drift(&self) -> f32 {
        self.splice_dx.max(self.splice_dz)
    }

    /// The F12/CONT verdict. For a CONT (splice != 0) only the splice-tick
    /// agreement counts, once playback has covered it: a transient that heals
    /// before the splice must never banner, not even while the playback head
    /// is inside it, which is why the head drift is not consulted. A plain
    /// PLAY has no verdict point, so any measured divergence banners.
    pub fn banner_visible(&self, state: &TasSharedState) -> bool {
        if (state.mode != TasMode::Play as u32 && state.mode != TasMode::Rec as u32)
            || state.arm_generation != self.window.latch_arm_generation
        {
            return false;
        }
        let splice = cont_verdict_boundary(state);
        if splice != 0 {
            return self.splice_tick == Some(splice) && self.splice_drift() > 0.0;
        }
        state.mode == TasMode::Play as u32
            && self.first_drift_tick.is_some()
            && self.max_drift() > 0.0
    }

    /// `(dx, dz, splice)` the banner should print: the splice agreement for a
    /// CONT, the prefix maxima for a plain PLAY.
    pub fn verdict(&self, state: &TasSharedState) -> (f32, f32, usize) {
        let splice = cont_verdict_boundary(state);
        if splice != 0 {
            (self.splice_dx, self.splice_dz, splice)
        } else {
            (self.max_dx, self.max_dz, 0)
        }
    }
}

pub fn coordinate_delta(play: f32, rec: f32) -> f32 {
    if play.is_finite() && rec.is_finite() {
        (play - rec).abs()
    } else {
        f32::INFINITY
    }
}

/// The splice a CONT verdicts at: the requested one during the catch-up PLAY,
/// the one cave2 actually spliced at once REC has started.
pub fn cont_verdict_boundary(state: &TasSharedState) -> usize {
    if state.mode == TasMode::Rec as u32 {
        state.segment_start_frame as usize
    } else {
        state.continue_from_frame as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_accumulates_incrementally_and_resets_when_playback_restarts() {
        let mut state = tas_shared::zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.recorded_count = 20;
        state.playback_pos = 10;
        for i in 0..20usize {
            state.play_coords[i] = [0.1, 0.0, 0.2];
        }
        state.play_coords[15] = [9.9, 0.0, 8.8];
        let mut tracker = DriftTracker::default();

        let (count, _) = tracker.scan(&state);
        assert_eq!(count, 10);
        assert!((tracker.max_dx - 0.1).abs() < 0.001);
        assert!((tracker.max_dz - 0.2).abs() < 0.001);
        assert_eq!(tracker.first_drift_tick, Some(0));

        state.playback_pos = 20;
        let (count, reset) = tracker.scan(&state);
        assert_eq!(count, 20);
        assert!(!reset);
        assert!((tracker.max_dx - 9.9).abs() < 0.001);
        assert!((tracker.max_dz - 8.8).abs() < 0.001);

        // Playback restarted from the top: the maxima belong to the old run.
        state.playback_pos = 0;
        let (_, reset) = tracker.scan(&state);
        assert!(reset);
        assert_eq!(tracker.max_dx, 0.0);
        assert_eq!(tracker.max_dz, 0.0);
        assert_eq!(tracker.first_drift_tick, None);
    }

    #[test]
    fn scan_keeps_gate_alignment_after_the_dll_clears_it() {
        // A no-ghost recording (gate 299) replayed with three Time Attack
        // ghosts loaded (gate 287) is bit-exact pair for pair, but the DLL
        // zeroes gate_index / gate_align_rec when the replay completes.
        let mut tracker = DriftTracker::default();
        let mut state = tas_shared::zeroed_boxed();
        state.recorded_count = 400;
        for i in 0..400usize {
            state.rec_coords[i] = [0.0, 0.0, i.saturating_sub(299) as f32];
            state.play_coords[i] = [0.0, 0.0, i.saturating_sub(287) as f32];
        }

        // Mid-replay, aligned: no drift.
        state.mode = TasMode::Play as u32;
        state.playback_pos = 380;
        state.gate_index = 287;
        state.gate_align_rec = 299;
        tracker.scan(&state);
        assert_eq!(tracker.window.aligned_bases(), Some((287, 299)));
        assert_eq!(tracker.max_dz, 0.0, "aligned replay is drift-free");

        // Playback completes: the DLL clears both gate fields.
        state.mode = TasMode::Off as u32;
        state.playback_pos = 388;
        state.gate_index = 0;
        state.gate_align_rec = 0;
        let (_, reset) = tracker.scan(&state);
        assert!(
            !reset,
            "completion must not restart the scan from raw indices"
        );
        assert_eq!(tracker.window.aligned_bases(), Some((287, 299)));
        assert_eq!(
            tracker.max_dz, 0.0,
            "the latched alignment survives completion"
        );

        // A fresh aligned session resets immediately, then waits for its own
        // live gate instead of comparing raw countdown indices.
        state.mode = TasMode::Play as u32;
        state.playback_pos = 100;
        state.gate_align_rec = 299;
        state.arm_generation += 1;
        let (count, reset) = tracker.scan(&state);
        assert!(tracker.window.aligned_bases().is_none());
        assert!(reset);
        assert_eq!(count, 0);
        assert_eq!(tracker.max_dz, 0.0, "pre-gate countdown is not drift");

        // Once this attempt's gate arrives, aligned comparison starts cleanly.
        state.playback_pos = 380;
        state.gate_index = 287;
        state.gate_align_rec = 299;
        tracker.scan(&state);
        assert_eq!(tracker.window.aligned_bases(), Some((287, 299)));

        // A new arm between two polls always drops the latch and maxima.
        tracker.max_dz = 7.0;
        state.gate_index = 0;
        state.arm_generation += 1;
        let (_, reset) = tracker.scan(&state);
        assert!(reset, "a new arm restarts the drift scan");
        assert!(tracker.window.aligned_bases().is_none());
        assert_eq!(tracker.max_dz, 0.0);
    }

    #[test]
    fn scan_ignores_stale_playback_while_off() {
        let mut tracker = DriftTracker::default();
        let mut state = tas_shared::zeroed_boxed();
        state.mode = TasMode::Off as u32;
        state.recorded_count = 20;
        state.playback_pos = 20;
        for i in 0..20 {
            state.rec_coords[i] = [1000.0, 0.0, 2000.0];
        }
        let (count, _) = tracker.scan(&state);
        assert_eq!(count, 0);
        assert_eq!(tracker.max_dx, 0.0);
        assert_eq!(tracker.max_dz, 0.0);
        assert_eq!(tracker.first_drift_tick, None);
    }

    #[test]
    fn scan_records_the_actual_first_drift_tick() {
        let mut tracker = DriftTracker::default();
        let mut state = tas_shared::zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.recorded_count = 10;
        state.playback_pos = 10;
        state.play_coords[4][0] = 0.25;
        state.play_coords[8][2] = 3.0;
        tracker.scan(&state);
        assert_eq!(tracker.first_drift_tick, Some(4));
        assert_eq!(tracker.max_dx, 0.25);
        assert_eq!(tracker.max_dz, 3.0);
    }

    #[test]
    fn cont_banner_requires_divergence_at_the_splice() {
        let mut tracker = DriftTracker::default();
        let mut state = tas_shared::zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.continue_from_frame = 8;
        state.recorded_count = 10;
        state.playback_pos = 10;
        state.arm_generation = 41;

        // Exact replay: splice covered, zero drift, no banner.
        tracker.scan(&state);
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.0);
        assert!(!tracker.banner_visible(&state));

        // Divergence PAST the splice (head inside a transient) must not banner:
        // the verdict point is clean.
        state.play_coords[9][0] = 0.5;
        state.arm_generation += 1;
        tracker.scan(&state);
        assert_eq!(tracker.first_drift_tick, Some(9));
        assert!(tracker.max_dx > 0.0, "history keeps the transient");
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.0);
        assert!(!tracker.banner_visible(&state));

        // A divergence AT the splice point is reported.
        state.play_coords[9][0] = 0.0;
        state.play_coords[7][0] = 0.5;
        state.arm_generation += 1;
        tracker.scan(&state);
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.5);
        assert!(tracker.banner_visible(&state));

        // An earlier transient that heals before the splice does not banner.
        state.play_coords[7][0] = 0.0;
        state.play_coords[4][0] = 0.5;
        state.arm_generation += 1;
        tracker.scan(&state);
        assert_eq!(tracker.first_drift_tick, Some(4));
        assert!(tracker.max_dx > 0.0, "history keeps the transient");
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.0);
        assert!(!tracker.banner_visible(&state));

        // Before the next frame's scan, the next arm must not flash the prior
        // attempt's warning.
        state.arm_generation += 1;
        assert!(!tracker.banner_visible(&state));
    }

    #[test]
    fn cont_banner_stays_clear_while_head_crosses_a_transient() {
        // The UI scans every frame while the catch-up replay advances, so the
        // playback head spends polls inside transients; the splice verdict must
        // stay clear at every step until the splice itself is covered.
        let mut tracker = DriftTracker::default();
        let mut state = tas_shared::zeroed_boxed();
        state.mode = TasMode::Play as u32;
        state.continue_from_frame = 8;
        state.recorded_count = 10;
        state.arm_generation = 7;
        state.play_coords[4][0] = 0.5;
        for playback_pos in [5u32, 6, 8, 9, 10] {
            state.playback_pos = playback_pos;
            tracker.scan(&state);
            assert!(
                !tracker.banner_visible(&state),
                "no banner with head at {playback_pos}"
            );
        }
        assert_eq!(
            tracker.first_drift_tick,
            Some(4),
            "transient still diagnosed"
        );
        assert!(tracker.max_dx > 0.0, "history keeps the transient");
        assert_eq!(tracker.splice_tick, Some(8));
        assert_eq!(tracker.splice_dx, 0.0);
    }
}
