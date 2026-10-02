//! Read-only view of the game's own memory, for tests that must check what the
//! game sees rather than what the DLL reports about itself.
//!
//! Two per-key byte arrays, both indexed by game key code:
//! - the key buffer the DLL writes: `[[[Supreme_Game.dll+0x1D5450]+0x530]+0x30]`
//!   (`TAS_Helper/src/game_addresses.hpp`). Supreme_Game.dll is always
//!   relocated, so its base is looked up per process.
//! - the held state steering reads: the EXE's `TC_Kbd_Impl` (the keyboard
//!   observer BB3B10 notifies, vtable 0x46D8E8) at `[[[EXE+0x889C4]+0x14]+0x1AC]`,
//!   array pointer at +0x38. Its `Update` (0x40F7A0) applies queued BB3B10
//!   events once per tick; `Reset` zeroes it at every race (re)start. Queued
//!   event count at +0x7C.

use crate::win32;

const ROOT_PTR_RVA: usize = 0x1D5450;
const APP_PTR_RVA: usize = 0x889C4;
const APP_GAME_OFFSET: usize = 0x14;
const GAME_KEYBOARD_OBSERVER_OFFSET: usize = 0x1AC;
const OBSERVER_HELD_PTR_OFFSET: usize = 0x38;
const KEYBOARD_OBJ_OFFSET: usize = 0x530;
const DI_BUFFER_PTR_OFFSET: usize = 0x30;

/// Game key codes of the six TAS keys, with the input bit each belongs to.
/// Gameplay ORs three codes for jump (CTRL, LCTRL, RCTRL) and for shift.
pub const TAS_KEYS: [(&str, u32, u8); 10] = [
    ("LEFT", 0x3A, 0x01),
    ("RIGHT", 0x3B, 0x02),
    ("UP", 0x38, 0x04),
    ("DOWN", 0x39, 0x08),
    ("JUMP", 0x27, 0x10),
    ("JUMP2", 0x28, 0x10),
    ("JUMP3", 0x29, 0x10),
    ("SHIFT", 0x24, 0x20),
    ("SHIFT2", 0x25, 0x20),
    ("SHIFT3", 0x26, 0x20),
];

pub struct GameMemory {
    process: win32::Handle,
    exe_base: usize,
    sg_base: usize,
}

impl GameMemory {
    /// Attach to the running game (found by its window). `None` when the game
    /// is not running or Supreme_Game.dll is not loaded yet.
    pub fn attach() -> Option<GameMemory> {
        let hwnd = win32::find_game_window()?;
        let mut pid = 0u32;
        unsafe { win32::GetWindowThreadProcessId(hwnd, &mut pid) };
        if pid == 0 {
            return None;
        }
        let process = unsafe {
            win32::OpenProcess(
                win32::PROCESS_VM_READ
                    | win32::PROCESS_QUERY_INFORMATION
                    | win32::PROCESS_SUSPEND_RESUME,
                0,
                pid,
            )
        };
        if process == 0 {
            return None;
        }
        let mut memory = GameMemory {
            process,
            exe_base: 0,
            sg_base: 0,
        };
        memory.sg_base = memory.module_base("supreme_game.dll")?;
        memory.exe_base = memory.main_module_base()?;
        Some(memory)
    }

    fn modules(&self) -> Option<Vec<win32::Handle>> {
        let mut modules = vec![0 as win32::Handle; 512];
        let mut needed = 0u32;
        let ok = unsafe {
            win32::K32EnumProcessModulesEx(
                self.process,
                modules.as_mut_ptr(),
                (modules.len() * std::mem::size_of::<win32::Handle>()) as u32,
                &mut needed,
                win32::LIST_MODULES_32BIT,
            )
        };
        if ok == 0 {
            return None;
        }
        let count = (needed as usize / std::mem::size_of::<win32::Handle>()).min(modules.len());
        modules.truncate(count);
        Some(modules)
    }

    /// The executable is the first module listed.
    fn main_module_base(&self) -> Option<usize> {
        self.modules()?.first().map(|&m| m as usize)
    }

    fn module_base(&self, lower_name: &str) -> Option<usize> {
        self.modules()?.into_iter().find_map(|module| {
            let mut name = [0u8; 260];
            let len = unsafe {
                win32::K32GetModuleBaseNameA(
                    self.process,
                    module,
                    name.as_mut_ptr(),
                    name.len() as u32,
                )
            } as usize;
            let name = String::from_utf8_lossy(&name[..len]).to_ascii_lowercase();
            (name == lower_name).then_some(module as usize)
        })
    }

