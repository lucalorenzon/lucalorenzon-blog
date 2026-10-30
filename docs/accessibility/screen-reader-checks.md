# Screen reader checks

Record of the on-demand screen reader checks: one section per run, newest at
the bottom. Each run lets Orca (GNOME's screen reader) read the built site in
Firefox, and compares what it said with the reference transcript
(`screen-reader-transcript.txt`, same folder). A "Where" line tells which
command made the run: `container` (Orca 51 and Firefox ESR in a Docker
container, nothing is heard) or no such line (the older desktop command,
removed on 2026-10-30, which also noted the user's answers on what only an ear
can judge).

Why: the build checks what the page gives a screen reader, not how it sounds.
The page is declared Italian but its title "Home Page" is English, and the
logo's name is spelled out letter by letter; a regression there changes what
Orca says. Cards: [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md),
[run it in a container](../done/2026-10-30_run-screen-reader-check-in-a-container.md)

How to run it, from the repository root (it needs Docker):

    python3 scripts/screen-reader/check.py

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

## 2026-10-30 22:01

- **Commit:** `ad53d95` with uncommitted changes
- **Where:** container
- **Orca:** 51.1-1~bpo13+1
- **Firefox:** 153.6.0esr-1~deb13u1
- **speech-dispatcher:** 0.12.1-5~bpo13+1
- **Transcript:** changed, change accepted (reference updated)

## 2026-10-30 22:16

- **Commit:** `8b05511` with uncommitted changes
- **Where:** container
- **Orca:** 51.1-1~bpo13+1
- **Firefox:** 153.6.0esr-1~deb13u1
- **speech-dispatcher:** 0.12.1-5~bpo13+1
- **Transcript:** same as the reference
