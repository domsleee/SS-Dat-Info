#pragma once
#include "stdafx.h"
#include "helper.hpp"
#include "log.hpp"

// Game module bases and resolved addresses.
// All offsets from reverse engineering documented in tasks/tas.md.

// Housemarque Kernel::Time — a 64-bit timestamp ({lo, hi} dwords; ~QPC-derived
// machine-uptime units). The input pipeline stamps every key event with the
// Time at message-pump dispatch: Win32_Driver::Translate(tagMSG&, Kernel::Time)
// → keyDown/keyUp (+3940/+3980) → BB3B10(keyIndex, pressed, Time.lo, Time.hi).
// The observer discards events whose Time predates the current race context —
// the "dynamic arg4" that silently killed stale injected input.
struct KernelTime { uint32_t lo; uint32_t hi; };

// ?Current@Time@Kernel@Housemarque@@SI?AV123@XZ — static __fastcall, returns
// the Time by value through a hidden return slot passed in ecx (verified by
// disasm: `mov esi, ecx; ... mov [esi], eax; mov [esi+4], edx; mov eax, esi`).
using KernelTimeCurrentFn = KernelTime*(__fastcall*)(KernelTime* out, void* edx_unused);

struct GameAddresses {
    // Module bases
    HMODULE exe = nullptr;   // Supreme.exe
    HMODULE sg = nullptr;    // Supreme_Game.dll
    HMODULE hmg = nullptr;   // HMG_Cetsup_Win32.dll
    HMODULE kernel = nullptr; // HMG_Kernel.dll

    // HMG_Kernel.dll: Kernel::Time::Current() — the game's own clock. Used to
    // stamp injected BB3B10 input events with a genuinely-current timestamp
    // (exactly what a real keypress carries), so injection is never discarded
    // as stale. nullptr = export missing (fall back to calibrated arg4).
    KernelTimeCurrentFn time_current = nullptr;

    // Supreme.exe offsets
    std::uint8_t* cave5_site = nullptr;     // exe+0x25C81: after ftol+mov esi,eax (tick override)
    std::uint8_t* is_in_game = nullptr;     // exe+0x8895C: 0=main menu, 1=in-game (race/level)

    // Supreme_Game.dll offsets
    std::uint8_t* cave2_site = nullptr;     // SG+0x13FE40: Supreme::Cycle
    std::uint8_t* replay_capture_site = nullptr; // SG+0x9E8F0: replay object capture
    std::uint8_t* player_base = nullptr;    // SG+0x1D5450: root pointer
    // SG+0x1D3304: pointer to the CURRENT level's resource path string.
    // Found by differential RE (tas_test level-hunt ptr) and verified on four
    // tracks. The pointed-to string changes the instant a level loads, which is
    // the level-change event. RELIABLE FOR AREA, NOT FOR DIFFICULTY: some tracks
    // share the easy/ shadow asset, so Village Hard reads ".../Tracks/easy/...".
    // Difficulty still comes from the majority-voted heap scan, which exists
    // precisely to survive that outlier.
    std::uint8_t* level_path_ptr = nullptr;
    std::uint8_t* vk_table = nullptr;       // SG+0x9AD8: VK -> keyIndex lookup

    // HMG_Cetsup_Win32.dll offsets
    std::uint8_t* cave1c_down = nullptr;    // HMG+0x3940: key handler (down)
    std::uint8_t* cave1c_up = nullptr;      // HMG+0x3980: key handler (up)
    std::uint8_t* bb3b10 = nullptr;         // HMG+0x3B10: BB3B10 observer

    // Game key codes (DI buffer indices)
    static constexpr uint8_t KEY_UP    = 0x38;
    static constexpr uint8_t KEY_DOWN  = 0x39;
    static constexpr uint8_t KEY_LEFT  = 0x3A;
    static constexpr uint8_t KEY_RIGHT = 0x3B;
    static constexpr uint8_t KEY_JUMP  = 0x27;  // LCTRL
    static constexpr uint8_t KEY_JUMP2 = 0x28;  // CTRL (duplicate)
    static constexpr uint8_t KEY_SHIFT = 0x24;  // SHIFT
    static constexpr uint8_t KEY_SHIFT2 = 0x25; // LSHIFT (duplicate)
    static constexpr uint8_t KEY_F5 = 0x58;     // F5 (restart race)

    // BB3B10 keyIndex values (game key codes for observer notification)
    static constexpr uint32_t BB3B10_LEFT  = 0x3A;
    static constexpr uint32_t BB3B10_RIGHT = 0x3B;
    static constexpr uint32_t BB3B10_UP    = 0x38;
    static constexpr uint32_t BB3B10_DOWN  = 0x39;
    static constexpr uint32_t BB3B10_JUMP  = 0x27;
    static constexpr uint32_t BB3B10_SHIFT = 0x24;
    static constexpr uint32_t BB3B10_F5    = 0x58;
    // ESC's game Key_Code (live-captured via RDIAG: VK 0x1B → ki 0x48). The
    // pause menu listens for it through the BB3B10 observer broadcast, so the
    // REC-mode observer block must exempt it (ESC-during-REC fix).
    static constexpr uint32_t BB3B10_ESC   = 0x48;

