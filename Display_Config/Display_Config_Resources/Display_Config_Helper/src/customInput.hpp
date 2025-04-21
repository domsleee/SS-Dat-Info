#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include <string>
#include <regex>
#include <ctime>

#pragma once
bool CheckKeyState(void** keyboardPtr, int keyValue);
void HandleF7();
void HandleM(bool isShiftDown);
void SetupFunctionPointers();

const int KEY_F7 = 90;
const int KEY_M = 10 + ('m' - 'a');
const int KEY_SHIFT = 36;

// F1 = 84
// F2 = 85
// F7 = 90
static HMODULE supremeGameModule;
static HMODULE hmgSoundModule;
static HMODULE hmgCetsupDIModule;

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
                    HandleF7();
                }
                if (i == KEY_M) {
                    HandleM(CheckKeyState(keyboardPtr, KEY_SHIFT));
                }
            }
            keyStates[i] = isKeyDown;
        }
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

void HandleF7() {
    Log("Handle F7");
    // Press "C"
    typedef void** (*FUN_1013e410_t)(void);
    FUN_1013e410_t FUN_1013e410 = (FUN_1013e410_t)((char*)supremeGameModule + 0x13e410);
    void **cameraPtr = FUN_1013e410(); // expected: 0x02AFD9D8
    int iVar5 = *(int*)((char*)(cameraPtr) + 0x144); // expected: 0CA7AE98
    if (iVar5 != 0) {
        *(int*)(iVar5 + 0x68) = (int)(*(int*)((char*)iVar5 + 0x68) == 0);
    }

    // AI LEARNING - TODO.
    //typedef bool (Set_AI_Learning_Mode_t)(void);
    //Set_AI_Learning_Mode_t* Set_AI_Learning_Mode = (Set_AI_Learning_Mode_t*)((std::uint8_t*)supremeGameModule + 0x142130);
    //Set_AI_Learning_Mode();
}

bool CheckKeyState(void** keyboardPtr, int keyValue) {
    void** vtable = *(void***)keyboardPtr;
    typedef bool(__fastcall* Supreme_Keyboard_State_t)(void* thisPtr, int keyValue);
    Supreme_Keyboard_State_t Supreme_Keyboard_State = (Supreme_Keyboard_State_t)vtable[5]; // 5 * 4 = 20 = 0x14.
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
}
