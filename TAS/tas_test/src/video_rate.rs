//! `tas_test video-rate [secs]` — measure how fast the game's picture actually
//! ADVANCES, from outside the process.
//!
//! Built to answer one question that shared memory cannot: how fast does the
//! menu background video run with NO TAS loaded? The DLL's own `present_count`
//! is the natural instrument, but it only exists once the DLL is injected — so
//! it can measure the suspect and never the control. This measures the screen
//! instead, so the SAME method works with TAS absent, with the DLL injected, and
//! with tas_ui also running.
//!
//! METHOD. Grab a small region of the game window as fast as Windows will allow,
//! hash the pixels, and timestamp every change. Distinct images per second is
//! the rate at which new frames are actually reaching the screen, which is what
//! "the video plays too fast / too slow" is a statement about.
//!
//! WHAT IT CANNOT SEE. The sampler is not free, so it has a ceiling: a true rate
//! above the sampling rate is undercounted. The report prints the sampling rate
//! next to the result for exactly this reason — treat a change-rate that lands
//! near the sample-rate as "at least this", not "this". Region choice matters
//! too: a patch of static UI never changes and would read 0, so the default sits
//! in the middle of the window where the video plays.
//!
//! Presented as intervals, not just an average, because the failure modes look
//! different: a Sleep-limiter running at the wrong timer resolution shifts the
//! whole distribution, while a stutter leaves the median alone and grows the
//! tail.

use std::time::{Duration, Instant};

type Handle = isize;

const SRCCOPY: u32 = 0x00CC_0020;
const DIB_RGB_COLORS: u32 = 0;

#[repr(C)]
#[derive(Clone, Copy)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct BitmapInfoHeader {
    size: u32,
    width: i32,
    height: i32,
    planes: u16,
    bit_count: u16,
    compression: u32,
    size_image: u32,
    x_ppm: i32,
    y_ppm: i32,
    clr_used: u32,
    clr_important: u32,
}

#[repr(C)]
struct BitmapInfo {
    header: BitmapInfoHeader,
    colors: [u32; 3],
}

// Split by DLL so the linker is told which import library each symbol lives in
// — without these the build fails with unresolved externals.
#[link(name = "user32")]
unsafe extern "system" {
    fn GetDesktopWindow() -> Handle;
    fn GetDC(hwnd: Handle) -> Handle;
    fn ReleaseDC(hwnd: Handle, hdc: Handle) -> i32;
    fn FindWindowA(cls: *const u8, name: *const u8) -> Handle;
    fn GetWindowRect(h: Handle, r: *mut Rect) -> i32;
    fn SetForegroundWindow(h: Handle) -> i32;
    fn BringWindowToTop(h: Handle) -> i32;
    fn ShowWindow(h: Handle, cmd: i32) -> i32;
    fn GetForegroundWindow() -> Handle;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn CreateCompatibleDC(hdc: Handle) -> Handle;
    fn DeleteDC(hdc: Handle) -> i32;
    fn CreateCompatibleBitmap(hdc: Handle, w: i32, h: i32) -> Handle;
    fn DeleteObject(o: Handle) -> i32;
    fn SelectObject(hdc: Handle, o: Handle) -> Handle;
    fn BitBlt(
        dst: Handle,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        src: Handle,
        sx: i32,
        sy: i32,
        rop: u32,
    ) -> i32;
    fn GetDIBits(
        hdc: Handle,
        hbm: Handle,
        start: u32,
        lines: u32,
        bits: *mut u8,
        bi: *mut BitmapInfo,
        usage: u32,
    ) -> i32;
}

const GAME_TITLE: &[u8] = b"Supreme Snowboarding Copyright (C) 1999 by Housemarque, Inc.\0";

/// Sample patch. Small enough that a grab costs well under a millisecond (the
/// sampling ceiling is the whole limit on what this can see), big enough that a
/// frame change is guaranteed to touch it.
const PATCH_W: i32 = 192;
const PATCH_H: i32 = 144;

