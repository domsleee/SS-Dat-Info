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
inline uint32_t g_mainMenuBase = 0;   // Main_Menu.dll base, for calling Get_Active_Component

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

// The SELECTOR: the index of the focused menu item within its container.
//
// UI_Menu::Get_Active_Component (RVA 0x1A6B0, __fastcall(ECX = UI_Menu)) walks
// the page hierarchy to the focused UI_Component. Then, by FIELD reads (UIT.c):
// the component's parent is [comp+0xC], and the parent container holds its
// children as a vector [parent+0x2C .. parent+0x30] of UI_Component* - the
// focused component's position in it is the selector index.
//
// Get_Active_Component does an indirect call, so it must run against a live
// UI_Menu. We only call it while a menu screen is published (menu_screen set;
// cleared in a level), and under SEH: worst case a page torn down mid-call
// faults and we publish "no selection" for a poll. Called from the level-scan
// worker (~10 Hz), never from the game thread.
static uint32_t ReadSelectorIndex() {
    if (!g_uiMenu || !g_mainMenuBase || !g_state || !g_state->menu_screen[0]) return 0xFFFFFFFFu;
    __try {
        using GetActiveComponent = uint32_t(__fastcall*)(uint32_t);
        auto fn = (GetActiveComponent)(uintptr_t)(g_mainMenuBase + 0x1A6B0);
        const uint32_t comp = fn(g_uiMenu);
        if (comp < 0x10000) return 0xFFFFFFFFu;
        const uint32_t parent = *(uint32_t*)(comp + 0xC);
        if (parent < 0x10000) return 0xFFFFFFFFu;
        const uint32_t begin = *(uint32_t*)(parent + 0x2C);
        const uint32_t end = *(uint32_t*)(parent + 0x30);
        if (begin < 0x10000 || end < begin || (end - begin) > 0x1000) return 0xFFFFFFFFu;
        const uint32_t count = (end - begin) / 4;
        // The container's children mix menu items with labels/decorations, so a
        // raw position jumps around. Count only PEERS of the focused item - the
        // children with the same vtable (widget class) - to get the clean visual
        // ordinal (0 = first item of this kind). Field reads only.
        const uint32_t compVtable = *(uint32_t*)comp;
        uint32_t ordinal = 0;
        for (uint32_t i = 0; i < count; i++) {
            const uint32_t child = *(uint32_t*)(begin + i * 4);
            if (child == comp) return ordinal;
            if (child >= 0x10000 && *(uint32_t*)child == compVtable) ordinal++;
        }
        return 0xFFFFFFFFu;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0xFFFFFFFFu;
    }
}

// Publish the current selector index (0xFFFFFFFF = no menu / unreadable).
// Call from the level-scan worker.
inline void RefreshSelector() {
    if (!g_state) return;
    const uint32_t idx = ReadSelectorIndex();
    if (g_state->menu_selector != idx) g_state->menu_selector = idx;
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
    g_mainMenuBase = (uint32_t)(uintptr_t)base;
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
