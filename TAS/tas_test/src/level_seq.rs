//! `tas_test level-seq`: the level-context seqlock is actually WIRED UP.
//!
//! A seqlock writer that is never called is invisible to every other check:
//! the fields are still written, the sequence sits at 0, and a reader that
//! sees an even, unchanged sequence accepts everything. So this asserts the
//! one thing a dead writer cannot fake, that the DLL has executed the
//! sequence:
//!
//!   1. SEQUENCE IS NON-ZERO. `Start()` publishes once during DLL init, so a
//!      live DLL always has seq >= 2.
//!   2. SEQUENCE IS EVEN at rest. Persistently odd means the writer died
//!      between its two increments and every reader is locked out.
//!   3. READS SUCCEED. A reader that rejects everything is never wrong.
//!   4. NO TORN PATH ESCAPES. Every non-empty path handed out must be a
//!      plausible level path; a splice of two tracks generally is not.
//!
//! Read-only: it never sends a command.

use crate::harness;
use std::time::{Duration, Instant};

/// Long enough to span several publisher cycles. `tas_test level-seq <secs>`
/// soaks for longer, which matters on Village where two tracks share a path
/// and scan instability shows as the context flapping.
const OBSERVE_DEFAULT_SECS: u64 = 4;

/// Mirrors the DLL's `levelpath::IsPlausible`; a spliced path fails it
/// whenever the two tracks differ before the "tracks" segment.
fn plausible(path: &str) -> bool {
    let l = path.to_ascii_lowercase();
    l.contains("levels") && l.contains("tracks")
}

/// `tas_test level-seq watch [secs]`: an instrument, not a gate. Logs every
/// level-context transition with a timestamp and asserts nothing, for when you
/// are about to change level on purpose.
pub fn watch(secs: Option<u64>) -> bool {
    let observe = Duration::from_secs(secs.unwrap_or(30).max(1));
    // Attach directly, not through ensure_game_running(): the engine cycle
    // stops at a static menu, so a liveness check would refuse exactly when
    // this tool is needed.
    let client = match tas_shared::TasSharedMemoryClient::open() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("ERROR: no TAS shared memory ({}). Is the DLL injected?", e);
            return false;
        }
    };
    let state = client.state();

    println!(
        "\n=== level-seq watch: {:?} — change the level now ===",
        observe
    );
    println!("  t(ms)  seq  state");

    let start = Instant::now();
    let mut last: Option<Option<(u32, String)>> = None;
    let mut transitions = 0usize;
    while start.elapsed() < observe {
        let ctx = tas_shared::level_context(state);
        if last.as_ref() != Some(&ctx) {
            let seq = state
                .level_ctx_seq
                .load(std::sync::atomic::Ordering::Acquire);
            match &ctx {
                Some((id, path)) => println!(
                    "  {:>6}  {:>3}  RESOLVED id={:#x} {:?}",
                    start.elapsed().as_millis(),
                    seq,
                    id,
                    path
                ),
                None => println!(
                    "  {:>6}  {:>3}  UNRESOLVED",
                    start.elapsed().as_millis(),
                    seq
                ),
            }
            if last.is_some() {
                transitions += 1;
            }
            last = Some(ctx);
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    println!("\n  {} transition(s) observed.", transitions);
    true
}

pub fn run(secs: Option<u64>) -> bool {
    let observe = Duration::from_secs(secs.unwrap_or(OBSERVE_DEFAULT_SECS).max(1));
    let client = harness::ensure_game_running();
    let state = client.state();

    println!("\n=== level-seq: is the seqlock actually engaged? ===");

    // --- 1. The writer has run at least once. ---
    let seq0 = state
        .level_ctx_seq
        .load(std::sync::atomic::Ordering::Acquire);
    println!("  level_ctx_seq at connect: {}", seq0);
    if seq0 == 0 {
        eprintln!(
            "\nFAIL: level_ctx_seq is 0 — the DLL has NEVER published through the \
             seqlock.\n  \
             Start() publishes during init, so a live DLL cannot be at 0: the \
             publishContext() calls are missing, or an older TAS_Helper.dll is loaded.\n  \
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
    // On a stable track one id is seen and the context never flips: the DLL
    // republishes only on a real change and goes unresolved when two scans
    // contradict each other.
    let mut ids_seen: Vec<u32> = Vec::new();
    let mut resolved_flips = 0usize;
    let mut was_resolved: Option<bool> = None;

    let start = Instant::now();
    while start.elapsed() < observe {
        samples += 1;
        let seq = state
            .level_ctx_seq
            .load(std::sync::atomic::Ordering::Acquire);
        if seq & 1 != 0 {
            odd_samples += 1;
        }
        if seq != last_seq {
            advances += 1;
            last_seq = seq;
        }

        let ctx = tas_shared::level_context(state);
        let resolved_now = ctx.is_some();
        if was_resolved.is_some_and(|prev| prev != resolved_now) {
            resolved_flips += 1;
        }
        was_resolved = Some(resolved_now);

        match ctx {
            Some((id, path)) => {
                reads_ok += 1;
                if !ids_seen.contains(&id) {
                    ids_seen.push(id);
                }
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
        samples, observe, reads_ok, reads_none, advances, odd_samples
    );
    println!(
        "  ids seen resolved: {:?} | resolved<->unresolved flips: {}",
        ids_seen, resolved_flips
    );
    match &last_ctx {
        Some((id, path)) => println!("  last context: id={:#x} path={:?}", id, path),
        None => println!("  last context: UNRESOLVED (menu, or the scan has not settled)"),
    }

    let mut ok = true;

    // A clean read that is not a level path is a torn read that escaped.
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

    // A reader that always returns None never splices either.
    if reads_ok == 0 {
        eprintln!(
            "\nFAIL: not one clean read in {} attempts.\n  \
             Either the sequence is stuck ODD (a writer killed between its two \
             increments locks out every reader), or the level is unresolved for the \
             whole window. Check `tas_test shm`: if the level is unresolved the scan \
             has not identified the track, which is not a seqlock fault — get into a \
             race and re-run.",
            samples
        );
        ok = false;
    }

    // Two tracks in one window means the scan contradicts itself: the DLL
    // answers by going unresolved, which the user sees as a flickering chip.
    if ids_seen.len() > 1 {
        eprintln!(
            "\nFAIL: the scan resolved to {} DIFFERENT tracks in one window: {:?}\n  \
             The level did not change (that would have shown as a path change), so \
             the level scan is not converging.",
            ids_seen.len(),
            ids_seen
        );
        ok = false;
    }

    // Persistently odd = wedged writer; transiently odd is the protocol working.
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
