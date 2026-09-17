#!/usr/bin/env python3
# /// script
# requires-python = ">=3.8"
# dependencies = ["pyserial"]
# ///
"""
test.py — Standalone Pico HID keyboard test.
Run on the PC (not the Pico) to verify the Pico is working.

Usage:
  uv run TAS/pico/test.py COM7

Opens the Pico data serial port, sends key commands, and you should
see the corresponding keystrokes appear in whatever window has focus.

Give yourself 3 seconds to click into Notepad or similar after starting.
"""

import sys
import time
import serial

LEFT  = 0x01
RIGHT = 0x02
UP    = 0x04
DOWN  = 0x08
CTRL  = 0x10
SHIFT = 0x20
RELEASE_ALL = 0xFF

def main():
    port = sys.argv[1] if len(sys.argv) > 1 else "COM7"
    print(f"Opening {port}...")
    ser = serial.Serial(port, baudrate=115200, timeout=1)
    time.sleep(0.5)

    print("You have 3 seconds to click into Notepad or a text field...")
    for i in range(3, 0, -1):
        print(f"  {i}...")
        time.sleep(1)

    print("\nSending arrow key sequence:")

    tests = [
        ("LEFT",  LEFT),
        ("RIGHT", RIGHT),
        ("UP",    UP),
        ("DOWN",  DOWN),
        ("CTRL",  CTRL),
        ("SHIFT", SHIFT),
        ("LEFT+UP (diagonal)", LEFT | UP),
    ]

    for name, mask in tests:
        print(f"  Press {name} (mask=0x{mask:02X})")
        ser.write(bytes([mask]))
        time.sleep(0.3)
        ser.write(bytes([0x00]))  # release
        time.sleep(0.2)

    ser.write(bytes([RELEASE_ALL]))
    print("\nDone! Release-all sent.")
    print("If you saw arrow key movement / cursor movement in Notepad, it works!")
    ser.close()

if __name__ == "__main__":
    main()
