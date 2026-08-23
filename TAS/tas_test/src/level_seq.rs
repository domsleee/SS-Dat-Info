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

/// How long to observe by default. Long enough to span several publisher cycles
/// (the DLL polls at 200ms while unresolved, 1.5s once settled) without being
/// tedious. `tas_test level-seq <secs>` soaks for longer — worth it on Village,
/// where two tracks share a resource path and the difficulty rests entirely on
/// the heap scan, so scan instability would show up as the context flapping in
/// and out of resolved.
const OBSERVE_DEFAULT_SECS: u64 = 4;

/// The grammar the DLL itself requires before it will believe a path. Mirrors
/// `levelpath::IsPlausible` — a spliced path fails it whenever the two tracks
/// differ before the "tracks" segment, which is the usual case.
fn plausible(path: &str) -> bool {
    let l = path.to_ascii_lowercase();
    l.contains("levels") && l.contains("tracks")
}

/// `tas_test level-seq watch [secs]` — an INSTRUMENT, not a gate.
///
/// Logs every level-context transition with a millisecond timestamp and asserts
/// nothing. `run()` deliberately fails on more than one id, because it assumes a
/// quiet track; this is for the opposite situation — you are about to change
/// level on purpose and want to see exactly what the DLL publishes and when.
///
/// What it answers: does quitting to the menu actually INVALIDATE (context goes
/// unresolved), or does the old track stay asserted through the teardown? That
/// distinction decides how bad the shared-path case (Village Easy and Village
/// Hard load the same `.../village/Tracks/easy/...`) really is — if every switch
/// passes through a teardown, the path never has to change for us to notice.
pub fn watch(secs: Option<u64>) -> bool {
    let observe = Duration::from_secs(secs.unwrap_or(30).max(1));
    // Attach directly — NOT through ensure_game_running(). That checks liveness,
    // and liveness is zero at exactly the moment this tool is for: the engine
    // cycle STOPS at a static menu, so the helper would refuse to connect
    // precisely while you are trying to watch a level change. Same reasoning as
    // the `gamestate` diagnostic: a tool for inspecting a wrong state must not
    // decline to run because the state is wrong.
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
    // Distinct ids seen resolved, and how often the context flipped between
    // resolved and unresolved. On a stable track both should be 1 and 0: the
    // DLL only republishes on a real change, and it goes unresolved when two
    // scans in one context CONTRADICT each other. So flapping here is the scan
    // being unable to make up its mind, which is exactly what you want to know
    // on Village, where the path cannot supply the difficulty.
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

    // Two different tracks inside one observation window means the scan is
    // contradicting itself. That does not corrupt anything — the DLL answers a
    // contradiction by going unresolved, and unresolved matches nothing — but it
    // means the difficulty is not actually being determined, which the user
    // would experience as the track chip flickering.
    if ids_seen.len() > 1 {
        eprintln!(
            "\nFAIL: the scan resolved to {} DIFFERENT tracks in one window: {:?}\n  \
             The level did not change (that would have shown as a path change), so \
             the heap scan is not converging. Raise MIN_TRACK_HITS / dominance in \
             level_scan.hpp, or the area hint is not constraining it.",
            ids_seen.len(),
            ids_seen
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
