//! Commands that arrive in a state the transport never produces: an arm while
//! another mode is live, or a RESTART while a CONT is parked at its splice.
//! The UI always STOPs first, but the DLL must still leave the game sane.
//!
//! Key state is read from the game's own key buffer (`gamemem`), not from the
//! DLL's shared memory, so a DLL that forgets a key cannot vouch for itself.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::{TasCommand, TasMode, TasSharedMemoryClient};

use crate::gamemem::GameMemory;
use crate::{harness, replay};

const FIXTURE: &str = "FE-tremendous.tasrec";
/// Replay speed while hunting for a held window: fast enough to reach it
/// quickly, slow enough that a 2 ms poll sees the window.
const SEARCH_SPEED: f32 = 2.0;
/// A window of recorded ticks that all hold at least one common key.
const HELD_WINDOW_TICKS: u32 = 60;

/// First index at or after `from` that starts `len` consecutive ticks sharing
/// at least one pressed key, with the keys common to all of them.
pub(crate) fn find_held_window(log: &[u8], count: u32, from: u32, len: u32) -> Option<(u32, u8)> {
    let count = (count as usize).min(log.len());
    let len = len as usize;
    (from as usize..=count.saturating_sub(len)).find_map(|start| {
        let common = log[start..start + len].iter().fold(0xFFu8, |m, &b| m & b) & 0x3F;
        (common != 0).then_some((start as u32, common))
    })
}

pub(crate) fn load_fixture(client: &mut TasSharedMemoryClient) -> Result<u32, String> {
    let path = harness::fixture_path(FIXTURE)?;
    let loaded = replay::load_tasrec(&path).map_err(|e| format!("loading {FIXTURE}: {e}"))?;
    replay::write_to_shared(client, &loaded);
    Ok(loaded.count)
}

/// STOP, wait for the DLL to take it, and put the speed back to 1x.
pub(crate) fn cleanup(client: &mut TasSharedMemoryClient) {
    harness::stop(client);
    if !wait_idle(client, Duration::from_secs(3)) {
        eprintln!("  WARNING: the cleanup STOP was not consumed");
    }
    client.state_mut().playback_speed = 1.0;
}

fn wait_arm(client: &TasSharedMemoryClient, generation: u32) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while client.state().arm_generation == generation {
        if Instant::now() > deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(2));
    }
    true
}

fn wait_idle(client: &TasSharedMemoryClient, within: Duration) -> bool {
    let deadline = Instant::now() + within;
    while !client.command_idle() {
        if Instant::now() > deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(5));
    }
    true
}

/// Common setup: own the channel, attach to game memory, load the fixture and
/// start an aligned PLAY. Returns the memory view, the recording's gate and
/// the live gate.
pub(crate) fn start_aligned_play(
    client: &mut TasSharedMemoryClient,
) -> Result<(GameMemory, u32, u32), String> {
    harness::stop_competing_tas_ui_writer();
    harness::stop(client);
    let memory = GameMemory::attach().ok_or("cannot read the game's memory")?;
    load_fixture(client)?;
    client.state_mut().playback_speed = SEARCH_SPEED;
    let (rec_gate, live_gate) = harness::restart_play_aligned_unwatched(client)?;
    Ok((memory, rec_gate, live_gate))
}

/// Wait until the aligned replay reads recorded tick `target` or later.
pub(crate) fn wait_for_recorded_index(
    client: &TasSharedMemoryClient,
    rec_gate: u32,
    live_gate: u32,
    target: u32,
) -> Result<u32, String> {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        if client.mode_volatile() != TasMode::Play as u32 {
            return Err(format!("PLAY ended before recorded tick {target}"));
        }
        let pos = client.playback_pos_volatile();
        let recorded = (pos + rec_gate).saturating_sub(live_gate);
        if recorded >= target {
            return Ok(recorded);
        }
        if Instant::now() > deadline {
            return Err(format!(
                "replay never reached recorded tick {target} (at {recorded})"
            ));
        }
        thread::sleep(Duration::from_millis(1));
    }
}

/// Keys the game holds, read after an arm landed.
struct ArmOutcome {
    /// Keys the recording held where the arm was sent.
    replayed: u8,
    /// Key buffer (what the DLL writes) and keyboard observer (what steering
    /// reads) 300 ms after the arm.
    buffer: u8,
    observer: u8,
    mode: u32,
}

