//! The rider stamp recordings carry: which character is on the board and in
//! which stance. Both change the physics, so a replay under a different rider
//! cannot line up.

/// Selectable riders published in `TasSharedState::rider_character` (v42).
pub const TAS_CHARACTER_UNKNOWN: u32 = 0;
pub const TAS_CHARACTER_KEITH: u32 = 1;
pub const TAS_CHARACTER_VINCENT: u32 = 2;
pub const TAS_CHARACTER_AKIKO: u32 = 3;
pub const TAS_CHARACTER_KARL: u32 = 4;
pub const TAS_CHARACTER_MIKE: u32 = 5;
pub const TAS_CHARACTER_ULRIKA: u32 = 6;
pub const TAS_CHARACTER_OTHER: u32 = 7;

/// Inverse of `character_name` (case-insensitive); unknown names map to
/// `TAS_CHARACTER_OTHER`, an empty name to `TAS_CHARACTER_UNKNOWN`.
pub fn character_id_from_name(name: &str) -> u32 {
    let name = name.trim();
    if name.is_empty() {
        return TAS_CHARACTER_UNKNOWN;
    }
    for id in TAS_CHARACTER_KEITH..=TAS_CHARACTER_OTHER {
        if character_name(id).eq_ignore_ascii_case(name) {
            return id;
        }
    }
    TAS_CHARACTER_OTHER
}

pub fn character_name(id: u32) -> &'static str {
    match id {
        TAS_CHARACTER_KEITH => "Keith",
        TAS_CHARACTER_VINCENT => "Vincent",
        TAS_CHARACTER_AKIKO => "Akiko",
        TAS_CHARACTER_KARL => "Karl",
        TAS_CHARACTER_MIKE => "Mike",
        TAS_CHARACTER_ULRIKA => "Ulrika",
        TAS_CHARACTER_OTHER => "other",
        _ => "unknown",
    }
}

/// Stances (`TasSharedState::rider_stance`): the value the game's menu keeps
/// in its setup object and builds every rider from.
pub const TAS_STANCE_REGULAR: u32 = 0; // left-foot icon, the game's default
pub const TAS_STANCE_GOOFY: u32 = 1; // right-foot icon

pub fn stance_name(stance: u32) -> &'static str {
    match stance {
        TAS_STANCE_REGULAR => "regular",
        TAS_STANCE_GOOFY => "goofy",
        _ => "unknown",
    }
}

/// Canonical rider stamp, e.g. `Vincent · goofy`. The physics depend on the
/// character (a Keith recording does not line up under Vincent) and on the
/// stance (the board does not matter), so this travels with recordings and
/// history entries and is compared against the live loadout. `None` until
/// the character is known; the stance is omitted when unknown so an older
/// stamp still compares by character.
pub fn rider_label(character: u32, stance: u32) -> Option<String> {
    if character == TAS_CHARACTER_UNKNOWN {
        return None;
    }
    let mut label = character_name(character).to_string();
    if stance != u32::MAX {
        label.push_str(" · ");
        label.push_str(stance_name(stance));
    }
    Some(label)
}

