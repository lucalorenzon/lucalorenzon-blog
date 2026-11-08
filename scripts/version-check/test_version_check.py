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
        if command[:2] == ["git", "remote"]:
            return "git@github.com:someone/blog.git\n"
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


REPO = "someone/blog"


def finding(tool: str, key: str) -> version_check.Finding:
    mark = version_check.marker(tool, key)
    return version_check.Finding(
        tool, key, f"title {tool} {key}", f"body\n\n{mark}", mark
    )


def issue(tool: str, key: str, number: int = 1, is_open: bool = True):
    return version_check.Issue(number, is_open, tool, key)


class DecideTests(unittest.TestCase):
    def decide(self, found, issues):
        actions = version_check.decide([found], issues)
        self.assertEqual(len(actions), 1)
        return actions[0].kind, actions[0].number

    def test_tailwind_without_issue_opens(self):
        self.assertEqual(self.decide(finding("tailwind", "4.3.5"), []), ("open", None))

    def test_tailwind_with_open_issue_for_that_version_is_nothing(self):
        found, issues = finding("tailwind", "4.3.5"), [issue("tailwind", "4.3.5", 7)]
        self.assertEqual(self.decide(found, issues), ("nothing", 7))

    def test_tailwind_with_closed_issue_for_that_version_is_nothing(self):
        found = finding("tailwind", "4.3.5")
        issues = [issue("tailwind", "4.3.5", 7, is_open=False)]
        self.assertEqual(self.decide(found, issues), ("nothing", 7))

    def test_tailwind_with_issue_for_older_version_only_opens(self):
        found = finding("tailwind", "4.3.5")
        for is_open in (True, False):
            issues = [issue("tailwind", "4.3.4", 7, is_open)]
            self.assertEqual(self.decide(found, issues), ("open", None))

    def test_robots_without_issue_opens(self):
        self.assertEqual(self.decide(finding("robots", "c2"), []), ("open", None))

    def test_robots_with_issue_for_latest_commit_is_nothing(self):
        for is_open in (True, False):
            issues = [issue("robots", "c2", 4, is_open)]
            self.assertEqual(
                self.decide(finding("robots", "c2"), issues), ("nothing", 4)
            )

    def test_robots_with_open_issue_for_older_commit_updates_it(self):
        issues = [issue("robots", "c1", 4)]
        self.assertEqual(self.decide(finding("robots", "c2"), issues), ("update", 4))

    def test_robots_with_only_closed_issue_for_older_commit_opens(self):
        issues = [issue("robots", "c1", 4, is_open=False)]
        self.assertEqual(self.decide(finding("robots", "c2"), issues), ("open", None))

    def test_robots_never_updates_a_tailwind_issue(self):
        issues = [issue("tailwind", "4.3.5", 4)]
        self.assertEqual(self.decide(finding("robots", "c2"), issues), ("open", None))

    def test_latest_commit_issue_wins_over_updating_another(self):
        issues = [issue("robots", "c1", 4), issue("robots", "c2", 5, is_open=False)]
        self.assertEqual(self.decide(finding("robots", "c2"), issues), ("nothing", 5))


class ParseIssueTests(unittest.TestCase):
    def test_marker_in_body(self):
        item = {
            "number": 3,
            "state": "closed",
            "body": "text\n\n" + version_check.marker("tailwind", "4.3.5"),
        }
        self.assertEqual(
            version_check.parse_issue(item), issue("tailwind", "4.3.5", 3, False)
        )

    def test_no_marker_pull_request_and_empty_body_are_skipped(self):
        marked = version_check.marker("robots", "c")
        self.assertIsNone(
            version_check.parse_issue({"number": 1, "state": "open", "body": "hi"})
        )
        self.assertIsNone(
            version_check.parse_issue({"number": 1, "state": "open", "body": None})
        )
        self.assertIsNone(
            version_check.parse_issue(
                {"number": 1, "state": "open", "body": marked, "pull_request": {}}
            )
        )

    def test_title_is_not_used(self):
        item = {"number": 1, "state": "open", "title": "Tailwind 4.3.5", "body": "x"}
        self.assertIsNone(version_check.parse_issue(item))


