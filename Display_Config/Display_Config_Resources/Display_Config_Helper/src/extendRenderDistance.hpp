#pragma once
#include <windows.h>
#include "helper.hpp"
#include "log.hpp"

// Ground render distance fix ("shredded far triangles").
//
// Supreme_Game.dll's ground renderer emits the slope as vertical strips of
// rows (row spacing = 1.2m * detail step; step = 1 at ground detail 4, 2 at
// detail 3, ...). A strip whose visible row count exceeds a hardcoded 400 is
// SKIPPED ENTIRELY:
//
//     rows = (end - start) / step + 1;
//     if (rows < 1 || rows > 400) { /* strip dropped */ }     FUN_100b0ef0
//
// At detail 4 that caps clean rendering at 400 * 1.2 = 480m — beyond that,
// far strips vanish ("missing triangles"), which is why the game shipped with
// visibility 450. The scratch buffers the rows feed are flush-batched every
// 4096 vertices, so the 400 is a draw-distance clamp, not a memory bound;
// raising it to 500 lifts the clean range to 500 * 1.2 = 600m at detail 4
// (detail 3 and below were already fine: bigger steps, fewer rows).
//
// Site (Supreme_Game.dll v1.035, image-relative 0xB0FA5):
//     0f 8e xx xx 00 00   jle  skip          ; rows < 1
//     81 ff 90 01 00 00   cmp  edi, 400      ; <-- imm32 patched to 500
//     0f 8f xx xx 00 00   jg   skip          ; rows > cap
inline void DoExtendRenderDistance() {
    auto module = GetModuleHandleA("Supreme_Game.dll");
    if (!module) {
        Log("ExtendRenderDistance: Supreme_Game.dll not found");
        return;
    }

    // idiv; inc edi; test edi,edi; mov [esp+20h],edi; mov ebx,eax;
    // mov [esp+14h],ebx; jle ...; cmp edi,190h; jg ...
    std::uint8_t* site = Memory::PatternScan(module,
        "F7 F9 47 85 FF 89 7C 24 20 8B D8 89 5C 24 14 0F 8E ?? ?? 00 00 81 FF 90 01 00 00 0F 8F");
    if (!site) {
        Log("ExtendRenderDistance: pattern not found (game version mismatch?) - not patched");
        return;
    }

    std::uint8_t* imm = site + 23;  // the 0x190 immediate inside `cmp edi, 0x190`
    Log(std::format("ExtendRenderDistance: row cap at Supreme_Game.dll+{:x}, 400 -> 500",
                    imm - (std::uint8_t*)module));
    Memory::Write<std::uint32_t>(imm, 500);
    Log("ExtendRenderDistance: applied (clean ground to 600m at ground detail 4)");
}
