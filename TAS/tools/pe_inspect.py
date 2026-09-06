"""Read-only inspection of a PE file. All offsets are hexadecimal RVAs."""
import argparse
import struct


def direct_sites(data, start, end, target):
    """Byte-scan candidates, not proof of instruction boundaries."""
    for offset in range(max(0, start), min(end, len(data)) - 4):
        if data[offset] in (0xE8, 0xE9):
            relative = struct.unpack_from("<i", data, offset + 1)[0]
            if (offset + 5 + relative) & 0xFFFFFFFF == target:
                yield offset, "call" if data[offset] == 0xE8 else "jmp"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("file", help="path to an EXE or DLL (never loaded/executed)")
    commands = parser.add_subparsers(dest="command", required=True)
    commands.add_parser("sections")
    commands.add_parser("imports")
    exports = commands.add_parser("exports")
    exports.add_argument("--filter", nargs="+", default=[], help="name substrings (case-insensitive, OR)")
    disasm = commands.add_parser("disasm")
    disasm.add_argument("start", type=lambda s: int(s, 16))
    disasm.add_argument("end", type=lambda s: int(s, 16), help="exclusive end RVA")
    disasm.add_argument("--stop-at-ret", action="store_true")
    commands.add_parser("callers").add_argument("target", type=lambda s: int(s, 16))
    args = parser.parse_args()
    try:
        import pefile
    except ImportError:
        parser.error("install pefile: python -m pip install pefile")
    with pefile.PE(args.file) as pe:
        base = pe.OPTIONAL_HEADER.ImageBase
        print(f"ImageBase {base:#x} SizeOfImage {pe.OPTIONAL_HEADER.SizeOfImage:#x}")
        if args.command == "sections":
            for section in pe.sections:
                name = section.Name.rstrip(b"\0").decode(errors="replace")
                flags = "".join(letter for bit, letter in ((0x20000000, "X"), (0x40000000, "R"), (0x80000000, "W"))
                                if section.Characteristics & bit)
                print(f"{name:8s} RVA={section.VirtualAddress:#x} size={section.Misc_VirtualSize:#x} {flags}")
        elif args.command == "imports":
            for entry in getattr(pe, "DIRECTORY_ENTRY_IMPORT", []):
                print(entry.dll.decode(errors="replace"))
        elif args.command == "exports":
            directory = getattr(pe, "DIRECTORY_ENTRY_EXPORT", None)
            for symbol in directory.symbols if directory else []:
                name = symbol.name.decode(errors="replace") if symbol.name else "<unnamed>"
                if not args.filter or any(part.lower() in name.lower() for part in args.filter):
                    print(f"{symbol.ordinal:5d} RVA={symbol.address:#x} {name}")
        else:
            data = pe.get_memory_mapped_image()
            if args.command == "disasm":
                if not 0 <= args.start < args.end <= len(data):
                    parser.error("disassembly range must lie inside the mapped image")
                try:
                    import capstone
                except ImportError:
                    parser.error("install capstone: python -m pip install capstone")
                if pe.FILE_HEADER.Machine not in (0x14C, 0x8664):
                    parser.error("disassembly supports x86/x64 only")
                mode = capstone.CS_MODE_64 if pe.FILE_HEADER.Machine == 0x8664 else capstone.CS_MODE_32
                decoder = capstone.Cs(capstone.CS_ARCH_X86, mode)
                for instruction in decoder.disasm(data[args.start:args.end], args.start):
                    print(f"{instruction.address:08x}: {instruction.mnemonic:8s} {instruction.op_str}")
                    if args.stop_at_ret and instruction.mnemonic.startswith("ret"):
                        break
            else:
                if pe.FILE_HEADER.Machine != 0x14C:
                    parser.error("caller scan supports 32-bit x86 only")
                if not 0 <= args.target < len(data):
                    parser.error("target RVA must lie inside the mapped image")
                print(f"target RVA {args.target:#x} (VA {base + args.target:#x})")
                for section in pe.sections:
                    if section.Characteristics & 0x20000000:
                        for offset, kind in direct_sites(data, section.VirtualAddress,
                                                         section.VirtualAddress + section.Misc_VirtualSize, args.target):
                            print(f"  candidate {kind} at RVA {offset:#x} (VA {base + offset:#x})")
                needle = struct.pack("<I", (base + args.target) & 0xFFFFFFFF)
                offset = data.find(needle)
                while offset != -1:
                    print(f"  raw pointer at RVA {offset:#x}")
                    offset = data.find(needle, offset + 1)


if __name__ == "__main__":
    main()
