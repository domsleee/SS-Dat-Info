//! UI break at 4500: the 46.69 Forest Easy run showed a permanent DRIFT
//! banner on F12 even though the run does not drift (CONT bucket matched,
//! no retries). Session logs showed `DRIFT first at tick 1831` with
//! run-varying X (e.g. `max X=0.785766602 Z=0.177551270`): brief jump
//! transients that heal long before the splice.
//!
//! `recording.tasrec` is history entry 2431 ("UI break at 4500", 5032 ticks).
//! The original capture script incorrectly assumed the last replay stopped
//! at 4500. `play-prefix.bin` actually contains a later playback tail whose
//! first 16 samples identify recording tick 945. It covers the 4500 boundary
//! but is not a complete zero-based CONT prefix. This test pins the evidence the
//! banner fix relies on: warning-scale transients from tick 1831 on, and a
//! bit-exact last replayed sample at tick 4499. The main.rs fixture regression
//! additionally drives the production scanner through PLAY -> REC.

use std::path::PathBuf;

const SPLICE: usize = 4500;

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("ui-break-at-4500")
}

fn read_recording() -> Vec<[f32; 3]> {
    let bytes = std::fs::read(fixture_dir().join("recording.tasrec"))
        .expect("fixture recording.tasrec missing; capture it with TAS/tools/capture_cont_fixture.ps1");
    assert!(bytes.len() >= 4, "fixture recording too short for a count");
    let count = u32::from_le_bytes(bytes[0..4].try_into().unwrap()) as usize;
    // Raw blob layout: `u32le count | input_log[count] | (f32le x,y,z)[count]`.
    assert_eq!(
        bytes.len(),
        4 + count * 13,
        "fixture recording must be a complete raw v2 blob"
    );
    let base = 4 + count;
    bytes[base..]
        .chunks_exact(12)
        .map(|c| {
            [
                f32::from_le_bytes(c[0..4].try_into().unwrap()),
                f32::from_le_bytes(c[4..8].try_into().unwrap()),
                f32::from_le_bytes(c[8..12].try_into().unwrap()),
            ]
        })
        .collect()
}

fn read_play_prefix() -> Vec<[f32; 3]> {
    let bytes = std::fs::read(fixture_dir().join("play-prefix.bin"))
        .expect("fixture play-prefix.bin missing; capture it with TAS/tools/capture_cont_fixture.ps1");
    assert_eq!(
        bytes.len(),
        SPLICE * 12,
        "play prefix must hold exactly SPLICE xyz triples"
    );
    bytes
        .chunks_exact(12)
        .map(|c| {
            [
                f32::from_le_bytes(c[0..4].try_into().unwrap()),
                f32::from_le_bytes(c[4..8].try_into().unwrap()),
                f32::from_le_bytes(c[8..12].try_into().unwrap()),
            ]
        })
        .collect()
}

#[test]
fn splice_point_agrees_exactly() {
    let rec = read_recording();
    let play = read_play_prefix();
    assert_eq!(rec.len(), 5032, "entry 2431 holds 5032 ticks");
    assert_eq!(play.len(), SPLICE);

    // The capture copies play_coords[played-SPLICE..played]; locate the live
    // tail inside the saved trajectory by a 16-sample probe run (a single
    // sample could coincide elsewhere).
    const PROBE: usize = 16;
    let mut offset = None;
    for start in 0..rec.len() {
        if start + PROBE > rec.len() || PROBE > play.len() {
            break;
        }
        if (0..PROBE).all(|i| play[i] == rec[start + i]) {
            offset = Some(start);
            break;
        }
    }
    let offset =
        offset.expect("live tail must start somewhere in the saved trajectory");
    assert_eq!(offset, 945, "pin the actual capture coverage");
    assert!(
        SPLICE > offset && offset + play.len() > SPLICE,
        "splice tick {SPLICE} must be covered by the live tail (offset={offset})"
    );

    // Pre-splice differences are warning-scale transients starting at the
    // logged tick 1831 — never "TAS INVALID" scale.
    let mut first: Option<(usize, f32, f32)> = None;
    let (mut max_dx, mut max_dz) = (0.0f32, 0.0f32);
    for i in 0..SPLICE - offset {
        let dx = (play[i][0] - rec[offset + i][0]).abs();
        let dz = (play[i][2] - rec[offset + i][2]).abs();
        if (dx > 0.0 || dz > 0.0) && first.is_none() {
            first = Some((offset + i, dx, dz));
        }
        max_dx = max_dx.max(dx);
        max_dz = max_dz.max(dz);
    }
    let (tick, dx, dz) = first.expect("the logged 1831 transient must be present");
    assert_eq!(tick, 1831, "first pre-splice difference");
    assert!(
        (dx - 0.003112793).abs() < 1e-6 && (dz - 0.051513672).abs() < 1e-6,
        "transient scale must match the session log (dx={dx}, dz={dz})"
    );
    assert!(
        max_dx < 1.0 && max_dz < 1.0,
        "pre-splice wobble stays warning-scale (max X={max_dx} Z={max_dz})"
    );

    // The last replayed sample is 4499; tick 4500 is the first new REC sample.
    let endpoint = play[SPLICE - offset - 1];
    let saved = rec[SPLICE - 1];
    assert_eq!(endpoint, saved, "prefix endpoint at the 4500 boundary");
    assert_eq!((endpoint[0] - saved[0]).abs(), 0.0);
    assert_eq!((endpoint[2] - saved[2]).abs(), 0.0);
    assert_eq!(
        play[SPLICE - offset - 1],
        rec[SPLICE - 1],
        "prefix end agrees exactly too"
    );
}
