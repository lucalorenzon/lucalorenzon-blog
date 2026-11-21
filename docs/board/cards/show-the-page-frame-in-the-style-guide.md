# Show the page frame in the style guide

- **Created:** 2026-11-21
- **Status:** Open

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md),
with the user's agreement. The spike found that a component does not
declare what changes in it at a breakpoint: its page in the style
guide shows it at each of its thresholds, and a test checks that
what it shows is true. So the page of the breakpoints, which was to
list the changes of the whole site
([show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md)),
was superseded. The page frame (header, logo, title, footer) is the
only part of the site left on the screen's breakpoints, and had no
page to show its changes.

## What is known

- **What the frame changes at each breakpoint** (`src/page_frame.rs`):
  at 768 pixels the written logo gives way to the logo drawn in
  characters; at 1280 pixels the header becomes one row, with the
  title beside the logo. Its text sizes are to grow within the site's
  ranges
  ([card](let-the-frame-text-grow-within-the-injected-ranges.md)).
- **Showing the frame at several widths on one page** would
  duplicate its header, footer and main heading, as the spike's
  experiment duplicated `<main>`. An `iframe` has a window of its own,
  so the screen's breakpoints switch at its width, and is a document
  of its own, so it duplicates nothing in the page that holds it.
  Proposed by the agent in the spike, not tried.
- **Decisions of the breakpoints page that still hold** (see that
  card): the style guide is published with its gaps declared; the
  components not yet cut in the code are declared as a limit of the
  site as it is; the pages the user wishes for later are the title,
  the logo, the header, the footer, the text column, the list of
  articles and the article's box in that list.
- **The breakpoints sort the devices into groups**, three today, and
  the breakpoints page was to show the devices of each group, taken
  from the list the browser tests run on.

## Questions to answer

The ones known when the card was written; there may be more. The
last three come from the breakpoints page's card, still open when it
was superseded.

- **Whether the `iframe` passes the accessibility checks**, the
  build's and axe's best practices, and how wide a sample may be on a
  320-pixel screen without scrolling the page sideways.
- **What the page puts to the test.**
- **Whether the frame is one page, or the header, the logo and the
  title are pages of their own**, whose boundaries were still to be
  drawn; the logo is under review.
- **Whether the list of devices can come from Firefox's**, the one
  the user reads in its responsive design mode: its address, its
  licence, and whether the tests' list should follow it.
- **What the tests' 320 is called**, since it is not the iPhone SE
  the user looks at.
- **How the index declares the pages to come.**

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike whose finding this page applies.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the page this one replaces, and the decisions it inherits.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  what the frame's text becomes.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the other component page born of the spike.
- [Build a design system](build-a-design-system.md): the card
  the style guide belongs to.
- [Review the logo](review-the-logo.md): a page of the logo would
  describe something that may change.
