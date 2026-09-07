//! `tas_test video-rate [secs] [--at X Y]`: how fast the game's picture
//! ADVANCES, measured from outside the process.
//!
//! Shared-memory counters only exist once the DLL is injected, so they can
//! measure the suspect and never the control. This grabs a small region of the
//! game window as fast as Windows allows, hashes the pixels and timestamps
//! every change; distinct images per second is the rate at which new frames
//! reach the screen, with or without TAS loaded.
//!
//! The sampler is not free, so a true rate above the sampling rate is
//! undercounted; the sampling rate is printed next to the result for that
//! reason. Intervals are reported as percentiles because a limiter at the
//! wrong timer resolution shifts the whole distribution while a stutter only
//! grows the tail.

use std::time::{Duration, Instant};

use crate::win32::{self, BitmapInfo, BitmapInfoHeader, Handle, Rect};

/// Sample patch: small enough that a grab costs well under a millisecond, big
/// enough that a frame change is guaranteed to touch it.
const PATCH_W: i32 = 192;
const PATCH_H: i32 = 144;

/// Bring the game to the front and return its rect. The game does not render
/// in the background (its window composites blank), so an unfocused
/// measurement reads "0 distinct frames".
fn focus_game_and_rect() -> Option<Rect> {
    let hwnd = win32::find_game_window()?;
    let focused = win32::bring_to_front(hwnd);
    // Give the game a beat to repaint once it is frontmost.
    std::thread::sleep(Duration::from_millis(900));
    if !focused {
        eprintln!(
            "  WARNING: could not bring the game to the front. It does not render\n  \
             in the background, so the measurement below is probably all zeros."
        );
    }
    win32::window_rect(hwnd)
}

/// A reusable GDI capture surface, so probing many patches costs one setup.
struct Grabber {
    screen: Handle,
    mem: Handle,
    bmp: Handle,
    old: Handle,
    bi: BitmapInfo,
    buf: Vec<u8>,
}

impl Grabber {
    fn new() -> Self {
        unsafe {
            let screen = win32::GetDC(win32::GetDesktopWindow());
            let mem = win32::CreateCompatibleDC(screen);
            let bmp = win32::CreateCompatibleBitmap(screen, PATCH_W, PATCH_H);
            let old = win32::SelectObject(mem, bmp);
            Grabber {
                screen,
                mem,
                bmp,
                old,
                bi: BitmapInfo {
                    header: BitmapInfoHeader {
                        size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                        width: PATCH_W,
                        height: -PATCH_H, // top-down; only consistency matters
                        planes: 1,
                        bit_count: 32,
                        compression: 0,
                        size_image: 0,
                        x_ppm: 0,
                        y_ppm: 0,
                        clr_used: 0,
                        clr_important: 0,
                    },
                    colors: [0; 3],
                },
                buf: vec![0u8; (PATCH_W * PATCH_H * 4) as usize],
            }
        }
    }

    fn hash_at(&mut self, x: i32, y: i32) -> u64 {
        unsafe {
            win32::BitBlt(
                self.mem,
                0,
                0,
                PATCH_W,
                PATCH_H,
                self.screen,
                x,
                y,
                win32::SRCCOPY,
            );
            win32::GetDIBits(
                self.mem,
                self.bmp,
                0,
                PATCH_H as u32,
                self.buf.as_mut_ptr(),
                &mut self.bi,
                win32::DIB_RGB_COLORS,
            );
        }
        fnv1a(&self.buf)
    }
}

impl Drop for Grabber {
    fn drop(&mut self) {
        unsafe {
            win32::SelectObject(self.mem, self.old);
            win32::DeleteObject(self.bmp);
            win32::DeleteDC(self.mem);
            win32::ReleaseDC(win32::GetDesktopWindow(), self.screen);
        }
    }
}

/// Probe a grid over the window and return the origin of the patch that
/// changes most: where the video actually is, not where it was assumed to be
/// (on the Arcade menu the centre lands on static buttons).
fn pick_liveliest_patch(g: &mut Grabber, r: &Rect) -> (i32, i32) {
    const COLS: i32 = 5;
    const ROWS: i32 = 4;
    const PROBE_MS: u64 = 420;

    let w = r.right - r.left;
    let h = r.bottom - r.top;
    let mut best = (r.left + w / 2 - PATCH_W / 2, r.top + h / 2 - PATCH_H / 2);
    let mut best_changes = -1i32;

    for row in 0..ROWS {
        for col in 0..COLS {
            // Spread patch CENTRES evenly, then clamp so none hangs off-window.
            let cx = r.left + (w * (2 * col + 1)) / (2 * COLS) - PATCH_W / 2;
            let cy = r.top + (h * (2 * row + 1)) / (2 * ROWS) - PATCH_H / 2;
            let cx = cx.clamp(r.left, r.right - PATCH_W);
            let cy = cy.clamp(r.top, r.bottom - PATCH_H);

            let mut last = g.hash_at(cx, cy);
            let mut changes = 0i32;
            let t0 = Instant::now();
            while t0.elapsed() < Duration::from_millis(PROBE_MS) {
                let hh = g.hash_at(cx, cy);
                if hh != last {
                    last = hh;
                    changes += 1;
                }
            }
            if changes > best_changes {
                best_changes = changes;
                best = (cx, cy);
            }
        }
    }
    println!(
        "  liveliest patch: ({},{})  [{} changes in {}ms while probing a {}x{} grid]",
        best.0, best.1, best_changes, PROBE_MS, COLS, ROWS
    );
    if best_changes == 0 {
        eprintln!("  WARNING: nothing on screen changed anywhere — there is no video to measure.");
    }
    best
}

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
}

