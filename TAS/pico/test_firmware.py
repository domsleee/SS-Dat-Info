"""Fault-injection tests of the production controller; no board or HID writes."""
import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location("firmware", Path(__file__).with_name("code.py"))
firmware = importlib.util.module_from_spec(spec)
spec.loader.exec_module(firmware)


class Device:
    def __init__(self):
        self.held = set()
        self.events = []
        self.failures = 0

    def send(self, event):
        if self.failures:
            self.failures -= 1
            raise OSError("HID unavailable")
        self.events.append(event)

    def press(self, key):
        self.send(("down", key))
        self.held.add(key)

    def release(self, key):
        self.send(("up", key))
        self.held.discard(key)

    def release_all(self):
        self.send(("neutral",))
        self.held.clear()


class Serial:
    def __init__(self):
        self.data = b""
        self.out_waiting = 0
        self.replies = []
        self.fail_read = False
        self.fail_write = False
        self.short_write = False
        self.output_resets = 0

    def reset_input_buffer(self):
        self.data = b""

    def reset_output_buffer(self):
        self.output_resets += 1

    @property
    def in_waiting(self):
        return len(self.data)

    def read(self, count):
        if self.fail_read:
            raise OSError("CDC unavailable")
        result, self.data = self.data[:count], self.data[count:]
        return result

    def write(self, data):
        if self.fail_write:
            raise OSError("CDC reply unavailable")
        self.replies.append(data)
        return 1 if self.short_write else len(data)


class FirmwareTests(unittest.TestCase):
    def setUp(self):
        self.device = Device()
        self.serial = Serial()
        self.now = 0.0
        self.resets = 0
        self.controller = firmware.Controller(lambda: self.device, self.serial,
                                              range(8), self.reset, lambda: self.now)

    def reset(self):
        self.resets += 1

    def step(self, data=b"", now=None):
        self.serial.data += data
        if now is not None:
            self.now = now
        self.controller.step()

    def test_startup_sends_neutral(self):
        self.assertEqual(self.device.events, [("neutral",)])

    def test_batched_edges_in_order(self):
        self.step(bytes([1, 0, 2, 0]))
        self.assertEqual(self.device.events[1:], [("down", 0), ("up", 0), ("down", 1), ("up", 1)])

    def test_all_eight_keys_and_duplicate_mask(self):
        self.step(bytes([0xF0, 0xF0, 0x0F, 0]))
        self.assertFalse(self.device.held)
        self.assertEqual(len(self.device.events), 17)

    def test_timeout_and_keepalive(self):
        self.step(b"\x01")
        self.step(b"\x01", now=0.4)
        self.step(now=0.6)
        self.assertEqual(self.device.held, {0})
        self.step(now=0.901)
        self.assertFalse(self.device.held)

    def test_failed_press_is_not_committed(self):
        self.device.failures = 1
        self.step(b"\x01")
        self.assertEqual(self.controller.mask, 0)
        self.step(b"\x01", now=0.1)
        self.assertEqual(self.device.held, {0})

    def test_release_failure_retries_without_more_commands(self):
        self.step(b"\x01")
        self.device.failures = 2
        self.step(now=0.501)
        self.assertTrue(self.controller.release_pending)
        self.step(now=0.56)
        self.step(now=0.62)
        self.assertFalse(self.device.held)
        self.assertFalse(self.controller.release_pending)

    def test_failed_factory_recovers_and_drops_uncertain_input(self):
        def fail():
            raise OSError("keyboard missing")
        self.controller.factory = fail
        self.controller.fault()
        self.step(b"\x01", now=0.1)
        self.assertTrue(self.controller.release_pending)
        self.controller.factory = lambda: self.device
        self.step(now=0.2)
        self.assertFalse(self.device.held)
        self.step(b"\x01")
        self.assertEqual(self.device.held, {0})

    def test_cdc_error_releases_keys(self):
        self.step(b"\x01")
        self.serial.fail_read = True
        self.step(b"\x02")
        self.assertFalse(self.device.held)

    def test_ack_is_bounded_and_reports_release_failure(self):
        self.assertEqual(self.serial.timeout, 0)
        self.assertEqual(self.serial.write_timeout, 0)
        self.device.failures = 1
        self.step(b"\xff")
        self.assertEqual(self.serial.replies, [bytes([0x5A, 0, 1])])
        self.serial.out_waiting = 3
        self.step(b"\xff")
        self.assertEqual(len(self.serial.replies), 1)

    def test_broken_ack_does_not_swallow_following_mask(self):
        self.serial.fail_write = True
        self.step(b"\xff\x01")
        self.assertEqual(self.device.held, {0})

    def test_reset_drops_rest_of_batch(self):
        for command in (0xFD, 0xFE):
            self.step(bytes([1, command, 2]))
            self.assertFalse(self.device.held)
        self.assertEqual(self.resets, 2)

    def test_serial_work_is_bounded_per_pass(self):
        self.step(bytes([1, 0]) * 100)
        self.assertEqual(len(self.serial.data), 136)

    def test_failed_press_discards_same_batch_and_queued_input(self):
        self.device.failures = 1
        self.step(bytes([1, 2]) * 100)
        self.assertFalse(self.device.held)
        self.assertEqual(self.serial.data, b"")
        self.step(b"\x02", now=0.1)
        self.assertEqual(self.device.held, {1})

    def test_partial_ack_is_not_left_queued(self):
        self.serial.short_write = True
        self.step(b"\xff\x01")
        self.assertEqual(self.serial.output_resets, 1)
        self.assertEqual(self.device.held, {0})


if __name__ == "__main__":
    unittest.main()
