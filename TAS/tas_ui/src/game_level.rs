//! Reads the current track ("Forest Easy") out of the running game's memory.
//!
//! There is no stable fixed-address anchor for the level (the obvious pointers
//! are transient buffers reused for other data). What *is* reliable: while a
//! level is loaded, the heap is full of resource path strings of the form
//! `".../Levels/<Area>/Tracks/<Difficulty>/..."` with the correct area AND
//! difficulty. So we scan the game's committed memory, tally every
//! `"<area>/tracks/<diff>"` occurrence, and take the majority — which shrugs off
//! outliers like the shared `shadow.qua` that always lives under `easy/`.
//!
//! Done from tas_ui via ReadProcessMemory (the game is a separate process), the
//! same raw-FFI approach the rest of main.rs uses — no extra deps. Throttled
//! hard: we only scan when in-game and we don't yet know the track, and we cache
//! the answer until the game returns to the menu.

#![cfg(windows)]

use std::ffi::c_void;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

type Handle = *mut c_void;

const PROCESS_QUERY_INFORMATION: u32 = 0x0400;
const PROCESS_VM_READ: u32 = 0x0010;
const MEM_COMMIT: u32 = 0x1000;
const MEM_PRIVATE: u32 = 0x2_0000;
const PAGE_GUARD: u32 = 0x100;
const PAGE_NOACCESS: u32 = 0x01;

// Supreme.exe is 32-bit (non-ASLR); the heap we care about sits under 2 GB.
const SCAN_MAX_ADDR: usize = 0x7FFF_0000;
// Stop once we've gathered enough evidence — matches are dense once a level is up.
const ENOUGH_MATCHES: u32 = 40;
const RESCAN_INTERVAL: Duration = Duration::from_millis(750);

#[repr(C)]
struct MemoryBasicInformation {
    base_address: usize,
    allocation_base: usize,
    allocation_protect: u32,
    region_size: usize,
    state: u32,
    protect: u32,
    type_: u32,
}

extern "system" {
    fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
    fn CloseHandle(h: Handle) -> i32;
    fn ReadProcessMemory(
        h: Handle,
        base: usize,
        buf: *mut u8,
        size: usize,
        read: *mut usize,
    ) -> i32;
    fn VirtualQueryEx(h: Handle, addr: usize, info: *mut MemoryBasicInformation, len: usize)
        -> usize;
}

const AREAS: [&str; 3] = ["forest", "alpine", "village"];
const DIFFS: [&str; 3] = ["easy", "medium", "hard"];

/// Count `"<area>/tracks/<diff>"` occurrences in `hay` (already lowercased),
/// accumulating into `tally` indexed `[area*3 + diff]`. Returns matches added.
fn tally_paths(hay: &[u8], tally: &mut [u32; 9]) -> u32 {
    let needle = b"/tracks/";
    let mut added = 0;
    let mut i = 0;
    while i + needle.len() < hay.len() {
        if &hay[i..i + needle.len()] == needle {
            // area = the path segment ending just before this "/tracks/"
            let area_end = i;
            let area_start = hay[..area_end]
                .iter()
                .rposition(|&b| b == b'/' || b == b'\\')
                .map(|p| p + 1)
                .unwrap_or(0);
            let area = &hay[area_start..area_end];
            let after = i + needle.len();
            let diff_end = hay[after..]
                .iter()
                .position(|&b| b == b'/' || b == b'\\')
                .map(|p| after + p)
                .unwrap_or(hay.len());
            let diff = &hay[after..diff_end];
            if let (Some(ai), Some(di)) = (
                AREAS.iter().position(|a| a.as_bytes() == area),
                DIFFS.iter().position(|d| d.as_bytes() == diff),
            ) {
                tally[ai * 3 + di] += 1;
                added += 1;
            }
            i = after;
        } else {
            i += 1;
        }
    }
    added
}

fn tally_to_label(tally: &[u32; 9]) -> Option<String> {
    let (best, &count) = tally.iter().enumerate().max_by_key(|(_, &c)| c)?;
    if count == 0 {
        return None;
    }
    Some(format!(
        "{} {}",
        title_case(AREAS[best / 3]),
        title_case(DIFFS[best % 3])
    ))
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_ascii_uppercase().to_string() + c.as_str(),
        None => String::new(),
    }
}

