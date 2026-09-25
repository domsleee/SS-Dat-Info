#include <windows.h>
#include "log.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "caves/replay_capture_cave.hpp"
#include "caves/cycle_cave.hpp"
#include "caves/key_handler_cave.hpp"
#include "caves/observer_cave.hpp"
#include "caves/tick_cave.hpp"
#include "caves/race_timer_cave.hpp"
#include "caves/menu_cave.hpp"
#include "caves/lifecycle_cave.hpp"

static TasSharedMemory g_sharedMem;
static GameAddresses g_addr;
static volatile LONG g_initState = 0; // 0=not started, 1=running, 2=ready, 3=failed

bool run() {
    Log("=== TAS_Helper.dll loading ===");
    Log(std::format("  sizeof(TasSharedState) = {}", sizeof(TasSharedState)));

    // Validate the game build before touching shared memory or game code.
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

    // The observer and key-handler caves must precede the cycle cave, which calls BB3B10.
    bool replay_ok = InstallReplayCapture(g_addr, state);
    bool observer_ok = InstallObserverCave(g_addr, state);
    bool key_handler_ok = InstallKeyHandlerCave(g_addr, state);
    bool cycle_ok = InstallCycleCave(g_addr, state);
    bool tick_ok = InstallTickCave(g_addr, state);
    bool f5_ok = f5restart::Install(g_addr);
    bool lifecycle_ok = lifecycle::Install(g_addr, state);

    // These hooks work only as a unit: on any failure, roll all back in
    // reverse order while shared state is still mapped.
    if (!(replay_ok && observer_ok && key_handler_ok && cycle_ok && tick_ok && f5_ok && lifecycle_ok)) {
        Log("FATAL: required TAS hook installation failed; rolling back all core hooks");
        lifecycle::Uninstall();
        f5restart::Uninstall();
        UninstallTickCave();
        UninstallCycleCave();
        UninstallKeyHandlerCave();
        UninstallObserverCave();
        UninstallReplayCapture();
        g_sharedMem.Destroy();
        return false;
    }

    Log("=== Hook installation summary ===");
    Log(std::format("  Replay capture (SG+9E8F0):  {}", replay_ok ? "OK" : "FAILED"));
    Log(std::format("  the observer cave (BB3B10 gate):      {}", observer_ok ? "OK" : "FAILED"));
    Log(std::format("  the key-handler cave (handler gate):      {}", key_handler_ok ? "OK" : "FAILED"));
    Log(std::format("  the cycle cave  (Supreme::Cycle):    {}", cycle_ok ? "OK" : "FAILED"));
    Log(std::format("  the tick cave  (fixed tick):        {}", tick_ok ? "OK" : "FAILED"));
    Log(std::format("  F5 restart (accept/done):    {}", f5_ok ? "OK" : "FAILED"));
    Log(std::format("  Lifecycle (launch/stop/pump): {}", lifecycle_ok ? "OK" : "FAILED"));

    if (racetimer::Install(g_addr, state)) {
        Log("  Race timer: started");
    } else {
        Log("  Race timer: unavailable");
    }

    // Deferred until Main_Menu.dll loads.
    menustate::Install(g_addr, state);

    Log(std::format("  Renderer plugin at init: {} (the x87 precision is logged when a race starts)",
        renderer::Name(renderer::Detect())));
    Log("=== TAS_Helper.dll ready ===");
    return true;
}

// Injector.exe calls this after LoadLibrary returns, so none of this runs under
// the loader lock.
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

    // Pin the DLL: the game jumps into our hooks, so it must never be unloaded.
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
