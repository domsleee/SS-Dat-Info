//! Steering pattern DSL — ported from _drift_regression_suite.lua buildSteps()
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
    pub name: String,
    pub mask: u8,
    pub stop_tick: u32, // cumulative tick at which this step ends
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

fn char_to_name(ch: char) -> &'static str {
    match ch {
        'L' => "LEFT",
        'R' => "RIGHT",
        'U' => "UP",
        'D' => "DOWN",
        'J' => "JUMP",
        'S' => "SHIFT",
        'N' => "NEUTRAL",
        _ => "UNKNOWN",
    }
}

/// Build steps from a pattern string with configurable hold/gap ticks.
pub fn build_from_pattern(pattern: &str, hold_ticks: u32, gap_ticks: u32) -> Vec<PatternStep> {
    let mut steps = Vec::new();
    let mut stop = 0u32;

    for (i, ch) in pattern.chars().enumerate() {
        let mask = char_to_mask(ch);
        let name_prefix = char_to_name(ch);
        stop += hold_ticks;
        steps.push(PatternStep {
            name: format!("{}{}", name_prefix, i + 1),
            mask,
            stop_tick: stop,
        });
        if gap_ticks > 0 && i + 1 < pattern.len() {
            stop += gap_ticks;
            steps.push(PatternStep {
                name: format!("GAP{}", i + 1),
                mask: 0x00,
                stop_tick: stop,
            });
        }
    }
    steps
}

/// Build steps from explicit step definitions (like Lua's case.steps).
pub fn build_from_explicit(defs: &[(& str, u8, u32)]) -> Vec<PatternStep> {
    let mut steps = Vec::new();
    let mut stop = 0u32;
    for (name, mask, ticks) in defs {
        stop += ticks;
        steps.push(PatternStep {
            name: name.to_string(),
            mask: *mask,
            stop_tick: stop,
        });
    }
    steps
}

/// Total duration of a step sequence in ticks.
pub fn total_ticks(steps: &[PatternStep]) -> u32 {
    steps.last().map(|s| s.stop_tick).unwrap_or(0)
}

/// Generate an input log (tick -> mask) from pattern steps.
/// Returns a Vec of length = total_ticks.
pub fn generate_input_log(steps: &[PatternStep]) -> Vec<u8> {
    let total = total_ticks(steps) as usize;
    let mut log = vec![0u8; total];
    let mut prev_stop = 0usize;
    for step in steps {
        let end = step.stop_tick as usize;
        for tick in prev_stop..end.min(total) {
            log[tick] = step.mask;
        }
        prev_stop = end;
    }
    log
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(steps[1].name, "GAP1");
        assert_eq!(steps[1].mask, 0);
        assert_eq!(steps[1].stop_tick, 66);
        assert_eq!(steps[2].stop_tick, 122);
    }

    #[test]
    fn test_input_log_generation() {
        let steps = build_from_pattern("LR", 4, 0);
        let log = generate_input_log(&steps);
        assert_eq!(log.len(), 8);
        assert_eq!(log[0..4], [0x01, 0x01, 0x01, 0x01]);
        assert_eq!(log[4..8], [0x02, 0x02, 0x02, 0x02]);
    }

    #[test]
    fn test_explicit_steps() {
        let steps = build_from_explicit(&[
            ("LEFT1", 0x01, 72),
            ("RIGHT2", 0x02, 36),
            ("LEFT3", 0x01, 72),
        ]);
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0].stop_tick, 72);
        assert_eq!(steps[1].stop_tick, 108);
        assert_eq!(steps[2].stop_tick, 180);
    }
}
