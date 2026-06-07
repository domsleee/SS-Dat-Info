#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include <tlhelp32.h>

// =============================================================================
// PROTOTYPE: full-process writable-memory snapshot / restore for instant CONT.
//
// Goal: replace the ~1.84s F5-restart+reload+replay CONT cycle with a memory
// snapshot taken once at the splice frame and restored on subsequent CONTs
// (~tens of ms). This is the libTAS / TMInterface design (see notes); the sim
// is already proven bit-deterministic from memory (zero-drift replays), so
// restoring the writable state rewinds the sim exactly.
//
// SCOPE (prototype): validate the mechanism + timing + sim reproduction. Region
// policy is deliberately CONSERVATIVE to protect the DLL and the OS:
//   INCLUDE: MEM_COMMIT, writable, NON-executable, and either MEM_PRIVATE (game
//            heap) or MEM_IMAGE within Supreme.exe (its .data/.bss).
//   EXCLUDE: all executable pages (protects code + safetyhook trampolines),
//            all thread stacks (by AllocationBase — restoring a stack without
//            its thread context would crash), MEM_MAPPED (the TAS shared memory
//            + file maps), TAS_Helper.dll's own image (auto: MEM_IMAGE non-
//            Supreme), and the snapshot buffer itself (VirtualAlloc'd).
//
// THREAD SAFETY: runs at the cave2 frame boundary (main/sim thread, FPU saved).
// We suspend the other game threads (render/audio) for the duration of the
// memcpy so they can't tear the heap mid-copy. Deadlock-safe: the buffer is
// pre-allocated via VirtualAlloc (no heap lock), and we only memcpy (lock-free)
// while threads are suspended — never malloc/Log under suspension.
//
// KNOWN PROTOTYPE LIMITATION: between snapshot and restore the render/audio
// threads run freely and may churn the heap; on restore their post-snapshot
// allocations vanish, so a pointer they still hold can dangle (libTAS solves
// this by also saving/restoring thread contexts + stacks). For the prototype we
// keep the snapshot->restore window short and suspend during the copy; a crash
// here is informative, not a regression (CONT still works the old way).
// =============================================================================

struct SnapRegion {
    uintptr_t base;
    uint32_t  size;
    uint32_t  offset; // into g_snapBuffer
};

static constexpr uint32_t SNAP_MAX_REGIONS = 8192;
static SnapRegion g_snapRegions[SNAP_MAX_REGIONS];
static uint32_t   g_snapRegionCount = 0;
static uint8_t*   g_snapBuffer = nullptr;     // VirtualAlloc'd; excluded from snapshot
static size_t     g_snapCapacity = 0;
static size_t     g_snapUsed = 0;
static bool       g_snapValid = false;
// Last restore region accounting (long-window diagnostic).
static uint32_t   g_snapLastRestored = 0;
static uint32_t   g_snapLastSkipped = 0;
static uint32_t   g_snapLastFaulted = 0;
// Armed by CMD_SNAPSHOT_AT_SPAWN: capture at the next PLAY frame-0 (the exact
// spawn the replay starts from) so arm timing isn't disturbed by the ~200ms
// capture. Cleared once it fires.
static bool       g_snapAtSpawn = false;

// ---- exclusion helpers ------------------------------------------------------

// Up to this many AllocationBases to skip (thread stacks + shared mem).
static constexpr int SNAP_MAX_SKIP_BASES = 64;

struct SnapExclusion {
    uintptr_t skipBases[SNAP_MAX_SKIP_BASES];
    int       skipCount;
    uintptr_t supremeBase, supremeEnd; // Supreme.exe image range (only MEM_IMAGE we keep)
};

static void snap_add_skip(SnapExclusion& ex, uintptr_t base) {
    if (base == 0) return;
    for (int i = 0; i < ex.skipCount; i++) if (ex.skipBases[i] == base) return;
    if (ex.skipCount < SNAP_MAX_SKIP_BASES) ex.skipBases[ex.skipCount++] = base;
}

static bool snap_is_skipped(const SnapExclusion& ex, uintptr_t allocBase) {
    for (int i = 0; i < ex.skipCount; i++) if (ex.skipBases[i] == allocBase) return true;
    return false;
}

