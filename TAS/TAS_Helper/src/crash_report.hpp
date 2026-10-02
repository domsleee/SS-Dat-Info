#pragma once
#include "stdafx.h"
#include <psapi.h>
#include "shared_state.hpp"
#include "crash_record.hpp"

// Crash record (DESIGN.md "Crashes"). Calls into game code are not guarded:
// a fault there crashes the game, and the UI says where. Nothing here
// allocates, formats or takes a lock once installed, because it runs inside a
// faulting process.
namespace crash {

inline TasSharedState* g_state = nullptr;
inline PVOID g_vectored = nullptr;
inline LPTOP_LEVEL_EXCEPTION_FILTER g_previousFilter = nullptr;

// Loaded modules, refreshed from the pump into the spare buffer and published
// by flipping the index, so a fault never reads a half-written table.
inline crashrecord::ModuleTable g_modules[2];
inline volatile LONG g_liveModules = 0;
inline ULONGLONG g_lastRefreshMs = 0;

// The DLL call into game code in flight, for the crash record. Not usable in a
// function that also holds a __try (MSVC forbids unwinding objects there).
class ScopedGameCall {
public:
    explicit ScopedGameCall(uint32_t call) {
        if (g_state) {
            previous_ = g_state->game_call;
            g_state->game_call = call;
        }
    }
    ~ScopedGameCall() {
        if (g_state) g_state->game_call = previous_;
    }
    ScopedGameCall(const ScopedGameCall&) = delete;
    ScopedGameCall& operator=(const ScopedGameCall&) = delete;

private:
    uint32_t previous_ = TAS_GAME_CALL_NONE;
};

static void Record(const EXCEPTION_RECORD* er) {
    auto* s = g_state;
    if (!s) return;
    const uint32_t address = (uint32_t)(uintptr_t)er->ExceptionAddress;
    s->crash_pid = GetCurrentProcessId();
    s->crash_code = er->ExceptionCode;
    s->crash_address = address;
    s->crash_game_call = s->game_call;
    s->crash_thread_id = GetCurrentThreadId();
    const auto& table = g_modules[g_liveModules & 1];
    const int i = table.Find(address);
    s->crash_module_offset = i >= 0 ? address - table.entries[i].base : address;
    for (uint32_t k = 0; k < TAS_CRASH_MODULE_MAX; k++) {
        s->crash_module[k] = i >= 0 ? table.entries[i].name[k] : '\0';
    }
    MemoryBarrier();  // the record is complete before the sequence moves
    InterlockedIncrement((volatile LONG*)&s->crash_seq);
}

// First chance, so it sees a fault inside a game call even when the game's own
// handler then catches it and shows its kernel-error dialog.
static LONG CALLBACK OnVectoredException(EXCEPTION_POINTERS* ep) {
    auto* s = g_state;
    if (s && s->game_call != TAS_GAME_CALL_NONE &&
        crashrecord::IsFault(ep->ExceptionRecord->ExceptionCode)) {
        Record(ep->ExceptionRecord);
    }
    return EXCEPTION_CONTINUE_SEARCH;
}

static LONG WINAPI OnUnhandledException(EXCEPTION_POINTERS* ep) {
    // A fault inside a game call is already recorded.
    if (g_state && g_state->game_call == TAS_GAME_CALL_NONE) Record(ep->ExceptionRecord);
    return g_previousFilter ? g_previousFilter(ep) : EXCEPTION_CONTINUE_SEARCH;
}

static void RefreshModules() {
    HMODULE handles[crashrecord::kMaxModules];
    DWORD needed = 0;
    if (!K32EnumProcessModules(GetCurrentProcess(), handles, sizeof handles, &needed)) return;
    const LONG spare = (g_liveModules & 1) ^ 1;
    auto& table = g_modules[spare];
    table.count = 0;
    const DWORD n = needed / sizeof(HMODULE);
    for (DWORD i = 0; i < n && i < crashrecord::kMaxModules; i++) {
        MODULEINFO info{};
        if (!K32GetModuleInformation(GetCurrentProcess(), handles[i], &info, sizeof info)) continue;
        auto& e = table.entries[table.count];
        e.base = (uint32_t)(uintptr_t)info.lpBaseOfDll;
        e.size = info.SizeOfImage;
        if (!K32GetModuleBaseNameA(GetCurrentProcess(), handles[i], e.name, sizeof e.name)) {
            e.name[0] = '\0';
        }
        table.count++;
    }
    InterlockedExchange(&g_liveModules, spare);
}

inline void Install(TasSharedState* s) {
    g_state = s;
    RefreshModules();
    g_lastRefreshMs = GetTickCount64();
    g_vectored = AddVectoredExceptionHandler(1, &OnVectoredException);
    g_previousFilter = SetUnhandledExceptionFilter(&OnUnhandledException);
}

// Pump: keep the module table current and the filter installed (the game or
// its runtime may install its own later; ours then chains to it).
inline void Maintain() {
    if (!g_state) return;
    const ULONGLONG now = GetTickCount64();
    if (now - g_lastRefreshMs >= 2000) {
        g_lastRefreshMs = now;
        RefreshModules();
    }
    LPTOP_LEVEL_EXCEPTION_FILTER current = SetUnhandledExceptionFilter(&OnUnhandledException);
    if (current != &OnUnhandledException) g_previousFilter = current;
}

// DllMain, process detach from ExitProcess: the game closed normally.
inline void MarkCleanExit() {
    if (g_state) g_state->game_exit_clean = 1;
}

inline void Uninstall() {
    if (g_vectored) RemoveVectoredExceptionHandler(g_vectored);
    g_vectored = nullptr;
    if (g_state) SetUnhandledExceptionFilter(g_previousFilter);
    g_state = nullptr;
}

}  // namespace crash
