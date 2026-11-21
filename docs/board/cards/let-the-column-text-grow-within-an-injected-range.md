# Let the column's text grow within a range the site injects

- **Created:** 2026-11-21
- **Status:** In work

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md),
with the user's agreement. The user decided there that the text of
the column grows smoothly with the space it has in the page, within a
range of sizes the site injects, instead of jumping from 18 to 20
pixels at the screen's breakpoint of 768 pixels. Why, in the spike:
inside a container the space is the same for every child, so the
site need not impose fixed sizes from outside, which would be too
rigid; and the site still sets the bounds, so the text stays uniform
across components.

As long as the column jumps at a screen breakpoint, its page in the
style guide has a threshold to explain and to keep in step with the
code, which is the problem the spike was opened for.

## What is known

- **Today**: the column's text is 18 pixels, and 20 from a screen of
  768 pixels, through the class `md:long-text-large`; its maximum
  width, 60 characters of its own font, follows the text, 600 pixels
  then 720 (`src/page_frame.rs`, `style/input.css`). The column is the
  frame's `<main>` since 2026-11-21, no longer a component of its own
  ([decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md)).
- **What the spike's experiment found** with a container query in
  place of the screen's breakpoint, on a branch deleted at the
  spike's close (details in the spike's card):
  - Tailwind 4.3.5 supports container queries with no plugin. A
    container size that is not defined, or whose name is mistyped,
    leaves the rule out with no error.
  - The Rust checks of the classes refused the new classes: the
    container's class and the `@` prefix. The experiment wrote the
    prefix into the code by hand, a shortcut.
  - A container does not include the scrollbar, the screen's
    breakpoint does: with a classic scrollbar, the column and the
    header, which stays on the screen's breakpoints, disagree in a
    band of about 15 pixels. The user accepted it in the spike. The
    tests' headless Chromium hides the scrollbars; showing them needs
    `ignoreDefaultArgs: ['--hide-scrollbars']`.
  - Declaring the frame's `div#top` a container changed nothing in
    the layout: every test passed, and screenshots were identical.
  - A container without a name is the one any descendant queries;
    the code blocks and the tables already use named containers
    (`code`, `table`), so the page's container needs a name too.
  - Tailwind's default container sizes are active in this project,
    since the build clears only the screen's breakpoints.
- **Units that measure a container** (`cqi`) need an ancestor
  declared a container, so the container of the experiment is still
  needed even with no threshold.
- **The code block and the table measure their thresholds in `em`**
  of the text (`style/input.css`, `src/table_widths.rs`), so they
  follow a text that grows; the table's column line is counted at the
  smallest size, 18 pixels (`COLUMN_LINE_DIVISOR`), which the range
  should not go below without checking it.

## Questions to answer

The ones known when the card was written; there may be more.

- **How the range is written and where it lives**: the smallest and
  largest size, and the widths at which they are reached; in
  `config/`, as the breakpoints and the fonts are, and whether it
  waits for the card on a structured configuration file.
- **Which widths the range spans**, and whether the sizes stay 18 and
  20 pixels.
- **Whether the text still grows with the browser's zoom**, as WCAG
  1.4.4 asks, with the smallest size in units of the page's font.
- **How the Rust checks of the classes learn the containers**, so a
  mistyped container name or size stops the build instead of leaving a
  rule out in silence.
- **What the browser test of the column checks**: that the size stays
  within the range at several widths, and whether it runs with the
  scrollbars shown.
- **Whether a line of the column stays near 60 characters** at every
  width of the range.

## Dependencies

- [Use a structured configuration file](use-a-structured-configuration-file.md),
  perhaps, depending on where the range lives.

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike that decided it.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  the same ranges, for the header and the footer.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that shows the column at several widths, since the
  column's own page joined it.
- [Decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md):
  the card that put the column back into the frame.
- [Use a structured configuration file](use-a-structured-configuration-file.md):
  a possible place for the range.
- [Check the table widths against Chromium's rounding](check-table-widths-against-chromium-rounding.md):
  the table's widths, measured at a size that would now vary.
