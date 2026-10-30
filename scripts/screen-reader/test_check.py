"""Tests for the pure functions of check.py (no process is started).

Run: python3 -m unittest discover -s scripts/screen-reader -p 'test_*.py'

The fixture is trimmed from the debug file of a real trial run of Orca 51.1.
"""

import unittest
from pathlib import Path

import check

FIXTURE = Path(__file__).parent / "fixtures" / "orca51-page-read.log"

EXPECTED = [
    ("system", "Page has 3 landmarks, 1 heading."),
    ("system", "banner"),
    ("system", "code start"),
    ("system", "code end"),
    ("it", "Home Page"),
    ("system", "heading 1"),
    ("system", "leaving banner."),
    ("system", "main content"),
    ("it", "Sito in costruzione"),
    ("system", "leaving main content."),
    ("system", "information"),
    ("it", "© Luca Lorenzon 2026"),
]


def utterance(text: str, orca_line: str = "") -> str:
    return (f"07:00:00.000000 - SPEECH DISPATCHER: Speaking "
            f"'<speak><mark name=\"0:3\"/>{text}</speak>' \n{orca_line}")


def orca_line(language: str) -> str:
    return (f"                  ORCA rate 50, pitch 5.0, volume 10.0, "
            f"language{language}, punctuation: MOST\n")


SAY_ALL = "07:00:00.000000 - SAY ALL PRESENTER: Speaking (contents): [x]'y' (0 - 1)\n"


class ExtractTranscript(unittest.TestCase):
    def setUp(self):
        self.log = FIXTURE.read_text(encoding="utf-8")

    def test_page_reading_is_extracted_exactly(self):
        self.assertEqual(check.extract_transcript(self.log), EXPECTED)

    def test_noise_is_excluded(self):
        texts = [text for _, text in check.extract_transcript(self.log)]
        for noise in ("Screen reader on.", "Screen reader off.", "Browse mode",
                      "Luca Lorenzon - Home Page", "document web"):
            self.assertNotIn(noise, texts)

    def test_noise_is_in_the_fixture(self):
        # Guards the test above: the noise must really be there to be excluded.
        texts = [text for _, text, _ in check.parse_utterances(self.log)]
        for noise in ("Screen reader on.", "Screen reader off.", "Browse mode"):
            self.assertIn(noise, texts)

    def test_ssml_is_stripped(self):
        for _, text in check.extract_transcript(self.log):
            self.assertNotIn("<", text)
            self.assertNotIn("name=", text)

    def test_no_reading_found(self):
        self.assertEqual(check.extract_transcript(""), [])
        self.assertEqual(check.extract_transcript(utterance("hello")), [])

    def test_summary_alone_is_not_a_reading(self):
        log = utterance("Page has 2 landmarks.") + utterance("Screen reader off.")
        self.assertEqual(check.extract_transcript(log), [])

    def test_reading_stops_at_first_non_say_all_utterance(self):
        log = (utterance("Page has 1 landmark.") + SAY_ALL + utterance("one")
               + utterance("stray") + SAY_ALL + utterance("two"))
        self.assertEqual([text for _, text in check.extract_transcript(log)],
                         ["Page has 1 landmark.", "one"])


class Language(unittest.TestCase):
    def test_language_asked_of_the_voice(self):
        log = utterance("ciao", orca_line(" it"))
        self.assertEqual(check.parse_utterances(log), [("it", "ciao", False)])

    def test_empty_language_is_the_default_voice(self):
        log = utterance("hello", orca_line(""))
        self.assertEqual(check.parse_utterances(log),
                         [(check.SYSTEM_LANGUAGE, "hello", False)])

    def test_sd_line_is_not_used(self):
        sd = "                  SD rate 0, pitch 0, volume 100, language it\n"
        log = utterance("hello", orca_line("") + sd)
        self.assertEqual(check.parse_utterances(log)[0][0], check.SYSTEM_LANGUAGE)

    def test_missing_language_line_means_system(self):
        self.assertEqual(check.parse_utterances(utterance("one")),
                         [(check.SYSTEM_LANGUAGE, "one", False)])


class StripSsml(unittest.TestCase):
    def test_tags_and_entities(self):
        ssml = '<speak><mark name="0:1"/>A &amp; <mark name="4:5"/>B</speak>'
        self.assertEqual(check.strip_ssml(ssml), "A & B")


class Transcripts(unittest.TestCase):
    def test_format_is_language_tab_text(self):
        self.assertEqual(check.format_transcript([("it", "Ciao"), ("system", "banner")]),
                         "it\tCiao\nsystem\tbanner\n")

    def test_diff_is_empty_when_equal(self):
        self.assertEqual(check.diff_transcripts("a\nb\n", "a\nb\n"), "")

    def test_diff_shows_the_change(self):
        diff = check.diff_transcripts("it\tHome Page\n", "en\tHome Page\n")
        self.assertIn("-it\tHome Page", diff)
        self.assertIn("+en\tHome Page", diff)


class Versions(unittest.TestCase):
    def test_parse(self):
        text = "Orca\t51.1-1~bpo13+1\nFirefox\t140.0esr-1\n\nbroken line\n"
        self.assertEqual(check.parse_versions(text),
                         [("Orca", "51.1-1~bpo13+1"), ("Firefox", "140.0esr-1")])

    def test_empty(self):
        self.assertEqual(check.parse_versions(""), [])


class DockerCommand(unittest.TestCase):
    def test_plain_docker_when_the_daemon_answers(self):
        self.assertEqual(check.docker_command(True, True, True), ["docker"])

    def test_sudo_when_it_does_not(self):
        self.assertEqual(check.docker_command(True, False, True), ["sudo", "docker"])

    def test_nothing_without_sudo(self):
        self.assertIsNone(check.docker_command(True, False, False))

    def test_nothing_without_docker(self):
        self.assertIsNone(check.docker_command(False, True, True))


class FormatRecord(unittest.TestCase):
    def test_clean_tree(self):
        text = check.format_record("2026-10-30 09:00", "abc1234", False,
                                   [("Orca", "51.1")], check.RESULT_SAME)
        self.assertEqual(text, "## 2026-10-30 09:00\n\n"
                               "- **Commit:** `abc1234`\n"
                               "- **Where:** container\n"
                               "- **Orca:** 51.1\n"
                               "- **Transcript:** same as the reference\n")

    def test_dirty_tree_is_said(self):
        text = check.format_record("t", "abc1234", True, [], check.RESULT_REGRESSION)
        self.assertIn("`abc1234` with uncommitted changes", text)
        self.assertIn("regression", text)


if __name__ == "__main__":
    unittest.main()
