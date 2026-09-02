#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../game_addresses.hpp"
#include "../external/safetyhook.hpp"
#include "../shared_state.hpp"

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
// fresh sample). The PLAYER line is identified by a CLOCK-INDEPENDENT signal:
// its parsed cs actually ADVANCES (a running timer), while an opponent par/
// record has a CONSTANT cs. (The earlier "stable (clock-cs)" test only holds
// in a LIVE race — during a REPLAY the HUD time is replay-data-driven and
// decoupled from the live clock, so (clock-cs) drifts and that test could
// never lock => the chip flickered. Advancing-cs works in both.) We still
// track (clock-cs) separately, but only to decide whether start_ts is
// trustworthy (a real live gate value) vs unknown (replay). We publish:
//   race_time_cs  = exact on-screen race time, centiseconds (u32::MAX = idle)
//   race_start_ts = 16-bit gate-cross clock value (F5 spawn-lottery metric)
//   clock         = SG + 0x1D5334 (16-bit centiseconds, wraps at 65536)
//
// LATCH MODEL (2026-06-09, fixes chip flicker): the HUD cs and the raw clock
// are not perfectly phase-locked — (clock - cs) occasionally lands +/-1 off
// (sub-tick rounding; the same wobble you see as start_ts 477<->478). The old
// "publish only while >= N consecutive identical (clock-cs)" gate blanked the
// chip for a few frames on every wobble => flicker. Now: once a line LOCKS as
// the player, we LATCH it and keep publishing its live cs through the +/-1
// wobble and through the finish freeze. We blank only when nothing is locked
// yet, an advancing ghost makes the pick ambiguous, or the epoch resets (menu).
// The cached start_ts is refreshed only when (clock-cs) has settled, so the
// wobble never moves it.
//
// DIAGNOSTICS: set the env var TAS_RACE_DIAG=1 before launch to log every
// blank<->show transition and a periodic line dump to TAS_Helper.log (used to
// understand why replays flicker — replay HUD time is replay-data-driven, not
// clock-start_ts, so (clock-cs) may never settle).
// ============================================================================

