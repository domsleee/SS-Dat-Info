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
void HandleG();
void HandleS();
void HandleM(bool isShiftDown);
void SetSpeedRectangleVisibility(bool showRectangle);
void SetupFunctionPointers();

constexpr int getKeyCode(char key) {
    return 10 + key - 'a';
}

const int KEY_F7 = 90;
const int KEY_M = getKeyCode('m');
const int KEY_G = getKeyCode('g');
const int KEY_S = getKeyCode('s');
const int KEY_SHIFT = 36;

// F1 = 84
// F2 = 85
// F7 = 90
static HMODULE supremeGameModule;
static HMODULE hmgSoundModule;
static HMODULE hmgCetsupDIModule;
static HMODULE srUitModule;
static HMODULE displayConfigHelperModule;

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
    displayConfigHelperModule = GetModuleHandleA("Display_Config_Helper.dll");
    if (!displayConfigHelperModule) {
        Log("DoCustomInput: Failed to find Display_Config_Helper.dll");
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
                if (i == KEY_S) {
                    HandleS();
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

void HandleF7(safetyhook::Context& ctx) {
    Log("Handle F7");

    Housemarque::Supreme_Snowboarding::Supreme::Set_AI_Learning_Mode((void*)ctx.ecx);
    Housemarque::Supreme_Snowboarding::Supreme::Set_Replay_Mode((void*)ctx.ecx);

    // Press "C"
    void *cameraPtr = FUN_1013e410(); // expected: 0x02AFD9D8
    int iVar5 = *(int*)((char*)(cameraPtr) + 0x144); // expected: 0CA7AE98
    if (iVar5 != 0) {
        *(int*)(iVar5 + 0x68) = (int)(*(int*)((char*)iVar5 + 0x68) == 0);
    }
}

void HandleS() {
    GlobalState::replaySpeedVisible = !GlobalState::replaySpeedVisible;
	SetSpeedRectangleVisibility(GlobalState::replaySpeedVisible);
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
