# Move the text column's tests to the frame's page

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md).
The text column belongs to the frame since 2026-11-21, and its page
joined the frame's; its tests were to move there once the page
existed (agreed by the user on 2026-11-19). On 2026-11-22 the user
also decided that the section headings' size is a rule the frame
injects, while their look belongs to the text page. Left as it is,
the style guide's text page keeps testing the column it does not
show, and a change of the headings' ratio breaks the text page's test
instead of the frame's.

## What is known

- **The column's tests that move**, from the style guide's text page
  (names as in `end2end/README.md`, file `text-column.spec.ts`):
  `text column of the style guide's text page` and
  `the column of the text page is wider at 1366px`.
- **The column's test that stays**: `the size the text column passes on
  to the` code block and the table, on their pages, since only there do
  those components exist (decided on 2026-11-22).
- **The section heading split in two**, decided on 2026-11-22: on the
  frame's page, the rule injected, section headings 1.2 times the
  column's text at every width, measured on the page's own headings
  with no sample; on the text page, what belongs to the heading (its
  colour, the room around it, how it wraps, the underline of level
  four). The test `content layout of the style guide's text page`
  (`content-layout.spec.ts`) gives up the size and keeps the rest.
- **The ratio** comes from the frame's ranges
  ([card](let-the-frame-text-grow-within-the-injected-ranges.md)).
- **Each changed test's row** in `end2end/README.md` changes in the
  same commit.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the frame's page has enough text** for the column's tests,
  which on the text page run on paragraphs written to take more than
  one line at every width.

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  which builds the page the tests move to.

## References

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card this one was cut from, where the split was decided.
- [Move the frame's tests to its page](move-the-frame-tests-to-its-page.md):
  the other slice cut from the same card.
- [Let the page frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  where the headings' ratio was set.
- [Show the text elements in the style guide](show-the-text-elements-in-the-style-guide.md):
  the text page, which keeps the heading's look.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
