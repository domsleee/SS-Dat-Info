//! Readers and the command channel for the game's menu (`menu_screen`,
//! `menu_doc`, `menu_cmd_*` in `TasSharedState`).

use std::sync::atomic::Ordering;

use crate::state::{
    with_seqlock, TasSharedState, TAS_MENU_CMD_ACTIVATE, TAS_MENU_CMD_DOWN, TAS_MENU_CMD_FOCUS,
    TAS_MENU_CMD_LEFT, TAS_MENU_CMD_RIGHT, TAS_MENU_CMD_TARGET_MAX, TAS_MENU_CMD_TRIGGER,
    TAS_MENU_CMD_UP, TAS_MENU_DOC_MAX, TAS_MENU_RESULT_BAD_KIND, TAS_MENU_RESULT_DISABLED,
    TAS_MENU_RESULT_EXPIRED, TAS_MENU_RESULT_FAULT, TAS_MENU_RESULT_NOT_FOCUSABLE,
    TAS_MENU_RESULT_NOT_FOUND, TAS_MENU_RESULT_NO_MENU, TAS_MENU_RESULT_OK,
    TAS_MENU_RESULT_STALE_PAGE, TAS_MENU_SCREEN_MAX,
};

/// The current menu page id exactly as the DLL publishes it
/// ("ID_ARCADE_MENU"), or `None` while a level runs (the buffer is empty
/// then) or the writer kept it busy. Read under `menu_seq`, together with
/// which the DLL writes it (v48), so it never names another page's items.
pub fn menu_screen_id(state: &TasSharedState) -> Option<String> {
    let bytes = with_seqlock(&state.menu_seq, || {
        let mut v = Vec::new();
        for i in 0..TAS_MENU_SCREEN_MAX {
            // SAFETY: shared mapping written by the DLL's menu thread.
            let b = unsafe { std::ptr::read_volatile(&state.menu_screen[i]) };
            if b == 0 {
                break;
            }
            v.push(b);
        }
        v
    })?;
    if bytes.is_empty() || bytes.iter().any(|&c| !(0x20..0x7f).contains(&c)) {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// The menu screen the game is showing as a human label ("Main Menu",
/// "Arcade Choose Track"), or `None` while a level runs. See [`menu_screen_id`].
pub fn menu_screen(state: &TasSharedState) -> Option<String> {
    menu_screen_id(state).map(|raw| prettify_menu_id(&raw))
}

/// The menu document (v46): the current page's items with labels and stable
/// ids as a JSON string (see the field doc), or `None` while a level runs or
/// while the writer kept it busy. One coherent read under `menu_seq`.
pub fn menu_doc(state: &TasSharedState) -> Option<String> {
    let bytes = with_seqlock(&state.menu_seq, || {
        let mut v = Vec::new();
        for i in 0..TAS_MENU_DOC_MAX {
            // SAFETY: shared mapping written by the DLL's worker thread.
            let b = unsafe { std::ptr::read_volatile(&state.menu_doc[i]) };
            if b == 0 {
                break;
            }
            v.push(b);
        }
        v
    })?;
    if bytes.is_empty() || bytes.iter().any(|&c| !(0x20..0x7f).contains(&c)) {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes).into_owned())
}

/// The menu command kind for a CLI word ("activate", "focus", "up", "down",
/// "left", "right", "trigger").
pub fn menu_command_kind(name: &str) -> Option<u32> {
    match name {
        "activate" => Some(TAS_MENU_CMD_ACTIVATE),
        "focus" => Some(TAS_MENU_CMD_FOCUS),
        "up" => Some(TAS_MENU_CMD_UP),
        "down" => Some(TAS_MENU_CMD_DOWN),
        "left" => Some(TAS_MENU_CMD_LEFT),
        "right" => Some(TAS_MENU_CMD_RIGHT),
        "trigger" => Some(TAS_MENU_CMD_TRIGGER),
        _ => None,
    }
}

/// Whether a command kind names an item (activate / focus) or acts on the
/// cursor as it is.
pub fn menu_command_needs_target(kind: u32) -> bool {
    kind == TAS_MENU_CMD_ACTIVATE || kind == TAS_MENU_CMD_FOCUS
}

/// A menu command result as a word.
pub fn menu_result_name(result: u32) -> &'static str {
    match result {
        TAS_MENU_RESULT_OK => "ok",
        TAS_MENU_RESULT_NO_MENU => "no menu",
        TAS_MENU_RESULT_NOT_FOUND => "not found",
        TAS_MENU_RESULT_DISABLED => "disabled",
        TAS_MENU_RESULT_BAD_KIND => "bad kind",
        TAS_MENU_RESULT_FAULT => "fault",
        TAS_MENU_RESULT_NOT_FOCUSABLE => "not focusable",
        TAS_MENU_RESULT_STALE_PAGE => "stale page",
        TAS_MENU_RESULT_EXPIRED => "expired",
        _ => "unknown",
    }
}

