#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"
#include <format>
#include <string>

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

// ---------------------------------------------------------------------------
// The MENU MODEL: the focused item plus every item on the current page, with
// its label - read by FIELD from the UIT objects (UIT.c / main_menu.c / SR_UIT.c):
//
//   UI_Menu::Get_Active_Component (RVA 0x1A6B0, __fastcall(ECX = UI_Menu)) ->
//     the focused UI_Component.
//   UI_Component: +0xC parent, +0x10 std::string name (Get_Name), +0x24 enabled,
//     +0x25 visible, +0x27 focused.  UI_Container: children vector at
//     +0x2C..+0x30 of UI_Component*.
//   UIT::Button (the menu items are Main_Menu::Menu_Button / Action_Button
//     subclasses): +0x44 Text_Line* (Set_Text_Line).  Labels keep theirs at
//     +0x28 and are not focusable (Label::Want_Focus = false) - not items.
//   SR_UIT::Sr_Plane_Text_Line: +4 std::string text (Get_Text returns this+4).
//
// An MSVC6 std::string object is {allocator, char* ptr, size, capacity}.
//
// Get_Active_Component does an indirect call, so it must run against a live
// UI_Menu: only while a menu screen is published (cleared in a level), under
// SEH - worst case a page torn down mid-call faults and this poll publishes
// nothing. Called from the level-scan worker (~10 Hz), never the game thread.
// ---------------------------------------------------------------------------

// Image ranges of the UI modules: a component's vtable must lie in
// Main_Menu.dll or UIT.dll and a text line's in SR_UIT.dll. That is the class
// test - by field, no virtual calls on children.
struct ImageRange {
    uint32_t lo = 0, hi = 0;
    bool Has(uint32_t p) const { return lo && p >= lo && p < hi; }
};
inline ImageRange g_imgMainMenu, g_imgUit, g_imgSrUit;

static ImageRange ImageRangeOf(const char* mod) {
    ImageRange r{};
    HMODULE h = GetModuleHandleA(mod);
    if (!h) return r;
    auto dos = (IMAGE_DOS_HEADER*)h;
    auto nt = (IMAGE_NT_HEADERS32*)((uint8_t*)h + dos->e_lfanew);
    r.lo = (uint32_t)(uintptr_t)h;
    r.hi = r.lo + nt->OptionalHeader.SizeOfImage;
    return r;
}

static constexpr uint32_t kMaxItems = 24;
static constexpr uint32_t kNameMax = 32;
static constexpr uint32_t kLabelMax = 40;

struct MenuItem {
    uint32_t comp = 0;              // the UI_Component (for the write side later)
    char name[kNameMax] = {};       // UI_Component name (+0x10)
    char label[kLabelMax] = {};     // the button's text line text
    uint8_t enabled = 0, visible = 0, focused = 0;
};

struct MenuSnapshot {
    uint32_t selector = 0xFFFFFFFFu;  // index into items of the focused one
    uint32_t count = 0;
    MenuItem items[kMaxItems];
};

