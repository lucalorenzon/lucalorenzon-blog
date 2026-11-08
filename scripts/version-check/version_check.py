"""Finds newer versions of the two things the build pins by hand.

Dependabot watches the Rust crates, the npm packages and the workflows'
actions, but not these two: Tailwind (version and checksum in
scripts/get-tailwind.sh) and the community list of AI robots (commit and
checksum in scripts/robots-list/robots_list.py). This step asks GitHub whether
either has something newer that is worth a decision and turns the answer into
findings, each with what an issue needs: a title, a Markdown body and a hidden
marker that lets a later run recognise the issue it already opened.

- Tailwind: a finding for every release newer than the pinned version.
- The robots list: a finding only if moving the pin to the project's latest
  commit would change which robots are blocked. The list changes often and
  most changes do not touch the blocked set, so they are not worth a signal.

Each finding becomes a GitHub issue with the label "version-check". A hidden
marker in the issue's body (not its title, which a person may edit) says which
finding it is, so a daily run can tell what it already reported:

- Tailwind: one issue per newer version. If an issue for that version exists,
  open or closed, nothing is done; otherwise one is opened. Closing an issue
  without updating means that version is skipped.
- The robots list: if an issue for the latest commit exists, open or closed,
  nothing is done; else, if an issue for an older commit is still open, its
  title and body are updated to the latest commit (one open issue, not a
  pile); otherwise a new one is opened. Closing without moving the pin skips
  the list until it changes again in a way that touches the blocked robots.

Run (read-only: prints what it would do, writes nothing):
  GITHUB_TOKEN=$(gh auth token) python3 scripts/version-check/version_check.py
Run and write (what the daily workflow does; needs a token that may write issues):
  GITHUB_TOKEN=... python3 scripts/version-check/version_check.py --apply
Tests: python3 -m unittest discover -s scripts/version-check -p 'test_*.py'

The repository is read from GITHUB_REPOSITORY (set by GitHub Actions) or, when
that is absent, from the "origin" remote. The token is optional for reading
but, on GitHub's shared runners, the unauthenticated limit of 60 requests an
hour is shared with others and is used up; it is required with --apply. It is
only ever sent to api.github.com.

Exit status: 0 when the check ran, whether or not it found something (the
issue is the signal); 1 when it could not run (network, unexpected answers),
so that the failed run is mailed by GitHub; 2 for a wrong command line.
"""

import json
import os
import re
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
from collections.abc import Callable, Mapping
from dataclasses import dataclass
from pathlib import Path

TAILWIND_LATEST_RELEASE = (
    "https://api.github.com/repos/tailwindlabs/tailwindcss/releases/latest"
)
TAILWIND_RELEASE_PAGE = "https://github.com/tailwindlabs/tailwindcss/releases/tag/v"

# Runs a command from the repository root and returns what it printed.
Runner = Callable[[list[str]], str]
# Returns the body of an HTTP GET.
Getter = Callable[[str], bytes]

LABEL = "version-check"
LABEL_COLOR = "0e8a16"
LABEL_DESCRIPTION = "A newer version of something the build pins by hand"
PAGE_SIZE = 100
MARKER_PATTERN = re.compile(r"<!-- version-check: tool=(\S+) key=(\S+) -->")
REPOSITORY_PATTERNS = (
    re.compile(r"^([\w.-]+)/([\w.-]+?)(?:\.git)?$"),
    re.compile(r"^https://github\.com/([\w.-]+)/([\w.-]+?)(?:\.git)?/?$"),
    re.compile(r"^(?:ssh://)?git@github\.com[:/]([\w.-]+)/([\w.-]+?)(?:\.git)?/?$"),
)


@dataclass(frozen=True)
class Response:
    status: int
    body: bytes


# Sends a request to GitHub's API: method, URL, optional JSON payload. A status
# of 400 or more is returned, not raised, because a 404 is sometimes an answer.
Api = Callable[[str, str, object | None], Response]


class VersionCheckError(Exception):
    """A version could not be found or an answer was not what was expected."""


