//! `tas_test level-seq` — verify the level-context seqlock is actually WIRED UP.
//!
//! This test exists because of a specific failure that every other check
//! missed. The seqlock writer (`publishContext` in the DLL's `level_scan.hpp`)
//! was written, reviewed, commented, and committed — and never called. The
//! stores still went out the old way, the sequence sat at 0 forever, and the
//! Rust reader, seeing an even and unchanged sequence on every read, accepted
//! everything exactly as it had before. The protocol was inert and looked
//! perfect: the unit tests passed (they drive their own writer), the live runs
//! passed (the fields are still written, just unprotected), and the level
//! detection worked.
//!
//! A protocol whose absence is invisible is not protecting anything. So this
//! asserts the one thing a dead writer cannot fake — that the DLL has actually
//! executed the sequence.
//!
//! Checks, in order of how much they catch:
//!
//!   1. SEQUENCE IS NON-ZERO. `Start()` publishes once during DLL init, so a
//!      live DLL always has seq >= 2. Zero means no publication has EVER
//!      happened: the writer is unwired (the exact bug above) or the DLL
//!      predates the protocol. This alone fails on the broken build and passes
//!      on the fixed one.
//!
//!   2. SEQUENCE IS EVEN when sampled at rest. Odd means a write is in flight
//!      (fine, transient — we retry) or the writer died between its two
//!      increments (fatal: every reader is locked out forever). Persistently
//!      odd is the second case.
//!
//!   3. READS SUCCEED. The reader must actually obtain clean windows, not just
//!      reject everything. A protocol that never returns is trivially never
//!      wrong, and that is the failure mode a naive seqlock test walks into.
//!
//!   4. NO TORN PATH ESCAPES. Every non-empty path handed out must be a
//!      plausible level path. A splice of two different tracks generally is
//!      not — it is the tail of one grafted onto the head of another.
//!
//! Read-only and non-destructive: it never sends a command, so it is safe to
//! run against a session mid-experiment.

use crate::harness;
use std::time::{Duration, Instant};

/// How long to observe. Long enough to span several publisher cycles (the DLL
/// polls at 200ms while unresolved, 1.5s once settled) without being tedious.
const OBSERVE: Duration = Duration::from_secs(4);

/// The grammar the DLL itself requires before it will believe a path. Mirrors
/// `levelpath::IsPlausible` — a spliced path fails it whenever the two tracks
/// differ before the "tracks" segment, which is the usual case.
fn plausible(path: &str) -> bool {
    let l = path.to_ascii_lowercase();
    l.contains("levels") && l.contains("tracks")
}

pub fn run() -> bool {
    let client = harness::ensure_game_running();
    let state = client.state();

    println!("\n=== level-seq: is the seqlock actually engaged? ===");

    // --- 1. The writer has run at least once. ---
    let seq0 = state.level_ctx_seq.load(std::sync::atomic::Ordering::Acquire);
    println!("  level_ctx_seq at connect: {}", seq0);
    if seq0 == 0 {
        eprintln!(
            "\nFAIL: level_ctx_seq is 0 — the DLL has NEVER published through the \
             seqlock.\n  \
             Start() publishes during init, so a live DLL cannot be at 0. Either the \
             publishContext() calls are missing (the writer is dead code and the \
             protocol is inert), or this game has an older TAS_Helper.dll loaded.\n  \
             Redeploy with `just deploy_run` and re-run."
        );
        return false;
    }

    // --- 2..4. Observe. ---
    let mut reads_ok = 0usize;
    let mut reads_none = 0usize;
    let mut odd_samples = 0usize;
    let mut samples = 0usize;
    let mut advances = 0usize;
    let mut bad_path: Option<String> = None;
    let mut last_seq = seq0;
    let mut last_ctx: Option<(u32, String)> = None;

    let start = Instant::now();
    while start.elapsed() < OBSERVE {
        samples += 1;
        let seq = state.level_ctx_seq.load(std::sync::atomic::Ordering::Acquire);
        if seq & 1 != 0 {
            odd_samples += 1;
        }
        if seq != last_seq {
            advances += 1;
            last_seq = seq;
        }

        match tas_shared::level_context(state) {
            Some((id, path)) => {
                reads_ok += 1;
                if !path.is_empty() && !plausible(&path) && bad_path.is_none() {
                    bad_path = Some(path.clone());
                }
                last_ctx = Some((id, path));
            }
            None => reads_none += 1,
        }
        std::thread::sleep(Duration::from_millis(2));
    }

    println!(
        "  {} samples over {:?}: {} clean reads, {} rejected, {} sequence advances, \
         {} samples caught mid-write",
        samples,
        OBSERVE,
        reads_ok,
        reads_none,
        advances,
        odd_samples
    );
    match &last_ctx {
        Some((id, path)) => println!("  last context: id={:#x} path={:?}", id, path),
        None => println!("  last context: UNRESOLVED (menu, or the scan has not settled)"),
    }

    let mut ok = true;

    // A path that survived the seqlock but is not a level path means a torn read
    // escaped — the protocol is present but broken.
    if let Some(p) = bad_path {
        eprintln!(
            "\nFAIL: a clean read returned an implausible path: {:?}\n  \
             It passed the sequence check, so either the writer mutates the group \
             outside publishContext(), or the reader's window does not cover every \
             field it reads.",
            p
        );
        ok = false;
    }

    // Never getting a clean read is a failure even though nothing was "wrong":
    // a reader that always returns None never splices either, and would pass a
    // less careful version of this test.
    if reads_ok == 0 {
        eprintln!(
            "\nFAIL: not one clean read in {} attempts.\n  \
             Either the sequence is stuck ODD (a writer killed between its two \
             increments locks out every reader permanently — Start() is meant to \
             clear that on reinjection), or the level is unresolved for the whole \
             window. Check `tas_test status`: if level_epoch != level_scan_epoch \
             the scan simply has not identified the track, which is not a seqlock \
             fault — get into a race and re-run.",
            samples
        );
        ok = false;
    }

    // Persistently odd = wedged writer. Transiently odd is normal and expected;
    // this is deliberately a loose bound, because catching the writer mid-update
    // is the protocol working, not failing.
    if samples > 0 && odd_samples * 2 > samples {
        eprintln!(
            "\nFAIL: the sequence was odd in {}/{} samples — the writer looks wedged \
             mid-publish rather than momentarily busy.",
            odd_samples, samples
        );
        ok = false;
    }

    if ok {
        println!("\n*** level-seq PASSED: the seqlock is engaged and reads are coherent ***");
    }
    ok
}
