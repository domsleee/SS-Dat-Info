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
// SafetyHook (all threads) intercepts every time-like ("NN:NN:NN") append,
// keyed by the LINE object. The PLAYER line is the one where
//   (clock - parsed_cs) & 0xFFFF   is STABLE across frames  (= start_ts):
// the opponent par's (clock-cs) drifts (par constant, clock advances) and
// pre-gate both drift. We publish the player cs + start_ts to shared state.
//
//   race_time_cs  = exact on-screen race time, centiseconds (u32::MAX = idle)
//   race_start_ts = 16-bit gate-cross clock value (F5 spawn-lottery metric)
//   clock         = SG + 0x1D5334 (16-bit centiseconds, wraps at 65536)
// ============================================================================

namespace racetimer {

inline uint8_t* g_clock = nullptr;
inline TasSharedState* g_state = nullptr;
inline uint32_t g_tick = 0;

inline uint32_t g_lines[8] = {};      // text-line objects seen
inline int      g_lineCs[8] = {};     // latest parsed centiseconds
inline int      g_lineStart[8] = {};  // last (clock - cs) & 0xFFFF
inline int      g_lineStable[8] = {}; // consecutive frames (clock-cs) unchanged
inline int      g_nLines = 0;
inline uint32_t g_playerLine = 0;     // locked player-line object once identified

static SafetyHookMid g_tickHook{};
static SafetyHookMid g_aptHook{};

static inline uint16_t ReadClk() {
    return g_clock ? *(volatile uint16_t*)g_clock : 0;
}

// "MM:SS:CC" -> centiseconds, or -1 if not a time.
static int ParseCs(const char* s, int len) {
    if (len < 8 || s[2] != ':' || s[5] != ':') return -1;
    for (int i : {0, 1, 3, 4, 6, 7})
        if (s[i] < '0' || s[i] > '9') return -1;
    int mm = (s[0] - '0') * 10 + (s[1] - '0');
    int ss = (s[3] - '0') * 10 + (s[4] - '0');
    int cc = (s[6] - '0') * 10 + (s[7] - '0');
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

// Hook of SR_UIT Append_Text: record the latest time-like text per LINE object.
static void AptCb(SafetyHookContext& ctx) {
    char buf[24]; int len;
    if (!ReadText((uint32_t)ctx.edx, buf, len)) return;
    int cs = ParseCs(buf, len);
    if (cs < 0) return;
    uint32_t line = (uint32_t)ctx.ecx;
    int idx = -1;
    for (int i = 0; i < g_nLines; i++) if (g_lines[i] == line) { idx = i; break; }
    if (idx < 0) {
        if (g_nLines >= 8) return;
        idx = g_nLines++; g_lines[idx] = line; g_lineStart[idx] = -1;
    }
    g_lineCs[idx] = cs;
}

// Once per clock tick: classify the player line and publish.
static void TickCb(SafetyHookContext&) {
    int clk = ReadClk();
    int best = -1, bestStable = 0;
    for (int i = 0; i < g_nLines; i++) {
        int sg = (clk - g_lineCs[i]) & 0xFFFF;
        if (sg == g_lineStart[i]) g_lineStable[i]++;
        else { g_lineStable[i] = 0; g_lineStart[i] = sg; }
        if (g_lineStable[i] > bestStable) { bestStable = g_lineStable[i]; best = i; }
    }
    if (best >= 0 && bestStable >= 8) g_playerLine = g_lines[best];

    if (g_state) {
        int pi = -1;
        for (int i = 0; i < g_nLines; i++) if (g_lines[i] == g_playerLine) { pi = i; break; }
        if (pi >= 0 && g_lineStable[pi] >= 3) {
            g_state->race_time_cs = (uint32_t)g_lineCs[pi];
            g_state->race_start_ts = (uint32_t)(g_lineStart[pi] & 0xFFFF);
        } else {
            g_state->race_time_cs = 0xFFFFFFFFu;
            g_state->race_start_ts = 0xFFFFFFFFu;
        }
    }
}

inline bool Install(GameAddresses& addr, TasSharedState* state) {
    if (!addr.sg) { Log("Race timer: no SG base"); return false; }
    g_state = state;
    auto sg = (uint8_t*)addr.sg;
    g_clock = sg + 0x1D5334;
    g_tickHook = safetyhook::create_mid(sg + 0xB4B80, TickCb);  // clock tick (100/sec)
    HMODULE uit = GetModuleHandleA("SR_UIT.dll");
    if (uit) g_aptHook = safetyhook::create_mid((uint8_t*)uit + 0xED40, AptCb);
    Log(std::format("Race timer: tick={} append={} (SR_UIT {:p})",
        (bool)g_tickHook, (bool)g_aptHook, (void*)uit));
    return (bool)g_tickHook && (bool)g_aptHook;
}

inline void Stop() {
    g_tickHook = {};
    g_aptHook = {};
}

} // namespace racetimer
