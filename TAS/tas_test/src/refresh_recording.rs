use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Number, Value};

use crate::{harness, replay};

fn set_u32(meta: &mut serde_json::Map<String, Value>, key: &str, value: u32) {
    meta.insert(key.to_string(), Value::Number(Number::from(value as u64)));
}

/// Overwrite the identity keys of a refreshed baseline with the words of the
/// game that produced its new coordinates. `Value::Null` clears a stale
/// source value when the live half is unknown — never inherited.
fn stamp_refresh_identity(
    meta: &mut serde_json::Map<String, Value>,
    renderer_id: u32,
    fpu_control_word: u32,
    rider_character: u32,
    rider_stance: u32,
) {
    use tas_shared::{character_name, renderer_name, TAS_CHARACTER_UNKNOWN, TAS_RENDERER_UNKNOWN};
    meta.insert(
        "renderer".to_string(),
        match renderer_id {
            TAS_RENDERER_UNKNOWN => Value::Null,
            id => Value::String(renderer_name(id).to_string()),
        },
    );
    meta.insert(
        "fpu_control_word".to_string(),
        match fpu_control_word {
            0 => Value::Null,
            v => Value::Number(Number::from(v as u64)),
        },
    );
    meta.insert(
        "character".to_string(),
        match rider_character {
            TAS_CHARACTER_UNKNOWN => Value::Null,
            id => Value::String(character_name(id).to_string()),
        },
    );
    meta.insert(
        "stance".to_string(),
        match rider_stance {
            u32::MAX => Value::Null,
            v => Value::Number(Number::from(v as u64)),
        },
    );
}

pub fn run(source: &str, out: &str) -> Result<(), String> {
    println!("=== Refresh .tasrec Baseline ===");
    println!("Source: {}", source);
    println!("Output: {}", out);

    let loaded = replay::load_tasrec(Path::new(source))?;
    if loaded.count == 0 {
        return Err("source recording is empty".into());
    }

    println!(
        "Loaded: {} ticks, fft={}",
        loaded.count, loaded.meta.force_fixed_tick
    );

    let mut client = harness::ensure_game_running();
    harness::print_status(&client);

    harness::ensure_exclusive_runtime_ownership(&mut client, "baseline refresh");
    replay::write_to_shared(&mut client, &loaded);

    let (rec_gate, play_gate) = restart_play_aligned_unwatched(&mut client, &loaded)?;
    let owed = loaded.count.saturating_sub(rec_gate);
    if !harness::wait_playback(&client, play_gate + owed) {
        return Err("playback did not complete during refresh".into());
    }

    let state = client.state();
    let expected = loaded.count as usize;
    if state.playback_pos < play_gate + owed {
        return Err(format!(
            "playback shortfall during refresh: {}/{}",
            state.playback_pos,
            play_gate + owed
        ));
    }
    // The live gate can land on a different tick from the recording's, so
    // re-index the live trajectory to the recording's gate. Before the gate
    // the boarder is stationary at the spawn.
    let coords = gate_shifted(&state.play_coords, rec_gate, play_gate, expected);

    println!(
        "Refreshed baseline start: ({:.6}, {:.6}, {:.6})",
        coords[0][0], coords[0][1], coords[0][2]
    );

    let mut raw_meta = loaded.raw_meta;
    let meta_obj = raw_meta
        .as_object_mut()
        .ok_or_else(|| "source metadata is not a JSON object".to_string())?;
    set_u32(meta_obj, "recorded_count", loaded.count);
    set_u32(meta_obj, "force_fixed_tick", 0);
    // The output coordinates were just captured from the live game, so the
    // output identity must be the live identity — not the source file's.
    // Unknown live halves clear stale source values instead of inheriting
    // them (the inverse of save-copy, which preserves the take's identity).
    stamp_refresh_identity(
        meta_obj,
        state.renderer_id,
        state.fpu_control_word,
        state.rider_character,
        state.rider_stance,
    );
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| format!("system clock error: {}", e))?
        .as_secs()
        .to_string();
    meta_obj.insert("timestamp".to_string(), Value::String(timestamp));

    replay::save_tasrec(
        Path::new(out),
        &raw_meta,
        &loaded.input_log[..expected],
        &coords,
    )?;
    println!("Saved refreshed baseline to {}", out);
    Ok(())
}

