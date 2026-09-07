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
    let screen = tas_shared::menu_screen_id(client.state()).unwrap_or_default();
    if before.is_none() {
        // Nobody consumes commands outside a menu (the hook runs from the
        // menu's own per-frame update), so say so at once.
        println!("{{\"result\":\"no menu\",\"doc\":null}}");
        return false;
    }
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