/// Why a submission was refused.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MenuSubmitError {
    /// A previous command has not been answered yet (`menu_cmd_seq !=
    /// menu_cmd_ack`). Submitting now would let the DLL pair the old sequence
    /// with the new payload and run it twice. Wait for the ack (the DLL
    /// expires an unconsumable command after 3 s).
    Busy,
}

/// Submit a menu command (v47/v48): write the target and the page id it was
/// read from (both cut to their buffers, always NUL-terminated) and the kind,
/// then bump the sequence. Returns the sequence to wait for with
/// [`menu_command_result`]. One command is outstanding at a time.
pub fn menu_command_submit(
    state: &mut TasSharedState,
    kind: u32,
    target: &str,
    screen: &str,
) -> Result<u32, MenuSubmitError> {
    let cur = state.menu_cmd_seq.load(Ordering::Acquire);
    if state.menu_cmd_ack.load(Ordering::Acquire) != cur {
        return Err(MenuSubmitError::Busy);
    }
    let mut tbuf = [0u8; TAS_MENU_CMD_TARGET_MAX];
    let n = target.len().min(TAS_MENU_CMD_TARGET_MAX - 1);
    tbuf[..n].copy_from_slice(&target.as_bytes()[..n]);
    let mut sbuf = [0u8; TAS_MENU_SCREEN_MAX];
    let m = screen.len().min(TAS_MENU_SCREEN_MAX - 1);
    sbuf[..m].copy_from_slice(&screen.as_bytes()[..m]);
    // SAFETY: shared mapping; the DLL reads these after it sees the new sequence.
    unsafe {
        std::ptr::write_volatile(
            &mut state.menu_cmd_target as *mut [u8; TAS_MENU_CMD_TARGET_MAX],
            tbuf,
        );
        std::ptr::write_volatile(
            &mut state.menu_cmd_screen as *mut [u8; TAS_MENU_SCREEN_MAX],
            sbuf,
        );
        std::ptr::write_volatile(&mut state.menu_cmd_kind as *mut u32, kind);
    }
    let seq = cur.wrapping_add(1);
    state.menu_cmd_seq.store(seq, Ordering::Release);
    Ok(seq)
}

/// The result of a submitted command once the DLL has executed it; `None`
/// while it is still pending (or nobody is consuming: no menu on screen).
pub fn menu_command_result(state: &TasSharedState, seq: u32) -> Option<u32> {
    if state.menu_cmd_ack.load(Ordering::Acquire) != seq {
        return None;
    }
    // SAFETY: shared mapping; written by the DLL before it stored the ack.
    Some(unsafe { std::ptr::read_volatile(&state.menu_cmd_result) })
}