/// Find the game window, bring it to the FRONT, and return its rect.
///
/// Focusing is not a convenience — the game does not render while it is in the
/// background, so its window composites as blank white and every frame reads
/// identical. The first run of this tool measured "0 distinct frames" for
/// exactly that reason, which looks indistinguishable from "the video is
/// frozen". Doing it here makes the measurement self-contained instead of
/// depending on whoever runs it remembering to click the game first.
fn focus_game_and_rect() -> Option<Rect> {
    unsafe {
        let hwnd = FindWindowA(std::ptr::null(), GAME_TITLE.as_ptr());
        if hwnd == 0 {
            return None;
        }
        ShowWindow(hwnd, 9); // SW_RESTORE
        BringWindowToTop(hwnd);
        SetForegroundWindow(hwnd);
        // Give the game a beat to repaint once it is actually frontmost.
        std::thread::sleep(Duration::from_millis(900));
        if GetForegroundWindow() != hwnd {
            eprintln!(
                "  WARNING: could not bring the game to the front. It does not render\n  \
                 in the background, so the measurement below is probably all zeros."
            );
        }
        let mut r = Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        if GetWindowRect(hwnd, &mut r) == 0 {
            return None;
        }
        Some(r)
    }
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
            let screen = GetDC(GetDesktopWindow());
            let mem = CreateCompatibleDC(screen);
            let bmp = CreateCompatibleBitmap(screen, PATCH_W, PATCH_H);
            let old = SelectObject(mem, bmp);
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
            BitBlt(self.mem, 0, 0, PATCH_W, PATCH_H, self.screen, x, y, SRCCOPY);
            GetDIBits(
                self.mem,
                self.bmp,
                0,
                PATCH_H as u32,
                self.buf.as_mut_ptr(),
                &mut self.bi,
                DIB_RGB_COLORS,
            );
        }
        fnv1a(&self.buf)
    }
}

impl Drop for Grabber {
    fn drop(&mut self) {
        unsafe {
            SelectObject(self.mem, self.old);
            DeleteObject(self.bmp);
            DeleteDC(self.mem);
            ReleaseDC(GetDesktopWindow(), self.screen);
        }
    }
}

