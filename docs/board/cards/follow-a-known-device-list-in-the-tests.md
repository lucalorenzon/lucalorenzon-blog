# Follow a known device list in the browser tests

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
which had inherited two questions from the superseded card
[show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md).
They concern every browser test, not the frame's page, and publishing
the style guide does not need them. Left as they are, the tests keep
names of devices that are not the ones the user looks at, so a reader
of a test's failure, or of the tests' README, may picture another
screen than the one tested.

## What is known

- **The tests' list of devices** is written by hand in
  `end2end/tests/support.ts` (`DEVICES`): from "iPhone SE" at 320 by
  568 pixels to "iPad Pro 11 landscape" at 1194 by 834; the widths at
  the two sides of each breakpoint are added to it (`VIEWPORTS`).
  Checked by the agent on 2026-11-22.
- **The smallest device of the tests is not the one the user looks
  at** (from the breakpoints page's card, 2026-11-18). The tests'
  "iPhone SE" is the first model, 320 by 568 pixels; the user checks
  the smallest screen on the later iPhone SE, 375 by 667, in Firefox's
  responsive design mode. The width 320 has a reason of its own: WCAG
  1.4.10 asks that text need not scroll sideways at 320 pixels.
- **The list of devices stays apart from the breakpoints** and does
  not move with them
  ([read the breakpoints from the configuration](read-the-breakpoints-from-the-configuration.md)).

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the list of devices can come from Firefox's**, the one the
  user reads in its responsive design mode: where it is published, its
  licence, and whether the tests' list should follow it.
- **What the tests' 320 is called**, since it is not the iPhone SE the
  user looks at.

## References

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card this one was cut from.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the card where the two questions were first asked.
- [Read the breakpoints from the configuration](read-the-breakpoints-from-the-configuration.md):
  why the devices and the breakpoints are kept apart.
