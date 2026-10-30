#!/usr/bin/env python3
"""On-demand screen reader check: let Orca read the built site in Firefox,
compare what it said with the reference transcript, and record the run.

Run from the repository root:

    python3 scripts/screen_reader_check.py

Why it exists: the build can check what the page gives a screen reader (axe,
accessibility tree) but not how it sounds. Card:
docs/done/2026-10-30_run-screen-reader-check-on-demand.md

The command never commits. It leaves the files in docs/accessibility/ changed,
to be committed afterwards.

Layout of this file: small pure functions first (log parsing, transcript
comparison, record formatting), which the unit tests import; then the functions
that start and stop processes; then main().
"""

import difflib
import html
import os
import re
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import time
from collections.abc import Sequence
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
ACCESSIBILITY_DIR = REPO_ROOT / "docs" / "accessibility"
REFERENCE_PATH = ACCESSIBILITY_DIR / "screen-reader-transcript.txt"
RECORD_PATH = ACCESSIBILITY_DIR / "screen-reader-checks.md"

# Outcomes of the transcript comparison, as written in the record.
RESULT_FIRST = "first reference (saved)"
RESULT_FIRST_NOT_SAVED = "no reference yet (transcript not saved)"
RESULT_SAME = "same as the reference"
RESULT_CHANGED = "changed, change accepted (reference updated)"
RESULT_REGRESSION = "regression (change rejected, reference unchanged)"
RESULT_NO_READING = "failed: no page reading found in Orca's debug file"

# The questions only an ear can answer: the same text can sound different after
# an update of Orca or of the voice. Wording is faithful to the card.
QUESTIONS = [
    ('Is "Lvk@73r" still read "elle vu cappa chiocciola settantatré erre"?', True),
    ('Is "Home Page" still pronounced as understandably as on 2026-10-30?', True),
    (('Does the Italian text ("Sito in costruzione", "© Luca Lorenzon 2026") '
      "sound natural?"), True),
    ("Anything else odd?", False),  # free note only, no yes/no
]

RECORD_HEADER = """\
# Screen reader checks

Record of the on-demand screen reader checks: one section per run, newest at
the bottom. Each run lets Orca (GNOME's screen reader) read the built site in
Firefox, compares what it said with the reference transcript
(`screen-reader-transcript.txt`, same folder), and notes the user's answers on
what only an ear can judge, such as how "Home Page" and "Lvk@73r" sound.

Why: the build checks what the page gives a screen reader, not how it sounds.
The page is declared Italian but its title "Home Page" is English, and the
logo's name is spelled out letter by letter; a regression there is only
audible. Card: [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)

How to run it, from the repository root, at the computer (Orca speaks and takes
the keyboard, so do not run it while using your own Orca):

    python3 scripts/screen_reader_check.py

The command never commits: commit the changed files here afterwards.
"""


# ---------------------------------------------------------------------------
# Pure functions: log parsing, comparison, record formatting
# ---------------------------------------------------------------------------

