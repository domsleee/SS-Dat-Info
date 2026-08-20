//! Differential memory hunt for the engine's own level identity.
//!
//! The heap-string scan in the DLL is a heuristic: it tallies
//! `<area>/Tracks/<diff>` occurrences and majority-votes. It needs a confidence
//! floor, a settle gate and a root-change epoch to be trustworthy, and it still
//! cannot see Practice/Halfpipe/Ramp. All of that scaffolding exists because we
//! INFER the level rather than READ it.
//!
//! If the engine keeps the current track as an index (area, difficulty, or a
//! combined id), that value almost certainly lives in a module's WRITABLE STATIC
//! DATA rather than the heap — which means a stable `Module+offset` we can read
//! directly, forever, with no polling and no inference.
//!
//! This is Cheat Engine's differential scan without Cheat Engine: it reads the
//! game with `ReadProcessMemory`, so there is no kernel driver and none of the
//! DBK64 risk, and it is restricted to loaded module images so every hit is
//! already a static offset rather than a heap address that moves.
//!
//! Usage — `begin` on one track, switch tracks in the game, then `diff`:
//!
//! ```text
//! tas_test level-hunt begin
//!   (switch to a different track in the game)
//! tas_test level-hunt diff
//!   (switch again)
//! tas_test level-hunt diff     // survivors that changed BOTH times
//! ```
//!
//! Each `diff` keeps only addresses that changed since the previous capture, so
//! two switches usually cuts millions of words down to a handful.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

type Dword = u32;
type Handle = usize;

#[repr(C)]
#[derive(Clone, Copy)]
struct ModuleEntry32 {
    dw_size: Dword,
    th32_module_id: Dword,
    th32_process_id: Dword,
    glblcnt_usage: Dword,
    proccnt_usage: Dword,
    mod_base_addr: usize,
    mod_base_size: Dword,
    h_module: usize,
    sz_module: [u8; 256],
    sz_exe_path: [u8; 260],
}

unsafe extern "system" {
    fn CreateToolhelp32Snapshot(flags: Dword, pid: Dword) -> Handle;
    fn Module32First(snap: Handle, me: *mut ModuleEntry32) -> i32;
    fn Module32Next(snap: Handle, me: *mut ModuleEntry32) -> i32;
    fn CloseHandle(h: Handle) -> i32;
    fn OpenProcess(access: Dword, inherit: i32, pid: Dword) -> Handle;
    fn ReadProcessMemory(h: Handle, addr: usize, buf: *mut u8, size: usize, read: *mut usize)
        -> i32;
}

const TH32CS_SNAPMODULE: Dword = 0x08;
const TH32CS_SNAPMODULE32: Dword = 0x10;
const PROCESS_VM_READ: Dword = 0x0010;
const PROCESS_QUERY_INFORMATION: Dword = 0x0400;

/// Modules worth scanning: the engine and its main DLL hold the globals.
const WANTED: &[&str] = &["supreme_game.dll", "supreme.exe", "supreme_v1.035.exe"];

/// Only values this small are plausible as a track index (0..8), an area or
/// difficulty index (0..2), or a small combined id. Filtering here keeps the
/// diff readable instead of drowning it in unrelated churn.
const MAX_PLAUSIBLE: u32 = 16;

fn state_path() -> PathBuf {
    std::env::temp_dir().join("tas_level_hunt.tsv")
}

pub fn find_pid() -> Option<Dword> {
    let out = std::process::Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "(Get-Process -Name Supreme,Supreme_v1.035 -ErrorAction SilentlyContinue | \
             Select-Object -First 1).Id",
        ])
        .output()
        .ok()?;
    String::from_utf8_lossy(&out.stdout).trim().parse().ok()
}

struct Region {
    name: String,
    base: usize,
    size: usize,
}

fn modules(pid: Dword) -> Vec<Region> {
    let mut out = Vec::new();
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
        if snap == 0 || snap == usize::MAX {
            return out;
        }
        let mut me: ModuleEntry32 = std::mem::zeroed();
        me.dw_size = std::mem::size_of::<ModuleEntry32>() as Dword;
        let mut ok = Module32First(snap, &mut me);
        while ok != 0 {
            let end = me.sz_module.iter().position(|&c| c == 0).unwrap_or(0);
            let name = String::from_utf8_lossy(&me.sz_module[..end]).to_lowercase();
            if WANTED.iter().any(|w| name == *w) {
                out.push(Region {
                    name,
                    base: me.mod_base_addr,
                    size: me.mod_base_size as usize,
                });
            }
            ok = Module32Next(snap, &mut me);
        }
        CloseHandle(snap);
    }
    out
}

