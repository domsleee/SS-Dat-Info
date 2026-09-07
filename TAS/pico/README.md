# Pico HID keyboard

The TAS live tests use a Raspberry Pi Pico 2 as a USB HID keyboard for
Escape, F5 and steering. The firmware targets CircuitPython 10.1.3 on RP2350
and requires the Adafruit HID library on the board.

## Deploy and check

Edit the repository files, not the board. From the repository root:

```powershell
.\TAS\pico\deploy.ps1 -Check   # Compare firmware without changing the board
.\TAS\pico\deploy.ps1          # Show differences and ask before copying
```

The script finds the CIRCUITPY volume; use `-Drive E:` to select it explicitly.
It copies only `code.py` and `boot.py`. Their line endings are pinned in
`.gitattributes` because deployment checks compare bytes.

A firmware change is trusted only after it has been flashed with `deploy.ps1`
and `just test_live` has passed against it. The current `code.py`, which
processes every serial byte in order instead of only the last byte of a read,
has had neither yet.

`test.py` is a manual, host-side keyboard check, not firmware or an automated
regression test. It requires pyserial and sends real keys to the focused window:

```powershell
uv run TAS/pico/test.py COM7
```

Select the data port explicitly, then focus a harmless text editor during the
three-second countdown. Do not run this alongside a live game test.

## USB connection

The UI and the harness use `TAS_PICO_PORT` (default `COM7`) and check VID
`2E8A`, PID `000B` and interface `02`. These identify the USB interface, not
the firmware version.

Use the CDC **data** interface (`MI_02`), never the CDC console (`MI_00`).
Both have the same VID/PID; sending a mask such as `0x03` to the console can
be interpreted as Ctrl-C. COM numbers can change after re-enumeration.

## Protocol

Each byte sets the held-key mask, except the reserved commands below.

| Bit | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Key | LEFT | RIGHT | UP | DOWN | LCTRL (jump) | SHIFT | F5 | ESCAPE |

- `0x00` or `0xFF`: release all keys.
- `0xFD`: release keys and disconnect/reconnect USB.
- `0xFE`: release keys, disconnect/reconnect USB, then reset the MCU.

The firmware processes every byte of a read in order, so a press and a release
that arrive together both happen, and a control byte followed by a mask applies
both. Keys latch until another mask changes them or 500 ms passes without a
command. A successful serial write is not an acknowledgement that the firmware
processed the command.

The timeout affects recorded input: the harness's default 56-tick hold is
560 ms at 100 Hz. Changing the timeout or adding keepalives changes test inputs
and must be validated with the live regression suite.

## Troubleshooting and recovery

If the configured COM port is missing, check the device's current CDC data port
before retrying. A missing port does not by itself identify a firmware crash or
prove that the HID interface disappeared.

If the port opens but keys do not work, inspect the CDC console: the firmware
prints the exception there before releasing all keys.

For a recoverable board, restore `code.py` and `boot.py` using the deploy script.
If CIRCUITPY is unavailable, CircuitPython safe mode bypasses `boot.py`.
BOOTSEL while plugging in enters the ROM bootloader for reinstalling
CircuitPython, the Adafruit HID library and these firmware files.
