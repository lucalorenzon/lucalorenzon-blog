"""Writes the names of the robots the site's robots.txt blocks.

The robots.txt blocks the robots that collect text to train AI models and
lets through those that read a page for a user or index it for search. The
list of AI robots is kept by a community project; this step downloads it at a
pinned commit, checks its checksum, filters it and hands the build the names
to block as a plain file (one per line, sorted).

With --check-updates it instead tells what would change in the blocked list
if the pin moved to the project's latest commit. It needs the network.

Run: python3 scripts/robots-list/robots_list.py --output FILE
     python3 scripts/robots-list/robots_list.py --check-updates
Tests: python3 -m unittest discover -s scripts/robots-list -p 'test_*.py'
"""

import argparse
import hashlib
import json
import sys
import urllib.request
from collections.abc import Callable
from pathlib import Path

# The pinned source: robots.json of https://github.com/ai-robots-txt/ai.robots.txt
# at this commit (2026-11-05). Moving the pin is a deliberate act: run
# --check-updates, read what would change, then change the three lines below.
PROJECT = "ai-robots-txt/ai.robots.txt"
PINNED_COMMIT = "508985e256295b837210c8d13cf8dd8f1bd04673"
PINNED_SHA256 = "4695fa6839dfb2ff8dbd9e00f673b9685fa5c16d0cb86e3fb779469baf185337"

# Only robots whose function is one of these pass: a robot that is new or
# unclear stays blocked until it falls into one of them.
ALLOWED_FUNCTIONS = (
    "AI Assistants",
    "AI Search Crawlers",
    "AI Agents",
    "AI Coding Agents",
)

# The categories the project uses again and again. Any other function text is
# a description of its own, which may hide a robot that reads on request.
KNOWN_FUNCTIONS = (
    *ALLOWED_FUNCTIONS,
    "AI Data Scrapers",
    "AI Data Providers",
    "Undocumented AI Agents",
)

# Robots whose own description says they only search or read on request, so
# they pass although their function is not in ALLOWED_FUNCTIONS.
EXCEPTIONS = (
    "Claude-SearchBot",
    "OAI-SearchBot",
    "PerplexityBot",
    "MistralAI-Index",
    "MistralAI-User/1.0",
    "cohere-ai",
    "iaskspider/2.0",
    "Andibot",
)

# The list has an entry for this policy token. It is not a robot: the Rust
# build writes its group in robots.txt itself, so it is left out here.
USAGE_TOKEN = "__usage_ai-training"

# Robot name -> its function text, the only part of robots.json used here.
Functions = dict[str, str]


class RobotsListError(Exception):
    """The list could not be obtained or did not match the pin."""


def raw_url(commit: str) -> str:
    return f"https://raw.githubusercontent.com/{PROJECT}/{commit}/robots.json"


def http_get(url: str) -> bytes:
    request = urllib.request.Request(
        url, headers={"User-Agent": "lucalorenzon-blog robots-list check"}
    )
    try:
        with urllib.request.urlopen(request, timeout=60) as response:
            return response.read()
    except OSError as error:  # URLError and timeouts are OSErrors
        raise RobotsListError(f"cannot download {url}: {error}") from error


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def parse_functions(data: bytes) -> Functions:
    try:
        robots = json.loads(data.decode("utf-8"))
        return {name: str(entry["function"]).strip() for name, entry in robots.items()}
    except (ValueError, KeyError, TypeError, AttributeError) as error:
        raise RobotsListError(
            f"robots.json is not in the expected shape: {error}"
        ) from error


def is_blocked(name: str, function: str) -> bool:
    if name == USAGE_TOKEN:
        return False
    return function.strip() not in ALLOWED_FUNCTIONS and name not in EXCEPTIONS


def blocked_names(functions: Functions) -> list[str]:
    return sorted(
        name for name, function in functions.items() if is_blocked(name, function)
    )


def render(names: list[str]) -> str:
    return "".join(f"{name}\n" for name in names)


def write(output: Path, text: str) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(text, encoding="utf-8", newline="\n")


def pinned_list(
    repo: Path,
    download: Callable[[str], bytes] | None = None,
    commit: str = PINNED_COMMIT,
    expected: str = PINNED_SHA256,
) -> bytes:
    """The pinned robots.json, from the cache in target/tools/ or downloaded."""
    download = download or http_get
    cached = repo / "target" / "tools" / f"ai-robots-{commit}.json"
    if cached.is_file():
        data = cached.read_bytes()
        if sha256(data) == expected:
            return data
    data = download(raw_url(commit))
    actual = sha256(data)
    if actual != expected:
        raise RobotsListError(
            f"robots.json checksum mismatch (expected {expected}, got {actual})"
        )
    # Written under a temporary name and renamed, so a stopped run never
    # leaves a half-written file that looks like a valid cache.
    cached.parent.mkdir(parents=True, exist_ok=True)
    temporary = cached.with_name(cached.name + ".tmp")
    temporary.write_bytes(data)
    temporary.replace(cached)
    return data


