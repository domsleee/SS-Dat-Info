# code.py — Pico 2 TAS HID keyboard spoofer
# Listens on CDC data serial for single-byte bitmask commands.
# Each byte = keys to hold (same format as TAS input log).
# Translates to real USB HID keyboard press/release events.

import usb_hid
import usb_cdc
import time
import microcontroller
import memorymap
from adafruit_hid.keyboard import Keyboard
from adafruit_hid.keycode import Keycode

# RP2350 USB device register access for D+ pullup control.
# Clearing the PULLUP_EN bit in SIE_CTRL is electrically identical to
# unplugging the USB cable — the host sees a real disconnect event.
# SIE_CTRL is at USBCTRL_REGS_BASE (0x50110000) + 0x4C, PULLUP_EN = bit 16.
_USB_REGS = memorymap.AddressRange(start=0x50110000, length=0x100)
_SIE_CTRL_OFFSET = 0x4C
_PULLUP_EN_BIT = 16

def _usb_disconnect_reconnect():
    """Pull D+ low (disconnect), wait, pull D+ high (reconnect).
    Host sees a real unplug/replug cycle and fully re-enumerates."""
    # Read current SIE_CTRL value (4 bytes, little-endian)
    raw = _USB_REGS[_SIE_CTRL_OFFSET:_SIE_CTRL_OFFSET + 4]
    val = int.from_bytes(raw, "little")
    # Clear PULLUP_EN — host sees disconnect
    val_off = val & ~(1 << _PULLUP_EN_BIT)
    _USB_REGS[_SIE_CTRL_OFFSET:_SIE_CTRL_OFFSET + 4] = val_off.to_bytes(4, "little")
    time.sleep(0.3)  # let host process disconnect
    # Set PULLUP_EN — host sees new device, re-enumerates
    val_on = val | (1 << _PULLUP_EN_BIT)
    _USB_REGS[_SIE_CTRL_OFFSET:_SIE_CTRL_OFFSET + 4] = val_on.to_bytes(4, "little")

kbd = Keyboard(usb_hid.devices)
# Use DATA port only (binary-safe). Never consume commands from console CDC:
# bytes like 0x03 can be interpreted as Ctrl-C on console and kill code.py.
serial = usb_cdc.data

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

current_mask = 0
last_rx_time = time.monotonic()
TIMEOUT_S = 0.5  # release all if no command in 500ms

while True:
    try:
        if serial and serial.in_waiting > 0:
            data = serial.read(serial.in_waiting)
            if not data:
                continue

            # only process the last byte (most recent command)
            cmd = data[-1]
            last_rx_time = time.monotonic()

            if cmd == 0xFD:
                # Soft USB reconnect: toggle D+ pullup to simulate unplug/replug.
                # Recovers dead HID without full MCU reset. COM port reopens on
                # same port number. Takes ~2s total (disconnect + re-enumeration).
                kbd.release_all()
                current_mask = 0
                _usb_disconnect_reconnect()
                # After reconnect, TinyUSB re-enumerates. Reinit keyboard.
                time.sleep(2)  # let host finish enumeration
                kbd = Keyboard(usb_hid.devices)
                serial = usb_cdc.data
                continue

            if cmd == 0xFE:
                # Hard reset: D+ pullup disconnect then full MCU reset.
                # More aggressive than 0xFD — resets all CircuitPython state.
                kbd.release_all()
                _usb_disconnect_reconnect()
                time.sleep(0.2)
                microcontroller.reset()
                # never reached

            if cmd == 0xFF:
                kbd.release_all()
                current_mask = 0
                continue

            changed = current_mask ^ cmd
            for i in range(8):
                bit = 1 << i
                if changed & bit:
                    if cmd & bit:
                        kbd.press(BIT_TO_KEY[i])
                    else:
                        kbd.release(BIT_TO_KEY[i])
            current_mask = cmd
    except Exception:
        # Keep HID alive even if a transient serial/USB error occurs.
        kbd.release_all()
        current_mask = 0
        time.sleep(0.05)

    # safety timeout
    if current_mask != 0 and (time.monotonic() - last_rx_time) > TIMEOUT_S:
        kbd.release_all()
        current_mask = 0
