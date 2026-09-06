#include <windows.h>
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "caves/cave1_replay.hpp"
#include "caves/cave2.hpp"
#include "caves/cave1c.hpp"
#include "caves/cave1d.hpp"
#include "caves/cave5.hpp"
#include "caves/race_timer.hpp"
#include "caves/menu_state.hpp"
#include "level_scan.hpp"

static TasSharedMemory g_sharedMem;
static GameAddresses g_addr;
static volatile LONG g_initState = 0; // 0=not started, 1=running, 2=ready, 3=failed

bool run() {
    Log("=== TAS_Helper.dll loading ===");
    Log(std::format("  sizeof(TasSharedState) = {}", sizeof(TasSharedState)));

    // Resolve and validate the exact game build before creating the readiness
    // signal or changing any game code.
    if (!g_addr.Resolve()) {
        Log("FATAL: Failed to resolve/validate game addresses");
        return false;
    }

    if (!g_sharedMem.Create()) {
        Log("FATAL: Failed to acquire TAS shared memory. Close any other injected Supreme instance and retry.");
        return false;
    }
    Log(std::format("Shared memory '{}' created ({} bytes)",
        TAS_SHARED_MEMORY_NAME, sizeof(TasSharedState)));

    auto* state = g_sharedMem.state;

    // Order matters: Cave 1D (BB3B10 gate) and Cave 1C (handler gate) must be
    // installed before Cave 2, which calls BB3B10 directly.
    bool replay_ok = InstallReplayCapture(g_addr, state);
    bool cave1d_ok = InstallCave1D(g_addr, state);
    bool cave1c_ok = InstallCave1C(g_addr, state);
    bool cave2_ok = InstallCave2(g_addr, state);
    bool cave5_ok = InstallCave5(g_addr, state);

    // These hooks are one functional unit. Reporting ready after any of them
    // failed leaves a partially intercepted input/game loop in production and
    // makes Injector.exe's explicit initialization result meaningless. Roll
    // back in reverse dependency order while shared state is still mapped.
    if (!(replay_ok && cave1d_ok && cave1c_ok && cave2_ok && cave5_ok)) {
        Log("FATAL: required TAS hook installation failed; rolling back all core hooks");
        UninstallCave5();
        UninstallCave2();
        UninstallCave1C();
        UninstallCave1D();
        UninstallReplayCapture();
        g_sharedMem.Destroy();
        return false;
    }

    Log("=== Hook installation summary ===");
    Log(std::format("  Replay capture (SG+9E8F0):  {}", replay_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1D (BB3B10 gate):      {}", cave1d_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1C (handler gate):      {}", cave1c_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 2  (Supreme::Cycle):    {}", cave2_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 5  (fixed tick):        {}", cave5_ok ? "OK" : "FAILED"));

    // Background worker: level identity, rider identity, renderer, menu
    // housekeeping, and the out-of-cycle STOP consumer (a STOP sent at a menu
    // would otherwise wait for a level's Supreme::Cycle to run again). It
    // takes cave2's cycle heartbeat because a frozen cycle is the only signal
    // that notices a return to the menu.
    levelscan::Start(state, (uint32_t)g_addr.level_path_ptr, &SafeReadPtr, &g_lastCycleMs);
    if (levelscan::g_thread) {
        Log("  Level scan thread: started");
    } else {
        Log("  Level scan thread: FAILED TO START — level stays unresolved");
    }

    // Race timer: read the exact on-screen race time (HUD/SR_UIT) -> shared state.
    if (racetimer::Install(g_addr, state)) {
        Log("  Race timer: started");
    } else {
        Log("  Race timer: unavailable");
    }

    // Menu state (which screen the game is on) - reads the Main_Menu.dll menu
    // object via a Change_Page hook, deferred until that DLL loads.
    menustate::Install(g_addr, state);

    Log(std::format("  Renderer plugin at init: {} (x87 control word is sampled on the game thread; see level-scan log lines)",
        renderer::Name(renderer::Detect())));
    Log("=== TAS_Helper.dll ready ===");
    return true;
}

// Injector.exe calls this only after its LoadLibrary remote thread has returned,
// so none of the CRT, file I/O, hook installation or worker startup below runs
// under the Windows loader lock.
extern "C" __declspec(dllexport) DWORD WINAPI TAS_Initialize(LPVOID) {
    LONG previous = InterlockedCompareExchange(&g_initState, 1, 0);
    if (previous == 2) return 1;
    if (previous != 0) return 0;
    try {
        if (!run()) {
            InterlockedExchange(&g_initState, 3);
            return 0;
        }
    }
    catch (const std::exception& e) {
        Log(std::format("FATAL exception: {}", e.what()));
        InterlockedExchange(&g_initState, 3);
        return 0;
    }

    // This DLL installs callbacks whose code and data are referenced directly by
    // the game. Pin it after successful initialization so an accidental
    // FreeLibrary cannot unload those callbacks and turn the next game tick into
    // a jump through freed memory. Process termination needs no explicit teardown.
    HMODULE pinned = nullptr;
    if (!GetModuleHandleExA(
            GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_PIN,
            reinterpret_cast<LPCSTR>(&TAS_Initialize), &pinned)) {
        Log(std::format("WARNING: failed to pin TAS_Helper.dll (error {})", GetLastError()));
    }
    InterlockedExchange(&g_initState, 2);
    return 1;
}

#if defined(_M_IX86)
#pragma comment(linker, "/EXPORT:TAS_Initialize=_TAS_Initialize@4")
#endif

BOOL APIENTRY DllMain(HMODULE module, DWORD reason, LPVOID) {
    if (reason == DLL_PROCESS_ATTACH) {
        DisableThreadLibraryCalls(module);
    }
    return TRUE;
}