def compare(old: Functions, new: Functions) -> dict[str, list[str]]:
    """What changes in the blocked list when the list goes from old to new."""

    def blocked(functions: Functions, name: str) -> bool:
        return is_blocked(name, functions[name])

    added = sorted(set(new) - set(old))
    kept = sorted(set(new) & set(old))
    # A new robot with a function of its own may be one to except: tell apart.
    new_blocked = [name for name in added if blocked(new, name)]
    return {
        "allowed_to_blocked": [
            n for n in kept if not blocked(old, n) and blocked(new, n)
        ],
        "blocked_to_allowed": [
            n for n in kept if blocked(old, n) and not blocked(new, n)
        ],
        "new_blocked_own_description": [
            n for n in new_blocked if new[n] not in KNOWN_FUNCTIONS
        ],
        "new_blocked_known_category": [
            n for n in new_blocked if new[n] in KNOWN_FUNCTIONS
        ],
        "new_allowed": [n for n in added if not blocked(new, n) and n != USAGE_TOKEN],
        "vanished_exceptions": [n for n in EXCEPTIONS if n not in new],
    }


def report(changes: dict[str, list[str]], new: Functions) -> list[str]:
    titles = {
        "allowed_to_blocked": "Would go from allowed to blocked",
        "blocked_to_allowed": "Would go from blocked to allowed",
        "new_blocked_own_description": (
            "New and blocked, with a description of their own (maybe an exception)"
        ),
        "new_blocked_known_category": "New and blocked, in a known category",
        "new_allowed": "New and allowed",
        "vanished_exceptions": "Exceptions no longer in the list",
    }
    lines: list[str] = []
    for key, title in titles.items():
        names = changes[key]
        if not names:
            continue
        lines.append(f"{title}:")
        for name in names:
            suffix = f"  [{new[name]}]" if key.startswith("new_blocked") else ""
            lines.append(f"  {name}{suffix}")
    return lines


def newer_commits(commits: list[dict], pinned: str) -> list[dict]:
    """The commits before the pinned one in a newest-first list."""
    for position, commit in enumerate(commits):
        if commit["sha"] == pinned:
            return commits[:position]
    raise RobotsListError(
        f"the pinned commit {pinned} is not in the project's recent history"
    )


def commits_changing_list(download: Callable[[str], bytes]) -> list[dict]:
    """Commits that changed robots.json after the pinned one, newest first."""
    commits: list[dict] = []
    for page in range(1, 6):
        url = (
            f"https://api.github.com/repos/{PROJECT}/commits"
            f"?path=robots.json&per_page=100&page={page}"
        )
        try:
            batch = json.loads(download(url).decode("utf-8"))
        except ValueError as error:
            raise RobotsListError(f"GitHub's answer is not JSON: {error}") from error
        commits.extend(batch)
        if any(c["sha"] == PINNED_COMMIT for c in batch) or len(batch) < 100:
            break
    return newer_commits(commits, PINNED_COMMIT)


def check_updates(repo: Path, download: Callable[[str], bytes] | None = None) -> int:
    download = download or http_get
    old = parse_functions(pinned_list(repo, download))
    newer = commits_changing_list(download)
    if not newer:
        print(f"Up to date: no commit changed robots.json after {PINNED_COMMIT}.")
        return 0
    latest = newer[0]
    date = latest["commit"]["committer"]["date"]
    print(
        f"{len(newer)} commit(s) changed robots.json after the pin; the latest is from {date}."
    )
    # A preview of the newest version: no checksum, it is not pinned yet.
    data = download(raw_url(latest["sha"]))
    new = parse_functions(data)
    lines = report(compare(old, new), new)
    print(
        f"Blocked now: {len(blocked_names(old))}; blocked after the move: {len(blocked_names(new))}."
    )
    print("\n".join(lines) if lines else "The blocked list would not change.")
    print("To move the pin, set in robots_list.py:")
    print(f'  PINNED_COMMIT = "{latest["sha"]}"')
    print(f'  PINNED_SHA256 = "{sha256(data)}"')
    return 0


def main(argv: list[str] | None = None) -> int:
    default_repo = Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser(
        description="Write the names of the robots to block, or check for a newer list."
    )
    parser.add_argument("--repo", type=Path, default=default_repo)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--check-updates", action="store_true")
    args = parser.parse_args(argv)
    if args.check_updates == (args.output is not None):
        parser.error("give exactly one of --output FILE and --check-updates")

    try:
        if args.check_updates:
            return check_updates(args.repo)
        # An error is an error here in the pipeline and on a laptop alike: a
        # list other than the pinned one must never reach the site.
        names = blocked_names(parse_functions(pinned_list(args.repo)))
        write(args.output, render(names))
    except RobotsListError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