// Supreme.exe image bounds (the only MEM_IMAGE we snapshot — its .data/.bss).
static void snap_supreme_bounds(SnapExclusion& ex) {
    ex.supremeBase = 0; ex.supremeEnd = 0;
    HMODULE h = GetModuleHandleA(nullptr); // main exe
    if (!h) return;
    auto* dos = (IMAGE_DOS_HEADER*)h;
    if (dos->e_magic != IMAGE_DOS_SIGNATURE) return;
    auto* nt = (IMAGE_NT_HEADERS*)((uint8_t*)h + dos->e_lfanew);
    if (nt->Signature != IMAGE_NT_SIGNATURE) return;
    ex.supremeBase = (uintptr_t)h;
    ex.supremeEnd  = (uintptr_t)h + nt->OptionalHeader.SizeOfImage;
}

// Decide whether a region is part of the snapshot set.
static bool snap_region_included(const MEMORY_BASIC_INFORMATION& mbi,
                                 const SnapExclusion& ex) {
    if (mbi.State != MEM_COMMIT) return false;
    if (mbi.Protect & PAGE_GUARD) return false;
    DWORD p = mbi.Protect & 0xFF;
    // Writable AND non-executable only (protect code + trampolines).
    if (p != PAGE_READWRITE && p != PAGE_WRITECOPY) return false;
    if (mbi.Type == MEM_PRIVATE) {
        // game/CRT heap — keep, unless it's a skipped base (stack / our buffer).
    } else if (mbi.Type == MEM_IMAGE) {
        uintptr_t b = (uintptr_t)mbi.BaseAddress;
        if (!(b >= ex.supremeBase && b < ex.supremeEnd)) return false; // only Supreme .data
    } else {
        return false; // MEM_MAPPED (shared mem, file maps) — never snapshot
    }
    if (snap_is_skipped(ex, (uintptr_t)mbi.AllocationBase)) return false;
    return true;
}

// ---- thread suspension ------------------------------------------------------

static constexpr int SNAP_MAX_THREADS = 64;
struct SnapThreads { HANDLE h[SNAP_MAX_THREADS]; int n; };

// Suspend every thread in this process except the caller. Also collects each
// thread's stack AllocationBase into `ex` so stacks are excluded from the snap.
static void snap_suspend_others(SnapThreads& st, SnapExclusion* ex) {
    st.n = 0;
    DWORD me = GetCurrentThreadId();
    DWORD pid = GetCurrentProcessId();
    HANDLE snap = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
    if (snap == INVALID_HANDLE_VALUE) return;
    THREADENTRY32 te; te.dwSize = sizeof(te);
    if (Thread32First(snap, &te)) {
        do {
            if (te.th32OwnerProcessID != pid) continue;
            // Record stack AllocationBase (all threads, incl. current).
            if (ex) {
                HANDLE ht = OpenThread(THREAD_GET_CONTEXT, FALSE, te.th32ThreadID);
                if (ht) {
                    CONTEXT ctx; ctx.ContextFlags = CONTEXT_CONTROL;
                    if (GetThreadContext(ht, &ctx)) {
                        MEMORY_BASIC_INFORMATION mbi;
                        if (VirtualQuery((LPCVOID)(uintptr_t)ctx.Esp, &mbi, sizeof(mbi)))
                            snap_add_skip(*ex, (uintptr_t)mbi.AllocationBase);
                    }
                    CloseHandle(ht);
                }
            }
            if (te.th32ThreadID == me) continue;
            HANDLE ht = OpenThread(THREAD_SUSPEND_RESUME, FALSE, te.th32ThreadID);
            if (ht && st.n < SNAP_MAX_THREADS) {
                SuspendThread(ht);
                st.h[st.n++] = ht;
            } else if (ht) {
                CloseHandle(ht);
            }
        } while (Thread32Next(snap, &te));
    }
    CloseHandle(snap);
}

static void snap_resume(SnapThreads& st) {
    for (int i = 0; i < st.n; i++) {
        ResumeThread(st.h[i]);
        CloseHandle(st.h[i]);
    }
    st.n = 0;
}

// ---- capture / restore ------------------------------------------------------

