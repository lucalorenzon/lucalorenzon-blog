# Run a screen reader check on demand

- **Opened:** 2026-10-30, from the split of the card "Check
  accessibility by hand", itself split on 2026-10-29 from the card "Add
  automated accessibility tests", opened on 2026-10-28 by the user.
- **Status:** noticed, not started.

## Why

The tools the build can run (axe, and the accessibility tree compared by
the card
[check the accessibility tree in the build](../wip/check-accessibility-tree-in-the-build.md))
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
