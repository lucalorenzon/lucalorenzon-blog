"""Tests for robots_list.py, on a small robots.json and with no network.

Run: python3 -m unittest discover -s scripts/robots-list -p 'test_*.py'
"""

import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import robots_list

FIXTURE = {
    "__usage_ai-training": {"function": "AI Data Scrapers"},
    "Assistant": {"function": "AI Assistants"},
    "Searcher": {"function": "AI Search Crawlers"},
    "Agent": {"function": "AI Agents"},
    "Coder": {"function": "AI Coding Agents"},
    "Scraper": {"function": "AI Data Scrapers"},
    "Provider": {"function": "AI Data Providers"},
    "Undocumented": {"function": "Undocumented AI Agents"},
    "Trainer": {"function": "LLM training."},
    "Padded": {"function": "  AI Assistants \n"},
    "PerplexityBot": {"function": "Search for Perplexity's users."},
}
FIXTURE_BYTES = json.dumps(FIXTURE).encode("utf-8")
FIXTURE_SHA = robots_list.sha256(FIXTURE_BYTES)


class FilterTests(unittest.TestCase):
    def blocked(self) -> list[str]:
        return robots_list.blocked_names(robots_list.parse_functions(FIXTURE_BYTES))

    def test_four_categories_pass(self):
        for name in ("Assistant", "Searcher", "Agent", "Coder"):
            self.assertNotIn(name, self.blocked())

    def test_other_categories_and_free_text_are_blocked(self):
        for name in ("Scraper", "Provider", "Undocumented", "Trainer"):
            self.assertIn(name, self.blocked())

    def test_function_is_stripped(self):
        self.assertNotIn("Padded", self.blocked())

    def test_exception_passes(self):
        self.assertNotIn("PerplexityBot", self.blocked())

    def test_usage_token_is_not_in_the_list(self):
        self.assertNotIn(robots_list.USAGE_TOKEN, self.blocked())

    def test_output_is_sorted_one_per_line_with_trailing_newline(self):
        text = robots_list.render(self.blocked())
        self.assertEqual(text, "Provider\nScraper\nTrainer\nUndocumented\n")

    def test_wrong_shape_is_an_error(self):
        with self.assertRaises(robots_list.RobotsListError):
            robots_list.parse_functions(b'{"a": {"nofunction": 1}}')


class PinnedListTests(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.repo = Path(self._tmp.name)

    def commit_list(self, data: bytes = FIXTURE_BYTES) -> None:
        path = self.repo / robots_list.COMMITTED_LIST
        path.parent.mkdir(parents=True)
        path.write_bytes(data)

    def test_committed_list_is_read_and_verified(self):
        self.commit_list()
        self.assertEqual(robots_list.pinned_list(self.repo, FIXTURE_SHA), FIXTURE_BYTES)

    def test_missing_file_is_an_error(self):
        with self.assertRaisesRegex(robots_list.RobotsListError, "cannot read"):
            robots_list.pinned_list(self.repo, FIXTURE_SHA)

    def test_checksum_mismatch_is_an_error(self):
        self.commit_list()
        with self.assertRaisesRegex(robots_list.RobotsListError, "PINNED_SHA256"):
            robots_list.pinned_list(self.repo, "0" * 64)

    def test_committed_file_matches_the_pin(self):
        repo = Path(__file__).resolve().parents[2]
        self.assertTrue(robots_list.pinned_list(repo))

    def test_main_writes_the_list_without_the_network(self):
        # The fixture is not the pinned file, so the pin is patched to it.
        self.commit_list()
        output = self.repo / "out" / "blocked.txt"
        with (
            mock.patch.object(robots_list, "PINNED_SHA256", FIXTURE_SHA),
            mock.patch.object(
                robots_list, "http_get", side_effect=AssertionError("network used")
            ),
        ):
            code = robots_list.main(["--repo", str(self.repo), "--output", str(output)])
        self.assertEqual(code, 0)
        self.assertEqual(
            output.read_text(encoding="utf-8"),
            "Provider\nScraper\nTrainer\nUndocumented\n",
        )

    def test_main_writes_nothing_on_mismatch(self):
        self.commit_list()
        output = self.repo / "out" / "blocked.txt"
        code = robots_list.main(["--repo", str(self.repo), "--output", str(output)])
        self.assertEqual(code, 1)
        self.assertFalse(output.exists())


OLD = {
    "Stays": "AI Assistants",
    "WillBlock": "AI Assistants",
    "WillAllow": "AI Data Scrapers",
    "PerplexityBot": "Search.",
    "Claude-SearchBot": "Search.",
}

NEW = {
    "Stays": "AI Assistants",
    "WillBlock": "AI Data Scrapers",
    "WillAllow": "AI Agents",
    "PerplexityBot": "Search.",
    "NewOwn": "Reads the web, no more words.",
    "NewKnown": "AI Data Providers",
    "NewAllowed": "AI Agents",
    robots_list.USAGE_TOKEN: "AI Data Scrapers",
}


class CompareTests(unittest.TestCase):
    def setUp(self):
        self.changes = robots_list.compare(OLD, NEW)

    def test_allowed_to_blocked(self):
        self.assertEqual(self.changes["allowed_to_blocked"], ["WillBlock"])

    def test_blocked_to_allowed(self):
        self.assertEqual(self.changes["blocked_to_allowed"], ["WillAllow"])

    def test_new_blocked_with_own_description(self):
        self.assertEqual(self.changes["new_blocked_own_description"], ["NewOwn"])

    def test_new_blocked_in_known_category(self):
        self.assertEqual(self.changes["new_blocked_known_category"], ["NewKnown"])

    def test_new_allowed_leaves_the_token_out(self):
        self.assertEqual(self.changes["new_allowed"], ["NewAllowed"])

    def test_vanished_exception(self):
        self.assertIn("Claude-SearchBot", self.changes["vanished_exceptions"])
        self.assertNotIn("PerplexityBot", self.changes["vanished_exceptions"])

    def test_no_change(self):
        changes = robots_list.compare(NEW, NEW)
        self.assertEqual(
            robots_list.report(changes, NEW)[0:1],
            ["Exceptions no longer in the list:"],
        )
        self.assertEqual(changes["allowed_to_blocked"], [])

    def test_report_shows_the_function_of_new_blocked(self):
        lines = robots_list.report(self.changes, NEW)
        self.assertIn("  NewOwn  [Reads the web, no more words.]", lines)


class CommitsTests(unittest.TestCase):
    def test_commits_before_the_pinned_one(self):
        commits = [{"sha": "c"}, {"sha": "b"}, {"sha": "p"}, {"sha": "a"}]
        self.assertEqual(robots_list.newer_commits(commits, "p"), commits[:2])

    def test_pinned_first_means_up_to_date(self):
        self.assertEqual(robots_list.newer_commits([{"sha": "p"}], "p"), [])

    def test_pinned_missing_is_an_error(self):
        with self.assertRaises(robots_list.RobotsListError):
            robots_list.newer_commits([{"sha": "x"}], "p")


if __name__ == "__main__":
    unittest.main()
