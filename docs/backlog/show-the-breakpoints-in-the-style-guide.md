# Show the breakpoints in the style guide

- **Created:** 2026-11-18
- **Status:** Open

## Why

The user opened this card on 2026-11-18, while cutting the last page
of the style guide before it is published (card
[build a design system](../wip/build-a-design-system.md)). The page
explains to the reader how the site adapts to the screen width: at
which widths it changes, and what changes there. It is also the
entry point to the breakpoints, whose values it reads from the place
the card
[read the breakpoints from the configuration](../wip/read-the-breakpoints-from-the-configuration.md)
gives them.

A breakpoint alone says nothing: it is the width where something
changes. So the page lists the changes too, and leads to the pages
where the components that change are shown.

## What is known

- **What changes at each breakpoint** (checked on 2026-11-18), and
  which page of the style guide speaks of it today:

  | Width | What changes | Page that speaks of it |
  |---|---|---|
  | 768 | The written logo becomes the logo drawn in characters | None; the fonts page names the logo, not the change |
  | 768 | The page title grows from 24 to 32 pixels | None |
  | 768 | The text of the column grows from 18 to 20 pixels, and the column widens | The tables, in passing; not the text page |
  | 768 | The footer's text grows | None |
  | 1280 | The header becomes one row, with the title beside the logo | None |

- **Most changes have no page to lead to.** The header and the footer
  are the frame every page shares, and the style guide has not shown
  them as components so far.
- **Decided by the user on 2026-11-18: published with the gaps
  declared.** The style guide is published with this page as its
  last, as planned. For each change whose component has no page yet,
  this page says what happens and that a page of its own will come;
  the index lists the pages to come. Why: publishing early tells
  which details the readers value, which cannot be known before; and
  the real launch of the site comes later, with more articles.
  Rejected: adding the header and footer pages before publishing,
  which would delay it by two pages, one of them tied to the logo's
  review still open.
- **The pages the user wishes for later**, named on 2026-11-18: the
  title, the logo, the header, the footer, the text column, the list
  of articles and the article's box in that list. No card yet. The
  title and the logo are parts of the header, and change with it at
  1280 pixels, so the boundary between those pages is still to be
  drawn.
- **The breakpoints sort the devices into groups**, three today
  (said by the user on 2026-11-18). The page shows, for information,
  the names of the devices in each group, taken from the list of
  devices the browser tests run on, which stays apart from the
  breakpoints and does not move with them (card
  [read the breakpoints from the configuration](../wip/read-the-breakpoints-from-the-configuration.md)).
- **The smallest device of the tests is not the one the user looks
  at.** The tests' "iPhone SE" is 320 by 568 pixels, the first model;
  the user checks the smallest screen on the later iPhone SE, 375 by
  667, in Firefox's responsive design mode. The width 320 has a
  reason of its own: WCAG 1.4.10 asks that text need not scroll
  sideways at 320 pixels.
- **The code block and the table** change form with the width of
  their own container, not the screen's; the page may say so and lead
  to their pages, which explain it.

## Questions to answer

The ones known when the card was written; there may be more.

- **Who writes the list of changes**: the author in the page's
  Markdown, as for the other pages, or the build from the components'
  classes. The second cannot get out of step with the code, the first
  can say why each change happens.
- **What the page puts to the test**, as every page of the style guide
  has its cases checked by the browser tests: perhaps only that the
  widths it shows are those of the configuration.
- **Whether the list of devices can come from Firefox's**, the one
  the user reads in its responsive design mode: whether it has a
  stable public address, under which licence, and whether the tests'
  list should follow it. Not checked.
- **What the tests' 320 is called**, since it is not the iPhone SE
  the user looks at.
- **How the index declares the pages to come**, and whether those
  pages get their cards now.

## Dependencies

- [Read the breakpoints from the configuration](../wip/read-the-breakpoints-from-the-configuration.md),
  which gives the page the values to read.

## References

- [Build a design system](../wip/build-a-design-system.md): the card
  this page completes before the style guide is published.
- [Read the breakpoints from the configuration](../wip/read-the-breakpoints-from-the-configuration.md):
  the values the page reads.
- [Review the logo](review-the-logo.md): a page of the logo would
  describe something that may change.
- [Write an article on the style guide](write-an-article-on-the-style-guide.md):
  the readers' way into the style guide, published with it.
- [Show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md):
  the last page done, the model for this one.
