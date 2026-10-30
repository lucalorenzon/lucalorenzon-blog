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

## 2026-10-30 08:18

- **Commit:** `815db4b` with uncommitted changes
- **Orca:** 50.3
- **Firefox:** Mozilla Firefox 158.0.1
- **Transcript:** first reference (saved)
- **Answers:**

1. Is "Lvk@73r" still read "elle vu cappa chiocciola settantatré erre"? **yes**
2. Is "Home Page" still pronounced as understandably as on 2026-10-30? **yes**
3. Does the Italian text ("Sito in costruzione", "© Luca Lorenzon 2026") sound natural? **yes**
4. Anything else odd? **no answer**