// Capture the snapshot. Returns bytes captured (0 on failure). Records timing
// (microseconds) into *out_us. FPU-safe (no float/format/Log-with-format).
static uint32_t SnapshotCapture(TasSharedState* s, uint64_t* out_us) {
    LARGE_INTEGER freq, t0, t1;
    QueryPerformanceFrequency(&freq);
    QueryPerformanceCounter(&t0);

    SnapExclusion ex; ex.skipCount = 0;
    snap_supreme_bounds(ex);

    // Exclude the TAS shared-memory mapping (by AllocationBase).
    {
        MEMORY_BASIC_INFORMATION mbi;
        if (s && VirtualQuery((LPCVOID)s, &mbi, sizeof(mbi)))
            snap_add_skip(ex, (uintptr_t)mbi.AllocationBase);
    }
    // Exclude the snapshot buffer itself, if already allocated.
    if (g_snapBuffer) snap_add_skip(ex, (uintptr_t)g_snapBuffer);

    // Pass 1 (threads RUNNING): size the snapshot so we can pre-allocate the
    // buffer BEFORE suspending (suspend-then-VirtualAlloc is fine, but we also
    // want the stack AllocationBases, gathered during suspend below). We compute
    // stacks first via a dry suspend? No: collect stacks now without suspending.
    // Collect stack bases by walking threads (no suspend) so pass-1 sizing
    // already excludes stacks.
    {
        SnapThreads dummy; // not used for suspending here
        // Reuse snap_suspend_others purely to gather stack bases would suspend;
        // instead gather contexts without suspending:
        DWORD pid = GetCurrentProcessId();
        HANDLE th = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if (th != INVALID_HANDLE_VALUE) {
            THREADENTRY32 te; te.dwSize = sizeof(te);
            if (Thread32First(th, &te)) {
                do {
                    if (te.th32OwnerProcessID != pid) continue;
                    HANDLE ht = OpenThread(THREAD_GET_CONTEXT, FALSE, te.th32ThreadID);
                    if (ht) {
                        CONTEXT ctx; ctx.ContextFlags = CONTEXT_CONTROL;
                        if (GetThreadContext(ht, &ctx)) {
                            MEMORY_BASIC_INFORMATION mbi;
                            if (VirtualQuery((LPCVOID)(uintptr_t)ctx.Esp, &mbi, sizeof(mbi)))
                                snap_add_skip(ex, (uintptr_t)mbi.AllocationBase);
                        }
                        CloseHandle(ht);
                    }
                } while (Thread32Next(th, &te));
            }
            CloseHandle(th);
        }
        (void)dummy;
    }

    // Pass 1: total bytes + region count.
    size_t total = 0; uint32_t count = 0;
    {
        uint8_t* addr = 0;
        MEMORY_BASIC_INFORMATION mbi;
        while (VirtualQuery((LPCVOID)addr, &mbi, sizeof(mbi)) == sizeof(mbi)) {
            if (snap_region_included(mbi, ex)) { total += mbi.RegionSize; count++; }
            uint8_t* next = (uint8_t*)mbi.BaseAddress + mbi.RegionSize;
            if (next <= addr) break; // overflow guard
            addr = next;
            if (count >= SNAP_MAX_REGIONS) break;
        }
    }
    if (total == 0 || count == 0) { LogRing(s, LOG_ERROR, "[snap] no regions"); return 0; }

    // (Re)allocate buffer if needed (threads still running — safe).
    if (g_snapCapacity < total) {
        if (g_snapBuffer) { VirtualFree(g_snapBuffer, 0, MEM_RELEASE); g_snapBuffer = nullptr; }
        g_snapBuffer = (uint8_t*)VirtualAlloc(nullptr, total, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if (!g_snapBuffer) { g_snapCapacity = 0; LogRing(s, LOG_ERROR, "[snap] VirtualAlloc failed"); return 0; }
        g_snapCapacity = total;
        // Re-exclude the (possibly new) buffer base for pass 2.
        snap_add_skip(ex, (uintptr_t)g_snapBuffer);
    }

    // Pass 2 (threads SUSPENDED): memcpy each region into the buffer.
    SnapThreads st; snap_suspend_others(st, nullptr);
    uint32_t rc = 0; size_t off = 0;
    {
        uint8_t* addr = 0;
        MEMORY_BASIC_INFORMATION mbi;
        while (VirtualQuery((LPCVOID)addr, &mbi, sizeof(mbi)) == sizeof(mbi)) {
            if (snap_region_included(mbi, ex) && rc < SNAP_MAX_REGIONS && off + mbi.RegionSize <= g_snapCapacity) {
                memcpy(g_snapBuffer + off, mbi.BaseAddress, mbi.RegionSize);
                g_snapRegions[rc].base   = (uintptr_t)mbi.BaseAddress;
                g_snapRegions[rc].size   = (uint32_t)mbi.RegionSize;
                g_snapRegions[rc].offset = (uint32_t)off;
                off += mbi.RegionSize; rc++;
            }
            uint8_t* next = (uint8_t*)mbi.BaseAddress + mbi.RegionSize;
            if (next <= addr) break;
            addr = next;
        }
    }
    snap_resume(st);

    g_snapRegionCount = rc;
    g_snapUsed = off;
    g_snapValid = true;

    QueryPerformanceCounter(&t1);
    uint64_t us = (uint64_t)((t1.QuadPart - t0.QuadPart) * 1000000 / freq.QuadPart);
    if (out_us) *out_us = us;

    // Publish to shared state (diagnostics).
    s->snapshot_size = (uint32_t)off;
    s->snapshot_buffer_ptr = (uint32_t)(uintptr_t)g_snapBuffer;
    s->snapshot_buffer_capacity = (uint32_t)g_snapCapacity;
    return (uint32_t)off;
}

// Restore the snapshot. Returns bytes restored (0 if none). Sets the clock
// backlog reset so the resumed sim doesn't burst. FPU-safe.
static uint32_t SnapshotRestore(TasSharedState* s, uint64_t* out_us) {
    if (!g_snapValid || !g_snapBuffer || g_snapRegionCount == 0) {
        LogRing(s, LOG_ERROR, "[snap] restore: no valid snapshot");
        return 0;
    }
    LARGE_INTEGER freq, t0, t1;
    QueryPerformanceFrequency(&freq);
    QueryPerformanceCounter(&t0);

    SnapThreads st; snap_suspend_others(st, nullptr);
    size_t bytes = 0;
    uint32_t restored = 0, skipped = 0, faulted = 0;
    for (uint32_t i = 0; i < g_snapRegionCount; i++) {
        // Heap-shape robustness: a region may have been freed, shrunk, or had its
        // protection changed since the snapshot (the game churns the heap while
        // the user records between snapshot and restore). Re-validate before
        // writing so we don't fault or clobber a reallocated region.
        MEMORY_BASIC_INFORMATION mbi;
        bool ok = VirtualQuery((LPCVOID)g_snapRegions[i].base, &mbi, sizeof(mbi)) == sizeof(mbi)
            && mbi.State == MEM_COMMIT
            && (uintptr_t)mbi.BaseAddress == g_snapRegions[i].base
            && mbi.RegionSize >= g_snapRegions[i].size
            && ((mbi.Protect & 0xFF) == PAGE_READWRITE || (mbi.Protect & 0xFF) == PAGE_WRITECOPY)
            && !(mbi.Protect & PAGE_GUARD);
        if (!ok) { skipped++; continue; }
        __try {
            memcpy((void*)g_snapRegions[i].base, g_snapBuffer + g_snapRegions[i].offset, g_snapRegions[i].size);
            bytes += g_snapRegions[i].size;
            restored++;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            faulted++;
        }
    }
    snap_resume(st);
    // Publish region accounting for the long-window diagnostic.
    g_snapLastRestored = restored;
    g_snapLastSkipped = skipped;
    g_snapLastFaulted = faulted;

    // Rewind injected the snapshot's tick clock; reset the backlog to "now" so
    // the next frame doesn't fast-forward (reuse the frame-exact CONT primitive).
    s->cont_reset_pending = 1;

    QueryPerformanceCounter(&t1);
    uint64_t us = (uint64_t)((t1.QuadPart - t0.QuadPart) * 1000000 / freq.QuadPart);
    if (out_us) *out_us = us;
    return (uint32_t)bytes;
}

// ---- frame-exact rewind proof ----------------------------------------------
// Read the player's X/Y/Z as raw bits via s->player_ptr (no float ops needed).
// Returns false if there's no live player pointer.
static bool SnapReadPlayerBits(TasSharedState* s, uint32_t out[3]) {
    if (!s || !s->player_ptr) return false;
    bool ok = true;
    __try {
        uint8_t* p = (uint8_t*)(uintptr_t)s->player_ptr;
        memcpy(&out[0], p + GameAddresses::PLAYER_X, 4);
        memcpy(&out[1], p + GameAddresses::PLAYER_Y, 4);
        memcpy(&out[2], p + GameAddresses::PLAYER_Z, 4);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        ok = false;
    }
    return ok;
}

// Player coords (bit pattern) captured at the snapshot instant; compared to the
// post-restore read in the SAME hook call (zero frames between) to prove the
// restore rewound the sim bit-exactly.
static uint32_t g_snapPlayerBits[3] = {0, 0, 0};
static bool     g_snapPlayerValid = false;

// Returns how many of X/Y/Z matched the snapshot bit-exactly (0..3); 3 = the
// player object was reverted perfectly. 0xFF if no player pointer.
static uint32_t SnapshotRevertMatch(TasSharedState* s) {
    if (!g_snapPlayerValid) return 0xFF;
    uint32_t now[3];
    if (!SnapReadPlayerBits(s, now)) return 0xFF;
    uint32_t m = 0;
    for (int i = 0; i < 3; i++) if (now[i] == g_snapPlayerBits[i]) m++;
    return m;
}

// ---- frame-exact trajectory determinism check ------------------------------
// Record the player coords for N frames after snapshot (traj A) and N frames
// after restore (traj B), in the DLL on consecutive game frames (no probe-
// timing confound). Compare with a small frame-shift tolerance — a perfect sim
// reproduction differs from the snapshot run only by the clock-reset's ±1 tick,
// so at the best shift the trajectories should be ~bit-identical. A large
// best-shift mismatch = genuine un-reverted sim state.
static constexpr int SNAP_TRAJ_N = 24;
static uint32_t g_trajA[SNAP_TRAJ_N][3];
static uint32_t g_trajB[SNAP_TRAJ_N][3];
static int g_trajMode = 0;   // 0=idle, 1=recording A, 2=recording B
static int g_trajCount = 0;
// Result (published when B fills): best_match = most frames bit-identical over
// shifts in [-3,3]; best_shift = the aligning shift. -1 until ready.
static int g_trajBestMatch = -1;
static int g_trajBestShift = 0;

static void SnapTrajStart(int mode) { g_trajMode = mode; g_trajCount = 0; if (mode == 1) g_trajBestMatch = -1; }

// Call once per frame from Cave2_Logic AFTER the player position is updated.
static void SnapTrajTick(TasSharedState* s) {
    if (g_trajMode == 0) return;
    uint32_t b[3];
    if (!SnapReadPlayerBits(s, b)) return;
    auto* dst = (g_trajMode == 1) ? g_trajA : g_trajB;
    if (g_trajCount < SNAP_TRAJ_N) {
        dst[g_trajCount][0] = b[0]; dst[g_trajCount][1] = b[1]; dst[g_trajCount][2] = b[2];
        g_trajCount++;
    }
    if (g_trajCount >= SNAP_TRAJ_N) {
        int finished = g_trajMode;
        g_trajMode = 0;
        if (finished == 2) {
            // Compare A vs B over shifts; count bit-exact frame matches.
            int best = -1, bestShift = 0;
            for (int sft = -3; sft <= 3; sft++) {
                int matches = 0;
                for (int i = 0; i < SNAP_TRAJ_N; i++) {
                    int j = i + sft;
                    if (j < 0 || j >= SNAP_TRAJ_N) continue;
                    if (g_trajA[i][0] == g_trajB[j][0] && g_trajA[i][1] == g_trajB[j][1] && g_trajA[i][2] == g_trajB[j][2])
                        matches++;
                }
                if (matches > best) { best = matches; bestShift = sft; }
            }
            g_trajBestMatch = best;
            g_trajBestShift = bestShift;
            // Publish for the probe: high16 = shift+8, low16 = best match count.
            s->event_count = ((uint32_t)(bestShift + 8) << 16) | (uint32_t)(best & 0xFFFF);
        }
    }
}
