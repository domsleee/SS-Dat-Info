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
#include "caves/menu_state.hpp"
#include "level_scan.hpp"

static TasSharedMemory g_sharedMem;
static GameAddresses g_addr;
static volatile LONG g_initState = 0; // 0=not started, 1=running, 2=ready, 3=failed

bool run() {
    Log("=== TAS_Helper.dll loading (Phase 2) ===");
    Log(std::format("  sizeof(TasSharedState) = {}", sizeof(TasSharedState)));

    // Resolve and validate the exact game build before creating the readiness
    // signal or changing any game code.
    if (!g_addr.Resolve()) {
        Log("FATAL: Failed to resolve/validate game addresses");
        return false;
    }

    if (!g_sharedMem.Create()) {
        Log("FATAL: Failed to create shared memory mapping");
        return false;
    }
    Log(std::format("Shared memory '{}' created ({} bytes)",
        TAS_SHARED_MEMORY_NAME, sizeof(TasSharedState)));

    auto* state = g_sharedMem.state;

    // Step 3: Install hooks
    // Order matters: Cave 1D (BB3B10 gate) must be installed before Cave 2
    // because Cave 2 calls BB3B10 directly. Cave 1C (handler gate) must be
    // installed before Cave 2 for the same reason.
    bool replay_ok = InstallReplayCapture(g_addr, state);
    bool cave1d_ok = InstallCave1D(g_addr, state);
    bool cave1c_ok = InstallCave1C(g_addr, state);
    bool cave2_ok = InstallCave2(g_addr, state);
    // TAS_NO_CAVE5=1 skips the tick-override hook. An A/B switch in the same
    // spirit as TAS_NO_LEVELSCAN / TAS_NO_RACETIMER, added because cave5's site
    // (EXE+0x25C81) sits inside the POST-LEVEL MENU dispatcher, not only in
    // gameplay — so it is the prime suspect for the menu video misbehaving only
    // after a level round-trip.
    char nc5[8] = {};
    bool cave5_ok = false;
    bool cave5_skipped =
        GetEnvironmentVariableA("TAS_NO_CAVE5", nc5, sizeof(nc5)) > 0 && nc5[0] == '1';
    if (cave5_skipped) {
        Log("  Cave 5: SKIPPED (TAS_NO_CAVE5=1)");
    } else {
        cave5_ok = InstallCave5(g_addr, state);
    }

    // These hooks are one functional unit. Reporting ready after any of them
    // failed leaves a partially intercepted input/game loop in production and
    // makes Injector.exe's explicit initialization result meaningless. Roll
    // back in reverse dependency order while shared state is still mapped.
    bool core_ok = replay_ok && cave1d_ok && cave1c_ok && cave2_ok
        && (cave5_skipped || cave5_ok);
    if (!core_ok) {
        Log("FATAL: required TAS hook installation failed; rolling back all core hooks");
        UninstallCave5();
        UninstallCave2();
        UninstallCave1C();
        UninstallCave1D();
        UninstallReplayCapture();
        g_sharedMem.Destroy();
        return false;
    }
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
    Log(std::format("  Cave 5  (fixed tick):        {}",
        cave5_skipped ? "SKIPPED" : (cave5_ok ? "OK" : "FAILED")));
    Log(std::format("  FrameLimit (SwapBuffers):    {}",
        framelimit_ok ? "OK" : "off (default — see TAS_FRAMELIMIT)"));

    // Background thread: detect the current track via an in-process heap scan.
    // Gated by TAS_NO_LEVELSCAN=1 to A/B whether the scan thread perturbs
    // replay determinism. NOTE: this worker is also the default out-of-cycle
    // STOP consumer (cave2 TryProcessStopCommand); without it a STOP sent at a
    // menu is only acknowledged once a level's Supreme::Cycle runs again, and
    // tas_ui refuses undo/redo/load until then.
    char nls[8] = {};
    if (GetEnvironmentVariableA("TAS_NO_LEVELSCAN", nls, sizeof(nls)) > 0 && nls[0] == '1') {
        Log("  Level scan thread: SKIPPED (TAS_NO_LEVELSCAN=1)");
    } else {
        // cave2's cycle heartbeat goes in as well: it is the only signal that
        // notices a return to the menu (nothing else changes there — see
        // level_scan.hpp's cycleFrozen).
        levelscan::Start(state, (uint32_t)g_addr.level_path_ptr, &SafeReadPtr,
                         &g_lastCycleMs, (uint32_t)g_addr.player_base);
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
        if (racetimer::Install(g_addr, state)) {
            Log("  Race timer: started");
        } else {
            Log("  Race timer: unavailable");
        }
    }

    // Menu state (which screen the game is on) - reads the Main_Menu.dll menu
    // OBJECT via a Change_Page hook, deferred until that DLL loads. Independent
    // of the race timer; publishes into the same menu_screen field.
    menustate::Install(g_addr, state);

    Log(std::format("  Renderer plugin at init: {} (x87 control word is sampled on the game thread; see level-scan log lines)",
        renderer::Name(renderer::Detect())));
    Log("=== TAS_Helper.dll ready (Phase 2) ===");
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
