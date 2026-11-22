# Let the column's text grow within a range the site injects

- **Created:** 2026-11-21
- **Status:** Closed
- **Closed:** 2026-11-22

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

- **How the trials were run** (2026-11-22): on a throwaway branch,
  the page's wrapper (`div#top`) declared a container named `page`,
  and the column's size written as a value clamped between the
  smallest and the largest size, growing linearly with the
  container's width (`cqi`) between two widths; the four numbers
  were stylesheet variables, changed live from a comparison page that
  showed the site in frames from 320 to 1366 pixels wide. Checked in
  Chromium and Firefox.
- **The size variable has to be registered** (`@property
  --long-text-size`, of type `<length>`): otherwise the code block and
  the table, which are containers too and use the variable, would
  compute its `cqi` against their own width. Registered, the variable
  reaches them already computed in pixels; checked on the branch,
  where the code block's spacing followed the text.

- **What the browser tests said** (2026-11-22), run on the throwaway
  branch with the chosen range, 18 to 20 pixels between 320 and 1366:
  267 passed, 8 failed, 1 skipped. Every check of readability and
  spacing passed at every width. The failures fell in three groups:
  - the column's own test (`text-column.spec.ts`), 4 failures, which
    expects 18 pixels below 768 and 20 from 768;
  - the tables' cases at the limit (`table-forms.spec.ts`), 3
    failures at 767: the six-column table and the rows of different
    heights, meant to be record viewers there, fit as tables, since
    the text at 767 is now about 18.85 pixels and the column wider;
  - the code block's case at the limit (`code-blocks.spec.ts`), 1
    failure at 768: the line that just fits the widest column, which
    is written to scroll only below 768, now scrolls there too, since
    the column reaches its widest only at 1366.

## Questions to answer

The ones known when the card was written; there may be more.

- **How the range is written and where it lives**: the smallest and
  largest size, and the widths at which they are reached; in
  `config/`, as the breakpoints and the fonts are, and whether it
  waits for the card on a structured configuration file.

  **Answered by the user on 2026-11-22.** The work is cut in two
  increments. In this card the range is written in the stylesheet, so
  the column's text grows smoothly with no change to the build or to
  the configuration. Moving the range into the configuration, read by
  the build, is a second increment, in the card
  [read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md).
- **Which widths the range spans**, and whether the sizes stay 18 and
  20 pixels.

  **Answered by the user on 2026-11-22**, after trying values on a
  comparison page: the text grows from 18 pixels at a page width of
  320 to 20 pixels at 1366. Below 18 the text at 320 pixels becomes
  hard to read; above 20 it looks too large at 1366; 18 to 20 keeps
  the text more uniform across widths. What the trials showed:
  - The end of the band decides how large the text is on tablets and
    laptops, not how long the lines are: where the column is at its
    widest, it widens with the text and keeps about 59 characters.
  - On phones the column fills the page, so the length of a line
    depends mostly on the smallest size: about 30 characters at 320
    pixels with 18-pixel text.
  - With the band ending at 1366, the text is smaller than today
    between 768 and 1366 pixels: about 18.9 pixels at 768, against 20
    today.