# What Orca sends to the voice. SSML (a small XML) with a <mark> per word.
_SPOKEN_RE = re.compile(r"SPEECH DISPATCHER: Speaking '(<speak>.*</speak>)'\s*$")
# Orca prints the voice settings on the line after each such utterance; the
# language there is the one the voice was really asked for ("it" for content
# the page declares Italian, the user's default voice, e.g. "en", otherwise).
_LANGUAGE_RE = re.compile(r"^\s+ORCA rate .*?\blanguage ([^\s,]+)")
# Orca logs this when "say all" (the page reading) hands an item to the voice.
# Anything spoken without it is not part of the page reading: key echo of the
# user's typing, Firefox dialogs such as "Translate", window announcements.
_SAY_ALL_RE = re.compile(r"SAY ALL PRESENTER: Speaking \(contents\)")
# Orca announces the end of the page load with this phrase; the automatic
# reading of the page follows. Everything before it is Firefox's own startup.
_LOADED_RE = re.compile(r"^Finished loading .+\.$")
# The landmark summary comes right after the load message, before say-all
# starts, so the say-all marker cannot recognise it.
_SUMMARY_RE = re.compile(r"^Page has \d+ landmarks?\b")

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
                utterances[-1] = (match.group(1), text, flag)
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

    It starts at the first "Finished loading <title>." and keeps the landmark
    summary plus every utterance that came from say-all. It ends at the first
    utterance after that which did not (e.g. the user's key echo)."""
    utterances = parse_utterances(log_text)
    start = next((i for i, (_, text, _) in enumerate(utterances)
                  if _LOADED_RE.match(text)), None)
    if start is None:
        return []
    transcript = [utterances[start][:2]]
    for language, text, from_say_all in utterances[start + 1:]:
        is_summary = len(transcript) == 1 and _SUMMARY_RE.match(text)
        if not (from_say_all or is_summary):
            break
        transcript.append((language, text))
    # A load message alone is not a reading of the page.
    return transcript if len(transcript) > 1 else []


def format_transcript(transcript: list[tuple[str, str]]) -> str:
    """One line per utterance: <language>TAB<text>."""
    return "".join(f"{language}\t{text}\n" for language, text in transcript)


def diff_transcripts(reference: str, current: str) -> str:
    """Unified diff of the reference against this run; empty when equal."""
    return "\n".join(difflib.unified_diff(
        reference.splitlines(), current.splitlines(),
        "reference", "this run", lineterm=""))


def format_answers(answers: Sequence[tuple[str, str, str]]) -> str:
    """Markdown list of (question, answer, note); answer is 'yes', 'no' or ''
    for the free-note question."""
    lines = []
    for number, (question, answer, note) in enumerate(answers, start=1):
        parts = [part for part in (answer, note) if part]
        reply = " — ".join(parts) if parts else "no answer"
        lines.append(f"{number}. {question} **{reply}**")
    return "\n".join(lines)


def format_record(when: str, commit: str, dirty: bool, orca_version: str,
                  firefox_version: str, result: str,
                  answers: Sequence[tuple[str, str, str]]) -> str:
    """One `## <date and time>` section of the record."""
    commit_text = f"`{commit}`" + (" with uncommitted changes" if dirty else "")
    answers_text = (format_answers(answers) if answers
                    else "not asked (the run failed)")
    return (f"## {when}\n\n"
            f"- **Commit:** {commit_text}\n"
            f"- **Orca:** {orca_version}\n"
            f"- **Firefox:** {firefox_version}\n"
            f"- **Transcript:** {result}\n"
            f"- **Answers:**\n\n{answers_text}\n")


# ---------------------------------------------------------------------------
# Processes: everything below touches the machine
# ---------------------------------------------------------------------------

ORCA_DEST = ["--session", "--dest", "org.gnome.Orca.Service",
             "--object-path", "/org/gnome/Orca/Service"]


def fail(message: str) -> None:
    print(f"error: {message}", file=sys.stderr)
    sys.exit(1)


def check_preconditions() -> None:
    """Refuse to run where the check would act on the wrong thing."""
    if not (os.environ.get("WAYLAND_DISPLAY") or os.environ.get("DISPLAY")):
        fail("no graphical session (neither WAYLAND_DISPLAY nor DISPLAY is set)")
    for tool in ("orca", "firefox", "gdbus", "pgrep"):
        if shutil.which(tool) is None:
            fail(f"'{tool}' is not installed or not in PATH")
    # The user's own Orca must never be replaced or closed by this command.
    if subprocess.run(["pgrep", "-x", "orca"], capture_output=True,
                      check=False).returncode == 0:
        fail("Orca is already running. Quit it first: this command starts its "
             "own Orca and closes it at the end, and must not touch yours.")


def run_text(command: list[str], timeout: float = 20) -> str:
    result = subprocess.run(command, capture_output=True, text=True, timeout=timeout,
        check=False)
    return (result.stdout or "").strip()


def git_state() -> tuple[str, bool]:
    commit = run_text(["git", "-C", str(REPO_ROOT), "rev-parse", "--short", "HEAD"])
    dirty = bool(run_text(["git", "-C", str(REPO_ROOT), "status", "--porcelain"]))
    return commit or "unknown", dirty


def free_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return sock.getsockname()[1]


def wait_for(condition, timeout: float, what: str) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if condition():
            return
        time.sleep(0.3)
    raise RuntimeError(f"timed out waiting for {what}")


def port_open(port: int) -> bool:
    with socket.socket() as sock:
        sock.settimeout(0.5)
        return sock.connect_ex(("127.0.0.1", port)) == 0


def orca_version_or_none() -> str | None:
    """Orca's version if its D-Bus service answers, else None."""
    try:
        result = subprocess.run(
            ["gdbus", "call", *ORCA_DEST, "--method",
             "org.gnome.Orca.Service.GetVersion"],
            capture_output=True, text=True, timeout=5, check=False)
    except subprocess.TimeoutExpired:
        return None
    match = re.search(r"'([^']+)'", result.stdout)
    return match.group(1) if result.returncode == 0 and match else None


def _symlink_chain(path: str) -> list[str]:
    """The path and every target it points to in turn (/snap/bin/firefox is
    itself a link to /usr/bin/snap, so only the intermediate hops show /snap/)."""
    chain = [path]
    for _ in range(40):
        if not os.path.islink(chain[-1]):
            break
        target = os.readlink(chain[-1])
        chain.append(os.path.join(os.path.dirname(chain[-1]), target))
    return chain


def _is_snap_launcher(path: str) -> bool:
    """Ubuntu's /usr/bin/firefox is a small shell script that runs the snap."""
    try:
        with open(path, "rb") as launcher:
            head = launcher.read(4096)
        if b"\0" in head:
            return False
        return "/snap/bin/firefox" in head.decode("utf-8")
    except (OSError, UnicodeDecodeError):
        return False


def firefox_is_snap(firefox: str | None = None) -> bool:
    """A snap Firefox cannot read /tmp, so its profile must live in its own folder.

    `firefox` is the launcher path; by default the one found on PATH."""
    path = firefox if firefox is not None else shutil.which("firefox") or ""
    if not path:
        return False
    if any("/snap/" in hop for hop in _symlink_chain(path)):
        return True
    return "/snap/" in os.path.realpath(path) or _is_snap_launcher(path)


# Each preference removes one thing that would open a tab or a popup and
# distract Orca from reading the page we asked for.
FIREFOX_PREFS = {
    # No "default browser?" question at start.
    "browser.shell.checkDefaultBrowser": "false",
    # No welcome / first-run pages.
    "browser.aboutwelcome.enabled": "false",
    "trailhead.firstrun.didSeeAboutWelcome": "true",
    "startup.homepage_welcome_url": '""',
    "startup.homepage_welcome_url.additional": '""',
    # No "what's new" page after an update ("ignore" is Firefox's own value).
    "browser.startup.homepage_override.mstone": '"ignore"',
    "browser.messaging-system.whatsNewPanel.enabled": "false",
    # No privacy notice tab (data reporting policy treated as already shown).
    "datareporting.policy.dataSubmissionPolicyBypassNotification": "true",
    "toolkit.telemetry.reportingpolicy.firstRun": "false",
    # No update check or prompt during the run.
    "app.update.auto": "false",
    "app.update.checkInstallTime": "false",
    # No "restore previous session" page after a killed run.
    "browser.sessionstore.resume_from_crash": "false",
    "browser.disableResetPrompt": "true",
    # No "Translate this page?" popup: the page is Italian, Firefox would offer
    # to translate it and Orca would read that dialog instead.
    "browser.translations.automaticallyPopup": "false",
    "browser.translations.enable": "false",
    # Make sure accessibility support is not forced off.
    "accessibility.force_disabled": "0",
}


def write_user_js(profile: Path) -> None:
    lines = ["// Written by scripts/screen_reader_check.py; deleted after the run."]
    lines += [f'user_pref("{name}", {value});' for name, value in FIREFOX_PREFS.items()]
    (profile / "user.js").write_text("\n".join(lines) + "\n", encoding="utf-8")


def make_profile() -> Path:
    if firefox_is_snap():
        parent = Path.home() / "snap" / "firefox" / "common"
        parent.mkdir(parents=True, exist_ok=True)
        profile = Path(tempfile.mkdtemp(prefix="screen-reader-check-", dir=parent))
    else:
        profile = Path(tempfile.mkdtemp(prefix="screen-reader-check-"))
    write_user_js(profile)
    return profile


def stop_group(proc: subprocess.Popen | None, name: str) -> None:
    """Stop a process we started and its process group (it was started in its
    own session, so this never touches anything else)."""
    if proc is None or proc.poll() is not None:
        return
    try:
        os.killpg(proc.pid, signal.SIGTERM)
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        os.killpg(proc.pid, signal.SIGKILL)
        proc.wait(timeout=5)
    except ProcessLookupError:
        pass
    except OSError as error:  # cleanup must not hide the original error
        print(f"warning: could not stop {name}: {error}", file=sys.stderr)


def quit_orca(orca: subprocess.Popen | None) -> None:
    """Ask the Orca we started to quit through D-Bus; fall back to terminating
    that process only."""
    if orca is None or orca.poll() is not None:
        return
    try:
        subprocess.run(["gdbus", "call", *ORCA_DEST, "--method",
                        "org.gnome.Orca.Service.Quit"],
                       capture_output=True, timeout=10, check=False)
        orca.wait(timeout=10)
    except (OSError, subprocess.SubprocessError):
        stop_group(orca, "Orca")


@dataclass
class Handles:
    """What a session started. Processes and profile are None until started,
    so cleanup knows what exists and a failure halfway still closes it."""
    log: Path
    server: subprocess.Popen | None = None
    orca: subprocess.Popen | None = None
    firefox: subprocess.Popen | None = None
    profile: Path | None = None
    orca_version: str = "unknown"
    firefox_version: str = "unknown"


def start_session(workdir: Path) -> Handles:
    """Start server, Orca, Firefox. Returns the handles for cleanup."""
    handles = Handles(log=workdir / "orca-debug.log")
    # Handles are filled in as they start, so a failure halfway still cleans up.
    try:
        subprocess.run([str(REPO_ROOT / "scripts" / "build-site.sh")],
                       cwd=REPO_ROOT, check=True)
        port = free_port()
        handles.server = subprocess.Popen(
            [sys.executable, "-m", "http.server", str(port), "--bind", "127.0.0.1",
             "--directory", str(REPO_ROOT / "target" / "site")],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            start_new_session=True)
        wait_for(lambda: port_open(port), 10, "the local web server")

        orca = subprocess.Popen(
            ["orca", "--debug-file", str(handles.log)],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            start_new_session=True)
        handles.orca = orca
        version: list[str | None] = [None]

        def orca_answers() -> bool:
            if orca.poll() is not None:
                raise RuntimeError("Orca exited right after starting")
            version[0] = orca_version_or_none()
            return version[0] is not None
        wait_for(orca_answers, 15, "Orca's D-Bus service")
        handles.orca_version = version[0] or "unknown"

        handles.firefox_version = run_text(["firefox", "--version"]) or "unknown"
        handles.profile = make_profile()
        # Firefox turns accessibility on only if a screen reader is running when
        # it starts, hence after Orca; GNOME_ACCESSIBILITY=1 makes it certain.
        # --new-instance --no-remote keeps it apart from the user's own Firefox.
        env = dict(os.environ, GNOME_ACCESSIBILITY="1")
        handles.firefox = subprocess.Popen(
            ["firefox", "--new-instance", "--no-remote",
             "--profile", str(handles.profile), f"http://127.0.0.1:{port}/"],
            env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            start_new_session=True)
    except BaseException:
        cleanup(handles)
        raise
    return handles


def cleanup(handles: Handles) -> None:
    """Close everything we started. Orca first and alone: its debug file is
    written in blocks and is complete only after it has exited."""
    quit_orca(handles.orca)
    stop_group(handles.firefox, "Firefox")
    stop_group(handles.server, "the web server")
    if handles.profile:
        shutil.rmtree(handles.profile, ignore_errors=True)


def ask_yes_no(question: str) -> bool:
    while True:
        answer = input(f"{question} [y/n] ").strip().lower()
        if answer in ("y", "yes"):
            return True
        if answer in ("n", "no"):
            return False


def ask_questions() -> list[tuple[str, str, str]]:
    answers = []
    print("\nNow the questions only an ear can answer.\n")
    for question, is_yes_no in QUESTIONS:
        if is_yes_no:
            answer = "yes" if ask_yes_no(question) else "no"
            note = input("  Note (optional, Enter to skip): ").strip()
        else:
            answer = ""
            note = input(f"{question} (free note, Enter for none): ").strip()
        answers.append((question, answer, note))
    return answers


def write_creating_folder(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text, encoding="utf-8")


def compare_with_reference(transcript_text: str) -> str:
    """Show the outcome, ask where the user's decision is needed, update the
    reference if accepted. Returns one of the RESULT_* strings."""
    if not REFERENCE_PATH.exists():
        print("\nTranscript of this run:\n")
        print(transcript_text)
        if ask_yes_no("There is no reference transcript yet. Save this one as the reference?"):
            write_creating_folder(REFERENCE_PATH, transcript_text)
            return RESULT_FIRST
        return RESULT_FIRST_NOT_SAVED
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


def main() -> int:
    os.chdir(REPO_ROOT)
    check_preconditions()
    commit, dirty = git_state()  # before the run writes anything under docs/

    workdir = Path(tempfile.mkdtemp(prefix="screen-reader-check-log-"))
    handles: Handles | None = None
    try:
        handles = start_session(workdir)
        print("\nFirefox is opening the page and Orca should start reading it.")
        print("Listen to the page being read. If the reading does not start, "
              "click inside the page in the new Firefox window.")
        input("Press Enter here when Orca has finished reading... ")
    except (KeyboardInterrupt, EOFError):
        print("\nInterrupted: closing everything.")
        return 130
    finally:
        if handles is not None:
            cleanup(handles)
        else:
            shutil.rmtree(workdir, ignore_errors=True)

    if handles is None:  # not reachable: only an interruption leaves it unset
        return 1

    # Orca has exited, so its debug file is complete.
    log_path = handles.log
    log_text = (log_path.read_text(encoding="utf-8", errors="replace")
                if log_path.exists() else "")
    shutil.rmtree(workdir, ignore_errors=True)

    transcript = extract_transcript(log_text)
    when = datetime.now().strftime("%Y-%m-%d %H:%M")
    if not transcript:
        print("\nNo page reading was found in Orca's debug file: the run failed "
              "and nothing is compared. Did the page get the focus?")
        result, answers = RESULT_NO_READING, []
    else:
        result = compare_with_reference(format_transcript(transcript))
        answers = ask_questions()

    append_record(format_record(when, commit, dirty, handles.orca_version,
                                handles.firefox_version, result, answers))

    print("\nNothing was committed. Files changed, to be committed:")
    print(run_text(["git", "status", "--short", "--", "docs/accessibility"])
          or "  (none)")
    return 1 if result in (RESULT_NO_READING, RESULT_REGRESSION) else 0


if __name__ == "__main__":
    sys.exit(main())
