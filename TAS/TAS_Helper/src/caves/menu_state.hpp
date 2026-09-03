#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../menu_model.hpp"
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

using menumodel::MenuItem;
using menumodel::MenuSnapshot;
using menumodel::kMaxItems;

// A Button-family component's label: [comp+0x44] -> Sr_Plane_Text_Line -> +4 string.
// The text line's vtable must be SR_UIT's; a Label (text line at +0x28, +0x44
// off the end of the object) fails this and is not an item.
static bool ReadItem(uint32_t child, uint32_t focusedComp, MenuItem& it) {
    __try {
        if (child < 0x10000) return false;
        const uint32_t vt = *(uint32_t*)child;
        if (!g_imgMainMenu.Has(vt) && !g_imgUit.Has(vt)) return false;   // not a UIT component
        // The label: a Button-family component's text line. Image buttons (the
        // page arrows) have none, and a non-Button's +0x44 is off its end, so
        // the line is trusted only if its vtable is SR_UIT's.
        const uint32_t tl = *(uint32_t*)(child + 0x44);
        const bool hasLabel = tl >= 0x10000 && g_imgSrUit.Has(*(uint32_t*)tl) &&
                              ReadMenuString(tl + 4, it.label, sizeof it.label);
        if (!hasLabel) it.label[0] = 0;
        if (!ReadMenuString(child + 0x10, it.name, sizeof it.name)) it.name[0] = 0;
        // An item is a control with visible text, or an id-bearing control
        // without any (the image arrows carry an ID_* name and no text line).
        // Labels, showers and decorations have neither and are skipped.
        const bool hasId = it.name[0] == 'I' && it.name[1] == 'D' && it.name[2] == '_';
        if (!hasLabel && !hasId) return false;
        // Only a control the cursor can land on is an item: Want_Focus (vtable
        // slot 0x2C, the one Request_Focus itself consults; a Label overrides
        // it to false). Headline labels carry ID_* names too, and activating
        // one would fire Enter on whatever was focused before. The slot must
        // point into the UI images before it is called.
        using WantFocusFn = uint8_t(__fastcall*)(uint32_t);
        const uint32_t wantFocus = *(uint32_t*)(vt + 0x2C);
        if (!g_imgMainMenu.Has(wantFocus) && !g_imgUit.Has(wantFocus)) return false;
        if (!(((WantFocusFn)(uintptr_t)wantFocus)(child) & 1)) return false;
        it.comp = child;
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
static std::string BuildDoc(const MenuSnapshot& s) {
    return g_state ? menumodel::BuildDoc(s, g_state->menu_screen) : std::string();
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

// ---------------------------------------------------------------------------
// The COMMAND channel (shm v47): the agent writes kind + target and bumps
// menu_cmd_seq; this side executes it on the MENU THREAD - a mid-hook at the
// entry of UI_Menu::Execute(float), called every frame from Menu::Paint while
// a menu is shown with ECX = the UI_Menu - through the game's own entry points:
//   UIT::UI_Component::Request_Focus(bool)       UIT.dll+0x196D0, ECX=comp, DL=1
//   UI_Menu::Trigger / Up / Down / Left / Right  Main_Menu.dll+0x19F50 / 0x19ED0 /
//     0x19EF0 / 0x19F10 / 0x19F30, ECX=UI_Menu - exactly what KB_Action calls
//     for Enter and the arrow keys (main_menu.c).
// "activate X" = Request_Focus(X, true) then Trigger: the Enter path on the
// target, so listeners, sounds and page transitions run as if a human did it.
// Nothing is poked into the game's objects by hand.
// ---------------------------------------------------------------------------
using MenuAction = void(__fastcall*)(uint32_t);
using RequestFocusFn = void(__fastcall*)(uint32_t, uint32_t);
inline MenuAction g_trigger = nullptr, g_up = nullptr, g_down = nullptr, g_left = nullptr, g_right = nullptr;
inline RequestFocusFn g_requestFocus = nullptr;
inline SafetyHookMid g_executeHook{};
inline bool g_cmdInstalled = false;

// UI_Menu::Execute: push esi; push edi; mov edi,[esp+0xC]; push edi; mov esi,ecx
static constexpr uint8_t kExecuteSig[] = { 0x56, 0x57, 0x8B, 0x7C, 0x24, 0x0C, 0x57, 0x8B, 0xF1 };
// UI_Menu::Trigger/Up/Down/Left/Right all open identically: mov eax,[ecx+0x10C]
// (the page); test eax,eax; jz; lea edx,[eax+4] (its listener) / xor edx,edx;
// add ecx,0x18 (the Input_Event_Generator) - then a jmp through an import
// thunk, an absolute address the pattern stops before.
static constexpr uint8_t kMenuActionSig[] = { 0x8B, 0x81, 0x0C, 0x01, 0x00, 0x00, 0x85, 0xC0, 0x74, 0x05,
                                              0x8D, 0x50, 0x04, 0xEB, 0x02, 0x33, 0xD2, 0x83, 0xC1, 0x18 };
// UI_Component::Request_Focus: push ebx; push esi; mov esi,ecx; mov ecx,[esi+0xC]
// (the parent); test ecx,ecx; mov ebx,edx; jz; test bl,bl; jz; mov eax,[esi];
// mov ecx,esi; call [eax+0x2C] (Want_Focus)
static constexpr uint8_t kRequestFocusSig[] = { 0x53, 0x56, 0x8B, 0xF1, 0x8B, 0x4E, 0x0C, 0x85, 0xC9, 0x8B, 0xDA, 0x74, 0x24,
                                                0x84, 0xDB, 0x74, 0x18, 0x8B, 0x06, 0x8B, 0xCE, 0xFF, 0x50, 0x2C };

static uint32_t RunCommand(uint32_t uiMenu, uint32_t kind, const char* target) {
    if (!g_state->menu_screen[0]) return TAS_MENU_RESULT_NO_MENU;
    switch (kind) {
    case TAS_MENU_CMD_UP: g_up(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_DOWN: g_down(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_LEFT: g_left(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_RIGHT: g_right(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_TRIGGER: g_trigger(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_ACTIVATE:
    case TAS_MENU_CMD_FOCUS: {
        MenuSnapshot snap;
        if (!ReadMenu(snap)) return TAS_MENU_RESULT_NO_MENU;
        const int i = menumodel::FindTarget(snap, target);
        if (i < 0) return TAS_MENU_RESULT_NOT_FOUND;
        if (!snap.items[i].enabled) return TAS_MENU_RESULT_DISABLED;
        g_requestFocus(snap.items[i].comp, 1);
        // Never press Enter blind: if the focus did not land on the target
        // (a control that refuses it), Trigger would fire on the PREVIOUS item.
        auto active = (GetActiveComponent)(uintptr_t)(g_mainMenuBase + 0x1A6B0);
        if (active(uiMenu) != snap.items[i].comp) return TAS_MENU_RESULT_NOT_FOCUSABLE;
        if (kind == TAS_MENU_CMD_ACTIVATE) g_trigger(uiMenu);
        return TAS_MENU_RESULT_OK;
    }
    default:
        return TAS_MENU_RESULT_BAD_KIND;
    }
}

// SEH around the whole execution: a page torn down between the agent's read
// and this frame faults here, and the agent gets FAULT instead of a crash.
static uint32_t RunCommandGuarded(uint32_t uiMenu, uint32_t kind, const char* target) {
    __try {
        return RunCommand(uiMenu, kind, target);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return TAS_MENU_RESULT_FAULT;
    }
}

static void ExecuteCb(SafetyHookContext& ctx) {
    if (!g_state) return;
    const uint32_t seq = g_state->menu_cmd_seq;
    if (seq == g_state->menu_cmd_ack) return;   // nothing pending: one compare per frame
    g_uiMenu = (uint32_t)ctx.ecx;               // the live UI_Menu
    char target[TAS_MENU_CMD_TARGET_MAX];
    for (uint32_t i = 0; i < TAS_MENU_CMD_TARGET_MAX; i++) target[i] = g_state->menu_cmd_target[i];
    target[TAS_MENU_CMD_TARGET_MAX - 1] = 0;
    const uint32_t kind = g_state->menu_cmd_kind;
    const uint32_t result = RunCommandGuarded((uint32_t)ctx.ecx, kind, target);
    g_state->menu_cmd_result = result;
    InterlockedExchange((volatile LONG*)&g_state->menu_cmd_ack, (LONG)seq);   // result is visible first
    Log(std::format("Menu cmd #{}: kind={} target='{}' -> {}", seq, kind, target, result));
}

static void InstallCommands(uint8_t* base) {
    auto uit = (uint8_t*)GetModuleHandleA("UIT.dll");
    if (!uit) {
        Log("Menu cmd: UIT.dll not loaded - commands unavailable");
        return;
    }
    using GA = GameAddresses;
    const bool ok =
        GA::ValidateCode("Main_Menu.dll+0x1A680 (UI_Menu::Execute)", base + 0x1A680, kExecuteSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F50 (UI_Menu::Trigger)", base + 0x19F50, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19ED0 (UI_Menu::Up)", base + 0x19ED0, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19EF0 (UI_Menu::Down)", base + 0x19EF0, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F10 (UI_Menu::Left)", base + 0x19F10, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F30 (UI_Menu::Right)", base + 0x19F30, kMenuActionSig) &&
        GA::ValidateCode("UIT.dll+0x196D0 (UI_Component::Request_Focus)", uit + 0x196D0, kRequestFocusSig);
    if (!ok) {
        Log("Menu cmd: a site did not validate - commands unavailable");
        return;
    }
    g_trigger = (MenuAction)(uintptr_t)(base + 0x19F50);
    g_up = (MenuAction)(uintptr_t)(base + 0x19ED0);
    g_down = (MenuAction)(uintptr_t)(base + 0x19EF0);
    g_left = (MenuAction)(uintptr_t)(base + 0x19F10);
    g_right = (MenuAction)(uintptr_t)(base + 0x19F30);
    g_requestFocus = (RequestFocusFn)(uintptr_t)(uit + 0x196D0);
    g_executeHook = safetyhook::create_mid(base + 0x1A680, ExecuteCb);
    g_cmdInstalled = (bool)g_executeHook;
    Log(g_cmdInstalled
            ? std::format("Menu cmd: hooked UI_Menu::Execute at {:p} (Request_Focus {:p})",
                          (void*)(base + 0x1A680), (void*)(uit + 0x196D0))
            : "Menu cmd: create_mid on Execute FAILED");
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
    if (g_changePageHook) InstallCommands(base);
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
