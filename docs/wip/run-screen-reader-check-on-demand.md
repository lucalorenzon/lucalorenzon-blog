# Run a screen reader check on demand

- **Opened:** 2026-10-30, from the split of the card "Check
  accessibility by hand", itself split on 2026-10-29 from the card "Add
  automated accessibility tests", opened on 2026-10-28 by the user.
- **Status:** being worked on since 2026-10-30.

## Why

The tools the build can run (axe, and the accessibility tree compared by
the card
[check the accessibility tree in the build](../done/2026-10-30_check-accessibility-tree-in-the-build.md))
see what the page gives a screen reader, not how it sounds. For
example: the page is declared Italian (`lang="it"`) but its title "Home
Page" is English, which the Italian voice probably mispronounces (WCAG
3.1.2, language of parts, level AA); and how the logo's name "Lvk@73r"
is spelled out.

On the user's machine, Orca (GNOME's screen reader) and Firefox are
already installed, the most common pair on Linux.

## Requirement from the user

The check by hand must start with one local command, so it can be
repeated on demand to look for regressions.

## Agreed direction

Decided with the user on 2026-10-30. Orca 50.3, on the user's machine,
writes every phrase it speaks to its debug file
(`SPEECH OUTPUT: '<text>'`, with the voice used), and offers a D-Bus
remote control (`org.gnome.Orca.Service`) to run its commands. So one
command can serve the site, open it in Firefox, have Orca read it while
the user listens, then compare the transcript with the previous run's
and record the user's answers on what only an ear can judge (such as
how "Home Page" and "Lvk@73r" are pronounced).

Before writing that command, a timeboxed trial (one hour at most) on
the user's machine checks what is not known yet:

- whether Firefox, Orca and the remote control work together on
  Wayland;
- which Orca command reads the whole page;
- whether the debug file tells the language of the voice for each
  phrase.

Orca acts on the user's real desktop (it speaks and takes the
keyboard), so it is started only with the user at the computer.

## Trial on the user's machine (2026-10-30)

About 35 minutes, Orca 50.3 with Firefox 158 (snap) on GNOME, Wayland,
the site served locally. Started by the agent with the user at the
computer.

- **Firefox must start after Orca.** Orca did not see the user's
  Firefox, already open: Firefox turns accessibility on only if a
  screen reader is running when it starts. A separate instance
  (`--new-instance --no-remote`, a temporary profile, with
  `GNOME_ACCESSIBILITY=1`) was seen at once, without touching the
  user's own Firefox. The snap reads only its own folders, so the
  profile went under `~/snap/firefox/common/`, deleted afterwards.
- **No command is needed to read the page.** When the page finished
  loading, Orca read it whole on its own: image "Lvk@73r", heading
  "Home Page", "Sito in costruzione", "© Luca Lorenzon 2026", after
  "Page has 3 landmarks, 1 heading."
- **The transcript can be captured.** The debug file holds each item
  read (`SAY ALL PRESENTER: CONTENT: ... [heading: 'Home Page'] ...`),
  what was sent to the voice (`SPEECH DISPATCHER: Speaking ...`, for
  example `Lvk @ 73r`) and the language asked for (`language='it'` for
  the page's content, English for Firefox's own messages: the user's
  default voice is English, automatic language switching is on). Orca
  writes the file in blocks, so it is read after Orca quits.
- **The remote control works** (`org.gnome.Orca.Service`: `SayAll`,
  `InterruptSpeech`, `Quit`), but `SayAll` reads the window in front,
  from the current position: once it read the terminal, once only the
  footer. Since the page is read on load, the command does not need it;
  `Quit` is what it needs, to close Orca at the end.
- **On Wayland the user gives the focus**, with a click on the window,
  if the new window does not get it by itself.
- Zed is not visible to Orca at all.

## Questions

- What the command does. It can prepare everything (build and serve the
  site, open Firefox, start Orca, show the list of checks), but a check
  by hand is a person listening, while a regression check compares with
  an earlier result. That comparison can come from the person's answers
  recorded with each run, or from the text Orca speaks, captured and
  compared with the previous run's; or both.
- Which checks are on the list, and where the list and the results are
  written.
- How often it is run (every version, every new kind of page).
