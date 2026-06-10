"""Scan HMG_Cetsup_Win32.dll for direct call/jmp sites targeting a given RVA
(default 0x3940, the keyDown handler). Direct E8 rel32 scan over all
executable sections."""
import sys
import struct
import pefile

DLL = r"T:\Games\SupremeORIG\HMG_Cetsup_Win32.dll"
target_rva = int(sys.argv[1], 16) if len(sys.argv) > 1 else 0x3940

pe = pefile.PE(DLL)
base = pe.OPTIONAL_HEADER.ImageBase
data = pe.get_memory_mapped_image()
target_va = base + target_rva

hits = []
for sec in pe.sections:
    if not sec.Characteristics & 0x20000000:  # IMAGE_SCN_MEM_EXECUTE
        continue
    start = sec.VirtualAddress
    end = start + sec.Misc_VirtualSize
    for i in range(start, end - 5):
        op = data[i]
        if op not in (0xE8, 0xE9):
            continue
        rel = struct.unpack_from("<i", data, i + 1)[0]
        dest = base + i + 5 + rel
        if dest == target_va:
            hits.append((i, "call" if op == 0xE8 else "jmp"))

print(f"target RVA {target_rva:#x} (VA {target_va:#x}): {len(hits)} direct hits")
for rva, kind in hits:
    print(f"  {kind} at RVA {rva:#x} (VA {base+rva:#x})")

# Also check if target VA appears as an immediate/pointer anywhere (vtables,
# function-pointer tables, push <imm32> for thunks/callbacks).
needle = struct.pack("<I", target_va)
pos = data.find(needle)
ptr_hits = []
while pos != -1:
    ptr_hits.append(pos)
    pos = data.find(needle, pos + 1)
print(f"raw pointer occurrences of {target_va:#x}: {[hex(p) for p in ptr_hits]}")
