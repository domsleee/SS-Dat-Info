#include <windows.h>
#include "log.hpp"
#include "helper.hpp"
#include "shared_state.hpp"
#include "game_addresses.hpp"
#include "caves/cave1_replay.hpp"
#include "caves/cave2.hpp"
#include "caves/cave1c.hpp"
#include "caves/cave1d.hpp"
#include "caves/cave5.hpp"
#include "caves/race_timer.hpp"
#include "level_scan.hpp"

static TasSharedMemory g_sharedMem;
static GameAddresses g_addr;

void run() {
    Log("=== TAS_Helper.dll loading (Phase 2) ===");
    Log(std::format("  sizeof(TasSharedState) = {}", sizeof(TasSharedState)));

    // Step 1: Create shared memory
    if (!g_sharedMem.Create()) {
        Log("FATAL: Failed to create shared memory mapping");
        return;
    }
    Log(std::format("Shared memory '{}' created ({} bytes)",
        TAS_SHARED_MEMORY_NAME, sizeof(TasSharedState)));

    // Step 2: Resolve game addresses
    if (!g_addr.Resolve()) {
        Log("FATAL: Failed to resolve game addresses");
        return;
    }

    auto* state = g_sharedMem.state;

    // Step 3: Install hooks
    // Order matters: Cave 1D (BB3B10 gate) must be installed before Cave 2
    // because Cave 2 calls BB3B10 directly. Cave 1C (handler gate) must be
    // installed before Cave 2 for the same reason.
    bool replay_ok = InstallReplayCapture(g_addr, state);
    bool cave1d_ok = InstallCave1D(g_addr, state);
    bool cave1c_ok = InstallCave1C(g_addr, state);
    bool cave2_ok = InstallCave2(g_addr, state);
    bool cave5_ok = InstallCave5(g_addr, state);

    Log("=== Hook installation summary ===");
    Log(std::format("  Replay capture (SG+9E8F0):  {}", replay_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1D (BB3B10 gate):      {}", cave1d_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1C (handler gate):      {}", cave1c_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 2  (Supreme::Cycle):    {}", cave2_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 5  (fixed tick):        {}", cave5_ok ? "OK" : "FAILED"));

    if (!cave2_ok) {
        Log("CRITICAL: Cave 2 hook failed - TAS will not function");
    }
    if (!cave1d_ok || !cave1c_ok) {
        Log("WARNING: Gate hooks failed - input blocking won't work correctly");
    }

    // Background thread: detect the current track via an in-process heap scan.
    levelscan::Start(state);
    Log("  Level scan thread: started");

    // Race timer: read the exact on-screen race time (HUD/SR_UIT) → shared state.
    racetimer::Install(g_addr, state);
    Log("  Race timer: started");

    Log("=== TAS_Helper.dll ready (Phase 2) ===");
}

BOOL APIENTRY DllMain(HMODULE, DWORD reason, LPVOID) {
    if (reason == DLL_PROCESS_DETACH) {
        Log("TAS_Helper.dll unloading");
        racetimer::Stop();
        levelscan::Stop();
        g_sharedMem.Destroy();
        return TRUE;
    }

    if (reason != DLL_PROCESS_ATTACH) {
        return TRUE;
    }

    try {
        run();
    }
    catch (const std::exception& e) {
        Log(std::format("FATAL exception: {}", e.what()));
        return TRUE;
    }

    return TRUE;
}
