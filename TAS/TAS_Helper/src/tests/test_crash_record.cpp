// Unit tests for crash_record.hpp: which exceptions are faults, and resolving a
// fault address to module + offset. Pure logic, no Windows/hook deps.

#include "../crash_record.hpp"
#include "check.hpp"
#include <cstring>

int main() {
    using namespace crashrecord;
    std::printf("crash_record tests:\n");

    check(IsFault(0xC0000005), "access_violation_is_a_fault");
    check(IsFault(0xC00000FD), "stack_overflow_is_a_fault");
    check(IsFault(0xC0000094), "divide_by_zero_is_a_fault");
    check(!IsFault(0xE06D7363), "cpp_throw_is_not_a_fault");
    check(!IsFault(0x80000003), "breakpoint_is_not_a_fault");
    check(!IsFault(0x40010006), "debug_print_is_not_a_fault");

    ModuleTable t;
    t.entries[0] = {0x00400000, 0x80000, "Supreme.exe"};
    t.entries[1] = {0x10000000, 0x1000, "Kernel.dll"};
    t.count = 2;
    check(t.Find(0x00400000) == 0, "module_base_is_inside");
    check(t.Find(0x0047FFFF) == 0, "module_last_byte_is_inside");
    check(t.Find(0x00480000) == -1, "module_end_is_outside");
    check(t.Find(0x10000800) == 1, "second_module_found");
    check(t.Find(0x003FFFFF) == -1, "below_every_module_is_outside");
    check(t.Find(0) == -1, "null_is_outside");
    ModuleTable empty;
    check(empty.Find(0x00400000) == -1, "empty_table_finds_nothing");

    return FinishTests();
}
