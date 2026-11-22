# Move the text column's tests to the frame's page

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

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

  **Answered by the agent on 2026-11-22.** Yes. The column's width
  does not depend on the text it holds: it takes the whole width
  available up to 60 characters and is centred, whatever is inside
  (`long-text-column` in `style/input.css`). The column's tests
  measure the text's size, the width and the centring, and none needs
  long paragraphs; the guard that every case wraps serves the text
  page's layout tests, which stay there. The frame's page has three
  section headings and paragraphs up to 360 characters long anyway.
- **Whether the sub-section headings' size belongs to the frame too**,
  a question that came up during the work: the same rule of the
  stylesheet that sets the section headings at 1.2 times the column's
  text sets the sub-section headings (level three) at 1.1 times; no
  test checks that size today, and the frame's page had no heading of
  level three.

  **Answered by the user on 2026-11-22, with a doubt declared.** The
  user is not sure, but places them in the frame, and checks them in
  this card. To keep the rule agreed for the section headings,
  measured on the page's own headings with no sample, the frame's page
  gets a real sub-section explaining the headings' ratios, still part
  of the draft text.

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  which builds the page the tests move to.

## Acceptance tests

Agreed with the user on 2026-11-22.

1. The two tests of the column, the one at every width of the column
   and at Full HD and the one that wants it wider at 1366 pixels than
   at 843, open the frame's page instead of the text page; their names
   say the frame's page.
2. On the frame's page, at the same widths, every heading of level two
   is 1.2 times the column's text and every heading of level three 1.1
   times, within a few hundredths of a pixel; a guard fails the test if
   the page has no heading of either level, so a text rewritten
   without them does not let it pass empty.
3. The frame page's text has a heading of level three: a real
   sub-section that explains the headings' ratios, still marked as a
   draft.
4. The text page's layout test (`content layout of the style guide's
   text page`) loses only the rule that section headings are larger
   than the text, and keeps the rest; the headings' colours test does
   not change.
5. The test of the size the column passes on to the code block and to
   the table stays on their pages; the number of tests stays the same.
6. In the same commit, the tests' rows in `end2end/README.md` and the
   comments of the test files change with them.
7. Before the commit, the user sees the frame's page with the new
   sub-section, in screenshots on the iPhone SE and on the laptop.
8. Every check of the pipeline passes, as in `build.yml`.

## Recap

Done on 2026-11-22: the card taken into work with its acceptance tests
(`7b60f28`), the tests moved and the frame page's sub-section written
(`bd3abf3`).

- The two tests of the column in `text-column.spec.ts` open the
  frame's page and are named after it: `text column of the style
  guide's frame page` and `the column of the frame page is wider at
  1366px`. The address is one constant, `FRAME_PAGE` in `support.ts`,
  shared with the frame's tests moved by the sibling card.
- The column's test also checks, at every width of the column and at
  Full HD, that each heading of level two is 1.2 times the column's
  text and each of level three 1.1 times, within 0.05 pixels, with a
  guard for a page that has none. Measured in Chromium at 320 pixels:
  text 18, section heading 21.6, sub-section heading 19.8.
- The text page's layout test lost the rule that section headings are
  larger than the text, and kept the rest.
- The frame page's text has a sub-section, "The headings grow with the
  text", still a draft by the agent; a unit test requires a heading of
  level three on the page. The user saw it in screenshots on the
  iPhone SE and on the laptop before the commit.
- Every acceptance test passed: the suite still counts 332 tests (331
  passed, 1 skipped, the "page not found" check that runs on the live
  site only); formatting, clippy, the 434 unit tests and the type
  check pass.

## Cautions

- The sub-section headings' ratio was placed in the frame with a doubt
  declared by the user: should the text page turn out to be its place,
  only the check and its row move.
- The guard needs a heading of each level in the frame page's text:
  the author's rewrite of the draft must keep at least one of each, or
  the column's test fails on purpose.
- The space under a heading, 2.25 times the text before the next
  paragraph, looks larger than the space above a sub-section heading;
  it is the column's spacing of paragraphs, not changed by this card.

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
