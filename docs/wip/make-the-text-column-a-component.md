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
- **How the component takes its two kinds of content**, the Markdown's
  HTML and children, without changing the HTML it writes.
- **Where the module lives**: in `src/` beside the frame, since every
  kind of page uses it, or elsewhere.

## Acceptance tests

To be agreed with the user before development starts.

## References

- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  the card that waits for this one.
- [Show the text column in the style guide](../backlog/show-the-text-column-in-the-style-guide.md):
  the page of this component, which comes next.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