/// Read every module image and return `(module, offset) -> value`.
///
/// `plausible_only` filters to small values, which is right when building the
/// INITIAL candidate set (it keeps the first diff readable). It must be OFF when
/// re-reading known candidates: a candidate whose value leaves the range would
/// silently VANISH from the table rather than being reported, which is how a
/// falsified candidate can look like a surviving one.
fn capture_opt(pid: Dword, plausible_only: bool) -> HashMap<(String, usize), u32> {
    let mut map = HashMap::new();
    let h = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };
    if h == 0 {
        eprintln!("ERROR: OpenProcess failed (is the game running?)");
        return map;
    }
    let mods = modules(pid);
    if mods.is_empty() {
        eprintln!("ERROR: none of {:?} found in the process", WANTED);
    }
    for m in mods {
        let mut buf = vec![0u8; m.size];
        let mut got = 0usize;
        let ok = unsafe { ReadProcessMemory(h, m.base, buf.as_mut_ptr(), m.size, &mut got) };
        if ok == 0 || got == 0 {
            // Partial reads are normal (guard pages); only a total failure matters.
            eprintln!("  {}: read failed", m.name);
            continue;
        }
        println!(
            "  {}: base={:#x} size={} read={}",
            m.name, m.base, m.size, got
        );
        // 4-byte aligned words only: an index is aligned, and this cuts the
        // candidate set 4x before any filtering.
        let mut i = 0usize;
        while i + 4 <= got {
            let v = u32::from_le_bytes([buf[i], buf[i + 1], buf[i + 2], buf[i + 3]]);
            if !plausible_only || v <= MAX_PLAUSIBLE {
                map.insert((m.name.clone(), i), v);
            }
            i += 4;
        }
    }
    unsafe { CloseHandle(h) };
    map
}

fn capture(pid: Dword) -> HashMap<(String, usize), u32> {
    capture_opt(pid, true)
}

fn save(map: &HashMap<(String, usize), u32>) {
    let mut s = String::new();
    for ((m, off), v) in map {
        s.push_str(&format!("{}\t{}\t{}\n", m, off, v));
    }
    let _ = fs::write(state_path(), s);
}

fn load() -> HashMap<(String, usize), u32> {
    let mut map = HashMap::new();
    if let Ok(s) = fs::read_to_string(state_path()) {
        for line in s.lines() {
            let mut it = line.split('\t');
            if let (Some(m), Some(o), Some(v)) = (it.next(), it.next(), it.next()) {
                if let (Ok(o), Ok(v)) = (o.parse::<usize>(), v.parse::<u32>()) {
                    map.insert((m.to_string(), o), v);
                }
            }
        }
    }
    map
}

