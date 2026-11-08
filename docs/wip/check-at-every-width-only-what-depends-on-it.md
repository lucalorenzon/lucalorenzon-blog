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
