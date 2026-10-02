#pragma once
#include <cstdint>

// Pure parts of the crash record (crash_report.hpp): which exceptions count as
// faults, and the module table a fault address is resolved against.
namespace crashrecord {

inline constexpr uint32_t kMaxModules = 128;
inline constexpr uint32_t kNameMax = 32;

// Error-severity exceptions, minus the C++ throw, which the game's own code
// raises and catches.
inline bool IsFault(uint32_t code) {
    constexpr uint32_t kCppException = 0xE06D7363;
    return (code & 0xC0000000u) == 0xC0000000u && code != kCppException;
}

struct ModuleEntry {
    uint32_t base;
    uint32_t size;
    char name[kNameMax];
};

struct ModuleTable {
    ModuleEntry entries[kMaxModules];
    uint32_t count = 0;

    // Index of the module containing address, or -1.
    int Find(uint32_t address) const {
        for (uint32_t i = 0; i < count; i++) {
            if (address - entries[i].base < entries[i].size) return (int)i;
        }
        return -1;
    }
};

}  // namespace crashrecord