/// Replay into a window where the recording holds a key, check both game
/// arrays show it, then send `arm` with no STOP first.
fn arm_over_held_play(
    client: &mut TasSharedMemoryClient,
    arm: TasCommand,
) -> Result<ArmOutcome, String> {
    let (memory, rec_gate, live_gate) = start_aligned_play(client)?;
    let (log, count) = {
        let s = client.state();
        (s.input_log.to_vec(), s.recorded_count)
    };
    let (window, keys) = find_held_window(&log, count, rec_gate + 50, HELD_WINDOW_TICKS)
        .ok_or("the fixture has no held-key window")?;
    println!(
        "  Recording holds mask {keys:#04x} over ticks {window}..{}",
        window + HELD_WINDOW_TICKS
    );
    let at = wait_for_recorded_index(client, rec_gate, live_gate, window + HELD_WINDOW_TICKS / 3)?;
    let (buffer, observer) = (memory.held_tas_mask()?, memory.observer_tas_mask()?);
    println!(
        "  At recorded tick {at}: key buffer {buffer:#04x}, keyboard observer {observer:#04x}"
    );
    if buffer & keys == 0 || observer & keys == 0 {
        return Err(format!(
            "the game does not show the replayed keys {keys:#04x} (buffer {buffer:#04x}, observer {observer:#04x}); a reader is wrong"
        ));
    }
    match arm {
        // A valid splice, so the only reason to refuse is the live mode.
        TasCommand::ArmContinue => client.state_mut().continue_from_frame = count / 2,
        // Unaligned, the new PLAY replays the recording's countdown, which
        // holds no keys, so any key down 300 ms later is the old session's.
        TasCommand::ArmPlay => client.state_mut().gate_align_rec = 0,
        _ => {}
    }
    let generation = client.state().arm_generation;
    client.send_command(arm);
    if !wait_arm(client, generation) {
        return Err(format!("the DLL never processed {arm:?}"));
    }
    thread::sleep(Duration::from_millis(300));
    Ok(ArmOutcome {
        replayed: keys,
        buffer: memory.held_tas_mask()?,
        observer: memory.observer_tas_mask()?,
        mode: client.mode_volatile(),
    })
}

/// Pass when neither game array still holds a key the replay was holding.
/// `expect_mode` is the mode the arm must leave (OFF for a refusal).
fn judge(
    name: &str,
    arm: TasCommand,
    expect_mode: TasMode,
    result: Result<ArmOutcome, String>,
) -> bool {
    let outcome = match result {
        Ok(outcome) => outcome,
        Err(e) => {
            eprintln!("ERROR: {name} ({arm:?}): {e}");
            return false;
        }
    };
    let ArmOutcome {
        replayed,
        buffer,
        observer,
        mode,
    } = outcome;
    println!(
        "  After {arm:?}: mode={mode}, key buffer {buffer:#04x}, keyboard observer {observer:#04x}"
    );
    if mode != expect_mode as u32 {
        println!(
            "  {name} INCONCLUSIVE for {arm:?}: expected mode {}, got {mode}",
            expect_mode as u32
        );
        return false;
    }
    let stuck = (buffer | observer) & replayed;
    if stuck != 0 {
        println!("  {name} FAILED for {arm:?}: the game still holds {stuck:#04x} from the previous session");
        return false;
    }
    println!("  {name} ok for {arm:?}: every replayed key was released");
    true
}

/// A refused ARM_CONTINUE (sent during PLAY) turns the DLL OFF. Whatever keys
/// the replay was holding must be released, or the rider keeps steering.
pub fn run_cont_refuse_release() -> bool {
    println!("=== CONT-REFUSE-RELEASE: a refused CONT must release injected keys ===\n");
    let mut client = harness::ensure_game_running();
    let result = arm_over_held_play(&mut client, TasCommand::ArmContinue);
    cleanup(&mut client);
    let ok = judge(
        "CONT-REFUSE-RELEASE",
        TasCommand::ArmContinue,
        TasMode::Off,
        result,
    );
    println!(
        "*** CONT-REFUSE-RELEASE {} ***",
        if ok {
            "PASSED: the refusal released every injected key"
        } else {
            "FAILED"
        }
    );
    ok
}

/// ARM_REC and ARM_PLAY sent over a live PLAY (no STOP first) start a new
/// session. The keys the old one held must not survive into it: the new
/// session's first mask is compared against no keys held, so a key it never
/// presses would otherwise stay down in the observer and keep steering.
pub fn run_arm_over_live_release() -> bool {
    println!("=== ARM-OVER-LIVE-RELEASE: an arm over a live session releases its keys ===\n");
    let mut client = harness::ensure_game_running();
    let mut ok = true;
    for (arm, mode) in [
        (TasCommand::ArmRec, TasMode::Rec),
        (TasCommand::ArmPlay, TasMode::Play),
    ] {
        println!("--- {arm:?} over a live PLAY ---");
        let result = arm_over_held_play(&mut client, arm);
        cleanup(&mut client);
        ok &= judge("ARM-OVER-LIVE-RELEASE", arm, mode, result);
    }
    println!(
        "*** ARM-OVER-LIVE-RELEASE {} ***",
        if ok {
            "PASSED: ARM_REC and ARM_PLAY released the previous session's keys"
        } else {
            "FAILED"
        }
    );
    ok
}