    // Pointer chain: root = [SG+1D5450], kbobj = [root+530], buffer = [kbobj+30]
    static constexpr uint32_t ROOT_PTR_OFFSET = 0x1D5450;
    static constexpr uint32_t LEVEL_PATH_PTR_OFFSET = 0x1D3304;
    static constexpr uint32_t KEYBOARD_OBJ_OFFSET = 0x530;
    static constexpr uint32_t DI_BUFFER_PTR_OFFSET = 0x30;

    // Action state byte offsets from keyboard object (kbobj = [root+0x530])
    static constexpr uint32_t AS_LEFT  = 0x455;
    static constexpr uint32_t AS_UP    = 0x456;
    static constexpr uint32_t AS_RIGHT = 0x457;
    static constexpr uint32_t AS_DOWN  = 0x458;
    static constexpr uint32_t AS_SHIFT = 0x440;
    static constexpr uint32_t AS_JUMP  = 0x441;

    // Replay object: player ptr at [replayObj+0x84]
    static constexpr uint32_t REPLAY_PLAYER_OFFSET = 0x84;
    // Player position offsets
    static constexpr uint32_t PLAYER_X = 0xF8;
    static constexpr uint32_t PLAYER_Y = 0xFC;
    static constexpr uint32_t PLAYER_Z = 0x100;
    static constexpr uint32_t PLAYER_PHYSICS = 0x110; // Pointer to physics sub-object
    static constexpr uint32_t PHYSICS_ROT = 0x1B4;   // 3x3 rotation matrix in physics sub-object (9 floats, row-major)

    // BB3B10 calling convention constant.
    // 2026-06: live capture of the REAL key handler's BB3B10 call (cave1d
    // diagnostic, holding a real steering key in OFF) showed arg4=0x96 for
    // every steering key — not 0x588. The 0x588 (set in d04104f) made cave2's
    // injected BB3B10 call a no-op for steering, which is why record AND
    // playback steered dead. The real this(kbobj+0x18) and keyIndex(0x3A/0x3B)
    // were already correct.
    static constexpr uint32_t BB3B10_ARG4 = 0x96;
    // BB3B10 this pointer offset from keyboard object
    static constexpr uint32_t BB3B10_THIS_OFFSET = 0x18;

    bool Resolve() {
        exe = GetModuleHandleA(nullptr);  // Supreme.exe (main executable)
        sg = GetModuleHandleA("Supreme_Game.dll");
        hmg = GetModuleHandleA("HMG_Cetsup_Win32.dll");
        kernel = GetModuleHandleA("HMG_Kernel.dll");

        if (!exe) { Log("ERROR: Supreme.exe not found"); return false; }
        if (!sg) { Log("ERROR: Supreme_Game.dll not loaded"); return false; }
        if (!hmg) { Log("ERROR: HMG_Cetsup_Win32.dll not loaded"); return false; }

        // Kernel::Time::Current — non-fatal if missing (injection falls back
        // to the keypress-calibrated arg4), but it should always resolve.
        if (kernel) {
            time_current = (KernelTimeCurrentFn)GetProcAddress(
                kernel, "?Current@Time@Kernel@Housemarque@@SI?AV123@XZ");
        }
        if (!time_current) {
            Log("WARNING: Kernel::Time::Current not resolved — injected input "
                "falls back to calibrated arg4");
        }

        auto exeBase = (std::uint8_t*)exe;
        auto sgBase = (std::uint8_t*)sg;
        auto hmgBase = (std::uint8_t*)hmg;

        // Supreme.exe offsets
        cave5_site = exeBase + 0x25C81;
        is_in_game = exeBase + 0x8895C;  // 0=menu, 1=in-game (RE'd via menu↔game diff)

        // Supreme_Game.dll offsets
        cave2_site = sgBase + 0x13FE40;
        replay_capture_site = sgBase + 0x9E8F0;
        player_base = sgBase + ROOT_PTR_OFFSET;
        level_path_ptr = sgBase + GameAddresses::LEVEL_PATH_PTR_OFFSET;
        vk_table = sgBase + 0x9AD8;

        // HMG_Cetsup_Win32.dll offsets
        cave1c_down = hmgBase + 0x3940;
        cave1c_up = hmgBase + 0x3980;
        bb3b10 = hmgBase + 0x3B10;

        Log(std::format("EXE base: {:p}", (void*)exeBase));
        Log(std::format("SG base: {:p}", (void*)sgBase));
        Log(std::format("HMG base: {:p}", (void*)hmgBase));
        Log(std::format("Cave 2 site (Supreme::Cycle): {:p} (SG+0x13FE40)", (void*)cave2_site));
        Log(std::format("Cave 5 site (tick override): {:p} (EXE+0x25C81)", (void*)cave5_site));
        Log(std::format("Replay capture site: {:p} (SG+0x9E8F0)", (void*)replay_capture_site));
        Log(std::format("Cave 1C down (+3940): {:p}", (void*)cave1c_down));
        Log(std::format("Cave 1C up (+3980): {:p}", (void*)cave1c_up));
        Log(std::format("BB3B10 (+3B10): {:p}", (void*)bb3b10));
        Log(std::format("Player base ptr: {:p}", (void*)player_base));
        Log(std::format("VK table: {:p}", (void*)vk_table));
        Log(std::format("Kernel::Time::Current: {:p}", (void*)time_current));

        return true;
    }
};