/// The DLL publishes the menu's internal page id ("ID_ARCADE_CHOOSE_TRACK").
/// Turn it into a human label ("Arcade Choose Track"): drop the `ID_` prefix
/// and title-case the underscore-separated words. Anything not in that shape is
/// returned unchanged.
pub fn prettify_menu_id(id: &str) -> String {
    let body = id.strip_prefix("ID_").unwrap_or(id);
    if body.is_empty() {
        return id.to_string();
    }
    body.split('_')
        .filter(|w| !w.is_empty())
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                Some(f) => f.to_ascii_uppercase().to_string() + &c.as_str().to_ascii_lowercase(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::zeroed_boxed;

    #[test]
    fn menu_id_prettifies() {
        assert_eq!(
            prettify_menu_id("ID_ARCADE_CHOOSE_TRACK"),
            "Arcade Choose Track"
        );
        assert_eq!(prettify_menu_id("ID_MAIN_MENU"), "Main Menu");
        assert_eq!(
            prettify_menu_id("ID_ARCADE_CHOOSE_BOARD"),
            "Arcade Choose Board"
        );
        assert_eq!(prettify_menu_id("Weird"), "Weird");
        assert_eq!(prettify_menu_id("ID_"), "ID_");
    }

    fn put_menu_doc(s: &mut TasSharedState, doc: &[u8], seq: u32) {
        s.menu_doc = [0u8; TAS_MENU_DOC_MAX];
        s.menu_doc[..doc.len()].copy_from_slice(doc);
        s.menu_seq.store(seq, Ordering::Relaxed);
    }

    /// The menu document (v46) comes back verbatim from a stable buffer.
    #[test]
    fn menu_doc_reads_the_published_json() {
        let mut s = zeroed_boxed();
        let sample = concat!(
            r#"{"screen":"ID_ARCADE_MENU","sel":2,"items":["#,
            r#"{"label":"Time Attack","id":"ID_ARCADE_TIME_ATTACK_SEQUENCE","en":true,"vis":true},"#,
            r#"{"label":"Pipe","id":"ID_ARCADE_HALF_PIPE_SEQUENCE","en":true,"vis":true}]}"#
        );
        put_menu_doc(&mut s, sample.as_bytes(), 2);
        assert_eq!(menu_doc(&s).as_deref(), Some(sample));
    }

    /// No document (a level running, or nothing published yet) is `None`,
    /// not an empty string the caller has to special-case.
    #[test]
    fn menu_doc_empty_is_none() {
        let s = zeroed_boxed();
        assert_eq!(menu_doc(&s), None);
    }

    /// A writer mid-update (odd sequence) never yields a half-written document.
    #[test]
    fn menu_doc_odd_seq_is_none() {
        let mut s = zeroed_boxed();
        put_menu_doc(
            &mut s,
            br#"{"screen":"ID_MAIN_MENU","sel":0,"items":[]}"#,
            3,
        );
        assert_eq!(menu_doc(&s), None);
    }

    /// A buffer with a non-printable byte (garbage, or a torn write that
    /// slipped past the seqlock) is rejected rather than handed to a parser.
    #[test]
    fn menu_doc_rejects_non_printable() {
        let mut s = zeroed_boxed();
        put_menu_doc(&mut s, b"{\"screen\":\"ID_MAIN\x01MENU\"}", 2);
        assert_eq!(menu_doc(&s), None);
    }

    /// Submitting a command writes the target, the page id and the kind, then
    /// bumps the sequence; the result stays pending until the DLL acks that
    /// sequence; a second submission is refused while the first is pending.
    #[test]
    fn menu_command_round_trip() {
        let mut s = zeroed_boxed();
        let seq = menu_command_submit(
            &mut s,
            TAS_MENU_CMD_ACTIVATE,
            "ID_ARCADE_MENU",
            "ID_MAIN_MENU",
        )
        .unwrap();
        assert_eq!(seq, 1);
        assert_eq!(s.menu_cmd_kind, TAS_MENU_CMD_ACTIVATE);
        assert_eq!(&s.menu_cmd_target[..15], b"ID_ARCADE_MENU\0");
        assert_eq!(&s.menu_cmd_screen[..13], b"ID_MAIN_MENU\0");
        assert_eq!(menu_command_result(&s, seq), None, "pending until acked");
        assert_eq!(
            menu_command_submit(&mut s, TAS_MENU_CMD_DOWN, "", ""),
            Err(MenuSubmitError::Busy),
            "one outstanding command at a time"
        );
        assert_eq!(
            s.menu_cmd_kind, TAS_MENU_CMD_ACTIVATE,
            "a refused submission writes nothing"
        );
        s.menu_cmd_result = TAS_MENU_RESULT_NOT_FOUND;
        s.menu_cmd_ack.store(seq, Ordering::Release);
        assert_eq!(
            menu_command_result(&s, seq),
            Some(TAS_MENU_RESULT_NOT_FOUND)
        );
        // acked: the next command gets the next sequence and is pending again
        let seq2 = menu_command_submit(&mut s, TAS_MENU_CMD_DOWN, "", "").unwrap();
        assert_eq!(seq2, 2);
        assert_eq!(menu_command_result(&s, seq2), None);
        assert_eq!(s.menu_cmd_target[0], 0, "an empty target clears the buffer");
        assert_eq!(
            s.menu_cmd_screen[0], 0,
            "an empty page id clears the buffer (= unchecked)"
        );
    }

    /// The raw page id and the pretty label come from the same seqlocked read.
    #[test]
    fn menu_screen_id_and_label() {
        let mut s = zeroed_boxed();
        assert_eq!(menu_screen_id(&s), None);
        s.menu_screen[..14].copy_from_slice(b"ID_ARCADE_MENU");
        s.menu_seq.store(2, Ordering::Relaxed);
        assert_eq!(menu_screen_id(&s).as_deref(), Some("ID_ARCADE_MENU"));
        assert_eq!(menu_screen(&s).as_deref(), Some("Arcade Menu"));
        s.menu_seq.store(3, Ordering::Relaxed); // writer mid-update
        assert_eq!(menu_screen_id(&s), None);
    }

    /// A target longer than the buffer is cut, never overrun, and stays NUL-terminated.
    #[test]
    fn menu_command_target_is_bounded() {
        let mut s = zeroed_boxed();
        let long = "X".repeat(200);
        menu_command_submit(&mut s, TAS_MENU_CMD_FOCUS, &long, &long).unwrap();
        assert_eq!(s.menu_cmd_screen[TAS_MENU_SCREEN_MAX - 1], 0);
        assert_eq!(s.menu_cmd_target[TAS_MENU_CMD_TARGET_MAX - 1], 0);
        assert!(s.menu_cmd_target[..TAS_MENU_CMD_TARGET_MAX - 1]
            .iter()
            .all(|&b| b == b'X'));
    }

    #[test]
    fn menu_command_kinds_and_names() {
        assert_eq!(menu_command_kind("activate"), Some(TAS_MENU_CMD_ACTIVATE));
        assert_eq!(menu_command_kind("down"), Some(TAS_MENU_CMD_DOWN));
        assert_eq!(menu_command_kind("jump"), None);
        assert!(menu_command_needs_target(TAS_MENU_CMD_ACTIVATE));
        assert!(menu_command_needs_target(TAS_MENU_CMD_FOCUS));
        assert!(!menu_command_needs_target(TAS_MENU_CMD_UP));
        assert_eq!(menu_result_name(TAS_MENU_RESULT_OK), "ok");
        assert_eq!(menu_result_name(TAS_MENU_RESULT_NOT_FOUND), "not found");
        assert_eq!(
            menu_result_name(TAS_MENU_RESULT_NOT_FOCUSABLE),
            "not focusable"
        );
        assert_eq!(menu_result_name(TAS_MENU_RESULT_STALE_PAGE), "stale page");
        assert_eq!(menu_result_name(TAS_MENU_RESULT_EXPIRED), "expired");
        assert_eq!(menu_result_name(99), "unknown");
    }
}
