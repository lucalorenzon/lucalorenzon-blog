#!/usr/bin/env python3
"""On-demand screen reader check in a Docker container: Orca 51 reads the built
site in Firefox, the transcript is compared with the reference, the run is
recorded.

Run from anywhere (it needs Docker; it may ask for the sudo password):

    python3 scripts/screen-reader/check.py

Why a container: the version of Orca and of the voice is then the same on every
machine, and nothing speaks or takes the keyboard. Card:
docs/wip/run-screen-reader-check-in-a-container.md

The command never commits. It leaves the files in docs/accessibility/ changed,
to be committed afterwards.

Layout: small pure functions first (log parsing, comparison, record formatting),
which the unit tests import; then the functions that start processes; then main().
"""

import difflib
import html
import os
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent.parent
ACCESSIBILITY_DIR = REPO_ROOT / "docs" / "accessibility"
REFERENCE_PATH = ACCESSIBILITY_DIR / "screen-reader-transcript.txt"
RECORD_PATH = ACCESSIBILITY_DIR / "screen-reader-checks.md"
SITE_DIR = REPO_ROOT / "target" / "site"
IMAGE = "lucalorenzon-blog-screen-reader"

# Outcomes of the transcript comparison, as written in the record.
RESULT_FIRST = "first reference (saved)"
RESULT_SAME = "same as the reference"
RESULT_CHANGED = "changed, change accepted (reference updated)"
RESULT_REGRESSION = "regression (change rejected, reference unchanged)"
RESULT_NO_READING = "failed: no page reading found in Orca's debug file"

RECORD_HEADER = """\
# Screen reader checks

Record of the on-demand screen reader checks: one section per run, newest at
the bottom. Each run lets Orca (GNOME's screen reader) read the built site in
Firefox, and compares what it said with the reference transcript
(`screen-reader-transcript.txt`, same folder). A "Where" line tells which
command made the run: `container` (Orca 51 and Firefox ESR in a Docker
container, nothing is heard) or no such line (the older desktop command, which
also noted the user's answers on what only an ear can judge).

Why: the build checks what the page gives a screen reader, not how it sounds.
The page is declared Italian but its title "Home Page" is English, and the
logo's name is spelled out letter by letter; a regression there changes what
Orca says. Cards: [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md),
[run it in a container](../wip/run-screen-reader-check-in-a-container.md)

How to run it, from the repository root:

    python3 scripts/screen-reader/check.py        # container, needs Docker
    python3 scripts/screen_reader_check.py        # desktop, Orca 50, to be removed

The commands never commit: commit the changed files here afterwards.
"""


# ---------------------------------------------------------------------------
# Pure functions: log parsing, comparison, record formatting
# ---------------------------------------------------------------------------

# What Orca sends to the voice: SSML (a small XML) with a <mark> per word.
_SPOKEN_RE = re.compile(r"SPEECH DISPATCHER: Speaking '(<speak>.*</speak>)'\s*$")
# Orca 51 prints the voice settings on the line after each such utterance. The
# "ORCA ... language X" part is the language asked for: "it" for content the
# page declares Italian, empty ("language,") for the default voice. The "SD"
# line below it is not used: speech-dispatcher keeps the last language set, so
# it says "it" even for English UI words after an Italian item.
_LANGUAGE_RE = re.compile(r"^\s+ORCA rate .*?\blanguage(?: ([^\s,]+))?,")
# Orca logs this when "say all" (the page reading) hands an item to the voice.
# Anything spoken without it is not part of the page reading: window and title
# announcements, "Browse mode", key echo.
_SAY_ALL_RE = re.compile(r"SAY ALL PRESENTER: Speaking \(contents\)")
# Orca 51 has no "Finished loading" message: the reading starts with the
# landmark summary, which comes before say-all, so the marker cannot see it.
_SUMMARY_RE = re.compile(r"^Page has \d+ landmarks?\b")

# Language of an utterance spoken with the default voice (no language asked).
SYSTEM_LANGUAGE = "system"


def strip_ssml(ssml: str) -> str:
    """Plain text of what was sent to the voice: tags removed, entities decoded."""
    text = re.sub(r"<[^>]*>", "", ssml)
    return re.sub(r"\s+", " ", html.unescape(text)).strip()


