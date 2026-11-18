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
- **Chromium on this machine rounds each character to whole pixels,
  in both directions.** Measured on 2026-11-17 by the agent, Chromium
  158 of Playwright 1.65, Linux, drawing a line of 100 characters of
  Hack inside the code block at every size from 10 to 30 pixels, in
  tenths: the width of a character is always a whole number of
  pixels, the nearest to the true one. About half the sizes are drawn
  narrower than the build assumes (15.3 pixels: 9 against 9.211; 17
  pixels: 10 against 10.235), the other half wider (16 pixels: 10
  against 9.633; 18 pixels: 11 against 10.837). Drawn wider, a block
  can overflow without the toggle, which breaks the page; the site's
  own sizes, 15.3 and 17 pixels, happen to be both narrower, but a
  reader who changes the browser's text size can reach the others.
- **Firefox draws the true width.** Same measure, Firefox 159 of
  Playwright, same machine: the width of a character differs from
  1233/2048 of an em by less than one hundredth of a pixel at every
  size, and the build rounds its threshold up, so the toggle is right
  there. Seen in passing, not looked into: Firefox gives the code's
  scroller other widths than Chromium (591 against 566 pixels at 767,
  661 against 686 at 768 and over).
- **With `text-rendering: geometricPrecision` on the code, Chromium
  draws the true width.** Same measure on 2026-11-17, with that rule
  added to the code of the page: at every size from 10 to 30 pixels a
  character differs from 1233/2048 of an em by at most 0.011 pixels,
  and is never wider by more than 0.01 pixels. What the rule changes
  in the look of the code on Linux, where it turns off the fitting of
  the letters to the pixels, has not been looked at yet.
- **Supposed, not checked:** the rounding comes from how Chromium draws
  fonts on Linux, so on another system Chromium may draw the true
  width, as Firefox does.
- **Measuring script:** `measure-advance.mjs`, in the session's
  scratchpad, not kept.
- **Why the tests did not see it.** The test `each one scrolls or fits
  as written` (`end2end/tests/code-blocks.spec.ts`) fails when a block
  that fits shows the toggle, but the code page has no block whose
  longest line falls in the window.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the pipeline's Chromium draws Hack as this machine does.**

  **Answered by the measure on 2026-11-17**, run by the user in
  Playwright's container `mcr.microsoft.com/playwright:v1.65.0-noble`,
  the one the pipeline uses: yes, the same width at every one of the
  201 sizes measured, Chromium 158.

- **What other browsers and systems draw**, at least Firefox and a
  system other than Linux, so the threshold is right for the readers
  and not only for the tests.

  **Answered in part on 2026-11-17.** Firefox on Linux draws the true
  width (see "What is known"). No system other than Linux was
  measured. WebKit 27.2 of Playwright, on the same machine, also draws
  the true width (at most 0.0075 pixels apart), with or without
  `text-rendering: geometricPrecision`; the user asked for Safari, but
  this is WebKit's port for Linux, which draws fonts with Linux's
  tools, so it says nothing certain of Safari on macOS. Its code
  scroller has Firefox's widths, not Chromium's.
- **What `text-rendering: geometricPrecision` changes in the look of
  the code** in Chromium on Linux, where it turns off the fitting of
  the letters to the pixels.

  **Answered by the user on 2026-11-18**, from screenshots of the code
  page taken by the agent in Playwright's Chromium, with and without
  the rule, at 375 pixels (device pixel ratio 2) and 1366 pixels
  (ratio 1), whole and magnified: the change is practically
  invisible. At ratio 2 the agent saw no difference in sharpness; at
  ratio 1, magnified, some strokes fall between two pixels and are a
  little softer, the underscore most of all, grey instead of black.
  Lines and indents keep their height and place. Measured on the same
  pages: with the rule a character of Hack is 9.2095 pixels at 15.3
  pixels and 10.235 at 17, the widths the build assumes; without it,
  9 and 10.

- **How the threshold can agree with the browser** when the width of
  a character depends on where the page is drawn, without making the
  opposite fault, a block that overflows without the toggle, which
  breaks the page.

  **Answered by the user on 2026-11-18**, with the answer above:
  `text-rendering: geometricPrecision` on the code block, so Chromium
  draws the width the build already assumes, as Firefox and WebKit do.
  The threshold of the build stays as it is. Known limit, from the
  measures of 2026-11-17: at sizes other than the site's, a character
  can be drawn up to about a hundredth of a pixel wider than the true
  width, so a line of a hundred characters could overflow by less
  than one pixel without the toggle.

- **Which case on the code page is the regression test.** Came up on
  2026-11-18.

  **Answered by the user on 2026-11-18.** Both cases at the limit of
  the card
  [measure the fonts again when they change](measure-the-fonts-again-when-they-change.md)
  are written here, with the fix: the line one character longer than
  the column is the regression test, since today it fits and shows the
  toggle, and after the fix it overflows and shows it; the line that
  fills the column exactly is its complement, right before and after.
  That card keeps only the proof that the guard works. Rejected:
  leaving that card as it was and writing a case of its own here,
  which would put two cases on the page for one threshold.

## Acceptance tests

Agreed with the user on 2026-11-18; tests 1 to 4 and 6 come from the
card
[measure the fonts again when they change](measure-the-fonts-again-when-they-change.md),
where they were agreed on 2026-11-17.

1. **The case that fits.** The code page of the style guide has a
   block whose longest line is the longest that fits the column at the
   width chosen for the cases; at that width the block does not scroll
   and shows no wrap toggle.
2. **The case one character longer, the regression test.** A second
   block has the same line with one character more; at the same width
   it scrolls and shows the wrap toggle. Run before the fix, the test
   fails on this case, because the block fits and shows the toggle;
   the failure is shown to the user.
3. **The width chosen.** The cases are measured at the widest column
   (768 pixels and over), where the line is longest, so one character
   is the smallest share of it and the test notices the smallest
   difference.
4. **At the other column widths** both cases do what their text says
   (at a narrower column both scroll and show the toggle), checked by
   the existing test `each one scrolls or fits as written`, extended to
   the new cases.
5. **The fix.** The code block is drawn with `text-rendering:
   geometricPrecision`, and every browser test passes.
6. **The browser tests' README** (`end2end/README.md`) describes the
   new cases in the test's row, and `end2end/tests/readme.spec.ts`
   passes.
7. **The text of the two cases** is drafted by the agent and reviewed
   by the user before the commit.

## What it leaves

- The measure on macOS, Safari and Chrome, put in Later by the user on
  2026-11-17: card
  [measure the code font on macOS](../backlog/measure-the-code-font-on-macos.md).

## References

- [Measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
  the card whose measurements found this fault, and which waits for
  this one.
- [Show the code block in the style guide](../done/2026-11-15_show-the-code-block-in-the-style-guide.md):
  where the code page and its cases come from.
- [Take the measures from the configured fonts](../backlog/take-the-measures-from-the-configured-fonts.md):
  the code font's advance, measured in the build, may come from there.
- [Measure the code font on macOS](../backlog/measure-the-code-font-on-macos.md):
  the measure on a system other than Linux, left to Later.
