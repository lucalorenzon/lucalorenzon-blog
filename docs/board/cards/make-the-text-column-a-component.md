# Make the text column a component

- **Created:** 2026-11-19
- **Status:** Closed
- **Closed:** 2026-11-19

## Why

The user opened this card on 2026-11-19, while answering the
questions of the card
[show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md).
That page lists what changes at each breakpoint by asking each
component what changes in it. The text column changes at 768 pixels,
the most visible change of all: its text grows from 18 to 20 pixels
and the column widens. But the column was no component, so nothing
could answer for it, and the change would have been missing from the
list.

The column is also the first of the components the user wishes to
show in the style guide
([show the text column in the style guide](show-the-text-column-in-the-style-guide.md)),
and a page of a component needs a component to show.

## What is known

- **The column was one element written by hand in eight files**
  (checked on 2026-11-19): `<main class="long-text-column
  md:long-text-large">`, in `src/home_page.rs`,
  `src/article_page.rs`, `src/curriculum_page.rs`,
  `src/about_page.rs`, `src/licence_page.rs`,
  `src/not_found_page.rs`, `src/style_guide/index_page.rs` and
  `src/style_guide/navigation.rs`. The change was the same in all
  eight.
- **It is filled in two ways.** The article, the curriculum, the about
  page and the licence page give it the HTML converted from their
  Markdown; the home page, the "page not found", the style guide's
  index and its other pages give it other components as children.
- **The name** is the user's: `one_column_text`, so the component is
  `OneColumnText`.
- **It is a refactor**: the published HTML stays as it is, byte for
  byte. Saying what changes at each breakpoint is not part of it: that
  comes with the breakpoints page, which brings the way components
  declare it.
- **No test checked the column's behaviour** before this card
  (checked on 2026-11-19). The browser tests ran at the column's
  widths (`COLUMN_WIDTHS` in `end2end/tests/support.ts`) to check what
  sits inside it, and no unit test read the classes of the `<main>`.
  When the card was written it was supposed, wrongly, that the browser
  tests already checked the column's width.

## Questions to answer

- **What guards the refactor**: which tests fix the column's behaviour
  before the structure is touched, and which have to be added.

  **Answered by the user on 2026-11-19.** The column's behaviour, not
  each page that uses it. Unit tests check that the component writes
  the right classes, for each of the two kinds of content. A browser
  test checks the column's behaviour at the breakpoints, written
  before the structure is touched, on the style guide's text page,
  whose text is written for the style guide; it moves to the column's
  own page when that page is made
  ([show the text column in the style guide](show-the-text-column-in-the-style-guide.md)).
  Why: no test checked that behaviour (the widths and the text sizes
  were written in a comment of `end2end/tests/support.ts`, measured
  once and never asserted), and the refactor needs a test that passes
  before and after it. Rejected: a test on a page of content, such as
  the licence or an article, which would bring back the pages of
  content used as test pages that the design system took away
  ([ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  and a test for every page that uses the column, which tests the
  pages instead of the component.
- **How the component takes its two kinds of content**, the Markdown's
  HTML and children, without changing the HTML it writes.

  **Proposed by the agent, agreed by the user on 2026-11-19.** The
  component takes either the HTML converted from the Markdown or other
  components as children, and writes the same `<main>` as before; the
  detail was left to the code. In the code: an optional property
  `html`, written as the element's inner HTML, and optional children,
  used when no `html` is given.
- **Where the module lives**: in `src/` beside the frame, since every
  kind of page uses it, or elsewhere.

  **Proposed by the agent, agreed by the user on 2026-11-19.**
  `src/one_column_text.rs`, beside `src/page_frame.rs`: every kind of
  page uses it, not only the style guide.

## Acceptance tests

Agreed with the user on 2026-11-19; all passed on 2026-11-19.

1. **Unit, content from the Markdown.** The component writes
   `<main class="long-text-column md:long-text-large">` holding the
   HTML it was given, unchanged.
2. **Unit, content from other components.** The same element, with
   the same classes, holding the children.
3. **Browser, on the style guide's text page, at the column's widths**
   (the three phones, 767 and 768 pixels). Below 768 pixels the text
   is 18 pixels and the column fills the screen up to its maximum of
   60 characters, centred. From 768 pixels the text is 20 pixels, and
   the maximum grows with it. Written, and passing, before the
   structure is touched; passing again after.
4. **The eight files use the component**: no
   `<main class="long-text-column` is left written by hand in the
   code.
5. **The site built before and after the refactor is the same, file
   by file.** A check made once, not a test that stays: the tests
   above check the component, not each page, and this proves no page
   changed.
6. **The new browser test has its row in `end2end/README.md`**, and
   every check of the pipeline passes: formatting, clippy, unit tests,
   type check and browser tests.

## Recap

Done in commit `2b77e75`, written by a sub-agent on Sonnet 5.5 and
checked by the main session.

- **The component**, `OneColumnText` in `src/one_column_text.rs`,
  with its two unit tests; the eight files use it.
- **The browser test**, `text-column.spec.ts`, with its row in
  `end2end/README.md`. It measures the maximum in the column's own
  font, with a probe 60 characters wide, rather than writing pixels
  that depend on the font; the breakpoint comes from
  `config/breakpoints.txt`. It passed on the code before the refactor
  and after it.
- **The checks**: the site built before and after the refactor was
  compared file by file and found the same; formatting, clippy, 423
  unit tests, the type check and the browser tests (275 passed, one
  skipped because it runs on the live site only) passed.

### What the browser test measured

| Screen width | Text | Column | Maximum, 60 characters |
|---|---|---|---|
| 320 | 18 | 320 | 600 |
| 393 | 18 | 393 | 600 |
| 412 | 18 | 412 | 600 |
| 767 | 18 | 600 | 600 |
| 768 | 20 | 720 | 720 |

The figures of 584 and 704 pixels written in
`end2end/tests/support.ts` are the width of the column's text,
without its 8 pixels of padding on each side; the comment there now
says so.

## Cautions

- **A page that gave the component both the HTML and children would
  lose the children**, without any error: the HTML wins. No page does
  it today, and the risk was judged low, so the code was left as it
  is.
- **The browser test lives on the style guide's text page** until the
  column has its own page, where it is meant to move.

## References

- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the card that waits for this one.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the page of this component, which comes next.
- [Build a design system](build-a-design-system.md): the card
  the style guide belongs to.
- [ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md):
  each component tested once, on cases written for the purpose.
