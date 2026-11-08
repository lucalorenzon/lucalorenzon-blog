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

Run: GITHUB_TOKEN=$(gh auth token) python3 scripts/version-check/version_check.py
Tests: python3 -m unittest discover -s scripts/version-check -p 'test_*.py'

For now the findings are only printed; nothing is written anywhere. The token
is optional but, on GitHub's shared runners, the unauthenticated limit of 60
requests an hour is shared with others and is used up.
"""

import json
import os
import re
import subprocess
import sys
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


def render(findings: list[Finding]) -> str:
    if not findings:
        return "Nothing newer\n"
    return "\n\n".join(f"{f.title}\n\n{f.body}" for f in findings) + "\n"


def main(
    argv: list[str] | None = None,
    run: Runner | None = None,
    download: Getter | None = None,
) -> int:
    if argv:
        print("usage: version_check.py (takes no arguments)", file=sys.stderr)
        return 2
    repo = Path(__file__).resolve().parents[2]
    try:
        findings = find(run or run_command(repo), download or http_get)
    except VersionCheckError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    print(render(findings), end="")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
