"""Tests for the pure functions of screen_reader_check.py (no process is started).

Run: python3 -m unittest discover -s scripts -p 'test_*.py'
"""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

import screen_reader_check as check

FIXTURE = Path(__file__).parent / "fixtures" / "orca-page-read.log"

EXPECTED = [
    ("en", "Finished loading Luca Lorenzon - Home Page."),
    ("en", "Page has 3 landmarks, 1 heading."),
    ("en", "banner"),
    ("it", "Lvk @ 73r"),
    ("en", "image"),
    ("it", "Home Page"),
    ("en", "heading 1"),
    ("en", "leaving banner."),
    ("en", "main content"),
    ("it", "Sito in costruzione"),
    ("en", "leaving main content."),
    ("en", "information"),
    ("it", "© Luca Lorenzon 2026"),
]


class ExtractTranscript(unittest.TestCase):
    def setUp(self):
        self.log = FIXTURE.read_text(encoding="utf-8")

    def test_page_reading_is_extracted_exactly(self):
        self.assertEqual(check.extract_transcript(self.log), EXPECTED)

    def test_noise_is_excluded(self):
        texts = [text for _, text in check.extract_transcript(self.log)]
        for noise in ("Mozilla Firefox", "left alt", "space", "return",
                      "Translate", "Browse mode", "page tab."):
            self.assertNotIn(noise, texts)

    def test_noise_is_in_the_fixture(self):
        # Guards the test above: the noise must really be there to be excluded.
        texts = [text for _, text, _ in check.parse_utterances(self.log)]
        for noise in ("Mozilla Firefox", "left alt", "return", "Translate"):
            self.assertIn(noise, texts)

    def test_ssml_is_stripped(self):
        for _, text in check.extract_transcript(self.log):
            self.assertNotIn("<", text)
            self.assertNotIn("name=", text)

    def test_language_is_the_one_of_the_voice(self):
        languages = {text: lang for lang, text in check.extract_transcript(self.log)}
        self.assertEqual(languages["Home Page"], "it")
        self.assertEqual(languages["banner"], "en")

    def test_missing_language_line_means_system(self):
        log = ("07:00:00.000000 - SPEECH DISPATCHER: Speaking "
               "'<speak><mark name=\"0:3\"/>one</speak>' \n")
        self.assertEqual(check.parse_utterances(log),
                         [(check.SYSTEM_LANGUAGE, "one", False)])

    def test_no_reading_found(self):
        self.assertEqual(check.extract_transcript(""), [])
        before_load = "\n".join(self.log.splitlines()[:12])
        self.assertEqual(check.extract_transcript(before_load), [])

    def test_load_message_alone_is_not_a_reading(self):
        lines = self.log.splitlines()
        # Keep only up to the "Finished loading" utterance (3 lines each).
        index = next(i for i, line in enumerate(lines) if "Finished " in line)
        self.assertEqual(check.extract_transcript("\n".join(lines[:index + 3])), [])


class StripSsml(unittest.TestCase):
    def test_tags_and_entities(self):
        self.assertEqual(
            check.strip_ssml('<speak><mark name="0:3"/>A &amp; <mark name="4:5"/>B</speak>'),
            "A & B")


class CompareTranscripts(unittest.TestCase):
    def test_equal_gives_no_diff(self):
        text = check.format_transcript(EXPECTED)
        self.assertEqual(check.diff_transcripts(text, text), "")

    def test_different_shows_the_changed_line(self):
        reference = check.format_transcript(EXPECTED)
        changed = reference.replace("it\tHome Page", "en\tHome Page")
        diff = check.diff_transcripts(reference, changed)
        self.assertIn("-it\tHome Page", diff)
        self.assertIn("+en\tHome Page", diff)

    def test_transcript_format_is_language_tab_text(self):
        self.assertEqual(check.format_transcript([("it", "ciao"), ("en", "hi")]),
                         "it\tciao\nen\thi\n")


