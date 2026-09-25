#pragma once
#include <cstdint>

// Which replay recorder the DLL follows (pure logic, tested in
// tests/test_replay_capture.cpp).
//
// In a steady race only the human's recorder pushes, once per Supreme::Cycle.
// Around an F5 restart other recorders (ghosts, AI, the guide) push too, some
// with a garbage owner (+0x84 == 2), and with ghosts the human's recorder is
// re-created at a new address. So the choice is by identity, not timing: a
// recorder is adopted, even mid-run, iff its owner is a plain `Player` that
// links back to it (replay_identity.hpp).
struct ReplayCaptureState {
    uint32_t cached = 0;                 // recorder the DLL currently follows
    uint32_t changes_while_active = 0;   // re-creations adopted during REC/PLAY (diagnostic)
    uint32_t rejected = 0;               // non-human pushers ignored (diagnostic)
    uint32_t dropped = 0;                // cached recorder that stopped being the human's (diagnostic)
};

// The cached recorder pushed again; drop it if it is no longer the human's
// (after F5 its address can be reused by a ghost). Returns true when dropped.
inline bool ReplayCaptureRevalidate(bool still_human, ReplayCaptureState& st) {
    if (st.cached == 0 || still_human) return false;
    st.cached = 0;
    st.dropped++;
    return true;
}

// Returns true when `incoming` replaced the cached recorder.
inline bool ReplayCaptureAdopt(bool mode_off, uint32_t incoming, bool incoming_is_human,
                               ReplayCaptureState& st) {
    if (incoming == 0 || incoming == st.cached) return false;
    if (!incoming_is_human) {
        st.rejected++;
        return false;
    }
    if (!mode_off && st.cached != 0) st.changes_while_active++;
    st.cached = incoming;
    return true;
}
