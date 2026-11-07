"""Writes the date of the last change of every page's Markdown file.

The sitemap needs a "last modified" date per page. Git knows it, but the
build job checks out one commit only, so this step runs where the full
history exists and hands the result to the build as a plain file.

Output: one line per file, "<path from the repository root>\t<YYYY-MM-DD>",
sorted by path. Files git has no commit for are left out.

Run: python3 scripts/content-dates/content_dates.py --output FILE
"""

import argparse
import os
import subprocess
import sys
from pathlib import Path

# Files in content/ that are not pages (the sitemap must not list them).
EXCLUDED = ("content/README.md", "content/LICENSE-MIT-0")


class GitError(Exception):
    """Git did not answer: missing, not a repository, or it failed."""


def git(repo: Path, *args: str) -> str:
    try:
        result = subprocess.run(
            ["git", *args], cwd=repo, capture_output=True, text=True, check=False
        )
    except OSError as error:
        raise GitError(f"cannot run git: {error}") from error
    if result.returncode != 0:
        raise GitError(f"git {' '.join(args)} failed: {result.stderr.strip()}")
    return result.stdout


def content_files(repo: Path) -> list[str]:
    """Markdown files under content/, as sorted paths with forward slashes."""
    found = (p.relative_to(repo).as_posix() for p in (repo / "content").rglob("*.md"))
    return sorted(p for p in found if p not in EXCLUDED)


def last_change_date(repo: Path, path: str) -> str | None:
    # %as is the AUTHOR date, the day the change was written: a rebase keeps
    # it, while it changes the committer date.
    date = git(repo, "log", "-1", "--format=%as", "--", path).strip()
    return date or None


def collect(repo: Path) -> list[tuple[str, str]]:
    git(repo, "rev-parse", "--git-dir")  # fails early when there is no repository
    dates = ((path, last_change_date(repo, path)) for path in content_files(repo))
    return [(path, date) for path, date in dates if date]


def render(entries: list[tuple[str, str]]) -> str:
    return "".join(f"{path}\t{date}\n" for path, date in entries)


def write(output: Path, text: str) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(text, encoding="utf-8", newline="\n")


def main(argv: list[str] | None = None) -> int:
    default_repo = Path(__file__).resolve().parents[2]
    parser = argparse.ArgumentParser(
        description="Write the date of the last change of every page's Markdown file."
    )
    parser.add_argument("--repo", type=Path, default=default_repo)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args(argv)

    try:
        text = render(collect(args.repo))
    except GitError as error:
        # Strict in the pipeline, where a missing date must stop the build;
        # tolerant on a laptop, where the history may be absent.
        if os.environ.get("CI"):
            print(f"error: {error}", file=sys.stderr)
            return 1
        print(f"warning: {error}; writing an empty dates file", file=sys.stderr)
        text = ""
    write(args.output, text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
