# Check the accessibility tree in the build

- **Opened:** 2026-10-30, from the split of the card "Check
  accessibility by hand", itself split on 2026-10-29 from the card "Add
  automated accessibility tests", opened on 2026-10-28 by the user.
- **Status:** being worked on since 2026-10-30.

## Why

axe, run by the build since the card
[check accessibility in the build](../done/2026-10-30_check-accessibility-in-the-build.md),
finds rule violations, but not a change in what a screen reader is
given: a logo that loses its name, a heading that changes level, parts
read in a different order. The user asked whether screen readers can be
checked automatically. Three kinds of tools exist:

- the accessibility tree, compared by Playwright's own
  `toMatchAriaSnapshot`: roles, names and order the screen reader
  receives, no new dependency;
- a simulated screen reader (`@guidepup/virtual-screen-reader`, 0.x),
  which turns the same tree into phrases, adding little to the above;
- real screen readers driven by a program (Guidepup), only VoiceOver on
  macOS and NVDA on Windows, not Orca on Linux, so slower and costlier
  machines on GitHub for a one-page site.

None of them hears how the page sounds (the voice's language, how
"Lvk@73r" is spelled out): that is the card
[run a screen reader check on demand](../backlog/run-screen-reader-check-on-demand.md).

## Agreed scope

Decided with the user on 2026-10-30: the build compares the page's
accessibility tree with the expected one, with Playwright's
`toMatchAriaSnapshot`, at one width per layout band, since the page
shows different elements per band.
