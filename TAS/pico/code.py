# Pico 2 TAS keyboard. Masks and the 500ms safety timeout are unchanged.
# Importable on the host so tests execute the same controller as the board.
import time

TIMEOUT_S = 0.5
RETRY_S = 0.05
ACK_VERSION = 1
ENTER_COMMAND = 0xFC


class Controller:
    def __init__(self, keyboard_factory, serial, keys, reset, clock=time.monotonic):
        self.factory = keyboard_factory
        self.serial = serial
        self.keys = keys
        self.reset = reset
        self.clock = clock
        self.keyboard = None
        self.mask = 0
        # A soft reload can inherit keys held by the previous interpreter.
        self.release_pending = True
        self.retry_at = 0
        self.last_rx = clock()
        self.serial.timeout = 0
        self.serial.write_timeout = 0
        self.release()

    def fault(self):
        # Adafruit's report buffer may have changed before send_report failed.
        # Rebuild it and send an unconditional neutral report before more input.
        self.keyboard = None
        self.release_pending = True
        self.retry_at = self.clock() + RETRY_S

    def release(self):
        self.release_pending = True
        try:
            if self.keyboard is None:
                self.keyboard = self.factory()
            self.keyboard.release_all()
        except Exception:
            self.fault()
            return False
        self.mask = 0
        self.release_pending = False
        return True

    def acknowledge(self, ok):
        try:
            # Older hosts do not read replies. Never wait for them or build a queue.
            if self.serial.out_waiting == 0:
                if self.serial.write(bytes((0x5A, int(ok), ACK_VERSION))) != 3:
                    self.serial.reset_output_buffer()
        except Exception:
            pass  # Losing diagnostics must not prevent key release.

    def command(self, cmd):
        self.last_rx = self.clock()
        if cmd in (0xFD, 0xFE):
            self.release()
            # Use the supported reset API, not USB register writes behind TinyUSB.
            # Both recovery commands now require the host to reopen the port.
            self.reset()
            return False  # Never apply buffered commands from before the reset.
        if cmd == 0xFF:
            self.acknowledge(self.release())
            return True
        if self.release_pending:
            return True  # Discard stale input while HID state is uncertain.
        if cmd == ENTER_COMMAND:
            cmd = 1 << 8  # Menu confirmation, exclusive of the eight game keys.
        try:
            changed = self.mask ^ cmd
            for index, key in enumerate(self.keys):
                if changed & (1 << index):
                    if cmd & (1 << index):
                        self.keyboard.press(key)
                    else:
                        self.keyboard.release(key)
            self.mask = cmd  # Commit only after every report succeeds.
        except Exception:
            self.fault()
            self.release()
            # Discard the rest of this batch and anything queued during the fault.
            # Applying it after recovery could resurrect stale steering.
            self.serial.reset_input_buffer()
            return False
        return True

    def step(self):
        now = self.clock()
        if self.release_pending and now >= self.retry_at:
            self.release()
        if self.mask and now - self.last_rx > TIMEOUT_S:
            if not self.release_pending:
                self.release()
        try:
            count = self.serial.in_waiting
            if count:
                # Bound work per pass so traffic cannot starve safety/recovery.
                data = self.serial.read(min(count, 64))
                for cmd in data or b"":
                    if not self.command(cmd):
                        break
        except Exception:
            self.fault()
            self.release()


def main():
    import usb_cdc
    import usb_hid
    import microcontroller
    from adafruit_hid.keyboard import Keyboard
    from adafruit_hid.keycode import Keycode

    keys = (Keycode.LEFT_ARROW, Keycode.RIGHT_ARROW, Keycode.UP_ARROW,
            Keycode.DOWN_ARROW, Keycode.LEFT_CONTROL, Keycode.LEFT_SHIFT,
            Keycode.F5, Keycode.ESCAPE, Keycode.ENTER)
    controller = Controller(lambda: Keyboard(usb_hid.devices), usb_cdc.data,
                            keys, microcontroller.reset)
    while True:
        controller.step()


if __name__ == "__main__":
    main()
