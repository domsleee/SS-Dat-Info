import pefile
pe = pefile.PE(r"T:\Games\SupremeORIG\Supreme_Game.dll", fast_load=True)
print(f"ImageBase={pe.OPTIONAL_HEADER.ImageBase:#x} SizeOfImage={pe.OPTIONAL_HEADER.SizeOfImage:#x}")
for s in pe.sections:
    name = s.Name.rstrip(b"\0").decode()
    w = "W" if s.Characteristics & 0x80000000 else "-"
    print(f"{name:10s} rva={s.VirtualAddress:#9x} vsize={s.Misc_VirtualSize:#9x} {w}")
