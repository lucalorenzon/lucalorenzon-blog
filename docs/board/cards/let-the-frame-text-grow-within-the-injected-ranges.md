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
- **Whether the longest title still fits** at every width, now that
  its size varies, and what the header's test checks.
- **Whether the text grows with the browser's zoom**, as for the
  column.

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
