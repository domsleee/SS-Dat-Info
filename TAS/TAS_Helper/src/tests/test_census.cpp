// census.hpp: options, sample selection, RTTI decoding (a synthetic
// object/vtable/locator laid out in a buffer, then this exe's own RTTI),
// target kinds against live memory, and sidecar publication.
#include "../census.hpp"
#include <map>
#include "check.hpp"

// Synthetic address space: [base, base + bytes) readable; rodata and code
// sub-ranges answer ReadOnlyImage / Code.
struct FakeMem {
    uint32_t base = 0x40000000;
    std::vector<uint8_t> bytes = std::vector<uint8_t>(0x6000, 0);
    uint32_t ro_lo = 0x40001000, ro_hi = 0x40002000, code_lo = 0x40000000, code_hi = 0x40001000;
    bool Read(uint32_t a, void* d, uint32_t n) const {
        if (a < base || a - base + n > bytes.size()) return false;
        memcpy(d, &bytes[a - base], n);
        return true;
    }
    bool ReadOnlyImage(uint32_t a) const { return a >= ro_lo && a < ro_hi; }
    bool Code(uint32_t a) const { return a >= code_lo && a < code_hi; }
    void Put(uint32_t a, uint32_t v) { memcpy(&bytes[a - base], &v, 4); }
    void Str(uint32_t a, const char* s) { memcpy(&bytes[a - base], s, strlen(s) + 1); }
};

struct Probe {
    virtual ~Probe() {}
    int x = 7;
};
namespace Outer {
struct Inner : Probe {
    int y = 9;
};
}