class RepositoryTests(unittest.TestCase):
    def test_accepted_forms(self):
        for text in (
            "someone/blog",
            "https://github.com/someone/blog.git",
            "https://github.com/someone/blog",
            "git@github.com:someone/blog.git",
            "ssh://git@github.com/someone/blog.git",
        ):
            self.assertEqual(version_check.parse_repository(text), REPO, text)

    def test_unparseable_is_an_error(self):
        for text in ("", "blog", "https://example.com/someone/blog", "a/b/c"):
            with self.assertRaises(version_check.VersionCheckError, msg=text):
                version_check.parse_repository(text)

    def test_environment_first_then_origin(self):
        run = runner()
        self.assertEqual(
            version_check.repository({"GITHUB_REPOSITORY": "x/y"}, run), "x/y"
        )
        self.assertEqual(version_check.repository({}, run), REPO)

    def test_bad_environment_value_is_an_error(self):
        with self.assertRaises(version_check.VersionCheckError):
            version_check.repository({"GITHUB_REPOSITORY": "nope"}, runner())


class FakeApi:
    """Records the calls and answers from a table of (method, url prefix)."""

    def __init__(self, answers):
        self.answers = answers
        self.calls: list[tuple[str, str, object]] = []

    def __call__(self, method: str, url: str, payload=None):
        self.calls.append((method, url, payload))
        for (m, prefix), response in self.answers:
            if m == method and url.startswith(prefix):
                return response
        raise AssertionError(f"unexpected call {method} {url}")


def ok(data, status: int = 200):
    return version_check.Response(status, json.dumps(data).encode("utf-8"))


API = "https://api.github.com/repos/" + REPO
ISSUES = ("GET", API + "/issues?")
LABEL_GET = ("GET", API + "/labels/version-check")
LABEL_POST = ("POST", API + "/labels")
ISSUE_POST = ("POST", API + "/issues")


def raw_issue(number: int, tool: str, key: str, state: str = "open"):
    return {
        "number": number,
        "state": state,
        "body": "b " + version_check.marker(tool, key),
    }


class ExistingIssuesTests(unittest.TestCase):
    def test_pagination_until_a_short_page(self):
        full = [raw_issue(n, "robots", f"c{n}") for n in range(1, 101)]
        full[3] = {"number": 4, "state": "open", "body": "m", "pull_request": {}}
        pages = {
            1: ok(full),
            2: ok([raw_issue(101, "tailwind", "4.3.5", "closed")]),
        }

        def api(method, url, payload=None):
            self.assertEqual(method, "GET")
            self.assertIn("labels=version-check&state=all&per_page=100", url)
            return pages[int(url.rsplit("page=", 1)[1])]

        issues = version_check.existing_issues(api, REPO)
        self.assertEqual(len(issues), 100)  # 99 from page 1 (one PR skipped) + 1
        self.assertEqual(issues[-1], issue("tailwind", "4.3.5", 101, False))

    def test_error_status_is_an_error(self):
        api = FakeApi([(ISSUES, ok({"message": "no"}, 500))])
        with self.assertRaises(version_check.VersionCheckError):
            version_check.existing_issues(api, REPO)


