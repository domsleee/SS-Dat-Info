// Stale-page validation for the menu command channel (no game, no hooks):
// a command names the page the agent read, and must be refused when the page
// moved on. Drives the real globals in menu_page.hpp through adoption and
// validation — the Execute path adopts before consuming for exactly this.
#include "../caves/menu_page.hpp"
#include "check.hpp"
#include <cstdio>
#include <cstring>
using namespace menustate;

static void SetPageEvent(const char* name, uint32_t gen) {
    std::strncpy(g_changedName, name, TAS_MENU_SCREEN_MAX - 1);
    g_changedName[TAS_MENU_SCREEN_MAX - 1] = '\0';
    g_pageGen = gen;
}

static void ResetPages() {
    g_changedName[0] = '\0';
    g_screen[0] = '\0';
    g_pageGen = 0;
    g_snapGen = 0;
}

int main() {
    std::printf("menu_stale_page tests:\n");

    // No page known: everything refuses, including the empty wildcard.
    ResetPages();
    check(ValidateScreen("ID_A") == TAS_MENU_RESULT_NO_MENU, "unknown page refuses a named command");
    check(ValidateScreen("") == TAS_MENU_RESULT_NO_MENU, "unknown page refuses even the wildcard");

    // Page A arrives via Change_Page. Before adoption the cache is still
    // empty, so validation cannot pass yet.
    SetPageEvent("ID_A", 1);
    check(ValidateScreen("ID_A") == TAS_MENU_RESULT_NO_MENU, "pre-adoption cache still empty");
    check(AdoptPendingPage(), "adoption reports the pending generation");
    check(!AdoptPendingPage(), "adoption is idempotent");
    check(ValidateScreen("ID_A") == TAS_MENU_RESULT_OK, "adopted page validates");
    check(ValidateScreen("ID_B") == TAS_MENU_RESULT_STALE_PAGE, "other page is stale");
    check(ValidateScreen("") == TAS_MENU_RESULT_OK, "empty submission stays unchecked");

    // Page changes to B. Before adoption the old page still validates — the
    // exact window Execute used to act in. After adoption the old page is
    // refused and only B validates.
    SetPageEvent("ID_B", 2);
    check(ValidateScreen("ID_A") == TAS_MENU_RESULT_OK, "pre-adoption still trusts the left page");
    check(AdoptPendingPage(), "second generation adopts");
    check(ValidateScreen("ID_A") == TAS_MENU_RESULT_STALE_PAGE, "left page refused after adoption");
    check(ValidateScreen("ID_B") == TAS_MENU_RESULT_OK, "new page validates after adoption");
    check(ValidateScreen("") == TAS_MENU_RESULT_OK, "wildcard survives a page change");

    return FinishTests();
}
