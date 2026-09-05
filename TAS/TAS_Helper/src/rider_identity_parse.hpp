#pragma once
#include <cstdint>

// Pure helpers for the rider stamp (unit-tested in
// tests/test_rider_identity.cpp; rider_identity.hpp does the game reads).
namespace riderparse {

// Mirrors TasCharacterId in shared_state.hpp (kept numeric here so the test
// compiles without the Windows headers).
constexpr uint32_t CHARACTER_UNKNOWN = 0;
constexpr uint32_t CHARACTER_KEITH   = 1;
constexpr uint32_t CHARACTER_VINCENT = 2;
constexpr uint32_t CHARACTER_AKIKO   = 3;
constexpr uint32_t CHARACTER_KARL    = 4;
constexpr uint32_t CHARACTER_MIKE    = 5;
constexpr uint32_t CHARACTER_ULRIKA  = 6;
constexpr uint32_t CHARACTER_OTHER   = 7;
constexpr uint32_t STANCE_UNKNOWN    = 0xFFFFFFFFu;

inline bool IsPrintableAscii(const char* s, uint32_t n) {
    if (n == 0) return false;
    for (uint32_t i = 0; i < n; i++) {
        if (s[i] < 0x20 || s[i] > 0x7E) return false;
    }
    return true;
}

inline bool EqualsIgnoreCase(const char* a, const char* b) {
    for (;; a++, b++) {
        char x = *a, y = *b;
        if (x >= 'A' && x <= 'Z') x = (char)(x - 'A' + 'a');
        if (y >= 'A' && y <= 'Z') y = (char)(y - 'A' + 'a');
        if (x != y) return false;
        if (!x) return true;
    }
}

// The character id for a display name ("Vincent") or data folder
// ("vincent"). Names outside the table (mods, custom riders) map to OTHER,
// never to UNKNOWN - UNKNOWN means "not resolved yet".
// Is an MSVC6 std::string header {allocator, ptr, size, capacity} usable for a
// copy into a `cap`-byte C buffer? Codex review 2026-09-03: the previous
// `len + 1 > cap` guard WRAPS when a dangling string reports len 0xFFFFFFFF,
// letting a ~4 GB memcpy loose on a 32-byte stack buffer before SEH can fire.
// Every bound is checked without arithmetic on `len`; the game's own
// capacity must not be smaller than its size either.
inline bool StringHeaderUsable(uint32_t ptr, uint32_t len, uint32_t capacity, uint32_t cap) {
    if (ptr < 0x10000 || cap == 0) return false;
    if (len == 0 || len >= cap) return false;
    if (capacity < len) return false;
    return true;
}

inline uint32_t CharacterFromName(const char* name) {
    if (!name || !*name) return CHARACTER_UNKNOWN;
    static const struct { const char* name; uint32_t id; } kTable[] = {
        {"keith", CHARACTER_KEITH},   {"vincent", CHARACTER_VINCENT},
        {"akiko", CHARACTER_AKIKO},   {"karl", CHARACTER_KARL},
        {"mike", CHARACTER_MIKE},     {"ulrika", CHARACTER_ULRIKA},
    };
    for (const auto& e : kTable) {
        if (EqualsIgnoreCase(name, e.name)) return e.id;
    }
    return CHARACTER_OTHER;
}

// The selected stance belongs to the selected character. During a menu change
// those fields can temporarily describe different riders, so publish a stance
// only when the setup character matches the live Player.
inline uint32_t StanceForRider(uint32_t setup_stance, const char* setup_character,
                               const char* live_character) {
    if (setup_stance > 1 || !setup_character || !live_character) return STANCE_UNKNOWN;
    return EqualsIgnoreCase(setup_character, live_character) ? setup_stance : STANCE_UNKNOWN;
}

} // namespace riderparse