class ApplyTests(unittest.TestCase):
    def test_label_is_created_on_404_then_issue_opened(self):
        api = FakeApi(
            [
                (LABEL_GET, ok({}, 404)),
                (LABEL_POST, ok({}, 201)),
                (ISSUE_POST, ok({"number": 12}, 201)),
            ]
        )
        found = finding("tailwind", "4.3.5")
        lines = version_check.apply(api, REPO, [version_check.Action("open", found)])
        self.assertEqual(lines, [f"opened issue #12: {found.title}"])
        methods = [(m, u.removeprefix(API)) for m, u, _ in api.calls]
        self.assertEqual(
            methods,
            [
                ("GET", "/labels/version-check"),
                ("POST", "/labels"),
                ("POST", "/issues"),
            ],
        )
        self.assertEqual(
            api.calls[2][2],
            {"title": found.title, "body": found.body, "labels": ["version-check"]},
        )

    def test_existing_label_is_not_created(self):
        api = FakeApi([(LABEL_GET, ok({})), (ISSUE_POST, ok({"number": 1}, 201))])
        found = finding("robots", "c")
        version_check.apply(api, REPO, [version_check.Action("open", found)])
        self.assertEqual([m for m, _, _ in api.calls], ["GET", "POST"])

    def test_update_patches_title_and_body_only(self):
        api = FakeApi([(("PATCH", API + "/issues/4"), ok({}))])
        found = finding("robots", "c2")
        lines = version_check.apply(
            api, REPO, [version_check.Action("update", found, 4)]
        )
        self.assertEqual(lines, [f"updated issue #4: {found.title}"])
        self.assertEqual(
            api.calls,
            [("PATCH", API + "/issues/4", {"title": found.title, "body": found.body})],
        )

    def test_nothing_makes_no_call(self):
        api = FakeApi([])
        found = finding("robots", "c2")
        lines = version_check.apply(
            api, REPO, [version_check.Action("nothing", found, 4)]
        )
        self.assertEqual(api.calls, [])
        self.assertIn("issue #4 already covers", lines[0])

    def test_failed_write_is_an_error(self):
        api = FakeApi([(LABEL_GET, ok({})), (ISSUE_POST, ok({"message": "x"}, 403))])
        action = version_check.Action("open", finding("robots", "c"))
        with self.assertRaises(version_check.VersionCheckError):
            version_check.apply(api, REPO, [action])


ENVIRONMENT = {"GITHUB_REPOSITORY": REPO}


class MainTests(unittest.TestCase):
    def main(self, argv=None, env=None, **kwargs) -> tuple[int, str, str]:
        out, err = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(out), contextlib.redirect_stderr(err):
            code = version_check.main(
                argv or [], environment=ENVIRONMENT if env is None else env, **kwargs
            )
        return code, out.getvalue(), err.getvalue()

    def test_nothing_newer(self):
        code, out, _ = self.main(run=runner(), download=release("v4.3.3"))
        self.assertEqual((code, out), (0, "Nothing newer\n"))

    def test_read_only_run_says_what_it_would_do_and_writes_nothing(self):
        api = FakeApi([(ISSUES, ok([raw_issue(9, "robots", ROBOTS_SHA)]))])
        code, out, _ = self.main(
            run=runner(robots=robots_output()), download=release("v4.3.5"), api=api
        )
        self.assertEqual(code, 0)
        self.assertIn("would open an issue: Tailwind 4.3.5 is available", out)
        self.assertIn("nothing to do: issue #9 already covers", out)
        self.assertEqual([m for m, _, _ in api.calls], ["GET"])

    def test_apply_writes(self):
        api = FakeApi(
            [
                (ISSUES, ok([])),
                (LABEL_GET, ok({})),
                (ISSUE_POST, ok({"number": 3}, 201)),
            ]
        )
        code, out, _ = self.main(
            ["--apply"],
            env={**ENVIRONMENT, "GITHUB_TOKEN": "t"},
            run=runner(),
            download=release("v4.3.5"),
            api=api,
        )
        self.assertEqual(code, 0)
        self.assertIn("opened issue #3: Tailwind 4.3.5", out)

    def test_apply_without_token_is_an_error_before_anything_runs(self):
        def never(*args):
            raise AssertionError("must not be called")

        code, out, err = self.main(["--apply"], run=never, download=never, api=never)
        self.assertEqual((code, out), (1, ""))
        self.assertIn("GITHUB_TOKEN", err)

    def test_unknown_argument_is_usage_error(self):
        code, _, err = self.main(["--force"])
        self.assertEqual(code, 2)
        self.assertIn("usage", err)

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
