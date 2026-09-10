//! The CONT (continue-record) F5-bucket judge, shared by tas_ui (the live
//! transport) and tas_test (the cont-reliability harness) so the harness
//! accepts or rejects a bucket on exactly the criteria the user experiences.

/// Max F5-restart retries to land the CONT replay on the recording's bucket.
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

/// Verdict from judging whether a CONT replay landed on the right F5 bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketVerdict {
    /// Playback hasn't progressed far enough to judge — keep polling.
    KeepWaiting,
    /// Spawn position doesn't match the recording's start — wrong bucket, reroll.
    WrongStart,
    /// Recording never moves out of spawn → no fingerprint → nothing to match.
    NoSignal,
    /// First-moving frame matched the recording → correct bucket.
    Match,
    /// First-moving frame differs from the recording → wrong bucket, reroll.
    WrongBucket { observed: Option<u32> },
}

/// MINIMUM frames past the recording's first-moving frame before the judge
/// will decide at all — enough settle trajectory to detect the first-moving
/// frame and seed the blowup guard.
pub const BUCKET_MATCH_WINDOW: u32 = 64;

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

/// MAXIMUM frames past first-moving the judge validates before declaring
/// Match. The blowup guard runs over the whole replayed prefix up to
/// `min(match_through, first_moving + 1024)` on every poll, so a bucket
/// that tracks through the settle and veers off hundreds of ticks later is
/// rerolled the instant its drift exceeds `BUCKET_MATCH_EPSILON`, while a
/// deep splice still gets a positive Match at the cap instead of running the
/// whole catch-up unjudged.
pub const BUCKET_VALIDATE_WINDOW: u32 = 1024;

/// How far the countdown's length can move between restarts, in ticks: the
/// countdown rounds by at most one tick and does not trend with the arm, so
/// a predicted first-moving frame is good to +/-1 and anything further out
/// cannot be the recording's bucket.
///
/// This is a REJECTION tolerance only. Loosening it costs a missed instant
/// reject (the replay judge then rules as before); tightening it risks
/// rerolling a bucket that would have matched. Neither can accept a wrong
/// one, which is what makes the whole predictive path safe to be wrong.
pub const COUNTDOWN_K_JITTER: i64 = 1;

/// Predictive rejects in a row before the learned countdown length is
/// thrown away and re-learned.
///
/// K is learned from an attempt's observed first-moving frame. A bucket
/// that lands somewhere else entirely can teach a WRONG K — and a wrong K
/// rejects good buckets at arm time, which means they never replay, which
/// means nothing ever observes a first-moving frame to correct it. That is
/// a deadlock, and the cycle would burn every retry without replaying once.
///
/// So a run of rejections with no confirming observation is treated as
/// evidence against K rather than against the buckets. Bounded at a few,
/// because legitimate runs of rejections are common — the arm offset has to
/// land in a narrow window — but an endless one never is.
pub const MAX_BLIND_PREDICTIVE_REJECTS: u32 = 4;

/// Per-axis BLOWUP guard (game units) over the judge window: a bucket whose
/// trajectory leaves the recording's by more than this is an impostor.
///
/// Deliberately GENEROUS, not a tracking tolerance. The bucket criterion is
/// the first-moving FRAME; a working bucket departs spawn with a sub-tick
/// phase skew and blips up to ~0.4 during the settle, while an impostor that
/// shares the frame diverges by 0.5 or more, so the guard sits at the top of
/// that gap. Demanding per-frame closeness here (bit-exact, or 0.01-0.02)
/// rejects the working buckets too and CONT never lands.
pub const BUCKET_MATCH_EPSILON: f32 = 0.5;

