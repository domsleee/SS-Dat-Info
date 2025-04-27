#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include <string>
#include <regex>
#include <windows.h>

// refer to Housemarque::Cetsup::DI_Driver::DI_Driver
// iVar1 = DirectInputCreateA(param_2,0x500,this + 0x44,0);
#define DIRECTINPUT_VERSION 0x0500
#include <dinput.h>

#pragma once

typedef HRESULT(WINAPI* DirectInputCreateA_t)(HINSTANCE, DWORD, LPDIRECTINPUTA*, LPUNKNOWN);
DirectInputCreateA_t Original_DirectInputCreateA = nullptr;

HRESULT WINAPI HookedDirectInputCreateA(HINSTANCE hinst, DWORD dwVersion, LPDIRECTINPUTA* ppDI, LPUNKNOWN punkOuter) {
    return S_FALSE;
}

void DoDisableDirectInput() {
    HMODULE hDinput = GetModuleHandleA("dinput.dll");
    if (!hDinput) {
        Log("DoDisableDirectInput: Failed to load dinput.dll");
        return;
    }

    const auto cetsupDIModule = GetModuleHandleA("HMG_Cetsup_DI.dll");
    if (!cetsupDIModule) {
        Log("DoDisableJoystick: Failed to find cetsupDIModule");
        return;
    }
    /*
    std::uint8_t* DI_DriverAddress = (std::uint8_t*)cetsupDIModule + 0x2fa4;
    static SafetyHookMid disableJoystickHook = safetyhook::create_mid(DI_DriverAddress, [](safetyhook::Context& ctx) {
        Log(std::format("DoDisableJoystick: ZERO IS SUCCESS: {}", ctx.eax));
    });
    */
    static SafetyHookMid hey = safetyhook::create_mid(
		(char*)cetsupDIModule + 0x2fa4,
		[](safetyhook::Context& ctx) {
            ctx.eax = 1;
		}
	);

    /*
    Original_DirectInputCreateA = (DirectInputCreateA_t)GetProcAddress(hDinput, "DirectInputCreateA");
    static SafetyHookInline dinputHook = safetyhook::create_inline(
        (BYTE*)Original_DirectInputCreateA,
        HookedDirectInputCreateA
    );*/
    Log("DoDisableDirectInput: Fix applied");
}
