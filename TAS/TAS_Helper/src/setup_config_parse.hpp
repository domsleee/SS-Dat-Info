#pragma once

#include "rider_identity_parse.hpp"
#include <cstdint>
#include <cstring>

// Pure, unit-tested reader for the game's selected track/rider configuration.
// The stock executable uses the same chain (FUN_0042f4d0):
// config = *(DAT_004889c4 + 0x14).
namespace setupconfig {

constexpr uint32_t MAIN_STATE_PTR_RVA = 0x889C4;
constexpr uint32_t CONFIG_PTR_OFFSET = 0x14;

constexpr uint32_t AREA_STRING = 0xB0;
constexpr uint32_t DIFFICULTY_STRING = 0xD0;
constexpr uint32_t CHARACTER_STRING = 0xF0;
constexpr uint32_t STANCE = 0x140;               // dword: 0 = regular, 1 = goofy
constexpr uint32_t CONTROLLER_STRING = 0x1B0;

constexpr uint32_t STANCE_UNKNOWN = 0xFFFFFFFFu;

struct Values {
    bool valid = false;
    char area[32] = {};
    char difficulty[32] = {};
    char character[32] = {};
    uint32_t stance = STANCE_UNKNOWN;
};

inline bool KnownController(const char* value) {
    return std::strcmp(value, "Keyboard") == 0 || std::strcmp(value, "Mouse") == 0 ||
           std::strcmp(value, "Joystick") == 0;
}

// Reader contract: `reader(address, destination, byte_count) -> bool`.
template <typename Reader>
bool Read(uint32_t exe_base, Reader& reader, Values* out, uint32_t* config_address = nullptr) {
    *out = Values{};
    if (config_address) *config_address = 0;

    uint32_t state = 0;
    if (!reader(exe_base + MAIN_STATE_PTR_RVA, &state, sizeof state) || state < 0x10000) return false;

    uint32_t config = 0;
    if (!reader(state + CONFIG_PTR_OFFSET, &config, sizeof config) || config < 0x10000) return false;
    if (config_address) *config_address = config;

    char controller_name[32] = {};
    if (!riderparse::ReadStdString(reader, config + CONTROLLER_STRING, controller_name,
                                   sizeof controller_name) ||
        !KnownController(controller_name) ||
        !riderparse::ReadStdString(reader, config + AREA_STRING, out->area, sizeof out->area) ||
        !riderparse::ReadStdString(reader, config + DIFFICULTY_STRING, out->difficulty,
                                   sizeof out->difficulty) ||
        !riderparse::ReadStdString(reader, config + CHARACTER_STRING, out->character,
                                   sizeof out->character)) {
        return false;
    }

    uint32_t stance_value = STANCE_UNKNOWN;
    if (reader(config + STANCE, &stance_value, sizeof stance_value) && stance_value <= 1) {
        out->stance = stance_value;
    }
    out->valid = true;
    return true;
}

}  // namespace setupconfig
