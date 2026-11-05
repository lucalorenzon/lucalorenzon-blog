# Let readers adjust font and spacing

- **Opened:** 2026-11-04, by the user, while giving wide code a "wrap"
  toggle ([card](../done/2026-11-05_annotate-markdown-for-wide-content.md)); in
  Later.
- **Status:** noticed, not started.

## Why

The user imagines, one day, a control on the site to change the text's
size and line spacing, as a Kindle lets its reader do. Readers of long
technical articles have different needs, and the browser's own zoom
changes the whole page rather than the text alone.

It would also remove a limit accepted on 2026-11-04: when a reader
widens the letter spacing with their own tools, code lines grow wider
than the threshold of the "wrap" toggle, which may then stay hidden
while the code scrolls. If the site itself sets the spacing, it knows
it, and the threshold can follow.

## Known from the start

- **No script on the site today:** a control that remembers the
  reader's choice probably needs one.
- **The readability rules stay:** every setting offered must keep the
  checks of WCAG 1.4.8 and 1.4.12 passing.
- **The toggle's threshold** counts characters of the code font as
  half an em (`src/markdown.rs`, `style/input.css`); a font or spacing
  setting must keep it right.

- **A light and a dark background** come with
  [ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md): at
  first the site follows the preference set in the reader's browser,
  with no script. A control on the site to choose between them belongs
  here.

## Questions

- Which settings: size, line spacing, letter spacing, font, background.
- Where the control lives, and whether the choice is remembered.
- How it fits the site's look, set by ADR-003 and built by the
  [look card](../done/2026-11-06_design-look-of-articles.md).
