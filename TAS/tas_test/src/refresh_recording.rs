use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::{Number, Value};

use crate::{harness, replay};

fn set_u32(meta: &mut serde_json::Map<String, Value>, key: &str, value: u32) {
    meta.insert(key.to_string(), Value::Number(Number::from(value as u64)));
}

fn set_f32(meta: &mut serde_json::Map<String, Value>, key: &str, value: f32) -> Result<(), String> {
    let number = Number::from_f64(value as f64)
        .ok_or_else(|| format!("cannot serialize {}={} as JSON number", key, value))?;
    meta.insert(key.to_string(), Value::Number(number));
    Ok(())
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
        "Loaded: {} ticks, inject_mode={}, fft={}, force_direct={}",
        loaded.count, loaded.meta.inject_mode, loaded.meta.force_fixed_tick, loaded.meta.force_direct
    );

    let mut client = harness::connect();
    harness::print_status(&client);
    if !harness::check_liveness(&client) {
        return Err("Cave 2 not firing".into());
    }

    harness::ensure_exclusive_runtime_ownership(&mut client, "baseline refresh");
    replay::write_to_shared(&mut client, &loaded);

    let target = loaded.rec_coords[0];
    println!(
        "Target start position: ({:.6}, {:.6}, {:.6})",
        target[0], target[1], target[2]
    );

    if !harness::restart_play_and_match_inprocess(&mut client, target, 20) {
        return Err("could not position-match live playback for refresh".into());
    }

    if !harness::wait_playback(&client, loaded.count) {
        return Err("playback did not complete during refresh".into());
    }

    let state = client.state();
    let expected = loaded.count as usize;
    if state.playback_pos < loaded.count {
        return Err(format!(
            "playback shortfall during refresh: {}/{}",
            state.playback_pos, loaded.count
        ));
    }

    println!(
        "Refreshed baseline start: ({:.6}, {:.6}, {:.6})",
        state.play_coords[0][0], state.play_coords[0][1], state.play_coords[0][2]
    );
    if expected > 250 {
        println!(
            "Refreshed baseline @250: ({:.6}, {:.6}, {:.6})",
            state.play_coords[250][0], state.play_coords[250][1], state.play_coords[250][2]
        );
    }

    let mut raw_meta = loaded.raw_meta;
    let meta_obj = raw_meta
        .as_object_mut()
        .ok_or_else(|| "source metadata is not a JSON object".to_string())?;
    set_u32(meta_obj, "recorded_count", loaded.count);
    set_u32(meta_obj, "inject_mode", state.inject_mode);
    set_u32(meta_obj, "force_fixed_tick", 0);
    set_u32(meta_obj, "force_direct", state.force_direct);
    set_u32(meta_obj, "input_source", state.input_source);
    set_f32(meta_obj, "max_drift_x", 0.0)?;
    set_f32(meta_obj, "max_drift_z", 0.0)?;
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
        &state.play_coords[..expected],
    )?;
    println!("Saved refreshed baseline to {}", out);
    Ok(())
}
