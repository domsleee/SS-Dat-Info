//! Conversion core for input editing: `input_log` bitmask buffer <-> a flat
//! list of held inputs (`InputEvent`) <-> a frame-based, TMInterface-style
//! text script.
//!
//! This module is pure (no egui, no shared memory) so it can be unit-tested
//! in isolation. The UI layers (timeline drag-edit, the `.tas` text file)
//! both funnel through `InputEvent` as the single source of truth.
//!
//! Text format mirrors TMInterface grammar but is **frame-based** (ticks),
//! not timer-relative seconds — SSB allows inputs before the run timer
//! starts, which would force negative timestamps in a timer-relative scheme.
//! 100 ticks per second.
//!
//! ```text
//! # units = ticks/frames (100 per second)
//! # timer starts @ tick 900 (T=0); lower ticks run BEFORE the timer
//!
//! 324-372 press left
//! 473-727 press up
//! ```

use tas_shared::input_bits;

/// One held input: `bit` is active over the half-open tick range
/// `[start, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InputEvent {
    pub bit: u8,
    pub start: u32,
    pub end: u32,
}

/// A line we couldn't parse. Surfaced to the user (count + first reason)
/// rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub text: String,
    pub reason: String,
}

/// (bit, TMInterface-style keyword). `press <keyword>`.
const KEYS: &[(u8, &str)] = &[
    (input_bits::LEFT, "left"),
    (input_bits::RIGHT, "right"),
    (input_bits::UP, "up"),
    (input_bits::DOWN, "down"),
    (input_bits::JUMP, "jump"),
    (input_bits::SHIFT, "shift"),
];

fn keyword(bit: u8) -> Option<&'static str> {
    KEYS.iter().find(|(b, _)| *b == bit).map(|(_, k)| *k)
}

fn bit_of(keyword: &str) -> Option<u8> {
    KEYS.iter()
        .find(|(_, k)| k.eq_ignore_ascii_case(keyword))
        .map(|(b, _)| *b)
}

fn key_mask() -> u8 {
    KEYS.iter().fold(0u8, |m, (b, _)| m | b)
}

/// Decode an `input_log` buffer into run-length `InputEvent`s, one run per
/// contiguous hold of each key. Sorted by (start, bit).
pub fn runs_from_log(log: &[u8], recorded_count: u32) -> Vec<InputEvent> {
    let n = (recorded_count as usize).min(log.len());
    let mut out = Vec::new();
    for &(bit, _) in KEYS {
        let mut start: Option<u32> = None;
        for (t, &mask) in log.iter().enumerate().take(n) {
            let on = mask & bit != 0;
            match (on, start) {
                (true, None) => start = Some(t as u32),
                (false, Some(s)) => {
                    out.push(InputEvent { bit, start: s, end: t as u32 });
                    start = None;
                }
                _ => {}
            }
        }
        if let Some(s) = start {
            out.push(InputEvent { bit, start: s, end: n as u32 });
        }
    }
    out.sort_by(|a, b| a.start.cmp(&b.start).then(a.bit.cmp(&b.bit)));
    out
}

/// Rewrite the key bits of `log` over `[0, recorded_count)` from `events`.
/// Only the six input bits are touched; any other bits in a byte are
/// preserved. Ranges are clamped to `recorded_count`.
pub fn apply_events_to_log(log: &mut [u8], recorded_count: u32, events: &[InputEvent]) {
    let n = (recorded_count as usize).min(log.len());
    let mask = key_mask();
    for byte in log.iter_mut().take(n) {
        *byte &= !mask;
    }
    for ev in events {
        let s = (ev.start as usize).min(n);
        let e = (ev.end as usize).min(n);
        for byte in log.iter_mut().take(e).skip(s) {
            *byte |= ev.bit;
        }
    }
}

/// Serialize events to the frame-based text script. `timer_start` is the
/// tick of T=0 (from `detect_first_moving`); inputs before it get a
/// `# pre-timer` tag for readability.
pub fn events_to_script(events: &[InputEvent], timer_start: u32) -> String {
    let mut evs = events.to_vec();
    evs.sort_by(|a, b| a.start.cmp(&b.start).then(a.bit.cmp(&b.bit)));

    let mut s = String::new();
    s.push_str("# units = ticks/frames (100 per second)\n");
    s.push_str(&format!(
        "# timer starts @ tick {} (T=0); lower ticks run BEFORE the timer\n\n",
        timer_start
    ));
    for ev in &evs {
        if let Some(kw) = keyword(ev.bit) {
            let tag = if ev.start < timer_start { "   # pre-timer" } else { "" };
            s.push_str(&format!("{}-{} press {}{}\n", ev.start, ev.end, kw, tag));
        }
    }
    s
}

/// Parse the text script. Returns the valid events plus a list of lines that
/// failed (so the UI can show "N line(s) ignored"). Comments start with `#`;
/// blank lines are skipped.
pub fn parse_script(text: &str) -> (Vec<InputEvent>, Vec<ParseError>) {
    let mut events = Vec::new();
    let mut errors = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        match parse_line(line) {
            Ok(ev) => events.push(ev),
            Err(reason) => errors.push(ParseError {
                line: i + 1,
                text: raw.to_string(),
                reason,
            }),
        }
    }
    (events, errors)
}

