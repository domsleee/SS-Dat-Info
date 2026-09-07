// Standalone unit tests for the menu model (no game, no Windows APIs): the JSON
// document an agent reads and the target matching a command uses. The live
// path (menu_state.hpp) only fills the snapshot; everything an agent depends
// on - the document's shape, escaping, and which item "activate X" picks - is
// decided here and pinned here.
#include "../menu_model.hpp"
#include "check.hpp"
#include <cstdio>
#include <cstring>
#include <string>

static void Put(menumodel::MenuSnapshot& s, const char* id, const char* label, bool enabled = true) {
    if (s.count >= menumodel::kMaxItems) return;
    auto& it = s.items[s.count++];
    std::strncpy(it.name, id, sizeof it.name - 1);
    std::strncpy(it.label, label, sizeof it.label - 1);
    it.enabled = enabled ? 1 : 0;
    it.visible = 1;
    it.comp = 0x1000 * s.count;
}

// The DLL writes into shm's fixed buffer; the tests read the same bytes back.
static std::string Doc(const menumodel::MenuSnapshot& s, const char* screen) {
    char buf[4096];
    menumodel::BuildDoc(s, screen, buf, sizeof buf);
    return buf;
}

static bool Has(const std::string& doc, const char* needle) { return doc.find(needle) != std::string::npos; }

int main() {
    using namespace menumodel;

    std::printf("menu_model tests:\n");

    // The Arcade page as the DLL sees it, plus the two shapes that matter: a
    // button without an id (the track select's environments) and a disabled one.
    MenuSnapshot s;
    Put(s, "ID_ARCADE_TIME_ATTACK_SEQUENCE", "Time Attack");
    Put(s, "ID_ARCADE_RACE_SEQUENCE", "Race");
    Put(s, "", "Village");
    Put(s, "ID_QUIT", "Quit", false);
    s.selector = 1;
    s.items[1].focused = 1;

    // --- The document. ---
    const std::string d = Doc(s, "ID_ARCADE_MENU");
    check(d.rfind("{\"screen\":\"ID_ARCADE_MENU\",\"sel\":1,\"items\":[", 0) == 0,
          "document opens with the screen and the selector");
    check(Has(d, "{\"label\":\"Time Attack\",\"id\":\"ID_ARCADE_TIME_ATTACK_SEQUENCE\",\"en\":true,\"vis\":true}"),
          "an item carries label, id and flags");
    check(Has(d, "{\"label\":\"Village\",\"id\":\"\",\"en\":true,\"vis\":true}"),
          "an id-less button keeps an empty id (never invented)");
    check(Has(d, "\"label\":\"Quit\",\"id\":\"ID_QUIT\",\"en\":false"), "a disabled item says so");
    check(d.size() >= 2 && d[d.size() - 1] == '}' && d[d.size() - 2] == ']',
          "document closes the array and the object");
    check(Has(d, "},{"), "items are comma-separated");

    MenuSnapshot none;
    check(Doc(none, "ID_X") == "{\"screen\":\"ID_X\",\"sel\":null,\"items\":[]}",
          "nothing focused = sel null, no items");
    check(Doc(s, "") == "", "no screen = no document");
    check(Doc(s, nullptr) == "", "null screen = no document");

    MenuSnapshot q;
    Put(q, "ID_A", "Say \"hi\" \\ bye");
    check(Has(Doc(q, "ID_S"), "\"label\":\"Say \\\"hi\\\" \\\\ bye\""), "quotes and backslashes are escaped");

    char exact[64];
    const uint32_t n = BuildDoc(none, "ID_X", exact, sizeof exact);
    check(n == std::strlen(exact) && n == 39, "BuildDoc returns the length it wrote");
    char tiny[16];
    check(BuildDoc(s, "ID_ARCADE_MENU", tiny, sizeof tiny) == 0 && tiny[0] == 0,
          "a document that does not fit is empty, never cut off");

    // --- Target matching: what "activate X" picks. ---
    check(FindTarget(s, "ID_ARCADE_RACE_SEQUENCE") == 1, "by id");
    check(FindTarget(s, "id_arcade_race_sequence") == 1, "id is case-insensitive");
    check(FindTarget(s, "Race") == 1, "by label");
    check(FindTarget(s, "time attack") == 0, "label is case-insensitive");
    check(FindTarget(s, "Village") == 2, "an id-less button is reachable by its label");
    check(FindTarget(s, "") == -1, "an empty target matches nothing (not the id-less button)");
    check(FindTarget(s, nullptr) == -1, "a null target matches nothing");
    check(FindTarget(s, "ID_NOPE") == -1, "an unknown id does not fall back to labels");
    check(FindTarget(s, "Rac") == -1, "no prefix matching");
    check(FindTarget(s, "Quit") == 3, "a disabled item is still found (the caller reports DISABLED)");
    check(FindTarget(none, "Race") == -1, "empty page: nothing to find");

    // The id is preferred even when a LABEL elsewhere equals the target.
    MenuSnapshot t;
    Put(t, "ID_RACE", "Race");
    Put(t, "Race", "Something else");   // an id that reads like a label
    check(FindTarget(t, "Race") == 1, "an exact id match beats a label match");

    return FinishTests();
}
