import pefile
pe = pefile.PE(r"T:\Games\SupremeORIG\HMG_Cetsup_Win32.dll")
print("Exports:")
if hasattr(pe, 'DIRECTORY_ENTRY_EXPORT'):
    for e in pe.DIRECTORY_ENTRY_EXPORT.symbols:
        name = e.name.decode() if e.name else None
        print(f"  ord={e.ordinal} rva={e.address:#x} name={name}")
print("Imports:")
for imp in getattr(pe, 'DIRECTORY_ENTRY_IMPORT', []):
    print(" ", imp.dll.decode())
