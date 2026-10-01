//! Controller ownership against real processes (DESIGN.md "Controller
//! ownership"). A child tas_test owns a replay cycle and is killed at a chosen
//! point: the DLL must stop the TAS, release the keys and lift the live-input
//! block on its own. A reader's death must change nothing, a second controller
//! is refused while the owner lives, and a living owner's block never times
//! out.

use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use tas_shared::transport::{Arm, ArmConfig, StepOutcome, TransportController};
use tas_shared::{TasMode, TasSharedMemoryClient};

use crate::command_edges::{cleanup, load_fixture};
use crate::gamemem::GameMemory;
use crate::harness;

/// Where the child stops stepping and waits to be killed.
const KILL_POINTS: [&str; 3] = ["restart", "countdown", "replay"];
const LOG_LINE: &str = "exited: stopped the TAS";

pub fn run() -> bool {
    println!("=== OWNER-DEATH: the DLL stops the TAS when its controller dies ===\n");
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let result = run_cases(&mut client);
    cleanup(&mut client);
    match result {
        Ok(()) => {
            println!("\n*** OWNER-DEATH PASSED ***");
            true
        }
        Err(e) => {
            eprintln!("\n*** OWNER-DEATH FAILED: {e} ***");
            false
        }
    }
}

fn run_cases(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    let memory = GameMemory::attach().ok_or("cannot read the game's memory")?;
    for point in KILL_POINTS {
        killed_owner_stops_the_tas(client, &memory, point)?;
    }
    reader_death_changes_nothing(client)?;
    second_controller_is_refused_while_the_owner_lives(client)?;
    living_owner_keeps_its_block(client)?;
    Ok(())
}

fn killed_owner_stops_the_tas(
    client: &mut TasSharedMemoryClient,
    memory: &GameMemory,
    point: &str,
) -> Result<(), String> {
    println!("  -- kill the owner at: {point}");
    harness::stop(client);
    load_fixture(client)?;
    let (mut child, pid) = spawn_child(point)?;
    let st = client.state();
    if st.owner_pid != pid {
        let _ = child.kill();
        return Err(format!(
            "{point}: owner_pid {} is not the child {pid}",
            st.owner_pid
        ));
    }
    if point != "restart" && st.cont_suppress_input != 1 {
        let _ = child.kill();
        return Err(format!(
            "{point}: an owned aligned cycle left live input unblocked"
        ));
    }
    let (_, log_cursor) = st.read_log_entries(0);
    let held_before = memory.held_tas_mask()?;
    println!(
        "     owner {pid} at mode={} restart_state={} pos={} protection={} keys={held_before:#04x}",
        st.mode, st.restart_state, st.playback_pos, st.cont_suppress_input
    );
    if point == "replay" && held_before == 0 {
        kill(&mut child);
        return Err("replay: no keys held at the kill, so the release proves nothing".into());
    }
    kill(&mut child);
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        let st = client.state();
        let stopped = st.mode == TasMode::Off as u32
            && st.cont_suppress_input == 0
            && st.restart_state != 1
            && st.owner_pid == 0
            && client.command_idle();
        if stopped {
            break;
        }
        if Instant::now() > deadline {
            return Err(format!(
                "{point}: 2 s after the owner died: mode={} protection={} restart_state={} owner_pid={} command={}",
                st.mode, st.cont_suppress_input, st.restart_state, st.owner_pid, st.command
            ));
        }
        thread::sleep(Duration::from_millis(5));
    }
    // Nothing the dead controller queued may land afterwards.
    thread::sleep(Duration::from_millis(1000));
    let st = client.state();
    if st.mode != TasMode::Off as u32 || st.cont_suppress_input != 0 {
        return Err(format!(
            "{point}: the TAS came back after the stop (mode={})",
            st.mode
        ));
    }
    let held = memory.held_tas_mask()?;
    let observed = memory.observer_tas_mask()?;
    if held != 0 || observed != 0 {
        return Err(format!(
            "{point}: keys still held after the stop (buffer {held:#04x}, observer {observed:#04x})"
        ));
    }
    let (entries, _) = client.state().read_log_entries(log_cursor);
    if !entries.iter().any(|(_, _, text)| text.contains(LOG_LINE)) {
        return Err(format!("{point}: the DLL did not log the owner's death"));
    }
    println!("     stopped, keys released, input unblocked, logged");
    Ok(())
}