/// Probe a grid over the window and return the origin of the patch that changes
/// most — i.e. where the video actually is, rather than where we assumed.
fn pick_liveliest_patch(r: &Rect) -> (i32, i32) {
    const COLS: i32 = 5;
    const ROWS: i32 = 4;
    const PROBE_MS: u64 = 420;

    let w = r.right - r.left;
    let h = r.bottom - r.top;
    let mut best = (r.left + w / 2 - PATCH_W / 2, r.top + h / 2 - PATCH_H / 2);
    let mut best_changes = -1i32;
    let mut g = Grabber::new();

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

/// `--cap N` sets the DLL's `menu_fps_cap` before measuring.
///
/// A true no-TAS baseline turned out to be unreachable on this machine: a
/// service auto-injects TAS_Helper into every launch, and with the DLL moved
/// aside the game starts but never creates a window at all — the launch pipeline
/// needs the injection to succeed. So the honest comparison available is between
/// the DLL's throttle ON and OFF, which the DLL itself supports: `menu_fps_cap`
/// is documented as "0 = OFF (no throttle, pure measurement)". That isolates the
/// present cap, though NOT the other half of the story — the 1 ms system timer
/// the tooling raises, which is what sped sr.dll's Sleep-limiter up in the first
/// place. Measuring that half needs a game that can run without the DLL.
pub fn run_with_cap(secs: Option<u64>, cap: Option<u32>, region: Option<(i32, i32)>) -> bool {
    if let Some(c) = cap {
        match tas_shared::TasSharedMemoryClient::open() {
            Ok(mut client) => {
                let prev = client.state().menu_fps_cap;
                client.state_mut().menu_fps_cap = c;
                println!(
                    "  menu_fps_cap: {} -> {}{}",
                    prev,
                    c,
                    if c == 0 { "  (throttle OFF)" } else { "" }
                );
                // The cap is read per-present, so give the game a few frames to
                // settle onto the new rate before sampling it.
                std::thread::sleep(Duration::from_millis(600));
            }
            Err(e) => {
                eprintln!("ERROR: --cap needs TAS shared memory ({})", e);
                return false;
            }
        }
    }
    run_region(secs, region)
}

pub fn run(secs: Option<u64>) -> bool {
    run_region(secs, None)
}

pub fn run_region(secs: Option<u64>, region: Option<(i32, i32)>) -> bool {
    let observe = Duration::from_secs(secs.unwrap_or(6).max(1));

    let Some(r) = focus_game_and_rect() else {
        eprintln!(
            "ERROR: no Supreme window found.\n  \
             Looked for the exact title {:?}. Is the game running?",
            String::from_utf8_lossy(&GAME_TITLE[..GAME_TITLE.len() - 1])
        );
        return false;
    };
    // FIND THE MOVING PART OF THE PICTURE. Do not assume where it is.
    //
    // The first version sampled the window's centre on the theory that "the
    // video plays behind everything, so the middle must be moving". On the
    // Arcade menu the centre lands squarely on the Pipe/Air BUTTONS — static UI
    // — while the actual video (a snowboarder) is up in the top right. It still
    // produced plausible-looking numbers, because button edges and a sliver of
    // sky change a bit, so nothing flagged that the measurement was pointed at
    // the wrong thing. A metric that silently measures the wrong pixels is worse
    // than one that fails.
    //
    // So: probe a grid, and pick the patch that actually changes the most. The
    // chosen coordinates are printed, so the answer to "what did you measure?"
    // is in the output rather than in an assumption.
    // An explicit region wins over the auto-pick: "liveliest" is not always the
    // thing you mean. On the Arcade menu the liveliest patch is the animated
    // logo strip, while the background VIDEO — the snowboarder — is elsewhere
    // and is what the complaint is actually about.
    let (cx, cy) = match region {
        Some((x, y)) => {
            println!("  region: ({},{}) — explicitly requested", x, y);
            (x, y)
        }
        None => pick_liveliest_patch(&r),
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
    // Say up front whether the DLL is in the process — this is the whole point
    // of the comparison, and reading it from the run itself beats trusting the
    // operator to remember which condition they were in.
    //
    // From the MODULE LIST, not from opening the shared memory. A named section
    // outlives its creator while any handle stays open, so with tas_ui running
    // the mapping of a long-dead game still opens — which reported "TAS present"
    // against a game that had never loaded the DLL. That false positive would
    // have quietly invalidated the entire comparison.
    match crate::level_hunt::tas_dll_loaded() {
        Some(true) => println!("  TAS_Helper.dll: LOADED in the game process"),
        Some(false) => println!("  TAS_Helper.dll: NOT loaded — this is a clean baseline"),
        None => println!("  TAS_Helper.dll: unknown (could not inspect the process)"),
    }

    // When the DLL is present, also read its own present counter. The screen
    // metric counts CONTENT CHANGES; present_count counts SwapBuffers calls.
    // They are not the same number, and conflating them is easy: if the menu
    // redraws the same image twice, the screen sees one change and the engine
    // made two presents. Reporting both says which of the two the throttle is
    // actually acting on.
    let shm = tas_shared::TasSharedMemoryClient::open().ok();
    let presents_before = shm.as_ref().map(|c| c.state().present_count);
    // The game's own tick/frame counters. If the menu's animation is advancing
    // too fast, the question is whether the TICK SOURCE is running fast — which
    // these answer directly, where the screen metric only shows the consequence.
    let ticks_before = shm.as_ref().map(|c| c.state().tick_count);
    let frames_before = shm.as_ref().map(|c| c.state().frame_count);

    let mut changes: Vec<Instant> = Vec::with_capacity(4096);
    let mut samples = 0u64;
    let mut last_hash = 0u64;

    unsafe {
        let screen = GetDC(GetDesktopWindow());
        let mem = CreateCompatibleDC(screen);
        let bmp = CreateCompatibleBitmap(screen, PATCH_W, PATCH_H);
        let old = SelectObject(mem, bmp);

        let mut bi = BitmapInfo {
            header: BitmapInfoHeader {
                size: std::mem::size_of::<BitmapInfoHeader>() as u32,
                width: PATCH_W,
                // Negative = top-down rows. Only consistency matters for hashing.
                height: -PATCH_H,
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
        };
        let mut buf = vec![0u8; (PATCH_W * PATCH_H * 4) as usize];

        let start = Instant::now();
        while start.elapsed() < observe {
            BitBlt(mem, 0, 0, PATCH_W, PATCH_H, screen, cx, cy, SRCCOPY);
            GetDIBits(
                mem,
                bmp,
                0,
                PATCH_H as u32,
                buf.as_mut_ptr(),
                &mut bi,
                DIB_RGB_COLORS,
            );
            let h = fnv1a(&buf);
            samples += 1;
            if h != last_hash {
                last_hash = h;
                changes.push(Instant::now());
            }
        }

        SelectObject(mem, old);
        DeleteObject(bmp);
        DeleteDC(mem);
        ReleaseDC(GetDesktopWindow(), screen);
    }

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
    if let (Some(before), Some(c)) = (presents_before, shm.as_ref()) {
        let presents = c.state().present_count.wrapping_sub(before);
        println!(
            "  engine presents: {} ({:.1} /s)  <- what the GAME draws (SwapBuffers)",
            presents,
            presents as f64 / elapsed
        );
        println!("  menu_fps_cap in effect: {}", c.state().menu_fps_cap);
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
