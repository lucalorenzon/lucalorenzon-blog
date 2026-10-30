"""Tests for the pure parts of record.py (parec is never started).

Run: python3 -m unittest discover -s scripts/screen-reader -p 'test_*.py'
"""

import array
import unittest

import record


def pcm(samples: list[int]) -> bytes:
    return array.array("h", samples).tobytes()


class BlockRmsTest(unittest.TestCase):
    def test_zeros_are_zero(self):
        self.assertEqual(record.block_rms(pcm([0] * 100)), 0.0)

    def test_empty_is_zero(self):
        self.assertEqual(record.block_rms(b""), 0.0)

    def test_constant_signal(self):
        self.assertAlmostEqual(record.block_rms(pcm([-300] * 50)), 300.0)

    def test_square_wave(self):
        self.assertAlmostEqual(record.block_rms(pcm([1000, -1000] * 20)), 1000.0)

    def test_odd_byte_count_ignores_last_byte(self):
        self.assertAlmostEqual(record.block_rms(pcm([500, 500]) + b"\x7f"), 500.0)

    def test_single_byte_is_zero(self):
        self.assertEqual(record.block_rms(b"\x01"), 0.0)


class IsSilentTest(unittest.TestCase):
    def test_zero_is_silent(self):
        self.assertTrue(record.is_silent(0.0))

    def test_speech_is_not_silent(self):
        self.assertFalse(record.is_silent(3000.0))

    def test_threshold_itself_is_not_silent(self):
        self.assertFalse(record.is_silent(record.SILENCE_THRESHOLD))


class EndDetectorTest(unittest.TestCase):
    def run_blocks(self, detector, verdicts, step=0.5):
        """Feeds (loud) verdicts one block apart; returns the first end."""
        for i, loud in enumerate(verdicts, start=1):
            reason = detector.feed(loud, i * step)
            if reason:
                return reason, i * step
        return None, None

    def test_no_end_before_any_sound(self):
        detector = record.EndDetector(quiet=10, maximum=90)
        self.assertEqual(self.run_blocks(detector, [False] * 100), (None, None))

    def test_max_ends_without_sound(self):
        detector = record.EndDetector(quiet=10, maximum=90)
        reason, _ = self.run_blocks(detector, [False] * 200)
        self.assertEqual(reason, record.REASON_MAX)

    def test_ends_after_sound_then_quiet(self):
        detector = record.EndDetector(quiet=10, maximum=90)
        reason, at = self.run_blocks(detector, [True] * 4 + [False] * 40)
        self.assertEqual(reason, record.REASON_OVER)
        self.assertEqual(at, 12.0)  # last loud at 2.0s, plus 10s

    def test_short_gap_does_not_end(self):
        detector = record.EndDetector(quiet=10, maximum=90)
        verdicts = [True] * 4 + [False] * 14 + [True] * 4  # 7s gap
        self.assertEqual(self.run_blocks(detector, verdicts), (None, None))
        self.assertTrue(detector.heard_sound)

    def test_max_ends_while_loud(self):
        detector = record.EndDetector(quiet=10, maximum=90)
        reason, at = self.run_blocks(detector, [True] * 400)
        self.assertEqual(reason, record.REASON_MAX)
        self.assertEqual(at, 90.0)


if __name__ == "__main__":
    unittest.main()
