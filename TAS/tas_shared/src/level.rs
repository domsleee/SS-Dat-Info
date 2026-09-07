//! Track identification shared by tas_ui (the status chip) and tas_test (the
//! pre-flight guard below). Single source of truth for the `level_id` encoding.

/// The nine main Time-Attack Tracks plus the start menu's Practice run,
/// indexed by `level_id` = area*3 + difficulty (area 0=Forest, 1=Alpine,
/// 2=Village, 3=Practice; diff 0=Easy, 1=Medium, 2=Hard). Practice has
/// exactly one difficulty on disk (Tracks/Easy), so its only id is
/// 3*3+0 = 9 = "PE".
pub const CODES: [&str; 10] = ["FE", "FM", "FH", "AE", "AM", "AH", "VE", "VM", "VH", "PE"];

/// Level code from the DLL's published `level_id`. `None` = unknown: either
/// the menu, or a mode outside the table (Halfpipe, Ramp, ...). The DLL
/// publishes 0xFFFFFFFF for all of those.
pub fn code_from_id(level_id: u32) -> Option<&'static str> {
    CODES.get(level_id as usize).copied()
}

/// The level a `.tasrec` declares via its filename, e.g.
/// `FE-tremendous.tasrec` -> `FE`. Recordings are named `<CODE>-<name>`, so
/// the file itself says which track it can be replayed on. `None` when the
/// name carries no recognised code.
pub fn code_from_recording_name(path: &str) -> Option<&'static str> {
    let stem = path.rsplit(['/', '\\']).next().unwrap_or(path);
    let head = stem.split(['-', '_', '.']).next().unwrap_or("");
    CODES.iter().find(|c| c.eq_ignore_ascii_case(head)).copied()
}

/// Pre-flight check: can `recording_path` be replayed on the live track?
///
/// Replaying a recording on the wrong track is not a subtle failure — the
/// spawn is somewhere else entirely, so the harness's start-position matcher
/// can never hit its target and simply burns its whole retry budget (~22s
/// per attempt) before giving up. Catching it up front turns a ~10 minute
/// mystery timeout into an immediate, accurate error.
///
/// Returns `Err(message)` only when the live level is KNOWN and different.
/// An unknown live level (menu, mid-teardown, scan not yet run) cannot prove
/// a mismatch, so it is allowed through.
pub fn check_recording_matches_live(
    recording_path: &str,
    live_level_id: u32,
) -> Result<(), String> {
    let (Some(want), Some(live)) = (
        code_from_recording_name(recording_path),
        code_from_id(live_level_id),
    ) else {
        return Ok(());
    };
    if want == live {
        Ok(())
    } else {
        Err(format!(
            "recording is for track {} but the game is on {} (level_id={}). \
                 Replaying it here can never match the recorded spawn — the start \
                 matcher would exhaust its retries and time out. Navigate to {} first.",
            want, live, live_level_id, want
        ))
    }
}

#[cfg(test)]
mod level_tests {
    use super::*;

    #[test]
    fn id_maps_to_code_and_unknown_is_none() {
        assert_eq!(code_from_id(0), Some("FE"));
        assert_eq!(code_from_id(8), Some("VH"));
        assert_eq!(code_from_id(9), Some("PE"), "practice is a level now");
        // Halfpipe / Ramp / menu all publish 0xFFFFFFFF.
        assert_eq!(code_from_id(u32::MAX), None);
        assert_eq!(code_from_id(10), None);
    }

    #[test]
    fn recording_name_declares_its_track() {
        assert_eq!(code_from_recording_name("FE-tremendous.tasrec"), Some("FE"));
        assert_eq!(
            code_from_recording_name("TAS/recordings/FE-10065.tasrec"),
            Some("FE")
        );
        assert_eq!(code_from_recording_name(r"C:\x\VH_run.tasrec"), Some("VH"));
        assert_eq!(code_from_recording_name("scratch.tasrec"), None);
    }

    #[test]
    fn mismatch_is_rejected_and_unknown_is_allowed() {
        // The real failure this guards: FE recording, game on another track.
        let err = check_recording_matches_live("FE-tremendous.tasrec", 4).unwrap_err();
        assert!(err.contains("FE"), "{}", err);
        assert!(err.contains("AM"), "{}", err);

        // Same track is fine.
        assert!(check_recording_matches_live("FE-tremendous.tasrec", 0).is_ok());
        // Unknown live level cannot PROVE a mismatch — must not hard-fail.
        assert!(check_recording_matches_live("FE-tremendous.tasrec", u32::MAX).is_ok());
        // Unrecognised recording name carries no claim either.
        assert!(check_recording_matches_live("scratch.tasrec", 0).is_ok());
    }
}
