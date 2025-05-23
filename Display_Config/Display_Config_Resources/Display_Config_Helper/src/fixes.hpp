#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include "globalState.hpp"

#pragma once
float fNewCameraHFOV;
void DoFovFix(int fovWidth, int fovHeight) {
    auto module = GetModuleHandleA("Supreme_Game.dll");
    if (!module) {
        Log("Failed to find Supreme_Game.dll");
        return;
    }

    std::uint8_t* CameraHFOVInstructionScanResult = Memory::PatternScan(module, "8D 94 24 A8 02 00 00 52 8D 94 24 D0 02 00 00 8D 8C 24 AC 02 00 00");

    if (CameraHFOVInstructionScanResult) {
        Log(std::format("Camera HFOV Instruction: Address is Supreme_Game.dll+{:x}", CameraHFOVInstructionScanResult + 0x8 - (std::uint8_t*)module));

        fNewCameraHFOV = ((float)fovWidth / (float)fovHeight) / (4.0f / 3.0f);

        static SafetyHookMid CameraHFOVInstructionMidHook{};
        Log("DoFovFix: Applying FOV fix");
        CameraHFOVInstructionMidHook = safetyhook::create_mid(CameraHFOVInstructionScanResult + 0x8, [](SafetyHookContext& ctx)
        {
            *reinterpret_cast<float*>(ctx.esp + 0x2D0) = fNewCameraHFOV;
        });
        Log("DoFovFix: FOV fix applied");
    }
    else {
        Log("DoFovFix: Couldn't find");
    }
}

void Do4xFont() {
    auto module = GetModuleHandleA("HMG_3DE.dll");
    if (!module) {
        Log("Failed to find HMG_3DE.dll");
        return;
    }

    std::uint8_t* getTextureScanResult = Memory::PatternScan(module, "85 C0 74 04 89 44 24 24 8D 54 24 20 52 8B CF FF 15 10 53");
    if (getTextureScanResult) {
        Log(std::format("Do4xFont: Address is HMG_3DE.dll+{:x}", getTextureScanResult + 0x8 - (std::uint8_t*)module));

        static SafetyHookMid fontMidHook{};
        Log("Do4xFont: Applying 4x font fix");
        fontMidHook = safetyhook::create_mid(getTextureScanResult + 0x8, [](SafetyHookContext& ctx)
        {
            if (ctx.eax == 32 && *(uint32_t*)(ctx.esp + 0x20) == 32)
            {
                ctx.eax = 128;
                *(uint32_t*)(ctx.esp + 0x20) = 128;
                *(uint32_t*)(ctx.esp + 0x24) = 128;
            }
        });
        Log("Do4xFont: 4x font fix applied");
    }
    else {
        Log("Do4xFont: Couldn't find program text");
    }
}

void DoMakeGhostsOpaque() {
    auto module = GetModuleHandleA("Supreme_Game.dll");
    if (!module) {
        Log("Failed to find Supreme_Game.dll");
        return;
    }

    std::uint8_t* createGhostMemoryScanResult = Memory::PatternScan(module, "8B 46 04 85 C0 57 74 0C 8B 4E 08 2B C8 C1 F9 02");
    if (createGhostMemoryScanResult) {
        Log(std::format("DoGhostOpaqueFix: Address is Supreme_Game.dll+{:x}", createGhostMemoryScanResult - (std::uint8_t*)module));

        Log("DoGhostOpaqueFix: Applying ghost opaque fix");
        static SafetyHookMid createGhostMemoryMidHook = safetyhook::create_mid(createGhostMemoryScanResult, [](SafetyHookContext& ctx)
        {
            if (GlobalState::ghostsOpaque) {
                *(float *)(ctx.esp + 0x28) = 1.0;
            }
        });
        Log("DoGhostOpaqueFix: Ghost opaque fix applied");
    }
    else {
        Log("DoGhostOpaqueFix: Couldn't find create ghost memory");
    }
}

void EnableLogging() {
    auto module = GetModuleHandleA("HMG_Kernel.dll");
    if (!module) {
        Log("Failed to find HMG_Kernel.dll");
        return;
    }

    typedef void (__fastcall *ToggleLoggingFunc)(bool);
    ToggleLoggingFunc toggleLogging = (ToggleLoggingFunc)GetProcAddress(module, "?Toggle_Logging@Kernel@Housemarque@@YIX_N@Z");

    if (toggleLogging) {
        toggleLogging(true);
        Log("EnableLogging: Logging enabled");
    } else {
        Log("EnableLogging: Failed to find Toggle_Logging function");
    }
}

