# Check the accessibility tree in the build

- **Opened:** 2026-10-30, from the split of the card "Check
  accessibility by hand", itself split on 2026-10-29 from the card "Add
  automated accessibility tests", opened on 2026-10-28 by the user.
- **Status:** done on 2026-10-30, in commit `082d32d`.

## Why it was opened

axe, run by the build since the card
[check accessibility in the build](2026-10-30_check-accessibility-in-the-build.md),
finds rule violations, but not a change in what a screen reader is
given: a logo that loses its name, a heading that changes level, parts
read in a different order. The user asked whether screen readers can be
checked automatically.

## Answers

### Which tool?

**Playwright's own `toMatchAriaSnapshot` (chosen by the user).** It
compares the page's accessibility tree, the roles, names and order a
screen reader receives, with an expected tree written in the test. No
new dependency.

Rejected:

- a simulated screen reader (`@guidepup/virtual-screen-reader`, still
  0.x), which turns the same tree into phrases and adds little to the
  comparison;
- real screen readers driven by a program (Guidepup): only VoiceOver on
  macOS and NVDA on Windows, not Orca on Linux, so slower and costlier
  machines on GitHub for a one-page site.

None of them hears how the page sounds; that is the card
[run a screen reader check on demand](../backlog/run-screen-reader-check-on-demand.md).

### What is compared, and how strictly?

**The tree under `<body>`, at the three widths of the axe test**, since
the page shows different elements per band: on phones the logo is the
text "Lvk@73r"; from 768px it is an image named "Lvk@73r". The expected
trees are written in the test, so a change is visible in review.

Every container is matched with `/children: equal`: an extra, missing
or reordered element fails, where Playwright's default accepts extra
ones. `deep-equal` was rejected, as it would also demand the ASCII art
inside the image, which a screen reader does not read: it reads the
name.

## Results

- In the Playwright 1.64.1 image: 19 tests out of 19 passed.
- With the title turned from level 1 into level 2 in a copy of the
  site, axe still passed (one level-1 heading is good practice, not a
  WCAG rule), and the three tree tests failed, showing the level change.
