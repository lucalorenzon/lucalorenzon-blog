#!/usr/bin/env python3
"""Records the container's null sink to a WAV file and stops when the reading
is over. Runs inside the container (started by entrypoint.sh), standard
library only.

Why: the sink's state never goes idle (speech-dispatcher keeps its stream
open), so the end of the reading is read from the recorded silence instead.
The WAV is also kept by check.py, so the reading can be listened to.

Layout: pure decision logic first (testable without parec), then the process.
"""

import argparse
import array
import math
import signal
import subprocess
import sys
import time
import wave

RATE = 22050
BLOCK_SECONDS = 0.5
BLOCK_BYTES = int(RATE * BLOCK_SECONDS) * 2  # 16-bit mono

# Loudness (RMS, scale 0..32767) below which a block counts as silence. The
# null sink gives exact zeros when nothing plays, and espeak-ng speech is
# thousands, so any low value separates them; 100 leaves room for dither or a
# click without ever mistaking speech for silence.
SILENCE_THRESHOLD = 100

REASON_OVER = "reading over"
REASON_MAX = "maximum reached"

# Orca can be silent for several seconds between two items (about seven while
# it looks for a braille display), so `quiet` must stay well above that gap.


def block_rms(data: bytes) -> float:
    """Root mean square of 16-bit little-endian samples; a trailing odd byte
    is ignored."""
    data = data[: len(data) // 2 * 2]
    if not data:
        return 0.0
    samples = array.array("h")
    samples.frombytes(data)
    if sys.byteorder == "big":
        samples.byteswap()
    return math.sqrt(sum(s * s for s in samples) / len(samples))


def is_silent(rms: float, threshold: float = SILENCE_THRESHOLD) -> bool:
    return rms < threshold


class EndDetector:
    """Decides when to stop. Time is passed in, never read here."""

    def __init__(self, quiet: float, maximum: float):
        self.quiet = quiet
        self.maximum = maximum
        self.heard_sound = False
        self._last_loud = 0.0

    def feed(self, loud: bool, elapsed: float) -> str | None:
        """One block's verdict at `elapsed` seconds since start: None to go
        on, otherwise the reason to stop."""
        if loud:
            self.heard_sound = True
            self._last_loud = elapsed
        if elapsed >= self.maximum:
            return REASON_MAX
        if self.heard_sound and elapsed - self._last_loud >= self.quiet:
            return REASON_OVER
        return None


# ---------------------------------------------------------------------------
# Process and files
# ---------------------------------------------------------------------------

def step(message: str) -> None:
    print(f"[{time.strftime('%H:%M:%S')}] {message}", flush=True)


def read_block(stream) -> bytes:
    """Up to BLOCK_BYTES from the stream; short only at its end."""
    data = b""
    while len(data) < BLOCK_BYTES:
        chunk = stream.read(BLOCK_BYTES - len(data))
        if not chunk:
            break
        data += chunk
    return data


def record(device: str, quiet: float, maximum: float, out: str) -> int:
    parec = subprocess.Popen(
        ["parec", f"--device={device}", "--format=s16le", f"--rate={RATE}",
         "--channels=1", "--raw"],
        stdout=subprocess.PIPE)
    # SIGTERM becomes a normal exit, so the finally block closes the WAV.
    signal.signal(signal.SIGTERM, lambda *_: sys.exit(0))
    detector = EndDetector(quiet, maximum)
    start = time.monotonic()
    reason = "interrupted"
    samples = 0
    wav = wave.open(out, "wb")
    try:
        wav.setnchannels(1)
        wav.setsampwidth(2)
        wav.setframerate(RATE)
        step(f"recording started: {out}")
        while True:
            data = read_block(parec.stdout)
            if not data:
                reason = "parec exited"
                break
            wav.writeframes(data[: len(data) // 2 * 2])
            samples += len(data) // 2
            loud = not is_silent(block_rms(data))
            was_heard = detector.heard_sound
            result = detector.feed(loud, time.monotonic() - start)
            if detector.heard_sound and not was_heard:
                step("sound heard")
            if result:
                reason = result
                break
    finally:
        wav.close()
        parec.terminate()
        try:
            parec.wait(timeout=5)
        except subprocess.TimeoutExpired:
            parec.kill()
        step(f"recording ended: {reason}")
        step(f"recorded {samples / RATE:.1f}s")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--device", default="null_out.monitor")
    parser.add_argument("--quiet", type=float, default=10,
                        help="seconds of silence after sound that end the reading")
    parser.add_argument("--max", type=float, default=90, dest="maximum",
                        help="seconds after which recording stops anyway")
    parser.add_argument("--out", default="/out/reading.wav")
    args = parser.parse_args()
    return record(args.device, args.quiet, args.maximum, args.out)


if __name__ == "__main__":
    sys.exit(main())