class FormatRecord(unittest.TestCase):
    ANSWERS = (("Is it fine?", "yes", ""), ("Is it good?", "no", "a bit harsh"),
               ("Anything else odd?", "", "nothing"))

    def test_section(self):
        section = check.format_record(
            "2026-10-30 14:05", "abc1234", False, "50.3", "Mozilla Firefox 158.0",
            check.RESULT_SAME, self.ANSWERS)
        self.assertEqual(section, (
            "## 2026-10-30 14:05\n\n"
            "- **Commit:** `abc1234`\n"
            "- **Orca:** 50.3\n"
            "- **Firefox:** Mozilla Firefox 158.0\n"
            "- **Transcript:** same as the reference\n"
            "- **Answers:**\n\n"
            "1. Is it fine? **yes**\n"
            "2. Is it good? **no — a bit harsh**\n"
            "3. Anything else odd? **nothing**\n"))

    def test_dirty_tree_is_stated(self):
        section = check.format_record("t", "abc1234", True, "50.3", "ff",
                                      check.RESULT_REGRESSION, self.ANSWERS)
        self.assertIn("`abc1234` with uncommitted changes", section)

    def test_failed_run_has_no_answers(self):
        section = check.format_record("t", "abc1234", False, "50.3", "ff",
                                      check.RESULT_NO_READING, [])
        self.assertIn("no page reading found", section)
        self.assertIn("not asked", section)


UBUNTU_LAUNCHER = """#!/bin/sh
if ! [ -x /snap/bin/firefox ]; then
    echo "Command '$0' requires the firefox snap to be installed." >&2
    exit 1
fi
exec /snap/bin/firefox "$@"
"""


class FirefoxIsSnap(unittest.TestCase):
    def setUp(self):
        self._dir = tempfile.TemporaryDirectory()
        self.addCleanup(self._dir.cleanup)
        self.dir = Path(self._dir.name)

    def write(self, name, content):
        path = self.dir / name
        path.write_bytes(content if isinstance(content, bytes) else content.encode())
        return str(path)

    def test_ubuntu_launcher_script_is_snap(self):
        self.assertTrue(check.firefox_is_snap(self.write("firefox", UBUNTU_LAUNCHER)))

    def test_path_under_snap_is_snap(self):
        self.assertTrue(check.firefox_is_snap("/snap/firefox/current/firefox"))

    def test_symlink_into_snap_is_snap(self):
        link = self.dir / "firefox"
        link.symlink_to("/snap/bin/firefox")
        self.assertTrue(check.firefox_is_snap(str(link)))

    def test_binary_is_not_snap(self):
        self.assertFalse(check.firefox_is_snap(self.write("firefox", b"\x7fELF\x00\x01/snap/bin/firefox")))

    def test_script_without_snap_is_not_snap(self):
        self.assertFalse(check.firefox_is_snap(
            self.write("firefox", "#!/bin/sh\nexec /opt/firefox/firefox \"$@\"\n")))

    def test_not_found_is_not_snap(self):
        self.assertFalse(check.firefox_is_snap(""))
        self.assertFalse(check.firefox_is_snap(str(self.dir / "missing")))


class FirstWritesCreateTheFolder(unittest.TestCase):
    """docs/accessibility/ does not exist yet on a first run."""

    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        folder = Path(tmp.name) / "docs" / "accessibility"
        self.reference = folder / "screen-reader-transcript.txt"
        self.record = folder / "screen-reader-checks.md"
        for name, value in (("ACCESSIBILITY_DIR", folder),
                            ("REFERENCE_PATH", self.reference),
                            ("RECORD_PATH", self.record)):
            patcher = mock.patch.object(check, name, value)
            patcher.start()
            self.addCleanup(patcher.stop)

    def test_first_reference_save_creates_the_folder(self):
        with mock.patch("builtins.input", return_value="y"):
            result = check.compare_with_reference("en: hello\n")
        self.assertEqual(result, check.RESULT_FIRST)
        self.assertEqual(self.reference.read_text(encoding="utf-8"), "en: hello\n")

    def test_first_record_creates_the_folder(self):
        check.append_record("## run\n")
        self.assertIn("## run", self.record.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
