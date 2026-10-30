# Run the screen reader check in a container

- **Opened:** 2026-10-30, asked by the user while the card
  [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)
  was being worked on.
- **Status:** done on 2026-10-30.

## Why

The on-demand screen reader check works only on the user's machine, and
is fragile there. It depends on GNOME on Wayland, Orca 50 and its D-Bus
remote control, the Firefox snap (with its profile in the snap's own
folder), a click to give the window the focus, the user's default voice
(English, which ends up in the transcript as the language of Orca's own
words), and the format of Orca's debug file. On another machine the
transcript would differ with the same page, so the comparison means
nothing there.

The user wants the check to work on any computer. The idea was a Docker
image with a virtual screen, the accessibility bus, Orca, Firefox and a
fixed synthetic voice, so the transcript is the same on any machine with
Docker.

## Answers in brief

- **Does Orca run without a real desktop session?** Yes, on a virtual
  screen, once it is given a sound output that drops the sound (trial
  below). The command is `scripts/screen-reader/check.py`.
- **Can the audio be written to a file?** Yes: the container records the
  voice into `target/screen-reader/reading.wav`. The user no longer
  answers questions for the ear on every run: listening is needed once,
  when a name or a text is chosen ("What the check is for", below).
- **Can the transcript comparison run in the build?** It could, but it
  does not: the build checks the accessibility tree and will check the
  declared languages; Orca in the container runs on demand.
- **What happens to the local command?** Removed by this card, once the
  container command worked.

## Trial: does Orca read the page in a container? Yes

Agreed with the user on 2026-10-30: one hour at most, and the trial
succeeds if Orca reads the site's page inside the container. Voice,
audio file and the build comparison stay out of it: they are the other
questions above. Done in three runs and about 17 minutes (08:37 to
08:47, paused by the user, then 16:32 to 16:39).

Set-up, chosen for the trial only: Debian trixie, where Firefox is an
ordinary package (on Ubuntu it is a snap, which does not work in a
container); a virtual screen (Xvfb) instead of GNOME; a session bus with
the accessibility bus; the openbox window manager, so Firefox gets the
focus without a click; PulseAudio with a null sink; Orca started before
Firefox, as in the local command. The site is built on the host and
mounted read-only. Versions: Orca 48.1, Firefox 153.6.0esr,
speech-dispatcher 0.12.0. The trial's files are in this card's git
history; what became of them is in `scripts/screen-reader/`.

### Runs

1. **Orca announced the page, then went silent.** It said the window
   title and "Page has 3 landmarks, 1 heading.", nothing more, and did
   not quit on SIGTERM.
2. **Diagnosis: Orca was stuck on the voice.** With Python's
   faulthandler on, SIGABRT showed Orca's main thread waiting for
   speech-dispatcher to answer, and speech-dispatcher itself waiting:
   the container has no sound device. Rejected explanation: that Orca
   went on reading and only the end of its debug file was lost.
3. **With a null audio sink, Orca read the page whole.** PulseAudio
   started before Orca, with a null sink as default output, which takes
   the sound and drops it.

### Result

The reading has the same structure as the reference taken on the
user's desktop (`docs/accessibility/screen-reader-transcript.txt`):
the landmark summary, banner, the logo, image, "Home Page", heading 1,
leaving banner, main content, "Sito in costruzione", leaving main
content, information, "© Luca Lorenzon 2026". Orca asks for an Italian
voice for the page's Italian text. The differences come from Orca 48
against Orca 50 on the desktop:

- no "Finished loading" announcement before the reading;
- the logo is sent to the voice as "Lvk@73r", not "Lvk @ 73r";
- the debug file has another format (`Speaking '...' as string`, not
  SSML), so the local command's parser does not read it as it is.

Also observed: Orca 48 does not quit on SIGTERM even when idle, and has
no D-Bus service to quit it. This time the debug file held the whole
reading anyway, but a reliable way to stop Orca and flush its file is
still to be found.

## Which reference, which Orca

### Which reading is the reference? The container's (decided by the user)

The reference is the reading produced in the container, no longer the
one taken on the user's desktop. Rejected: making the container match
the desktop exactly (Orca 50.3), which would tie the image to the
user's own system and its updates.

Open consequence: the local command compares the desktop reading with
`docs/accessibility/screen-reader-transcript.txt`; once the reference
comes from the container, the two readings differ by Orca version
alone. Resolved by removing the local command (below).