/// Aligned PLAY WITHOUT the bit-exact watcher. A refresh exists to capture a
/// trajectory that changed (new physics, a renderer switch), which is exactly
/// what the watcher would reject. Returns `(recording gate, live gate)`.
fn restart_play_aligned_unwatched(
    client: &mut tas_shared::TasSharedMemoryClient,
    loaded: &replay::LoadedRecording,
) -> Result<(u32, u32), String> {
    let rec_gate = tas_shared::cont::detect_first_moving(&loaded.rec_coords, loaded.count)
        .ok_or("source recording never leaves the spawn")?;
    if !harness::restart_and_stabilize_inprocess(client) {
        return Err("in-process restart failed".into());
    }
    client.state_mut().gate_align_rec = rec_gate;
    harness::arm_play(client);
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    loop {
        let play_gate = client.state().gate_index;
        if play_gate != 0 {
            println!(
                "  Refresh aligned: recording gate {}, live gate {}",
                rec_gate, play_gate
            );
            return Ok((rec_gate, play_gate));
        }
        if std::time::Instant::now() > deadline {
            return Err("aligned PLAY never observed a live gate".into());
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
}

/// `count` coordinates in recording index space: the spawn up to `rec_gate`,
/// then the live trajectory from `play_gate` on.
fn gate_shifted(play: &[[f32; 3]], rec_gate: u32, play_gate: u32, count: usize) -> Vec<[f32; 3]> {
    (0..count)
        .map(|i| match (i as u32).checked_sub(rec_gate) {
            Some(k) => play[(play_gate + k) as usize],
            None => play[0],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tas_shared::{TAS_CHARACTER_KEITH, TAS_RENDERER_OPENGL};

    fn meta_with_stamps() -> serde_json::Map<String, Value> {
        serde_json::json!({
            "renderer": "DirectX7",
            "fpu_control_word": 127u64,
            "character": "Vincent",
            "stance": 0u64,
        })
        .as_object()
        .unwrap()
        .clone()
    }

    #[test]
    fn gate_shift_reindexes_the_live_trajectory_to_the_recording_gate() {
        let spawn = [1.0, 2.0, 3.0];
        let mut play = vec![spawn; 10];
        play[4] = [5.0, 0.0, 0.0];
        play[5] = [6.0, 0.0, 0.0];
        let out = gate_shifted(&play, 6, 4, 8);
        assert_eq!(&out[..6], &[spawn; 6]);
        assert_eq!(out[6], [5.0, 0.0, 0.0]);
        assert_eq!(out[7], [6.0, 0.0, 0.0]);
    }

    #[test]
    fn refresh_takes_live_identity_not_source() {
        let mut meta = meta_with_stamps();
        stamp_refresh_identity(
            &mut meta,
            TAS_RENDERER_OPENGL,
            0x027F,
            TAS_CHARACTER_KEITH,
            1,
        );
        assert_eq!(meta["renderer"], Value::String("OpenGL".to_string()));
        assert_eq!(
            meta["fpu_control_word"],
            Value::Number(Number::from(0x027Fu64))
        );
        assert_eq!(meta["character"], Value::String("Keith".to_string()));
        assert_eq!(meta["stance"], Value::Number(Number::from(1u64)));
    }

    #[test]
    fn refresh_clears_stale_values_when_live_unknown() {
        let mut meta = meta_with_stamps();
        stamp_refresh_identity(
            &mut meta,
            tas_shared::TAS_RENDERER_UNKNOWN,
            0,
            tas_shared::TAS_CHARACTER_UNKNOWN,
            u32::MAX,
        );
        assert_eq!(meta["renderer"], Value::Null);
        assert_eq!(meta["fpu_control_word"], Value::Null);
        assert_eq!(meta["character"], Value::Null);
        assert_eq!(meta["stance"], Value::Null);
    }
}
