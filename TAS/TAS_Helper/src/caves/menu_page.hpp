#pragma once
// Page-identity state for the menu command channel, without the hooks: the
// cached screen id, its generation tracking, adoption and the submitted-page
// check. Separated so the standalone suites can drive the exact adoption +
// validation the Execute path uses, without linking the hooking library.
#include "../shared_state.hpp"
#include <cstring>

namespace menustate {

inline char g_changedName[TAS_MENU_SCREEN_MAX] = {};   // from the last Change_Page
inline volatile uint32_t g_pageGen = 0;                 // bumped by every Change_Page
inline uint32_t g_snapGen = 0;                          // the generation the snapshot has adopted
inline char g_screen[TAS_MENU_SCREEN_MAX] = {};         // the current page id

// Adopt a pending page change into the cached screen. The 50 ms snapshot
// throttle must not let a command validate against the page it just left:
// ConsumeCommand adopts first, Snapshot reuses the same adoption. Same
// thread on both paths (Execute hook), so no locking.
static bool AdoptPendingPage() {
    const uint32_t gen = g_pageGen;
    if (gen == g_snapGen) return false;
    g_snapGen = gen;
    memcpy(g_screen, g_changedName, sizeof g_screen);
    return true;
}

// Pure page check: the submitted page must be the adopted one, else the
// command acts on a page the agent never saw. An empty submission is the
// unchecked wildcard (see menu_command_submit).
static uint32_t ValidateScreen(const char* screen) {
    if (!g_screen[0]) return TAS_MENU_RESULT_NO_MENU;
    if (screen[0] && strncmp(screen, g_screen, TAS_MENU_SCREEN_MAX) != 0)
        return TAS_MENU_RESULT_STALE_PAGE;
    return TAS_MENU_RESULT_OK;
}

}  // namespace menustate
