#pragma once
#include "stdafx.h"
#include "helper.hpp"
#include "log.hpp"

// Game module bases and resolved addresses.
// All offsets from reverse engineering documented in tasks/tas.md.

struct GameAddresses {
    // Module bases
    HMODULE sg = nullptr;    // Supreme_Game.dll
    HMODULE hmg = nullptr;   // HMG_Cetsup_Win32.dll

    // Supreme_Game.dll offsets
    std::uint8_t* cave2_site = nullptr;     // SG+0x13FE40: Supreme::Cycle
    std::uint8_t* player_base = nullptr;    // [SG+0x1D5450]
    std::uint8_t* vk_table = nullptr;       // SG+0x9AD8: VK -> keyIndex lookup

    // HMG_Cetsup_Win32.dll offsets
    std::uint8_t* cave1c_down = nullptr;    // HMG+0x3940: key handler (down)
    std::uint8_t* cave1c_up = nullptr;      // HMG+0x3980: key handler (up)
    std::uint8_t* bb3b10 = nullptr;         // HMG+0x3B10: BB3B10 observer

    // Game key codes
    static constexpr uint8_t KEY_UP    = 0x38;
    static constexpr uint8_t KEY_DOWN  = 0x39;
    static constexpr uint8_t KEY_LEFT  = 0x3A;
    static constexpr uint8_t KEY_RIGHT = 0x3B;
    static constexpr uint8_t KEY_JUMP  = 0x27;  // VK_CTRL
    static constexpr uint8_t KEY_SHIFT = 0x24;  // VK_SHIFT

    // Action state offsets from [SG+1D5450]
    static constexpr uint32_t ACTION_LEFT  = 0xC4;
    static constexpr uint32_t ACTION_UP    = 0xC8;
    static constexpr uint32_t ACTION_RIGHT = 0xCC;
    static constexpr uint32_t ACTION_DOWN  = 0xD0;

    // DI buffer offset from [SG+1D5450]
    static constexpr uint32_t KEYBOARD_OBJ_OFFSET = 0x530;
    static constexpr uint32_t DI_BUFFER_OFFSET = 0x530 + 0x30;  // 256 bytes

    bool Resolve() {
        sg = GetModuleHandleA("Supreme_Game.dll");
        hmg = GetModuleHandleA("HMG_Cetsup_Win32.dll");

        if (!sg) { Log("ERROR: Supreme_Game.dll not loaded"); return false; }
        if (!hmg) { Log("ERROR: HMG_Cetsup_Win32.dll not loaded"); return false; }

        auto sgBase = (std::uint8_t*)sg;
        auto hmgBase = (std::uint8_t*)hmg;

        // Direct offset resolution (known addresses)
        cave2_site = sgBase + 0x13FE40;
        player_base = sgBase + 0x1D5450;
        vk_table = sgBase + 0x9AD8;

        cave1c_down = hmgBase + 0x3940;
        cave1c_up = hmgBase + 0x3980;
        bb3b10 = hmgBase + 0x3B10;

        Log(std::format("SG base: {:p}", (void*)sgBase));
        Log(std::format("HMG base: {:p}", (void*)hmgBase));
        Log(std::format("Cave 2 site (Supreme::Cycle): {:p} (SG+0x13FE40)", (void*)cave2_site));
        Log(std::format("Cave 1C down (+3940): {:p}", (void*)cave1c_down));
        Log(std::format("Cave 1C up (+3980): {:p}", (void*)cave1c_up));
        Log(std::format("BB3B10 (+3B10): {:p}", (void*)bb3b10));
        Log(std::format("Player base ptr: {:p}", (void*)player_base));
        Log(std::format("VK table: {:p}", (void*)vk_table));

        return true;
    }
};
