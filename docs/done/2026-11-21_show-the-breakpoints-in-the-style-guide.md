# Show the breakpoints in the style guide

- **Created:** 2026-11-18
- **Status:** Closed
- **Closed:** 2026-11-21

## Why

The user opened this card on 2026-11-18, while cutting the last page
of the style guide before it is published (card
[build a design system](../wip/build-a-design-system.md)). The page
explains to the reader how the site adapts to the screen width: at
which widths it changes, and what changes there. It is also the
entry point to the breakpoints, whose values it reads from the place
the card
[read the breakpoints from the configuration](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md)
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
  [read the breakpoints from the configuration](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md)).
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

  **Answered by the user on 2026-11-18.** Both, each for what it
  does best. The build finds what changes, so the list cannot get out
  of step with the code; the author's Markdown explains it; the build
  puts the two together. Made precise in the answer below. Why: a
  class carries only its own name, which tells the reader neither what
  happens nor which page of the style guide shows it, while the
  decision to publish with the gaps declared asks the page to say both.
  Rejected: the list written by the author alone, which nothing would
  check when a class is added or moved; and the list generated from
  the classes alone, which would show the reader technical names with
  no explanation and no links.
- **What one piece of the Markdown explains, and what ties it to the
  classes**: one class, or one change as the reader sees it. They
  differ: at 768 pixels the reader sees one change, the written logo
  giving way to the logo drawn in characters, made in the code by two
  classes on two elements (`md:not-shown` and `md:art-logo-left`).
  Added on 2026-11-18, from the answer above.

  **Answered by the user on 2026-11-18.** One component. The meaning
  of a breakpoint's classes is known by the component that uses them,
  so the build asks each component what changes in it at each
  breakpoint, and groups the answers by component. The page's
  Markdown is cut in pieces, each marked with the component it speaks
  of, as `fonts.md` is cut by the fonts' headings. So the build can
  join each piece of text to a component that changes, and write a
  text on the breakpoints that links to the page of each component,
  where its look at each breakpoint is explained. Why: the component
  is where the meaning of its classes lives, and the reader sees a
  change of a component, not of a class. Rejected: one explanation per
  class, which would show the reader two entries for what they see as
  one change; and the tie between classes and changes written in the
  Markdown, away from the code that knows it.
- **What to do while the components the page needs are not components
  in the code.** The header, the logo, the title and the footer all
  live in one Leptos component, `PageFrame` (`src/page_frame.rs`); the
  text column is no component at all, its class written by hand in the
  `<main>` of every kind of page, in eight files. Asked today, one
  component would answer for four changes, and the column would not
  answer. Added on 2026-11-18, from the answer above.

  **Answered by the user on 2026-11-19.** The page declares it as a
  limit of the site as it is today, a first version kept small on
  purpose, and says that a refactor of the code into components is
  planned, listing the components to come. Why: as for the pages that
  do not exist yet, publishing with the gap declared tells the reader
  where the site stands, and does not hold the style guide back for a
  refactor. Rejected: splitting the code into components before the
  page, which this card did not foresee.
- **What to do with the text column, for which no component would
  answer.** Its change at 768 pixels, the text growing from 18 to 20
  pixels and the column widening, is the most visible one, and would
  be missing from a list the build generates, against the first
  answer above. Added on 2026-11-19, from the answer above.

  **Answered by the user on 2026-11-19.** The text column becomes a
  component, `OneColumnText`, before this page, and gets a page of its
  own in the style guide, which explains its look at each breakpoint;
  the build links the breakpoints page to it. Cut in two cards:
  [make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md),
  a refactor that keeps the published HTML as it is, then
  [show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md).
  This changes, knowingly, the decision of 2026-11-18 to add no page
  of a component before publishing: the style guide waits one page
  more, for the change that would otherwise be missing. Rejected:
  leaving the column out of the list until the refactor of the other
  components.
- **Whether the page is still needed**, once the spike on the column
  found how a component shows what changes in it. Added on
  2026-11-21, at the spike's close.

  **Answered by the user on 2026-11-21: no, the page is superseded.**
  The spike
  ([try a breakpoint declaration on the text column](../wip/try-a-breakpoint-declaration-on-the-text-column.md))
  found that a component does not declare what changes in it: its own
  page in the style guide shows it at each of its thresholds, and a
  test checks that what it shows is true. The column moves off the
  screen's breakpoints: its text grows within a range the site
  injects, following its container. Only the page frame's changes of
  layout stay on the screen's breakpoints, and they get a page of the
  frame. So a page listing the changes of the whole site would repeat
  what the components' pages show. This supersedes the answers above
  on who writes the list of changes and what ties it to the classes:
  nothing is listed. Why the card is closed rather than deleted, as a
  card split in smaller ones would be: it was worked on, its
  questions were answered, and the work it foresaw is not done as a
  consequence of that work; the new cards were not cut from it.

## Dependencies

- [Read the breakpoints from the configuration](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md),
  which gives the page the values to read.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md)
  and [show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md),
  so the column can say what changes in it, and has a page to link to.

## Recap

No page was written. The card answered its questions from 2026-11-18
to 2026-11-19, and was then superseded by the finding of the spike on
the text column, closed the same day. What still holds of its
decisions passed to the card
[show the page frame in the style guide](../backlog/show-the-page-frame-in-the-style-guide.md):
the style guide published with its gaps declared, the components not
yet cut in the code declared as a limit of the site, and the pages
the user wishes for later.

## What it leaves

The questions still open when the card was superseded, all moved to
[show the page frame in the style guide](../backlog/show-the-page-frame-in-the-style-guide.md):

- what the page puts to the test;
- whether the list of devices can come from Firefox's;
- what the tests' 320 is called;
- how the index declares the pages to come.

## References

- [Try a breakpoint declaration on the text column](../wip/try-a-breakpoint-declaration-on-the-text-column.md):
  the spike whose finding superseded this page.
- [Show the page frame in the style guide](../backlog/show-the-page-frame-in-the-style-guide.md):
  the page that replaces this one, and takes its open questions.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md):
  the first of the two cards this page waits for.
- [Show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md):
  the second, the page this one links the column to.

- [Build a design system](../wip/build-a-design-system.md): the card
  this page completes before the style guide is published.
- [Read the breakpoints from the configuration](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md):
  the values the page reads.
- [Review the logo](../backlog/review-the-logo.md): a page of the logo would
  describe something that may change.
- [Write an article on the style guide](../backlog/write-an-article-on-the-style-guide.md):
  the readers' way into the style guide, published with it.
- [Show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md):
  the last page done, the model for this one.
