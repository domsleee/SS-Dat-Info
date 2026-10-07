#pragma once
#include <windows.h>
#include <algorithm>
#include <cstdint>
#include <cstring>
#include <vector>
#include "safe_read.hpp"

// Physics read-set (ssb-recomp re/harness.md "Pointer census"): which code
// touches given memory during one physics window (the human Body's step entry
// to the human Player tick's return).
//
// Page guards: Arm() adds PAGE_GUARD to every page covering the spans. A hit
// (STATUS_GUARD_PAGE_VIOLATION, any thread) inside a span is counted per
// (thread, EIP, span, read/write) site; any hit sets the trap flag, and the
// single-step after the access re-applies the guard. Disarm() restores every
// page's original protection; a page is never left guarded. Over
// `maxExceptions` hits in one window the window bails out: every page is
// restored at once and nothing is re-armed.
//
// Data breakpoints: HwArm() puts up to four 4-byte read/write watches into
// DR0-3 of the calling thread by raising a private exception whose handler
// edits the context (NtContinue applies the debug registers); each trap is
// logged with the EIP after the access and the dword's value then.
//
// The handler runs on any thread, possibly inside the heap manager with its
// lock held, so it never allocates: fixed tables only.
namespace readset {

constexpr uint32_t kMaxSpans = 2048, kMaxPages = 4096, kMaxSites = 8192, kMaxHw = 4096;
constexpr DWORD kHwArm = 0xE0535301, kHwDisarm = 0xE0535302;   // private, continuable
constexpr DWORD kTrapFlag = 0x100;

struct Span {
    uint32_t lo, hi;   // [lo, hi)
};
struct Page {
    uint32_t addr;
    DWORD prot;   // the original protection
};
struct Site {
    uint32_t tid, eip, span, addr, count;   // addr: the first data address
    uint8_t write, used;
};
struct HwHit {
    uint32_t tid, eip, dr, addr, value;
    uint8_t changed, unread;   // unread: the dword sat on a guarded page, not read
};

inline Span g_spans[kMaxSpans];
inline uint32_t g_nspans = 0;
inline Page g_pages[kMaxPages];
inline uint32_t g_npages = 0;
inline Site g_sites[kMaxSites];
inline volatile LONG g_lock = 0;
inline volatile LONG g_armed = 0, g_bailed = 0;
inline volatile LONG g_exceptions = 0, g_hits = 0, g_ignored = 0, g_other = 0, g_siteOverflow = 0;
inline uint32_t g_maxExceptions = 100000;
inline DWORD g_gameTid = 0;
inline thread_local uint32_t t_rearm[8];
inline thread_local uint32_t t_nrearm = 0;
inline thread_local bool t_inHw = false;

inline uint32_t g_hwAddr[4], g_hwLast[4], g_hwCount = 0, g_dr7 = 0;
inline DWORD g_hwTid = 0;
inline volatile LONG g_hwArmed = 0, g_nhw = 0, g_hwOverflow = 0;
inline HwHit g_hw[kMaxHw];
inline PVOID g_veh = nullptr;

struct Lock {
    Lock() {
        while (InterlockedExchange(&g_lock, 1)) YieldProcessor();
    }
    ~Lock() { InterlockedExchange(&g_lock, 0); }
};

inline const Page* FindPage(uint32_t page) {
    uint32_t lo = 0, hi = g_npages;
    while (lo < hi) {
        const uint32_t mid = (lo + hi) / 2;
        if (g_pages[mid].addr < page) lo = mid + 1;
        else hi = mid;
    }
    return lo < g_npages && g_pages[lo].addr == page ? &g_pages[lo] : nullptr;
}

// Under the lock.
inline void RestoreAll() {
    for (uint32_t i = 0; i < g_npages; i++) {
        DWORD old;
        VirtualProtect((void*)(uintptr_t)g_pages[i].addr, 0x1000, g_pages[i].prot, &old);
    }
    g_armed = 0;
}

// Under the lock.
inline void RecordSite(uint32_t tid, uint32_t eip, uint32_t span, uint32_t addr, bool write) {
    uint32_t h = (tid * 0x9E3779B1u) ^ (eip * 0x85EBCA6Bu) ^ (span * 0xC2B2AE35u) ^ (write ? 0x27D4EB2Fu : 0);
    for (uint32_t k = 0; k < kMaxSites; k++) {
        Site& s = g_sites[(h + k) % kMaxSites];
        if (!s.used) {
            s = {tid, eip, span, addr, 1, (uint8_t)write, 1};
            return;
        }
        if (s.tid == tid && s.eip == eip && s.span == span && s.write == (uint8_t)write) {
            s.count++;
            return;
        }
    }
    g_siteOverflow++;
}

inline void SetDebugRegisters(CONTEXT* c, bool on) {
    c->Dr0 = on && g_hwCount > 0 ? g_hwAddr[0] : 0;
    c->Dr1 = on && g_hwCount > 1 ? g_hwAddr[1] : 0;
    c->Dr2 = on && g_hwCount > 2 ? g_hwAddr[2] : 0;
    c->Dr3 = on && g_hwCount > 3 ? g_hwAddr[3] : 0;
    c->Dr6 = 0;
    c->Dr7 = on ? g_dr7 : 0;
    c->ContextFlags |= CONTEXT_DEBUG_REGISTERS;
}

inline LONG CALLBACK Handler(EXCEPTION_POINTERS* ep) {
    const EXCEPTION_RECORD* er = ep->ExceptionRecord;
    CONTEXT* c = ep->ContextRecord;
    const DWORD code = er->ExceptionCode;
    if (code == STATUS_GUARD_PAGE_VIOLATION && er->NumberParameters >= 2) {
        const uint32_t addr = (uint32_t)er->ExceptionInformation[1];
        const bool write = er->ExceptionInformation[0] == 1;
        const DWORD tid = GetCurrentThreadId();
        Lock lock;
        const Page* p = FindPage(addr & ~0xFFFu);
        if (!p) return EXCEPTION_CONTINUE_SEARCH;           // not our guard
        if (!g_armed) return EXCEPTION_CONTINUE_EXECUTION;  // fired as the window closed: already restored
        if ((uint32_t)++g_exceptions > g_maxExceptions) {
            g_bailed = 1;
            RestoreAll();
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        bool any = false;
        for (uint32_t i = 0; i < g_nspans; i++) {
            if (addr - g_spans[i].lo < g_spans[i].hi - g_spans[i].lo) {
                RecordSite(tid, c->Eip, i, addr, write);
                any = true;
            }
        }
        if (!any) {
            g_ignored++;
        } else {
            g_hits++;
            if (tid != g_gameTid) g_other++;
        }
        if (t_nrearm < 8) t_rearm[t_nrearm++] = p->addr;
        c->EFlags |= kTrapFlag;
        return EXCEPTION_CONTINUE_EXECUTION;
    }
    if (code == STATUS_SINGLE_STEP) {
        bool mine = false;
        // Reading a watched dword below traps again, inside this handler.
        if (t_inHw) return EXCEPTION_CONTINUE_EXECUTION;
        if (g_hwArmed && GetCurrentThreadId() == g_hwTid && (c->Dr6 & 0xF)) {
            t_inHw = true;
            for (uint32_t i = 0; i < g_hwCount; i++) {
                if (!(c->Dr6 & (1u << i))) continue;
                const LONG n = InterlockedIncrement(&g_nhw) - 1;
                if (n >= (LONG)kMaxHw) {
                    g_nhw = kMaxHw;
                    g_hwOverflow++;
                    continue;
                }
                // A guarded dword is not read here: that would fault inside the handler.
                const bool guarded = g_armed && FindPage(g_hwAddr[i] & ~0xFFFu);
                const uint32_t v = guarded ? 0 : *(volatile uint32_t*)(uintptr_t)g_hwAddr[i];
                g_hw[n] = {GetCurrentThreadId(), c->Eip, i, g_hwAddr[i], v, (uint8_t)(!guarded && v != g_hwLast[i]),
                           (uint8_t)guarded};
                if (!guarded) g_hwLast[i] = v;
            }
            t_inHw = false;
            SetDebugRegisters(c, true);
            mine = true;
        }
        if (t_nrearm) {
            Lock lock;
            for (uint32_t i = 0; i < t_nrearm; i++) {
                const Page* p = FindPage(t_rearm[i]);
                DWORD old;
                if (g_armed && p) VirtualProtect((void*)(uintptr_t)p->addr, 0x1000, p->prot | PAGE_GUARD, &old);
            }
            t_nrearm = 0;
            c->EFlags &= ~kTrapFlag;
            mine = true;
        }
        return mine ? EXCEPTION_CONTINUE_EXECUTION : EXCEPTION_CONTINUE_SEARCH;
    }
    if ((code == kHwArm || code == kHwDisarm) && GetCurrentThreadId() == g_hwTid) {
        SetDebugRegisters(c, code == kHwArm);
        g_hwArmed = code == kHwArm;
        return EXCEPTION_CONTINUE_EXECUTION;
    }
    return EXCEPTION_CONTINUE_SEARCH;
}

// First in line, before the crash recorder's handler. Stays installed until
// Uninstall: another thread's single-step may still be on its way after a
// window closes.
inline bool Install() {
    if (!g_veh) g_veh = AddVectoredExceptionHandler(1, &Handler);
    return g_veh != nullptr;
}

// ---- page-guard windows ---------------------------------------------------------
struct ArmResult {
    uint32_t pages = 0, skipped = 0;   // skipped: not committed private read/write memory, or a stack
    bool truncated = false;
};

// Guards the pages of `spans` (at most kMaxSpans spans, kMaxPages pages).
// Only committed, private, unguarded, accessible pages off the calling
// thread's stack are guarded; the caller is the game thread.
inline ArmResult Arm(const std::vector<Span>& spans, uint32_t maxExceptions) {
    ArmResult r;
    MEMORY_BASIC_INFORMATION own{};
    VirtualQuery(&r, &own, sizeof own);
    std::vector<uint32_t> want;
    for (const Span& s : spans)
        for (uint32_t p = s.lo & ~0xFFFu; s.hi > s.lo && p <= ((s.hi - 1) & ~0xFFFu); p += 0x1000) want.push_back(p);
    std::sort(want.begin(), want.end());
    want.erase(std::unique(want.begin(), want.end()), want.end());
    std::vector<Page> pages;
    for (uint32_t p : want) {
        MEMORY_BASIC_INFORMATION mi;
        if (VirtualQuery((void*)(uintptr_t)p, &mi, sizeof mi) != sizeof mi || mi.State != MEM_COMMIT ||
            mi.Type != MEM_PRIVATE || (mi.Protect & (PAGE_GUARD | PAGE_NOACCESS)) || !mi.Protect ||
            mi.AllocationBase == own.AllocationBase) {
            r.skipped++;
            continue;
        }
        pages.push_back({p, mi.Protect});
    }
    r.truncated = spans.size() > kMaxSpans || pages.size() > kMaxPages;
    Lock lock;
    if (g_armed) return r;
    g_nspans = (uint32_t)std::min<size_t>(spans.size(), kMaxSpans);
    std::copy(spans.begin(), spans.begin() + g_nspans, g_spans);
    g_npages = (uint32_t)std::min<size_t>(pages.size(), kMaxPages);
    std::copy(pages.begin(), pages.begin() + g_npages, g_pages);
    memset(g_sites, 0, sizeof g_sites);
    g_exceptions = g_hits = g_ignored = g_other = g_siteOverflow = 0;
    g_bailed = 0;
    g_maxExceptions = maxExceptions;
    g_gameTid = GetCurrentThreadId();
    g_armed = 1;
    for (uint32_t i = 0; i < g_npages; i++) {
        DWORD old;
        if (VirtualProtect((void*)(uintptr_t)g_pages[i].addr, 0x1000, g_pages[i].prot | PAGE_GUARD, &old)) r.pages++;
    }
    return r;
}

// Idempotent; safe from any thread.
inline void Disarm() {
    Lock lock;
    if (g_armed) RestoreAll();
}

// The window's sites, in table order.
inline std::vector<Site> Sites() {
    std::vector<Site> out;
    for (const Site& s : g_sites)
        if (s.used) out.push_back(s);
    return out;
}

// ---- data breakpoints --------------------------------------------------------------
// Up to four 4-aligned addresses, read/write, 4 bytes, on the calling thread.
inline bool HwArm(const uint32_t* addrs, uint32_t n) {
    if (!g_veh || g_hwArmed || n == 0 || n > 4) return false;
    g_hwCount = n;
    g_dr7 = 0;
    for (uint32_t i = 0; i < n; i++) {
        g_hwAddr[i] = addrs[i];
        g_hwLast[i] = 0;
        if (addrs[i] & 3 || !SafeCopy(addrs[i], &g_hwLast[i], 4)) return false;
        g_dr7 |= (1u << (2 * i)) | (0xFu << (16 + 4 * i));   // local enable, RW=11 (read/write), LEN=11 (4)
    }
    g_nhw = g_hwOverflow = 0;
    g_hwTid = GetCurrentThreadId();
    RaiseException(kHwArm, 0, 0, nullptr);
    return g_hwArmed != 0;
}

// The hardware watch's thread clears its own registers; another thread
// suspends it and clears them through its context.
inline void HwDisarm() {
    if (!g_hwArmed) return;
    if (GetCurrentThreadId() == g_hwTid) {
        RaiseException(kHwDisarm, 0, 0, nullptr);
        return;
    }
    HANDLE t = OpenThread(THREAD_GET_CONTEXT | THREAD_SET_CONTEXT | THREAD_SUSPEND_RESUME, FALSE, g_hwTid);
    if (!t) return;
    if (SuspendThread(t) != (DWORD)-1) {
        CONTEXT c{};
        c.ContextFlags = CONTEXT_DEBUG_REGISTERS;
        if (GetThreadContext(t, &c)) {
            SetDebugRegisters(&c, false);
            if (SetThreadContext(t, &c)) g_hwArmed = 0;
        }
        ResumeThread(t);
    }
    CloseHandle(t);
}

inline std::vector<HwHit> HwHits() {
    const LONG n = std::min<LONG>((LONG)g_nhw, (LONG)kMaxHw);
    return std::vector<HwHit>(g_hw, g_hw + n);
}

inline void Uninstall() {
    Disarm();
    HwDisarm();
    if (g_veh) RemoveVectoredExceptionHandler(g_veh);
    g_veh = nullptr;
}

} // namespace readset
