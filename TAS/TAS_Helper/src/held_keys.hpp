#pragma once
#include <cstdint>

// The keyboard observer's held-key array, as the TAS input mask (DESIGN.md
// "How game input works"). Steering reads the array; the observer's Update
// fills it from queued key events before each Supreme::Cycle. Gameplay polls
// it through TC_Kbd_Impl::Is_Pressed (0x40F710), which ORs three codes for
// jump (0x27..0x29) and for shift (0x24..0x26); a bit reads as held when any
// of its codes is, and writing a bit sets all of them. Nothing listens to the
// observer's events, so writing the array loses nothing. Pure logic over a
// 256-byte array, so tests can run it.
namespace heldkeys {

struct Key {
    uint32_t code;
    uint8_t bit;
};

// Game key codes (game_addresses.hpp KEY_*) and their input_log bits.
inline constexpr Key kKeys[] = {
    {0x3A, 0x01},  // LEFT
    {0x3B, 0x02},  // RIGHT
    {0x38, 0x04},  // UP
    {0x39, 0x08},  // DOWN
    {0x27, 0x10},  // JUMP: CTRL
    {0x28, 0x10},  // LCTRL
    {0x29, 0x10},  // RCTRL
    {0x24, 0x20},  // SHIFT
    {0x25, 0x20},  // LSHIFT
    {0x26, 0x20},  // RSHIFT
};

// Whether a game key code is one of the six recorded keys (any alias).
inline bool IsTasCode(uint32_t code) {
    for (const auto& k : kKeys) {
        if (k.code == code) return true;
    }
    return false;
}

// Release `bits`: every code of each, whatever set it; other keys untouched.
inline void Clear(volatile uint8_t* held, uint8_t bits) {
    for (const auto& k : kKeys) {
        if (bits & k.bit) held[k.code] = 0;
    }
}

// The codes real key events set. HMG maps each key message's wParam through
// one VK table and window messages carry the neutral VK, so either Ctrl is
// 0x27 and either Shift 0x24; a key-up clears the one code its key-down set
// (HMG+0x17B0, table HMG+0x9AD8).
inline constexpr Key kRealKeys[] = {
    {0x3A, 0x01}, {0x3B, 0x02}, {0x38, 0x04}, {0x39, 0x08}, {0x27, 0x10}, {0x24, 0x20},
};
inline constexpr uint32_t kAliasCodes[] = {0x28, 0x29, 0x25, 0x26};

// The held state real key events would have left for `mask`: its codes set
// or cleared, the aliases cleared, so the player's key-ups release it all.
inline void WriteReal(volatile uint8_t* keys, uint8_t mask) {
    for (const auto& k : kRealKeys) keys[k.code] = (mask & k.bit) ? 1 : 0;
    for (uint32_t code : kAliasCodes) keys[code] = 0;
}

inline uint8_t MaskOf(const volatile uint8_t* held) {
    uint8_t mask = 0;
    for (const auto& k : kKeys) {
        if (held[k.code]) mask |= k.bit;
    }
    return mask;
}

inline void Write(volatile uint8_t* held, uint8_t mask) {
    for (const auto& k : kKeys) held[k.code] = (mask & k.bit) ? 1 : 0;
}

// What the observer held each tick for a take made by injection (before
// v56): out[k] = the mask held during tick k. Tick t queued one event per
// changed bit (LEFT, RIGHT, UP, DOWN, JUMP, SHIFT, in that order) and the
// next tick's Update applied them: at most one event per key, and the event
// right after an applied one waits for the following Update (it erases the
// event, then also advances past the next). Stamps were always in the past.
inline void SimulateInjected(const uint8_t* log, uint32_t count, uint8_t* out) {
    struct Event {
        uint8_t bit;
        uint8_t pressed;
    };
    constexpr uint32_t kCap = 128;  // the observer's queue; more is dropped
    Event queue[kCap];
    uint32_t queued = 0;
    uint8_t held = 0, prev = 0;
    for (uint32_t t = 0; t < count; t++) {
        // Update before tick t's physics.
        uint8_t applied_bits = 0;
        uint32_t i = 0;
        while (i < queued) {
            const Event e = queue[i];
            if (applied_bits & e.bit) {
                i++;  // one event per key per Update
                continue;
            }
            held = e.pressed ? (uint8_t)(held | e.bit) : (uint8_t)(held & ~e.bit);
            applied_bits |= e.bit;
            for (uint32_t j = i; j + 1 < queued; j++) queue[j] = queue[j + 1];
            queued--;
            i++;  // skips the event now at i
        }
        out[t] = held;
        // Tick t injects its transitions for the next Update.
        const uint8_t transitions = log[t] ^ prev;
        for (uint8_t bit = 1; bit < 0x40; bit <<= 1) {
            if ((transitions & bit) && queued < kCap) {
                queue[queued++] = {bit, (uint8_t)((log[t] & bit) ? 1 : 0)};
            }
        }
        prev = log[t];
    }
}

// A legacy (injected) take converted to held masks at a CONT splice: the held
// state the replay produced where it was seen, else the simulated one. In
// place; `seen` marks the indices observed[] holds. Returns how many seen
// ticks disagreed with the simulation.
inline uint32_t ConvertInjectedPrefix(uint8_t* log, const uint8_t* observed, const uint8_t* seen,
                                      uint32_t count, uint8_t* scratch) {
    SimulateInjected(log, count, scratch);
    uint32_t disagreements = 0;
    for (uint32_t i = 0; i < count; i++) {
        if (seen[i]) {
            if (observed[i] != scratch[i]) disagreements++;
            log[i] = observed[i];
        } else {
            log[i] = scratch[i];
        }
    }
    return disagreements;
}

}  // namespace heldkeys
