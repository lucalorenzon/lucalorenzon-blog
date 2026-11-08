"""Tests for version_check.py, with no network and no subprocess.

Run: python3 -m unittest discover -s scripts/version-check -p 'test_*.py'
"""

import contextlib
import io
import json
import unittest

import version_check

ROBOTS_SHA = "a" * 40


def release(tag: str):
    def download(url: str) -> bytes:
        assert url == version_check.TAILWIND_LATEST_RELEASE
        return json.dumps({"tag_name": tag}).encode("utf-8")

    return download


def robots_output(changes: bool = True, commit: str | None = ROBOTS_SHA) -> str:
    return json.dumps(
        {
            "pinned_commit": "p" * 40,
            "latest_commit": commit,
            "latest_date": "2026-11-07T08:00:00Z",
            "commits_after_pin": 2,
            "blocked_list_changes": changes,
            "blocked_now": 10,
            "blocked_after": 12,
            "report": ["New and blocked, in a known category:", "  Fresh"],
            "latest_sha256": "b" * 64,
            "latest_url": "https://example.test/robots.json",
        }
    )


def runner(pinned: str = "4.3.3", robots: str | None = None):
    def run(command: list[str]) -> str:
        if command[0] == "scripts/get-tailwind.sh":
            assert command[1:] == ["--version"]
            return pinned + "\n"
        assert command[-2:] == ["--check-updates", "--json"]
        return robots if robots is not None else robots_output(changes=False)

    return run


class ParseVersionTests(unittest.TestCase):
    def test_with_and_without_v(self):
        self.assertEqual(version_check.parse_version("v4.3.5"), (4, 3, 5))
        self.assertEqual(version_check.parse_version("4.3.5"), (4, 3, 5))

    def test_compared_as_numbers_not_text(self):
        self.assertGreater(
            version_check.parse_version("4.10.0"), version_check.parse_version("4.9.9")
        )

    def test_unparsable_is_an_error(self):
        for text in ("4.3", "v4.3.5-beta.1", "latest", ""):
            with self.assertRaises(version_check.VersionCheckError, msg=text):
                version_check.parse_version(text)


class TailwindTests(unittest.TestCase):
    def test_newer_version_is_a_finding(self):
        finding = version_check.tailwind_finding(runner(), release("v4.3.5"))
        assert finding is not None
        self.assertEqual((finding.tool, finding.key), ("tailwind", "4.3.5"))
        self.assertEqual(finding.title, "Tailwind 4.3.5 is available (pinned: 4.3.3)")
        self.assertEqual(
            finding.marker, "<!-- version-check: tool=tailwind key=4.3.5 -->"
        )
        self.assertTrue(finding.body.endswith(finding.marker))
        self.assertIn(
            "https://github.com/tailwindlabs/tailwindcss/releases/tag/v4.3.5",
            finding.body,
        )
        self.assertIn("TAILWIND_VERSION", finding.body)
        self.assertIn("TAILWIND_SHA256", finding.body)
        self.assertIn("sha256sum", finding.body)
        self.assertIn("skipped", finding.body)

    def test_same_or_older_version_is_nothing(self):
        for tag in ("v4.3.3", "v4.2.9"):
            self.assertIsNone(version_check.tailwind_finding(runner(), release(tag)))

    def test_tag_that_does_not_parse_is_an_error(self):
        with self.assertRaises(version_check.VersionCheckError):
            version_check.tailwind_finding(runner(), release("nightly"))

    def test_answer_without_tag_is_an_error(self):
        with self.assertRaises(version_check.VersionCheckError):
            version_check.tailwind_finding(runner(), lambda url: b"{}")

    def test_answer_that_is_not_json_is_an_error(self):
        with self.assertRaises(version_check.VersionCheckError):
            version_check.tailwind_finding(runner(), lambda url: b"<html>")


class RobotsTests(unittest.TestCase):
    def test_changed_blocked_list_is_a_finding(self):
        finding = version_check.robots_finding(runner(robots=robots_output()))
        assert finding is not None
        self.assertEqual((finding.tool, finding.key), ("robots", ROBOTS_SHA))
        self.assertEqual(
            finding.title,
            "The list of AI robots changed: 12 would be blocked instead of 10 "
            "(latest commit aaaaaaa)",
        )
        self.assertEqual(
            finding.marker, f"<!-- version-check: tool=robots key={ROBOTS_SHA} -->"
        )
        self.assertTrue(finding.body.endswith(finding.marker))
        self.assertIn("  Fresh", finding.body)
        self.assertIn(f'PINNED_COMMIT = "{ROBOTS_SHA}"', finding.body)
        self.assertIn('PINNED_SHA256 = "' + "b" * 64 + '"', finding.body)
        self.assertIn("https://example.test/robots.json", finding.body)
        self.assertIn("skipped", finding.body)

    def test_unchanged_blocked_list_is_nothing(self):
        self.assertIsNone(
            version_check.robots_finding(runner(robots=robots_output(changes=False)))
        )

    def test_nothing_after_the_pin_is_nothing(self):
        out = robots_output(changes=False, commit=None)
        self.assertIsNone(version_check.robots_finding(runner(robots=out)))

    def test_unexpected_output_is_an_error(self):
        for out in ("not json", "{}"):
            with self.assertRaises(version_check.VersionCheckError, msg=out):
                version_check.robots_finding(runner(robots=out))


class HeadersTests(unittest.TestCase):
    def test_token_only_to_the_api_host(self):
        env = {"GITHUB_TOKEN": "secret"}
        api = version_check.request_headers("https://api.github.com/repos/x", env)
        self.assertEqual(api["Authorization"], "Bearer secret")
        for url in (
            "https://github.com/x",
            "https://api.github.com.evil.example/x",
        ):
            self.assertNotIn(
                "Authorization", version_check.request_headers(url, env), url
            )

    def test_no_token_no_header(self):
        headers = version_check.request_headers("https://api.github.com/x", {})
        self.assertNotIn("Authorization", headers)


class MainTests(unittest.TestCase):
    def main(self, **kwargs) -> tuple[int, str, str]:
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = version_check.main([], **kwargs)
        return code, out.getvalue(), err.getvalue()

    def test_nothing_newer(self):
        code, out, _ = self.main(run=runner(), download=release("v4.3.3"))
        self.assertEqual((code, out), (0, "Nothing newer\n"))

    def test_findings_are_printed_title_then_body(self):
        code, out, _ = self.main(
            run=runner(robots=robots_output()), download=release("v4.3.5")
        )
        self.assertEqual(code, 0)
        self.assertIn("Tailwind 4.3.5 is available (pinned: 4.3.3)\n\n", out)
        self.assertIn("The list of AI robots changed:", out)

    def test_error_goes_to_stderr_with_exit_1(self):
        def failing(url: str) -> bytes:
            raise version_check.VersionCheckError("no network")

        code, out, err = self.main(run=runner(), download=failing)
        self.assertEqual((code, out), (1, ""))
        self.assertIn("error: no network", err)

    def test_subprocess_failure_is_exit_1(self):
        def run(command: list[str]) -> str:
            raise version_check.VersionCheckError("boom")

        code, _, err = self.main(run=run, download=release("v4.3.5"))
        self.assertEqual(code, 1)
        self.assertIn("boom", err)


if __name__ == "__main__":
    unittest.main()
