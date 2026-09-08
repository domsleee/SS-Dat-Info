//! Shared steered REC/PLAY procedure. Case data changes coverage, not failure handling.
use crate::{drift, gates, harness, patterns};
use tas_shared::TasSharedMemoryClient;

pub struct Case {
    pub label: &'static str,
    pub rec_speed: f32,
    pub play_speed: f32,
    pub tail_ticks: u32,
    pub iterations: u32,
    pub require_movement_gates: bool,
}

fn cycle(client: &mut TasSharedMemoryClient, case: &Case) -> Result<bool, String> {
    client.state_mut().playback_speed = case.rec_speed;
    if !harness::restart_and_stabilize(client) {
        return Err("REC restart failed".into());
    }
    client.state_mut().playback_speed = case.rec_speed;
    harness::arm_rec(client);
    let steps =
        patterns::with_neutral_tail(patterns::build_from_pattern("LRLR", 56, 0), case.tail_ticks);
    harness::drive_pico_steps(&steps)?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    let rec_speed = client.state().playback_speed;
    let count = client.recorded_count_volatile();
    let start = client.state().rec_coords[0];
    harness::stop(client);
    if count < 100 {
        return Err(format!("Only {count} ticks recorded"));
    }
    let input = &client.state().input_log[..count.min(tas_shared::TAS_MAX_TICKS as u32) as usize];
    if !input
        .iter()
        .any(|mask| mask & tas_shared::input_bits::LEFT != 0)
        || !input
            .iter()
            .any(|mask| mask & tas_shared::input_bits::RIGHT != 0)
    {
        return Err("Recording did not capture both driven steering directions".into());
    }
    if rec_speed != case.rec_speed {
        return Err("REC speed was overwritten".into());
    }
    client.state_mut().playback_speed = case.play_speed;
    if !harness::restart_play_and_match(client, start, harness::START_MATCH_RETRIES) {
        return Err("PLAY start did not match".into());
    }
    if !harness::wait_playback(client, count) {
        return Err("PLAY did not complete".into());
    }
    if client.state().playback_speed != case.play_speed {
        return Err("PLAY speed was overwritten".into());
    }
    let drift = drift::compute_drift(client.state(), count);
    let assessment = gates::run_gates(client.state(), count);
    assessment.print_summary();
    println!(
        "{}: {count} ticks, drift X={} Y={} Z={}",
        case.label, drift.max_drift_x, drift.max_drift_y, drift.max_drift_z
    );
    Ok(drift.is_zero() && (!case.require_movement_gates || assessment.all_pass()))
}

pub fn run(cases: &[Case]) -> bool {
    if cases.is_empty()
        || cases.iter().any(|c| {
            c.iterations == 0
                || !c.rec_speed.is_finite()
                || !c.play_speed.is_finite()
                || c.rec_speed <= 0.0
                || c.play_speed <= 0.0
        })
    {
        eprintln!("Invalid or empty cycle plan");
        return false;
    }
    if let Err(error) = harness::PicoKeys::open_checked() {
        eprintln!("{error}");
        return false;
    }
    let mut client = harness::ensure_game_running();
    harness::ensure_exclusive_runtime_ownership(&mut client, "steered REC/PLAY cycles");
    if !harness::require_speed_preconditions(&client) {
        return false;
    }
    for case in cases {
        for iteration in 1..=case.iterations {
            println!(
                "{} {iteration}/{}: REC {}x, PLAY {}x",
                case.label, case.iterations, case.rec_speed, case.play_speed
            );
            let result = cycle(&mut client, case);
            // Cleanup also runs for failed restart, steering, match and completion.
            harness::stop(&mut client);
            client.state_mut().playback_speed = 1.0;
            match result {
                Ok(true) => (),
                Ok(false) => return false,
                Err(error) => {
                    eprintln!("{} failed: {error}", case.label);
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_or_invalid_plans_fail_before_hardware_access() {
        assert!(!run(&[]));
        for (iterations, speed) in [
            (0, 1.0),
            (1, 0.0),
            (1, -1.0),
            (1, f32::NAN),
            (1, f32::INFINITY),
        ] {
            assert!(!run(&[Case {
                label: "invalid",
                rec_speed: speed,
                play_speed: speed,
                tail_ticks: 50,
                iterations,
                require_movement_gates: true
            }]));
        }
    }
}
