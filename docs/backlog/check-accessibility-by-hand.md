# Check accessibility by hand

- **Opened:** 2026-10-29, from the split of the card "Add automated
  accessibility tests", opened on 2026-10-28 by the user.
- **Status:** noticed, not started.

## Why

An automated scanner such as axe, used by the card
[check accessibility in the build](../wip/check-accessibility-in-the-build.md),
finds only part of the problems: most WCAG criteria, and most of level
AAA, need a person. For example, how the page sounds in a screen reader,
including the ASCII-art logo, which is marked as an image named
"Lvk@73r" (`role="img"`, `aria-label`).

## Questions

- Which checks are done by hand, with which screen reader and browser,
  and how often (every version, every new kind of page).
- Where the checklist and its results are written.