pub fn run(sub: &str) -> bool {
    let Some(pid) = find_pid() else {
        eprintln!("ERROR: game not running");
        return false;
    };
    println!("=== LEVEL HUNT ({}) — pid {} ===", sub, pid);

    match sub {
        "begin" => {
            let now = capture(pid);
            if now.is_empty() {
                return false;
            }
            save(&now);
            println!(
                "\nCaptured {} candidate words (aligned, value <= {}).",
                now.len(),
                MAX_PLAUSIBLE
            );
            println!("Now SWITCH TO A DIFFERENT TRACK in the game, then run:");
            println!("    tas_test level-hunt diff");
            true
        }
        "diff" => {
            let prev = load();
            if prev.is_empty() {
                eprintln!("ERROR: no previous capture — run `level-hunt begin` first");
                return false;
            }
            let now = capture(pid);
            if now.is_empty() {
                return false;
            }
            // Survivors = present in both AND changed. A level index MUST change
            // when the level changes; anything constant is not it.
            let mut survivors: Vec<(String, usize, u32, u32)> = prev
                .iter()
                .filter_map(|((m, off), old)| {
                    now.get(&(m.clone(), *off))
                        .filter(|new| *new != old)
                        .map(|new| (m.clone(), *off, *old, *new))
                })
                .collect();
            survivors.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));

            println!(
                "\n{} of {} candidates CHANGED across this switch.",
                survivors.len(),
                prev.len()
            );
            for (m, off, old, new) in survivors.iter().take(60) {
                println!("  {}+{:#x}   {} -> {}", m, off, old, new);
            }
            if survivors.len() > 60 {
                println!("  ... and {} more", survivors.len() - 60);
            }

            // Keep only the survivors for the next round.
            let keep: HashMap<(String, usize), u32> = survivors
                .into_iter()
                .map(|(m, off, _, new)| ((m, off), new))
                .collect();
            let n = keep.len();
            save(&keep);
            println!(
                "\nKept {} survivors. Switch tracks AGAIN and re-run `diff` to drop\n\
                 coincidences — a real level index changes EVERY time, so the set\n\
                 should collapse fast.",
                n
            );
            true
        }
        // Print the CURRENT value of every surviving candidate.
        // Once the set is small, pairwise diffing is the wrong tool: it discards
        // addresses that did not change, but "did not change" is itself evidence
        // (an AREA index is constant across two tracks in the same area). Reading
        // the whole set after each switch builds a table instead.
        // Hunt for a STATIC POINTER to the level path string.
        //
        // The value-diff above establishes there is no small track index in the
        // module statics — which matches the original RE note that the level
        // identity is "heap-only". But a static POINTER into that heap string is
        // just as good, and better: dereferencing it yields the full path, so it
        // names Practice/Halfpipe/Ramp too, which no index encoding we control
        // would give us.
        //
        // Value-diffing cannot find this: a pointer is a large number, excluded
        // by the plausibility filter, and it changes for reasons unrelated to the
        // level. So probe directly — treat every aligned static word as an
        // address, read what it points at, and keep the ones pointing at a level
        // path.
        "ptr" => {
            let h = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };
            if h == 0 {
                eprintln!("ERROR: OpenProcess failed");
                return false;
            }
            let mut hits = 0usize;
            for m in modules(pid) {
                let mut buf = vec![0u8; m.size];
                let mut got = 0usize;
                let ok =
                    unsafe { ReadProcessMemory(h, m.base, buf.as_mut_ptr(), m.size, &mut got) };
                if ok == 0 || got == 0 {
                    continue;
                }
                println!("  scanning {} ({} bytes) for pointers to a level path", m.name, got);
                let mut i = 0usize;
                while i + 4 <= got {
                    let p = u32::from_le_bytes([buf[i], buf[i + 1], buf[i + 2], buf[i + 3]])
                        as usize;
                    // Plausible user-space address only.
                    if p >= 0x10000 && p < 0x7fff_0000 {
                        let mut s = [0u8; 160];
                        let mut rd = 0usize;
                        let ok2 = unsafe {
                            ReadProcessMemory(h, p, s.as_mut_ptr(), s.len(), &mut rd)
                        };
                        if ok2 != 0 && rd > 8 {
                            let end = s.iter().position(|&c| c == 0).unwrap_or(rd);
                            if end > 8 {
                                let txt = String::from_utf8_lossy(&s[..end]).to_lowercase();
                                if txt.contains("levels") {
                                    println!(
                                        "  {}+{:#x} -> {:#x}  {:?}",
                                        m.name,
                                        i,
                                        p,
                                        String::from_utf8_lossy(&s[..end])
                                    );
                                    hits += 1;
                                }
                            }
                        }
                    }
                    i += 4;
                }
            }
            unsafe { CloseHandle(h) };
            println!("\n{} static pointer(s) into a level path.", hits);
            hits > 0
        }
        "probe" => probe(),
        "show" => {
            let prev = load();
            if prev.is_empty() {
                eprintln!("ERROR: no candidates — run `level-hunt begin` first");
                return false;
            }
            // Unfiltered: report every candidate's CURRENT value, even if it has
            // left the plausible range. A vanishing row reads as "still a
            // candidate" when it is actually a refutation.
            let now = capture_opt(pid, false);
            let mut rows: Vec<(String, usize, u32)> = prev
                .keys()
                .filter_map(|(m, off)| {
                    now.get(&(m.clone(), *off)).map(|v| (m.clone(), *off, *v))
                })
                .collect();
            rows.sort_by(|a, b| (&a.0, a.1).cmp(&(&b.0, b.1)));
            println!("
{} candidates, current values:", rows.len());
            for (m, off, v) in &rows {
                println!("  {}+{:#x}	{}", m, off, v);
            }
            true
        }
        _ => {
            eprintln!("Usage: tas_test level-hunt <begin|diff|show>");
            false
        }
    }
}

