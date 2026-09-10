//! Steering pattern DSL.
//!
//! Pattern string: sequence of characters mapped to input masks.
//!   L = LEFT (0x01), R = RIGHT (0x02), U = UP (0x04), D = DOWN (0x08)
//!   J = JUMP (0x10), S = SHIFT (0x20), N = neutral (0x00)
//!
//! Each character becomes a "hold" phase of `hold_ticks` duration.
//! Optional `gap_ticks` neutral phases are inserted between holds.

use tas_shared::input_bits;

/// A single step in a steering pattern.
#[derive(Debug, Clone)]
pub struct PatternStep {
    pub mask: u8,
    /// Cumulative tick at which this step ends.
    pub stop_tick: u32,
}

/// Default hold duration per pattern character (ticks).
pub const DEFAULT_HOLD_TICKS: u32 = 56;
/// Default gap duration between pattern characters (ticks).
pub const DEFAULT_GAP_TICKS: u32 = 0;

fn char_to_mask(ch: char) -> u8 {
    match ch {
        'L' => input_bits::LEFT,
        'R' => input_bits::RIGHT,
        'U' => input_bits::UP,
        'D' => input_bits::DOWN,
        'J' => input_bits::JUMP,
        'S' => input_bits::SHIFT,
        'N' => 0x00,
        _ => panic!("unsupported pattern token: {}", ch),
    }
}

/// Build steps from a pattern string with configurable hold/gap ticks.
pub fn build_from_pattern(pattern: &str, hold_ticks: u32, gap_ticks: u32) -> Vec<PatternStep> {
    let mut steps = Vec::new();
    let mut stop = 0u32;

    for (i, ch) in pattern.chars().enumerate() {
        let mask = char_to_mask(ch);
        stop += hold_ticks;
        steps.push(PatternStep {
            mask,
            stop_tick: stop,
        });
        if gap_ticks > 0 && i + 1 < pattern.len() {
            stop += gap_ticks;
            steps.push(PatternStep {
                mask: 0x00,
                stop_tick: stop,
            });
        }
    }
    steps
}

/// Build steps from explicit `(mask, ticks)` holds.
pub fn build_from_explicit(defs: &[(u8, u32)]) -> Vec<PatternStep> {
    let mut steps = Vec::new();
    let mut stop = 0u32;
    for &(mask, ticks) in defs {
        stop += ticks;
        steps.push(PatternStep {
            mask,
            stop_tick: stop,
        });
    }
    steps
}

/// `steps` followed by `ticks` of neutral input.
pub fn with_neutral_tail(mut steps: Vec<PatternStep>, ticks: u32) -> Vec<PatternStep> {
    let stop_tick = total_ticks(&steps) + ticks;
    steps.push(PatternStep {
        mask: 0x00,
        stop_tick,
    });
    steps
}

/// Total duration of a step sequence in ticks.
pub fn total_ticks(steps: &[PatternStep]) -> u32 {
    steps.last().map(|s| s.stop_tick).unwrap_or(0)
}

