//! F5 bucket characterization probe.
//!
//! Repeatedly F5-restarts the game and records the resulting "stabilized"
//! starting position AND rotation matrix. Outputs the distribution of buckets
//! the snowboarder lands in, to characterize the variance that drives
//! `restart_play_and_match` retry counts and CONT anchor mismatches.
//!
//! The cont-reliability test reveals that bit-identical position-0 matches
//! still produce divergent trajectories, suggesting the rotation/velocity
//! state is also bucket-quantized. This probe captures both to confirm.

use std::collections::BTreeMap;

type PositionKey = (u32, u32, u32);
type PositionVelocityKey = (u32, u32, u32, u32, u32, u32);
type FullKey = (u32, u32, u32, [u32; 9]);
type BucketList<'a, K> = Vec<(&'a K, &'a Vec<u32>)>;

use crate::harness;

#[derive(Debug, Clone, Copy)]
struct Sample {
    cycle: u32,
    frame: u32,
    pos: [f32; 3],
    rot: [f32; 9],
    vel: [f32; 3],
}

pub fn run(iterations: u32) {
    println!("=== F5 Bucket Probe ({} cycles) ===\n", iterations);
    let client = harness::ensure_game_running();
    harness::print_status(&client);

    let mut samples: Vec<Sample> = Vec::with_capacity(iterations as usize);

    for cycle in 1..=iterations {
        println!("\n--- Cycle {}/{} ---", cycle, iterations);
        if !harness::restart_and_stabilize(&client) {
            eprintln!("ERROR: Game not alive after F5; aborting probe");
            std::process::exit(1);
        }
        let frame = client.frame_count_volatile();
        let s = client.state();
        let pos = [s.player_x, s.player_y, s.player_z];
        let rot = s.rotation_matrix;
        let vel = [s.velocity_x, s.velocity_y, s.velocity_z];
        println!(
            "  Sample: frame={} pos=({:.6}, {:.6}, {:.6}) rot[0..3]=({:.4},{:.4},{:.4}) vel=({:.6},{:.6},{:.6})",
            frame, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2], vel[0], vel[1], vel[2]
        );
        samples.push(Sample {
            cycle,
            frame,
            pos,
            rot,
            vel,
        });
    }

    summarize(&samples);
}

fn pos_key(s: &Sample) -> (u32, u32, u32) {
    (s.pos[0].to_bits(), s.pos[1].to_bits(), s.pos[2].to_bits())
}

fn full_key(s: &Sample) -> (u32, u32, u32, [u32; 9]) {
    let mut rot_bits = [0u32; 9];
    for (i, v) in s.rot.iter().enumerate() {
        rot_bits[i] = v.to_bits();
    }
    (
        s.pos[0].to_bits(),
        s.pos[1].to_bits(),
        s.pos[2].to_bits(),
        rot_bits,
    )
}

fn pos_vel_key(s: &Sample) -> (u32, u32, u32, u32, u32, u32) {
    (
        s.pos[0].to_bits(),
        s.pos[1].to_bits(),
        s.pos[2].to_bits(),
        s.vel[0].to_bits(),
        s.vel[1].to_bits(),
        s.vel[2].to_bits(),
    )
}

