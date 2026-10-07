// readset.hpp, live in this process: page guards hit, re-armed after every
// access, attributed to spans and threads, bounded by the exception cap, and
// never left behind; other guard pages pass through; data breakpoints set and
// cleared through the handler, also from another thread. Prints the cost of a
// guard hit (guard exception + single-step re-arm).
#include "../readset.hpp"
#include <cstdio>
#include "check.hpp"

static bool Guarded(const void* p) {
    MEMORY_BASIC_INFORMATION mi;
    VirtualQuery(p, &mi, sizeof mi);
    return (mi.Protect & PAGE_GUARD) != 0;
}

__declspec(noinline) static uint32_t ReadAt(const volatile uint32_t* p) { return *p; }
__declspec(noinline) static void WriteAt(volatile uint32_t* p, uint32_t v) { *p = v; }

// A guard page that is not ours reaches the program's own handler.
static bool ForeignGuardPassesThrough(const volatile uint32_t* p) {
    __try {
        (void)*p;
        return false;
    } __except (GetExceptionCode() == STATUS_GUARD_PAGE_VIOLATION ? EXCEPTION_EXECUTE_HANDLER
                                                                    : EXCEPTION_CONTINUE_SEARCH) {
        return true;
    }
}

static DWORD WINAPI OtherReader(void* p) {
    return ReadAt((const volatile uint32_t*)p);
}

static HANDLE g_go;
static DWORD WINAPI RemoteDisarm(void*) {
    WaitForSingleObject(g_go, INFINITE);
    readset::HwDisarm();
    return 0;
}

static readset::Site Find(uint32_t span, bool write) {
    for (const readset::Site& s : readset::Sites())
        if (s.span == span && s.write == (uint8_t)write) return s;
    return {};
}

