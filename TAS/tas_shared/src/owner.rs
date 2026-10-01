//! Controller ownership (DESIGN.md "Controller ownership"). A controller
//! registers before it drives a cycle; the DLL keeps a handle to its process
//! and stops the TAS if it exits without releasing.

use std::sync::atomic::Ordering;

use crate::state::*;

/// What the DLL answered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerAnswer {
    Owned,
    Released,
    /// Another live process owns the TAS.
    Busy {
        owner_pid: u32,
    },
    /// The DLL could not open or verify this process.
    Refused(u32),
    /// A request from another process replaced this one before the DLL
    /// read it.
    Superseded,
}

/// This process's identity: its pid and creation time (FILETIME), which
/// the DLL checks so a reused pid cannot pass as the owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProcessIdentity {
    pub pid: u32,
    pub created: u64,
}

/// Write a request and bump the sequence. Returns the sequence to wait for.
pub fn owner_request_submit(state: &mut TasSharedState, kind: u32, who: ProcessIdentity) -> u32 {
    // SAFETY: shared mapping; the DLL reads these after it sees the new sequence.
    unsafe {
        std::ptr::write_volatile(&mut state.owner_request_kind, kind);
        std::ptr::write_volatile(&mut state.owner_request_pid, who.pid);
        std::ptr::write_volatile(&mut state.owner_request_created_lo, who.created as u32);
        std::ptr::write_volatile(
            &mut state.owner_request_created_hi,
            (who.created >> 32) as u32,
        );
    }
    state
        .owner_request_seq
        .fetch_add(1, Ordering::AcqRel)
        .wrapping_add(1)
}

/// The DLL's answer to request `seq`, once it has one.
pub fn owner_request_answer(state: &TasSharedState, seq: u32, pid: u32) -> Option<OwnerAnswer> {
    let ack = state.owner_ack_seq.load(Ordering::Acquire);
    if ack == seq {
        // SAFETY: shared mapping; written by the DLL before it stored the ack.
        let (result, owner_pid) = unsafe {
            (
                std::ptr::read_volatile(&state.owner_result),
                std::ptr::read_volatile(&state.owner_pid),
            )
        };
        return Some(decode(result, owner_pid));
    }
    // Acked past ours: the DLL answered a later request instead. Ours still
    // won only if the owner it published is this process.
    if ack.wrapping_sub(seq) < u32::MAX / 2 {
        let owner_pid = unsafe { std::ptr::read_volatile(&state.owner_pid) };
        return Some(if owner_pid == pid {
            OwnerAnswer::Owned
        } else {
            OwnerAnswer::Superseded
        });
    }
    None
}

fn decode(result: u32, owner_pid: u32) -> OwnerAnswer {
    match result {
        TAS_OWNER_RESULT_OWNED => OwnerAnswer::Owned,
        TAS_OWNER_RESULT_RELEASED => OwnerAnswer::Released,
        TAS_OWNER_RESULT_BUSY => OwnerAnswer::Busy { owner_pid },
        other => OwnerAnswer::Refused(other),
    }
}

impl std::fmt::Display for OwnerAnswer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerAnswer::Owned => write!(f, "owned"),
            OwnerAnswer::Released => write!(f, "released"),
            OwnerAnswer::Busy { owner_pid } => {
                write!(f, "another controller (pid {owner_pid}) is driving the TAS")
            }
            OwnerAnswer::Refused(TAS_OWNER_RESULT_NO_PROCESS) => {
                write!(f, "the DLL could not open this controller's process")
            }
            OwnerAnswer::Refused(TAS_OWNER_RESULT_WRONG_PROCESS) => {
                write!(
                    f,
                    "the DLL found a different process under this controller's pid"
                )
            }
            OwnerAnswer::Refused(TAS_OWNER_RESULT_NOT_OWNER) => {
                write!(f, "this controller did not own the TAS")
            }
            OwnerAnswer::Refused(code) => write!(f, "the DLL refused ownership (result {code})"),
            OwnerAnswer::Superseded => {
                write!(f, "another controller registered at the same moment")
            }
        }
    }
}

