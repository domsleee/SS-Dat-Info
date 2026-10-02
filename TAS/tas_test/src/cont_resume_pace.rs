//! A CONT must resume recording at the resume speed from its first tick. The
//! fast catch-up leaves the game clock behind wall time; if that backlog is
//! not dropped at the splice, the first ticks of the new take run as a burst
//! (up to 64 per frame) before the pace settles.
//!
//! For each resume speed: CONT FE-tremendous at 64x, then sample tick_count
//! for the first 1.5 s of REC and require every 50 ms window to stay near the
//! resume speed's native rate.

use std::thread;
use std::time::{Duration, Instant};

use tas_shared::TasMode;

use crate::{harness, replay};

const RECORDING: &str = "FE-tremendous.tasrec";
const SPLICE_FRAME: u32 = 2200;
const CATCHUP_SPEED: f32 = 64.0;
const RESUME_SPEEDS: [f32; 3] = [1.0, 2.0, 0.5];
const WINDOW: Duration = Duration::from_millis(50);
const SAMPLE_FOR: Duration = Duration::from_millis(1500);

/// Ticks a 50 ms window may hold at `speed`: twice the native 5 per window
/// (frame jitter can put two frames' ticks in one window), plus slack.
fn window_limit(speed: f32) -> u32 {
    (10.0 * speed).ceil() as u32 + 4
}

/// Ticks per consecutive `WINDOW`, for the failure report.
fn buckets(samples: &[(Duration, u32)]) -> Vec<u32> {
    let mut out = Vec::new();
    let mut start = 0;
    for (i, sample) in samples.iter().enumerate() {
        if sample.0 - samples[start].0 >= WINDOW {
            out.push(sample.1.wrapping_sub(samples[start].1));
            start = i;
        }
    }
    out
}

/// The largest tick count in any `WINDOW` of `(time, tick_count)` samples.
fn max_window(samples: &[(Duration, u32)]) -> u32 {
    let mut max = 0;
    let mut start = 0;
    for end in 0..samples.len() {
        while samples[end].0 - samples[start].0 > WINDOW {
            start += 1;
        }
        max = max.max(samples[end].1.wrapping_sub(samples[start].1));
    }
    max
}

fn resume_at(
    client: &mut tas_shared::TasSharedMemoryClient,
    rec: &replay::LoadedRecording,
    speed: f32,
) -> Result<(u32, u32, Vec<u32>), String> {
    harness::stop(client);
    replay::write_to_shared(client, rec);
    client.state_mut().cont_resume_speed = speed;
    client.state_mut().playback_speed = CATCHUP_SPEED;
    harness::restart_continue_and_splice_inprocess(client, SPLICE_FRAME, 30)
        .ok_or("the CONT cycle failed")?;
    let start = Instant::now();
    while client.mode_volatile() != TasMode::Rec as u32 {
        if start.elapsed() > Duration::from_secs(10) {
            return Err("the splice never switched to REC".into());
        }
        thread::sleep(Duration::from_millis(1));
    }
    let t0 = Instant::now();
    let mut samples = Vec::new();
    while t0.elapsed() < SAMPLE_FOR {
        samples.push((t0.elapsed(), client.state().tick_count));
        thread::sleep(Duration::from_millis(2));
    }
    harness::stop(client);
    let total = samples.last().unwrap().1.wrapping_sub(samples[0].1);
    Ok((max_window(&samples), total, buckets(&samples)))
}

pub fn run() -> bool {
    println!("=== CONT resume pace: no burst after the splice at any resume speed ===\n");
    let rec = match harness::fixture_path(RECORDING).and_then(|p| replay::load_tasrec(&p)) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return false;
        }
    };
    let mut client = harness::ensure_game_running();
    harness::stop_competing_tas_ui_writer();
    let original_speed = client.state().playback_speed;
    let mut passed = true;
    for speed in RESUME_SPEEDS {
        match resume_at(&mut client, &rec, speed) {
            Ok((max, total, per_window)) => {
                let limit = window_limit(speed);
                let ok = max <= limit;
                println!(
                    "  resume {speed}x: max {max} ticks/50 ms (limit {limit}), {total} ticks in {:.1} s -> {}",
                    SAMPLE_FOR.as_secs_f32(),
                    if ok { "OK" } else { "BURST" }
                );
                if !ok {
                    println!("    ticks per 50 ms: {:?}", per_window);
                }
                passed &= ok;
            }
            Err(e) => {
                eprintln!("  resume {speed}x: ERROR: {e}");
                passed = false;
            }
        }
    }
    client.state_mut().playback_speed = original_speed;
    client.state_mut().cont_resume_speed = 0.0;
    println!(
        "\n*** CONT-RESUME-PACE {} ***",
        if passed { "PASSED" } else { "FAILED" }
    );
    passed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_burst_shows_in_the_largest_window() {
        let ms = Duration::from_millis;
        let steady: Vec<_> = (0..20).map(|i| (ms(i * 10), i as u32)).collect();
        assert_eq!(max_window(&steady), 5);
        let mut burst = steady.clone();
        for sample in burst.iter_mut().skip(10) {
            sample.1 += 64;
        }
        assert!(max_window(&burst) > window_limit(1.0));
    }
}
