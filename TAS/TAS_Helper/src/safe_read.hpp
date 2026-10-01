#pragma once
#include <windows.h>
#include <cstdint>
#include <cstring>

// SEH-guarded reads of game memory, which an F5 restart or a page change can
// free under us. Nothing is mapped below 0x10000. No C++ objects (C2712).

inline bool SafeCopy(uint32_t src, void* dst, uint32_t n) {
    if (src < 0x10000) return false;
    __try {
        memcpy(dst, (const void*)(uintptr_t)src, n);
        return true;
    } __except (EXCEPTION_EXECUTE_HANDLER) {
        return false;
    }
}

// The u32 at `addr`, or 0 when unreadable.
inline uint32_t SafeRead32(uint32_t addr) {
    uint32_t v = 0;
    return SafeCopy(addr, &v, sizeof v) ? v : 0;
}