def parse_utterances(log_text: str) -> list[tuple[str, str, bool]]:
    """Every utterance sent to the voice, in order, as
    (language, text, from_say_all). `from_say_all` is True when Orca handed an
    item of the page reading to the voice since the previous utterance."""
    utterances: list[tuple[str, str, bool]] = []
    say_all_seen = False
    awaiting_language = False
    for line in log_text.splitlines():
        if awaiting_language:
            awaiting_language = False
            match = _LANGUAGE_RE.match(line)
            if match:
                _, text, flag = utterances[-1]
                utterances[-1] = (match.group(1) or SYSTEM_LANGUAGE, text, flag)
                continue
        if _SAY_ALL_RE.search(line):
            say_all_seen = True
            continue
        match = _SPOKEN_RE.search(line)
        if match:
            utterances.append((SYSTEM_LANGUAGE, strip_ssml(match.group(1)), say_all_seen))
            say_all_seen = False
            awaiting_language = True
    return utterances


def extract_transcript(log_text: str) -> list[tuple[str, str]]:
    """The page reading as (language, text) pairs, or [] when none is found.

    It starts at the first landmark summary ("Page has N landmarks...") and keeps
    every following utterance that came from say-all. It ends at the first
    utterance after that which did not (e.g. "Screen reader off.")."""
    utterances = parse_utterances(log_text)
    start = next((i for i, (_, text, _) in enumerate(utterances)
                  if _SUMMARY_RE.match(text)), None)
    if start is None:
        return []
    transcript = [utterances[start][:2]]
    for language, text, from_say_all in utterances[start + 1:]:
        if not from_say_all:
            break
        transcript.append((language, text))
    # A summary alone is not a reading of the page.
    return transcript if len(transcript) > 1 else []


def format_transcript(transcript: list[tuple[str, str]]) -> str:
    """One line per utterance: <language>TAB<text>."""
    return "".join(f"{language}\t{text}\n" for language, text in transcript)


def diff_transcripts(reference: str, current: str) -> str:
    """Unified diff of the reference against this run; empty when equal."""
    return "\n".join(difflib.unified_diff(
        reference.splitlines(), current.splitlines(),
        "reference", "this run", lineterm=""))


def parse_versions(text: str) -> list[tuple[str, str]]:
    """The container's versions.txt: one `name<TAB>version` per line."""
    versions = []
    for line in text.splitlines():
        name, separator, version = line.partition("\t")
        if separator and name.strip():
            versions.append((name.strip(), version.strip()))
    return versions


def docker_command(docker_found: bool, docker_info_works: bool,
                   sudo_found: bool) -> list[str] | None:
    """The command prefix for docker: plain when the user may talk to the
    daemon, `sudo docker` otherwise (only docker commands get sudo), None when
    neither is possible."""
    if not docker_found:
        return None
    if docker_info_works:
        return ["docker"]
    return ["sudo", "docker"] if sudo_found else None


def format_record(when: str, commit: str, dirty: bool,
                  versions: list[tuple[str, str]], result: str) -> str:
    """One `## <date and time>` section of the record."""
    commit_text = f"`{commit}`" + (" with uncommitted changes" if dirty else "")
    version_lines = "".join(f"- **{name}:** {version}\n" for name, version in versions)
    return (f"## {when}\n\n"
            f"- **Commit:** {commit_text}\n"
            f"- **Where:** container\n"
            f"{version_lines}"
            f"- **Transcript:** {result}\n")


# ---------------------------------------------------------------------------
# Processes and files: everything below touches the machine
# ---------------------------------------------------------------------------

def fail(message: str) -> None:
    print(f"error: {message}", file=sys.stderr)
    sys.exit(1)


def find_docker() -> list[str]:
    """Docker command prefix, or stop with a clear message."""
    found = shutil.which("docker") is not None
    if not found:
        fail("'docker' is not installed or not in PATH; this check runs Orca in a container")
    works = subprocess.run(["docker", "info"], capture_output=True,
                           check=False).returncode == 0
    command = docker_command(found, works, shutil.which("sudo") is not None)
    if command is None:
        fail("docker is not usable by this user and 'sudo' is not available")
    if command[0] == "sudo":
        print("Docker needs administrator rights here: sudo may ask for your password.")
    return command


def run_text(command: list[str], timeout: float = 20) -> str:
    result = subprocess.run(command, capture_output=True, text=True, timeout=timeout,
                            check=False)
    return (result.stdout or "").strip()