fn summarize(samples: &[Sample]) {
    println!("\n========== Probe Summary ==========");
    println!("Total cycles: {}", samples.len());

    // Position-only buckets (what restart_play_and_match matches against).
    let mut pos_buckets: BTreeMap<PositionKey, Vec<u32>> = BTreeMap::new();
    for s in samples {
        pos_buckets.entry(pos_key(s)).or_default().push(s.cycle);
    }

    // Combined position+rotation buckets (what would actually need to match
    // for the post-arm trajectory to be deterministic).
    let mut full_buckets: BTreeMap<FullKey, Vec<u32>> = BTreeMap::new();
    for s in samples {
        full_buckets.entry(full_key(s)).or_default().push(s.cycle);
    }

    // Combined position+velocity buckets (the cont-reliability hypothesis).
    let mut pos_vel_buckets: BTreeMap<PositionVelocityKey, Vec<u32>> = BTreeMap::new();
    for s in samples {
        pos_vel_buckets
            .entry(pos_vel_key(s))
            .or_default()
            .push(s.cycle);
    }

    println!("Distinct position-only buckets:    {}", pos_buckets.len());
    println!("Distinct (pos+rotation) buckets:   {}", full_buckets.len());
    println!(
        "Distinct (pos+velocity) buckets:   {}",
        pos_vel_buckets.len()
    );

    if full_buckets.len() > pos_buckets.len() {
        println!(
            "\n>>> Multiple rotation states per position bucket. <<<\n\
             >>> This means cont-reliability's bit-identical position match isn't enough. <<<"
        );
    }
    if pos_vel_buckets.len() > pos_buckets.len() {
        println!(
            "\n>>> Multiple velocity states per position bucket. <<<\n\
             >>> Adding velocity match to start-match would filter these out. <<<"
        );
    }

    let mut pos_list: BucketList<'_, PositionKey> = pos_buckets.iter().collect();
    pos_list.sort_by_key(|entry| std::cmp::Reverse(entry.1.len()));

    println!("\n--- Position buckets ---");
    for (i, (key, cycles)) in pos_list.iter().enumerate() {
        let x = f32::from_bits(key.0);
        let y = f32::from_bits(key.1);
        let z = f32::from_bits(key.2);
        // For each position bucket, count how many distinct rotation states it has.
        let mut rot_in_pos: BTreeMap<[u32; 9], u32> = BTreeMap::new();
        for s in samples {
            if pos_key(s) == **key {
                let mut rot_bits = [0u32; 9];
                for (j, v) in s.rot.iter().enumerate() {
                    rot_bits[j] = v.to_bits();
                }
                *rot_in_pos.entry(rot_bits).or_default() += 1;
            }
        }
        println!(
            "  P#{:<2} [{:>3}/{}] pos=({:.6}, {:.6}, {:.6}) rotation-buckets={}",
            i + 1,
            cycles.len(),
            samples.len(),
            x,
            y,
            z,
            rot_in_pos.len()
        );
        for (j, (_, count)) in rot_in_pos.iter().enumerate() {
            println!("       R#{}: {} cycles", j + 1, count);
        }
    }

    // Hit probability summary: assume the "right" combined bucket is the most
    // common one (most likely matches the recording's true state).
    let mut full_list: BucketList<'_, FullKey> = full_buckets.iter().collect();
    full_list.sort_by_key(|entry| std::cmp::Reverse(entry.1.len()));

    if let Some((_, top)) = full_list.first() {
        let p = top.len() as f64 / samples.len() as f64;
        let p_fail_60 = (1.0 - p).powi(60);
        println!("\nIf you target the most-common (pos+rotation) bucket:");
        println!("  Hit probability per attempt: {:.3}", p);
        println!("  Expected attempts to first hit: {:.2}", 1.0 / p);
        println!("  Probability of needing >60 attempts: {:.6}", p_fail_60);
    }

    // Position spread.
    if !samples.is_empty() {
        let (mut xmin, mut xmax) = (f32::MAX, f32::MIN);
        let (mut ymin, mut ymax) = (f32::MAX, f32::MIN);
        let (mut zmin, mut zmax) = (f32::MAX, f32::MIN);
        for s in samples {
            xmin = xmin.min(s.pos[0]);
            xmax = xmax.max(s.pos[0]);
            ymin = ymin.min(s.pos[1]);
            ymax = ymax.max(s.pos[1]);
            zmin = zmin.min(s.pos[2]);
            zmax = zmax.max(s.pos[2]);
        }
        println!("\nPosition spread:");
        println!("  X: [{:.6}, {:.6}] range={:.6}", xmin, xmax, xmax - xmin);
        println!("  Y: [{:.6}, {:.6}] range={:.6}", ymin, ymax, ymax - ymin);
        println!("  Z: [{:.6}, {:.6}] range={:.6}", zmin, zmax, zmax - zmin);
    }

    // Frame-count parity check.
    if pos_buckets.len() > 1 {
        let mut even_in_pos: BTreeMap<(u32, u32, u32), (u32, u32)> = BTreeMap::new();
        for s in samples {
            let entry = even_in_pos.entry(pos_key(s)).or_insert((0, 0));
            if s.frame % 2 == 0 {
                entry.0 += 1;
            } else {
                entry.1 += 1;
            }
        }
        println!("\nFrame parity per position bucket (even / odd):");
        for (i, (key, _)) in pos_list.iter().enumerate().take(5) {
            let (even, odd) = even_in_pos.get(key).copied().unwrap_or((0, 0));
            let x = f32::from_bits(key.0);
            let z = f32::from_bits(key.2);
            println!(
                "  P#{} (X={:.3}, Z={:.3}): even={}, odd={}",
                i + 1,
                x,
                z,
                even,
                odd
            );
        }
    }
}
