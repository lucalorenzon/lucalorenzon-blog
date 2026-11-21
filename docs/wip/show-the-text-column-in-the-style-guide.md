# Show the text column in the style guide

- **Created:** 2026-11-19
- **Status:** In work

## Why

The user opened this card on 2026-11-19, while answering the
questions of the card
[show the breakpoints in the style guide](../done/2026-11-21_show-the-breakpoints-in-the-style-guide.md).
The breakpoints page links each component that changes to the page
where its look at each breakpoint is explained. The text column
changes at 768 pixels, and has no such page: without it, the
breakpoints page would have nowhere to lead the reader for the most
visible change of the site.

## What is known

- **The column is a component**, `OneColumnText`
  (`src/one_column_text.rs`), since the card
  [make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md).
- **What changes in it** (checked on 2026-11-19 by the browser test
  `text-column.spec.ts`): at 768 pixels its text grows from 18 to 20
  pixels, and its maximum width, 60 characters of its own font, grows
  with it: 600 pixels below 768, 720 pixels from 768, with Literata.
  Below its maximum it fills the screen; it is always centred. The
  figures of 584 and 704 pixels in `end2end/tests/support.ts` are the
  width of its text, without its 8 pixels of padding on each side.
- **The browser test of the column moves here**: written on the style
  guide's text page by the refactor, it moves to this page once it
  exists (agreed by the user on 2026-11-19).
- **The page declares what changes at each breakpoint**, and is the
  one the build links from the breakpoints page (said by the user on
  2026-11-19). Superseded on 2026-11-21 by the spike's finding, see
  the first question below: the page shows the column, it does not
  declare its changes, and the breakpoints page is no longer written.
- **The style guide's text page** already shows the elements of the
  text, and the tables page names the column's width in passing.
- **This page is added before publishing**, changing knowingly the
  decision of 2026-11-18 to add no page of a component before then:
  see the breakpoints card.

## Questions to answer

The ones known when the card was written; there may be more.

- **How the component tells the build what changes in it**: the
  mechanism the breakpoints page needs. Whether it is built here,
  with the first component that uses it, or with the breakpoints
  page.

  **Answered by the user on 2026-11-19, in part.** Neither yet: the
  mechanism is tried first, in a spike on this component alone
  ([try a breakpoint declaration on the text column](../done/2026-11-21_try-a-breakpoint-declaration-on-the-text-column.md)),
  and this page waits for its finding. Why: the user's first idea of
  the mechanism, a trait the component implements, which also gives
  the build what it needs to compose the component's own page, was
  a brainstorm, and the user said they did not know where it led.
  Built here, it would also put two kinds of work in one card: the
  mechanism, for every future component, and the page of one
  component. Whether the mechanism is then built here or with the
  breakpoints page is left to the spike's finding.

  **Answered by the user on 2026-11-21, in the spike.** No mechanism
  is built: the component does not tell the build what changes in it.
  This page shows the column at several widths, and a test checks
  that what it shows is true. The column no longer changes at a
  screen breakpoint: its text grows smoothly within a range the site
  injects, following its container
  ([let the column's text grow within a range the site injects](../backlog/let-the-column-text-grow-within-an-injected-range.md)),
  so the page shows a range, not two sizes. The breakpoints page was
  superseded. Why, and the options weighed: the spike's card.
- **How the page shows the column at each breakpoint**, since the
  reader sees it at one width only. After the spike: at several
  widths, in containers of those widths, as its experiment did; each
  sample is then a `<main>`, which axe's best practices report, and a
  sample of fixed width scrolls the page sideways on a 320-pixel
  screen (see
  [decide whether to cut the text column again](../backlog/decide-whether-to-cut-the-text-column-again.md)).
- **Where the explanation of a component lives**: beside its code, or
  in the Markdown of `content/style-guide/` as for the other pages.
  Added on 2026-11-21, from the spike, where it stayed open. The
  user's view on 2026-11-19: the documentation stays with the code,
  as a principle; the user had thought of a separate place, dedicated
  and linked to the component, to avoid one large file to open and
  scroll for every change; ready to revise this.
- **Where the boundary lies with the text page**, which shows what
  goes inside the column.
- **What the page puts to the test.**

## Dependencies

- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md).
- [Try a breakpoint declaration on the text column](../done/2026-11-21_try-a-breakpoint-declaration-on-the-text-column.md),
  whose finding says how the component tells the build what changes
  in it.
- [Let the column's text grow within a range the site injects](../backlog/let-the-column-text-grow-within-an-injected-range.md),
  the behaviour this page shows.

## References

- [Try a breakpoint declaration on the text column](../done/2026-11-21_try-a-breakpoint-declaration-on-the-text-column.md):
  the spike this page waits for.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md):
  the component this page shows.
- [Show the breakpoints in the style guide](../done/2026-11-21_show-the-breakpoints-in-the-style-guide.md):
  the page that was to link to this one, superseded.
- [Let the column's text grow within a range the site injects](../backlog/let-the-column-text-grow-within-an-injected-range.md):
  the behaviour this page shows.
- [Decide whether to cut the text column again](../backlog/decide-whether-to-cut-the-text-column-again.md):
  bears on how a sample avoids a second `<main>`.
- [Show the page frame in the style guide](../backlog/show-the-page-frame-in-the-style-guide.md):
  the other component page born of the spike.
- [Show the text elements in the style guide](../done/2026-11-16_show-the-text-elements-in-the-style-guide.md):
  the text page, the closest to this one.
- [Build a design system](build-a-design-system.md): the card
  the style guide belongs to.