@dataclass(frozen=True)
class Finding:
    """Something newer, in the form an issue needs."""

    tool: str  # "tailwind" or "robots"
    key: str  # the version, or the latest commit's full SHA
    title: str
    body: str  # Markdown, ends with the marker
    marker: str


def marker(tool: str, key: str) -> str:
    """The hidden comment that tells a later run which finding an issue is."""
    return f"<!-- version-check: tool={tool} key={key} -->"


def request_headers(url: str, environment: Mapping[str, str]) -> dict[str, str]:
    """The token goes only to GitHub's API host: no other host may see it.

    Same rule as in scripts/robots-list/robots_list.py; the scripts are kept
    free of imports from one another, so it is written twice.
    """
    headers = {
        "User-Agent": "lucalorenzon-blog version-check",
        "Accept": "application/vnd.github+json",
    }
    token = environment.get("GITHUB_TOKEN")
    if token and urllib.parse.urlparse(url).hostname == "api.github.com":
        headers["Authorization"] = f"Bearer {token}"
    return headers


def http_get(url: str) -> bytes:
    request = urllib.request.Request(url, headers=request_headers(url, os.environ))
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return response.read()
    except OSError as error:  # URLError and timeouts are OSErrors
        raise VersionCheckError(f"cannot download {url}: {error}") from error


def http_api(method: str, url: str, payload: object | None = None) -> Response:
    headers = request_headers(url, os.environ)
    data = None
    if payload is not None:
        data = json.dumps(payload).encode("utf-8")
        headers["Content-Type"] = "application/json"
    request = urllib.request.Request(url, data=data, headers=headers, method=method)
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return Response(response.status, response.read())
    except urllib.error.HTTPError as error:
        return Response(error.code, error.read())
    except OSError as error:
        raise VersionCheckError(f"cannot reach {url}: {error}") from error


def run_command(repo: Path) -> Runner:
    def run(command: list[str]) -> str:
        try:
            result = subprocess.run(
                command, cwd=repo, capture_output=True, text=True, check=False
            )
        except OSError as error:
            raise VersionCheckError(
                f"cannot run {' '.join(command)}: {error}"
            ) from error
        if result.returncode != 0:
            raise VersionCheckError(
                f"{' '.join(command)} failed: {result.stderr.strip()}"
            )
        return result.stdout

    return run


def parse_version(text: str) -> tuple[int, int, int]:
    """'v4.3.5' or '4.3.5' as (4, 3, 5); anything else is an error."""
    match = re.fullmatch(r"v?(\d+)\.(\d+)\.(\d+)", text.strip())
    if not match:
        raise VersionCheckError(f"{text!r} is not a version like 4.3.5")
    major, minor, patch = match.groups()
    return int(major), int(minor), int(patch)


def tailwind_finding(run: Runner, download: Getter) -> Finding | None:
    pinned_text = run(["scripts/get-tailwind.sh", "--version"]).strip()
    pinned = parse_version(pinned_text)
    try:
        # GitHub leaves pre-releases out of "latest".
        tag = json.loads(download(TAILWIND_LATEST_RELEASE).decode("utf-8"))["tag_name"]
    except (ValueError, KeyError, TypeError) as error:
        raise VersionCheckError(
            f"GitHub's answer about Tailwind's latest release is not as expected: {error}"
        ) from error
    latest = parse_version(str(tag))
    if latest <= pinned:
        return None
    version = ".".join(str(part) for part in latest)
    key_marker = marker("tailwind", version)
    body = "\n".join(
        [
            f"Tailwind CSS {version} is out; the site is pinned at {pinned_text}.",
            "",
            f"Release notes: {TAILWIND_RELEASE_PAGE}{version}",
            "",
            (
                "To update, in `scripts/get-tailwind.sh` change `TAILWIND_VERSION` to "
                f"`{version}` and `TAILWIND_SHA256` to the checksum of the new file: "
                f"download `tailwindcss-linux-x64` from the release page and run "
                "`sha256sum` on it."
            ),
            "",
            "Closing this issue without updating means this version is skipped.",
            "",
            key_marker,
        ]
    )
    return Finding(
        tool="tailwind",
        key=version,
        title=f"Tailwind {version} is available (pinned: {pinned_text})",
        body=body,
        marker=key_marker,
    )


