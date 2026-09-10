#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include "globalState.hpp"
#include <string>
#include <regex>
#include <ctime>

#pragma once
bool CheckKeyState(void** keyboardPtr, int keyValue);
void HandleF7(safetyhook::Context& ctx);
namespace hideresults {
    static void TrackMode(void* supreme);
    static void Pump();
    static void OnSpaceSeen();
}
static void TrackLevelRoot();
void HandleG();
void HandleM(bool isShiftDown);
void SetupFunctionPointers();

constexpr int getKeyCode(char key) {
    return 10 + key - 'a';
}

const int KEY_F7 = 90;
const int KEY_SPACE = 49;
const int KEY_M = getKeyCode('m');
const int KEY_G = getKeyCode('g');
const int KEY_SHIFT = 36;

// F1 = 84
// F2 = 85
// F7 = 90
static HMODULE supremeGameModule;
static HMODULE hmgSoundModule;
static HMODULE hmgCetsupDIModule;
static HMODULE srUitModule;

namespace Housemarque::Supreme_Snowboarding::Music_Handler {
    typedef void (*voidFunction)();
    voidFunction Play_Slope_Music;
    voidFunction Play_Prev_Slope_Music;
}

namespace Housemarque::Game_Construction_Kit::Sound_System {
    typedef void (*Set_Master_Volume_Streams_t)(float);
    Set_Master_Volume_Streams_t Set_Master_Volume_Streams;
    typedef void (*Set_Master_Volume_Samples_t)(float);
    Set_Master_Volume_Samples_t Set_Master_Volume_Samples;

    typedef float (*Get_Master_Volume_Streams_t)();
    Get_Master_Volume_Streams_t Get_Master_Volume_Streams;
    typedef float(*Get_Master_Volume_Samples_t)();
    Get_Master_Volume_Samples_t Get_Master_Volume_Samples;
}

namespace Housemarque::Supreme_Snowboarding::Supreme {
    typedef bool(__fastcall* Set_Replay_Mode_t)(void*);
    Set_Replay_Mode_t Set_Replay_Mode;

    typedef bool(__fastcall* Set_AI_Learning_Mode_t)(void*);
    Set_AI_Learning_Mode_t Set_AI_Learning_Mode;
}

namespace Housemarque::SR_UIT::Sr_Plane_Text_Line {
    typedef void (__fastcall* Hide_t)(void*);
    Hide_t Hide;
}

namespace Housemarque::Supreme_Snowboarding::Supreme_Keyboard {
    typedef bool(__fastcall* State_t)(void* thisPtr, int keyValue);
}

typedef void* (*FUN_1013e410_t)(void);
FUN_1013e410_t FUN_1013e410;

void DoCustomInput() {
    supremeGameModule = GetModuleHandleA("Supreme_Game.dll");
    if (!supremeGameModule) {
        Log("DoCustomInput: Failed to find Supreme_Game.dll");
        return;
    }
    hmgSoundModule = GetModuleHandleA("HMG_Sound.dll");
    if (!hmgSoundModule) {
        Log("DoCustomInput: Failed to find HMG_Sound.dll");
        return;
    }
    hmgCetsupDIModule = GetModuleHandleA("HMG_Cetsup_DI.dll");
    if (!hmgCetsupDIModule) {
        Log("DoCustomInput: Failed to find HMG_Cetsup_DI.dll");
        return;
    }
    srUitModule = GetModuleHandleA("SR_UIT.dll");
    if (!srUitModule) {
        Log("DoCustomInput: Failed to find SR_UIT.dll");
        return;
    }

    SetupFunctionPointers();

    std::uint8_t* FUN_10140650Address = Memory::PatternScan(supremeGameModule, "83 C4 0C C3 90 51 A0 93 53");
    if (!FUN_10140650Address) {
        Log("DoCustomInput: Couldn't find FUN_10140650Address");
        return;
    }
    FUN_10140650Address += 0x5;

    Log(std::format("DoCustomInput: Address is Supreme_Game.dll+{:x}", reinterpret_cast<std::uintptr_t>(FUN_10140650Address) - reinterpret_cast<std::uintptr_t>(supremeGameModule)));

    static std::vector<bool> keyStates(100, false);
    static safetyhook::MidHook saveReplayHook{};
    saveReplayHook = safetyhook::create_mid(FUN_10140650Address, [](safetyhook::Context& ctx) {
        typedef void** (*FUN_100d0b80_t)(void);
        FUN_100d0b80_t FUN_100d0b80 = (FUN_100d0b80_t)((std::uint8_t*)supremeGameModule + 0xd0b80);
        void** keyboardPtr = FUN_100d0b80();
        TrackLevelRoot();
        hideresults::TrackMode((void*)ctx.ecx);
        hideresults::Pump();

        for (int i = 0; i <= 92; ++i) {
            auto isKeyDown = CheckKeyState(keyboardPtr, i);
            if (isKeyDown && isKeyDown != keyStates[i]) {
                Log(std::format("DoCustomInput: Key {} pressed!", i));
                if (i == KEY_F7) {
                    HandleF7(ctx);
                }
                if (i == KEY_SPACE) {
                    hideresults::OnSpaceSeen();
                }
                if (i == KEY_M) {
                    HandleM(CheckKeyState(keyboardPtr, KEY_SHIFT));
                }
                if (i == KEY_G) {
                    HandleG();
                }
            }
            keyStates[i] = isKeyDown;
        }
    });


    static safetyhook::MidHook aiHeuristicHook;
    aiHeuristicHook = safetyhook::create_mid((void*)((std::uint8_t*)supremeGameModule + 0x12536b), [](safetyhook::Context& ctx) {
        void* textLine = *(void**)((char*)ctx.esi + 0x4);
        Housemarque::SR_UIT::Sr_Plane_Text_Line::Hide(textLine);
    });
    Log("DoCustomInput: Fix applied");
}