/// Compare the captured sequence with the requested 100 Hz input schedule.
/// Ignore only the initial arm-to-drive offset; every subsequent edge, including
/// the final release, must arrive within tolerance. Call after capturing a tail.
pub fn verify_capture(steps: &[PatternStep], log: &[u8], tolerance: u32) -> Result<(), String> {
    let mut expected = Vec::new();
    let mut tick = 0;
    let mut mask = 0;
    for step in steps {
        if step.stop_tick <= tick {
            return Err("Input schedule contains an empty or reversed hold".into());
        }
        if step.mask != mask {
            expected.push((tick, step.mask));
            mask = step.mask;
        }
        tick = step.stop_tick;
    }
    if mask != 0 {
        expected.push((tick, 0));
    }
    let mut actual = Vec::new();
    mask = 0;
    for (tick, &next) in log.iter().enumerate() {
        if next != mask {
            actual.push((tick as u32, next));
            mask = next;
        }
    }
    if expected.is_empty() || actual.is_empty() {
        return Err(format!(
            "Input edges: expected {expected:?}, captured {actual:?}"
        ));
    }
    let offset = actual[0].0 as i64 - expected[0].0 as i64;
    // HID/game dispatch can split a multi-key change across adjacent ticks.
    // Compare each key's edges, sharing ONE time origin across all keys: this
    // permits delivery skew, not dropped keys, extra presses or shortened holds.
    for bit in 0..8 {
        let edges = |changes: &[(u32, u8)]| {
            let mut previous = false;
            changes
                .iter()
                .filter_map(|&(tick, mask)| {
                    let down = mask & (1 << bit) != 0;
                    let changed = down != previous;
                    previous = down;
                    changed.then_some((tick, down))
                })
                .collect::<Vec<_>>()
        };
        let wanted = edges(&expected);
        let captured = edges(&actual);
        if wanted.len() != captured.len() {
            return Err(format!(
                "Key bit {bit}: expected edges {wanted:?}, captured {captured:?}"
            ));
        }
        for ((want_tick, want_down), (got_tick, got_down)) in wanted.iter().zip(&captured) {
            let skew = (*got_tick as i64 - *want_tick as i64 - offset).unsigned_abs();
            if want_down != got_down || skew > u64::from(tolerance) {
                return Err(format!("Key bit {bit}: expected ({want_tick}, {want_down}), captured ({got_tick}, {got_down}), aligned skew {skew} ticks"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_checks_schedule_not_just_reproducibility() {
        let steps = build_from_explicit(&[
            (input_bits::LEFT | input_bits::SHIFT, 72),
            (0, 20),
            (input_bits::RIGHT, 36),
        ]);
        let mut log = vec![0; 5];
        log.extend(vec![input_bits::LEFT | input_bits::SHIFT; 72]);
        log.extend(vec![0; 20]);
        log.extend(vec![input_bits::RIGHT; 36]);
        log.extend(vec![0; 10]);
        assert!(verify_capture(&steps, &log, 12).is_ok());
        let mut split_report = log.clone();
        split_report[5] = input_bits::LEFT;
        assert!(verify_capture(&steps, &split_report, 12).is_ok());
        split_report[6..19].fill(input_bits::LEFT);
        assert!(verify_capture(&steps, &split_report, 12).is_err());
        let mut extra_press = log.clone();
        extra_press[85] = input_bits::JUMP;
        assert!(verify_capture(&steps, &extra_press, 12).is_err());
        let mut lost_modifier = log.clone();
        for mask in &mut lost_modifier {
            *mask &= !input_bits::SHIFT;
        }
        assert!(verify_capture(&steps, &lost_modifier, 12).is_err());
        let mut timed_out = log.clone();
        timed_out[55..77].fill(0);
        assert!(verify_capture(&steps, &timed_out, 12).is_err());
        assert!(verify_capture(&steps, &log[..133], 12).is_err());
        assert!(verify_capture(&steps, &[], 12).is_err());
    }

    #[test]
    fn test_simple_pattern() {
        let steps = build_from_pattern("LR", 56, 0);
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].mask, input_bits::LEFT);
        assert_eq!(steps[0].stop_tick, 56);
        assert_eq!(steps[1].mask, input_bits::RIGHT);
        assert_eq!(steps[1].stop_tick, 112);
    }

    #[test]
    fn test_pattern_with_gap() {
        let steps = build_from_pattern("LR", 56, 10);
        assert_eq!(steps.len(), 3); // L, GAP, R
        assert_eq!(steps[1].mask, 0);
        assert_eq!(steps[1].stop_tick, 66);
        assert_eq!(steps[2].stop_tick, 122);
    }

    #[test]
    fn test_explicit_steps() {
        let steps = build_from_explicit(&[(0x01, 72), (0x02, 36), (0x01, 72)]);
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].stop_tick, 72);
        assert_eq!(steps[1].stop_tick, 108);
        assert_eq!(steps[2].stop_tick, 180);
    }

    #[test]
    fn test_neutral_tail_extends_the_sequence() {
        let steps = with_neutral_tail(build_from_pattern("LR", 56, 0), 100);
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[2].mask, 0);
        assert_eq!(total_ticks(&steps), 212);
        assert_eq!(total_ticks(&with_neutral_tail(vec![], 5)), 5);
    }

    #[test]
    fn test_total_ticks_empty() {
        assert_eq!(total_ticks(&[]), 0);
    }

    #[test]
    fn test_total_ticks_matches_last_stop() {
        let steps = build_from_pattern("LRL", 56, 0);
        assert_eq!(total_ticks(&steps), 168); // 3 * 56
    }

    #[test]
    fn test_total_ticks_with_gaps() {
        let steps = build_from_pattern("LR", 56, 10);
        // L(56) + GAP(10) + R(56) = 122
        assert_eq!(total_ticks(&steps), 122);
    }

    #[test]
    fn test_neutral_pattern() {
        let steps = build_from_pattern("N", 10, 0);
        assert_eq!(steps.len(), 1);
        assert_eq!(steps[0].mask, 0x00);
    }

    #[test]
    fn test_all_directions() {
        let steps = build_from_pattern("LUDRSJ", 1, 0);
        assert_eq!(steps.len(), 6);
        assert_eq!(steps[0].mask, input_bits::LEFT);
        assert_eq!(steps[1].mask, input_bits::UP);
        assert_eq!(steps[2].mask, input_bits::DOWN);
        assert_eq!(steps[3].mask, input_bits::RIGHT);
        assert_eq!(steps[4].mask, input_bits::SHIFT);
        assert_eq!(steps[5].mask, input_bits::JUMP);
    }

    #[test]
    fn test_gap_not_after_last() {
        let steps = build_from_pattern("L", 56, 10);
        assert_eq!(steps.len(), 1);
    }

    #[test]
    #[should_panic(expected = "unsupported pattern token")]
    fn test_invalid_token_panics() {
        build_from_pattern("X", 10, 0);
    }
}