def robots_finding(run: Runner) -> Finding | None:
    output = run(
        [
            sys.executable,
            "scripts/robots-list/robots_list.py",
            "--check-updates",
            "--json",
        ]
    )
    try:
        data = json.loads(output)
        if not data["blocked_list_changes"]:
            return None
        commit = str(data["latest_commit"])
        now, after = int(data["blocked_now"]), int(data["blocked_after"])
        lines = [str(line) for line in data["report"]]
        url, sha256 = str(data["latest_url"]), str(data["latest_sha256"])
        date = str(data["latest_date"])
    except (ValueError, KeyError, TypeError) as error:
        raise VersionCheckError(
            f"the robots list check printed something unexpected: {error}"
        ) from error
    key_marker = marker("robots", commit)
    body = "\n".join(
        [
            (
                f"The community list of AI robots has a commit from {date} "
                f"({commit}) that would change the robots the site blocks: "
                f"{after} would be blocked instead of {now}."
            ),
            "",
            "```",
            *lines,
            "```",
            "",
            "To move the pin:",
            "",
            (
                f"1. download {url} and put it at "
                "`scripts/robots-list/ai-robots-txt/robots.json` (check `LICENSE` too);"
            ),
            (
                "2. in `scripts/robots-list/robots_list.py` set "
                f'`PINNED_COMMIT = "{commit}"` and `PINNED_SHA256 = "{sha256}"`.'
            ),
            "",
            "Closing this issue without updating means this change is skipped.",
            "",
            key_marker,
        ]
    )
    return Finding(
        tool="robots",
        key=commit,
        title=(
            f"The list of AI robots changed: {after} would be blocked "
            f"instead of {now} (latest commit {commit[:7]})"
        ),
        body=body,
        marker=key_marker,
    )


def find(run: Runner, download: Getter) -> list[Finding]:
    candidates = (tailwind_finding(run, download), robots_finding(run))
    return [finding for finding in candidates if finding is not None]


def parse_repository(text: str) -> str:
    """'owner/name' from 'owner/name' or from a GitHub remote's address."""
    for pattern in REPOSITORY_PATTERNS:
        match = pattern.match(text.strip())
        if match:
            return f"{match.group(1)}/{match.group(2)}"
    raise VersionCheckError(f"{text!r} is not a GitHub repository")


def repository(environment: Mapping[str, str], run: Runner) -> str:
    named = environment.get("GITHUB_REPOSITORY")
    if named:
        return parse_repository(named)
    return parse_repository(run(["git", "remote", "get-url", "origin"]))


@dataclass(frozen=True)
class Issue:
    number: int
    is_open: bool
    tool: str
    key: str


@dataclass(frozen=True)
class Action:
    kind: str  # "open", "update" or "nothing"
    finding: Finding
    number: int | None = None  # the issue to update, or the one that covers it


def parse_issue(item: object) -> Issue | None:
    """An issue we opened, recognised by the marker in its body; else None."""
    if not isinstance(item, dict) or "pull_request" in item:
        return None
    try:
        number, state = int(item["number"]), str(item["state"])
    except (KeyError, TypeError, ValueError) as error:
        raise VersionCheckError(f"an issue has no number or state: {error}") from error
    match = MARKER_PATTERN.search(str(item.get("body") or ""))
    if not match:
        return None
    return Issue(number, state == "open", match.group(1), match.group(2))


def decide(findings: list[Finding], issues: list[Issue]) -> list[Action]:
    """What to do about each finding, given the issues that exist.

    Pure on purpose: the whole policy is here, and the tests cover its branches.
    """
    actions = []
    for finding in findings:
        same_key = [i for i in issues if (i.tool, i.key) == (finding.tool, finding.key)]
        if same_key:
            actions.append(Action("nothing", finding, same_key[0].number))
            continue
        if finding.tool == "robots":
            open_older = [i for i in issues if i.tool == "robots" and i.is_open]
            if open_older:
                newest = max(open_older, key=lambda i: i.number)
                actions.append(Action("update", finding, newest.number))
                continue
        actions.append(Action("open", finding))
    return actions


