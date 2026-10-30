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
