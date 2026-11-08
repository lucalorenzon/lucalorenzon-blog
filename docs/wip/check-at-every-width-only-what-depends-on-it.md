# Check at every width only what depends on the width

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on making the browser tests check behaviour, not articles
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)); in Now, second of three slices.
- **Status:** being worked on, since 2026-11-08.

## Why

Many browser tests run at the 12 screen widths of `VIEWPORTS`
(`end2end/tests/support.ts`). The widths are needed where the layout
changes with them; they are not needed for behaviour that does not
depend on the width, such as reaching a control with the keyboard,
copying code or following a link. For example, the keyboard test of
the record viewer runs at every width and in both themes for each page
with tables.

## Decision to take

For each test that runs at several widths, whether its result can
change with the width, using the census in [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md); those that
cannot run at one width, and which one.

## Known from the start

- **No check is lost** if only the tests whose result cannot change
  with the width are reduced.
- **Some behaviour exists only at some widths:** the record viewer is
  shown only where a table no longer fits, the wrap toggle of code only
  where a line does not fit; a single width must be one where the
  component is shown.

## Decided so far

1. **Scope widened** (user): the card decides, for every layout test,
   the widths that can change its result, accepting losses where they
   are named, instead of reducing only the tests whose result cannot
   change with the width (which would have touched one test). Not left
   to the card on the net, which is wide enough already.
2. **Five widths for what is inside the text column** (agent's
   measurement, user's decision). Measured on the hybris PK article and
   the curriculum: the column grows with the screen from 304 to 584
   pixels up to about 600, stays at 584 up to 767 (text 18 pixels), and
   at 704 from 768 up (text 20 pixels); table forms and code toggles
   follow the column. So 659 and 767 give the same result inside the
   column, and so do the seven widths from 768 to 1920. The column's
   checks keep 320, 393, 412, 767 and 768: readability, content
   layout, home page boxes, code toggle, table forms and viewer rows;
   the table cells, at phone widths, keep 320, 393, 412, 767. The
   keyboard test of the record viewer runs at 320 only. The header (12
   widths) and accessibility (3 bands) stay as they are. Known limit,
   as today: the widths between 412 and 600, where the column grows,
   are sampled at three points only.
3. **The text spacing test is split** (agent's proposal, user's
   decision): every page at the five widths for everything outside the
   header; the header with the reader's spacing moves into the header's
   own test, at 12 widths on the invented 50-character titles. **On one
   condition** (user): the 50-character limit is first enforced in the
   build ([card](../wip/enforce-the-title-length-limit-in-the-build.md)),
   since otherwise a longer real title would be seen by no test.
