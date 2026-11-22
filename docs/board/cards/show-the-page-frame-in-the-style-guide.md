# Show the page frame in the style guide

- **Created:** 2026-11-21
- **Status:** In work

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

- **What the frame changes at each breakpoint** (`src/page_frame.rs`,
  checked by the agent on 2026-11-22): one change only, at 1280
  pixels, where the header goes from two rows (the logo and the menu's
  room above the line, the title below it) to one row, with the title
  between the logo and the menu's room. Until the logo was reviewed
  ([card](review-the-logo.md), closed on 2026-11-22) there was a
  second change, at 768 pixels, where the written logo gave way to the
  logo drawn in characters; the new logo, a round icon with the name
  beside it, is the same at every width. Its text grows within ranges of its own over
  the page's widths from 320 to 1366 pixels, written in the
  stylesheet (`style/input.css`): the title from 24 to 32 pixels, the
  footer's text from 16 to 18; the section headings are 1.2 times the
  column's text
  ([card](let-the-frame-text-grow-within-the-injected-ranges.md),
  done on 2026-11-22).
- **The frame writes the text column** since 2026-11-21: the
  `<main>`, with the look of the text, went back to it from the
  component `OneColumnText`, removed
  ([decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md)).
  The column's own page
  ([show the text column in the style guide](show-the-text-column-in-the-style-guide.md))
  joined this one, decided by the user on 2026-11-21, at least until
  the user changes their mind. What was known there:
  - **What changes in the column** (checked by the browser test
    `text-column.spec.ts`): its text no longer jumps at 768 pixels; it
    grows on a straight line from 18 pixels at a 320-pixel page to 20
    pixels from 1366 up, written in the stylesheet
    ([card](let-the-column-text-grow-within-an-injected-range.md),
    done on 2026-11-22). Its maximum width, 60 characters of its own
    font, grows with its text. Below its maximum it fills the screen;
    it is always centred. Until 2026-11-22 the text was 18 pixels
    below 768 and 20 from 768, and the column 600 and 720 pixels wide
    with Literata.
  - **The browser test of the column moves here**: written on the
    style guide's text page, it moves to this page once it exists
    (agreed by the user on 2026-11-19, for the column's page).
  - **The style guide's text page** already shows the elements of the
    text inside the column
    ([card](show-the-text-elements-in-the-style-guide.md)), and the
    tables page names the column's width in passing.
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
- **Where the explanation of a component lives**: beside its code, or
  in the Markdown of `content/style-guide/` as for the other pages.
  Came from the column's page. The user's view on 2026-11-19: the
  documentation stays with the code, as a principle; the user had
  thought of a separate place, dedicated and linked to the component,
  to avoid one large file to open and scroll for every change; ready
  to revise this.
- **Where the boundary lies with the text page**, which shows what
  goes inside the column. Came from the column's page.
- **Whether the frame is one page, or the header, the logo and the
  title are pages of their own**, whose boundaries were still to be
  drawn. For the logo, the same question is asked by the card
  [decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md),
  opened on 2026-11-22 once the logo was reviewed: the answer here
  answers it there too.

  **Answered by the user on 2026-11-22, with the agent.** One page, the
  frame shown whole at its widths; its parts are not cut into
  components of their own for now. The site is static HTML today, and
  the user expects logic soon: background images, a header that is
  transparent and turns opaque when the text passes under it,
  parallax for depth, the title and the logo's name moving when the
  title reaches the header, the articles' cards scrolling sideways,
  perhaps two columns on wide screens. Those behaviours are expected
  to give the components their shape; cutting them now would draw
  boundaries from the wrong information. The agent agreed, adding
  that a component is cut along the parts that change together, and
  those behaviours tie the logo, the title and the header together:
  cut now along the static layout, they would likely be put back
  together, as the text column was. The agent added one point: the
  page's tests check what the visitor sees, not the code's
  boundaries, so they stay true after a later cut and guard it.
- **Whether the list of devices can come from Firefox's**, the one
  the user reads in its responsive design mode: its address, its
  licence, and whether the tests' list should follow it.
- **What the tests' 320 is called**, since it is not the iPhone SE
  the user looks at.
- **How the index declares the pages to come.**

## Dependencies

- **Waits for the logo**
  ([review the logo](review-the-logo.md)), decided by the user on
  2026-11-22: the logo's change at 768 pixels, one of the two changes
  this page would show, is expected to go, so the page is made once
  the logo is settled. Met on 2026-11-22: the logo's card and the
  card that hid the header's line under the logo's name
  ([card](hide-the-header-line-under-the-logo-name.md)) are closed,
  and the change at 768 pixels is gone.

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike whose finding this page applies.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the page this one replaces, and the decisions it inherits.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  what the frame's text becomes.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the other component page born of the spike, joined to this one.
- [Decide whether to cut the text column again](decide-whether-to-cut-the-text-column-again.md):
  the card that put the column back into the frame.
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  what the column's text becomes.
- [Show the text elements in the style guide](show-the-text-elements-in-the-style-guide.md):
  the text page, the closest to the column.
- [Build a design system](build-a-design-system.md): the card
  the style guide belongs to.
- [Review the logo](review-the-logo.md): a page of the logo would
  describe something that may change; closed on 2026-11-22.
- [Hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md):
  the last change to the header before this page.
- [Decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md):
  the same question as this card's on the frame's pages, from the
  logo's side.