const float DEFAULT_SOUND_ON_VOLUME = 0.75f;
void HandleM(bool isShiftDown) {
    Log(std::format("HandleM %b", isShiftDown));
    if (isShiftDown) {
        // mute snowboard sounds
        const float current = Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Samples();
        static float samplesOnVolume = current != 0.00f ? current : DEFAULT_SOUND_ON_VOLUME;
        Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Samples(current == 0.00f ? samplesOnVolume : 0.00f);
    }
    else {
        // mute music
        const float current = Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Streams();
        static float streamsOnVolume = current != 0.00f ? current : DEFAULT_SOUND_ON_VOLUME;
        const float newValue = current == 0.00f ? streamsOnVolume : 0.00f;
        Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Streams(newValue);
        if (newValue == 0.00) {
            Housemarque::Supreme_Snowboarding::Music_Handler::Play_Prev_Slope_Music();
        } else {
            Housemarque::Supreme_Snowboarding::Music_Handler::Play_Slope_Music();
        }
    }
}

// Whether F7 has a replay to play.
//
// Set_Replay_Mode feeds the game's replay parser from the run written at
// the finish line; before a finish in the current level there is nothing to
// parse and the parser throws the game's "String literal must begin (and
// end) with a '\"'" kernel error - the modal that ends the session. The
// finish is observed where the game writes that run (saveReplayTimestamp.hpp
// sets GlobalState::replayReady); a new level (TrackLevelRoot) or leaving the
// replay for a new run (hideresults::TrackMode) clears it.
static bool F7ReplayAvailable() {
    if (!GlobalState::replayReady) {
        Log("F7: no finished run in this level to replay - ignored");
        return false;
    }
    return true;
}

// The level root at Supreme_Game+0x1D5450 is reallocated when a level is
// loaded (it survives F5 restarts). A new root means the finished run that
// was replayable belongs to a level that is gone.
static uint32_t g_lastLevelRoot = 0;

static uint32_t ReadLevelRoot() {
    __try {
        return *(uint32_t*)((char*)supremeGameModule + 0x1D5450);
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return 0;
    }
}

static void TrackLevelRoot() {
    uint32_t root = ReadLevelRoot();
    if (root == g_lastLevelRoot) return;
    g_lastLevelRoot = root;
    GlobalState::replayReady = false;
    GlobalState::resultsVisible = false;
}

// Set_Replay_Mode on a run the game cannot parse throws a C++ exception the
// game's own top level turns into the fatal dialog. Catching it here covers
// a throw from inside these two calls when the ready flag is wrong (a replay
// invalidated by something this DLL does not see): the game's frames unwind
// normally, the press is ignored, and the reason is logged. It does not
// cover the game throwing later, from its own cycle, once a bad replay has
// been switched on - keeping the ready flag right (F7ReplayAvailable,
// TrackLevelRoot, hideresults::TrackMode) is what prevents the dialog.
static bool EnterReplayMode(void* supreme) {
    try {
        Housemarque::Supreme_Snowboarding::Supreme::Set_AI_Learning_Mode(supreme);
        Housemarque::Supreme_Snowboarding::Supreme::Set_Replay_Mode(supreme);
        return true;
    } catch (...) {
        Log("F7: the game refused to start the replay (no parsable run) - ignored");
        GlobalState::replayReady = false;
        return false;
    }
}

