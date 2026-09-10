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

    let target = loaded.rec_coords[0];
    println!(
        "Target start position: ({:.6}, {:.6}, {:.6})",
        target[0], target[1], target[2]
    );

    if !harness::restart_play_and_match_inprocess(&mut client, target, harness::START_MATCH_RETRIES)
    {
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
    set_u32(meta_obj, "force_fixed_tick", 0);
    set_f32(meta_obj, "max_drift_x", 0.0)?;
    set_f32(meta_obj, "max_drift_z", 0.0)?;
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
        &state.play_coords[..expected],
    )?;
    println!("Saved refreshed baseline to {}", out);
    Ok(())
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
