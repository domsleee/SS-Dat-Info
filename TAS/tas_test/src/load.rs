//! Load a `.tasrec` into shared memory and exit, without stopping tas_ui or
//! arming anything, so a script can arm through the real UI afterwards.

use tas_shared::{TasMode, TasSharedMemoryClient};

use crate::replay;

pub fn run(path: &str) -> bool {
    let mut client = match TasSharedMemoryClient::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: no TAS shared memory ({})", e);
            return false;
        }
    };
    // Bulk-writing the input/coord arrays races a REC that is appending to them
    // and swaps the data a PLAY is consuming, so OFF is the only safe state.
    let mode = client.state().mode;
    if mode != TasMode::Off as u32 {
        eprintln!(
            "ERROR: refusing to load while the DLL is in mode {} (REC/PLAY active). Press STOP first.",
            mode
        );
        return false;
    }
    match replay::load_tasrec(std::path::Path::new(path)) {
        Ok(r) => {
            replay::write_to_shared(&mut client, &r);
            println!("loaded {} ticks from {}", r.count, path);
            true
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            false
        }
    }
}