fn reader_death_changes_nothing(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    println!("  -- a reader that never owned dies mid-replay");
    harness::stop(client);
    load_fixture(client)?;
    harness::restart_play_aligned_unwatched(client)?;
    let (mut child, _) = spawn_child("reader")?;
    kill(&mut child);
    thread::sleep(Duration::from_millis(1000));
    if client.mode_volatile() != TasMode::Play as u32 {
        return Err("the replay stopped when a non-owner died".into());
    }
    println!("     replay unaffected");
    harness::stop(client);
    Ok(())
}

fn second_controller_is_refused_while_the_owner_lives(
    client: &mut TasSharedMemoryClient,
) -> Result<(), String> {
    println!("  -- a second controller while the owner lives");
    harness::stop(client);
    load_fixture(client)?;
    let (mut child, pid) = spawn_child("replay")?;
    let mut mine = TransportController::new(aligned_play(client));
    let refused = step_until_decided(&mut mine, client);
    kill(&mut child);
    match refused {
        StepOutcome::Aborted { reason } if reason.contains(&format!("pid {pid}")) => {
            println!("     refused: {reason}")
        }
        other => {
            return Err(format!(
                "expected a refusal naming pid {pid}, got {other:?}"
            ))
        }
    }
    // Once the owner is gone, this process may own.
    wait_for(
        || client.state().owner_pid == 0,
        "the dead owner to be dropped",
    )?;
    let mut next = TransportController::new(aligned_play(client));
    let outcome = step_until_decided(&mut next, client);
    let owned = client.state().owner_pid == std::process::id();
    next.release(client);
    if !matches!(outcome, StepOutcome::InProgress) || !owned {
        return Err(format!(
            "after the owner died, a new controller got {outcome:?}"
        ));
    }
    wait_for(|| client.state().owner_pid == 0, "the release")?;
    println!("     the successor owned it, then released");
    harness::stop(client);
    Ok(())
}

fn living_owner_keeps_its_block(client: &mut TasSharedMemoryClient) -> Result<(), String> {
    println!("  -- a living owner's block has no timeout");
    harness::stop(client);
    let (mut child, _) = spawn_child("hold")?;
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(12) {
        if client.state().cont_suppress_input != 1 {
            kill(&mut child);
            return Err(format!(
                "the block was lifted after {:.1} s with its owner alive",
                start.elapsed().as_secs_f32()
            ));
        }
        thread::sleep(Duration::from_millis(50));
    }
    kill(&mut child);
    wait_for(
        || client.state().cont_suppress_input == 0,
        "the block to lift after its owner died",
    )?;
    println!("     held 12 s while alive, lifted on death");
    Ok(())
}

/// Step until the ownership request is answered: refused (Aborted) or owned
/// and past it (InProgress with this process as the owner).
fn step_until_decided(
    c: &mut TransportController,
    client: &mut TasSharedMemoryClient,
) -> StepOutcome {
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        let outcome = c.step(client);
        if !matches!(outcome, StepOutcome::InProgress)
            || client.state().owner_pid == std::process::id()
            || Instant::now() > deadline
        {
            return outcome;
        }
        thread::sleep(Duration::from_millis(5));
    }
}

fn wait_for(cond: impl Fn() -> bool, what: &str) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(2);
    while !cond() {
        if Instant::now() > deadline {
            return Err(format!("timed out waiting for {what}"));
        }
        thread::sleep(Duration::from_millis(5));
    }
    Ok(())
}

