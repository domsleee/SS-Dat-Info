#pragma once
#include <windows.h>
#include <cstdint>
#include <cstring>
#include "shared_state.hpp"
#include "external/safetyhook.hpp"

// Event-driven level identity, replacing the heap-scan heuristic.
//
// The engine loads each track from loose files under
// `Data/Levels/<Area>/<Category>/<Difficulty>/...`, so the moment it opens one
// we know EXACTLY which track is loading — no tally, no majority vote, no
// confidence floor, no settle gate, and no polling latency.
//
// This is what level.rs always assumed would happen ("live detection ... needs a
// load-time hook like the helper's saveReplayTimestamp"); the Display_Config
// helper already proves the engine hands these paths around as plain strings.
//
// It is also STRICTLY MORE INFORMATIVE than the scan: the scan only matches
// "<area>/Tracks/<diff>", so Practice, Special, Halfpipe and Ramp all collapse
// to "unknown". The path carries the category, so the UI can name them.
namespace levelhook {

inline TasSharedState* g_state = nullptr;
inline safetyhook::InlineHook g_createFileA{};
inline safetyhook::InlineHook g_createFileW{};

// Case-insensitive search for `needle` in `hay` (ASCII).
static const char* findNoCase(const char* hay, const char* needle) {
    if (!hay || !needle) return nullptr;
    for (const char* h = hay; *h; ++h) {
        const char* a = h;
        const char* b = needle;
        while (*a && *b) {
            char ca = *a, cb = *b;
            if (ca >= 'A' && ca <= 'Z') ca = (char)(ca + 32);
            if (cb >= 'A' && cb <= 'Z') cb = (char)(cb + 32);
            if (ca != cb) break;
            ++a; ++b;
        }
        if (!*b) return h;
    }
    return nullptr;
}

// Record a level path if this looks like one. Cheap enough to run on every file
// open: one case-insensitive substring probe, then a bounded copy.
//
// Publishes ONLY when the path changes, so the common case (the same level's
// many resource files) costs a compare and nothing else. The change itself is
// the level-change event — no root pointer, no epoch, no invalidation window.
static void notePath(const char* path) {
    auto* s = g_state;
    if (!s) return;
    s->level_hook_calls++;
    if (!path) return;
    const char* levels = findNoCase(path, "levels");
    if (!levels) return;
    // Need at least "<Area>/<Category>/<Difficulty>/" after "Levels".
    if (!levels[6]) return;

    // Copy from "Levels" onward; the UI parser keys off the `levels` segment and
    // does not care about the drive/install prefix.
    char buf[TAS_LEVEL_PATH_MAX];
    size_t n = 0;
    for (const char* p = levels; *p && n < TAS_LEVEL_PATH_MAX - 1; ++p, ++n) {
        buf[n] = *p;
    }
    buf[n] = '\0';

    if (std::strcmp(buf, s->level_path) == 0) return;  // same level, nothing to say
    // Publish the string, THEN bump the generation: a reader that sees a new
    // generation must already be able to see the path it refers to.
    std::memcpy(s->level_path, buf, n + 1);
    MemoryBarrier();
    s->level_path_gen++;
}

static HANDLE WINAPI CreateFileA_Detour(LPCSTR name, DWORD access, DWORD share,
                                        LPSECURITY_ATTRIBUTES sa, DWORD disp,
                                        DWORD flags, HANDLE tmpl) {
    notePath(name);
    return g_createFileA.stdcall<HANDLE>(name, access, share, sa, disp, flags, tmpl);
}

static HANDLE WINAPI CreateFileW_Detour(LPCWSTR name, DWORD access, DWORD share,
                                        LPSECURITY_ATTRIBUTES sa, DWORD disp,
                                        DWORD flags, HANDLE tmpl) {
    if (name) {
        char narrow[TAS_LEVEL_PATH_MAX];
        int n = WideCharToMultiByte(CP_ACP, 0, name, -1, narrow,
                                    (int)sizeof(narrow), nullptr, nullptr);
        if (n > 0) notePath(narrow);
    }
    return g_createFileW.stdcall<HANDLE>(name, access, share, sa, disp, flags, tmpl);
}

inline bool Install(TasSharedState* s) {
    g_state = s;
    s->level_path[0] = '\0';
    HMODULE k32 = GetModuleHandleA("kernel32.dll");
    if (!k32) return false;
    auto* a = GetProcAddress(k32, "CreateFileA");
    auto* w = GetProcAddress(k32, "CreateFileW");
    bool ok = false;
    if (a) {
        g_createFileA = safetyhook::create_inline((void*)a, (void*)CreateFileA_Detour);
        ok = ok || (bool)g_createFileA;
    }
    if (w) {
        g_createFileW = safetyhook::create_inline((void*)w, (void*)CreateFileW_Detour);
        ok = ok || (bool)g_createFileW;
    }
    return ok;
}

} // namespace levelhook