/// Parse a single `<start>-<end> press <key>` line (comment already stripped).
fn parse_line(line: &str) -> Result<InputEvent, String> {
    let mut it = line.split_whitespace();
    let range = it.next().ok_or("empty line")?;
    let verb = it.next().ok_or("missing 'press'")?;
    let key = it.next().ok_or("missing key")?;
    if it.next().is_some() {
        return Err("unexpected trailing text".to_string());
    }
    if !verb.eq_ignore_ascii_case("press") {
        return Err(format!("expected 'press', got '{}'", verb));
    }
    let (a, b) = range
        .split_once('-')
        .ok_or("range must be 'start-end'")?;
    let start: u32 = a
        .trim()
        .parse()
        .map_err(|_| format!("bad start tick '{}'", a))?;
    let end: u32 = b.trim().parse().map_err(|_| format!("bad end tick '{}'", b))?;
    if end <= start {
        return Err(format!("end ({}) must be greater than start ({})", end, start));
    }
    let bit = bit_of(key).ok_or_else(|| format!("unknown key '{}'", key))?;
    Ok(InputEvent { bit, start, end })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tas_shared::input_bits::{DOWN, JUMP, LEFT, SHIFT, UP};

    fn log_with(events: &[InputEvent], n: u32) -> Vec<u8> {
        let mut log = vec![0u8; n as usize];
        apply_events_to_log(&mut log, n, events);
        log
    }

    #[test]
    fn runs_round_trip_through_log() {
        let events = vec![
            InputEvent { bit: LEFT, start: 10, end: 20 },
            InputEvent { bit: UP, start: 5, end: 30 },
            InputEvent { bit: UP, start: 40, end: 50 },
        ];
        let log = log_with(&events, 64);
        let mut back = runs_from_log(&log, 64);
        let mut expected = events.clone();
        back.sort_by(|a, b| a.start.cmp(&b.start).then(a.bit.cmp(&b.bit)));
        expected.sort_by(|a, b| a.start.cmp(&b.start).then(a.bit.cmp(&b.bit)));
        assert_eq!(back, expected);
    }

    #[test]
    fn run_to_recorded_count_when_held_to_end() {
        // A hold that never releases closes at recorded_count, not buffer len.
        let events = vec![InputEvent { bit: DOWN, start: 8, end: 16 }];
        let mut log = vec![0u8; 64];
        apply_events_to_log(&mut log, 16, &events);
        let back = runs_from_log(&log, 16);
        assert_eq!(back, vec![InputEvent { bit: DOWN, start: 8, end: 16 }]);
    }

    #[test]
    fn apply_only_touches_key_bits_and_clamps() {
        let mut log = vec![0u8; 10];
        log[0] = 0x80; // a non-input bit we must preserve
        apply_events_to_log(&mut log, 10, &[InputEvent { bit: LEFT, start: 0, end: 100 }]);
        assert_eq!(log[0], 0x80 | LEFT); // preserved + set
        assert_eq!(log[9], LEFT); // clamped to recorded_count, still set
    }

    #[test]
    fn script_is_frame_based_with_pre_timer_tag() {
        let events = vec![
            InputEvent { bit: LEFT, start: 100, end: 200 }, // before T=0
            InputEvent { bit: UP, start: 950, end: 1000 },  // after T=0
        ];
        let s = events_to_script(&events, 900);
        assert!(s.contains("100-200 press left   # pre-timer"));
        assert!(s.contains("950-1000 press up\n"));
        assert!(!s.contains("950-1000 press up   # pre-timer"));
    }

    #[test]
    fn script_round_trips_through_parse() {
        let events = vec![
            InputEvent { bit: JUMP, start: 12, end: 18 },
            InputEvent { bit: SHIFT, start: 3, end: 9 },
        ];
        let s = events_to_script(&events, 0);
        let (parsed, errs) = parse_script(&s);
        assert!(errs.is_empty(), "unexpected parse errors: {:?}", errs);
        let mut a = parsed;
        let mut b = events;
        a.sort_by(|x, y| x.start.cmp(&y.start));
        b.sort_by(|x, y| x.start.cmp(&y.start));
        assert_eq!(a, b);
    }

    #[test]
    fn parse_collects_bad_lines() {
        let text = "10-20 press left\ngarbage here\n30-25 press up\n50-60 press flip\n# comment\n";
        let (events, errs) = parse_script(text);
        assert_eq!(events.len(), 1); // only the first line is valid
        assert_eq!(errs.len(), 3); // garbage, end<=start, unknown key
        assert_eq!(errs[0].line, 2);
        assert!(errs[1].reason.contains("greater than start"));
        assert!(errs[2].reason.contains("unknown key"));
    }

    #[test]
    fn parse_ignores_comments_and_blanks() {
        let text = "# header\n\n  \n10-20 press down  # trailing note\n";
        let (events, errs) = parse_script(text);
        assert!(errs.is_empty());
        assert_eq!(events, vec![InputEvent { bit: DOWN, start: 10, end: 20 }]);
    }
}