/// The advice a replay arm should print when the take in the buffer was
/// recorded as a different rider than the one on the board now: which menu
/// screen fixes it. The game bakes the character and the stance into the
/// rider when a level is entered from the menu and no in-process restart
/// re-reads them, so the fix is always a menu trip. `None` = same rider, or
/// either side unknown.
pub fn rider_mismatch_advice(
    loaded_rider: Option<&str>,
    live_rider: Option<&str>,
) -> Option<String> {
    let want = loaded_rider?;
    let have = live_rider?;
    // "Keith · goofy" -> ("Keith", Some("goofy")); "Keith" -> ("Keith", None).
    // A stamp without a stance (recorded before the setup object was found)
    // carries no stance claim, so it can only ever disagree on the character.
    fn split(label: &str) -> (&str, Option<&str>) {
        let mut parts = label.splitn(2, " · ");
        let character = parts.next().unwrap_or(label);
        (character, parts.next().filter(|s| !s.is_empty()))
    }
    let (want_char, want_stance) = split(want);
    let (have_char, have_stance) = split(have);
    let screen = if want_char != have_char {
        "Select Character (and Select Board for the stance)"
    } else {
        match (want_stance, have_stance) {
            (Some(a), Some(b)) if a != b => "Select Board (Stance)",
            _ => return None, // same character; the stances agree or one is unknown
        }
    };
    Some(format!(
        "this take was recorded as {} but the rider is {}: the physics differ, it will not \
         line up. Return to the menu, set it on the {} screen and re-enter the track - a \
         restart does not change it",
        want, have, screen
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rider stamp: character by id, stance appended when known. A Keith
    /// recording under Vincent must compare unequal; an unresolved rider
    /// (fresh DLL, no player yet) is `None`, not a bogus "unknown" stamp.
    #[test]
    fn rider_stamp_distinguishes_characters_and_stances() {
        assert_eq!(
            rider_label(TAS_CHARACTER_VINCENT, TAS_STANCE_REGULAR).as_deref(),
            Some("Vincent · regular")
        );
        assert_eq!(
            rider_label(TAS_CHARACTER_KEITH, TAS_STANCE_GOOFY).as_deref(),
            Some("Keith · goofy")
        );
        assert_ne!(
            rider_label(TAS_CHARACTER_KEITH, 0),
            rider_label(TAS_CHARACTER_VINCENT, 0)
        );
        assert_ne!(
            rider_label(TAS_CHARACTER_KEITH, 0),
            rider_label(TAS_CHARACTER_KEITH, 1)
        );
        assert_eq!(
            rider_label(TAS_CHARACTER_KEITH, u32::MAX).as_deref(),
            Some("Keith")
        );
        assert_eq!(rider_label(TAS_CHARACTER_UNKNOWN, 0), None);
        assert_eq!(
            rider_label(TAS_CHARACTER_OTHER, 0).as_deref(),
            Some("other · regular")
        );
        assert_eq!(
            rider_label(TAS_CHARACTER_KEITH, 7).as_deref(),
            Some("Keith · unknown")
        );
        assert_eq!(character_name(TAS_CHARACTER_ULRIKA), "Ulrika");
        assert_eq!(character_name(99), "unknown");
        assert_eq!(character_id_from_name("vincent"), TAS_CHARACTER_VINCENT);
        assert_eq!(character_id_from_name("Nobody"), TAS_CHARACTER_OTHER);
        assert_eq!(character_id_from_name(""), TAS_CHARACTER_UNKNOWN);
    }

    /// A replay armed on a take recorded as a different rider gets told
    /// which menu screen fixes it; same rider or an unknown side says nothing.
    #[test]
    fn rider_mismatch_advice_names_the_menu_screen() {
        let stance = rider_mismatch_advice(Some("Keith · goofy"), Some("Keith · regular")).unwrap();
        assert!(
            stance.contains("Keith · goofy") && stance.contains("Keith · regular"),
            "{}",
            stance
        );
        assert!(
            stance.contains("Select Board (Stance)") && !stance.contains("Select Character"),
            "{}",
            stance
        );
        assert!(stance.contains("restart does not change it"), "{}", stance);
        let character =
            rider_mismatch_advice(Some("Keith · regular"), Some("Vincent · regular")).unwrap();
        assert!(character.contains("Select Character"), "{}", character);
        assert_eq!(
            rider_mismatch_advice(Some("Keith · goofy"), Some("Keith · goofy")),
            None
        );
        // A stamp without a stance makes no stance claim: same character = no warning.
        assert_eq!(
            rider_mismatch_advice(Some("Keith"), Some("Keith · regular")),
            None
        );
        assert_eq!(
            rider_mismatch_advice(Some("Keith · regular"), Some("Keith")),
            None
        );
        assert_eq!(rider_mismatch_advice(Some("Keith"), Some("Keith")), None);
        let no_stance = rider_mismatch_advice(Some("Keith"), Some("Vincent · regular")).unwrap();
        assert!(no_stance.contains("Select Character"), "{}", no_stance);
        assert_eq!(rider_mismatch_advice(Some("Keith · goofy"), None), None);
        assert_eq!(rider_mismatch_advice(None, Some("Keith · goofy")), None);
    }
}
