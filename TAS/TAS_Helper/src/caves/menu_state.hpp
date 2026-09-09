#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../game_addresses.hpp"
#include "../menu_model.hpp"
#include "../rider_identity_parse.hpp"
#include <safetyhook.hpp>
#include <format>
#include "menu_page.hpp"

// Menu state, read from the MENU OBJECTS - and ONLY on the menu thread.
//
// The menus are a UI toolkit (UIT) driven by Main_Menu.dll. Two hooks:
//
//   UI_Menu::Change_Page(name)  RVA 0x1A7C0, __fastcall(ECX = UI_Menu, EDX =
//     the page-name std::string). Records the page id the game is switching to
//     (menu thread). Nothing is published from here: at entry the OLD page is
//     still installed, so a snapshot taken now would pair the new name with
//     the old items.
//   UI_Menu::Execute(float)     RVA 0x1A680, called every frame from Menu::Paint
//     while a menu is shown, ECX = UI_Menu. THE producer: it walks the current
//     page's items (field reads + the game's own Get_Active_Component /
//     Want_Focus), publishes screen + selector + document together under
//     menu_seq, and executes pending agent commands through the game's own
//     entry points. Everything that touches a UI object happens here, on the
//     thread that owns those objects, never on the worker: a worker-side
//     traversal races page teardown, and a vtable inside a UI image proves
//     nothing about liveness.
//
// The level-scan worker only does housekeeping: it clears the document when
// Execute stops heartbeating (a level, a load, the in-game pause menu - none
// of which run UI_Menu::Execute) and expires commands nobody can consume.
// Both writers share a lock, so the odd/even seqlock always has one writer.
namespace menustate {

inline TasSharedState* g_state = nullptr;
inline uint32_t g_mainMenuBase = 0;
inline SafetyHookMid g_changePageHook{};
inline SafetyHookMid g_executeHook{};
inline bool g_cmdInstalled = false;
inline bool g_menuDiag = false;   // TAS_MENU_DIAG=1: log the item list on change
using GetModalFn = uint32_t(__fastcall*)(uint32_t);
inline GetModalFn g_getModal = nullptr;

// Image ranges of the UI modules: a component's vtable must lie in
// Main_Menu.dll or UIT.dll and a text line's in SR_UIT.dll.
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

// An MSVC6 std::string object at `obj` ({allocator, char* ptr, size,
// capacity}). The whole header is read first and bounds-checked WITHOUT
// arithmetic on the length (riderparse::StringHeaderUsable: len < cap and
// capacity >= len): a torn header with size 0xFFFFFFFF passes a
// `len + 1 > cap` test by wrapping and runs off the output buffer.
// Printable ASCII only.
static bool ReadMenuString(uint32_t obj, char* out, uint32_t cap) {
    if (obj < 0x10000 || cap == 0) return false;
    __try {
        uint32_t hdr[4];
        for (int i = 0; i < 4; i++) hdr[i] = *(uint32_t*)(obj + 4 * i);
        const uint32_t ptr = hdr[1], len = hdr[2], capacity = hdr[3];
        if (!riderparse::StringHeaderUsable(ptr, len, capacity, cap)) return false;
        const char* s = (const char*)ptr;
        for (uint32_t i = 0; i < len; i++) {
            const char c = s[i];
            if (c < 0x20 || c > 0x7E) return false;
            out[i] = c;
        }
        out[len] = 0;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

static bool IsIdLike(const char* s) { return s[0] == 'I' && s[1] == 'D' && s[2] == '_'; }

// ---------------------------------------------------------------------------
// PUBLISHING: screen + selector + document go out TOGETHER under menu_seq.
// Two writers exist (the menu thread's snapshot, the worker's clear), so the
// section is serialized by g_pubLock and the seqlock never sees two writers.
// Runs inside the Execute hook: fixed buffers, no allocation, no file log.
// ---------------------------------------------------------------------------
inline SRWLOCK g_pubLock = SRWLOCK_INIT;
inline char g_lastDoc[TAS_MENU_DOC_MAX] = {};   // what shm holds (under g_pubLock)

// `doc` fits TAS_MENU_DOC_MAX including its NUL (BuildDoc's cap).
static void Publish(const char* screen, uint32_t selector, const char* doc) {
    if (!g_state) return;
    AcquireSRWLockExclusive(&g_pubLock);
    const bool changed = strcmp(doc, g_lastDoc) != 0 ||
                         strncmp(g_state->menu_screen, screen, TAS_MENU_SCREEN_MAX) != 0 ||
                         g_state->menu_selector != selector;
    if (changed) {
        InterlockedIncrement((volatile LONG*)&g_state->menu_seq);   // odd: writing
        uint32_t i = 0;
        for (; screen[i] && i < TAS_MENU_SCREEN_MAX - 1; i++) g_state->menu_screen[i] = screen[i];
        g_state->menu_screen[i] = 0;
        g_state->menu_selector = selector;
        const size_t n = strlen(doc) + 1;
        memcpy(g_state->menu_doc, doc, n);
        InterlockedIncrement((volatile LONG*)&g_state->menu_seq);   // even: stable
        memcpy(g_lastDoc, doc, n);
    }
    ReleaseSRWLockExclusive(&g_pubLock);
}

// No menu on screen: nothing published at all.
inline void Clear() { Publish("", 0xFFFFFFFFu, ""); }

// ---------------------------------------------------------------------------
// The MENU MODEL - field reads of the UIT objects (UIT.c / main_menu.c /
// SR_UIT.c), menu thread only:
//   UI_Menu +0x10C = the current Menu_Page.
//   UI_Menu::Get_Active_Component (RVA 0x1A6B0, __fastcall(ECX)) -> the
//     focused UI_Component.
//   UI_Component: +0xC parent, +0x10 std::string name (Get_Name), +0x24
//     enabled, +0x25 visible. UI_Container: children vector +0x2C..+0x30.
//   UIT::Button (Menu_Button / Action_Button): +0x44 Text_Line*; Labels keep
//     theirs at +0x28 and refuse focus.
//   SR_UIT::Sr_Plane_Text_Line: +4 std::string text (Get_Text = this+4).
//   Want_Focus: vtable slot 0x2C - the test Request_Focus itself makes.
// ---------------------------------------------------------------------------
using GetActiveComponent = uint32_t(__fastcall*)(uint32_t);
using WantFocusFn = uint8_t(__fastcall*)(uint32_t);

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
        if (!hasLabel && !IsIdLike(it.name)) return false;
        // ...and only if the cursor can land on it: Want_Focus (a Label
        // overrides it to false; headline labels carry ID_* names too, and
        // activating one would fire Enter on whatever was focused before).
        // The slot must point into the UI images before it is called.
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

// The focused item's container, enumerated. False = nothing capturable now
// (no focused component - a transition - or an unreadable container).
static bool ReadMenu(uint32_t uiMenu, MenuSnapshot& out) {
    out = MenuSnapshot{};
    if (!uiMenu || !g_mainMenuBase) return false;
    uint32_t comp = 0, begin = 0, end = 0;
    __try {
        auto fn = (GetActiveComponent)(uintptr_t)(g_mainMenuBase + 0x1A6B0);
        comp = fn(uiMenu);
        if (comp < 0x10000) return false;
        const uint32_t parent = *(uint32_t*)(comp + 0xC);
        if (parent < 0x10000) return false;
        const uint32_t page = *(uint32_t*)(uiMenu + 0x10C);
        // GetActiveComponent still returns the underlying pause-menu item
        // while an "Are you sure?" modal is on top. Never expose or activate it.
        if (!g_getModal || !menumodel::UnobscuredMenu(page, parent, g_getModal)) return false;
        begin = *(uint32_t*)(parent + 0x2C);
        end = *(uint32_t*)(parent + 0x30);
        if (begin < 0x10000 || end < begin || (end - begin) > 0x1000) return false;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
    out.container = *(uint32_t*)(comp + 0xC);
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

// ---------------------------------------------------------------------------
// The SNAPSHOT (menu thread, from the Execute hook).
// ---------------------------------------------------------------------------
inline volatile uint64_t g_lastExecuteMs = 0;           // heartbeat: a menu is executing
inline uint64_t g_lastSnapMs = 0;
inline uint32_t g_lastDumpHash = 0;

static void ChangePageCb(SafetyHookContext& ctx) {
    char name[TAS_MENU_SCREEN_MAX];
    if (ReadMenuString((uint32_t)ctx.edx, name, sizeof name)) {
        memcpy(g_changedName, name, sizeof name);
        g_pageGen++;   // adopted by the next Execute on this same thread, i.e. after the swap
    }
}

static uint32_t SnapshotHash(const MenuSnapshot& s) {
    uint32_t h = 2166136261u;
    auto mix = [&](const char* p) { for (; *p; p++) h = (h ^ (uint8_t)*p) * 16777619u; };
    mix(g_screen);
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
    char cname[TAS_MENU_SCREEN_MAX] = {};
    ReadMenuString(s.container + 0x10, cname, sizeof cname);
    Log(std::format("Menu diag: screen='{}' selector={} items={} container={:#x} '{}'", g_screen,
                    s.selector == 0xFFFFFFFFu ? -1 : (int)s.selector, s.count, s.container, cname));
    for (uint32_t i = 0; i < s.count; i++) {
        const auto& it = s.items[i];
        Log(std::format("  [{}] {}'{}' name='{}' comp={:#x} en={} vis={}", i, it.focused ? "* " : "",
                        it.label, it.name, it.comp, it.enabled, it.visible));
    }
}

// [UI_Menu+0x10C] = the current Menu_Page (0 during a swap). Its own SEH
// scope: the caller builds std::strings, which cannot share a frame with __try.
static uint32_t ReadPage(uint32_t uiMenu) {
    __try {
        return *(uint32_t*)(uiMenu + 0x10C);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

// Every ~50 ms (or at once after a page swap / a command): read the page and
// publish. The game keeps ONE Menu_Page object and reconfigures it on every
// Change_Page (UI_Menu::Change_Page calls Menu_Page::Change_Page on
// [this+0x10C]; the pointer never moves), so the screen id is keyed on the
// Change_Page EVENT: its name is adopted by the next Execute on the same
// thread, after the swap completed, so screen and items always describe the
// same page. Injected after the last Change_Page? The page's own name (if it
// carries one) bootstraps it; otherwise the id arrives with the next change.

static void Snapshot(uint32_t uiMenu, bool force) {
    const uint64_t now = GetTickCount64();
    if (!force && now - g_lastSnapMs < 50) return;
    g_lastSnapMs = now;
    const uint32_t page = ReadPage(uiMenu);
    if (AdoptPendingPage()) {
        if (g_menuDiag) Log(std::format("Menu diag: page change #{} -> screen '{}' (page {:#x})", g_snapGen, g_screen, page));
    } else if (!g_screen[0] && page) {

        // No Change_Page seen since injection: take the page's own name if it
        // reads as an id (bootstrap), else wait for the next page change.
        char own[TAS_MENU_SCREEN_MAX];
        if (ReadMenuString(page + 0x10, own, sizeof own) && IsIdLike(own)) {
            memcpy(g_screen, own, sizeof own);
            if (g_menuDiag) Log(std::format("Menu diag: bootstrapped screen '{}' from the page object", g_screen));
        }
    }
    if (!page || !g_screen[0]) {
        Clear();
        return;
    }
    MenuSnapshot snap;
    if (!ReadMenu(uiMenu, snap)) {
        // The page is known but its items are not capturable right now (a
        // transition, nothing focused): publish the screen and NO document -
        // never a valid-looking empty page.
        Publish(g_screen, 0xFFFFFFFFu, "");
        return;
    }
    // The items' parent container carries the PAGE ID as its name (verified
    // live: 'ID_ARCADE_CHOOSE_BOARD', 'ID_ARCADE_IN_GAME_MENU', ...) - the very
    // object the items came from, so screen and items agree by construction
    // and no Change_Page is needed to know the page. The announced name stays
    // the fallback for a layout whose items sit in an unnamed sub-container.
    char cname[TAS_MENU_SCREEN_MAX];
    if (ReadMenuString(snap.container + 0x10, cname, sizeof cname) && IsIdLike(cname)) memcpy(g_screen, cname, sizeof cname);
    // Menu thread only, so one static buffer serves every snapshot. A document
    // that does not fit is published as "screen, no document", like a
    // transition.
    static char doc[TAS_MENU_DOC_MAX];
    const uint32_t n = menumodel::BuildDoc(snap, g_screen, doc, sizeof doc);
    Publish(g_screen, n ? snap.selector : 0xFFFFFFFFu, doc);
    DumpIfChanged(snap);
}

// ---------------------------------------------------------------------------
// The COMMAND channel (shm v47/v48): the agent writes kind + target + the page
// id it read, and bumps menu_cmd_seq; the menu thread executes it from the
// Execute hook through the game's own entry points:
//   UIT::UI_Component::Request_Focus(bool)       UIT.dll+0x196D0, ECX=comp, DL=1
//   UI_Menu::Trigger / Up / Down / Left / Right  Main_Menu.dll+0x19F50 / 0x19ED0 /
//     0x19EF0 / 0x19F10 / 0x19F30, ECX=UI_Menu - exactly what KB_Action calls
//     for Enter and the arrow keys (main_menu.c).
// "activate X" = Request_Focus(X, true), verify the focus landed, Trigger.
// A command names the page it was read from and is refused (STALE_PAGE) if
// the page moved on; one command is outstanding at a time (the agent side
// refuses to submit while seq != ack); a command nobody can consume - no
// menu executing - is EXPIRED by the worker after 3 s, so nothing stays armed
// to fire on a later page.
// ---------------------------------------------------------------------------
using MenuAction = void(__fastcall*)(uint32_t);
using RequestFocusFn = void(__fastcall*)(uint32_t, uint32_t);
inline MenuAction g_trigger = nullptr, g_up = nullptr, g_down = nullptr, g_left = nullptr, g_right = nullptr;
inline RequestFocusFn g_requestFocus = nullptr;
inline SRWLOCK g_cmdLock = SRWLOCK_INIT;   // one handler per command: menu thread executes, worker expires
inline uint32_t g_pendingSeq = 0;          // worker: the sequence it is timing
inline uint64_t g_pendingSinceMs = 0;
static constexpr uint64_t kExpireMs = 3000;
static constexpr uint64_t kExecuteIdleMs = 1500;   // no Execute for this long = no menu on screen

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
static uint32_t RunCommand(uint32_t uiMenu, uint32_t kind, const char* target, const char* screen) {
    const uint32_t page = ValidateScreen(screen);
    if (page != TAS_MENU_RESULT_OK) return page;
    MenuSnapshot snap;
    if (!ReadMenu(uiMenu, snap)) return TAS_MENU_RESULT_NO_MENU;
    switch (kind) {
    case TAS_MENU_CMD_UP: g_up(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_DOWN: g_down(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_LEFT: g_left(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_RIGHT: g_right(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_TRIGGER: g_trigger(uiMenu); return TAS_MENU_RESULT_OK;
    case TAS_MENU_CMD_ACTIVATE:
    case TAS_MENU_CMD_FOCUS: {
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
static uint32_t RunCommandGuarded(uint32_t uiMenu, uint32_t kind, const char* target, const char* screen) {
    __try {
        return RunCommand(uiMenu, kind, target, screen);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return TAS_MENU_RESULT_FAULT;
    }
}

static void Answer(uint32_t seq, uint32_t result) {
    g_state->menu_cmd_result = result;
    InterlockedExchange((volatile LONG*)&g_state->menu_cmd_ack, (LONG)seq);   // result is visible first
}

// Menu thread: execute a pending command, then re-snapshot at once.
static bool ConsumeCommand(uint32_t uiMenu) {
    const uint32_t seq = g_state->menu_cmd_seq;
    if (seq == g_state->menu_cmd_ack) return false;   // nothing pending: one compare per frame
    AcquireSRWLockExclusive(&g_cmdLock);
    bool ran = false;
    if (seq != g_state->menu_cmd_ack) {   // the worker did not expire it meanwhile
        char target[TAS_MENU_CMD_TARGET_MAX], screen[TAS_MENU_SCREEN_MAX];
        for (uint32_t i = 0; i < TAS_MENU_CMD_TARGET_MAX; i++) target[i] = g_state->menu_cmd_target[i];
        for (uint32_t i = 0; i < TAS_MENU_SCREEN_MAX; i++) screen[i] = g_state->menu_cmd_screen[i];
        target[TAS_MENU_CMD_TARGET_MAX - 1] = 0;
        screen[TAS_MENU_SCREEN_MAX - 1] = 0;
        const uint32_t kind = g_state->menu_cmd_kind;
        // Validate against the page the agent could have seen, not the one
        // cached before the last Change_Page: adopt first, bypassing the
        // 50 ms snapshot throttle for this command.
        AdoptPendingPage();
        const uint32_t result = RunCommandGuarded(uiMenu, kind, target, screen);
        Answer(seq, result);
        // Ring log, not the file log: this runs inside the Execute hook.
        char msg[TAS_LOG_ENTRY_SIZE];
        menumodel::TextWriter w{msg, sizeof msg};
        w.Put("Menu cmd #");
        w.PutU32(seq);
        w.Put(": kind=");
        w.PutU32(kind);
        w.Put(" target='");
        w.Put(target);
        w.Put("' page='");
        w.Put(screen);
        w.Put("' -> ");
        w.PutU32(result);
        LogRing(g_state, LOG_INFO, w.Finish());
        ran = true;
    }
    ReleaseSRWLockExclusive(&g_cmdLock);
    return ran;
}

static void ExecuteCb(SafetyHookContext& ctx) {
    if (!g_state) return;
    const uint32_t uiMenu = (uint32_t)ctx.ecx;
    g_lastExecuteMs = GetTickCount64();
    const bool ran = ConsumeCommand(uiMenu);
    Snapshot(uiMenu, ran);
}

// Worker thread (level scan, ~10 Hz): housekeeping only - no UI object is
// touched here. Clears the document once Execute stops (a level, a load, the
// in-game pause menu), and answers EXPIRED for a command nobody can consume,
// so it never lingers to fire on a later page.
inline void Housekeeping() {
    if (!g_state) return;
    const uint64_t now = GetTickCount64();
    const uint64_t last = g_lastExecuteMs;
    const bool executing = last && now - last < kExecuteIdleMs;
    if (!executing && g_state->menu_screen[0]) Clear();
    const uint32_t seq = g_state->menu_cmd_seq;
    if (seq == g_state->menu_cmd_ack) {
        g_pendingSeq = 0;
        return;
    }
    if (g_pendingSeq != seq) {
        g_pendingSeq = seq;
        g_pendingSinceMs = now;
        return;
    }
    if (executing || now - g_pendingSinceMs < kExpireMs) return;
    AcquireSRWLockExclusive(&g_cmdLock);
    if (seq != g_state->menu_cmd_ack) {
        Answer(seq, TAS_MENU_RESULT_EXPIRED);
        Log(std::format("Menu cmd #{}: expired - no menu executing for {} ms", seq, now - last));
    }
    ReleaseSRWLockExclusive(&g_cmdLock);
}

// Game thread, in a level (race timer tick): the menu is definitely gone.
inline void ClearForLevel() {
    if (g_state && g_state->menu_screen[0]) Clear();
}

static void InstallCommands(uint8_t* base) {
    auto uit = (uint8_t*)GetModuleHandleA("UIT.dll");
    if (!uit) {
        Log("Menu cmd: UIT.dll not loaded - commands unavailable");
        return;
    }
    using GA = GameAddresses;
    const auto getModal = GetProcAddress((HMODULE)uit,
        "?Get_Modal@UI_Container@UIT@Housemarque@@QBIPAV123@XZ");
    if (!getModal) {
        Log("Menu cmd: Get_Modal unavailable - refusing to expose obscured menus");
        return;
    }
    const bool ok =
        GA::ValidateCode("Main_Menu.dll+0x1A680 (UI_Menu::Execute)", base + 0x1A680, kExecuteSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F50 (UI_Menu::Trigger)", base + 0x19F50, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19ED0 (UI_Menu::Up)", base + 0x19ED0, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19EF0 (UI_Menu::Down)", base + 0x19EF0, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F10 (UI_Menu::Left)", base + 0x19F10, kMenuActionSig) &&
        GA::ValidateCode("Main_Menu.dll+0x19F30 (UI_Menu::Right)", base + 0x19F30, kMenuActionSig) &&
        GA::ValidateCode("UIT.dll+0x196D0 (UI_Component::Request_Focus)", uit + 0x196D0, kRequestFocusSig);
    if (!ok) {
        Log("Menu cmd: a site did not validate - the menu document and commands are unavailable");
        return;
    }
    g_trigger = (MenuAction)(uintptr_t)(base + 0x19F50);
    g_up = (MenuAction)(uintptr_t)(base + 0x19ED0);
    g_down = (MenuAction)(uintptr_t)(base + 0x19EF0);
    g_left = (MenuAction)(uintptr_t)(base + 0x19F10);
    g_right = (MenuAction)(uintptr_t)(base + 0x19F30);
    g_requestFocus = (RequestFocusFn)(uintptr_t)(uit + 0x196D0);
    g_getModal = (GetModalFn)getModal;
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
