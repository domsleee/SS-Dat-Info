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
and `just test_live` has passed against it. The current recovery rewrite has
offline fault-injection coverage, but has not yet passed board/live validation.
Run `python -m unittest discover -s TAS/pico -p test_firmware.py -v`, or
`just check_all`, to run its production controller against simulated HID/CDC failures.

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
- `0xFD` or `0xFE`: attempt to release keys, then reset the MCU using CircuitPython's
  reset API. The former register-level soft reconnect is removed. Reopen the COM
  port after it reappears; the UI's Reset Pico button closes its stale handle.
- `0xFF` replies with `[0x5A, release_ok, 1]` (protocol version 1). The UI Health
  Check discards old replies and requires a fresh successful acknowledgement.
  Older firmware without an ACK now fails this explicit check. Connecting alone
  and the live harness's write-only port checks do not establish firmware liveness.

The firmware processes mask edges in order, including press/release in one read.
Reset discards the rest of its batch. Keys latch until another mask changes them
or 500 ms passes without a command. A successful serial write alone is not proof
that the firmware processed it. ACK writes are non-blocking and skipped while a
reply is queued, so write-only hosts cannot stall the firmware by leaving replies unread.

Startup sends an unconditional neutral HID report. After a HID error, the firmware
recreates the keyboard and keeps retrying neutral reports at 50 ms intervals until
one succeeds. It does not mark failed key reports as applied or forget failed
releases. Commands received while neutral recovery is pending are discarded rather
than replaying stale steering after recovery. A successful release ACK proves the
HID call returned, not that Windows or the game consumed the report.

The timeout affects recorded input: the harness's default 56-tick hold is
560 ms at 100 Hz. Changing the timeout or adding keepalives changes test inputs
and must be validated with the live regression suite.

## Troubleshooting and recovery

If the configured COM port is missing, check the device's current CDC data port
before retrying. A missing port does not by itself identify a firmware crash or
prove that the HID interface disappeared.

If the port opens but keys do not work, use the UI Health Check. Reset Pico can
recover an executing firmware loop, but cannot reach firmware that is completely
stuck or a device missing from USB. This rewrite does not claim to fix physical
USB faults or every cause of replugging, and does not install an automatic reset loop.

For a recoverable board, restore `code.py` and `boot.py` using the deploy script.
If CIRCUITPY is unavailable, CircuitPython safe mode bypasses `boot.py`.
BOOTSEL while plugging in enters the ROM bootloader for reinstalling
CircuitPython, the Adafruit HID library and these firmware files.