int main() {
    // ---- demangling
    check(census::Demangle(".?AVBody@Supreme_Snowboarding@Housemarque@@") == "Housemarque::Supreme_Snowboarding::Body",
          "demangle_nested");
    check(census::Demangle(".?AUPoint@@") == "Point", "demangle_struct");
    check(census::Demangle(".?AV?$vector@H@std@@") == "?$vector@H@std", "demangle_template_kept");
    check(census::Demangle(".?AW4Enum@@").empty() && census::Demangle("garbage").empty(), "demangle_rejects");

    // ---- chains (the region syntax)
    census::Chain c;
    check(census::ParseChain("body>0xB8+0x4", &c) && c.base == "body" && c.derefs.size() == 1 && c.derefs[0] == 0xB8 &&
              c.add == 4,
          "chain_parse");
    check(!census::ParseChain("player+4>8", &c), "chain_add_last");
    uint32_t cell[2] = {0, 0};
    uint32_t target[2] = {0x11, 0x22};
    cell[1] = (uint32_t)(uintptr_t)target;
    census::ParseChain("x>4+4", &c);
    check(census::Resolve(c, (uint32_t)(uintptr_t)cell) == (uint32_t)(uintptr_t)&target[1], "chain_resolve");
    cell[1] = 0;
    check(census::Resolve(c, (uint32_t)(uintptr_t)cell) == 0, "chain_null_stops");

    // ---- options
    census::Options o;
    std::string err;
    check(census::ParseOptions("# defaults\n\n", &o, &err) && o.every == 500 && !o.readset && o.summary.empty(),
          "options_defaults");
    check(census::ParseOptions("every 100\r\nreadset\nreadset_ticks 1695-1710,1800\nmax_exceptions 5000\n"
                               "guard body+0xC0 0xC0\nhw body+0xC0\nhw body>0xB8+0x4\nhw_ticks 1695-1710\n",
                               &o, &err) &&
              o.every == 100 && o.readset && o.rs_ticks.size() == 2 && o.max_exceptions == 5000 &&
              o.guards.size() == 1 && o.guards[0].len == 0xC0 && o.hw.size() == 2 && o.hw_ticks.size() == 1,
          "options_all");
    check(o.summary == "every 100;readset;readset_ticks 1695-1710,1800;max_exceptions 5000;guard body+0xC0 0xC0;"
                       "hw body+0xC0;hw body>0xB8+0x4;hw_ticks 1695-1710",
          "options_summary");
    check(census::ParseOptions("probe_ticks 1695-1710\n", &o, &err) && o.probe_ticks.size() == 1 && !o.readset &&
              census::WantsReadset(o),
          "options_probe");
    check(census::ParseOptions("readset_first 0\n", &o, &err) && o.readset && o.rs_first == 0, "options_readset_key_enables");
    check(!census::ParseOptions("hw body+0xC0\n", &o, &err), "options_hw_needs_ticks");
    check(!census::ParseOptions("hw body+2\nhw_ticks 1\n", &o, &err), "options_hw_aligned");
    check(!census::ParseOptions("hw a+0\nhw a+4\nhw a+8\nhw a+12\nhw a+16\nhw_ticks 1\n", &o, &err), "options_hw_four");
    check(!census::ParseOptions("readset_ticks 10-5\n", &o, &err), "options_bad_range");
    check(!census::ParseOptions("every x\n", &o, &err) && !census::ParseOptions("bogus 1\n", &o, &err), "options_unknown");

    // ---- sample selection
    census::ParseOptions("every 500\nreadset_ticks 1695-1697\n", &o, &err);
    check(census::CensusAt(o, 0, UINT32_MAX, true) && census::CensusAt(o, 1000, UINT32_MAX, false) &&
              !census::CensusAt(o, 7, UINT32_MAX, true),
          "census_fixed_samples");
    check(census::CensusAt(o, 7, 7, false) && !census::CensusAt(o, 7, 7, true) && census::CensusAt(o, 8, 7, true) &&
              !census::CensusAt(o, 8, 7, false),
          "census_first_entry");
    check(census::ReadsetAt(o, 0, 5) && census::ReadsetAt(o, 2, 5) && !census::ReadsetAt(o, 3, 5) &&
              census::ReadsetAt(o, 250, 5) && census::ReadsetAt(o, 9, 1696) && !census::ReadsetAt(o, 9, 1698),
          "readset_selection");

    // ---- synthetic RTTI: object -> vtable (rodata) -> locator -> type descriptor
    FakeMem m;
    const uint32_t td = 0x40003000, col = 0x40001000, vt = 0x40001018, obj = 0x40004000;
    m.Put(td, 0x40001100);
    m.Str(td + 8, ".?AVBody@Supreme_Snowboarding@Housemarque@@");
    m.Put(col + 0xC, td);                  // signature, offset, cdOffset = 0
    m.Put(vt - 4, col);
    m.Put(vt, 0x40000010);
    m.Put(vt + 4, 0x40000020);
    m.Put(vt + 8, 0x12345678);             // not code: the table ends
    m.Put(obj, vt);
    check(census::ObjectClass(m, obj) == "Housemarque::Supreme_Snowboarding::Body", "rtti_object");
    uint32_t start = 0;
    std::string cls;
    check(census::VtableAt(m, vt + 4, &start, &cls) && start == vt && cls == "Housemarque::Supreme_Snowboarding::Body",
          "rtti_inside_vtable");
    check(!census::VtableAt(m, vt + 8, &start, &cls), "rtti_past_vtable");
    check(census::ObjectClass(m, obj + 4).empty(), "rtti_not_an_object");
    m.Put(col, 1);
    check(census::ObjectClass(m, obj).empty(), "rtti_bad_signature");
    m.Put(col, 0);
    m.Str(td + 8, ".?AW4Enum@@");
    check(census::ObjectClass(m, obj).empty(), "rtti_bad_name");
    m.Str(td + 8, ".?AVBody@@");
    m.ro_lo = 0x40001100;                  // the vtable is no longer read-only image data
    check(census::ObjectClass(m, obj).empty(), "rtti_vtable_must_be_readonly");
    m.ro_lo = 0x40001000;
    check(census::ObjectClass(m, obj) == "Body", "rtti_restored");

    // ---- this exe's own RTTI and target kinds, against live memory
    // The game's allocator: msvcrt.dll's malloc (operator new calls it).
    HMODULE crt = LoadLibraryA("msvcrt.dll");
    using Malloc = void*(__cdecl*)(size_t);
    using Free = void(__cdecl*)(void*);
    auto crtMalloc = (Malloc)GetProcAddress(crt, "malloc");
    auto crtFree = (Free)GetProcAddress(crt, "free");
    void* game = crtMalloc(0x548);
    auto* inner = new Outer::Inner();
    census::HeapIndex hx;
    hx.Build();
    std::printf("  info heaps %zu, blocks %zu, msvcrt heap %s the process heap\n", hx.heaps.size(), hx.blocks.size(),
                hx.crt == GetProcessHeap() ? "IS" : "is NOT");
    const uint32_t stack = census::StackBase();
    const census::Block* b = nullptr;
    const uint32_t ip = (uint32_t)(uintptr_t)inner;
    std::string d = census::Describe(ip, hx, stack, &b);
    std::printf("  info %s\n", d.c_str());
    check(b && b->base == ip && b->size == sizeof(Outer::Inner), "heap_block_found");
    check(d.find("heap process ") == 0 && d.find(" rtti Outer::Inner") != std::string::npos, "heap_kind_and_rtti");
    d = census::Describe((uint32_t)(uintptr_t)game + 0x10, hx, stack, &b);
    std::printf("  info msvcrt malloc(0x548)+0x10: %s\n", d.c_str());
    check(d.find("heap crt ") == 0 && b && b->size == 0x548 && d.find(" +0x10") != std::string::npos,
          "msvcrt_block_found");
    d = census::Describe(ip + 8, hx, stack, &b);
    check(d.find(" +0x8 base_rtti Outer::Inner") != std::string::npos, "heap_interior_base_rtti");
    d = census::Describe(*(uint32_t*)inner, hx, stack);
    std::printf("  info %s\n", d.c_str());
    check(d.find("image ") == 0 && d.find(" .rdata vtable Outer::Inner +0x0") != std::string::npos, "image_vtable");
    int local = 0;
    check(census::Describe((uint32_t)(uintptr_t)&local, hx, stack) == "stack", "stack_kind");
    void* priv = VirtualAlloc(nullptr, 0x2000, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
    check(census::Describe((uint32_t)(uintptr_t)priv + 0x1004, hx, stack).find("private ") == 0 &&
              census::Describe((uint32_t)(uintptr_t)priv + 0x1004, hx, stack).find(" +0x1004") != std::string::npos,
          "private_kind");
    HANDLE map = CreateFileMappingA(INVALID_HANDLE_VALUE, nullptr, PAGE_READWRITE, 0, 0x1000, nullptr);
    void* view = MapViewOfFile(map, FILE_MAP_READ, 0, 0, 0);
    check(census::Describe((uint32_t)(uintptr_t)view + 4, hx, stack).find("mapped ") == 0, "mapped_kind");
    DWORD old;
    VirtualProtect(priv, 0x1000, PAGE_NOACCESS, &old);
    check(census::Describe((uint32_t)(uintptr_t)priv, hx, stack).empty(), "noaccess_is_not_a_target");
    check(census::Describe(0x10, hx, stack).empty(), "null_page_is_not_a_target");

    // ---- one sample into a published sidecar
    char dir[MAX_PATH], unique[MAX_PATH];
    GetTempPathA(MAX_PATH, dir);
    GetTempFileNameA(dir, "cen", 0, unique);
    DeleteFileA(unique);
    const std::string path = std::string(unique) + ".census";
    census::Sidecar s;
    check(s.Open(path, "SSBCENSUS 1\n"), "sidecar_open");
    check(GetFileAttributesA(path.c_str()) == INVALID_FILE_ATTRIBUTES, "sidecar_not_published_while_open");
    // null, a heap object, a PAGE_NOACCESS page (skipped), a vtable
    const uint32_t region[4] = {0, ip, (uint32_t)(uintptr_t)priv, *(uint32_t*)inner};
    const uint32_t n = census::Scan(s, hx, "0 -5 pre", {{"player", (const uint8_t*)region, sizeof region}});
    check(n == 2, "scan_counts_targets");
    check(s.Close() == 0, "sidecar_published");
    FILE* f = nullptr;
    std::string text;
    if (!fopen_s(&f, path.c_str(), "rb") && f) {
        char buf[4096];
        size_t k;
        while ((k = fread(buf, 1, sizeof buf, f))) text.append(buf, k);
        fclose(f);
    }
    char want[64];
    snprintf(want, sizeof want, "ptr 0 -5 pre player 0x4 0x%x heap ", ip);
    check(text.find("SSBCENSUS 1\n") == 0 && text.find(want) != std::string::npos &&
              text.find("player 0xc ") != std::string::npos && text.size() > 6 &&
              text.compare(text.size() - 6, 6, "end 2\n") == 0,
          "sidecar_contents");
    check(GetFileAttributesA((path + ".pending").c_str()) == INVALID_FILE_ATTRIBUTES, "sidecar_pending_gone");
    census::Sidecar again;
    again.Open(path, "other\n");
    check(again.Close() != 0, "sidecar_never_replaces");
    DeleteFileA(path.c_str());
    delete inner;
    crtFree(game);
    return FinishTests();
}
