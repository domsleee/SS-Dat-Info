#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include <safetyhook.hpp>
#include "../shared_state.hpp"
#include "../race_timer_table.hpp"
#include "menu_state.hpp"

// ============================================================================
// Race timer — reads the EXACT on-screen player race time, map-agnostically.
//
// start_ts is not a plain stored value (the displayed time is computed each
// frame as clock - start_ts and formatted). So instead of hunting it, we read
// the authoritative value the game already produced: the HUD time string.
//
// HUD times render through SR_UIT.dll (HMG UI-text, exported symbols):
//   Housemarque::SR_UIT::Sr_Plane_Text_Line::Append_Text   (SR_UIT + 0xED40)
//   fastcall: ecx = the text-LINE object, edx = textObj (+0x04 char* data,
//             +0x08 int length).
// SafetyHook (all threads) intercepts every time-like ("MM:SS:CC") append,
// keyed by the LINE object, and classifies/publishes RIGHT THERE (on the same
// fresh sample) - the table logic lives in race_timer_table.hpp (pure,
// unit-tested): the PLAYER line is the one whose parsed cs ADVANCES, it is
// LATCHED once locked, and lines that stop being sampled are evicted (a menu
// trip rebuilds the HUD without dropping game_in_game, so dead lines cannot
// be cleared on that flag).
// We publish:
//   race_time_cs  = exact on-screen race time, centiseconds (u32::MAX = idle)
//   race_start_ts = 16-bit gate-cross clock value (F5 spawn-lottery metric)
//   clock         = SG + 0x1D5334 (16-bit centiseconds, wraps at 65536)
//
// DIAGNOSTICS: set the env var TAS_RACE_DIAG=1 before launch to log every
// blank<->show transition, slot claims and a periodic line dump to
// TAS_Helper.log.
// ============================================================================

