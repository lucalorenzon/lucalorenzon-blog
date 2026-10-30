# Run a screen reader check on demand

- **Opened:** 2026-10-30, from the split of the card "Check
  accessibility by hand", itself split on 2026-10-29 from the card "Add
  automated accessibility tests", opened on 2026-10-28 by the user.
- **Status:** done on 2026-10-30, in commits `1d952b1` (the command)
  and `97569ec` (the first reference and run record).

## Why it was opened

The tools the build can run (axe, and the accessibility tree compared by
the card
[check the accessibility tree in the build](2026-10-30_check-accessibility-tree-in-the-build.md))
see what the page gives a screen reader, not how it sounds: for
example how the Italian voice pronounces the English title "Home Page"
(WCAG 3.1.2, language of parts, level AA), or how the logo's name
"Lvk@73r" is spelled out.

The user asked for the check by hand to start with one local command,
so it can be repeated on demand to look for regressions.

## Answers

### How can a check by hand also find regressions?

**By capturing what Orca says and comparing it with a reference, while
the user listens and answers what only an ear can judge (agreed with
the user).** A person listening judges the sound; the captured
transcript, compared with the previous run's, finds changes in what is
said. Together they cover both.

### Can Orca be driven and its speech captured? (Trial, about 35 minutes)

**Yes, on the user's machine** (Orca 50.3, Firefox 158 snap, GNOME on
Wayland):

- Orca writes every utterance to its debug file, with the voice and its
  language. Orca writes the file in blocks, so it is read after Orca
  quits.
- When a page finishes loading, Orca reads it whole on its own: no
  command is needed.
- Orca's D-Bus remote control (`org.gnome.Orca.Service`) works; it is
  used to quit Orca. `SayAll` was rejected: it reads the window in
  front, from the current position, and once read the terminal instead.
- Firefox exposes its accessibility only if a screen reader runs when
  it starts: the command starts a separate Firefox after Orca, with a
  temporary profile, and never touches the user's own Firefox. The snap
  can read only its own folder, so the profile goes there.
- On Wayland the user may have to click the window to give it the focus.

### What does the command do?

`python3 scripts/screen_reader_check.py` builds and serves the site,
starts Orca, opens the page, and when the user presses Enter closes
everything it started, also after an error. It refuses to run if Orca is
already running, so the user's own Orca is never replaced. Then:

- the transcript (each utterance with its voice language) is compared
  with `docs/accessibility/screen-reader-transcript.txt`; a change is
  shown and the user says whether it is intended: if so it becomes the
  reference, if not the run is recorded as a regression;
- four questions, yes or no with a note: is "Lvk@73r" still read "elle
  vu cappa chiocciola settantatré erre", is "Home Page" still as
  understandable, does the Italian text sound natural, anything else
  odd. The first two readings were accepted by the user on 2026-10-30;
  the questions exist because the same text can sound different after
  an update of Orca or of the voice;
- the run is appended to `docs/accessibility/screen-reader-checks.md`.
  The command never commits.

Where the records live: `docs/accessibility/` (chosen by the user), as
they are records of checks to be read as documents, and a change there
alone does not start the build. Rejected: `end2end/screen-reader/`,
where every run would start the build.

When it runs: on demand, as the user asked; no fixed rule for now.

### Limits

**It works only on the user's GNOME desktop, and is fragile there**
(noted by the user after the first runs): it depends on Orca 50, the
Firefox snap, a click for the focus, the format of Orca's debug file,
and the user's default voice, English, which appears in the transcript
as the language of Orca's own words. On another computer the
transcript would differ with the same page. The user wants a check that
works on any computer, not urgently: card
[run the screen reader check in a container](../wip/run-screen-reader-check-in-a-container.md).

The command's unit tests run locally only, not in the build.

## Results

- First runs failed: the Firefox snap was not detected behind Ubuntu's
  launcher `/usr/bin/firefox`, so its profile went where the snap cannot
  read; then the first reference could not be saved in a missing folder.
  Each was reproduced by a failing unit test, then fixed. 23 unit tests
  pass; ruff and basedpyright, as configured in the user's editor,
  report nothing.
- First successful run on 2026-10-30: Orca read the page whole, the
  transcript became the reference, and the user answered yes to the
  three questions.