/// `tas_test level-hunt probe` — read the three engine words the level-identity
/// design rests on, straight out of the live process.
///
/// Built to answer one question the in-process publisher cannot: WHAT DOES THE
/// ENGINE LOOK LIKE AT THE MENU? The DLL's own view is not trustworthy there —
/// `game_in_game` in shared memory is written by the Cycle hook, and the cycle
/// STOPS at a static menu, so that copy freezes at its last in-race value. This
/// reads the game's variable directly instead.
///
///   Supreme_Game.dll + 0x1D3304  -> ptr to the current level's resource path
///   Supreme_Game.dll + 0x1D5450  -> the engine root object (cave2's auto-stop
///                                   anchor; reallocated on a real teardown)
///   Supreme.exe      + 0x8895C   -> "engine is running a level"
pub fn probe() -> bool {
    const LEVEL_PATH_PTR_OFF: usize = 0x1D3304;
    const ROOT_PTR_OFF: usize = 0x1D5450;
    const IN_GAME_OFF: usize = 0x8895C;

    let Some(pid) = find_pid() else {
        eprintln!("ERROR: no Supreme process");
        return false;
    };
    let mods = modules(pid);
    let find = |n: &str| mods.iter().find(|m| m.name == n).map(|m| m.base);
    let Some(sg) = find("supreme_game.dll") else {
        eprintln!("ERROR: Supreme_Game.dll not found in PID {}", pid);
        return false;
    };
    let exe = find("supreme.exe").or_else(|| find("supreme_v1.035.exe"));

    let h = unsafe { OpenProcess(PROCESS_VM_READ | PROCESS_QUERY_INFORMATION, 0, pid) };
    if h == 0 {
        eprintln!("ERROR: OpenProcess failed for PID {}", pid);
        return false;
    }
    let rd32 = |addr: usize| -> Option<u32> {
        let mut buf = [0u8; 4];
        let mut got = 0usize;
        let ok = unsafe { ReadProcessMemory(h, addr, buf.as_mut_ptr(), 4, &mut got) };
        (ok != 0 && got == 4).then(|| u32::from_le_bytes(buf))
    };
    let rdstr = |addr: usize| -> Option<String> {
        let mut buf = [0u8; 160];
        let mut got = 0usize;
        let ok = unsafe { ReadProcessMemory(h, addr, buf.as_mut_ptr(), buf.len(), &mut got) };
        if ok == 0 || got == 0 {
            return None;
        }
        let end = buf[..got].iter().position(|&c| c == 0).unwrap_or(got);
        Some(String::from_utf8_lossy(&buf[..end]).into_owned())
    };

    println!("\n=== engine probe (PID {}) ===", pid);
    println!("  Supreme_Game.dll @ {:#010x}", sg);

    let path_ptr = rd32(sg + LEVEL_PATH_PTR_OFF);
    match path_ptr {
        Some(0) => println!("  level_path_ptr  = NULL  <- no level path"),
        Some(p) => println!(
            "  level_path_ptr  = {:#010x} -> {:?}",
            p,
            rdstr(p as usize).unwrap_or_else(|| "<unreadable>".into())
        ),
        None => println!("  level_path_ptr  = <read failed>"),
    }
    match rd32(sg + ROOT_PTR_OFF) {
        Some(0) => println!("  root_ptr        = NULL  <- level torn down"),
        Some(r) => println!("  root_ptr        = {:#010x}", r),
        None => println!("  root_ptr        = <read failed>"),
    }
    match exe {
        Some(e) => {
            println!("  Supreme.exe      @ {:#010x}", e);
            match rd32(e + IN_GAME_OFF) {
                Some(v) => println!("  game_in_game    = {} (LIVE, not the DLL's copy)", v),
                None => println!("  game_in_game    = <read failed>"),
            }
        }
        None => println!("  Supreme.exe      = <not found>"),
    }
    unsafe { CloseHandle(h) };
    true
}

/// Is TAS_Helper.dll actually loaded in the game process?
///
/// Ground truth for "is TAS on", and NOT interchangeable with "can I open the
/// shared memory". A named section outlives the process that created it for as
/// long as any handle stays open — so with tas_ui still running, the mapping
/// from a DEAD game is still openable, and a shared-memory probe reports TAS
/// present against a game that has never seen the DLL. That false positive
/// would silently invalidate any with-vs-without comparison, which is the one
/// thing `video-rate` exists to do.
///
/// `None` = no game process to ask.
pub fn tas_dll_loaded() -> Option<bool> {
    let pid = find_pid()?;
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid);
        if snap == 0 || snap == usize::MAX {
            return None;
        }
        let mut me: ModuleEntry32 = std::mem::zeroed();
        me.dw_size = std::mem::size_of::<ModuleEntry32>() as Dword;
        let mut ok = Module32First(snap, &mut me);
        let mut found = false;
        while ok != 0 {
            let end = me.sz_module.iter().position(|&c| c == 0).unwrap_or(0);
            let name = String::from_utf8_lossy(&me.sz_module[..end]).to_lowercase();
            if name == "tas_helper.dll" {
                found = true;
                break;
            }
            ok = Module32Next(snap, &mut me);
        }
        CloseHandle(snap);
        Some(found)
    }
}
