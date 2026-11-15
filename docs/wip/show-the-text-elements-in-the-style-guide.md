# Show the text elements in the style guide

- **Opened:** 2026-11-15, by the user, as the fourth page of the style
  guide in the card [build a design system](build-a-design-system.md),
  after [the table](../done/2026-11-15_show-the-table-in-the-style-guide.md).
  The card's typography page was cut in two by the user: this one, the
  elements of the text, comes first; the fonts have
  [a card of their own](../backlog/show-the-fonts-in-the-style-guide.md).
  In Now.
- **Status:** being worked on, since 2026-11-15.

## Why

The text checks are the last browser tests that use an article as a
test page. They run on the After Save Event article, chosen as the
worst case because it has every kind of element they look at, and a
guard stops the build if an edit made for the text takes one of them
away. A page of the style guide, with cases written for the purpose,
gives the text checks a place of their own, as the code block's and the
table's pages did, and the article goes back to being content. It is
also one of the pages the style guide needs before it is published.

Why the elements of the text come before the fonts: they free the
article; the fonts are mostly to be shown and move few tests.

## Known from the start

- **What the tests check today on the article** (rows in
  `end2end/README.md`), at the column widths (`COLUMN_WIDTHS`), each
  after the guard (`expectWorstCaseStillWorst` in `support.ts`):
  - `readability.spec.ts`, visual presentation (WCAG 1.4.8): line
    height of paragraphs and list items at least 1.5 times the font
    size, space between consecutive paragraphs at least 1.5 times the
    line height, no justified text;
  - `readability.spec.ts`, text spacing (WCAG 1.4.12): with the
    reader's spacing, consecutive blocks do not overlap, no element cuts
    off its content, the content ends at least 32 px above the footer;
  - `content-layout.spec.ts`: section headings larger than the body
    text, no overlap of consecutive headings, paragraphs and list items,
    the content clear of the footer;
  - `content-layout.spec.ts`, at one width and in both schemes: section
    and role headings in a colour other than the body text's.
- **Two tests name the article without the guard**, in
  `content-layout.spec.ts`: the numbered list shows its numbers and is
  indented and spaced like the bulleted list; the list markers take the
  amber accent while the item text keeps its colour.
- **The guard asks for more than text:** two consecutive paragraphs, a
  level-2 and a level-3 heading, a bulleted and a numbered list, and
  also a code block and a table. The check that nothing cuts off its
  content makes exceptions for the code's scroller and for the record
  viewer, which exist because the article has both.
- **A gap:** level-4 headings are only in the curriculum, so the text
  checks do not see them; they enter only the overlap check of the
  net.
- **Not part of this card:** the net's checks (the 80-character line,
  no sideways scroll), which run on every page and stay there; the
  home page's boxes test, in `content-layout.spec.ts`, which is about
  the home page.
- **The components do not change:** the text is written by the
  Markdown converter (`src/markdown.rs`); this card shows the elements
  and moves their tests, it does not redesign them.
- **What it touches, roughly:** a new page of the style guide (its
  generator in `src/style_guide/`, its text in `content/style-guide/`),
  listed by the index and the sitemap; `readability.spec.ts`,
  `content-layout.spec.ts` and the guard in `support.ts`; their rows in
  `end2end/README.md`.
- **The page is covered by the net** with no test added, like the other
  pages of the style guide: it is in the sitemap.

## Questions

To be discussed one at a time; if they turn out too many for one card,
the card is cut.

1. Which cases the page needs, written for the purpose, so that each
   test finds what it checks without a guard; level-4 headings
   included.
2. Whether the text checks still need a code block and a table beside
   the text, or the code block's and the table's pages already cover
   what they check there.
3. Whether the elements inside a line (bold, italic, inline code,
   links) belong to this page.
4. Where the text checks run: on the style guide's page only, as for
   the code block and the table, or on the articles too.
5. The texts for the readers: placeholders written by the agent and
   texts written by the user, as for the pages already made, or
   otherwise.