/// Scan the process heap and return the majority track label, if found.
fn scan_process(pid: u32) -> Option<String> {
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, pid);
        if h.is_null() {
            return None;
        }
        let mut tally = [0u32; 9];
        let mut total = 0u32;
        let mut addr = 0usize;
        let mut buf: Vec<u8> = vec![0; 1 << 20]; // 1 MiB working window
        while addr < SCAN_MAX_ADDR {
            let mut mbi: MemoryBasicInformation = std::mem::zeroed();
            let got = VirtualQueryEx(
                h,
                addr,
                &mut mbi,
                std::mem::size_of::<MemoryBasicInformation>(),
            );
            if got == 0 {
                break;
            }
            let next = mbi.base_address.wrapping_add(mbi.region_size);
            // Only private (heap) committed pages — the level path strings live
            // there. Skips the module images + mapped files (~tens of MB), so we
            // touch far less memory before the early-exit.
            let readable = mbi.state == MEM_COMMIT
                && mbi.type_ == MEM_PRIVATE
                && (mbi.protect & PAGE_GUARD) == 0
                && (mbi.protect & PAGE_NOACCESS) == 0;
            if readable && mbi.region_size > 0 {
                let mut off = 0;
                while off < mbi.region_size {
                    let chunk = (mbi.region_size - off).min(buf.len());
                    let mut read = 0usize;
                    if ReadProcessMemory(h, mbi.base_address + off, buf.as_mut_ptr(), chunk, &mut read)
                        != 0
                        && read > 0
                    {
                        for b in &mut buf[..read] {
                            b.make_ascii_lowercase();
                        }
                        total += tally_paths(&buf[..read], &mut tally);
                    }
                    off += chunk;
                }
            }
            if total >= ENOUGH_MATCHES || next <= addr {
                break;
            }
            addr = next;
        }
        CloseHandle(h);
        tally_to_label(&tally)
    }
}

/// Stateful track reader. The heap scan (~200 ms) runs on a **background
/// thread** so it never touches the UI frame time — poll() just kicks off a
/// worker when needed and picks up the result when it's ready. Caches the label
/// and only rescans when in-game without a known track (throttled); cleared back
/// in the menu.
#[derive(Default)]
pub struct LevelReader {
    label: Option<String>,
    last_scan: Option<Instant>,
    pending: Option<Receiver<Option<String>>>,
}

impl LevelReader {
    /// Call each frame with the live `game_in_game` flag. Returns the current
    /// track label (`Some("Forest Easy")`) or `None` (menu / not yet detected).
    /// Never blocks: the scan happens on a worker thread.
    pub fn poll(&mut self, in_game: bool) -> Option<&str> {
        if !in_game {
            self.label = None;
            self.last_scan = None;
            self.pending = None;
            return None;
        }
        // Collect a finished background scan, if any.
        if let Some(rx) = &self.pending {
            match rx.try_recv() {
                Ok(result) => {
                    self.label = result;
                    self.pending = None;
                }
                Err(mpsc::TryRecvError::Disconnected) => self.pending = None,
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        // Kick off a new scan if we still don't know the track and none is in
        // flight (throttled).
        if self.label.is_none() && self.pending.is_none() {
            let due = self
                .last_scan
                .map(|t| t.elapsed() >= RESCAN_INTERVAL)
                .unwrap_or(true);
            if due {
                self.last_scan = Some(Instant::now());
                let (tx, rx) = mpsc::channel();
                thread::spawn(move || {
                    let result = crate::find_supreme_pid().and_then(scan_process);
                    let _ = tx.send(result);
                });
                self.pending = Some(rx);
            }
        }
        self.label.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tally_picks_majority_difficulty_over_shadow_outlier() {
        // Village Hard: many hard paths + one stray shared shadow under easy/.
        let mut hay = String::new();
        for _ in 0..10 {
            hay.push_str("data/levels/village/tracks/hard/map.tga\0");
        }
        hay.push_str("data/levels/village/tracks/easy/cloudy/shadow.qua\0");
        let mut tally = [0u32; 9];
        tally_paths(hay.as_bytes(), &mut tally);
        assert_eq!(tally_to_label(&tally).as_deref(), Some("Village Hard"));
    }

    #[test]
    fn tally_parses_each_area_and_difficulty() {
        for (ai, a) in AREAS.iter().enumerate() {
            for (di, d) in DIFFS.iter().enumerate() {
                let s = format!("x/levels/{}/tracks/{}/y", a, d);
                let mut tally = [0u32; 9];
                tally_paths(s.as_bytes(), &mut tally);
                assert_eq!(tally[ai * 3 + di], 1, "{a}/{d}");
            }
        }
    }

    #[test]
    fn tally_ignores_unrelated_paths() {
        let mut tally = [0u32; 9];
        tally_paths(b"data/objects/hud/tracks/foo/bar", &mut tally);
        assert_eq!(tally_to_label(&tally), None);
    }

    #[test]
    fn empty_tally_is_none() {
        assert_eq!(tally_to_label(&[0u32; 9]), None);
    }
}
