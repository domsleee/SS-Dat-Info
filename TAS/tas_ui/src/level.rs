//! Level codes (`FE` = Forest Easy, `AM` = Alpine Medium, `PE` = Practice)
//! and the `<code>-<race time>` default recording name built from them.

/// Level code from the DLL's published `level_id`: 0..8 = area*3 +
/// difficulty (area 0=Forest, 1=Alpine, 2=Village; diff 0=Easy, 1=Medium,
/// 2=Hard), 9 = Practice. `u32::MAX` = unknown/menu → None. Delegates to
/// tas_shared so the encoding has ONE definition — tas_test's pre-flight track
/// guard reads the same table.
pub fn level_code_from_id(level_id: u32) -> Option<&'static str> {
    tas_shared::level::code_from_id(level_id)
}

/// The live track code (e.g. `"FE"`), or `None` when the game has not (yet)
/// identified the level.
///
/// The only sanctioned way for the UI to ask "which track are we on". Reading
/// `state.level_id` directly is a bug: across a level change that word still
/// holds the PREVIOUS track until the scan catches up, so a raw read confidently
/// names the level you just left — which is how a recording gets saved into the
/// wrong track's folder and how the wrong start-line geometry times a run.
pub fn resolved_level_code(state: &tas_shared::TasSharedState) -> Option<&'static str> {
    tas_shared::resolved_level_id(state).and_then(level_code_from_id)
}

/// Which track a recording being SAVED belongs to.
///
/// Not the same question as "which track are we on": the engine stops its cycle
/// for its own post-run "Save attack player?" dialog, and a frozen cycle means
/// the level reads UNKNOWN — exactly when the user reaches for Save. A recording
/// belongs to the track it was RECORDED on, so fall back to the last track we
/// were confidently on. `live` still wins when we have it.
pub fn level_for_save<'a>(live: Option<&'a str>, last_known: Option<&'a str>) -> Option<&'a str> {
    live.or(last_known)
}

/// The in-race duration of a recording, in centiseconds (= ticks, since the
/// game runs at exactly 100 ticks/s). Measured from the gate (`first_moving`,
/// when the character leaves spawn) to the end of the recording. Returns
/// `None` if the recording never leaves spawn.
pub fn race_centiseconds(rec_coords: &[[f32; 3]], recorded_count: u32) -> Option<u32> {
    let gate = tas_shared::cont::detect_first_moving(rec_coords, recorded_count)?;
    Some(recorded_count.saturating_sub(gate))
}

/// Default file name for the Save dialog: `<code>-<time>` e.g.
/// `FE-5876.tasrec`. Time-only when the level is unknown (`5876.tasrec`), and
/// a timestamp when there is no race time either (an empty or unmoved
/// recording).
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
    fn race_centiseconds_measures_from_gate() {
        // 3 stationary spawn frames, then it moves at index 3; recorded 100.
        let mut coords = vec![[1.0f32, 2.0, 3.0]; 100];
        for c in coords.iter_mut().skip(3) {
            c[0] = 9.0;
        }
        assert_eq!(race_centiseconds(&coords, 100), Some(97));
    }

    #[test]
    fn race_centiseconds_none_when_unmoved() {
        let coords = vec![[1.0f32, 2.0, 3.0]; 50];
        assert_eq!(race_centiseconds(&coords, 50), None);
    }

    #[test]
    fn default_name_formats_level_and_time() {
        assert_eq!(
            default_recording_name(Some("FE"), Some(5876)),
            "FE-5876.tasrec"
        );
    }

    /// The post-run dialog freezes the cycle and the live level goes unknown
    /// right when the user clicks Save; the recording must keep its track.
    #[test]
    fn save_level_survives_the_post_run_dialog() {
        assert_eq!(level_for_save(None, Some("FM")), Some("FM"));
        assert_eq!(
            default_recording_name(level_for_save(None, Some("FM")), Some(5876)),
            "FM-5876.tasrec"
        );
    }

    #[test]
    fn save_level_prefers_live_when_known() {
        assert_eq!(level_for_save(Some("VH"), Some("FE")), Some("VH"));
        assert_eq!(level_for_save(Some("VH"), None), Some("VH"));
    }

    #[test]
    fn save_level_unknown_degrades_to_time_only() {
        assert_eq!(level_for_save(None, None), None);
        assert_eq!(
            default_recording_name(level_for_save(None, None), Some(5876)),
            "5876.tasrec"
        );
    }

    #[test]
    fn level_code_from_id_maps_all_ten_levels() {
        assert_eq!(level_code_from_id(0), Some("FE"));
        assert_eq!(level_code_from_id(4), Some("AM"));
        assert_eq!(level_code_from_id(8), Some("VH"));
        assert_eq!(level_code_from_id(9), Some("PE"));
        assert_eq!(level_code_from_id(10), None);
        assert_eq!(level_code_from_id(0xFFFF_FFFF), None);
    }

    #[test]
    fn default_name_falls_back_to_timestamp() {
        let n = default_recording_name(None, None);
        assert!(n.ends_with(".tasrec"));
        assert!(n.contains('-') || n.contains('_'));
    }
}
