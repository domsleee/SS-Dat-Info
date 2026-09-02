#pragma once
#include <cstdint>

// Which replay-recorder object the DLL follows (pure logic, unit-tested in
// tests/test_replay_capture.cpp).
//
// The replay-capture hook (SG+0x9E8F0, the recorder's per-frame "push 112-byte
// frame") hands us ECX = the recorder object; cave2 derives the player from
// [recorder+0x84] every cycle and reads the position from there.
//
// What the site actually sees (2026-09-02, ring-logged in-game):
//   - Steady state: exactly ONE recorder pushes, once per Supreme::Cycle - the
//     human's. This holds with Time Attack ghosts enabled too.
//   - Around an F5 restart (reload window and countdown) OTHER recorder
//     objects push on some frames - ghosts / AI riders / the guide being
//     (re)built - some with a garbage owner (+0x84 == 2). With ghosts the
//     human's own recorder and player are re-created at new addresses as well.
//
// Two policies failed on that:
//   - "freeze the pointer while REC/PLAY" (the original): with ghosts a judged
//     PLAY kept reading a dead object after the restart - every attempt
//     mismatched at gate+0 and rerolled forever.
//   - "follow whatever pushed last": a transient pusher during the countdown
//     hijacks the position source mid-run - FE without ghosts went from ~0 to
//     5-13 rerolls per PLAY.
//
// So the hook decides by IDENTITY, not by timing: a recorder is adopted iff
// its owner is the keyboard-driven player ([[player+0x1B8]+0x590] ==
// [root+0x530], see GameAddresses). That is true for the human's recorder
// whether it was just re-created (adopt, even mid-run) and false for ghosts,
// AI riders and garbage owners (ignore, however often they push).
struct ReplayCaptureState {
    uint32_t cached = 0;                 // recorder the DLL currently follows
    uint32_t changes_while_active = 0;   // re-creations adopted during REC/PLAY (diagnostic)
    uint32_t rejected = 0;               // non-human pushers ignored (diagnostic)
};

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
