//! Level identity + default recording-name helpers (game-awareness 1.2/1.4).
//!
//! The game stores the active track as a resource path the engine loads from,
//! e.g. `"Data/Levels/Forest/Tracks/Easy/Replays/Replay.dat"` (confirmed in
//! memory — the Display_Config helper hooks the replay-save site and reads this
//! exact string from `ebx`). The path encodes everything we need for a level
//! code: the **area** (Forest/Alpine/Village) and the **category + difficulty**
//! (Tracks/Halfpipe/Ramp × Easy/Medium/Hard).
//!
//! The user's naming convention is `<code>-<time>` e.g. `FE-5876` = Forest-Easy,
//! 58.76 s. So a code is `{AREA}{DIFF}` for the main Tracks, with a category
//! marker inserted for the Halfpipe/Ramp variants (`FPE`, `FRE`).
//!
//! NOTE: live detection (publishing the current path into shared state from the
//! DLL) is not wired yet — it needs a load-time hook like the helper's
//! `saveReplayTimestamp`. Until then `parse_level_code` is exercised by tests
//! and ready to format a name the moment the DLL starts publishing the path.

/// Area folder → single-letter prefix. Forest/Alpine/Village are the three
/// race areas; anything else (Practice/Special) has no stable convention yet.
fn area_initial(area: &str) -> Option<char> {
    match area.to_ascii_lowercase().as_str() {
        "forest" => Some('F'),
        "alpine" => Some('A'),
        "village" => Some('V'),
        _ => None,
    }
}

/// Difficulty folder → single-letter suffix.
fn difficulty_initial(diff: &str) -> Option<char> {
    match diff.to_ascii_lowercase().as_str() {
        "easy" => Some('E'),
        "medium" => Some('M'),
        "hard" => Some('H'),
        _ => None,
    }
}

/// Category folder → optional middle marker. The main `Tracks` get no marker
/// (so Forest/Tracks/Easy = `FE`), Halfpipe = `P` (pipe), Ramp = `R` — chosen so
/// no marker collides with a difficulty letter at the end.
fn category_marker(category: &str) -> Option<&'static str> {
    match category.to_ascii_lowercase().as_str() {
        "tracks" => Some(""),
        "halfpipe" => Some("P"),
        "ramp" => Some("R"),
        _ => None,
    }
}

/// Derive a short level code (e.g. `FE`, `AME`, `FPE`) from a game resource
/// path that contains a `Levels/<Area>/<Category>/<Difficulty>/...` segment.
/// Accepts both `/` and `\` separators. Returns `None` when the path doesn't
/// look like a known race track.
///
/// `allow(dead_code)`: ready and tested, but only called once the DLL starts
/// publishing the live track path (a load-time hook, like the helper's
/// `saveReplayTimestamp`). Until then `default_recording_name` is passed `None`.
#[allow(dead_code)]
pub fn parse_level_code(path: &str) -> Option<String> {
    // Split on either separator, drop empties.
    let segs: Vec<&str> = path
        .split(['/', '\\'])
        .filter(|s| !s.is_empty())
        .collect();
    // Find the "Levels" anchor; area/category/difficulty are the next three.
    let li = segs
        .iter()
        .position(|s| s.eq_ignore_ascii_case("levels"))?;
    let area = segs.get(li + 1)?;
    let category = segs.get(li + 2)?;
    let difficulty = segs.get(li + 3)?;

    let a = area_initial(area)?;
    let mark = category_marker(category)?;
    let d = difficulty_initial(difficulty)?;
    Some(format!("{}{}{}", a, mark, d))
}

/// The in-race duration of a recording, in centiseconds (= ticks, since the
/// game runs at exactly 100 ticks/s). Measured from the gate (`first_moving`,
/// when the character leaves spawn) to the end of the recording — that matches
/// the on-screen race timer, which starts at the start line, not at the F5
/// spawn/countdown. Returns `None` if the recording never leaves spawn.
pub fn race_centiseconds(rec_coords: &[[f32; 3]], recorded_count: u32) -> Option<u32> {
    let gate = tas_shared::cont::detect_first_moving(rec_coords, recorded_count)?;
    Some(recorded_count.saturating_sub(gate))
}

/// Default file name for the Save dialog. Convention `<code>-<time>` e.g.
/// `FE-5876.tasrec`. Falls back gracefully: time-only when the level is unknown
/// (`5876.tasrec`), and a timestamp when there's no race time either (an empty
/// or unmoved recording).
pub fn default_recording_name(level: Option<&str>, race_cs: Option<u32>) -> String {
    match (level, race_cs) {
        (Some(code), Some(cs)) => format!("{}-{}.tasrec", code, cs),
        (None, Some(cs)) => format!("{}.tasrec", cs),
        _ => format!("{}.tasrec", chrono::Local::now().format("%Y-%m-%d_%H%M%S")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forest_easy_track() {
        assert_eq!(
            parse_level_code("Data/Levels/Forest/Tracks/Easy/Replays/Replay.dat").as_deref(),
            Some("FE")
        );
    }

    #[test]
    fn parse_all_areas_and_difficulties() {
        assert_eq!(parse_level_code("Levels/Alpine/Tracks/Medium/x").as_deref(), Some("AM"));
        assert_eq!(parse_level_code("Levels/Village/Tracks/Hard/x").as_deref(), Some("VH"));
        assert_eq!(parse_level_code("Levels/Forest/Tracks/Easy/x").as_deref(), Some("FE"));
    }

    #[test]
    fn parse_halfpipe_and_ramp_get_category_marker() {
        assert_eq!(parse_level_code("Levels/Forest/Halfpipe/Easy/x").as_deref(), Some("FPE"));
        assert_eq!(parse_level_code("Levels/Alpine/Ramp/Hard/x").as_deref(), Some("ARH"));
    }

    #[test]
    fn parse_accepts_backslashes_and_mixed_case() {
        assert_eq!(
            parse_level_code(r"data\levels\forest\tracks\easy\Times.txt").as_deref(),
            Some("FE")
        );
    }

    #[test]
    fn parse_rejects_unknown_or_malformed() {
        assert_eq!(parse_level_code("Levels/Practice/Foo/Bar"), None);
        assert_eq!(parse_level_code("Levels/Forest/Tracks"), None); // no difficulty
        assert_eq!(parse_level_code("nothing/useful/here"), None);
        assert_eq!(parse_level_code(""), None);
    }

    #[test]
    fn race_centiseconds_measures_from_gate() {
        // 3 stationary spawn frames, then it moves at index 3; recorded 100.
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 100];
        for c in coords.iter_mut().skip(3) {
            c[0] = 9.0;
        }
        // gate = 3, so race = 100 - 3 = 97 cs.
        assert_eq!(race_centiseconds(&coords, 100), Some(97));
    }

    #[test]
    fn race_centiseconds_none_when_unmoved() {
        let coords = vec![[1.0f32, 2.0, 3.0]; 50];
        assert_eq!(race_centiseconds(&coords, 50), None);
    }

    #[test]
    fn default_name_formats_level_and_time() {
        assert_eq!(default_recording_name(Some("FE"), Some(5876)), "FE-5876.tasrec");
    }

    #[test]
    fn default_name_time_only_when_level_unknown() {
        assert_eq!(default_recording_name(None, Some(5876)), "5876.tasrec");
    }

    #[test]
    fn default_name_falls_back_to_timestamp() {
        let n = default_recording_name(None, None);
        assert!(n.ends_with(".tasrec"));
        // timestamp form contains the date separator we used
        assert!(n.contains('-') || n.contains('_'));
    }
}
