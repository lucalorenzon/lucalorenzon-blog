"""Tests for content_dates.py, on small temporary git repositories.

Run: python3 -m unittest discover -s scripts/content-dates -p 'test_*.py'
"""

import os
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import content_dates


def run_git(repo: Path, *args: str, author="2026-01-01", committer=None) -> None:
    env = {
        **os.environ,
        "GIT_AUTHOR_DATE": f"{author}T12:00:00+00:00",
        "GIT_COMMITTER_DATE": f"{committer or author}T12:00:00+00:00",
    }
    subprocess.run(["git", *args], cwd=repo, env=env, check=True, capture_output=True)


class RepoTestCase(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.repo = Path(self._tmp.name) / "repo"
        self.repo.mkdir()
        run_git(self.repo, "init", "-q")
        run_git(self.repo, "config", "user.name", "Test")
        run_git(self.repo, "config", "user.email", "test@example.com")
        run_git(self.repo, "config", "commit.gpgsign", "false")

    def commit(self, files: dict[str, str], **dates) -> None:
        for name, text in files.items():
            path = self.repo / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(text, encoding="utf-8")
        run_git(self.repo, "add", "-A")
        run_git(self.repo, "commit", "-q", "-m", "change", **dates)

    def run_main(self, repo: Path | None = None):
        output = Path(self._tmp.name) / "out" / "dates.tsv"
        code = content_dates.main(
            ["--repo", str(repo or self.repo), "--output", str(output)]
        )
        return code, output


class CollectTest(RepoTestCase):
    def test_older_file_keeps_its_older_date(self):
        self.commit({"content/a.md": "a"}, author="2026-01-10")
        self.commit({"content/b.md": "b"}, author="2026-02-20")
        self.assertEqual(
            content_dates.collect(self.repo),
            [("content/a.md", "2026-01-10"), ("content/b.md", "2026-02-20")],
        )

    def test_date_is_the_last_change(self):
        self.commit({"content/a.md": "1"}, author="2026-01-10")
        self.commit({"content/a.md": "2"}, author="2026-03-05")
        self.assertEqual(
            content_dates.collect(self.repo), [("content/a.md", "2026-03-05")]
        )

    def test_file_in_subfolder(self):
        self.commit({"content/articles/x.md": "x"}, author="2026-04-01")
        self.assertEqual(
            content_dates.collect(self.repo),
            [("content/articles/x.md", "2026-04-01")],
        )

    def test_excluded_files_are_not_listed(self):
        self.commit(
            {
                "content/README.md": "r",
                "content/LICENSE-MIT-0": "l",
                "content/a.md": "a",
            }
        )
        self.assertEqual(
            [p for p, _ in content_dates.collect(self.repo)], ["content/a.md"]
        )

    def test_uncommitted_file_is_not_listed(self):
        self.commit({"content/a.md": "a"})
        (self.repo / "content" / "new.md").write_text("n", encoding="utf-8")
        self.assertEqual(
            [p for p, _ in content_dates.collect(self.repo)], ["content/a.md"]
        )

    def test_author_date_wins_over_committer_date(self):
        self.commit({"content/a.md": "a"}, author="2026-01-10", committer="2026-02-15")
        self.assertEqual(
            content_dates.collect(self.repo), [("content/a.md", "2026-01-10")]
        )

    def test_files_outside_content_are_ignored(self):
        self.commit({"other/a.md": "a", "content/a.txt": "t"})
        self.assertEqual(content_dates.collect(self.repo), [])


class OutputTest(RepoTestCase):
    def test_sorted_and_exact_format(self):
        self.commit(
            {"content/b.md": "b", "content/articles/z.md": "z", "content/a.md": "a"},
            author="2026-05-06",
        )
        code, output = self.run_main()
        self.assertEqual(code, 0)
        self.assertEqual(
            output.read_bytes(),
            b"content/a.md\t2026-05-06\n"
            b"content/articles/z.md\t2026-05-06\n"
            b"content/b.md\t2026-05-06\n",
        )


class NotARepositoryTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.plain = Path(self._tmp.name) / "plain"
        (self.plain / "content").mkdir(parents=True)
        (self.plain / "content" / "a.md").write_text("a", encoding="utf-8")
        self.output = Path(self._tmp.name) / "out" / "dates.tsv"
        # Stop git from finding a repository in a parent folder.
        patcher = mock.patch.dict(
            os.environ, {"GIT_CEILING_DIRECTORIES": self._tmp.name}
        )
        patcher.start()
        self.addCleanup(patcher.stop)

    def run_main(self):
        return content_dates.main(
            ["--repo", str(self.plain), "--output", str(self.output)]
        )

    def test_in_ci_it_fails(self):
        with mock.patch.dict(os.environ, {"CI": "true"}):
            self.assertNotEqual(self.run_main(), 0)
        self.assertFalse(self.output.exists())

    def test_outside_ci_it_warns_and_writes_an_empty_file(self):
        env = {k: v for k, v in os.environ.items() if k != "CI"}
        with (
            mock.patch.dict(os.environ, env, clear=True),
            mock.patch("sys.stderr") as stderr,
        ):
            self.assertEqual(self.run_main(), 0)
        self.assertEqual(self.output.read_text(encoding="utf-8"), "")
        written = "".join(c.args[0] for c in stderr.write.call_args_list)
        self.assertIn("warning", written)

    def test_empty_ci_variable_counts_as_unset(self):
        with mock.patch.dict(os.environ, {"CI": ""}):
            self.assertEqual(self.run_main(), 0)


if __name__ == "__main__":
    unittest.main()
