# code.py — Pico 2 TAS HID keyboard spoofer  [PROPOSED — NOT FLASHED]
#
# Reviewed rewrite of the shipped firmware (see code.py for the running version
# and README.md for the review that produced this). Every change below is one the
# review confirmed; the ideas it called wrong or cargo cult are deliberately NOT
# here — there is no loop sleep, and no watchdog.
#
# WHAT IS DELIBERATELY UNCHANGED:
#   * The 500ms release-all timeout. It is load-bearing for the test suite — that
#     release is currently the only thing putting a key UP inside the recording
#     window. Changing it broke `regression` once already (see README).
#   * The bitmask protocol and bit assignments.
#   * usb_cdc.data as the only command channel (0x03 on the console is Ctrl-C).
#
# WHAT CHANGED, and why:
#   1. Every received byte is processed, in order. `data[-1]` discarded the rest of
#      the batch, so a press and release arriving together erased the press
#      entirely. Real input-correctness bug.
#   2. Nothing can kill the program any more. Releases go through _safe_release(),
#      and the timeout path — which was OUTSIDE the try — is now covered too. A
#      raise inside the old handler ended code.py and left the device enumerated
#      but deaf: port opens, writes "succeed", nothing happens.
#   3. release_all() at startup, so a soft reload cannot leave a key held that the
#      timeout will never clear (after a reload current_mask is 0, so the timeout
#      believes there is nothing to release).
#   4. 0xFF is ACKed, turning the host's existing write-only health check into a
#      real liveness probe. No new command byte was invented: all 256 values are
#      potentially valid masks, and the protocol has already spent 0xFD/0xFE/0xFF.
#   5. 0xFE's detach -> 200ms -> reset sequence is gone. It reset the MCU midway
#      through the host's re-enumeration. It was also dead code — nothing in the
#      repo ever sent it — so it now resets while still detached.
#   6. The ACK reports why the firmware last started (power-on, soft reload,
#      watchdog, brownout), so a mystery restart is visible to the host.

import usb_hid
import usb_cdc
import time
import microcontroller
import supervisor
import memorymap
from adafruit_hid.keyboard import Keyboard
from adafruit_hid.keycode import Keycode

# A soft auto-reload does NOT drop USB — it is not the cause of the vanishing COM
# port, and the README says so. It is disabled anyway for a smaller, real reason:
# an unsolicited reload mid-test silently discards current_mask and any buffered
# commands, which is a bad thing to have happen in the middle of a recording.
try:
    supervisor.runtime.autoreload = False
except Exception:
    pass

# RP2350 USB device register access for D+ pullup control.
# Clearing PULLUP_EN in SIE_CTRL makes the host see a disconnect. Note this is NOT
# electrically identical to unplugging — VBUS stays powered — and poking the
# register behind TinyUSB's back is unsupported and can race its own SIE writes.
# It is kept only because the manual "soft reconnect" button in tas_ui uses it.
_USB_REGS = memorymap.AddressRange(start=0x50110000, length=0x100)
_SIE_CTRL_OFFSET = 0x4C
_PULLUP_EN_BIT = 16


def _usb_set_pullup(enabled):
    raw = _USB_REGS[_SIE_CTRL_OFFSET:_SIE_CTRL_OFFSET + 4]
    val = int.from_bytes(raw, "little")
    if enabled:
        val |= 1 << _PULLUP_EN_BIT
    else:
        val &= ~(1 << _PULLUP_EN_BIT)
    _USB_REGS[_SIE_CTRL_OFFSET:_SIE_CTRL_OFFSET + 4] = val.to_bytes(4, "little")


def _usb_disconnect_reconnect():
    """Host sees an unplug/replug cycle and re-enumerates."""
    _usb_set_pullup(False)
    time.sleep(0.3)
    _usb_set_pullup(True)


kbd = None
serial = usb_cdc.data


def _make_keyboard():
    global kbd
    try:
        kbd = Keyboard(usb_hid.devices)
        return True
    except Exception:
        kbd = None
        return False


def _safe_release():
    """Release everything without ever raising.

    The old code called kbd.release_all() bare, in the exception handler AND in
    the timeout check. If HID was what broke, that raised a second time from a
    place nothing was guarding, ended code.py, and left the device enumerated but
    deaf — the worst shape, because the port still opens and writes still appear
    to succeed.
    """
    try:
        if kbd is not None:
            kbd.release_all()
            return True
    except Exception:
        pass
    return False


