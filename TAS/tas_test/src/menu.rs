//! The menu as text and the menu command channel, for agents:
//!
//! ```text
//! tas_test menu                       print the document (one JSON line)
//! tas_test menu activate <id|label>   focus that item, then Enter on it
//! tas_test menu focus <id|label>      just move the cursor there
//! tas_test menu up|down|left|right    move the cursor; `trigger` = Enter
//! ```
//!
//! A command is executed by the DLL on the menu thread through the game's own
//! entry points. This waits for the ack, then for the document to settle, and
//! prints `{"result":..,"doc":..}`; the exit code is 0 on ok.

use std::time::{Duration, Instant};

use tas_shared::TasSharedMemoryClient;

use crate::cli;

/// The page id named by a menu document (`{"screen":"ID_...","sel":..}`).
/// Screen ids are C identifiers (no quotes or escapes), so a prefix scan is
/// exact — no JSON parser needed for this one field.
fn screen_in_doc(doc: &str) -> Option<String> {
    let rest = doc.split_once("\"screen\":\"")?.1;
    let id = rest.split_once('"')?.0;
    if id.is_empty() {
        return None;
    }
    Some(id.to_string())
}

/// The document can be momentarily unavailable while the DLL is writing it or
/// the page is in transition, so retry briefly before calling it "no menu".
fn read_doc(client: &TasSharedMemoryClient) -> Option<String> {
    let t = Instant::now();
    loop {
        if let Some(d) = tas_shared::menu_doc(client.state()) {
            return Some(d);
        }
        if t.elapsed() > Duration::from_millis(400) {
            return None;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// Wait for the document to change and then hold still for 400 ms (a page
/// transition takes ~1-2 s); give up on "no change" after 1.5 s; cap at 4 s.
fn wait_for_settle(client: &TasSharedMemoryClient, before: &Option<String>) {
    let t1 = Instant::now();
    let mut last = before.clone();
    let mut since = Instant::now();
    loop {
        std::thread::sleep(Duration::from_millis(50));
        let now = tas_shared::menu_doc(client.state());
        if now != last {
            last = now;
            since = Instant::now();
        }
        let held = since.elapsed();
        if (&last != before && held >= Duration::from_millis(400))
            || (&last == before && held >= Duration::from_millis(1500))
            || t1.elapsed() >= Duration::from_millis(4000)
        {
            break;
        }
    }
}

pub fn run(sub: Option<&str>, target: Option<&str>) -> bool {
    let mut client = match TasSharedMemoryClient::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: no TAS shared memory ({})", e);
            return false;
        }
    };
    let Some(sub) = sub else {
        match tas_shared::menu_doc(client.state()) {
            Some(doc) => println!("{}", doc),
            None => println!("{{\"screen\":null,\"sel\":null,\"items\":[]}}"),
        }
        return true;
    };
    let Some(kind) = tas_shared::menu_command_kind(sub) else {
        cli::usage_error(format!(
            "unknown menu command '{sub}'; expected activate <id|label> | focus <id|label> | up | down | left | right | trigger"
        ));
    };
    let target = target.unwrap_or_default();
    if tas_shared::menu_command_needs_target(kind) && target.is_empty() {
        cli::usage_error(format!("menu {sub} needs an <id|label>"));
    }
    let before = read_doc(&client);
    let Some(before_doc) = before.as_deref() else {
        // Nobody consumes commands outside a menu (the hook runs from the
        // menu's own per-frame update), so say so at once.
        println!("{{\"result\":\"no menu\",\"doc\":null}}");
        return false;
    };
    // The screen comes from the same document the command will act on, not a
    // separate read: doc and screen publish together, so parsing here is one
    // coherent snapshot. A doc without a screen id is unusable — refuse
    // rather than submitting an unchecked (empty-screen) command.
    let Some(screen) = screen_in_doc(before_doc) else {
        println!("{{\"result\":\"unstable\",\"doc\":{}}}", before_doc);
        return false;
    };
    // The command names the page it was read from, so it is refused rather
    // than executed if the menu moves on first.
    let seq = match tas_shared::menu_command_submit(client.state_mut(), kind, target, &screen) {
        Ok(seq) => seq,
        Err(tas_shared::MenuSubmitError::Busy) => {
            println!(
                "{{\"result\":\"busy\",\"doc\":{}}}",
                before.as_deref().unwrap_or("null")
            );
            return false;
        }
    };
    let t0 = Instant::now();
    let result = loop {
        if let Some(r) = tas_shared::menu_command_result(client.state(), seq) {
            break Some(r);
        }
        if t0.elapsed() > Duration::from_millis(3000) {
            break None;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let name = match result {
        Some(r) => tas_shared::menu_result_name(r).to_string(),
        None => "timeout".to_string(),
    };
    let ok = result == Some(tas_shared::TAS_MENU_RESULT_OK);
    if ok {
        wait_for_settle(&client, &before);
    }
    let doc = read_doc(&client).unwrap_or_else(|| "null".to_string());

    println!("{{\"result\":\"{}\",\"doc\":{}}}", name, doc);
    ok
}

/// One parsed menu document: the screen id plus its items' published ids and
/// labels. Only the fields navigation needs.
#[derive(serde::Deserialize)]
struct MenuDocLite {
    screen: String,
    #[serde(default)]
    items: Vec<MenuDocItemLite>,
}

#[derive(serde::Deserialize)]
struct MenuDocItemLite {
    #[serde(default)]
    id: String,
    #[serde(default)]
    label: String,
}

/// Navigate one step through the menu protocol: wait for the live document,
/// find `label` (case-insensitive, trimmed), activate it by its published id
/// (falling back to the label when the item has none — the DLL matches
/// either), wait for the ack, and verify the destination page. Returns the
/// destination screen id. `expect_screen` pins it; `None` accepts any page
/// change. Entering a menu still needs a physical key — everything after
/// goes through here instead of blind cursor counting.
pub fn activate_label_and_wait(
    client: &mut TasSharedMemoryClient,
    label: &str,
    expect_screen: Option<&str>,
    timeout: Duration,
) -> Result<String, String> {
    let start = Instant::now();
    let before = loop {
        if let Some(doc) = read_doc(client) {
            break doc;
        }
        if start.elapsed() >= timeout {
            return Err("no menu document appeared".to_string());
        }
    };
    let doc: MenuDocLite = serde_json::from_str(&before)
        .map_err(|e| format!("menu document does not parse: {}", e))?;
    let item = doc
        .items
        .iter()
        .find(|i| i.label.trim().eq_ignore_ascii_case(label.trim()))
        .ok_or_else(|| {
            let labels: Vec<&str> = doc.items.iter().map(|i| i.label.as_str()).collect();
            format!(
                "menu page {} has no {:?} item (items: [{}])",
                doc.screen,
                label,
                labels.join(", ")
            )
        })?;
    let target = if item.id.is_empty() {
        item.label.clone()
    } else {
        item.id.clone()
    };
    let kind =
        tas_shared::menu_command_kind("activate").ok_or_else(|| "unknown command".to_string())?;
    let seq = tas_shared::menu_command_submit(client.state_mut(), kind, &target, &doc.screen)
        .map_err(|_| "menu busy (a command is already outstanding)".to_string())?;
    let t0 = Instant::now();
    let result = loop {
        if let Some(r) = tas_shared::menu_command_result(client.state(), seq) {
            break Some(r);
        }
        if t0.elapsed() > Duration::from_millis(3000) {
            break None;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    match result {
        Some(r) if r == tas_shared::TAS_MENU_RESULT_OK => {}
        Some(r) => {
            return Err(format!(
                "menu command refused: {}",
                tas_shared::menu_result_name(r)
            ));
        }
        None => return Err("menu command timed out waiting for the ack".to_string()),
    }
    wait_for_settle(client, &Some(before));
    let after =
        read_doc(client).ok_or_else(|| "menu document gone after navigation".to_string())?;
    let dest =
        screen_in_doc(&after).ok_or_else(|| "navigated-to document names no page".to_string())?;
    if let Some(expected) = expect_screen {
        if dest != expected {
            return Err(format!(
                "navigation landed on {} (expected {})",
                dest, expected
            ));
        }
    }
    Ok(dest)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_in_doc_reads_the_named_page() {
        assert_eq!(
            screen_in_doc(r#"{"screen":"ID_ARCADE_MENU","sel":2,"items":[]}"#).as_deref(),
            Some("ID_ARCADE_MENU")
        );
        assert_eq!(screen_in_doc(r#"{"sel":2,"items":[]}"#), None);
        assert_eq!(screen_in_doc(r#"{"screen":"","sel":0,"items":[]}"#), None);
        assert_eq!(screen_in_doc("not json"), None);
    }
}