def git_state() -> tuple[str, bool]:
    commit = run_text(["git", "-C", str(REPO_ROOT), "rev-parse", "--short", "HEAD"])
    dirty = bool(run_text(["git", "-C", str(REPO_ROOT), "status", "--porcelain"]))
    return commit or "unknown", dirty


def ask_yes_no(question: str) -> bool:
    while True:
        answer = input(f"{question} [y/n] ").strip().lower()
        if answer in ("y", "yes"):
            return True
        if answer in ("n", "no"):
            return False


def write_creating_folder(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def compare_with_reference(transcript_text: str) -> str:
    """Show the outcome, ask where the user's decision is needed, update the
    reference if accepted. Returns one of the RESULT_* strings."""
    if not REFERENCE_PATH.exists():
        print("\nNo reference transcript yet: this one becomes the reference:\n")
        print(transcript_text)
        write_creating_folder(REFERENCE_PATH, transcript_text)
        return RESULT_FIRST
    reference = REFERENCE_PATH.read_text(encoding="utf-8")
    diff = diff_transcripts(reference, transcript_text)
    if not diff:
        print("\nThe transcript is the same as the reference.")
        return RESULT_SAME
    print("\nThe transcript differs from the reference:\n")
    print(diff)
    if ask_yes_no("\nIs the change intended?"):
        write_creating_folder(REFERENCE_PATH, transcript_text)
        return RESULT_CHANGED
    print("Recorded as a regression; the reference is unchanged.")
    return RESULT_REGRESSION


def append_record(section: str) -> None:
    if not RECORD_PATH.exists():
        write_creating_folder(RECORD_PATH, RECORD_HEADER)
    with RECORD_PATH.open("a", encoding="utf-8") as record:
        record.write("\n" + section)


def run_container(docker: list[str], out_dir: Path) -> None:
    """Build the image and run the reading; output is streamed to the terminal."""
    subprocess.run([*docker, "build", "-t", IMAGE, str(SCRIPT_DIR)], check=True)
    # Same uid as the host user, so what the container writes in /out is ours.
    subprocess.run([*docker, "run", "--rm",
                    "--user", f"{os.getuid()}:{os.getgid()}",
                    "-v", f"{SITE_DIR}:/site:ro",
                    "-v", f"{out_dir}:/out",
                    IMAGE], check=True)


def main() -> int:
    docker = find_docker()
    commit, dirty = git_state()  # before the run writes anything under docs/

    try:
        subprocess.run([str(REPO_ROOT / "scripts" / "build-site.sh")],
                       cwd=REPO_ROOT, check=True)
    except subprocess.CalledProcessError:
        fail("the site build failed")

    out_dir = Path(tempfile.mkdtemp(prefix="screen-reader-check-"))
    try:
        try:
            run_container(docker, out_dir)
        except subprocess.CalledProcessError as error:
            fail(f"docker failed: {' '.join(map(str, error.cmd[:3]))} ... "
                 f"exited with {error.returncode}")
        log_path = out_dir / "orca-debug.log"
        log_text = (log_path.read_text(encoding="utf-8", errors="replace")
                    if log_path.exists() else "")
        versions_path = out_dir / "versions.txt"
        versions = parse_versions(versions_path.read_text(encoding="utf-8")
                                  if versions_path.exists() else "")
        transcript = extract_transcript(log_text)
        when = datetime.now().strftime("%Y-%m-%d %H:%M")
        if not transcript:
            print("\nNo page reading was found in Orca's debug file: the run failed "
                  "and nothing is compared. The container's log above shows why.")
            result = RESULT_NO_READING
        else:
            result = compare_with_reference(format_transcript(transcript))
    except (KeyboardInterrupt, EOFError):
        print("\nInterrupted: nothing recorded.")
        return 130
    finally:
        shutil.rmtree(out_dir, ignore_errors=True)

    append_record(format_record(when, commit, dirty, versions, result))
    print("\nNothing was committed. Files changed, to be committed:")
    print(run_text(["git", "status", "--short", "--", "docs/accessibility"])
          or "  (none)")
    return 1 if result in (RESULT_NO_READING, RESULT_REGRESSION) else 0


if __name__ == "__main__":
    sys.exit(main())
