# boot.py — runs before code.py, configures USB devices
# Enables a second CDC serial port (data channel) that is binary-safe.
# The REPL console (first CDC) interprets Ctrl-C as interrupt, so we
# need the data port for the TAS binary bitmask protocol.

import usb_cdc

usb_cdc.enable(console=True, data=True)
