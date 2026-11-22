# Read the column text's range from the configuration

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md),
as its second increment. That card makes the column's text grow
smoothly with the space of the page, between a smallest and a largest
size written in the stylesheet. Here the range moves to `config/`, as
the breakpoints and the fonts did, so that the site injects it from
one setting and the build and the tests read the same values.

As long as the range is written only in the stylesheet, a browser test
that checks the size stays within it has to repeat its values, and a
change of the range is a change of the stylesheet and of the tests
together.

## What is known

- The breakpoints and the fonts already go this way: read from
  `config/` by the build and written for the stylesheet into
  `target/generated/` (`src/style_guide/breakpoints.rs`,
  `src/style_guide/fonts.rs`); the breakpoints are read by the browser
  tests too.
- How the range is written in the stylesheet is decided in the first
  increment, not known yet.

## Questions to answer

The ones known when the card was written; there may be more.

- **How the range is written in `config/`**: the smallest and largest
  size, and the widths at which they are reached, in a file of its own
  or in a structured configuration file.
- **Whether it waits for the card on a structured configuration
  file**, now that `config/` would hold a fourth setting.
- **Whether the frame's ranges go into the same setting**, once the
  card on the frame's text has written them.

## Dependencies

- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the first increment, to be done first.
- [Use a structured configuration file](use-a-structured-configuration-file.md),
  perhaps, depending on where the range lives.

## References

- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the card this one was cut from.
- [Use a structured configuration file](use-a-structured-configuration-file.md):
  a possible place for the range.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  the same ranges, for the header and the footer.
- [Read the breakpoints from the configuration](read-the-breakpoints-from-the-configuration.md):
  the same path, taken by the breakpoints.