namespace racetimer {

inline uint8_t* g_clock = nullptr;
inline TasSharedState* g_state = nullptr;
inline Table g_table{};
inline uint32_t g_tickNow = 0;        // monotonic clock ticks since install (staleness)
inline int      g_wasInGame = -1;     // game_in_game edge tracker for epoch reset

// Diagnostics (TAS_RACE_DIAG=1)
inline bool     g_diag = false;
inline uint32_t g_diagTick = 0;
inline uint32_t g_lastPub = MAXU;     // last published cs (MAXU = blanked)
inline int      g_lastUsed = 0;

static SafetyHookMid g_tickHook{};
static SafetyHookMid g_aptHook{};

static inline uint16_t ReadClk() {
    return g_clock ? *(volatile uint16_t*)g_clock : 0;
}

// Publish, recording blank<->show transitions for diagnosis.
static void Publish(uint32_t cs, uint32_t start, const char* reason) {
    if (!g_state) return;
    // The pair goes out under race_seq so a reader never sees a new time with
    // the previous start stamp. Game thread only.
    if (g_state->race_time_cs != cs || g_state->race_start_ts != start) {
        InterlockedIncrement((volatile LONG*)&g_state->race_seq);   // odd: writing
        g_state->race_time_cs = cs;
        g_state->race_start_ts = start;
        InterlockedIncrement((volatile LONG*)&g_state->race_seq);   // even: stable
    }
    if (g_diag && (g_lastPub == MAXU) != (cs == MAXU)) {
        Log(std::format("[racetimer] {} ({}) clk={} cs={} start={}",
            cs == MAXU ? "BLANK" : "SHOW", reason, (int)ReadClk(),
            cs == MAXU ? -1 : (int)cs, start == MAXU ? -1 : (int)start));
    }
    g_lastPub = cs;
}

static void ResetEpoch() {
    g_table.Reset();
    Publish(MAXU, MAXU, "epoch-reset");
}

// "MM:SS:CC" -> centiseconds, or -1 if not a valid time. Tight: exact 8-char
// shape, digit-only, seconds < 60, centis < 100 (rejects scores / clock-of-day).
static int ParseCs(const char* s, int len) {
    if (len < 8 || s[2] != ':' || s[5] != ':') return -1;
    for (int i : {0, 1, 3, 4, 6, 7})
        if (s[i] < '0' || s[i] > '9') return -1;
    int mm = (s[0] - '0') * 10 + (s[1] - '0');
    int ss = (s[3] - '0') * 10 + (s[4] - '0');
    int cc = (s[6] - '0') * 10 + (s[7] - '0');
    if (ss >= 60 || cc >= 100) return -1;
    return mm * 6000 + ss * 100 + cc;
}

// SEH-only: read the appended text (textObj+0x04 = char*, +0x08 = int len).
static bool ReadText(uint32_t textObj, char out[24], int& len) {
    out[0] = 0; len = 0;
    __try {
        int l = *(int*)(textObj + 8);
        char* d = *(char**)(textObj + 4);
        if (l < 1 || l > 22 || d == nullptr) return false;
        for (int i = 0; i < l; i++) out[i] = d[i];
        out[l] = 0; len = l;
        return true;
    } __except(EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// Hook of SR_UIT Append_Text: classify on each fresh time-like sample.
static void AptCb(SafetyHookContext& ctx) {
    char buf[24]; int len;
    if (!ReadText((uint32_t)ctx.edx, buf, len)) return;

    int cs = ParseCs(buf, len);
    if (cs < 0) return;
    Verdict v = g_table.Sample((uint32_t)ctx.ecx, cs, ReadClk(), g_tickNow);
    if (g_diag) {
        int used = g_table.Used();
        if (used != g_lastUsed) {
            Log(std::format("[racetimer] table now holds {} line(s) (was {})", used, g_lastUsed));
            g_lastUsed = used;
        }
    }
    Publish(v.cs, v.start, v.reason);
}

// Once per clock tick: staleness clock, epoch reset on a game_in_game 1 -> 0
// edge (a level torn down while the clock still ticks) + periodic diag.
static void TickCb(SafetyHookContext&) {
    if (!g_state) return;
    g_tickNow++;
    // A ticking clock means a level is running and no menu is on screen (the
    // engine cycle - and this clock tick with it - is frozen at menus).
    if (g_state->game_in_game) menustate::ClearForLevel();
    int inGame = g_state->game_in_game ? 1 : 0;
    if (inGame == 0) {
        if (g_wasInGame != 0) ResetEpoch();
        else Publish(MAXU, MAXU, "menu");
    }
    g_wasInGame = inGame;

    // Staleness is tick-driven, not sample-driven: the HUD line of a finished
    // race keeps its frozen time published for as long as the line is still
    // appended, but once the HUD is torn down nothing samples any more - and
    // game_in_game does NOT drop at the menu, so without this the last time
    // would stay published forever. Eight compares per tick; eviction
    // unlatches the player line, which blanks the feed.
    if (inGame && g_table.Evict(g_tickNow) > 0 && g_table.playerLine == 0 && g_lastPub != MAXU) {
        Publish(MAXU, MAXU, "stale");
    }

    if (g_diag && inGame && (++g_diagTick % 128) == 0) {
        int clk = ReadClk();
        std::string s = std::format("[racetimer] clk={} tick={} player={:#x} pub={}",
            clk, g_tickNow, g_table.playerLine, g_lastPub == MAXU ? -1 : (int)g_lastPub);
        for (int i = 0; i < SLOTS; i++) {
            if (g_table.line[i] == 0) continue;
            s += std::format(" | L{:#x} cs={} adv={} sg={} stbl={} age={}", g_table.line[i],
                g_table.cs[i], g_table.adv[i], g_table.start[i], g_table.stable[i],
                g_tickNow - g_table.seen[i]);
        }
        Log(s);
    }
}

inline bool Install(GameAddresses& addr, TasSharedState* state) {
    if (!addr.sg) { Log("Race timer: no SG base"); return false; }
    auto sg = (uint8_t*)addr.sg;
    HMODULE uit = GetModuleHandleA("SR_UIT.dll");
    static constexpr GameAddresses::ModuleIdentity kUitIdentity{
        "SR_UIT.dll v1.035", 0x381DA317u, 0x00022000u
    };
    // On-disk bytes. Both sites embed an absolute address the loader rebases
    // (these DLLs never load at their preferred 0x10000000), so the imm32 at +3
    // is compared after rebasing - see GameAddresses::ValidateCodeAbs.
    static constexpr uint8_t kRaceTick[] =                    // inc word [SG+0x1D5334]; ret
        { 0x66, 0xFF, 0x05, 0x34, 0x53, 0x1D, 0x10, 0xC3 };
    static constexpr uint8_t kAppendText[] =                  // push -1; push UIT+0x12E87
        { 0x6A, 0xFF, 0x68, 0x87, 0x2E, 0x01, 0x10 };
    // Validate every site BEFORE installing anything. The race timer is optional,
    // so a mismatch here makes it unavailable rather than failing TAS_Initialize.
    bool sitesOk =
        GameAddresses::ValidateCodeAbs<3>("Supreme_Game.dll+0xB4B80", sg + 0xB4B80,
                                          kRaceTick, sg, 0x1D5334) &&
        GameAddresses::ValidateModule(uit, kUitIdentity) &&
        GameAddresses::ValidateCodeAbs<3>("SR_UIT.dll+0xED40", (uint8_t*)uit + 0xED40,
                                          kAppendText, (uint8_t*)uit, 0x12E87);
    if (!sitesOk) {
        Log(std::format("Race timer: unavailable - site validation failed (SR_UIT {:p})", (void*)uit));
        return false;
    }

    g_state = state;
    char buf[8] = {};
    g_diag = (GetEnvironmentVariableA("TAS_RACE_DIAG", buf, sizeof(buf)) > 0 && buf[0] == '1');
    g_tickNow = STALE_TICKS + 1;  // so a brand-new table never looks "just sampled"
    // Shared memory survives reinjection: an ODD race_seq from a DLL killed
    // mid-publish would make every reader reject the pair forever.
    if (g_state->race_seq & 1) InterlockedIncrement((volatile LONG*)&g_state->race_seq);
    ResetEpoch();
    g_clock = sg + 0x1D5334;
    g_tickHook = safetyhook::create_mid(sg + 0xB4B80, TickCb);  // clock tick (100/sec)
    g_aptHook = safetyhook::create_mid((uint8_t*)uit + 0xED40, AptCb);
    if (!g_tickHook || !g_aptHook) {
        // Installation is all-or-none: a failed UI hook must not leave the
        // per-tick callback running against a feature reported as unavailable.
        g_tickHook = {};
        g_aptHook = {};
        g_state = nullptr;
    }
    Log(std::format("Race timer: tick={} append={} diag={} stale={} ticks (SR_UIT {:p})",
        (bool)g_tickHook, (bool)g_aptHook, g_diag, STALE_TICKS, (void*)uit));
    return (bool)g_tickHook && (bool)g_aptHook;
}

inline void Stop() {
    g_tickHook = {};
    g_aptHook = {};
    g_state = nullptr;
}

} // namespace racetimer