fn aligned_play(client: &TasSharedMemoryClient) -> ArmConfig {
    let s = client.state();
    let gate =
        tas_shared::align::detect_first_moving(&s.rec_coords[..], s.recorded_count).unwrap_or(0);
    ArmConfig {
        arm: Arm::Play,
        speed: 1.0,
        continue_from_frame: 0,
        gate_align_rec: gate,
        max_retries: 0,
        input_model: s.input_model,
        trajectory_ticks: u32::MAX,
    }
}

fn spawn_child(point: &str) -> Result<(Child, u32), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let mut child = Command::new(exe)
        .args(["owner-child", point])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawning the owner child: {e}"))?;
    let pid = child.id();
    let stdout = child.stdout.take().ok_or("no child stdout")?;
    let mut lines = BufReader::new(stdout).lines();
    loop {
        match lines.next() {
            Some(Ok(line)) if line == "READY" => return Ok((child, pid)),
            Some(Ok(line)) => println!("     child: {line}"),
            _ => {
                let _ = child.kill();
                return Err(format!(
                    "the owner child ({point}) exited before it was ready"
                ));
            }
        }
    }
}

fn kill(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

/// The child: drive (or just map) until `point`, say READY, then wait to be
/// killed.
pub fn child(point: &str) -> bool {
    let mut client = match TasSharedMemoryClient::open() {
        Ok(c) => c,
        Err(e) => {
            println!("cannot open the mapping: {e}");
            return false;
        }
    };
    let ready = match point {
        "reader" => true,
        "hold" => hold_block(&mut client),
        _ => drive_to(&mut client, point),
    };
    if !ready {
        return false;
    }
    println!("READY");
    thread::sleep(Duration::from_secs(120));
    true
}

fn hold_block(client: &mut TasSharedMemoryClient) -> bool {
    let me = tas_shared::owner::current_process_identity();
    let seq = tas_shared::owner::owner_request_submit(
        client.state_mut(),
        tas_shared::TAS_OWNER_ACQUIRE,
        me,
    );
    let deadline = Instant::now() + Duration::from_secs(3);
    loop {
        match tas_shared::owner::owner_request_answer(client.state(), seq, me.pid) {
            Some(tas_shared::owner::OwnerAnswer::Owned) => break,
            Some(other) => {
                println!("ownership refused: {other}");
                return false;
            }
            None if Instant::now() > deadline => {
                println!("ownership never answered");
                return false;
            }
            None => thread::sleep(Duration::from_millis(5)),
        }
    }
    client.state_mut().cont_suppress_input = 1;
    true
}

fn drive_to(client: &mut TasSharedMemoryClient, point: &str) -> bool {
    let mut controller = TransportController::new(aligned_play(client));
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        if let outcome @ (StepOutcome::Aborted { .. } | StepOutcome::Done { .. }) =
            controller.step(client)
        {
            println!("the cycle ended before {point}: {outcome:?}");
            return false;
        }
        let st = client.state();
        let reached = match point {
            "restart" => st.restart_state == 1,
            "countdown" => {
                st.mode == TasMode::Play as u32 && st.playback_pos > 0 && st.gate_index == 0
            }
            // Past the gate, on a tick that holds keys, so the kill leaves
            // something to release.
            "replay" => {
                let rec_gate =
                    tas_shared::align::detect_first_moving(&st.rec_coords[..], st.recorded_count)
                        .unwrap_or(0);
                let index = (st.playback_pos + rec_gate).saturating_sub(st.gate_index) as usize;
                st.mode == TasMode::Play as u32
                    && st.gate_index != 0
                    && st.input_log.get(index).is_some_and(|m| m & 0x3F != 0)
            }
            _ => {
                println!("unknown point {point}");
                return false;
            }
        };
        if reached {
            // Never released: the parent kills this process.
            return true;
        }
        if Instant::now() > deadline {
            println!("never reached {point}");
            return false;
        }
        thread::sleep(Duration::from_millis(1));
    }
}
