# Review the logo

- **Opened:** 2026-11-05, by the user, building the light background in
  the card [give the articles' code, quotes and tables a look](../done/2026-11-06_design-look-of-articles.md);
  in Now.
- **Status:** noticed, not started.

## Why

From 768 pixels up the logo is drawn in characters, in the pixel font
at 16 pixels. On the light page the user found it illegible. The
drawing is not made of letters but of slashes, underscores and
backslashes (`/ _ \ |`): on black the light strokes merge into the
shape of the letters, on paper-white the dark strokes stay separate
lines, and the eye struggles to read them as letters. Below 768
pixels the logo is the name written in the same pixel font at 32
pixels, "Lvk@73r", and reads well on both backgrounds.

The light background was committed with the drawn logo as it was, so
this card could be decided on its own.

## Known from the start

- **Not a matter of colour.** The user compared, on the light page,
  the logo in the heading shade (`#a9600d`, 4.61:1), in a darker shade
  of the same hue (`#7e480a`, 7.14:1), with titles and headings darker
  too, and in the shade of code inside a sentence (`#6f3f09`,
  8.42:1): the lightest read best.
- **A black header was tried and rejected** by the user: the whole
  header black on the light page, the header and the footer black, and
  only the band above the header line black.
- **The user's leaning:** drop the drawn logo, and show the written
  logo everywhere, on both backgrounds.
- **What the drawn logo touches today:** the frame shared by every page
  (`src/page_frame.rs`, the `LOGO` constant and its `div`), two rules
  in `style/input.css`, two unit tests in `src/home_page.rs`, the
  header test (`end2end/tests/header-layout.spec.ts`, which expects it
  from 768 pixels up), and the accessibility test, which drops the
  contrast findings on the drawn logo only (WCAG exempts logos from
  contrast) and would no longer need to.
- **ADR-003** calls the logo in the pixel font "the one sign of 1985
  left in the frame" and says a logo drawn in characters needs a
  monospaced font: to be updated if the drawing goes.
