#pragma once
#include "stdafx.h"
#include "log.hpp"
#include "srconfig_guard_policy.hpp"
#include <safetyhook.hpp>

// sr.dll srConfig hash-node unlink guard - see srconfig_guard_policy.hpp for
// the why. The detour validates the node with SEH-probed reads before letting
// the original unlink run; a corrupt node is skipped and counted. The count is
// logged from the level-scan worker (no formatting or file I/O inside the
// game's call).
//
// This lived in Display_Config_Helper on the unmerged render-distance-600
// branch (PR #44); TAS_Helper is what is actually injected here, so it
// carries the guard itself.
namespace srconfigguard {

inline SafetyHookInline g_unlinkHook{};
inline volatile LONG g_skipped = 0;
inline LONG g_skippedLogged = 0;
inline uintptr_t g_lastSkippedNode = 0;

// sr.dll+0x13550: sub esp,8; push ebx; push esi; mov esi,[esp+14h];
// test esi,esi; push edi; mov edi,ecx; mov [esp+0Ch],edi. No absolute
// immediates, so the on-disk bytes are the live bytes.
inline constexpr std::uint32_t UNLINK_RVA = 0x13550;
inline constexpr std::uint8_t UNLINK_PROLOGUE[] = {
    0x83, 0xEC, 0x08, 0x53, 0x56, 0x8B, 0x74, 0x24, 0x14,
    0x85, 0xF6, 0x57, 0x8B, 0xF9, 0x89, 0x7C, 0x24, 0x0C};

inline bool ProbeReadPtr(uintptr_t address, uintptr_t* out) {
    if (address < SRCONFIG_MIN_HEAP_ADDRESS) return false;
    __try {
        *out = *(uintptr_t const volatile*)address;
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

inline void __fastcall UnlinkDetour(void* self, void* /*edx*/, void* node) {
    if (!SrConfigNodeLooksSane((uintptr_t)node, ProbeReadPtr)) {
        g_lastSkippedNode = (uintptr_t)node;
        InterlockedIncrement(&g_skipped);
        return;  // leak the node instead of dying
    }
    g_unlinkHook.thiscall<void>(self, node);
}

inline bool Install() {
    auto module = (std::uint8_t*)GetModuleHandleA("sr.dll");
    if (!module) {
        Log("srConfig guard: sr.dll not loaded - not installed");
        return false;
    }
    std::uint8_t* fn = module + UNLINK_RVA;
    if (std::memcmp(fn, UNLINK_PROLOGUE, sizeof UNLINK_PROLOGUE) != 0) {
        Log("srConfig guard: sr.dll+0x13550 bytes do not match the known unlink prologue - not installed");
        return false;
    }
    g_unlinkHook = safetyhook::create_inline((void*)fn, (void*)UnlinkDetour);
    if (!g_unlinkHook) {
        Log("srConfig guard: hook failed");
        return false;
    }
    Log(std::format("srConfig guard: installed at sr.dll+{:#x} ({:p})", UNLINK_RVA, (void*)fn));
    return true;
}

// Worker-thread side: report skips the detour counted.
inline void FlushLog() {
    LONG skipped = g_skipped;
    if (skipped != g_skippedLogged) {
        g_skippedLogged = skipped;
        Log(std::format("srConfig guard: SKIPPED corrupt unlink #{} (node={:#x}) - the game would have crashed here",
                        skipped, g_lastSkippedNode));
    }
}

}  // namespace srconfigguard
