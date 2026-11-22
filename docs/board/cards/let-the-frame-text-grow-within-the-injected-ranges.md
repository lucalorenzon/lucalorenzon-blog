# Let the page frame's text grow within the injected ranges

- **Created:** 2026-11-21
- **Status:** In work

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md),
with the user's agreement. The user decided there that the text of
the page frame takes its sizes from the ranges the site injects, as
the column's text does, growing with the screen rather than with a
container. Without it, one page would mix sizes that jump at a
breakpoint, in the header and the footer, with text that grows
smoothly in the column: the lack of uniformity the user worried
about.

## What is known

- **Today** (`src/page_frame.rs`): the page title grows at 768
  pixels (`md:title-below-line-large`), from 24 to 32 pixels, and so
  does the footer's text (`md:footer-text-large`).
- **The frame stays on the screen's breakpoints** for its changes of
  layout, which cannot be smooth: the header from two rows to one at
  1280 pixels, and the written logo giving way to the logo drawn in
  characters at 768 (decided by the user in the spike).
- **The ranges are those of the column's card**
  ([let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md)),
  which sets how a range is written, first in the stylesheet; the
  range moves to the configuration in a second increment
  ([read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md)).
- **The title has a limit of graphemes**
  (`config/heading-max-graphemes.txt`), checked by the header's
  browser test at today's sizes.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which ranges the title and the footer use**, and whether the
  title's range is one of the column's or one of its own.

  **Answered for the title by the user on 2026-11-22.** The title
  grows over the column's band of widths, from 320 to 1366 pixels,
  between its own bounds of today: 24 pixels at 320, 32 at 1366. It
  does not keep a fixed ratio to the column's text, which would have
  left it at about 26.7 pixels on wide screens. Its jump at 768 is not
  kept in step with the logo's change at the same width: the user
  expects that change to fall, with a fixed icon in place of the two
  logos, a matter of the card
  [review the logo](review-the-logo.md). On tablets and small
  laptops the title is smaller than today: about 27.4 pixels at 768
  instead of 32.

  **Answered for the footer by the user on 2026-11-22.** The footer's
  text follows the title's rule: the column's band of widths, between
  its own bounds of today, 16 pixels at 320 and 18 at 1366. Kept in
  proportion to the column's text instead, it would have reached about
  17.8 pixels, a difference no reader sees; the same rule for the
  whole frame was preferred.
- **Whether the longest title still fits** at every width, now that
  its size varies, and what the header's test checks.

  **Answered by the user on 2026-11-22, from the agent's reasoning.**
  The longest title still fits. From 768 pixels up the title is never
  larger than today (about 27.4 pixels at 768, 31.3 at 1280, 32 only
  from 1366), so it takes no more lines than today, and the limit of
  two lines from 1280 holds better. Below 768 it is larger than today,
  up to about 27.4 pixels instead of 24, but there it sits under the
  header line, centred and free to wrap, and the room grows faster
  than the font: 140 per cent more width against 14 per cent more
  size. The header's browser test
  (`end2end/tests/header-layout.spec.ts`), which today expects
  exactly 24 pixels below 768 and 32 from 768, checks instead that the
  title lies on its range's straight line, 24 pixels at 320 and 32
  from 1366, within 0.05 pixels, as the column's test does, at the
  widths it already runs on: the devices and the two sides of each
  breakpoint. Its other checks, two lines at most from 1280 and no
  overlap, prove the reasoning above.
- **Whether the text grows with the browser's zoom**, as for the
  column.
- **Whether a browser test checks the footer's text size.** Came up
  during the work: no browser test checks it today, neither the 16
  pixels below 768 nor the 18 from 768, so its change to a range
  would go unchecked, as would any later change.

  **Answered by the user on 2026-11-22.** Yes: a browser test checks
  that the footer's text lies on its range's straight line, 16 pixels
  at 320 and 18 from 1366, within 0.05 pixels, as the title's and the
  column's do.

## Dependencies

- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md),
  which defines the ranges.

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike that decided it.
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the first use of the ranges.
- [Read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md):
  where the ranges move after the stylesheet.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that shows the frame at several widths.
- [Review the logo](review-the-logo.md): the logo's change at 768
  pixels, which the title's growth no longer follows.
