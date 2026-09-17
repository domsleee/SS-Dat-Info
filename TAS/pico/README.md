# Pico HID keyboard

The TAS live tests use a Raspberry Pi Pico 2 as a USB HID keyboard for
Escape, F5, menu confirmation and steering. The firmware targets CircuitPython 10.1.3 on RP2350
and requires the Adafruit HID library on the board.

## Deploy and check

Edit the repository files, not the board. From the repository root:

```powershell
.\TAS\pico\deploy.ps1 -Check   # Read-only file comparison
.\TAS\pico\deploy.ps1          # Confirm, back up, pause, update, reboot, verify
.\TAS\target\release\tas_test.exe pico  # Discover drive/ports, verify files and live ACK
sudo pwsh -File .\TAS\pico\deploy.ps1 -Recover -Force  # Restart this USB device first
```

Discovery follows Windows PnP parent chains from the disk and both CDC interfaces
to one physical Pico USB device. Volume labels, disk numbers and COM numbers are
not identities. Multiple boards are rejected unless selected with `-Serial UID`
(or `TAS_PICO_SERIAL`); the updater also accepts `-Drive E:` as a selector, never
as a substitute for the physical-device check. After resets it rediscovers the
same USB serial even if drive letters or ports change. `tas_test pico` prints the
identity as JSON and the live-suite preflight uses the same discovery and verification.
An explicit `TAS_PICO_PORT` belonging to a different board is rejected.

Close the game and TAS controllers before updating. The updater backs up board
files under `TAS/artifacts/pico-updates/<serial>/<unique-id>/`, then resets through
the console to clear stale device storage state. It waits for a verified REPL
prompt before staging, flushing, hashing and replacing `boot.py` and `code.py`.
REPL pause prevents auto-reload while the pair is incomplete. A final full reset
applies boot.py too; success requires matching hashes AND a live release ACK.
On failure it retains the backup and reports failure, never formats the board,
disables filesystem protection or automatically restarts a partial copy.
The two-file replacement is not power-loss atomic; restore the backup if interrupted.

If even the console is unresponsive, `-Recover` uses elevated `pnputil` to restart
only the selected composite USB device before resetting via the console. This is
the recovery sequence that cleared the observed read-only/serial-timeout state;
clearing Windows disk attributes alone did not. `-Check` never resets or sends HID
commands and cannot be combined with `-Recover`. Firmware line endings remain pinned.

A firmware change is trusted only after it has been flashed with `deploy.ps1`
and `just test_live` has passed against it. The current recovery rewrite has
offline fault-injection coverage and basic board verification; the full game suite
is still separate and has not yet passed against this rewrite.
Run `python -m unittest discover -s TAS/pico -p test_firmware.py -v`, or
`just check_all`, to run its production controller against simulated HID/CDC failures.
That gate also tests physical-device selection with mocked Windows ancestry.
`pwsh -File TAS/pico/test_live.ps1` separately checks real Windows LEFT delivery,
timeout/explicit release, 100 ACKs and reset/reopen. It sends real keys: close other
controllers and keep the interactive desktop available (no pending UAC prompt).

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
- `0xFC`: hold Enter exclusively for menu confirmation. The normal release and
  500 ms timeout apply; this does not add Enter to the recording's input mask.
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
Partial acknowledgement writes clear their unsent tail rather than poisoning the
next response. A HID fault discards the remainder of its input batch and queued
bytes before accepting new steering.

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
