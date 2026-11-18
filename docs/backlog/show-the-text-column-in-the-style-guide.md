# Show the text column in the style guide

- **Created:** 2026-11-19
- **Status:** Open

## Why

The user opened this card on 2026-11-19, while answering the
questions of the card
[show the breakpoints in the style guide](../wip/show-the-breakpoints-in-the-style-guide.md).
The breakpoints page links each component that changes to the page
where its look at each breakpoint is explained. The text column
changes at 768 pixels, and has no such page: without it, the
breakpoints page would have nowhere to lead the reader for the most
visible change of the site.

## What is known

- **The column becomes a component first**, `OneColumnText`, in the
  card [make the text column a component](../wip/make-the-text-column-a-component.md).
- **What changes in it** (checked on 2026-11-18): at 768 pixels its
  text grows from 18 to 20 pixels and the column widens. Measured in
  the browser, the column grows with the screen from 304 to 584
  pixels up to about 600 pixels, stays at 584 pixels up to just below
  768, and is 704 pixels wide from 768 up (`end2end/tests/support.ts`).
- **The page declares what changes at each breakpoint**, and is the
  one the build links from the breakpoints page (said by the user on
  2026-11-19).
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
- **How the page shows the column at each breakpoint**, since the
  reader sees it at one width only.
- **Where the boundary lies with the text page**, which shows what
  goes inside the column.
- **What the page puts to the test.**

## Dependencies

- [Make the text column a component](../wip/make-the-text-column-a-component.md).

## References

- [Make the text column a component](../wip/make-the-text-column-a-component.md):
  the component this page shows.
- [Show the breakpoints in the style guide](../wip/show-the-breakpoints-in-the-style-guide.md):
  the page that links to this one.
- [Show the text elements in the style guide](../done/2026-11-16_show-the-text-elements-in-the-style-guide.md):
  the text page, the closest to this one.
- [Build a design system](../wip/build-a-design-system.md): the card
  the style guide belongs to.