// Results-overlay dismissal.
//
// After a finish the game shows the results overlay ("Your time was ...",
// the hiscore table, "press space to hide the results") and it stays up over
// an F7 replay. The game itself takes it down on SPACE - but SPACE is a
// TOGGLE of the table while a replay runs (measured 2026-09-10), so the
// helper must know whether the table is up before pressing anything:
//   - up from the moment the game writes the finished run (saveReplayTimestamp
//     hook sets GlobalState::resultsVisible);
//   - flipped by every SPACE the game honours - the key loop sees exactly
//     those presses (for ~10 s after the auto replay starts the game ignores
//     every key, and the key loop sees nothing either);
//   - gone when the replay drops back to game mode (F5 rebuilds the HUD) or
//     the level changes.
// When F7 starts a replay with the table up, the helper presses SPACE the way
// the player would: WM_KEYDOWN/WM_KEYUP posted to the game window (the game's
// Win32 keyboard driver reads them; with DirectInput enabled they are ignored
// and the overlay simply stays). PostMessage is asynchronous, so the hook
// never re-enters the game's input path. The key is released as soon as the
// key loop sees the game's own keyboard report it down - the press has been
// honoured and the visibility flag flips with it - or after a short timeout,
// in which case nothing was toggled and the flag stays as it was.
namespace hideresults {
    constexpr DWORD HOLD_MS = 2000;     // release an unseen press after this
    const int MODE_REPLAY = 1;          // Supreme+0: Set_Replay_Mode writes 1, Set_Game_Mode 0

    static bool g_active = false;
    static DWORD g_startMs = 0;
    static HWND g_window = nullptr;
    static int g_lastMode = -1;

    struct WindowSearch { HWND anyVisible; HWND gameClass; };

    static BOOL CALLBACK FindGameWindowProc(HWND hwnd, LPARAM lParam) {
        DWORD pid = 0;
        GetWindowThreadProcessId(hwnd, &pid);
        if (pid != GetCurrentProcessId() || !IsWindowVisible(hwnd)) return TRUE;
        WindowSearch* search = (WindowSearch*)lParam;
        if (!search->anyVisible) search->anyVisible = hwnd;
        char className[64] = {};
        GetClassNameA(hwnd, className, sizeof(className));
        if (strcmp(className, "Supreme") == 0) {
            search->gameClass = hwnd;
            return FALSE;
        }
        return TRUE;
    }

    // The game's own top-level window (class "Supreme"), else any visible one.
    static HWND FindGameWindow() {
        WindowSearch search{};
        EnumWindows(FindGameWindowProc, (LPARAM)&search);
        return search.gameClass ? search.gameClass : search.anyVisible;
    }

    static void PostSpace(bool down) {
        const LPARAM scan = (LPARAM)(MapVirtualKeyA(VK_SPACE, MAPVK_VK_TO_VSC) << 16) | 1;
        if (down) PostMessageA(g_window, WM_KEYDOWN, VK_SPACE, scan);
        else PostMessageA(g_window, WM_KEYUP, VK_SPACE, scan | (1u << 30) | (1u << 31));
    }

    // SEH leaf: the Supreme object's mode word, -1 if unreadable.
    static int ReadMode(void* supreme) {
        __try {
            return *(int*)supreme;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return -1;
        }
    }

    // Called from the key loop every frame, before the keys are scanned.
    //
    // Leaving replay mode (F5 during a replay drops straight into a new run)
    // rebuilds the HUD, so the overlay is gone - and the finished run with it:
    // the new run takes over the recorder, and F7 on it throws the same
    // "String literal" kernel error as F7 before any finish (measured
    // 2026-09-10; the throw escapes the F7 handler's try/catch, so it has to
    // be refused up front). The next finish raises both flags again.
    static void TrackMode(void* supreme) {
        const int mode = ReadMode(supreme);
        if (mode == g_lastMode) return;
        if (g_lastMode == MODE_REPLAY && GlobalState::replayReady) {
            Log("F7: replay left - the finished run is gone until the next finish");
            GlobalState::replayReady = false;
            GlobalState::resultsVisible = false;
        }
        g_lastMode = mode;
    }

