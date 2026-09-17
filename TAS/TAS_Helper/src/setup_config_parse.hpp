#pragma once

#include "rider_identity_parse.hpp"
#include <cstdint>
#include <cstring>
#include <limits>

// Pure, testable reader for the game's selected track/rider configuration.
//
// The stock executable uses this exact chain itself (Supreme_v1.035.c around
// FUN_0042f4d0): config = *(DAT_004889c4 + 0x14), then accesses the fields
// below. Keeping the chain and field offsets in this dependency-free header
// lets the unit suite exercise the same pointer arithmetic and string-header
// validation as the injected DLL, without needing a live game process.
namespace setupconfig {

constexpr uint32_t MAIN_STATE_PTR_RVA = 0x889C4;
constexpr uint32_t CONFIG_PTR_OFFSET = 0x14;

constexpr uint32_t AREA_STRING = 0xB0;
constexpr uint32_t DIFFICULTY_STRING = 0xD0;
constexpr uint32_t CHARACTER_STRING = 0xF0;
constexpr uint32_t STANCE = 0x140;
constexpr uint32_t CONTROLLER_STRING = 0x1B0;

constexpr uint32_t STANCE_UNKNOWN = 0xFFFFFFFFu;

struct Values {
    bool valid = false;
    char area[32] = {};
    char difficulty[32] = {};
    char character[32] = {};
    uint32_t stance = STANCE_UNKNOWN;
};

inline bool AddAddress(uint32_t base, uint32_t offset, uint32_t* out) {
    if (base > (std::numeric_limits<uint32_t>::max)() - offset) return false;
    *out = base + offset;
    return true;
}

inline bool KnownController(const char* value) {
    return std::strcmp(value, "Keyboard") == 0 || std::strcmp(value, "Mouse") == 0 ||
           std::strcmp(value, "Joystick") == 0;
}

// Reader contract: `reader(address, destination, byte_count) -> bool`.
template <typename Reader>
bool ReadString(Reader& reader, uint32_t object, char* out, uint32_t cap) {
    uint32_t header[4] = {};
    if (!reader(object, header, sizeof header)) return false;
    const uint32_t ptr = header[1];
    const uint32_t len = header[2];
    const uint32_t capacity = header[3];
    if (!riderparse::StringHeaderUsable(ptr, len, capacity, cap)) return false;
    if (!reader(ptr, out, len)) return false;
    out[len] = 0;
    return riderparse::IsPrintableAscii(out, len);
}

template <typename Reader>
bool Read(uint32_t exe_base, Reader& reader, Values* out, uint32_t* config_address = nullptr) {
    *out = Values{};
    if (config_address) *config_address = 0;

    uint32_t global_address = 0;
    if (!AddAddress(exe_base, MAIN_STATE_PTR_RVA, &global_address)) return false;

    uint32_t state = 0;
    if (!reader(global_address, &state, sizeof state) || state < 0x10000) return false;

    uint32_t config_slot = 0;
    if (!AddAddress(state, CONFIG_PTR_OFFSET, &config_slot)) return false;
    uint32_t config = 0;
    if (!reader(config_slot, &config, sizeof config) || config < 0x10000) return false;
    if (config_address) *config_address = config;

    uint32_t area = 0, difficulty = 0, character = 0, controller = 0, stance = 0;
    if (!AddAddress(config, AREA_STRING, &area) ||
        !AddAddress(config, DIFFICULTY_STRING, &difficulty) ||
        !AddAddress(config, CHARACTER_STRING, &character) ||
        !AddAddress(config, CONTROLLER_STRING, &controller) ||
        !AddAddress(config, STANCE, &stance)) {
        return false;
    }

    char controller_name[32] = {};
    if (!ReadString(reader, controller, controller_name, sizeof controller_name) ||
        !KnownController(controller_name) ||
        !ReadString(reader, area, out->area, sizeof out->area) ||
        !ReadString(reader, difficulty, out->difficulty, sizeof out->difficulty) ||
        !ReadString(reader, character, out->character, sizeof out->character)) {
        return false;
    }

    uint32_t stance_value = STANCE_UNKNOWN;
    if (reader(stance, &stance_value, sizeof stance_value) && stance_value <= 1) {
        out->stance = stance_value;
    }
    out->valid = true;
    return true;
}

}  // namespace setupconfig
