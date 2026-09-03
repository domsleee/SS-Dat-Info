#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"
#include <format>

// Menu state, read from the MENU OBJECT (not the render text).
//
// The menus are a UI toolkit (UIT) driven by Main_Menu.dll. The page manager
// UI_Menu::Change_Page(name) is called with a fresh page whenever the screen
// changes - RVA 0x1A7C0, __fastcall(ECX = the UI_Menu, EDX = the page-name
// std::string). Hooking it (SafetyHook mid) captures the page name straight
// from the object and caches the UI_Menu instance for reading the selector.
// (Verified: the byte at +0x1A7C0 is a MSVC SEH prologue whose handler rebases
// to Main_Menu.dll+0x41169, matching the decompiled Change_Page.)
//
// This replaces the earlier approach of scraping the SR_UIT text renderer for
// menu titles - the object is the authoritative source and does not touch the
// per-frame HUD render path.
namespace menustate {

inline TasSharedState* g_state = nullptr;
inline uint32_t g_uiMenu = 0;   // cached UI_Menu (ECX at Change_Page); 0 until first change
inline SafetyHookMid g_changePageHook{};

// Change_Page's page-name string is an MSVC6 std::string ({alloc, char* ptr,
// size, capacity}); the pointer is at +4, the length at +8.
static bool ReadMenuString(uint32_t obj, char* out, uint32_t cap) {
    if (obj < 0x10000) return false;
    __try {
        const uint32_t ptr = *(uint32_t*)(obj + 4);
        const uint32_t len = *(uint32_t*)(obj + 8);
        if (ptr < 0x10000 || len == 0 || len + 1 > cap) return false;
        const char* s = (const char*)ptr;
        for (uint32_t i = 0; i < len; i++) {
            const char c = s[i];
            if (c < 0x20 || c > 0x7E) return false;   // not a plain page name
            out[i] = c;
        }
        out[len] = 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Publish menu_screen on change (a display aid, not seqlocked: a torn read is a
// one-frame blip that self-heals). Also called with "" to clear it in a level.
inline void PublishMenuScreen(const char* title) {
    if (!g_state) return;
    if (strncmp(g_state->menu_screen, title, TAS_MENU_SCREEN_MAX) == 0) return;
    uint32_t i = 0;
    for (; title[i] && i < TAS_MENU_SCREEN_MAX - 1; i++) g_state->menu_screen[i] = title[i];
    g_state->menu_screen[i] = 0;
}

static void ChangePageCb(SafetyHookContext& ctx) {
    g_uiMenu = (uint32_t)ctx.ecx;   // the UI_Menu instance, for the selector
    char name[TAS_MENU_SCREEN_MAX];
    if (ReadMenuString((uint32_t)ctx.edx, name, sizeof name)) PublishMenuScreen(name);
}

static DWORD WINAPI InstallThread(LPVOID) {
    HMODULE mm = nullptr;
    for (int i = 0; i < 600 && !mm; i++) {
        mm = GetModuleHandleA("Main_Menu.dll");
        if (!mm) Sleep(100);
    }
    if (!mm) {
        Log("Menu state: Main_Menu.dll never loaded - menu screen unavailable");
        return 0;
    }
    auto base = (uint8_t*)mm;
    // push -1; push <Change_Page SEH handler = base+0x41169>  (imm32 at +3)
    static constexpr uint8_t kChangePage[] = { 0x6A, 0xFF, 0x68, 0, 0, 0, 0 };
    if (!GameAddresses::ValidateCodeAbs<3>("Main_Menu.dll+0x1A7C0 (UI_Menu::Change_Page)",
                                           base + 0x1A7C0, kChangePage, base, 0x41169)) {
        Log("Menu state: Change_Page site did not validate - menu screen unavailable");
        return 0;
    }
    g_changePageHook = safetyhook::create_mid(base + 0x1A7C0, ChangePageCb);
    Log(g_changePageHook
            ? std::format("Menu state: hooked UI_Menu::Change_Page at {:p} (Main_Menu.dll {:p})",
                          (void*)(base + 0x1A7C0), (void*)base)
            : "Menu state: create_mid on Change_Page FAILED");
    return 0;
}

// Spawn the deferred installer. Main_Menu.dll loads with the menu, which may be
// after this DLL initializes, so the hook waits for the module rather than
// assuming it is present (unlike the always-loaded SR_UIT).
inline void Install(GameAddresses& /*addr*/, TasSharedState* state) {
    g_state = state;
    HANDLE t = CreateThread(nullptr, 0, InstallThread, nullptr, 0, nullptr);
    if (t) CloseHandle(t);
    else Log("Menu state: failed to start install thread");
}

}  // namespace menustate
