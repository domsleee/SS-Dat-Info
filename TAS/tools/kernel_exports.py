import pefile
pe = pefile.PE(r"T:\Games\SupremeORIG\HMG_Kernel.dll")
print(f"ImageBase={pe.OPTIONAL_HEADER.ImageBase:#x}")
for e in pe.DIRECTORY_ENTRY_EXPORT.symbols:
    name = e.name.decode() if e.name else None
    if name and ('Time' in name or 'Clock' in name or 'Tick' in name):
        print(f"  ord={e.ordinal} rva={e.address:#x} {name}")
