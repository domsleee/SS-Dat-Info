#include <windows.h>
#include "external/safetyhook.hpp"
#include "helper.hpp"
#include "Log.hpp"
#include "globalState.hpp"
#include <string>
#include <regex>
#include <ctime>

#pragma once

static uintptr_t g_slot_1634C0 = 0; // *(slot) = function ptr, then call [slot]
static uintptr_t g_slot_1634C8 = 0;
static uintptr_t g_slot_1634D8 = 0;
static uintptr_t g_addr_1CE1C0 = 0;  // data address used by mov edi/esi, then scas/movs*
static uintptr_t g_addr_12DD20 = 0;  // data address used by mov edi/esi, then scas/movs*
static uintptr_t g_addr_1B8D44 = 0;
static uintptr_t g_slot_163738 = 0;
static uintptr_t g_speedJumpOutAddress = 0;

void SetSpeedRectangleVisibility(bool showRectangle);
void ShowSpeedAndHideBlinkingRFunction();
void ShowSpeedOnly();
void HideBlinkingRFunction();
void* GetFunctionAddress(bool hideBlinkingR, bool showReplaySpeed);
static void WriteJmpRel32(void* at, void* dst);

enum ReplayUISetting {
    ShowSpeedAndHideBlinkingR,
    HideBlinkingR,
    None
};

void DoShowSpeedAndHideBlinkingR(bool hideBlinkingR, bool showReplaySpeed) {
    Log(std::format("DoShowSpeedAndHideBlinkingR: Setting up with hideBlinkingR: {}, showReplaySpeed: {}", hideBlinkingR, showReplaySpeed));
    if (showReplaySpeed) {
        SetSpeedRectangleVisibility(true);
    }

    const HMODULE supremeGameModule = GetModuleHandleA("Supreme_Game.dll");
    if (!supremeGameModule) {
        Log("DoCustomInput: Failed to find Supreme_Game.dll");
        return;
    }

    g_slot_1634C0 = (uintptr_t)((char*)supremeGameModule + 0x1634C0);
    g_slot_1634C8 = (uintptr_t)((char*)supremeGameModule + 0x1634C8);
    g_slot_1634D8 = (uintptr_t)((char*)supremeGameModule + 0x1634D8);
    g_addr_1CE1C0 = (uintptr_t)((char*)supremeGameModule + 0x1CE1C0);
    g_addr_12DD20 = (uintptr_t)((char*)supremeGameModule + 0x12DD20);
    g_addr_1B8D44 = (uintptr_t)((char*)supremeGameModule + 0x1B8D44);
    g_slot_163738 = (uintptr_t)((char*)supremeGameModule + 0x163738);
    g_speedJumpOutAddress = (uintptr_t)((char*)supremeGameModule + 0x46912);

    uint8_t* const jmpAddr = (uint8_t*)((char*)supremeGameModule + 0x45CD3);
    if (jmpAddr[0] == 0xE9) {
        // assert the jump is to g_speedJumpOutAddress
		if (*(uintptr_t*)(jmpAddr + 1) != g_speedJumpOutAddress - (uintptr_t)(jmpAddr + 5)) {
			Log("DoShowSpeedAndHideBlinkingR: The jump at 0x45CD3 does not point to the expected g_speedJumpOutAddress.");
			return;
		}
    }
    else {
        Log("DoShowSpeedAndHideBlinkingR: Expected jmp instruction at 0x45CD3, but found something else.");
        return;
    }

    const auto funcAddress = GetFunctionAddress(hideBlinkingR, showReplaySpeed);

    WriteJmpRel32(jmpAddr, funcAddress);
}

void* GetFunctionAddress(bool hideBlinkingR, bool showReplaySpeed) {
    if (hideBlinkingR && showReplaySpeed) {
        return &ShowSpeedAndHideBlinkingRFunction;
    }
	if (hideBlinkingR) return &HideBlinkingRFunction;
	if (showReplaySpeed) return &ShowSpeedOnly;
	Log("GetFunctionAddress: No function address found for the given parameters.");
}

