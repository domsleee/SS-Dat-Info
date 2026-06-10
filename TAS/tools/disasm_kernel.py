"""Linear-disassemble an RVA range of HMG_Kernel.dll."""
import sys
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_32

DLL = r"T:\Games\SupremeORIG\HMG_Kernel.dll"
start = int(sys.argv[1], 16)
end = int(sys.argv[2], 16)

pe = pefile.PE(DLL)
base = pe.OPTIONAL_HEADER.ImageBase
data = pe.get_memory_mapped_image()

md = Cs(CS_ARCH_X86, CS_MODE_32)
for insn in md.disasm(data[start:end], base + start):
    print(f"{insn.address - base:#07x}  {insn.mnemonic:8s} {insn.op_str}")
