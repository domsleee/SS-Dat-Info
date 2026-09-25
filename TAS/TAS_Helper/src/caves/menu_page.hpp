#pragma once
// Page-identity state for the menu command channel, split from the hooks so
// the standalone suites can test it without the hooking library.
#include "../shared_state.hpp"
#include <cstring>

namespace menustate {

inline char g_changedName[TAS_MENU_SCREEN_MAX] = {};   // from the last Change_Page
inline volatile uint32_t g_pageGen = 0;                 // bumped by every Change_Page
inline uint32_t g_snapGen = 0;                          // the generation the snapshot has adopted
inline char g_screen[TAS_MENU_SCREEN_MAX] = {};         // the current page id

// Adopt a pending page change into the cached screen. Both callers run on
// the Execute hook's thread, so no locking.
static bool AdoptPendingPage() {
    const uint32_t gen = g_pageGen;
    if (gen == g_snapGen) return false;
    g_snapGen = gen;
    memcpy(g_screen, g_changedName, sizeof g_screen);
    return true;
}

// The submitted page must be the adopted one. An empty submission is an
// unchecked wildcard (see menu_command_submit).
static uint32_t ValidateScreen(const char* screen) {
    if (!g_screen[0]) return TAS_MENU_RESULT_NO_MENU;
    if (screen[0] && strncmp(screen, g_screen, TAS_MENU_SCREEN_MAX) != 0)
        return TAS_MENU_RESULT_STALE_PAGE;
    return TAS_MENU_RESULT_OK;
}

}  // namespace menustate
