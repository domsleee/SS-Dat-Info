//! A hitch must not open the live-input block. During PLAY the only input the
//! game may see is the recording's; the block lifts only for the pause menu
//! and dialogs so they stay usable. A frame that merely took long is not a
//! pause.
//!
//! The test freezes the whole game process (a hitch on demand) while the Pico
//! presses LEFT during an aligned replay, in a stretch where the recording
//! holds no LEFT, then reads the game's keyboard observer: LEFT there is live
//! input that leaked into the replay.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasSharedMemoryClient;

use crate::command_edges::{start_aligned_play, wait_for_recorded_index};
use crate::harness;

const LEFT: u8 = 0x01;
/// Longer than the 250 ms the input gate treats as a pause.
const FREEZE: Duration = Duration::from_millis(400);
/// How long LEFT stays down after the game resumes.
const HOLD_AFTER: Duration = Duration::from_millis(300);
/// Recorded ticks without LEFT around the hitch.
const FREE_WINDOW_TICKS: u32 = 200;

/// First index at or after `from` that starts `len` ticks none of which
/// presses any of `bits`.
pub(crate) fn find_free_window(
    log: &[u8],
    count: u32,
    from: u32,
    len: u32,
    bits: u8,
) -> Option<u32> {
    let count = (count as usize).min(log.len());
    let len = len as usize;
    (from as usize..=count.saturating_sub(len))
        .find(|&start| log[start..start + len].iter().all(|&b| b & bits == 0))
        .map(|start| start as u32)
}

pub fn run() -> bool {
    println!("=== HITCH-INPUT-LEAK: a frozen frame must not let live keys into PLAY ===\n");
    harness::run_case("HITCH-INPUT-LEAK", hitch_during_play)
}

fn hitch_during_play(client: &mut TasSharedMemoryClient) -> Result<String, String> {
    // Keys only reach a foreground window.
    harness::focus_game();
    let (memory, rec_gate, live_gate) = start_aligned_play(client)?;
    client.state_mut().playback_speed = 1.0;
    let (log, count) = {
        let s = client.state();
        (s.input_log.to_vec(), s.recorded_count)
    };
    let window = find_free_window(&log, count, rec_gate + 50, FREE_WINDOW_TICKS, LEFT)
        .ok_or("the fixture has no LEFT-free window")?;
    let at = wait_for_recorded_index(client, rec_gate, live_gate, window + 20)?;
    let before = memory.observer_tas_mask()?;
    println!(
        "  Recorded ticks {window}..{} hold no LEFT; at {at} the observer holds {before:#04x}",
        window + FREE_WINDOW_TICKS
    );
    if before & LEFT != 0 {
        return Err("LEFT is already held before the hitch".into());
    }

    let mut keys = harness::PicoKeys::open_checked()?;
    {
        let _frozen = memory.freeze()?;
        if !keys.send(LEFT) {
            return Err("Pico LEFT press failed".into());
        }
        // Refresh inside the firmware's 500 ms safety release.
        let start = Instant::now();
        while start.elapsed() < FREEZE {
            thread::sleep(Duration::from_millis(100));
            keys.send(LEFT);
        }
    }
    println!(
        "  Game frozen {} ms with LEFT down, resumed",
        FREEZE.as_millis()
    );

    let mut leaked_at = None;
    let start = Instant::now();
    while start.elapsed() < HOLD_AFTER {
        if memory.observer_tas_mask()? & LEFT != 0 && leaked_at.is_none() {
            leaked_at = Some(start.elapsed());
        }
        keys.send(LEFT);
        thread::sleep(Duration::from_millis(5));
    }
    drop(keys);
    thread::sleep(Duration::from_millis(200));
    let after = memory.observer_tas_mask()?;
    println!("  Observer after release: {after:#04x}");
    match leaked_at {
        None => Ok("the observer never saw the live LEFT".into()),
        Some(t) => Err(format!(
            "the observer held the live LEFT {} ms after the game resumed (still {:#04x} after release)",
            t.as_millis(),
            after
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::find_free_window;

    #[test]
    fn free_window_skips_any_press_of_the_bits() {
        let mut log = vec![0u8; 100];
        log[10] = 0x01;
        log[30] = 0x05;
        assert_eq!(find_free_window(&log, 100, 0, 10, 0x01), Some(0));
        assert_eq!(find_free_window(&log, 100, 5, 10, 0x01), Some(11));
        assert_eq!(find_free_window(&log, 100, 5, 30, 0x01), Some(31));
        assert_eq!(find_free_window(&log, 100, 5, 30, 0x02), Some(5));
        assert_eq!(find_free_window(&log, 100, 5, 90, 0x01), None);
    }
}
