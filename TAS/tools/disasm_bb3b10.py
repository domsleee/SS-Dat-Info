"""Offline disassembly of HMG_Cetsup_Win32.dll around BB3B10 (HMG+0x3B10).

Goal: find where the dynamic 4th arg (the per-race-load counter) is COMPARED
against a memory location inside the observer-broadcast path. That memory
operand is the address cave2 should read live instead of calibrating from
keypresses.
"""
import sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

DLL = r"T:\Games\SupremeORIG\HMG_Cetsup_Win32.dll"
pe = pefile.PE(DLL)
base = pe.OPTIONAL_HEADER.ImageBase
print(f"ImageBase = {base:#x}")

data = pe.get_memory_mapped_image()

md = Cs(CS_ARCH_X86, CS_MODE_32)
md.detail = True

def disasm(rva, n_bytes=0x200, label=""):
    print(f"\n=== {label} RVA {rva:#x} (VA {base+rva:#x}) ===")
    code = data[rva:rva + n_bytes]
    for insn in md.disasm(code, base + rva):
        print(f"{insn.address - base:#07x}  {insn.mnemonic:8s} {insn.op_str}")
        if insn.mnemonic == "ret":
            break

rva = int(sys.argv[1], 16) if len(sys.argv) > 1 else 0x3B10
size = int(sys.argv[2], 16) if len(sys.argv) > 2 else 0x200
disasm(rva, size, f"func+{rva:#x}")