/// RESTART while an aligned CONT is parked at an unapproved splice. The park
/// runs zero ticks, so the command must still be consumed and the restart
/// must complete.
pub fn run_restart_while_parked() -> bool {
    println!("=== RESTART-WHILE-PARKED: RESTART must be consumed at a parked splice ===\n");
    let mut client = harness::ensure_game_running();
    let ok = match restart_while_parked(&mut client) {
        Ok(()) => true,
        Err(e) => {
            println!("*** RESTART-WHILE-PARKED FAILED: {e} ***");
            false
        }
    };
    cleanup(&mut client);
    if ok {
        println!("*** RESTART-WHILE-PARKED PASSED: the parked CONT gave way to the restart ***");
    }
    ok
}

fn restart_while_parked(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    harness::stop_competing_tas_ui_writer();
    harness::stop(client);
    let count = load_fixture(client)?;
    let rec_gate = tas_shared::align::detect_first_moving(&client.state().rec_coords[..], count)
        .ok_or("the fixture never leaves the spawn")?;
    let splice = rec_gate + 400;
    if splice >= count {
        return Err("the fixture is too short".into());
    }
    client.state_mut().playback_speed = 8.0;
    if !harness::restart_and_stabilize_inprocess(client) {
        return Err("in-process restart failed".into());
    }
    {
        let s = client.state_mut();
        s.gate_align_rec = rec_gate;
        s.continue_from_frame = splice;
        s.cont_splice_approved = 0;
    }
    let generation = client.state().arm_generation;
    client.send_command(TasCommand::ArmContinue);
    if !wait_arm(client, generation) || client.mode_volatile() != TasMode::Play as u32 {
        return Err(format!(
            "CONT did not arm (mode={})",
            client.mode_volatile()
        ));
    }

    // Parked = PLAY, past the gate, playback_pos frozen for half a second.
    let deadline = Instant::now() + Duration::from_secs(30);
    let (mut last, mut since) = (u32::MAX, Instant::now());
    loop {
        let pos = client.playback_pos_volatile();
        if client.mode_volatile() != TasMode::Play as u32 {
            return Err(format!("CONT left PLAY before parking (pos {pos})"));
        }
        if pos != last {
            (last, since) = (pos, Instant::now());
        } else if pos > rec_gate && since.elapsed() > Duration::from_millis(500) {
            break;
        }
        if Instant::now() > deadline {
            return Err(format!("CONT never parked (pos {pos}, splice {splice})"));
        }
        thread::sleep(Duration::from_millis(10));
    }
    println!("  Parked at playback_pos {last} (splice {splice}, recording gate {rec_gate})");

    client.reset_restart_state();
    client.send_command(TasCommand::Restart);
    if !wait_idle(client, Duration::from_secs(3)) {
        return Err(format!(
            "RESTART still pending after 3 s (command={}, restart_state={}, playback_pos={})",
            client.state().command,
            client.restart_state(),
            client.playback_pos_volatile()
        ));
    }
    let deadline = Instant::now() + Duration::from_secs(15);
    while client.restart_state() != 2 {
        if Instant::now() > deadline {
            return Err(format!(
                "RESTART consumed but never completed (restart_state={})",
                client.restart_state()
            ));
        }
        thread::sleep(Duration::from_millis(20));
    }
    client.reset_restart_state();
    // Left armed, the CONT would splice into REC on the rebuilt level.
    thread::sleep(Duration::from_millis(500));
    let (mode, recorded) = (client.mode_volatile(), client.recorded_count_volatile());
    println!("  RESTART consumed and completed; mode={mode} recorded_count={recorded}");
    if mode != TasMode::Off as u32 {
        return Err(format!(
            "the restart left the DLL armed (mode={mode}, recorded_count={recorded})"
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::find_held_window;

    #[test]
    fn held_window_needs_a_common_key() {
        let mut log = vec![0u8; 100];
        log[10..20].fill(0x01);
        log[20..30].fill(0x02);
        assert_eq!(find_held_window(&log, 100, 0, 10), Some((10, 0x01)));
        assert_eq!(find_held_window(&log, 100, 11, 10), Some((20, 0x02)));
        assert_eq!(find_held_window(&log, 100, 0, 15), None);
        log[40..60].fill(0x11);
        assert_eq!(find_held_window(&log, 100, 0, 15), Some((40, 0x11)));
    }

    #[test]
    fn held_window_ignores_escape_and_stops_before_count() {
        let mut log = vec![0x80u8; 50];
        assert_eq!(find_held_window(&log, 50, 0, 5), None);
        log[40..50].fill(0x04);
        assert_eq!(find_held_window(&log, 45, 0, 10), None);
    }
}