### Which Orca? 51.1 from trixie-backports (decided by the user)

Debian has no Orca 50 for trixie: trixie has 48.1, trixie-backports
(an official Debian repository) has 51.1, as do testing and unstable
(checked on 2026-10-30 in the Debian archive). The user chose 51.1 from
the backports, so the image stays on Debian stable. Rejected: Ubuntu
for the exact 50 (Firefox there is a snap; it would need Mozilla's own
repository); building Orca from source (most work).

Orca 51 pulls speech-dispatcher 0.12.1 and at-spi2-core 2.62 from the
backports; the espeak-ng module of speech-dispatcher must then come
from the backports too, or apt refuses the mix (the first build
failed on that).

### Run with Orca 51.1

- Orca read the page whole and **quit cleanly on SIGTERM** ("Screen
  reader off."), which solves Orca 48's quit problem.
- The debug file is back to Orca 50's format (SSML), so the local
  command's parser should read it.
- Still no "Finished loading" announcement before the reading.
- **The logo is read as "code start", "code end", without its name**,
  where Orca 50 says "Lvk @ 73r", "image". Recorded by the user as a
  card in Now:
  [logo name not read by Orca 51](../backlog/logo-name-not-read-by-orca-51.md).
- Orca spends about seven seconds looking for a braille display that is
  not there before it goes on reading.
- Harmless warnings: the `ScreenReaderEnabled` property no longer
  exists on the accessibility bus 2.62; Firefox, built for trixie's
  libraries, warns about a caret event of the backported ones.

## What the check is for

### Does the reading's quality depend on the site or on the visitor's software? Both, on different things

Asked by the user. The site is responsible for what the page gives the
screen reader: structure (landmarks, headings and their levels), names
(such as the logo's), the language of each part, reading order, and
what is hidden as decoration. The visitor's software is responsible for
how that sounds: voice, pronunciation, speed, how verbose it is and how
it presents some elements, and the visitor's own settings. In between,
WCAG asks the site to use techniques the screen readers in use really
support.

So what depends on the site shows in a written transcript and can be
compared without listening; listening judges mostly the voice, which
the site does not control. Listening still helps once, when a name or a
text is chosen (does "Lvk@73r" read aloud make sense?), not as a check
repeated on every build. This drops the reason given by the card
[run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)
for listening on every run: that the same text may sound different
after an update of Orca or of the voice.

### Where does each part of the check run? Languages in the build, Orca on demand (decided by the user)

The build already compares the accessibility tree (roles, names,
heading levels, order) in `end2end/tests/accessibility.spec.ts`: that
covers most of what depends on the site. It does not see the language
of each part; a small check of the declared languages goes into the
build: card
[check the declared languages in the build](../backlog/check-declared-languages-in-the-build.md).

Orca in the container stays a tool run on demand, on any computer with
Docker and without listening, when the screen reader or the markup
changes. Its value is the case the tree cannot see: markup that is
correct by the standard but that a real screen reader presents
differently, as the logo with Orca 51.

Rejected: Orca in the pipeline. With Orca's version pinned, as it must
be for a stable comparison, it would repeat the same reading on every
build, redundant with the tree for changes of the site, at the cost of
about a minute per build and of updating Orca by hand. The logo case was
found by changing Orca's version, not by repeating a fixed one.

### What happens to the local command? Removed by this card (decided by the user)

The local command (`scripts/screen_reader_check.py`, with its unit
tests and log sample) is removed once the command in the container
works, so the site is never left without a screen reader check. Its
records in `docs/accessibility/` stay as history; the reference
transcript is then taken from the container. Rejected: keeping it for
occasional listening, now covered by the audio file below.

### How is the reading heard when needed? The container writes the audio to a file (decided by the user)

The container records what the voice says into an audio file the user
plays when a name or a text needs hearing. The null audio sink that
unblocked Orca in the trial can be recorded from, so the file should
cost little.

## Slices

Agreed with the user: three slices, one decision and one commit each.
The local command goes last, so the site is never without a screen
reader check.

1. The container command in the repository, Orca pinned, transcript
   compared with the reference, no audio. Done (below).
2. The audio written to a file, and the end of the reading found from
   its silence. Done (below).
3. The local command removed, with its tests, README and repository map
   updated. Done (below).

### Slice 1: the container command

**Where do the files go? `scripts/screen-reader/` (decided by the user).**
Every folder outside `docs/` starts the build, so only order counted:
the local command already lives in `scripts/`, and a subfolder keeps
everything about the check in one place. Rejected: a top-level
`screen-reader/` beside `end2end/`, one more root folder for a command
used rarely.

What it does: `python3 scripts/screen-reader/check.py` builds the site,
builds the image and runs it as the user's own uid (with `sudo docker`
only if the user may not use Docker directly), extracts the transcript
from Orca's debug file and compares it with
`docs/accessibility/screen-reader-transcript.txt`, asks whether a
change is intended, and appends the run to
`docs/accessibility/screen-reader-checks.md`. It never commits. Agreed
with the user before the code was written:

- only Orca is pinned (`51.1-1~bpo13+1`); Firefox ESR is not, since
  Debian drops old ESR versions and a pin would break the build: a
  Firefox change shows as a transcript change for the user to judge;
- Orca 51 does not say "Finished loading", so the transcript starts at
  the landmark summary;
- the default voice's language is recorded as `system`, not the
  desktop's `en`: the container has no user setting to depend on;
- the first container run replaces the desktop reference; the old one
  stays in git history.

Results:

- 23 unit tests for the pure functions, with a 6 KB sample cut from a
  real Orca 51 debug file; the local command's 23 tests still pass.
- First run: dbus-daemon refused to start for a uid with no entry in
  the image's passwd file. Fixed with `nss_wrapper`, which gives the
  programs a passwd file of the container's own at run time, without
  making the image's `/etc/passwd` writable.
- Second run: Orca read the page whole; the user accepted the change,
  and the container's reading is now the reference.
- **The end-of-reading signal did not work.** The null sink stayed
  RUNNING for the whole run, because speech-dispatcher keeps its sound
  stream open while the voice is silent, so the run ended at the
  90-second maximum instead of about 20 seconds after the reading. The
  result is right, the run only slower. Slice 2 replaces the signal
  with the silence of the recorded audio (agreed with the user).

The trial files that were kept in an appendix here are now in
`scripts/screen-reader/`; their trial versions stay in this card's git
history.

### Slice 2: the audio file and the end of the reading

**Where does the audio file go? `target/screen-reader/reading.wav`
(decided by the user).** It is overwritten on every run and never
committed: the folder is already ignored by git, and a recording changes
at every run even when the page does not, so committing it would add
about a megabyte per run for nothing the transcript does not already
record. Rejected: keeping it in `docs/accessibility/` as history.

What it does: `scripts/screen-reader/record.py` starts before Orca,
records the null sink's monitor with `parec` into a WAV file, and
measures the loudness of every half second. The reading is over when the
voice has been heard and then ten seconds have passed in silence; ninety
seconds stay the maximum. It replaces the sink-state signal of slice 1.
`check.py` copies the file out of the container's folder and prints
where it is.

- **Silence threshold:** a loudness of 100 on a scale up to 32767. The
  null sink gives exact zeros when nothing plays, and the voice is in
  the thousands, so any low value separates them. Rejected: `sox`, which
  can record and stop on silence by itself but adds a package to the
  image and shows less of what it decides in the log.
- **Quiet time stays ten seconds:** Orca is silent for about six seconds
  while it looks for a braille display, after announcing itself and
  before the reading.

Results:

- 20 unit tests for the decision logic; all 37 tests of the folder pass.
- Run on 2026-10-30: the recording ended on "reading over" ten seconds
  after the last word, 39.5 seconds in all instead of the 90 of slice 1;
  the transcript was the same as the reference.
- The user listened to the file: the whole reading is there.
- **Orca's own words are pronounced as Italian.** "Leaving banner" is
  read with the Italian voice, letter sounds and all. Orca speaks its
  own words in English (the container has no Italian locale), but
  speech-dispatcher keeps the last language it was given, Italian after
  the page's Italian text. It comes from the visitor's software, not
  from the site, and does not show in the transcript.

### Slice 3: the local command removed

`scripts/screen_reader_check.py`, its unit tests and its log sample are
removed, with the README paragraph and the repository map row about
them. The records it made stay in `docs/accessibility/screen-reader-checks.md`
as history: a section with no "Where" line comes from it. The container
command's tests still pass without it.
