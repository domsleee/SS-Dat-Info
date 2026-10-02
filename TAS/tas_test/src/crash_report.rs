//! A fault inside a DLL call into game code crashes the game and leaves a
//! crash record naming it (DESIGN.md "Crashes"). Sends the test-only fault
//! command, then reads the record from the mapping this process keeps after
//! the game is gone.
//!
//! A crashing game can cut a replay write short and leave zero-padded
//! Replay_N.dat files that break every later restart, so the level's replay
//! files are backed up first and restored if the crash changed them.

use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::crash::{game_exit, GameExit};
use tas_shared::{TasCommand, TAS_GAME_CALL_TEST_FAULT};

use crate::{harness, win32};

const REPLAYS: &str = r"Data\Levels\Forest\Tracks\Easy\replays";

pub fn run() -> bool {
    println!("=== CRASH-REPORT: a fault in a game call leaves a crash record ===\n");
    let backup = match backup_replays() {
        Ok(b) => b,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    let result = crash_and_read_record();
    let restored = restore_replays(&backup);
    // Leave a working game for whatever runs next.
    let client = harness::ensure_game_running();
    let fresh = client
        .state()
        .crash_seq
        .load(std::sync::atomic::Ordering::Acquire)
        == 0;
    match (result, restored) {
        (Ok(()), Ok(n)) if fresh => {
            println!("  replay files restored: {n}; relaunched game has no crash record");
            println!("\n*** CRASH-REPORT PASSED ***");
            true
        }
        (Ok(()), Ok(_)) => {
            eprintln!("\n*** CRASH-REPORT FAILED: the relaunched game kept the old record ***");
            false
        }
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("\n*** CRASH-REPORT FAILED: {e} ***");
            false
        }
    }
}

fn crash_and_read_record() -> Result<(), String> {
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let pid = game_pid().ok_or("no game process")?;
    client.send_command(TasCommand::TestFault);
    let deadline = Instant::now() + Duration::from_secs(10);
    let record = loop {
        if let Some(record) = tas_shared::crash::crash_record(client.state(), pid) {
            break record;
        }
        if Instant::now() > deadline {
            return Err("no crash record 10 s after the fault command".into());
        }
        thread::sleep(Duration::from_millis(10));
    };
    println!("  record: {record}");
    if record.game_call != TAS_GAME_CALL_TEST_FAULT || record.code != 0xC000_0005 {
        return Err(format!(
            "the record does not describe the test fault: {record:?}"
        ));
    }
    if record.module.is_empty() {
        return Err("the fault address was not resolved to a module".into());
    }
    // The game dies by itself, or its own error dialog holds it open.
    let deadline = Instant::now() + Duration::from_secs(10);
    while game_pid() == Some(pid) && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(100));
    }
    if game_pid() == Some(pid) {
        println!("  the game is still up 10 s after the fault (its error dialog?): killing it");
        harness::kill_game();
    } else {
        println!("  the game exited by itself");
    }
    match game_exit(client.state(), pid) {
        GameExit::Crashed(_) => Ok(()),
        other => Err(format!("after the game died the mapping reads {other:?}")),
    }
}

fn game_pid() -> Option<u32> {
    let hwnd = win32::find_game_window()?;
    let mut pid = 0u32;
    unsafe { win32::GetWindowThreadProcessId(hwnd, &mut pid) };
    (pid != 0).then_some(pid)
}

fn replays_dir() -> Result<PathBuf, String> {
    let folder = std::env::var("SUPREME_FOLDER").map_err(|_| "SUPREME_FOLDER is not set")?;
    Ok(PathBuf::from(folder).join(REPLAYS))
}

fn backup_replays() -> Result<Vec<(PathBuf, Vec<u8>)>, String> {
    let dir = replays_dir()?;
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut files = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            let bytes = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            files.push((path, bytes));
        }
    }
    Ok(files)
}

/// Put back every file the crash changed; returns how many.
fn restore_replays(backup: &[(PathBuf, Vec<u8>)]) -> Result<usize, String> {
    let mut restored = 0;
    for (path, bytes) in backup {
        if std::fs::read(path).ok().as_deref() != Some(bytes.as_slice()) {
            std::fs::write(path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
            restored += 1;
        }
    }
    Ok(restored)
}
