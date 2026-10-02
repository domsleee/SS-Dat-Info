//! The DLL's crash record (DESIGN.md "Crashes"), read from the mapping after
//! the game process is gone.

use std::sync::atomic::Ordering;

use crate::state::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashRecord {
    pub code: u32,
    pub address: u32,
    /// Empty when the address was in no known module.
    pub module: String,
    pub module_offset: u32,
    pub game_call: u32,
}

/// What the game process left behind when it went away.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameExit {
    Crashed(CrashRecord),
    /// Closed through ExitProcess.
    Clean,
    /// Neither: killed, or a fault the DLL never saw.
    Unexplained,
}

/// The exit of game process `pid`, from a mapping that outlived it.
pub fn game_exit(state: &TasSharedState, pid: u32) -> GameExit {
    if let Some(record) = crash_record(state, pid) {
        return GameExit::Crashed(record);
    }
    // SAFETY: shared mapping; the DLL writes it once, at process exit.
    if unsafe { std::ptr::read_volatile(&state.game_exit_clean) } != 0 {
        GameExit::Clean
    } else {
        GameExit::Unexplained
    }
}

/// The last fault the DLL recorded for game process `pid`, if any.
pub fn crash_record(state: &TasSharedState, pid: u32) -> Option<CrashRecord> {
    if state.crash_seq.load(Ordering::Acquire) == 0 {
        return None;
    }
    // SAFETY: shared mapping; the DLL wrote these before it bumped crash_seq.
    let read = |v: &u32| unsafe { std::ptr::read_volatile(v) };
    if read(&state.crash_pid) != pid {
        return None;
    }
    let module = unsafe { std::ptr::read_volatile(&state.crash_module) };
    let len = module.iter().position(|&b| b == 0).unwrap_or(module.len());
    Some(CrashRecord {
        code: read(&state.crash_code),
        address: read(&state.crash_address),
        module: String::from_utf8_lossy(&module[..len]).into_owned(),
        module_offset: read(&state.crash_module_offset),
        game_call: read(&state.crash_game_call),
    })
}

pub fn game_call_name(call: u32) -> &'static str {
    match call {
        TAS_GAME_CALL_NONE => "no TAS call",
        TAS_GAME_CALL_MENU_TRIGGER => "UI_Menu::Trigger",
        TAS_GAME_CALL_MENU_MOVE => "UI_Menu::Up/Down/Left/Right",
        TAS_GAME_CALL_MENU_FOCUS => "UI_Component::Request_Focus / Want_Focus",
        TAS_GAME_CALL_MENU_ACTIVE => "UI_Menu::Get_Active_Component / Get_Modal",
        TAS_GAME_CALL_TIME_CURRENT => "Kernel::Time::Current",
        TAS_GAME_CALL_TEST_FAULT => "the tas_test fault",
        TAS_GAME_CALL_OBSERVER_FLUSH => "TC_Kbd_Impl::Flush",
        _ => "an unknown TAS call",
    }
}

fn exception_name(code: u32) -> Option<&'static str> {
    Some(match code {
        0xC000_0005 => "access violation",
        0xC000_001D => "illegal instruction",
        0xC000_0094 => "integer divide by zero",
        0xC000_00FD => "stack overflow",
        0xC000_0374 => "heap corruption",
        0xC000_0409 => "stack buffer overrun",
        _ => return None,
    })
}

impl std::fmt::Display for CrashRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match exception_name(self.code) {
            Some(name) => write!(f, "{name} (0x{:08X})", self.code)?,
            None => write!(f, "exception 0x{:08X}", self.code)?,
        }
        if self.module.is_empty() {
            write!(f, " at 0x{:08X}", self.address)?;
        } else {
            write!(f, " at {}+0x{:X}", self.module, self.module_offset)?;
        }
        if self.game_call == TAS_GAME_CALL_NONE {
            write!(f, ", outside any TAS call into the game")
        } else {
            write!(f, " during {}", game_call_name(self.game_call))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crashed(state: &mut TasSharedState) {
        state.crash_pid = 1234;
        state.crash_code = 0xC000_0005;
        state.crash_address = 0x1001_2345;
        state.crash_module[..10].copy_from_slice(b"Kernel.dll");
        state.crash_module_offset = 0x2345;
        state.crash_game_call = TAS_GAME_CALL_TIME_CURRENT;
        state.crash_seq.store(1, Ordering::Release);
    }

    #[test]
    fn no_record_and_no_clean_exit_is_unexplained() {
        let state = zeroed_boxed();
        assert_eq!(game_exit(&state, 1234), GameExit::Unexplained);
    }

    #[test]
    fn clean_exit_is_clean() {
        let mut state = zeroed_boxed();
        state.game_exit_clean = 1;
        assert_eq!(game_exit(&state, 1234), GameExit::Clean);
    }

    #[test]
    fn a_record_names_the_fault_the_module_and_the_call() {
        let mut state = zeroed_boxed();
        crashed(&mut state);
        let GameExit::Crashed(record) = game_exit(&state, 1234) else {
            panic!("expected a crash");
        };
        assert_eq!(
            record.to_string(),
            "access violation (0xC0000005) at Kernel.dll+0x2345 during Kernel::Time::Current"
        );
    }

    #[test]
    fn a_crash_beats_a_clean_exit() {
        // The game's kernel-error dialog exits through ExitProcess.
        let mut state = zeroed_boxed();
        crashed(&mut state);
        state.game_exit_clean = 1;
        assert!(matches!(game_exit(&state, 1234), GameExit::Crashed(_)));
    }

    #[test]
    fn another_process_record_is_not_this_crash() {
        let mut state = zeroed_boxed();
        crashed(&mut state);
        assert_eq!(game_exit(&state, 999), GameExit::Unexplained);
    }

    #[test]
    fn a_fault_outside_any_module_or_call_says_so() {
        let record = CrashRecord {
            code: 0xC000_0409,
            address: 0x0BAD_F00D,
            module: String::new(),
            module_offset: 0,
            game_call: TAS_GAME_CALL_NONE,
        };
        assert_eq!(
            record.to_string(),
            "stack buffer overrun (0xC0000409) at 0x0BADF00D, outside any TAS call into the game"
        );
    }
}