namespace racetimer {

constexpr int      SLOTS = 8;
constexpr int      LOCK_FRAMES = 8;     // advancing-cs samples to LOCK a line as the player
constexpr int      SETTLE_FRAMES = 3;   // distinct ticks (clock-cs) constant before start_ts is trusted
constexpr uint32_t MAXU = 0xFFFFFFFFu;

inline uint8_t* g_clock = nullptr;
inline TasSharedState* g_state = nullptr;

// Fixed slot table (g_lines[i] == 0 means free). Single realistic writer is the
// render thread via AptCb, but the insert is interlocked to be safe.
inline volatile LONG g_lines[SLOTS] = {};   // text-line object ptrs (0 = free)
inline int      g_lineCs[SLOTS] = {};        // latest parsed centiseconds
inline int      g_lineAdv[SLOTS] = {};       // consecutive samples where cs ADVANCED (player signal)
inline int      g_lineStart[SLOTS] = {};     // last (clock - cs) & 0xFFFF (= start_ts candidate)
inline int      g_lineStable[SLOTS] = {};    // consecutive distinct-tick (clock-cs)-constant samples
inline uint32_t g_lineLastClk[SLOTS] = {};   // clock at last sample (sentinel = MAXU)

inline uint32_t g_playerLine = 0;     // locked player-line object once identified
inline uint32_t g_playerStartTs = MAXU; // cached good start_ts (survives wobble/finish)
inline int      g_wasInGame = -1;     // game_in_game edge tracker for epoch reset

// Diagnostics (TAS_RACE_DIAG=1)
inline bool     g_diag = false;
inline uint32_t g_diagTick = 0;
inline uint32_t g_lastPub = MAXU;     // last published cs (MAXU = blanked)

static SafetyHookMid g_tickHook{};
static SafetyHookMid g_aptHook{};

static inline uint16_t ReadClk() {
    return g_clock ? *(volatile uint16_t*)g_clock : 0;
}

// Publish, recording blank<->show transitions for diagnosis.
static void Publish(uint32_t cs, uint32_t start, const char* reason) {
    if (!g_state) return;
    g_state->race_time_cs = cs;
    g_state->race_start_ts = start;
    if (g_diag && (g_lastPub == MAXU) != (cs == MAXU)) {
        Log(std::format("[racetimer] {} ({}) clk={} cs={} start={}",
            cs == MAXU ? "BLANK" : "SHOW", reason, (int)ReadClk(),
            cs == MAXU ? -1 : (int)cs, start == MAXU ? -1 : (int)start));
    }
    g_lastPub = cs;
}

static void ResetEpoch() {
    for (int i = 0; i < SLOTS; i++) {
        g_lines[i] = 0;
        g_lineCs[i] = 0;
        g_lineAdv[i] = 0;
        g_lineStart[i] = -1;
        g_lineStable[i] = 0;
        g_lineLastClk[i] = MAXU;
    }
    g_playerLine = 0;
    g_playerStartTs = MAXU;
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

// Find the slot for `line`, or claim a free one (interlocked). -1 if full.
static int SlotFor(uint32_t line) {
    for (int i = 0; i < SLOTS; i++)
        if ((uint32_t)g_lines[i] == line) return i;
    for (int i = 0; i < SLOTS; i++) {
        if (g_lines[i] == 0 &&
            InterlockedCompareExchange(&g_lines[i], (LONG)line, 0) == 0) {
            g_lineCs[i] = 0;
            g_lineAdv[i] = 0;
            g_lineStart[i] = -1;
            g_lineStable[i] = 0;
            g_lineLastClk[i] = MAXU;
            if (g_diag) Log(std::format("[racetimer] slot {} claimed line={:#x}", i, line));
            return i;
        }
    }
    return -1;
}

// Classify against all current lines and (re)publish. The player line is the
// one whose cs ADVANCES (g_lineAdv >= LOCK_FRAMES) — clock-independent, so it
// works in a replay too. Latch model: once locked we keep publishing its live
// cs even when (clock-cs) wobbles +/-1 or freezes at the finish — blanking only
// when no advancing line is locked yet, a 2nd advancing line (ghost/racer)
// makes the pick ambiguous, or the epoch resets (menu). start_ts is published
// only when (clock-cs) has settled (live race); in a replay it stays unknown.
static void ClassifyAndPublish() {
    int advCount = 0, best = -1, bestAdv = 0;
    for (int i = 0; i < SLOTS; i++) {
        if (g_lines[i] == 0) continue;
        if (g_lineAdv[i] >= LOCK_FRAMES) {
            advCount++;
            if (g_lineAdv[i] > bestAdv) { bestAdv = g_lineAdv[i]; best = i; }
        }
    }

    // Acquire / refresh the lock only when exactly one line is advancing.
    if (advCount == 1 && best >= 0) g_playerLine = (uint32_t)g_lines[best];

    int pi = -1;
    if (g_playerLine != 0)
        for (int i = 0; i < SLOTS; i++)
            if ((uint32_t)g_lines[i] == g_playerLine) { pi = i; break; }

    if (pi < 0) { Publish(MAXU, MAXU, "acquiring"); return; }

    if (advCount >= 2) {                          // advancing ghost / 2nd racer
        g_playerLine = 0; g_playerStartTs = MAXU;
        Publish(MAXU, MAXU, "ambiguous");
        return;
    }

    // Latched: publish the live displayed time. Refresh the cached start_ts only
    // once (clock-cs) has settled (live race); never recompute it from a
    // drifting replay clock or the post-finish freeze.
    if (g_lineStable[pi] >= SETTLE_FRAMES)
        g_playerStartTs = (uint32_t)(g_lineStart[pi] & 0xFFFF);
    Publish((uint32_t)g_lineCs[pi], g_playerStartTs, "latched");
}

// Hook of SR_UIT Append_Text: classify on each fresh time-like sample.
static void AptCb(SafetyHookContext& ctx) {
    char buf[24]; int len;
    if (!ReadText((uint32_t)ctx.edx, buf, len)) return;
    int cs = ParseCs(buf, len);
    if (cs < 0) return;
    uint32_t line = (uint32_t)ctx.ecx;
    int i = SlotFor(line);
    if (i < 0) return;

    uint32_t clk = ReadClk();
    int prevCs = g_lineCs[i];
    bool first = (g_lineLastClk[i] == MAXU);

    // Player signal: cs advances (a running timer). A small forward step bumps
    // the counter; a backward jump (F5 / new race) resets it; an equal cs
    // (opponent par, or a frozen finish) leaves it unchanged.
    if (!first) {
        int d = cs - prevCs;
        if (d > 0 && d < 30000) { if (g_lineAdv[i] < 1000000) g_lineAdv[i]++; }
        else if (d < 0)         { g_lineAdv[i] = 0; }
    }

    // start_ts trust: is (clock - cs) constant across distinct clock ticks?
    bool clkMoved = (g_lineLastClk[i] != clk);
    int sg = (clk - cs) & 0xFFFF;
    if (clkMoved) {
        if (sg == g_lineStart[i]) {
            if (g_lineStable[i] < 1000000) g_lineStable[i]++;
        } else {
            g_lineStart[i] = sg;
            g_lineStable[i] = 0;
        }
        g_lineLastClk[i] = clk;
    }
    g_lineCs[i] = cs;

    ClassifyAndPublish();
}

// Once per clock tick: menu epoch reset (game_in_game 1 -> 0) + periodic diag.
static void TickCb(SafetyHookContext&) {
    if (!g_state) return;
    int inGame = g_state->game_in_game ? 1 : 0;
    if (inGame == 0) {
        if (g_wasInGame != 0) ResetEpoch();
        else Publish(MAXU, MAXU, "menu");
    }
    g_wasInGame = inGame;

    if (g_diag && inGame && (++g_diagTick % 128) == 0) {
        int clk = ReadClk();
        std::string s = std::format("[racetimer] clk={} player={:#x} pub={}",
            clk, g_playerLine, g_lastPub == MAXU ? -1 : (int)g_lastPub);
        for (int i = 0; i < SLOTS; i++) {
            if (g_lines[i] == 0) continue;
            s += std::format(" | L{:#x} cs={} adv={} sg={} stbl={}", (uint32_t)g_lines[i],
                g_lineCs[i], g_lineAdv[i], g_lineStart[i], g_lineStable[i]);
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
    // Validate every site BEFORE installing anything. The race timer is optional
    // (TAS_NO_RACETIMER), so a mismatch here makes it unavailable rather than
    // failing TAS_Initialize.
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
    Log(std::format("Race timer: tick={} append={} diag={} (SR_UIT {:p})",
        (bool)g_tickHook, (bool)g_aptHook, g_diag, (void*)uit));
    return (bool)g_tickHook && (bool)g_aptHook;
}

inline void Stop() {
    g_tickHook = {};
    g_aptHook = {};
    g_state = nullptr;
}

} // namespace racetimer