/// Judge a CONT replay's F5 bucket. Pure function over the shared-memory
/// snapshot so tas_ui and the harness share one source of truth.
///
/// 1. Spawn (frame 0) bits must equal the recording's start EXACTLY: the
///    pre-movement state is deterministic and is the bucket's identity.
/// 2. The replay's FIRST-MOVING FRAME must equal the recording's. This is
///    the bucket fingerprint; rerolling until it matches is the lottery.
/// 3. Blowup guard: the trajectory must stay within `BUCKET_MATCH_EPSILON`
///    of the recording over the whole replayed prefix, up to
///    `min(match_through, first_moving + BUCKET_VALIDATE_WINDOW)` and never
///    less than `first_moving + BUCKET_MATCH_WINDOW`.
///
/// `match_through` is the splice target (CONT) or the recording length
/// (PLAY): the bucket is validated clean THROUGH there before `Match`, and
/// a divergence anywhere before it rerolls the instant playback reaches it.
/// `observed` in `WrongBucket` is the replay's first-moving frame on a
/// fingerprint mismatch, or the divergence frame if the blowup guard fired.
pub fn judge_cont_bucket(
    play_coords: &[[f32; 3]],
    rec_coords: &[[f32; 3]],
    recorded_count: u32,
    playback_pos: u32,
    expected_start_bits: [u32; 3],
    expected_first_moving: Option<u32>,
    match_through: u32,
) -> BucketVerdict {
    if playback_pos == 0 || play_coords.is_empty() {
        return BucketVerdict::KeepWaiting;
    }
    let end = (playback_pos as usize)
        .min(play_coords.len())
        .min(rec_coords.len())
        .min(recorded_count as usize)
        .min(
            expected_first_moving
                .unwrap_or(0)
                .saturating_add(BUCKET_VALIDATE_WINDOW) as usize,
        );
    for k in 0..end {
        if !play_coords[k]
            .iter()
            .chain(&rec_coords[k])
            .all(|v| v.is_finite())
        {
            return BucketVerdict::WrongBucket {
                observed: Some(k as u32),
            };
        }
    }
    let play0 = play_coords[0];
    let play0_bits = [play0[0].to_bits(), play0[1].to_bits(), play0[2].to_bits()];
    if play0_bits != expected_start_bits {
        return BucketVerdict::WrongStart;
    }
    let expected_fm = match expected_first_moving {
        Some(f) => f,
        None => return BucketVerdict::NoSignal,
    };
    // The fingerprint is decidable the moment the replay has passed the
    // recording's first moving frame, and ruling here is the SAME ruling
    // the settle window would give, not a looser one: `detect_first_moving`
    // returns the FIRST index that differs from frame 0, so a longer prefix
    // can never change an answer it already gave, only supply one.
    //
    //   found k < expected  -> wrong bucket; scanning further still finds k
    //   found k == expected -> fingerprint matches
    //   found nothing yet   -> the replay left spawn LATER than the
    //                          recording, so any k a longer scan finds is
    //                          > expected and would be rejected anyway
    //
    // Only `observed` in the rejection differs: the third case reports None
    // instead of the exact late frame, which is diagnostic, not a decision.
    // Ruling early matters because a judged PLAY hands back to 1x at
    // first_moving + 1, so a wrong bucket would otherwise be watched in real
    // time for the whole settle window before rerolling.
    if playback_pos < expected_fm + 1 {
        return BucketVerdict::KeepWaiting;
    }
    // The bucket fingerprint: the replay must leave spawn on the SAME frame
    // the recording did.
    let observed_fm = detect_first_moving(play_coords, playback_pos);
    if observed_fm != Some(expected_fm) {
        return BucketVerdict::WrongBucket {
            observed: observed_fm,
        };
    }
    // Accepting, on the other hand, still needs the settle window: the
    // blowup guard below wants trajectory to judge, and BUCKET_MATCH_WINDOW
    // is the minimum that has ever been trusted for it.
    let min_judge = expected_fm + BUCKET_MATCH_WINDOW;
    if playback_pos < min_judge {
        return BucketVerdict::KeepWaiting;
    }
    // Validate clean through the splice target, capped so a deep splice
    // still confirms before running the whole catch-up. Never below the
    // settle window (we already have at least that much).
    let validate_to = match_through
        .min(expected_fm + BUCKET_VALIDATE_WINDOW)
        .max(min_judge);
    // Blowup guard over the WHOLE replayed prefix seen so far (not just the
    // settle window): a late divergence is caught the instant playback
    // reaches it. Working buckets stay ≤~0.42 the entire way — allowed.
    let end = (playback_pos as usize)
        .min(validate_to as usize)
        .min(play_coords.len())
        .min(rec_coords.len())
        .min(recorded_count as usize);
    for k in 0..end {
        let p = play_coords[k];
        let r = rec_coords[k];
        if (p[0] - r[0]).abs() > BUCKET_MATCH_EPSILON
            || (p[1] - r[1]).abs() > BUCKET_MATCH_EPSILON
            || (p[2] - r[2]).abs() > BUCKET_MATCH_EPSILON
        {
            return BucketVerdict::WrongBucket {
                observed: Some(k as u32),
            };
        }
    }
    // Clean so far — but only declare Match once we've actually validated
    // THROUGH the target depth. Until then keep replaying + re-checking, so
    // a divergence that hasn't happened yet can still reroll.
    if playback_pos < validate_to {
        return BucketVerdict::KeepWaiting;
    }
    BucketVerdict::Match
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_coordinates_never_match_or_become_no_signal() {
        for bad in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            for axis in 0..3 {
                for tick in [0, 10, 20] {
                    for recording_side in [false, true] {
                        let mut rec = vec![[1.0, 2.0, 3.0]; 100];
                        for c in &mut rec[10..] {
                            c[2] += 1.0;
                        }
                        let mut play = rec.clone();
                        if recording_side {
                            rec[tick][axis] = bad;
                        } else {
                            play[tick][axis] = bad;
                        }
                        for fingerprint in [None, Some(10)] {
                            assert!(matches!(
                                judge_cont_bucket(
                                    &play,
                                    &rec,
                                    100,
                                    100,
                                    [1.0f32, 2.0, 3.0].map(f32::to_bits),
                                    fingerprint,
                                    100
                                ),
                                BucketVerdict::WrongBucket { .. }
                            ));
                        }
                    }
                }
            }
        }
    }

    fn bits(x: f32, y: f32, z: f32) -> [u32; 3] {
        [x.to_bits(), y.to_bits(), z.to_bits()]
    }

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

    /// A matching fingerprint is not accepted before the settle window
    /// (first_moving + BUCKET_MATCH_WINDOW) has replayed.
    #[test]
    fn judge_keep_waiting_before_window() {
        let mut play = vec![[1.0, 2.0, 3.0]; 400];
        play[250] = [1.0, 2.0, 3.5];
        let rec = play.clone();
        // pos must be >= first_moving + BUCKET_MATCH_WINDOW (314) to judge.
        assert_eq!(
            judge_cont_bucket(&play, &rec, 400, 100, bits(1.0, 2.0, 3.0), Some(250), 320),
            BucketVerdict::KeepWaiting
        );
        assert_eq!(
            judge_cont_bucket(&play, &rec, 400, 313, bits(1.0, 2.0, 3.0), Some(250), 320),
            BucketVerdict::KeepWaiting
        );
    }

    /// Ruling at first_moving+1 must be the SAME ruling as at
    /// first_moving+BUCKET_MATCH_WINDOW, never a looser one: for every way a
    /// replay can leave the spawn, the two verdicts must agree.
    #[test]
    fn early_rejection_agrees_with_the_settle_window() {
        const FM: usize = 250;
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        for (k, item) in rec.iter_mut().enumerate().skip(FM) {
            item[2] = 3.0 + (k - FM + 1) as f32 * 0.001;
        }
        let start = bits(1.0, 2.0, 3.0);

        // Departure frames either side of the recording's, plus the exact
        // match, plus a replay that never leaves the spawn at all.
        for depart in [
            Some(FM - 8),
            Some(FM - 1),
            Some(FM),
            Some(FM + 1),
            Some(FM + 30),
            None,
        ] {
            let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
            if let Some(d) = depart {
                for (k, item) in play.iter_mut().enumerate().skip(d) {
                    item[2] = 3.0 + (k - d + 1) as f32 * 0.001;
                }
            }
            let early = judge_cont_bucket(
                &play,
                &rec,
                400,
                (FM + 1) as u32,
                start,
                Some(FM as u32),
                400,
            );
            let late = judge_cont_bucket(
                &play,
                &rec,
                400,
                (FM + BUCKET_MATCH_WINDOW as usize) as u32,
                start,
                Some(FM as u32),
                400,
            );
            let agree = match (&early, &late) {
                // Same rejection. `observed` may differ - a replay that has
                // not moved yet reports None early and the exact late frame
                // later - and that field is diagnostic, not a decision.
                (BucketVerdict::WrongBucket { .. }, BucketVerdict::WrongBucket { .. }) => true,
                // Matching bucket: rejected by neither. Accepting still waits
                // for the settle window, so early is KeepWaiting there.
                (BucketVerdict::KeepWaiting, BucketVerdict::KeepWaiting) => true,
                _ => false,
            };
            assert!(
                agree,
                "depart={:?}: early={:?} but the settle window said {:?}",
                depart, early, late
            );
        }
    }

    /// ...and it really is EARLIER: a wrong bucket is rejected at
    /// first_moving+1, long before the settle window.
    #[test]
    fn a_wrong_bucket_is_rejected_at_first_moving_plus_one() {
        let mut rec = vec![[1.0f32, 2.0, 3.0]; 400];
        rec[250] = [1.0, 2.0, 3.5];
        let mut play = vec![[1.0f32, 2.0, 3.0]; 400];
        play[249] = [1.0, 2.0, 3.5]; // left the spawn a frame early
        assert_eq!(
            judge_cont_bucket(&play, &rec, 400, 251, bits(1.0, 2.0, 3.0), Some(250), 400),
            BucketVerdict::WrongBucket {
                observed: Some(249)
            }
        );
    }

    /// A working bucket reproduces the recording's first-moving frame but not
    /// its trajectory to the bit: float noise (~6e-5) plus a settle blip of up
    /// to ~0.4. Only an impostor past the 0.5 guard is rejected.
    #[test]
    fn judge_tolerates_right_bucket_float_noise() {
        let mut rec = vec![[100.0_f32, 200.0, 300.0]; 400];
        for (f, coord) in rec.iter_mut().enumerate().skip(250) {
            *coord = [100.0 + (f as f32) * 0.5, 200.0, 300.0];
        }
        let start = bits(100.0, 200.0, 300.0);
        // Working bucket: same first-moving frame, tiny noise everywhere.
        let mut noisy = rec.clone();
        for coord in noisy.iter_mut().skip(250) {
            coord[0] += 0.00006;
            coord[2] -= 0.00004;
        }
        assert_eq!(
            judge_cont_bucket(&noisy, &rec, 400, 320, start, Some(250), 320),
            BucketVerdict::Match
        );
        // Working bucket with a settle blip (0.4 transient): still accepted.
        let mut blip = rec.clone();
        blip[252][0] += 0.4;
        assert_eq!(
            judge_cont_bucket(&blip, &rec, 400, 320, start, Some(250), 320),
            BucketVerdict::Match
        );
        // Impostor: same first-moving frame but past the blowup guard.
        let mut wrong = rec.clone();
        wrong[252][0] += 0.7;
        assert_eq!(
            judge_cont_bucket(&wrong, &rec, 400, 320, start, Some(250), 320),
            BucketVerdict::WrongBucket {
                observed: Some(252)
            }
        );
        // Wrong fingerprint: departs spawn a frame early, reported with the
        // replay's first-moving frame.
        let mut early = rec.clone();
        early[249] = [100.5, 200.0, 300.0];
        assert_eq!(
            judge_cont_bucket(&early, &rec, 400, 320, start, Some(250), 320),
            BucketVerdict::WrongBucket {
                observed: Some(249)
            }
        );
    }

    /// The blowup guard reports the divergence frame, and it stops looking at
    /// `first_moving + BUCKET_VALIDATE_WINDOW`: a splice deeper than that is
    /// confirmed at the cap, not at the splice.
    #[test]
    fn late_divergence_reports_its_frame_and_validation_is_capped() {
        let fm = 250usize;
        let splice = 1400u32; // deeper than fm + BUCKET_VALIDATE_WINDOW
        let cap = fm as u32 + BUCKET_VALIDATE_WINDOW;
        let mut rec = vec![[100.0_f32, 200.0, 300.0]; 1400];
        for (f, coord) in rec.iter_mut().enumerate().skip(fm) {
            *coord = [100.0, 200.0, 300.0 + (f as f32) * 0.5]; // rides +Z
        }
        let start = bits(100.0, 200.0, 300.0);

        // Identical through the settle, an 8-unit veer at fm+500.
        let mut play = rec.clone();
        for coord in play.iter_mut().skip(fm + 500) {
            coord[0] += 8.0;
        }
        assert_eq!(
            judge_cont_bucket(&play, &rec, 1400, 320, start, Some(250), splice),
            BucketVerdict::KeepWaiting,
            "clean so far is not yet validated through the target"
        );
        assert_eq!(
            judge_cont_bucket(&play, &rec, 1400, 800, start, Some(250), splice),
            BucketVerdict::WrongBucket {
                observed: Some(750)
            }
        );

        // A bucket that only diverges past the cap is confirmed at the cap.
        let mut past_cap = rec.clone();
        past_cap[cap as usize + 10][0] += 8.0;
        assert_eq!(
            judge_cont_bucket(&past_cap, &rec, 1400, cap - 1, start, Some(250), splice),
            BucketVerdict::KeepWaiting
        );
        assert_eq!(
            judge_cont_bucket(&past_cap, &rec, 1400, cap, start, Some(250), splice),
            BucketVerdict::Match
        );
    }
}