#[cfg(windows)]
pub fn current_process_identity() -> ProcessIdentity {
    #[repr(C)]
    #[derive(Default)]
    struct FileTime {
        low: u32,
        high: u32,
    }
    extern "system" {
        fn GetCurrentProcess() -> *mut std::ffi::c_void;
        fn GetProcessTimes(
            process: *mut std::ffi::c_void,
            creation: *mut FileTime,
            exit: *mut FileTime,
            kernel: *mut FileTime,
            user: *mut FileTime,
        ) -> i32;
    }
    let (mut c, mut e, mut k, mut u) = Default::default();
    // SAFETY: the pseudo-handle needs no closing; the out-params are locals.
    unsafe {
        GetProcessTimes(GetCurrentProcess(), &mut c, &mut e, &mut k, &mut u);
    }
    let c: FileTime = c;
    ProcessIdentity {
        pid: std::process::id(),
        created: ((c.high as u64) << 32) | c.low as u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zeroed() -> Box<TasSharedState> {
        zeroed_boxed()
    }

    const ME: ProcessIdentity = ProcessIdentity {
        pid: 42,
        created: 0x0123_4567_89AB_CDEF,
    };

    #[test]
    fn submit_writes_identity_then_sequence() {
        let mut s = zeroed();
        let seq = owner_request_submit(&mut s, TAS_OWNER_ACQUIRE, ME);
        assert_eq!(seq, 1);
        assert_eq!(s.owner_request_kind, TAS_OWNER_ACQUIRE);
        assert_eq!(s.owner_request_pid, 42);
        assert_eq!(s.owner_request_created_lo, 0x89AB_CDEF);
        assert_eq!(s.owner_request_created_hi, 0x0123_4567);
        assert_eq!(owner_request_answer(&s, seq, ME.pid), None);
    }

    #[test]
    fn answer_decodes_the_dll_result() {
        let mut s = zeroed();
        let seq = owner_request_submit(&mut s, TAS_OWNER_ACQUIRE, ME);
        s.owner_result = TAS_OWNER_RESULT_BUSY;
        s.owner_pid = 7;
        s.owner_ack_seq.store(seq, Ordering::Release);
        assert_eq!(
            owner_request_answer(&s, seq, ME.pid),
            Some(OwnerAnswer::Busy { owner_pid: 7 })
        );
    }

    #[test]
    fn a_later_request_answered_first_supersedes_ours() {
        let mut s = zeroed();
        let mine = owner_request_submit(&mut s, TAS_OWNER_ACQUIRE, ME);
        let theirs = owner_request_submit(
            &mut s,
            TAS_OWNER_ACQUIRE,
            ProcessIdentity { pid: 9, created: 1 },
        );
        s.owner_result = TAS_OWNER_RESULT_OWNED;
        s.owner_pid = 9;
        s.owner_ack_seq.store(theirs, Ordering::Release);
        assert_eq!(
            owner_request_answer(&s, mine, ME.pid),
            Some(OwnerAnswer::Superseded)
        );
    }

    #[test]
    fn an_older_ack_is_not_an_answer() {
        let mut s = zeroed();
        let first = owner_request_submit(&mut s, TAS_OWNER_ACQUIRE, ME);
        s.owner_ack_seq.store(first, Ordering::Release);
        let second = owner_request_submit(&mut s, TAS_OWNER_RELEASE, ME);
        assert_eq!(owner_request_answer(&s, second, ME.pid), None);
    }

    #[cfg(windows)]
    #[test]
    fn current_identity_is_this_process() {
        let me = current_process_identity();
        assert_eq!(me.pid, std::process::id());
        assert_ne!(me.created, 0);
        assert_eq!(current_process_identity(), me);
    }
}