    fn read(&self, address: usize, out: &mut [u8]) -> bool {
        let mut read = 0usize;
        let ok = unsafe {
            win32::ReadProcessMemory(
                self.process,
                address,
                out.as_mut_ptr(),
                out.len(),
                &mut read,
            )
        };
        ok != 0 && read == out.len()
    }

    fn read_u32(&self, address: usize) -> Result<u32, String> {
        let mut bytes = [0u8; 4];
        if self.read(address, &mut bytes) {
            Ok(u32::from_le_bytes(bytes))
        } else {
            Err(format!(
                "ReadProcessMemory({address:#x}) failed, error {}",
                unsafe { win32::GetLastError() }
            ))
        }
    }

    /// Follow a pointer chain resolved fresh (an F5 restart can rebuild the
    /// objects on it): from `base`, read the pointer at each offset in turn.
    /// The error names the link that failed.
    fn chain(&self, base: usize, links: &[(usize, &str)]) -> Result<usize, String> {
        links.iter().try_fold(base, |pointer, &(offset, what)| {
            let address = pointer + offset;
            match self.read_u32(address) {
                Ok(0) => Err(format!("{what} at {address:#x} is null")),
                Ok(p) => Ok(p as usize),
                Err(e) => Err(format!("{what}: {e}")),
            }
        })
    }

    /// Address of the game's 256-byte key buffer.
    fn key_buffer(&self) -> Result<usize, String> {
        self.chain(
            self.sg_base,
            &[
                (ROOT_PTR_RVA, "root pointer"),
                (KEYBOARD_OBJ_OFFSET, "keyboard object"),
                (DI_BUFFER_PTR_OFFSET, "key buffer pointer"),
            ],
        )
        .map_err(|e| format!("{e} (Supreme_Game.dll at {:#x})", self.sg_base))
    }

    /// The keyboard observer's held-state array.
    fn observer_held(&self) -> Result<usize, String> {
        self.chain(
            self.exe_base,
            &[
                (APP_PTR_RVA, "app pointer"),
                (APP_GAME_OFFSET, "game object"),
                (GAME_KEYBOARD_OBSERVER_OFFSET, "keyboard observer"),
                (OBSERVER_HELD_PTR_OFFSET, "observer held array"),
            ],
        )
    }

    fn mask_of(&self, array: usize, what: &str) -> Result<u8, String> {
        let mut bytes = [0u8; 256];
        if !self.read(array, &mut bytes) {
            return Err(format!("{what} at {array:#x} unreadable"));
        }
        Ok(TAS_KEYS
            .iter()
            .filter(|(_, code, _)| bytes[*code as usize] != 0)
            .fold(0u8, |mask, (_, _, bit)| mask | bit))
    }

    /// The TAS input mask the game's key buffer currently holds (bits as in
    /// `input_log`): a bit is set when any of its key codes reads pressed.
    pub fn held_tas_mask(&self) -> Result<u8, String> {
        self.mask_of(self.key_buffer()?, "key buffer")
    }

    /// Ticks since the rider's last reset: the Player's countdown float
    /// (`[[player+0x154]+0]`, +0.01 per Player::Cycle from 0 at reset; the rider
    /// is released on the tick it passes 3.0).
    pub fn countdown_ticks(&self, player: u32) -> Result<u32, String> {
        let countdown = self.read_u32(player as usize + 0x154)?;
        if countdown == 0 {
            return Err("the player has no countdown object".into());
        }
        let t = f32::from_bits(self.read_u32(countdown as usize)?);
        Ok((t * 100.0).round() as u32)
    }

    /// The TAS input mask the keyboard observer holds: what steering reads.
    pub fn observer_tas_mask(&self) -> Result<u8, String> {
        self.mask_of(self.observer_held()?, "observer held array")
    }
}

/// Freezes every game thread until dropped: a hitch on demand. Dropping it
/// resumes the game, so a failing test cannot leave the game frozen.
pub struct Frozen<'a>(&'a GameMemory);

impl GameMemory {
    pub fn freeze(&self) -> Result<Frozen<'_>, String> {
        let status = unsafe { win32::NtSuspendProcess(self.process) };
        if status < 0 {
            return Err(format!("NtSuspendProcess failed ({status:#x})"));
        }
        Ok(Frozen(self))
    }
}

impl Drop for Frozen<'_> {
    fn drop(&mut self) {
        unsafe { win32::NtResumeProcess(self.0.process) };
    }
}

impl Drop for GameMemory {
    fn drop(&mut self) {
        unsafe { win32::CloseHandle(self.process) };
    }
}