// A Button-family component's label: [comp+0x44] -> Sr_Plane_Text_Line -> +4 string.
// The text line's vtable must be SR_UIT's; a Label (text line at +0x28, +0x44
// off the end of the object) fails this and is not an item.
static bool ReadItem(uint32_t child, uint32_t focusedComp, MenuItem& it) {
    __try {
        if (child < 0x10000) return false;
        const uint32_t vt = *(uint32_t*)child;
        if (!g_imgMainMenu.Has(vt) && !g_imgUit.Has(vt)) return false;   // not a UIT component
        const uint32_t tl = *(uint32_t*)(child + 0x44);
        if (tl < 0x10000 || !g_imgSrUit.Has(*(uint32_t*)tl)) return false;  // no text line: not a button
        if (!ReadMenuString(tl + 4, it.label, sizeof it.label)) return false;
        it.comp = child;
        if (!ReadMenuString(child + 0x10, it.name, sizeof it.name)) it.name[0] = 0;
        it.enabled = *(uint8_t*)(child + 0x24);
        it.visible = *(uint8_t*)(child + 0x25);
        it.focused = (child == focusedComp);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

using GetActiveComponent = uint32_t(__fastcall*)(uint32_t);

// One Get_Active_Component call, then field reads of the focused item's
// container. Returns false (empty snapshot) when there is no live menu.
static bool ReadMenu(MenuSnapshot& out) {
    out = MenuSnapshot{};
    if (!g_uiMenu || !g_mainMenuBase || !g_state || !g_state->menu_screen[0]) return false;
    uint32_t comp = 0, begin = 0, end = 0;
    __try {
        auto fn = (GetActiveComponent)(uintptr_t)(g_mainMenuBase + 0x1A6B0);
        comp = fn(g_uiMenu);
        if (comp < 0x10000) return false;
        const uint32_t parent = *(uint32_t*)(comp + 0xC);
        if (parent < 0x10000) return false;
        begin = *(uint32_t*)(parent + 0x2C);
        end = *(uint32_t*)(parent + 0x30);
        if (begin < 0x10000 || end < begin || (end - begin) > 0x1000) return false;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    const uint32_t n = (end - begin) / 4;
    for (uint32_t i = 0; i < n && out.count < kMaxItems; i++) {
        uint32_t child = 0;
        __try { child = *(uint32_t*)(begin + i * 4); } __except (EXCEPTION_EXECUTE_HANDLER) { break; }
        MenuItem it;
        if (!ReadItem(child, comp, it)) continue;
        if (it.focused) out.selector = out.count;
        out.items[out.count++] = it;
    }
    return true;
}

// Diagnostic: TAS_MENU_DIAG=1 logs the item list whenever it changes.
inline bool g_menuDiag = false;
inline uint32_t g_lastDumpHash = 0;

static uint32_t SnapshotHash(const MenuSnapshot& s) {
    uint32_t h = 2166136261u;
    auto mix = [&](const char* p) { for (; *p; p++) h = (h ^ (uint8_t)*p) * 16777619u; };
    mix(g_state ? g_state->menu_screen : "");
    h = (h ^ s.selector) * 16777619u;
    for (uint32_t i = 0; i < s.count; i++) {
        mix(s.items[i].name);
        mix(s.items[i].label);
        h = (h ^ s.items[i].enabled) * 16777619u;
    }
    return h;
}

static void DumpIfChanged(const MenuSnapshot& s) {
    if (!g_menuDiag) return;
    const uint32_t h = SnapshotHash(s);
    if (h == g_lastDumpHash) return;
    g_lastDumpHash = h;
    Log(std::format("Menu diag: screen='{}' selector={} items={}", g_state->menu_screen,
                    s.selector == 0xFFFFFFFFu ? -1 : (int)s.selector, s.count));
    for (uint32_t i = 0; i < s.count; i++) {
        const auto& it = s.items[i];
        Log(std::format("  [{}] {}'{}' name='{}' comp={:#x} en={} vis={}", i, it.focused ? "* " : "",
                        it.label, it.name, it.comp, it.enabled, it.visible));
    }
}

// ---- the document (shm v46) ----
// Labels and names are printable ASCII (ReadMenuString enforces it), so only
// the two JSON metacharacters need escaping.
static void AppendJsonString(std::string& out, const char* s) {
    out += '"';
    for (; *s; s++) {
        if (*s == '"' || *s == '\\') out += '\\';
        out += *s;
    }
    out += '"';
}

// {"screen":..,"sel":N|null,"items":[{"label":..,"id":..,"en":b,"vis":b},..]}
// Empty string = no menu.
static std::string BuildDoc(const MenuSnapshot& s) {
    if (!g_state || !g_state->menu_screen[0]) return {};
    std::string d;
    d.reserve(512);
    d += "{\"screen\":";
    AppendJsonString(d, g_state->menu_screen);
    d += ",\"sel\":";
    d += (s.selector == 0xFFFFFFFFu) ? std::string("null") : std::to_string(s.selector);
    d += ",\"items\":[";
    for (uint32_t i = 0; i < s.count; i++) {
        const auto& it = s.items[i];
        if (i) d += ',';
        d += "{\"label\":";
        AppendJsonString(d, it.label);
        d += ",\"id\":";
        AppendJsonString(d, it.name);
        d += ",\"en\":";
        d += it.enabled ? "true" : "false";
        d += ",\"vis\":";
        d += it.visible ? "true" : "false";
        d += '}';
    }
    d += "]}";
    return d;
}

inline std::string g_lastDoc;   // what shm holds (worker thread only)

// Publish under menu_seq, only on change. The bound cannot be hit (24 items
// of bounded strings stay well under the buffer) but is guarded, not assumed.
static void PublishDoc(const std::string& doc) {
    if (!g_state || doc == g_lastDoc) return;
    if (doc.size() + 1 > TAS_MENU_DOC_MAX) {
        Log(std::format("Menu state: document too large ({} bytes) - not published", doc.size()));
        return;
    }
    InterlockedIncrement((volatile LONG*)&g_state->menu_seq);   // odd: writing
    memcpy(g_state->menu_doc, doc.c_str(), doc.size() + 1);
    InterlockedIncrement((volatile LONG*)&g_state->menu_seq);   // even: stable
    g_lastDoc = doc;
}

// Publish the current menu model (selector + document). Call from the
// level-scan worker.
inline void RefreshMenu() {
    if (!g_state) return;
    MenuSnapshot snap;
    ReadMenu(snap);
    if (g_state->menu_selector != snap.selector) g_state->menu_selector = snap.selector;
    PublishDoc(BuildDoc(snap));
    DumpIfChanged(snap);
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
    g_imgMainMenu = ImageRangeOf("Main_Menu.dll");
    g_imgUit = ImageRangeOf("UIT.dll");
    g_imgSrUit = ImageRangeOf("SR_UIT.dll");
    char diag[8] = {};
    g_menuDiag = GetEnvironmentVariableA("TAS_MENU_DIAG", diag, sizeof diag) && diag[0] == '1';
    Log(std::format("Menu state: images Main_Menu {:#x}-{:#x} UIT {:#x}-{:#x} SR_UIT {:#x}-{:#x}{}",
                    g_imgMainMenu.lo, g_imgMainMenu.hi, g_imgUit.lo, g_imgUit.hi, g_imgSrUit.lo, g_imgSrUit.hi,
                    g_menuDiag ? " (diag on)" : ""));
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
