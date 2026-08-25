#pragma once
#include "../stdafx.h"
#include "../log.hpp"
#include "../shared_state.hpp"
#include "../external/safetyhook.hpp"

// ---------------------------------------------------------------------------
// DIAGNOSTIC ONLY (env-gated TAS_DRAWPROBE=1) — glDrawElements logger.
//
// The "missing far triangles at 600m detail 4" bug: srDD_OpenGL draws
// GL_UNSIGNED_INT (32-bit) yet geometry wraps at 65,536 vertices. This hook
// answers the decisive question WITHOUT patching anything: what count/type
// actually reaches glDrawElements, and does terrain submit one >65535 call
// (index-content truncation) or many sub-batches (vertex aliasing)?
//
// It publishes per-frame maxima into spare shared fields so an out-of-process
// reader (a Python script) can sample them at 450 vs 600 without a debugger.
// Pure observation: no behaviour change, safe on a live game.
// ---------------------------------------------------------------------------

namespace drawprobe {

inline SafetyHookInline g_hook{};
inline TasSharedState*  g_state = nullptr;

// Per-frame terrain vertex accounting via a SwapBuffers boundary. If the
// per-frame terrain total CAPS at 65,536 at 600 (but is lower at 450), the
// visible terrain is being TRUNCATED at a u16 vertex budget — far patches
// dropped = missing distant triangles. g_frameMaxTerrain = max over frames.
inline SafetyHookInline g_swapHook2{};
inline volatile uint32_t g_frameTerrain = 0;      // accumulating this frame
inline volatile uint32_t g_frameMaxTerrain = 0;   // max frame total seen
inline volatile uint32_t g_frameDraws = 0;        // terrain draws this frame
inline volatile uint32_t g_frameMaxDraws = 0;

inline int __stdcall Swap2Detour(void* hdc) {
    if (g_frameTerrain > g_frameMaxTerrain) g_frameMaxTerrain = g_frameTerrain;
    if (g_frameDraws > g_frameMaxDraws) g_frameMaxDraws = g_frameDraws;
    g_frameTerrain = 0;
    g_frameDraws = 0;
    return g_swapHook2.stdcall<int, void*>(hdc);
}

// GL constants (avoid pulling in gl.h).
inline constexpr unsigned GL_UNSIGNED_BYTE  = 0x1401;
inline constexpr unsigned GL_UNSIGNED_SHORT = 0x1403;
inline constexpr unsigned GL_UNSIGNED_INT   = 0x1405;

// Per-frame accumulators (reset by cave2's cycle each frame is overkill; we
// just publish running maxima and let the reader diff across a fresh restart).
inline volatile uint32_t g_maxCount = 0;   // largest `count` seen
inline volatile uint32_t g_lastType = 0;   // most recent index type
inline volatile uint32_t g_callTally = 0;  // total glDrawElements calls
inline volatile uint32_t g_over64k = 0;    // calls with count > 65535
inline volatile uint32_t g_maxIndex = 0;   // largest INDEX VALUE seen (the wrap tell)

typedef void(__stdcall* glDrawElements_t)(unsigned mode, int count, unsigned type,
                                          const void* indices);
typedef void(__stdcall* glVertexPointer_t)(int size, unsigned type, int stride,
                                           const void* ptr);

inline SafetyHookInline g_vpHook{};
// Correlate the LAST glVertexPointer base with the NEXT draw: only draws with a
// terrain-sized count (>1000) count toward the terrain pool span, filtering out
// HUD/boarder meshes that also call glVertexPointer.
inline volatile uintptr_t g_lastVpBase = 0;
inline volatile uint32_t  g_vpStride = 0;
inline volatile uintptr_t g_terMin = (uintptr_t)-1;
inline volatile uintptr_t g_terMax = 0;
inline volatile uint32_t  g_vpSpanVerts = 0;

// One-shot log of the first N glVertexPointer + the NEXT glDrawElements, so we
// see the REAL terrain vertex-array address, size, and a garbage check —
// entirely via the log file, immune to objsnap-buffer contention. Armed by
// TAS_DRAWLOG=1 (checked once).
inline volatile int g_vpLogLeft = 0;
inline volatile int g_dumpNext = 0;
inline uintptr_t g_pendBase = 0;
inline int g_pendSize = 0, g_pendStride = 0;

// SEH-guarded vertex scan in its own function (no C++ object unwinding here, so
// __try is legal). Returns count of garbage vertices, or -1 on fault.
inline int scanVerts(uintptr_t base, int stride, int n, float* outMaxMag) {
    int bad = 0; float maxmag = 0.0f;
    const uint8_t* vb = (const uint8_t*)base;
    __try {
        for (int v = 0; v < n; v++) {
            const float* f = (const float*)(vb + (size_t)v * stride);
            for (int a = 0; a < 3; a++) {
                float val = f[a];
                if (val != val || val > 1e9f || val < -1e9f) { bad++; break; }
                float m = val < 0 ? -val : val;
                if (m > maxmag) maxmag = m;
            }
        }
    } __except (EXCEPTION_EXECUTE_HANDLER) { *outMaxMag = maxmag; return -1; }
    *outMaxMag = maxmag;
    return bad;
}

inline void __stdcall VpDetour(int size, unsigned type, int stride, const void* ptr) {
    uintptr_t p = (uintptr_t)ptr;
    if (p > 0x10000) {
        g_lastVpBase = p;
        if (stride > 0) g_vpStride = (uint32_t)stride;
    }
    if (g_vpLogLeft > 0 && p > 0x10000 && stride >= 12) {
        g_vpLogLeft--;
        int lastFinite = -1, firstBad = -1;
        for (int v = 0; v < 200000; v += 512) {
            float mm = 0.0f;
            int bad = scanVerts(p + (size_t)v * stride, stride, 512, &mm);
            if (bad < 0) { firstBad = v; break; }
            if (bad == 0) { lastFinite = v + 511; }
            else if (firstBad < 0) { firstBad = v; }
        }
        Log(std::format("VPBUF: base={:#x} stride={} lastFinite~{} firstBad~{}",
                        p, stride, lastFinite, firstBad));
    }
    g_vpHook.stdcall<void, int, unsigned, int, const void*>(size, type, stride, ptr);
}

// Ring of distinct terrain vertex arrays (base, maxIndexSeen) recorded at draw
// time, published to shared memory so an out-of-process reader can DUMP the
// actual vertex data and check it for garbage. 48 slots is plenty: the visible
// set is a few dozen chunk arrays.
inline constexpr uint32_t VP_RING = 48;
inline uintptr_t g_ringBase[VP_RING] = {};
inline uint32_t  g_ringMaxIdx[VP_RING] = {};
inline volatile uint32_t g_ringUsed = 0;

inline void ringRecord(uintptr_t base, uint32_t maxIdx) {
    for (uint32_t i = 0; i < g_ringUsed; i++) {
        if (g_ringBase[i] == base) {
            if (maxIdx > g_ringMaxIdx[i]) g_ringMaxIdx[i] = maxIdx;
            return;
        }
    }
    if (g_ringUsed < VP_RING) {
        g_ringBase[g_ringUsed] = base;
        g_ringMaxIdx[g_ringUsed] = maxIdx;
        g_ringUsed++;
    }
}

// Hook srGlobalRecycler::allocate(K) in sr.dll — the >=16KB allocator the
// terrain vertex buffers come from. Capture the LARGEST K and the return
// address that requested it: that caller is the terrain buffer sizer, where a
// u16 vertex total would wrap. If maxK SHRINKS at 600 vs 450, caught.
inline volatile uint32_t g_allocMaxK   = 0;  // largest allocation size seen
inline volatile uint32_t g_allocMaxRet = 0;  // caller ret-addr for that maxK (SG-relative)
inline volatile uint32_t g_allocCalls  = 0;
inline uintptr_t g_sgBase = 0;               // Supreme_Game.dll live base

// Capture the max u16 value flowing through the terrain unpack at SG+0xC082B
// (right after `and eax,0xffff`). If it saturates at 65535 at 600 but not 450,
// the terrain index is a fundamental 16-bit field — the real reason 600@4
// shreds and the devs capped distance at 450.
inline volatile uint32_t g_u16max = 0;
inline volatile uint32_t g_u16calls = 0;
inline void u16Mid(SafetyHookContext& ctx) {
    uint32_t v = ctx.eax & 0xffff;
    g_u16calls++;
    if (v > g_u16max) g_u16max = v;
}
inline SafetyHookMid g_u16Mid{};

inline void allocMid(SafetyHookContext& ctx) {
    // __thiscall: ecx=this, K at [esp+4] (esp points at return addr on entry).
    uint32_t* sp = (uint32_t*)ctx.esp;
    uint32_t ret = sp[0];   // caller return address
    uint32_t k   = sp[1];   // requested size in bytes
    g_allocCalls++;
    if (k > g_allocMaxK && k < 0x08000000u) {
        g_allocMaxK = k;
        // Report the caller as Supreme_Game-relative if it lives there.
        g_allocMaxRet = (g_sgBase && ret >= g_sgBase && ret < g_sgBase + 0x300000)
                            ? (uint32_t)(ret - g_sgBase)
                            : ret;
    }
}
inline SafetyHookMid g_allocMid{};



inline void __stdcall Detour(unsigned mode, int count, unsigned type,
                             const void* indices) {
    // One-shot terrain draw dump: after a logged glVertexPointer, examine the
    // vertex array this draw references and report garbage. Terrain-sized only.
    if (g_dumpNext && count > 1000 && g_pendBase > 0x10000 && g_pendStride >= 12) {
        g_dumpNext = 0;
        uint32_t maxi = 0;
        if ((uintptr_t)indices > 0x10000 && type == GL_UNSIGNED_INT) {
            const uint32_t* idx = (const uint32_t*)indices;
            for (int k = 0; k < count; k++) if (idx[k] > maxi && idx[k] < 0x100000u) maxi = idx[k];
        }
        float maxmag = 0.0f;
        int n = (int)maxi + 1; if (n > 8000) n = 8000;
        int bad = scanVerts(g_pendBase, g_pendStride, n, &maxmag);
        Log(std::format("DRAW: count={} maxidx={} verts_checked={} BAD={} maxmag={:.0f}",
                        count, maxi, n, bad, maxmag));
    }
    if (count > 0) {
        uint32_t c = (uint32_t)count;
        if (c > g_maxCount) g_maxCount = c;
        g_lastType = type;
        g_callTally++;
        if (c > 65535u) g_over64k++;
        // Max INDEX VALUE: if a small batch's indices point past 65535 into a
        // large shared vertex pool, THAT is where a u16 slot would wrap.
        // Old GL: `indices` is a client pointer (no VBO in srDD_OpenGL). Guard
        // the deref and only sample first + last (O(1)) — the terrain strips
        // are ~monotonic, so those bracket the range.
        // Terrain-sized batch: fold its vertex-array base into the terrain pool
        // span. The base+localMaxIndex reaches the highest pool slot touched;
        // if that exceeds 65535 at 600 but not 450, the pool overflowed.
        // Terrain draws: stride-16 GL_FLOAT arrays from a real client base, ANY
        // size (they are many small batches, not one big one). Sum all of them
        // per frame.
        if (g_lastVpBase > 0x10000 && g_vpStride == 16) {
            uintptr_t b = g_lastVpBase;
            if (b < g_terMin) g_terMin = b;
            if (b > g_terMax) g_terMax = b;
            g_vpSpanVerts = (uint32_t)((g_terMax - g_terMin) / (uintptr_t)g_vpStride) + c;
            g_frameTerrain += c;
            g_frameDraws++;
            // Terrain array ring: record base + its max referenced index so the
            // out-of-process reader knows how many vertices to dump.
            uint32_t localMax = 0;
            if ((uintptr_t)indices > 0x10000 && type == GL_UNSIGNED_INT) {
                const uint32_t* idx = (const uint32_t*)indices;
                localMax = idx[0] > idx[c - 1] ? idx[0] : idx[c - 1];
            }
            ringRecord(b, localMax);
        }
        uintptr_t p = (uintptr_t)indices;
        if (p > 0x10000 && type == GL_UNSIGNED_INT) {
            const uint32_t* idx = (const uint32_t*)indices;
            uint32_t a = idx[0];
            uint32_t b = idx[c - 1];
            uint32_t m = a > b ? a : b;
            if (m > g_maxIndex && m < 0x0FFFFFFFu) g_maxIndex = m;
        } else if (p > 0x10000 && type == GL_UNSIGNED_SHORT) {
            const uint16_t* idx = (const uint16_t*)indices;
            uint32_t a = idx[0];
            uint32_t b = idx[c - 1];
            uint32_t m = a > b ? a : b;
            if (m > g_maxIndex) g_maxIndex = m;
        }
        if (g_state) {
            // Publish into objsnap_arm_player[] — a research buffer that is idle
            // except during snapshot experiments, so nothing clobbers us (the
            // diag_* fields are rewritten by cave5 every frame).
            g_state->objsnap_arm_player[0] = 0xD4A3B0BEu;  // locator sentinel
            g_state->objsnap_arm_player[1] = g_maxCount;
            g_state->objsnap_arm_player[2] = g_lastType;
            g_state->objsnap_arm_player[3] = g_callTally;
            g_state->objsnap_arm_player[4] = g_over64k;
            g_state->objsnap_arm_player[5] = g_maxIndex;
            g_state->objsnap_arm_player[6] = g_vpSpanVerts;  // vertices spanned by the pool
            g_state->objsnap_arm_player[7] = g_vpStride;
            // u16-unpack stats on the idle gate buffer (cave2 only writes it at
            // a REC/PLAY gate, not during terrain viewing): sentinel + max + calls.
            g_state->objsnap_gate_player[0] = 0xB16C0DE5u;
            g_state->objsnap_gate_player[1] = g_u16max;
            g_state->objsnap_gate_player[2] = g_u16calls;
            g_state->objsnap_gate_player[3] = g_frameMaxTerrain;  // max terrain verts/frame
            g_state->objsnap_gate_player[4] = g_frameMaxDraws;    // max terrain draws/frame
            g_state->objsnap_arm_player[8] = g_allocMaxK;    // largest allocation (bytes)
            g_state->objsnap_arm_player[9] = g_allocMaxRet;  // caller ret (SG-relative)
            g_state->objsnap_arm_player[10] = g_allocCalls;
            // Terrain array ring for the out-of-process dumper:
            // [16] = used count, then pairs (base, maxIdx) from [17].
            g_state->objsnap_arm_player[16] = g_ringUsed;
            for (uint32_t i = 0; i < g_ringUsed && (17 + i * 2 + 1) < 128; i++) {
                g_state->objsnap_arm_player[17 + i * 2] = (uint32_t)g_ringBase[i];
                g_state->objsnap_arm_player[17 + i * 2 + 1] = g_ringMaxIdx[i];
            }
        }
    }
    g_hook.stdcall<void, unsigned, int, unsigned, const void*>(mode, count, type, indices);
}



inline DWORD WINAPI InstallThread(LPVOID param) {
    g_state = (TasSharedState*)param;
    // Install the ALLOCATOR hook IMMEDIATELY — sr.dll is present from injection,
    // and the terrain vertex buffers are allocated at LEVEL LOAD, which can
    // happen before opengl32 comes up. Waiting would miss them.
    g_sgBase = (uintptr_t)GetModuleHandleA("Supreme_Game.dll");
    if (HMODULE sr0 = GetModuleHandleA("sr.dll")) {
        void* al = (void*)((uintptr_t)sr0 + 0x3E9B0);
        g_allocMid = safetyhook::create_mid(al, allocMid);
        Log(g_allocMid ? "DrawProbe: srGlobalRecycler::allocate mid-hooked (early)"
                       : "DrawProbe: srGlobalRecycler::allocate hook FAILED");
    }
    if (g_sgBase) {
        void* u16 = (void*)(g_sgBase + 0xC082B);
        g_u16Mid = safetyhook::create_mid(u16, u16Mid);
        Log(g_u16Mid ? "DrawProbe: SG+0xC082B u16-unpack mid-hooked"
                     : "DrawProbe: SG+0xC082B hook FAILED");
    }
    // opengl32 loads well after DLL init (the game brings the renderer up
    // lazily). Poll for it, then hook glDrawElements.
    for (int i = 0; i < 600; i++) {  // up to ~60s
        HMODULE gl = GetModuleHandleA("opengl32.dll");
        if (gl) {
            auto* fn = GetProcAddress(gl, "glDrawElements");
            if (fn) {
                g_hook = safetyhook::create_inline((void*)fn, (void*)Detour);
                if (g_hook) {
                    Log(std::format("DrawProbe: opengl32!glDrawElements hooked at {:p}", (void*)fn));
                } else {
                    Log("DrawProbe: SafetyHook on glDrawElements FAILED");
                }
                if (auto* vp = GetProcAddress(gl, "glVertexPointer")) {
                    g_vpHook = safetyhook::create_inline((void*)vp, (void*)VpDetour);
                    Log(g_vpHook ? "DrawProbe: glVertexPointer hooked"
                                 : "DrawProbe: glVertexPointer hook FAILED");
                }
                if (HMODULE gdi = GetModuleHandleA("gdi32.dll")) {
                    if (auto* sb = GetProcAddress(gdi, "SwapBuffers")) {
                        g_swapHook2 = safetyhook::create_inline((void*)sb, (void*)Swap2Detour);
                        Log(g_swapHook2 ? "DrawProbe: SwapBuffers hooked (frame boundary)"
                                        : "DrawProbe: SwapBuffers hook FAILED");
                    }
                }
                // Arm the one-shot vertex-array log (TAS_DRAWLOG=1): log the
                // next 60 glVertexPointer calls + their draws' garbage check.
                char dl[8] = {0};
                if (GetEnvironmentVariableA("TAS_DRAWLOG", dl, sizeof(dl)) > 0 && dl[0] == '1') {
                    g_vpLogLeft = 6;
                    Log("DrawProbe: vertex-array log ARMED (60 samples)");
                }
                return 0;
            }
        }
        Sleep(100);
    }
    Log("DrawProbe: opengl32.dll never appeared");
    return 0;
}



inline bool Install(TasSharedState* state) {
    CreateThread(nullptr, 0, InstallThread, state, 0, nullptr);
    return true;
}

}  // namespace drawprobe
