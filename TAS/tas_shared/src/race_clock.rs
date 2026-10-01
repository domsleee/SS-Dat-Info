//! The game's own race clock (DESIGN.md "Race time"): the player's timer
//! float, which grows by 0.01f per Player update while the race runs and
//! which the HUD formats. Its value drifts from ticks/100 (60,000 ticks read
//! 600.27 s), so the tick count is found by replaying the float sum.

use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaceClock {
    pub seconds: f32,
    pub started: bool,
    pub finished: bool,
}

/// The clock as last published, or None outside a race.
pub fn race_clock(state: &TasSharedState) -> Option<RaceClock> {
    let (bits, flags) = with_seqlock(&state.race_seq, || unsafe {
        (
            std::ptr::read_volatile(&state.race_clock_bits),
            std::ptr::read_volatile(&state.race_clock_flags),
        )
    })?;
    (bits != u32::MAX).then(|| RaceClock {
        seconds: f32::from_bits(bits),
        started: flags & TAS_RACE_CLOCK_STARTED != 0,
        finished: flags & TAS_RACE_CLOCK_FINISHED != 0,
    })
}

/// The last HUD player-line time (centiseconds) with the clock read in the
/// same call, or None before the first one.
pub fn race_ab(state: &TasSharedState) -> Option<(u32, f32)> {
    let (cs, bits) = with_seqlock(&state.race_seq, || unsafe {
        (
            std::ptr::read_volatile(&state.race_ab_cs),
            std::ptr::read_volatile(&state.race_ab_bits),
        )
    })?;
    (cs != 0 || bits != 0).then(|| (cs, f32::from_bits(bits)))
}

/// The human rider's last finish, from the game's Finish_Point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RaceFinish {
    /// Bumps once per finish; compare with a value read earlier.
    pub seq: u32,
    /// Recorded (REC) or playback (PLAY) index of the tick it finished in.
    pub tick: u32,
    /// TasMode at the finish.
    pub mode: u32,
    /// False when a checkpoint was missed (no official time).
    pub valid: bool,
    /// The race timer's final value.
    pub seconds: f32,
}

/// The last finish, or None before the first. `seq` counts finishes.
pub fn race_finish(state: &TasSharedState) -> Option<RaceFinish> {
    use std::sync::atomic::Ordering;
    for _ in 0..64 {
        let seq = state.race_finish_seq.load(Ordering::Acquire);
        if seq == 0 {
            return None;
        }
        if seq % 2 == 1 {
            std::hint::spin_loop();
            continue;
        }
        // SAFETY: shared mapping; the DLL writes the fields before the sequence.
        let (tick, mode, valid, bits) = unsafe {
            (
                std::ptr::read_volatile(&state.race_finish_tick),
                std::ptr::read_volatile(&state.race_finish_mode),
                std::ptr::read_volatile(&state.race_finish_valid),
                std::ptr::read_volatile(&state.race_finish_time_bits),
            )
        };
        if state.race_finish_seq.load(Ordering::Acquire) == seq {
            return Some(RaceFinish {
                seq: seq / 2,
                tick,
                mode,
                valid: valid != 0,
                seconds: f32::from_bits(bits),
            });
        }
    }
    None
}

/// What the HUD shows now, in centiseconds, once the race has started.
pub fn hud_time_cs(state: &TasSharedState) -> Option<u32> {
    let clock = race_clock(state)?;
    clock
        .started
        .then(|| hud_cs(clock.seconds, extended_precision(state.fpu_control_word)))
}

/// Race ticks the clock has counted: the n with n additions of 0.01f from
/// 0 equal to `seconds`, or None if no such n below `limit`. (The float is
/// stored after every addition, so the sum is the same at either x87
/// precision.)
pub fn ticks(seconds: f32, limit: u32) -> Option<u32> {
    let mut t = 0.0f32;
    for n in 0..=limit {
        if t.to_bits() == seconds.to_bits() {
            return Some(n);
        }
        if t > seconds {
            return None;
        }
        t += 0.01f32;
    }
    None
}

/// The time the HUD shows for `seconds`, in centiseconds: MM:SS:CC with
/// MM = ftol(t * 0.016666668f), SS = ftol(t) % 60, CC = ftol(t * 100) % 100,
/// each truncated. `extended` = the products were computed at 53-bit x87
/// precision (OpenGL) rather than 24-bit (DirectX).
pub fn hud_cs(seconds: f32, extended: bool) -> u32 {
    let (mm, cc) = if extended {
        let t = seconds as f64;
        (
            (t * 0.016666668f32 as f64).trunc() as u32,
            (t * 100.0).trunc() as u32 % 100,
        )
    } else {
        (
            (seconds * 0.016666668f32).trunc() as u32,
            (seconds * 100.0f32).trunc() as u32 % 100,
        )
    };
    let ss = seconds.trunc() as u32 % 60;
    mm * 6000 + ss * 100 + cc
}

/// Whether an x87 control word selects more than 24-bit precision.
pub fn extended_precision(fpu_control_word: u32) -> bool {
    (fpu_control_word >> 8) & 3 != 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum(n: u32) -> f32 {
        (0..n).fold(0.0f32, |t, _| t + 0.01f32)
    }

    #[test]
    fn ticks_invert_the_float_sum() {
        for n in [0, 1, 2, 299, 6000, 30_000, 60_000] {
            assert_eq!(ticks(sum(n), 65_536), Some(n), "n = {n}");
        }
    }

    #[test]
    fn a_value_off_the_sum_has_no_tick_count() {
        assert_eq!(ticks(0.015, 65_536), None);
        assert_eq!(ticks(sum(100), 50), None, "beyond the limit");
    }

    #[test]
    fn hud_shows_the_drift_the_game_shows() {
        // The RE agent's simulation (double-precision products): 6000 ticks
        // read 59.99, 30000 read 299.98 (4:59:98), 60000 read 600.27.
        assert_eq!(hud_cs(sum(6000), true), 5999);
        assert_eq!(hud_cs(sum(30_000), true), 4 * 6000 + 59 * 100 + 98);
        assert_eq!(hud_cs(sum(60_000), true), 10 * 6000 + 27);
    }

    #[test]
    fn the_first_tick_reads_zero_at_53_bits() {
        assert_eq!(hud_cs(sum(1), true), 0);
        assert_eq!(hud_cs(0.0, false), 0);
    }

    #[test]
    fn precision_can_move_the_centiseconds_by_one() {
        // 0.01f * 100 is 1.0 in single precision and just under 1 in double.
        assert_eq!(hud_cs(0.01, false), 1);
        assert_eq!(hud_cs(0.01, true), 0);
    }

    #[test]
    fn hud_time_shows_only_a_started_race() {
        let mut state = zeroed_boxed();
        state.race_clock_bits = u32::MAX;
        assert_eq!(hud_time_cs(&state), None, "no clock");
        state.race_clock_bits = sum(6000).to_bits();
        assert_eq!(hud_time_cs(&state), None, "not started");
        state.race_clock_flags = TAS_RACE_CLOCK_STARTED;
        state.fpu_control_word = 0x027F;
        assert_eq!(hud_time_cs(&state), Some(5999));
    }

    #[test]
    fn control_word_precision() {
        assert!(!extended_precision(0x007F));
        assert!(extended_precision(0x027F));
        assert!(extended_precision(0x037F));
    }
}
