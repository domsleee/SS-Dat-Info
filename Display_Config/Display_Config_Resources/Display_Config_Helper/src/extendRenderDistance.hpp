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
        Log("ExtendRenderDistance: NOT APPLIED (pattern not found - game version mismatch?): distances above 480m at ground detail 4 will show missing far terrain");
        return;
    }

    // 1024, not 500: the visibility volume extends 600m in BOTH directions
    // along a strip, so mid-track strips span up to ~1000 rows (1200m / 1.2m).
    // 500 covered the start-gate view but still dropped the longest strips
    // mid-run, visible as missing terrain in the distance while riding. 1024
    // covers the theoretical maximum at the launcher's 600m clamp; the
    // renderer's scratch flushes every 4096 vertices, so long strips are safe.
    std::uint8_t* imm = site + 23;  // the 0x190 immediate inside `cmp edi, 0x190`
    Log(std::format("ExtendRenderDistance: row cap at Supreme_Game.dll+{:x}, 400 -> 1024",
                    imm - (std::uint8_t*)module));
    Memory::Write<std::uint32_t>(imm, 1024);
    Log("ExtendRenderDistance: applied (clean ground to 600m at ground detail 4)");
}
