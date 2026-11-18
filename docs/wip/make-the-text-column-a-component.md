# Make the text column a component

- **Created:** 2026-11-19
- **Status:** In work

## Why

The user opened this card on 2026-11-19, while answering the
questions of the card
[show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md).
That page lists what changes at each breakpoint by asking each
component what changes in it. The text column changes at 768 pixels,
the most visible change of all: its text grows from 18 to 20 pixels
and the column widens. But the column is no component, so nothing
could answer for it, and the change would be missing from the list.

The column is also the first of the components the user wishes to
show in the style guide
([show the text column in the style guide](../backlog/show-the-text-column-in-the-style-guide.md)),
and a page of a component needs a component to show.

## What is known

- **The column is one element written by hand in eight files**
  (checked on 2026-11-19): `<main class="long-text-column
  md:long-text-large">`, in `src/home_page.rs`,
  `src/article_page.rs`, `src/curriculum_page.rs`,
  `src/about_page.rs`, `src/licence_page.rs`,
  `src/not_found_page.rs`, `src/style_guide/index_page.rs` and
  `src/style_guide/navigation.rs`. The change is the same in all
  eight.
- **It is filled in two ways.** The article, the curriculum, the about
  page and the licence page give it the HTML converted from their
  Markdown (`inner_html`); the home page, the "page not found", the
  style guide's index and its other pages give it other components as
  children.
- **The name** is the user's: `one_column_text`, so the component is
  `OneColumnText`.
- **It is a refactor**: the published HTML stays as it is, byte for
  byte. Saying what changes at each breakpoint is not part of it: that
  comes with the breakpoints page, which brings the way components
  declare it.
- **The browser tests already check the column's width** at every
  width where it changes (`COLUMN_WIDTHS` in `end2end/tests/support.ts`),
  and the unit tests of several pages read the `<main>` element.
  Whether they fix its HTML closely enough to guard the refactor is
  not checked yet.

## Questions to answer

The ones known when the card was written; there may be more.

- **What guards the refactor**: which tests fix the column's HTML on
  every kind of page before the structure is touched, and which have
  to be added.

  **Answered by the user on 2026-11-19.** The column's behaviour, not
  each page that uses it. Unit tests check that the component writes
  the right classes, for each of the two kinds of content. A browser
  test checks the column's behaviour at the breakpoints, written
  before the structure is touched, on the style guide's text page,
  whose text is written for the style guide; it moves to the column's
  own page when that page is made
  ([show the text column in the style guide](../backlog/show-the-text-column-in-the-style-guide.md)).
  Why: no test checks that behaviour today (checked on 2026-11-19:
  the widths of 584 and 704 pixels and the text of 18 and 20 pixels
  are written in a comment of `end2end/tests/support.ts`, measured
  once and never asserted), and the refactor needs a test that passes
  before and after it. Rejected: a test on a page of content, such as
  the licence or an article, which would bring back the pages of
  content used as test pages that the design system took away
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  and a test for every page that uses the column, which tests the
  pages instead of the component.
- **How the component takes its two kinds of content**, the Markdown's
  HTML and children, without changing the HTML it writes.

  **Proposed by the agent, agreed by the user on 2026-11-19.** The
  component takes either the HTML converted from the Markdown or other
  components as children, and writes the same `<main>` as today; how,
  in detail, is left to the code, and the unit tests check that the
  HTML stays the same.
- **Where the module lives**: in `src/` beside the frame, since every
  kind of page uses it, or elsewhere.

  **Proposed by the agent, agreed by the user on 2026-11-19.**
  `src/one_column_text.rs`, beside `src/page_frame.rs`: every kind of
  page uses it, not only the style guide.

## Acceptance tests

Agreed with the user on 2026-11-19.

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

## References

- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the card that waits for this one.
- [Show the text column in the style guide](../backlog/show-the-text-column-in-the-style-guide.md):
  the page of this component, which comes next.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
- [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md):
  each component tested once, on cases written for the purpose.