    // Called from the key loop when it sees a SPACE press (edge).
    static void OnSpaceSeen() {
        if (g_lastMode != MODE_REPLAY) return;
        GlobalState::resultsVisible = !GlobalState::resultsVisible;
        Log(std::format("F7: SPACE seen in the replay - results overlay now {}",
                        GlobalState::resultsVisible ? "up" : "hidden"));
        if (g_active) {
            Log(std::format("F7: results overlay dismissed after {} ms", GetTickCount() - g_startMs));
            PostSpace(false);
            g_active = false;
        }
    }

    // Called right after F7 has started a replay.
    static void Begin() {
        if (!GlobalState::resultsVisible) return;
        if (g_active) { PostSpace(false); g_active = false; }
        g_window = FindGameWindow();
        if (!g_window) {
            Log("F7: game window not found - results overlay left as is");
            return;
        }
        g_active = true;
        g_startMs = GetTickCount();
        PostSpace(true);
    }

    // Called from the key loop every frame.
    static void Pump() {
        if (!g_active) return;
        if (GetTickCount() - g_startMs < HOLD_MS) return;
        Log("F7: the game never saw the SPACE press - results overlay left as is");
        PostSpace(false);
        g_active = false;
    }
}

void HandleF7(safetyhook::Context& ctx) {
    Log("Handle F7");
    void* supreme = (void*)ctx.ecx;
    if (!F7ReplayAvailable()) return;
    if (!EnterReplayMode(supreme)) return;
    hideresults::Begin();

    // Press "C"
    void *cameraPtr = FUN_1013e410(); // expected: 0x02AFD9D8
    int iVar5 = *(int*)((char*)(cameraPtr) + 0x144); // expected: 0CA7AE98
    if (iVar5 != 0) {
        *(int*)(iVar5 + 0x68) = (int)(*(int*)((char*)iVar5 + 0x68) == 0);
    }
}

void HandleG() {
    GlobalState::ghostsOpaque = !GlobalState::ghostsOpaque;
}

bool CheckKeyState(void** keyboardPtr, int keyValue) {
    void** vtable = *(void***)keyboardPtr;
    Housemarque::Supreme_Snowboarding::Supreme_Keyboard::State_t Supreme_Keyboard_State = (Housemarque::Supreme_Snowboarding::Supreme_Keyboard::State_t)vtable[5]; // 5 * 4 = 20 = 0x14.
    auto isKeyDown = Supreme_Keyboard_State(keyboardPtr, keyValue);
    return isKeyDown;
}

void SetupFunctionPointers() {
    Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Streams = (Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Streams_t)((char*)hmgSoundModule + 0x5d90);
    Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Samples = (Housemarque::Game_Construction_Kit::Sound_System::Set_Master_Volume_Samples_t)((char*)hmgSoundModule + 0x5da0);
    Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Streams = (Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Streams_t)((char*)hmgSoundModule + 0x5db0);
    Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Samples = (Housemarque::Game_Construction_Kit::Sound_System::Get_Master_Volume_Samples_t)((char*)hmgSoundModule + 0x5dc0);

    Housemarque::Supreme_Snowboarding::Music_Handler::Play_Slope_Music = (Housemarque::Supreme_Snowboarding::Music_Handler::voidFunction)((char*)supremeGameModule + 0x108000);
    Housemarque::Supreme_Snowboarding::Music_Handler::Play_Prev_Slope_Music = (Housemarque::Supreme_Snowboarding::Music_Handler::voidFunction)((char*)supremeGameModule + 0x108040);

    Housemarque::Supreme_Snowboarding::Supreme::Set_Replay_Mode = (Housemarque::Supreme_Snowboarding::Supreme::Set_Replay_Mode_t)((char*)supremeGameModule + 0x1417e0);
    Housemarque::Supreme_Snowboarding::Supreme::Set_AI_Learning_Mode = (Housemarque::Supreme_Snowboarding::Supreme::Set_AI_Learning_Mode_t)((char*)supremeGameModule + 0x142130);

    Housemarque::SR_UIT::Sr_Plane_Text_Line::Hide = (Housemarque::SR_UIT::Sr_Plane_Text_Line::Hide_t)((char*)srUitModule + 0xf4f0);

    FUN_1013e410 = (FUN_1013e410_t)((char*)supremeGameModule + 0x13e410);
}
