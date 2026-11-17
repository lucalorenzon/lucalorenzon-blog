# Hide the wrap toggle on blocks that fit

- **Created:** 2026-11-17
- **Status:** In work

## Why

The user opened this card on 2026-11-17, while working on the card
[measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
the cases at the limit of the code page, measured before being
written, showed that the wrap toggle of the code blocks can show on a
block that fits its column.

The user's rule, given in that card: the toggle on a block that fits
is a broken page, because it takes space, and what can be avoided is
better avoided. The cases at the limit cannot be written until the
toggle's threshold agrees with what the browser draws.

## What is known

- **How the toggle shows.** The build gives each block the class
  `@max-[Wem]/code:wrap-shown`, W being its longest line in characters
  times Hack's advance, 1233/2048 of an em, rounded up to three
  decimals (`code_width_em` in `src/markdown.rs`); em is the block's
  own font size, 85% of the text. The toggle shows when the block's
  content box is at most W wide.
- **What Chromium draws on this machine.** Measured on 2026-11-17 by a
  sub-agent, on the built code page, Linux, Playwright's Chromium: at
  17 pixels (the code at 768 pixels and over) each character of Hack
  is 10 pixels wide, against the 10.234 pixels the build assumes; at
  15.3 pixels (below 768) it is 9 pixels. In the wide column (scroller
  686 pixels) a line of 67 characters fits with no toggle, one of 68
  fits (680 pixels) and shows the toggle, one of 69 overflows and
  shows it.
- **Supposed, not checked:**
  - the cause is the rounding of the characters' advances to whole
    pixels, which depends on the platform and its font settings, so
    another system may draw Hack at 10.234 pixels, where the threshold
    is right;
  - the same window, a block that fits with the toggle, exists in the
    narrower columns too, since the build assumes 9.21 pixels per
    character at 15.3 pixels;
  - the pipeline's Chromium, in Playwright's container on Linux,
    rounds as this machine does.
- **Why the tests did not see it.** The test `each one scrolls or fits
  as written` (`end2end/tests/code-blocks.spec.ts`) fails when a block
  that fits shows the toggle, but the code page has no block whose
  longest line falls in the window.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the pipeline's Chromium draws Hack as this machine does.**
- **What other browsers and systems draw**, at least Firefox and a
  system other than Linux, so the threshold is right for the readers
  and not only for the tests.
- **How the threshold can agree with the browser** when the width of
  a character depends on where the page is drawn, without making the
  opposite fault, a block that overflows without the toggle, which
  breaks the page.

## References

- [Measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
  the card whose measurements found this fault, and which waits for
  this one.
- [Show the code block in the style guide](../done/2026-11-15_show-the-code-block-in-the-style-guide.md):
  where the code page and its cases come from.
- [Take the measures from the configured fonts](../backlog/take-the-measures-from-the-configured-fonts.md):
  the code font's advance, measured in the build, may come from there.
