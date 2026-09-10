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
namespace playerhandler { static void Install(); }
void HandleG();
void HandleM(bool isShiftDown);
void SetupFunctionPointers();

constexpr int getKeyCode(char key) {
    return 10 + key - 'a';
}

const int KEY_F7 = 90;
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

        for (int i = 0; i <= 92; ++i) {
            auto isKeyDown = CheckKeyState(keyboardPtr, i);
            if (isKeyDown && isKeyDown != keyStates[i]) {
                Log(std::format("DoCustomInput: Key {} pressed!", i));
                if (i == KEY_F7) {
                    HandleF7(ctx);
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


    playerhandler::Install();

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

// F7 = replay.
//
// Set_Replay_Mode hands the player handler to the replay controller, which
// plays the Ghost_Players in the handler's ghost list: the guide rider / TOP5
// ghosts loaded with the level, or the human run once the finish turned it
// into a ghost for the auto replay. With that list empty - a fresh spawn, a
// new run after F5 - there is nothing to play and the replay start throws
// the game's fatal "String literal" kernel error from a later cycle (a
// try/catch around the calls does not see it; measured 2026-09-10/11). So
// F7 is refused unless the ghost list has an entry.
//
// The player handler is captured from Player_Handler::Set_Controller
// (SG+0x942F0), which Set_Game_Mode calls at every level start and F5
// restart (the exe's own path). Its lists are FLIT trees (HMG_HTL_2): every
// object embeds its node at +4, a node is {succ, pred, parent, child_head,
// child_tail, child_tailpred}, and an empty list points its child_head at
// its own child_tail slot (layout confirmed live 2026-09-11).
namespace playerhandler {
    const size_t NODE_OFFSET = 0x4;        // object -> its FLIT node
    const size_t NODE_CHILD_HEAD = 0xC;
    const size_t NODE_CHILD_TAIL = 0x10;   // the sentinel an empty head points at
    const size_t GHOST_LIST_OFFSET = 0x20; // handler -> list object

    const uintptr_t LEVEL_ROOT_RVA = 0x1D5450; // reallocated on level teardown, kept across F5

    static void* g_handler = nullptr;
    static uint32_t g_handlerRoot = 0;      // the level the handler belongs to
    static safetyhook::MidHook g_setControllerHook{};

    static uint32_t LevelRoot() {
        __try {
            return *(uint32_t*)((uint8_t*)supremeGameModule + LEVEL_ROOT_RVA);
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return 0;
        }
    }

    static void Install() {
        g_setControllerHook = safetyhook::create_mid((void*)((uint8_t*)supremeGameModule + 0x942f0),
            [](safetyhook::Context& ctx) { g_handler = (void*)ctx.ecx; g_handlerRoot = LevelRoot(); });
        Log("DoCustomInput: Player_Handler::Set_Controller hook installed");
    }

    // SEH leaf: 1 when the handler's ghost list has an entry, 0 when it is
    // empty, -1 when no handler was seen for the current level (the capture
    // is stamped with the level root, so a handler freed with its level is
    // never read) or the chain cannot be read.
    static int GhostsLoaded() {
        __try {
            uint8_t* handler = (uint8_t*)g_handler;
            if (!handler) return -1;
            const uint32_t root = LevelRoot();
            if (!root || root != g_handlerRoot) return -1;
            uint8_t* list = *(uint8_t**)(handler + GHOST_LIST_OFFSET);
            if (!list) return -1;
            uint8_t* node = list + NODE_OFFSET;
            uint8_t* head = *(uint8_t**)(node + NODE_CHILD_HEAD);
            if (!head) return -1;
            return head != node + NODE_CHILD_TAIL ? 1 : 0;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return -1;
        }
    }
}

static bool F7ReplayAvailable() {
    const int ghosts = playerhandler::GhostsLoaded();
    if (ghosts == 1) return true;
    Log(ghosts == 0 ? "F7: nothing to replay (no ghost loaded, no finished run) - ignored"
                    : "F7: player handler not seen yet - ignored");
    return false;
}

// The finish results.
//
// The results overlay is not part of Supreme_Game. The hiscore table is the
// Main_Menu page ID_IN_GAME_RESULTS, driven by Supreme.exe: the exe keeps a
// Game object at [[exe+0x889C4]+0x30] whose byte +0x10 is "results active";
// the finish handler (exe+0x26DA0) sets it to 1 and Game::Reset (exe+0x24F40,
// every restart) sets it back to 0, and the main loop paints the page only
// while the game is in replay mode and that byte is set. The "press space to
// hide the results" line is a HUD text line Supreme_Game refreshes every
// frame from the std::string at [game+0x34]+0xD4 (game = the Supreme_Game
// singleton FUN_1013e410 returns), which the exe assigns "" the moment the
// byte drops. The overlay staying up over an F7 replay is just that byte
// still being 1 from the finish, and the line showing over a ghost replay
// started before any finish is the string never having been blanked - so F7
// does both the way the exe does: clear the byte, blank the string. (Static
// RE of the decompiled exe + Supreme_Game, live checks 2026-09-10/11.)
namespace results {
    const uintptr_t APP_POINTER_RVA = 0x889C4;
    const uintptr_t GAME_VTABLE_RVA = 0x6D6EC;
    const size_t GAME_OFFSET_IN_APP = 0x30;
    const size_t RESULTS_ACTIVE_OFFSET = 0x10;
    const size_t START_INFO_OFFSET = 0x34;
    const size_t PROMPT_STRING_OFFSET = 0xD4;

    // SEH leaf: the exe's Game object, validated by its vtable; null if the
    // chain is unreadable or points somewhere else (level teardown nulls it).
    static uint8_t* GameObject() {
        __try {
            uint8_t* exe = (uint8_t*)GetModuleHandleA(nullptr);
            uint8_t* app = *(uint8_t**)(exe + APP_POINTER_RVA);
            if (!app) return nullptr;
            uint8_t* game = *(uint8_t**)(app + GAME_OFFSET_IN_APP);
            if (!game) return nullptr;
            if (*(uint8_t**)game != exe + GAME_VTABLE_RVA) return nullptr;
            return game;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return nullptr;
        }
    }

    // SEH leaf: 1 when the page was up and is now down, 0 when it was not
    // up, -1 when the exe's Game object is not there.
    static int ClearPageFlag() {
        __try {
            uint8_t* game = GameObject();
            if (!game) return -1;
            if (!game[RESULTS_ACTIVE_OFFSET]) return 0;
            game[RESULTS_ACTIVE_OFFSET] = 0;
            return 1;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return -1;
        }
    }

    // The exe's own std::string::assign(const char*, size) from MSVCP60 - the
    // routine the exe itself blanks the string with (MSVC6 strings share
    // reference-counted buffers, so the string is never written directly).
    typedef void* (__fastcall* StringAssign_t)(void* self, void* unusedEdx, const char* text, unsigned length);
    static StringAssign_t g_stringAssign = nullptr;

    static StringAssign_t StringAssign() {
        if (g_stringAssign) return g_stringAssign;
        HMODULE msvcp = GetModuleHandleA("MSVCP60.dll");
        if (!msvcp) {
            Log("F7: MSVCP60.dll not loaded - replay instructions line left as is");
            return nullptr;
        }
        g_stringAssign = (StringAssign_t)GetProcAddress(msvcp,
            "?assign@?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@QAEAAV12@PBDI@Z");
        if (!g_stringAssign) Log("F7: MSVCP60 basic_string::assign(const char*, size) export not found");
        return g_stringAssign;
    }

    // SEH leaf: the prompt string object, checked to look like an MSVC6
    // std::string {alloc, ptr, len, res} holding text; null when unreadable,
    // empty or not recognised.
    static void* PromptStringHoldingText() {
        __try {
            uint8_t* startInfo = *(uint8_t**)((uint8_t*)FUN_1013e410() + START_INFO_OFFSET);
            if (!startInfo) return nullptr;
            uint32_t* str = (uint32_t*)(startInfo + PROMPT_STRING_OFFSET);
            const char* text = (const char*)str[1];
            const uint32_t len = str[2];
            if (!text || len == 0 || len > 256) return nullptr;
            size_t actual = 0;
            while (actual < len && text[actual]) actual++;
            return actual == len ? str : nullptr;
        } __except (EXCEPTION_EXECUTE_HANDLER) {
            return nullptr;
        }
    }

    // Blanks the prompt string the way the exe does. 1 when it held text and
    // was blanked, 0 when there was nothing to blank, -1 when the runtime
    // routine is not available.
    static int BlankPrompt() {
        void* str = PromptStringHoldingText();
        if (!str) return 0;
        StringAssign_t assign = StringAssign();
        if (!assign) return -1;
        assign(str, nullptr, "", 0);
        return 1;
    }

    // Takes the results down like a restart does. Safe when nothing is up.
    static void Hide() {
        const int page = ClearPageFlag();
        const int prompt = BlankPrompt();
        if (page == 1) Log("F7: results page hidden");
        if (page == -1) Log("F7: exe Game object not found - results page left as is");
        if (prompt == 1) Log("F7: replay instructions line blanked");
        if (prompt == -1) Log("F7: MSVCP60 string assign not found - replay instructions line left as is");
    }
}

void HandleF7(safetyhook::Context& ctx) {
    Log("Handle F7");
    if (!F7ReplayAvailable()) return;
    void* supreme = (void*)ctx.ecx;

    Housemarque::Supreme_Snowboarding::Supreme::Set_AI_Learning_Mode(supreme);
    Housemarque::Supreme_Snowboarding::Supreme::Set_Replay_Mode(supreme);
    results::Hide();

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