/// `region` is the top-left of the patch to sample; `None` auto-picks the
/// liveliest patch. An explicit region wins because "liveliest" is not always
/// the thing you mean (the animated logo strip beats the background video).
pub fn run(secs: Option<u64>, region: Option<(i32, i32)>) -> bool {
    let observe = Duration::from_secs(secs.unwrap_or(6).max(1));

    let Some(r) = focus_game_and_rect() else {
        eprintln!(
            "ERROR: no Supreme window found.\n  \
             Looked for the exact title {:?}. Is the game running?",
            String::from_utf8_lossy(&win32::GAME_TITLE[..win32::GAME_TITLE.len() - 1])
        );
        return false;
    };
    let mut grabber = Grabber::new();
    let (cx, cy) = match region {
        Some((x, y)) => {
            println!("  region: ({},{}) — explicitly requested", x, y);
            (x, y)
        }
        None => pick_liveliest_patch(&mut grabber, &r),
    };

    println!("\n=== video-rate: how fast is the picture advancing? ===");
    println!(
        "  window {}x{} at ({},{}) — sampling a {}x{} patch at its centre for {:?}",
        r.right - r.left,
        r.bottom - r.top,
        r.left,
        r.top,
        PATCH_W,
        PATCH_H,
        observe
    );
    let shm = tas_shared::TasSharedMemoryClient::open().ok();
    // The game's own tick/frame counters answer whether the TICK SOURCE runs
    // fast, where the screen metric only shows the consequence.
    let ticks_before = shm.as_ref().map(|c| c.state().tick_count);
    let frames_before = shm.as_ref().map(|c| c.state().frame_count);

    let mut changes: Vec<Instant> = Vec::with_capacity(4096);
    let mut samples = 0u64;
    let mut last_hash = 0u64;

    let start = Instant::now();
    while start.elapsed() < observe {
        let h = grabber.hash_at(cx, cy);
        samples += 1;
        if h != last_hash {
            last_hash = h;
            changes.push(Instant::now());
        }
    }
    drop(grabber);

    let elapsed = observe.as_secs_f64();
    let sample_hz = samples as f64 / elapsed;
    // The first "change" is just the first sample, not an observed transition.
    let transitions = changes.len().saturating_sub(1);
    let change_hz = transitions as f64 / elapsed;

    let mut gaps_ms: Vec<f64> = changes
        .windows(2)
        .map(|w| w[1].duration_since(w[0]).as_secs_f64() * 1000.0)
        .collect();
    gaps_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let pct = |p: f64| -> f64 {
        if gaps_ms.is_empty() {
            return f64::NAN;
        }
        let i = ((gaps_ms.len() - 1) as f64 * p).round() as usize;
        gaps_ms[i]
    };

    println!(
        "\n  sampled {} times ({:.0} Hz) — this is the CEILING on what can be seen",
        samples, sample_hz
    );
    println!(
        "  distinct frames: {} ({:.1} fps)  <- what the SCREEN shows",
        transitions, change_hz
    );
    if let Some(c) = shm.as_ref() {
        if let (Some(t0), Some(f0)) = (ticks_before, frames_before) {
            let dt = c.state().tick_count.wrapping_sub(t0);
            let df = c.state().frame_count.wrapping_sub(f0);
            println!(
                "  cave5 ticks: {} ({:.1} /s)   cave2 frames: {} ({:.1} /s)   speed={:.2} fixed_tick={}",
                dt,
                dt as f64 / elapsed,
                df,
                df as f64 / elapsed,
                c.state().playback_speed,
                c.state().force_fixed_tick
            );
        }
    }
    if !gaps_ms.is_empty() {
        println!(
            "  frame interval ms: p10 {:.1} | median {:.1} | p90 {:.1} | max {:.1}",
            pct(0.10),
            pct(0.50),
            pct(0.90),
            gaps_ms[gaps_ms.len() - 1]
        );
        println!("  -> median implies {:.1} fps", 1000.0 / pct(0.50));
    }

    if change_hz > sample_hz * 0.8 {
        println!(
            "\n  WARNING: the change rate is close to the sampling rate, so the true\n  \
             rate is probably HIGHER than reported — this is a lower bound."
        );
    }
    if transitions == 0 {
        eprintln!(
            "\nFAIL: the patch never changed. Either the game is showing a static\n  \
             screen with no video, or the window moved. Nothing to measure."
        );
        return false;
    }
    true
}