def check(response: Response, what: str, allowed: tuple[int, ...] = ()) -> Response:
    if response.status >= 400 and response.status not in allowed:
        detail = response.body.decode("utf-8", "replace")[:200]
        raise VersionCheckError(f"{what} failed with {response.status}: {detail}")
    return response


def existing_issues(api: Api, repo: str) -> list[Issue]:
    issues = []
    page = 1
    while True:
        url = (
            f"https://api.github.com/repos/{repo}/issues?labels={LABEL}"
            f"&state=all&per_page={PAGE_SIZE}&page={page}"
        )
        body = check(api("GET", url, None), "listing the issues").body
        try:
            items = json.loads(body.decode("utf-8"))
        except ValueError as error:
            raise VersionCheckError(
                f"the list of issues is not JSON: {error}"
            ) from error
        if not isinstance(items, list):
            raise VersionCheckError("the list of issues is not a list")
        issues.extend(i for i in map(parse_issue, items) if i is not None)
        if len(items) < PAGE_SIZE:
            return issues
        page += 1


def ensure_label(api: Api, repo: str) -> None:
    base = f"https://api.github.com/repos/{repo}/labels"
    found = check(api("GET", f"{base}/{LABEL}", None), "reading the label", (404,))
    if found.status != 404:
        return
    check(
        api(
            "POST",
            base,
            {"name": LABEL, "color": LABEL_COLOR, "description": LABEL_DESCRIPTION},
        ),
        "creating the label",
    )


def describe(action: Action) -> str:
    title = action.finding.title
    if action.kind == "open":
        return f"would open an issue: {title}"
    if action.kind == "update":
        return f"would update issue #{action.number}: {title}"
    return f"nothing to do: issue #{action.number} already covers: {title}"


def apply(api: Api, repo: str, actions: list[Action]) -> list[str]:
    """Do what decide() said; returns one line per action."""
    base = f"https://api.github.com/repos/{repo}/issues"
    lines = []
    label_ready = False
    for action in actions:
        finding = action.finding
        if action.kind == "nothing":
            lines.append(describe(action))
        elif action.kind == "update":
            payload = {"title": finding.title, "body": finding.body}
            check(api("PATCH", f"{base}/{action.number}", payload), "updating an issue")
            lines.append(f"updated issue #{action.number}: {finding.title}")
        else:
            if not label_ready:
                ensure_label(api, repo)
                label_ready = True
            payload = {
                "title": finding.title,
                "body": finding.body,
                "labels": [LABEL],
            }
            created = check(api("POST", base, payload), "opening an issue")
            try:
                number = json.loads(created.body.decode("utf-8"))["number"]
            except (ValueError, KeyError, TypeError) as error:
                raise VersionCheckError(
                    f"GitHub's answer has no number: {error}"
                ) from error
            lines.append(f"opened issue #{number}: {finding.title}")
    return lines


USAGE = "usage: version_check.py [--apply]"


def main(
    argv: list[str] | None = None,
    run: Runner | None = None,
    download: Getter | None = None,
    api: Api | None = None,
    environment: Mapping[str, str] | None = None,
) -> int:
    argv = argv or []
    if argv not in ([], ["--apply"]):
        print(USAGE, file=sys.stderr)
        return 2
    do_apply = argv == ["--apply"]
    environment = os.environ if environment is None else environment
    if do_apply and not environment.get("GITHUB_TOKEN"):
        print("error: --apply needs GITHUB_TOKEN to write issues", file=sys.stderr)
        return 1
    root = Path(__file__).resolve().parents[2]
    run = run or run_command(root)
    api = api or http_api
    try:
        findings = find(run, download or http_get)
        if not findings:
            print("Nothing newer")
            return 0
        repo = repository(environment, run)
        actions = decide(findings, existing_issues(api, repo))
        lines = (
            apply(api, repo, actions) if do_apply else [describe(a) for a in actions]
        )
    except VersionCheckError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