__declspec(naked) void ShowSpeedOnly() {
    __asm {
        /*Supreme_Game.dll + 45D49 */ push 1
        /*Supreme_Game.dll + 45D4B */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D4E */ mov  dword ptr [ebp - 4], 0FFFFFFFFh

        /*Supreme_Game.dll + 45D55 */ mov  eax, dword ptr [g_slot_1634C0]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D5B */ mov  eax, [ebp - 30h]
        /*Supreme_Game.dll + 45D5E */ push eax
        /*Supreme_Game.dll + 45D5F */ lea  ecx, [ebp - 00000158h]
        /*Supreme_Game.dll + 45D65 */ push dword ptr [g_addr_1B8D44]
        /*Supreme_Game.dll + 45D6A */ push ecx
        /*Supreme_Game.dll + 45D6B */ mov  eax, dword ptr [g_slot_163738]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D71 */ mov  dl, [ebp - 0Dh]
        /*Supreme_Game.dll + 45D74 */ add  esp, 0Ch

        /*Supreme_Game.dll + 45D77 */ push 0
        /*Supreme_Game.dll + 45D79 */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D7C */ mov  [ebp - 20h], dl
        /*Supreme_Game.dll + 45D7F */ mov  eax, dword ptr [g_slot_1634C0]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D85 */ or   ecx, -1
        /*Supreme_Game.dll + 45D88 */ xor  eax, eax
        /*Supreme_Game.dll + 45D8A */ lea  edi, [ebp - 00000158h]
        /*Supreme_Game.dll + 45D90 */ repne scasb
        /*Supreme_Game.dll + 45D92 */ not  ecx
        /*Supreme_Game.dll + 45D94 */ dec  ecx
        /*Supreme_Game.dll + 45D95 */ push 1
        /*Supreme_Game.dll + 45D97 */ mov  [ebp - 30h], ecx
        /*Supreme_Game.dll + 45D9A */ push ecx
        /*Supreme_Game.dll + 45D9B */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D9E */ mov  eax, dword ptr [g_slot_1634C8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45DA4 */ test al, al
        /*Supreme_Game.dll + 45DA6 */ je   L_45DD1

        /*Supreme_Game.dll + 45DA8 */ mov  eax, [ebp - 30h]
        /*Supreme_Game.dll + 45DAB */ mov  edi, [ebp - 1Ch]
        /*Supreme_Game.dll + 45DAE */ mov  ecx, eax
        /*Supreme_Game.dll + 45DB0 */ mov  edx, ecx
        /*Supreme_Game.dll + 45DB2 */ shr  ecx, 2
        /*Supreme_Game.dll + 45DB5 */ lea  esi, [ebp - 00000158h]
        /*Supreme_Game.dll + 45DBB */ repe movsd
        /*Supreme_Game.dll + 45DBD */ mov  ecx, edx
        /*Supreme_Game.dll + 45DBF */ and  ecx, 3
        /*Supreme_Game.dll + 45DC2 */ repe movsb
        /*Supreme_Game.dll + 45DC4 */ push eax
        /*Supreme_Game.dll + 45DC5 */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45DC8 */ mov  eax, dword ptr [g_slot_1634D8]
        /*                         */ call dword ptr [eax]
        /*Supreme_Game.dll + 45DCE */ mov  esi, [ebp - 24h]

L_45DD1:
        /*Supreme_Game.dll + 45DD1 */ mov  eax, [ebx + 28h]
        /*Supreme_Game.dll + 45DD4 */ mov  ecx, [eax + 48h]
        /*Supreme_Game.dll + 45DD7 */ lea  edx, [ebp - 20h]
        /*Supreme_Game.dll + 45DDA */ push edx
        /*Supreme_Game.dll + 45DDB */ mov  dword ptr [ebp - 4], 0Ah

        /*Supreme_Game.dll + 45DE2 */ mov  eax, dword ptr [g_addr_12DD20]
        /*                         */ call eax

        // Tail jump back to the real continuation
        mov  eax, dword ptr [g_speedJumpOutAddress]
        jmp  eax
    }
}

__declspec(naked) void ShowSpeedAndHideBlinkingRFunction() {
    __asm {
        /*Supreme_Game.dll + 45CD8 */ mov  cl, [ebp - 0Dh]
        /*Supreme_Game.dll + 45CDB */ mov  [ebp - 20h], cl
        /*Supreme_Game.dll + 45CDE */ push 0
        /*Supreme_Game.dll + 45CE0 */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45CE3 */ mov  eax, dword ptr [g_slot_1634C0]   // eax = &slot
        /*                         */ call dword ptr [eax]                   // call *(slot)

        /*Supreme_Game.dll + 45CE9 */ or   ecx, -1
        /*Supreme_Game.dll + 45CEC */ xor  eax, eax

        /*Supreme_Game.dll + 45CEE */ mov  edi, dword ptr [g_addr_1CE1C0]
        /*Supreme_Game.dll + 45CF3 */ repne scasb
        /*Supreme_Game.dll + 45CF5 */ not  ecx
        /*Supreme_Game.dll + 45CF7 */ dec  ecx
        /*Supreme_Game.dll + 45CF8 */ push 1
        /*Supreme_Game.dll + 45CFA */ mov  [ebp - 28h], ecx
        /*Supreme_Game.dll + 45CFD */ push ecx
        /*Supreme_Game.dll + 45CFE */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45D01 */ mov  eax, dword ptr [g_slot_1634C8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D07 */ test al, al
        /*Supreme_Game.dll + 45D09 */ je   L_45D33

        /*Supreme_Game.dll + 45D0B */ mov  eax, [ebp - 28h]
        /*Supreme_Game.dll + 45D0E */ mov  edi, [ebp - 1Ch]
        /*Supreme_Game.dll + 45D11 */ mov  ecx, eax
        /*Supreme_Game.dll + 45D13 */ mov  edx, ecx
        /*Supreme_Game.dll + 45D15 */ shr  ecx, 2
        /*Supreme_Game.dll + 45D18 */ mov  esi, dword ptr [g_addr_1CE1C0]
        /*Supreme_Game.dll + 45D1D */ repe movsd
        /*Supreme_Game.dll + 45D1F */ mov  ecx, edx
        /*Supreme_Game.dll + 45D21 */ and  ecx, 3
        /*Supreme_Game.dll + 45D24 */ repe movsb
        /*Supreme_Game.dll + 45D26 */ push eax
        /*Supreme_Game.dll + 45D27 */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45D2A */ mov  eax, dword ptr [g_slot_1634D8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D30 */ mov  esi, [ebp - 24h]

L_45D33:
        /*Supreme_Game.dll + 45D33 */ mov  eax, [ebx + 28h]
        /*Supreme_Game.dll + 45D36 */ mov  ecx, [eax + 3Ch]
        /*Supreme_Game.dll + 45D39 */ lea  edx, [ebp - 20h]
        /*Supreme_Game.dll + 45D3C */ push edx
        /*Supreme_Game.dll + 45D3D */ mov  [ebp - 4], 9

        /*Supreme_Game.dll + 45D44 */ mov eax, dword ptr [g_addr_12DD20]
        /*                         */ call eax

        /*Supreme_Game.dll + 45D49 */ push 1
        /*Supreme_Game.dll + 45D4B */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D4E */ mov  dword ptr [ebp - 4], 0FFFFFFFFh

        /*Supreme_Game.dll + 45D55 */ mov  eax, dword ptr [g_slot_1634C0]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D5B */ mov  eax, [ebp - 30h]
        /*Supreme_Game.dll + 45D5E */ push eax
        /*Supreme_Game.dll + 45D5F */ lea  ecx, [ebp - 00000158h]
        /*Supreme_Game.dll + 45D65 */ push dword ptr [g_addr_1B8D44]
        /*Supreme_Game.dll + 45D6A */ push ecx
        /*Supreme_Game.dll + 45D6B */ mov  eax, dword ptr [g_slot_163738]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D71 */ mov  dl, [ebp - 0Dh]
        /*Supreme_Game.dll + 45D74 */ add  esp, 0Ch

        /*Supreme_Game.dll + 45D77 */ push 0
        /*Supreme_Game.dll + 45D79 */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D7C */ mov  [ebp - 20h], dl
        /*Supreme_Game.dll + 45D7F */ mov  eax, dword ptr [g_slot_1634C0]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D85 */ or   ecx, -1
        /*Supreme_Game.dll + 45D88 */ xor  eax, eax
        /*Supreme_Game.dll + 45D8A */ lea  edi, [ebp - 00000158h]
        /*Supreme_Game.dll + 45D90 */ repne scasb
        /*Supreme_Game.dll + 45D92 */ not  ecx
        /*Supreme_Game.dll + 45D94 */ dec  ecx
        /*Supreme_Game.dll + 45D95 */ push 1
        /*Supreme_Game.dll + 45D97 */ mov  [ebp - 30h], ecx
        /*Supreme_Game.dll + 45D9A */ push ecx
        /*Supreme_Game.dll + 45D9B */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45D9E */ mov  eax, dword ptr [g_slot_1634C8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45DA4 */ test al, al
        /*Supreme_Game.dll + 45DA6 */ je   L_45DD1

        /*Supreme_Game.dll + 45DA8 */ mov  eax, [ebp - 30h]
        /*Supreme_Game.dll + 45DAB */ mov  edi, [ebp - 1Ch]
        /*Supreme_Game.dll + 45DAE */ mov  ecx, eax
        /*Supreme_Game.dll + 45DB0 */ mov  edx, ecx
        /*Supreme_Game.dll + 45DB2 */ shr  ecx, 2
        /*Supreme_Game.dll + 45DB5 */ lea  esi, [ebp - 00000158h]
        /*Supreme_Game.dll + 45DBB */ repe movsd
        /*Supreme_Game.dll + 45DBD */ mov  ecx, edx
        /*Supreme_Game.dll + 45DBF */ and  ecx, 3
        /*Supreme_Game.dll + 45DC2 */ repe movsb
        /*Supreme_Game.dll + 45DC4 */ push eax
        /*Supreme_Game.dll + 45DC5 */ lea  ecx, [ebp - 20h]
        /*Supreme_Game.dll + 45DC8 */ mov  eax, dword ptr [g_slot_1634D8]
        /*                         */ call dword ptr [eax]
        /*Supreme_Game.dll + 45DCE */ mov  esi, [ebp - 24h]

L_45DD1:
        /*Supreme_Game.dll + 45DD1 */ mov  eax, [ebx + 28h]
        /*Supreme_Game.dll + 45DD4 */ mov  ecx, [eax + 48h]
        /*Supreme_Game.dll + 45DD7 */ lea  edx, [ebp - 20h]
        /*Supreme_Game.dll + 45DDA */ push edx
        /*Supreme_Game.dll + 45DDB */ mov  dword ptr [ebp - 4], 0Ah

        /*Supreme_Game.dll + 45DE2 */ mov  eax, dword ptr [g_addr_12DD20]
        /*                         */ call eax

        // Tail jump back to the real continuation
        mov  eax, dword ptr [g_speedJumpOutAddress]
        jmp  eax
    }
}

__declspec(naked) void HideBlinkingRFunction() {
    __asm {
        /*Supreme_Game.dll + 45CD8 */ mov  cl, [ebp - 0Dh]
        /*Supreme_Game.dll + 45CDB */ mov  [ebp - 20h], cl
        /*Supreme_Game.dll + 45CDE */ push 0
        /*Supreme_Game.dll + 45CE0 */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45CE3 */ mov  eax, dword ptr [g_slot_1634C0]   // eax = &slot
        /*                         */ call dword ptr [eax]                   // call *(slot)

        /*Supreme_Game.dll + 45CE9 */ or   ecx, -1
        /*Supreme_Game.dll + 45CEC */ xor  eax, eax

        /*Supreme_Game.dll + 45CEE */ mov  edi, dword ptr [g_addr_1CE1C0]
        /*Supreme_Game.dll + 45CF3 */ repne scasb
        /*Supreme_Game.dll + 45CF5 */ not  ecx
        /*Supreme_Game.dll + 45CF7 */ dec  ecx
        /*Supreme_Game.dll + 45CF8 */ push 1
        /*Supreme_Game.dll + 45CFA */ mov  [ebp - 28h], ecx
        /*Supreme_Game.dll + 45CFD */ push ecx
        /*Supreme_Game.dll + 45CFE */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45D01 */ mov  eax, dword ptr [g_slot_1634C8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D07 */ test al, al
        /*Supreme_Game.dll + 45D09 */ je   L_45D33

        /*Supreme_Game.dll + 45D0B */ mov  eax, [ebp - 28h]
        /*Supreme_Game.dll + 45D0E */ mov  edi, [ebp - 1Ch]
        /*Supreme_Game.dll + 45D11 */ mov  ecx, eax
        /*Supreme_Game.dll + 45D13 */ mov  edx, ecx
        /*Supreme_Game.dll + 45D15 */ shr  ecx, 2
        /*Supreme_Game.dll + 45D18 */ mov  esi, dword ptr [g_addr_1CE1C0]
        /*Supreme_Game.dll + 45D1D */ repe movsd
        /*Supreme_Game.dll + 45D1F */ mov  ecx, edx
        /*Supreme_Game.dll + 45D21 */ and  ecx, 3
        /*Supreme_Game.dll + 45D24 */ repe movsb
        /*Supreme_Game.dll + 45D26 */ push eax
        /*Supreme_Game.dll + 45D27 */ lea  ecx, [ebp - 20h]

        /*Supreme_Game.dll + 45D2A */ mov  eax, dword ptr [g_slot_1634D8]
        /*                         */ call dword ptr [eax]

        /*Supreme_Game.dll + 45D30 */ mov  esi, [ebp - 24h]

L_45D33:
        /*Supreme_Game.dll + 45D33 */ mov  eax, [ebx + 28h]
        /*Supreme_Game.dll + 45D36 */ mov  ecx, [eax + 3Ch]
        /*Supreme_Game.dll + 45D39 */ lea  edx, [ebp - 20h]
        /*Supreme_Game.dll + 45D3C */ push edx
        /*Supreme_Game.dll + 45D3D */ mov  [ebp - 4], 9

        /*Supreme_Game.dll + 45D44 */ mov eax, dword ptr [g_addr_12DD20]
        /*                         */ call eax

        // Tail jump back to the real continuation
        mov  eax, dword ptr [g_speedJumpOutAddress]
        jmp  eax
    }
}

// --- Helper: write a 5-byte E9 rel32 jump at 'at' to 'dst'
static void WriteJmpRel32(void* at, void* dst)
{
    DWORD oldProt;
    VirtualProtect(at, 5, PAGE_EXECUTE_READWRITE, &oldProt);

    uint8_t* p = static_cast<uint8_t*>(at);
    p[0] = 0xE9;
    int32_t rel = static_cast<int32_t>(reinterpret_cast<uint8_t*>(dst) - (p + 5));
    std::memcpy(p + 1, &rel, sizeof(rel));

    FlushInstructionCache(GetCurrentProcess(), at, 5);
    VirtualProtect(at, 5, oldProt, &oldProt);
}

void SetSpeedRectangleVisibility(bool showRectangle) {
    uint8_t* addr = (uint8_t*)((char*)supremeGameModule + 0x455b2);
    uint8_t* dest = showRectangle
        ? (uint8_t*)((char*)supremeGameModule + 0x11c7a0)
        : (uint8_t*)((char*)supremeGameModule + 0x11c7b0);
    DWORD oldProtect;
    VirtualProtect(addr, 5, PAGE_EXECUTE_READWRITE, &oldProtect);
    if (addr[0] != 0xE8) {
        Log("SetSpeedRectangleVisibility: Expected call instruction at 0x455b2, but found something else.");
        VirtualProtect(addr, 5, oldProtect, &oldProtect);
        return;
    }
    intptr_t rel = (intptr_t)dest - ((intptr_t)addr + 5);
    *(int32_t*)((uint8_t*)addr + 1) = (int32_t)rel;
    VirtualProtect(addr, 5, oldProtect, &oldProtect);
}
