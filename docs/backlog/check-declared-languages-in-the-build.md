# Check the declared languages in the build

- **Opened:** 2026-10-30, decided by the user while the card
  [run the screen reader check in a container](../wip/run-screen-reader-check-in-a-container.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

A screen reader picks its voice from the language the page declares for
each part (WCAG 3.1.1, language of page, level A; 3.1.2, language of
parts, level AA). That is the site's responsibility, and today only the
screen reader transcript shows it. The build's accessibility tree test
(`end2end/tests/accessibility.spec.ts`) compares roles, names, heading
levels and order, not languages; axe checks that the page's language is
present and valid, not that each part has the intended one.

The user decided that languages are checked in the build, while Orca
stays a tool run on demand (reasons in the card linked above).

## Questions

- What exactly to compare: the language of the page and of every part
  that declares its own, against an expected list, like the tree test
  does for roles and names?
- On which screen widths, since the page shows different elements per
  width (the text logo and the ASCII-art logo)?
