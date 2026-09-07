#pragma once
#include <cstdint>

// Which replay-recorder object the DLL follows (pure logic, unit-tested in
// tests/test_replay_capture.cpp).
//
// The replay-capture hook (SG+0x9E8F0, the recorder's per-frame "push 112-byte
// frame") hands us ECX = the recorder object; cave2 derives the player from
// [recorder+0x84] every cycle and reads the position from there.
//
// What the site sees (ring-logged in-game):
//   - Steady state: exactly ONE recorder pushes, once per Supreme::Cycle - the
//     human's. This holds with Time Attack ghosts enabled too.
//   - Around an F5 restart (reload window and countdown) OTHER recorder
//     objects push on some frames - ghosts / AI riders / the guide being
//     (re)built - some with a garbage owner (+0x84 == 2). With ghosts the
//     human's own recorder and player are re-created at new addresses as well.
//
// No timing rule survives both: a pointer frozen for the run reads a dead
// object after a ghost restart (every judged PLAY mismatches at gate+0), and
// following the last pusher lets a countdown transient hijack the position
// source mid-run. So the hook decides by IDENTITY: a recorder is adopted iff
// its owner is the keyboard-driven rider - a plain `Player`, still linking
// back to this recorder (replay_identity.hpp). That is true for the human's
// recorder whether it was just re-created (adopt, even mid-run) and false for
// ghosts (Ghost_Player), AI riders and garbage owners (ignore, however often
// they push).
struct ReplayCaptureState {
    uint32_t cached = 0;                 // recorder the DLL currently follows
    uint32_t changes_while_active = 0;   // re-creations adopted during REC/PLAY (diagnostic)
    uint32_t rejected = 0;               // non-human pushers ignored (diagnostic)
    uint32_t dropped = 0;                // cached recorder that stopped being the human's (diagnostic)
};

// The cached recorder pushed again: is it STILL the human's? Checked on every
// push of the cached address, not only when the pushing address changes: an
// F5 can free the human's recorder and the allocator can hand the same
// address to a ghost. Returns true when the cached recorder was dropped; the
// caller clears the published pointers and the next human push re-adopts
// through ReplayCaptureAdopt.
inline bool ReplayCaptureRevalidate(bool still_human, ReplayCaptureState& st) {
    if (st.cached == 0 || still_human) return false;
    st.cached = 0;
    st.dropped++;
    return true;
}

// Feed one hook invocation. `incoming_is_human` is the identity check above.
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
