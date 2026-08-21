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
#include "caves/frame_limit.hpp"
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
    // OFF BY DEFAULT — measured to cost more than it fixes.
    //
    // This hook exists for the "2x menu video" bug: tas_ui raised the system
    // timer to 1 ms, sr.dll's Sleep-based limiter stopped undershooting, and the
    // menu presented at ~68 fps instead of ~34. Capping presents fixed that.
    //
    // Two things have since been MEASURED (same process, same menu, 30s windows):
    //   1. The hook costs ~12 ms on EVERY menu frame merely by EXISTING. Not the
    //      throttle — cap=0 is just as slow — but the inline patch on
    //      gdi32!SwapBuffers itself. Native 48.8 ms; hooked 60.6 ms (cap=34) and
    //      59.9 ms (cap=0); hook skipped 48.5 ms, i.e. native restored.
    //   2. The 2x no longer reproduces. With tas_ui RUNNING and this hook off,
    //      the menu measures 48.4 ms — native, and smoother than native
    //      (p90 49.9 vs 66.4). Windows 10 2004+/11 made timeBeginPeriod
    //      PER-PROCESS, so tas_ui can no longer raise the game's timer at all.
    //      The original diagnosis was right for its OS and has been overtaken.
    //
    // So it now makes the menu ~25% SLOWER than doing nothing, to fix something
    // the OS already fixed. Kept rather than deleted — if the 2x ever returns,
    // set TAS_FRAMELIMIT=1; menu_fps_cap remains live-tunable.
    char fl[8] = {0};
    bool framelimit_ok = false;
    if (GetEnvironmentVariableA("TAS_FRAMELIMIT", fl, sizeof(fl)) > 0 && fl[0] == '1') {
        framelimit_ok = InstallFrameLimit(state);
        Log("  FrameLimit: ENABLED by TAS_FRAMELIMIT=1");
    } else {
        Log("  FrameLimit: off by default (costs ~12ms/menu-frame; 2x no longer reproduces)");
    }

    Log("=== Hook installation summary ===");
    Log(std::format("  Replay capture (SG+9E8F0):  {}", replay_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1D (BB3B10 gate):      {}", cave1d_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 1C (handler gate):      {}", cave1c_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 2  (Supreme::Cycle):    {}", cave2_ok ? "OK" : "FAILED"));
    Log(std::format("  Cave 5  (fixed tick):        {}", cave5_ok ? "OK" : "FAILED"));
    Log(std::format("  FrameLimit (SwapBuffers):    {}",
        framelimit_ok ? "OK" : "off (default — see TAS_FRAMELIMIT)"));

    if (!cave2_ok) {
        Log("CRITICAL: Cave 2 hook failed - TAS will not function");
    }
    if (!cave1d_ok || !cave1c_ok) {
        Log("WARNING: Gate hooks failed - input blocking won't work correctly");
    }

    // Background thread: detect the current track via an in-process heap scan.
    // Gated by TAS_NO_LEVELSCAN=1 to A/B whether the scan thread perturbs
    // replay determinism.
    char nls[8] = {};
    if (GetEnvironmentVariableA("TAS_NO_LEVELSCAN", nls, sizeof(nls)) > 0 && nls[0] == '1') {
        Log("  Level scan thread: SKIPPED (TAS_NO_LEVELSCAN=1)");
    } else {
        // cave2's cycle heartbeat goes in as well: it is the only signal that
        // notices a return to the menu (nothing else changes there — see
        // level_scan.hpp's cycleFrozen).
        levelscan::Start(state, (uint32_t)g_addr.level_path_ptr, &SafeReadPtr,
                         &g_lastCycleMs);
        if (levelscan::g_thread) {
            Log("  Level scan thread: started");
        } else {
            Log("  Level scan thread: FAILED TO START — level stays unresolved");
        }
    }

    // Race timer: read the exact on-screen race time (HUD/SR_UIT) → shared state.
    // Gated by TAS_NO_RACETIMER=1 so we can A/B whether its per-tick hooks
    // perturb replay determinism (the bucket lottery is sub-tick sensitive).
    char nrt[8] = {};
    if (GetEnvironmentVariableA("TAS_NO_RACETIMER", nrt, sizeof(nrt)) > 0 && nrt[0] == '1') {
        Log("  Race timer: SKIPPED (TAS_NO_RACETIMER=1)");
    } else {
        racetimer::Install(g_addr, state);
        Log("  Race timer: started");
    }

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