int main() {
    setvbuf(stdout, nullptr, _IONBF, 0);   // a crash still shows how far it got
    check(readset::Install(), "install_handler");
    auto* mem = (uint8_t*)VirtualAlloc(nullptr, 0x3000, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    const uint32_t base = (uint32_t)(uintptr_t)mem;
    auto* a = (volatile uint32_t*)(mem + 0x100);   // span 0
    auto* outside = (volatile uint32_t*)(mem + 0x800);   // same page, no span
    auto* c = (volatile uint32_t*)(mem + 0x2004);  // span 1
    auto* untouched = mem + 0x1000;                // page 1, not covered
    int local = 0;
    std::vector<readset::Span> spans = {{base + 0x100, base + 0x200},
                                        {base + 0x2000, base + 0x2010},
                                        {(uint32_t)(uintptr_t)&local, (uint32_t)(uintptr_t)&local + 4}};

    readset::ArmResult r = readset::Arm(spans, 1000);
    check(r.pages == 2 && r.skipped == 1 && !r.truncated, "arm_guards_covering_pages_not_the_stack");
    check(Guarded(mem) && Guarded(mem + 0x2000) && !Guarded(untouched), "arm_page_protection");
    for (int i = 0; i < 5; i++) ReadAt(a);
    check(Guarded(mem), "rearmed_after_each_hit");
    WriteAt(a + 1, 42);
    ReadAt(outside);
    ReadAt(c);
    HANDLE t = CreateThread(nullptr, 0, OtherReader, (void*)c, 0, nullptr);
    WaitForSingleObject(t, INFINITE);
    CloseHandle(t);
    check(Guarded(mem) && Guarded(mem + 0x2000), "still_armed_before_disarm");
    readset::Disarm();
    check(!Guarded(mem) && !Guarded(mem + 0x2000) && !Guarded(untouched), "disarm_leaves_no_guard");
    check(readset::g_hits == 8 && readset::g_ignored == 1 && readset::g_other == 1 && !readset::g_bailed,
          "hits_ignored_other_counted");
    const readset::Site rd = Find(0, false), wr = Find(0, true);
    check(rd.count == 5 && rd.tid == GetCurrentThreadId() && rd.addr == base + 0x100, "read_site_deduplicated");
    check(wr.count == 1 && wr.addr == base + 0x104 && *(a + 1) == 42, "write_site_and_value");
    uint32_t span1 = 0, threads = 0;
    for (const readset::Site& s : readset::Sites())
        if (s.span == 1) span1 += s.count, threads |= s.tid == GetCurrentThreadId() ? 1 : 2;
    check(span1 == 2 && threads == 3, "other_thread_site");
    check(rd.eip != wr.eip && rd.eip, "sites_carry_eip");

    // The cap: the window restores everything at once and stops re-arming.
    r = readset::Arm(spans, 10);
    for (int i = 0; i < 100; i++) ReadAt(a);
    check(readset::g_bailed && !readset::g_armed && !Guarded(mem) && !Guarded(mem + 0x2000), "storm_bails_out");
    check(readset::g_exceptions == 11 && readset::g_hits == 10, "storm_bounded");
    readset::Disarm();

    // A guard page of someone else's is not swallowed.
    DWORD old;
    VirtualProtect(untouched, 0x1000, PAGE_READWRITE | PAGE_GUARD, &old);
    check(ForeignGuardPassesThrough((const volatile uint32_t*)untouched), "foreign_guard_passes_through");

    // Cost of one hit (guard exception + single-step re-arm).
    r = readset::Arm(spans, 1000000);
    LARGE_INTEGER f, t0, t1;
    QueryPerformanceFrequency(&f);
    QueryPerformanceCounter(&t0);
    for (int i = 0; i < 20000; i++) ReadAt(a);
    QueryPerformanceCounter(&t1);
    readset::Disarm();
    std::printf("  info guard hit round trip: %.2f us (20000 hits)\n",
                (t1.QuadPart - t0.QuadPart) * 1e6 / f.QuadPart / 20000);
    check(readset::g_hits == 20000 && !Guarded(mem), "timing_window_clean");

    // Data breakpoints on this thread.
    alignas(16) static volatile uint32_t watched[4] = {1, 2, 3, 4};
    const uint32_t w1 = (uint32_t)(uintptr_t)&watched[1];
    check(readset::HwArm(&w1, 1), "hw_arm");
    ReadAt(&watched[1]);
    WriteAt(&watched[1], 22);
    ReadAt(&watched[0]);
    ReadAt(&watched[2]);
    readset::HwDisarm();
    check(!readset::g_hwArmed, "hw_disarm");
    ReadAt(&watched[1]);
    std::vector<readset::HwHit> hw = readset::HwHits();
    check(hw.size() == 2 && hw[0].addr == w1 && hw[0].value == 2 && !hw[0].changed && hw[1].value == 22 &&
              hw[1].changed && hw[0].dr == 0 && hw[0].tid == GetCurrentThreadId(),
          "hw_hits_values");
    check(hw.size() == 2 && hw[0].eip != hw[1].eip, "hw_eip_after_access");

    // Four watches, one combined with a page guard: that dword is not read.
    const uint32_t ws[4] = {(uint32_t)(uintptr_t)&watched[0], w1, (uint32_t)(uintptr_t)&watched[3],
                            base + 0x104};
    check(readset::HwArm(ws, 4), "hw_arm_four");
    r = readset::Arm(spans, 1000);
    ReadAt(&watched[3]);
    ReadAt(a + 1);
    readset::Disarm();
    readset::HwDisarm();
    hw = readset::HwHits();
    check(hw.size() == 2 && hw[0].dr == 2 && hw[1].dr == 3 && hw[1].unread && !hw[0].unread, "hw_four_and_guarded");
    check(Find(0, false).count == 1 && !Guarded(mem), "hw_and_guard_together");

    // Cleared from another thread while this one waits.
    check(readset::HwArm(&w1, 1), "hw_arm_again");
    g_go = CreateEventA(nullptr, TRUE, FALSE, nullptr);
    t = CreateThread(nullptr, 0, RemoteDisarm, nullptr, 0, nullptr);
    SetEvent(g_go);
    WaitForSingleObject(t, INFINITE);
    CloseHandle(t);
    check(!readset::g_hwArmed, "hw_remote_disarm");
    ReadAt(&watched[1]);
    check(readset::HwHits().empty(), "hw_remote_disarm_effective");

    readset::Uninstall();
    check(!Guarded(mem) && !Guarded(mem + 0x2000), "uninstall_clean");
    VirtualFree(mem, 0, MEM_RELEASE);
    return FinishTests();
}