# TAS input log bitmask -> HID keycode
BIT_TO_KEY = [
    Keycode.LEFT_ARROW,    # bit 0 = LEFT
    Keycode.RIGHT_ARROW,   # bit 1 = RIGHT
    Keycode.UP_ARROW,      # bit 2 = UP
    Keycode.DOWN_ARROW,    # bit 3 = DOWN
    Keycode.LEFT_CONTROL,  # bit 4 = JUMP (CTRL)
    Keycode.LEFT_SHIFT,    # bit 5 = SHIFT
    Keycode.F5,            # bit 6 = F5 (restart race)
    Keycode.ESCAPE,        # bit 7 = Escape (pause/resume in-race)
]

CMD_SOFT_RECONNECT = 0xFD
CMD_HARD_RESET = 0xFE
CMD_RELEASE_ALL = 0xFF

ACK_MAGIC = 0x5A          # reply to 0xFF: [0x5A, current_mask, start_reason]
TIMEOUT_S = 0.5           # release all if no command in 500ms — LOAD-BEARING


def _start_reason():
    """One byte describing why the firmware is running, for the ACK.

    Lets the host tell a clean power-on from a soft reload or a brownout instead
    of guessing after the fact.
    """
    try:
        if supervisor.runtime.run_reason == supervisor.RunReason.AUTO_RELOAD:
            return 1
    except Exception:
        pass
    try:
        rr = microcontroller.cpu.reset_reason
        return {
            microcontroller.ResetReason.POWER_ON: 2,
            microcontroller.ResetReason.BROWNOUT: 3,
            microcontroller.ResetReason.SOFTWARE: 4,
            microcontroller.ResetReason.WATCHDOG: 5,
        }.get(rr, 0)
    except Exception:
        return 0


_make_keyboard()
# Startup release: a reload leaves current_mask at 0, so the timeout below would
# never clear a key that was physically down when the reload happened.
_safe_release()
START_REASON = _start_reason()

current_mask = 0
last_rx_time = time.monotonic()

while True:
    try:
        if serial is not None and serial.in_waiting > 0:
            data = serial.read(serial.in_waiting)
            if data:
                last_rx_time = time.monotonic()

                # EVERY byte, in order. Taking only data[-1] dropped a press whose
                # release arrived in the same batch, silently losing the input.
                for cmd in data:
                    if cmd == CMD_SOFT_RECONNECT:
                        _safe_release()
                        current_mask = 0
                        _usb_disconnect_reconnect()
                        time.sleep(2)  # let the host finish enumeration
                        _make_keyboard()
                        serial = usb_cdc.data
                        break

                    if cmd == CMD_HARD_RESET:
                        # Reset while STILL DETACHED. The old sequence reattached,
                        # gave Windows 200ms, then reset the MCU underneath it.
                        _safe_release()
                        _usb_set_pullup(False)
                        time.sleep(0.1)
                        microcontroller.reset()
                        # never reached

                    if cmd == CMD_RELEASE_ALL:
                        ok = _safe_release()
                        current_mask = 0
                        # ACK so the host's health check proves code.py is ALIVE,
                        # not merely that a write landed in a driver buffer.
                        try:
                            if serial is not None:
                                serial.write(bytes((ACK_MAGIC,
                                                    0x01 if ok else 0x00,
                                                    START_REASON)))
                        except Exception:
                            pass
                        continue

                    changed = current_mask ^ cmd
                    for i in range(8):
                        bit = 1 << i
                        if changed & bit:
                            try:
                                if cmd & bit:
                                    kbd.press(BIT_TO_KEY[i])
                                else:
                                    kbd.release(BIT_TO_KEY[i])
                            except Exception:
                                # Re-acquire once and carry on; never die here.
                                _make_keyboard()
                    current_mask = cmd
    except Exception:
        _safe_release()
        current_mask = 0
        serial = usb_cdc.data
        time.sleep(0.05)

    # Safety timeout — now outside-the-try-safe. DO NOT REMOVE OR LENGTHEN: the
    # test suite depends on this release landing inside the recording window.
    if current_mask != 0 and (time.monotonic() - last_rx_time) > TIMEOUT_S:
        _safe_release()
        current_mask = 0