- **Whether the text still grows with the browser's zoom**, as WCAG
  1.4.4 asks, with the smallest size in units of the page's font.

  **Answered by the user on 2026-11-22.** It does, and that is
  enough. Zooming makes the page narrower in CSS pixels, so the text
  slides down its range while the zoom enlarges it: at 200 per cent
  zoom the text is enlarged 187 to 199 per cent, depending on the
  window (187 at 1366 pixels). The criterion asks only that text "can
  be resized without assistive technology up to 200 percent", and its
  Understanding document
  (https://www.w3.org/WAI/WCAG22/Understanding/resize-text.html) says
  that when text does not grow steadily with zoom, as when a media
  query changes it, 200 per cent must still be reachable in some way:
  here, by zooming a little further. The failure it names, text sized
  in viewport units that zoom does not enlarge, does not apply: the
  bounds are in pixels. A reading by Maxwell Barvian
  (https://www.smashingmagazine.com/2023/11/addressing-accessibility-concerns-fluid-type/),
  an interpretation and not part of WCAG, asks that text be at least
  doubled at the browser's largest zoom, 500 per cent, and finds that
  always true when the largest size is at most 2.5 times the smallest;
  here it is 20 over 18. Today's site does worse: in a 1366-pixel
  window at 200 per cent the page falls below 768 pixels, the text
  drops back to 18 pixels and is enlarged 180 per cent. Whether the
  bounds should follow the browser's own font-size setting is a
  question of its own, below.
- **Whether the text follows the browser's font-size setting**, which
  enlarges text without zooming. Bounds in pixels ignore it, as the
  site does today; WCAG asks for at least one way to enlarge text,
  and zoom is one. Added on 2026-11-22, out of the question on zoom.

  **Answered by the user on 2026-11-22.** Zoom is enough, as today:
  the bounds stay in pixels. Following the setting would have taken
  the text below 18 pixels for whoever sets a smaller default, under
  the size at which the table's column line is counted
  (`src/table_widths.rs`), and would have made the column follow the
  setting while the header and the footer do not.
- **How the Rust checks of the classes learn the containers**, so a
  mistyped container name or size stops the build instead of leaving a
  rule out in silence.

  **Answered by the user on 2026-11-22**: the question falls for this
  card. The trials needed no new class in the Rust code: the container
  is declared inside the stylesheet's rule the page wrapper already
  uses (`page-fills-screen`), and the growth lives in the column's
  rule (`long-text-column`), with no threshold and no prefix; the Rust
  code only loses the class `md:long-text-large`. A different risk
  stays, which the class checks would not see anyway: without the
  container's declaration, the unit that measures it falls back to the
  window's width, and the text still grows, a few pixels off because
  of the scrollbar, with no error. Only a browser test can notice it.
- **What the browser test of the column checks**: that the size stays
  within the range at several widths, and whether it runs with the
  scrollbars shown.

  **Answered by the user on 2026-11-22.** The visible behaviour is
  enough: the text is 18 pixels at 320 and 20 from 1366, between the
  two in between, growing with the width without jumps; the column
  stays 60 characters of its own font at most, and centred. The test
  runs with the scrollbars hidden, as every other test: so it does
  not notice a missing container declaration, which would make the
  text follow the window's width instead of the page's, a scrollbar's
  width apart.
- **What happens to the browser tests built on the jump at 768.**
  Added on 2026-11-22, after the browser tests ran on the branch. The
  cases at the limit of the tables and of the code block were placed
  where the column widened at once, which no longer happens.

  **Answered by the user on 2026-11-22.** The tables' cases at the
  limit are children of a breakpoint with a fixed change of size, so
  this card takes them out; whether other cases at the limit can be
  found is a card of its own. The code block's case at the limit
  stays, moved from 768 to 1366: it is the check that stops the build
  when a code font moves the wrap toggle's threshold
  ([measure the fonts again when they change](measure-the-fonts-again-when-they-change.md)),
  and from 1366 the column is as wide as it is today from 768, its
  text being 20 pixels there. That the move is enough is supposed,
  not yet checked. The tables keep a check of the same kind: on every
  width, no table may be wider than its column.
- **Which widths the tests of what is inside the column run on.**
  Added on 2026-11-22: they were one width per size of the column
  (320, 393, 412, 767 and 768,
  [check at every width only what depends on it](check-at-every-width-only-what-depends-on-it.md)),
  a rule that gives no finite set once the column grows smoothly up
  to 1366.

  **Answered by the user on 2026-11-22**: 320, 393, 412, 843 and
  1366. 767 and 768 go, since they no longer tell anything about the
  column; 1366 comes in, where the column reaches its widest, as the
  code block's case at the limit needs; 843, where the text is 19
  pixels, is a token presence in the middle, so the tests see the
  text at 18, 19 and 20 pixels. The widths between 412 and 1366 stay
  covered by the tests of layout and readability that run on every
  device.
- **Whether a line of the column stays near 60 characters** at every
  width of the range.

  **Answered by the agent on 2026-11-22**, from the trials: where the
  column is at its widest, from about 700 pixels, a line keeps about
  59 characters whatever the size, since the column's width is
  measured in characters of its own font. Below, the column fills the
  page and lines are shorter, as today: about 30 characters at 320
  pixels. The range does not change this; the smallest size does,
  and it stays 18 pixels.

## Dependencies

None known. The possible dependency on the card
[use a structured configuration file](use-a-structured-configuration-file.md)
went, with the range's place in the configuration, to the card
[read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md).

## Acceptance tests

Agreed with the user on 2026-11-22. The card is done when:

1. **The growth.** The column's text is 18 pixels at 320, 20 pixels at
   1366 and at 1920, and grows with the page's width without jumps:
   at the column widths it follows the range's straight line, within
   a small tolerance.
2. **The column.** It stays at most 60 characters of its own font
   wide, centred, filling the screen up to that limit (already
   checked; the check stays).
3. **The size inside the components.** Inside the code block and the
   table, the text size the column passes on is the same as the
   column's, in pixels: the guard against losing the registration of
   the size variable.
4. **The code block's case at the limit.** The line that just fits
   does not scroll from 1366 up and scrolls below; the line one
   character longer always scrolls.
5. **The tables.** The tables' cases at the limit leave the style
   guide's tables page and their tests: the test of the keyboard on
   the record viewer moves to a phone's width, where the six-column
   table is a viewer; the test of the rows of different heights loses
   767. The page's text no longer names 767 and 768, with the
   sentences the user approved on 2026-11-22, and its section "Texts
   at the limit" is gone.
6. **The column widths** are 320, 393, 412, 843 and 1366; 843 and
   1366 are among the column widths only, not among the devices, and
   are written in the tests by hand until the range moves to the
   configuration.
7. **The old class.** `md:long-text-large` is gone from the Rust code
   and from the stylesheet.
8. **The descriptions.** The browser tests' README and the tests'
   comments are updated for every test touched.
9. **The pipeline.** Every check of the pipeline passes: formatting,
   clippy, the Rust tests, the type check and the browser tests.

## Recap

The column's text grows from 18 pixels at a page width of 320 to 20
pixels at 1366, in a straight line, instead of jumping at 768
(`8e9c396`). The page's wrapper is a container named `page`; the
column's size is a value clamped between the two sizes and measured
against that container; the size variable is registered, so the code
block and the table receive it in pixels; the class
`md:long-text-large` is gone. The tables' cases at the limit left the
style guide's tables page and their tests, with the sentences the user
approved; the code block's case at the limit moved to 1366; the
column widths of the browser tests became 320, 393, 412, 843 and
1366; the browser tests' README was updated. Every check of the
pipeline passed. Before the code, the decisions were taken on a
throwaway branch with a comparison page (the trials above), deleted at
the close.

### What the column does in Chromium on Linux

Measured by the agent on 2026-11-22, in Chromium and Firefox, after
the code was written. Chromium on Linux rounds each character's width
to a whole pixel, as it does with the code font. The column's maximum,
60 characters of the digit zero, 0.579 em in Literata, so steps in
Chromium: 600 pixels with 18-pixel text, 660 from about 18.13 pixels,
720 from about 19.86, that is from a page of about 1294 pixels; in
Firefox it grows smoothly, 625 to 695. Lines stay at 60 characters,
since the text is drawn with the same rounding. Today's site steps
too, 600 and 720, but at 768, where the text jumps. The visible
consequence is in the six-column table, whose estimated threshold,
33.904 em, lies close to the column's widest: in Chromium on Linux it
is a record viewer up to 412, a table from 659 to 810, a viewer again
from 843 to 1280 and a table from about 1300. Every form fits its
column and every test passes. The user decided on 2026-11-22 to close
the card as it is and to deal with this in the cards on Chromium's
rounding and on cases at the limit.

## Cautions

- The range, 18 to 20 pixels between 320 and 1366, is written twice
  by hand: in `style/input.css` and in `end2end/tests/support.ts`
  (`TEXT_RANGE`). A change goes in both until
  [read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md)
  is done.
- The browser tests run with the scrollbars hidden, so they do not
  notice a missing container declaration: the text would follow the
  window's width, a scrollbar's width apart.
- The form of a table between 412 and 1366 pixels is checked only by
  "exactly one form is shown", and in Chromium on Linux it changes
  more than once as the page widens (see the recap).

## What it leaves

- The search for new cases at the limit, in place of the tables'
  ones taken out: the card
  [find cases at the limit for the growing column](find-cases-at-the-limit-for-the-growing-column.md).
- The tables that change form more than once as the page widens, in
  Chromium on Linux: the cards
  [check the table widths against Chromium's rounding](check-table-widths-against-chromium-rounding.md)
  and
  [find cases at the limit for the growing column](find-cases-at-the-limit-for-the-growing-column.md).
- The range written in `config/` and read by the build, instead of in
  the stylesheet: the card
  [read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md).

## References

- [Check at every width only what depends on it](check-at-every-width-only-what-depends-on-it.md):
  the card that chose one width per size of the column.
- [Find cases at the limit for the growing column](find-cases-at-the-limit-for-the-growing-column.md):
  cut from this card, for the tables' cases taken out.
- [Measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
  the check the code block's case at the limit carries, kept.
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
  a possible place for the range, now a matter of the next card.
- [Read the column text's range from the configuration](read-the-column-text-range-from-the-configuration.md):
  the second increment, cut from this card.
- [Check the table widths against Chromium's rounding](check-table-widths-against-chromium-rounding.md):
  the table's widths, measured at a size that would now vary.
