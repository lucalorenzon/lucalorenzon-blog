# Show the table in the style guide

- **Opened:** 2026-11-15, by the user, as the third page of the style
  guide in the card [build a design system](build-a-design-system.md),
  the second one about a component, after
  [the code block](../done/2026-11-15_show-the-code-block-in-the-style-guide.md);
  in Now.
- **Status:** being worked on, since 2026-11-15.

## Why

The tables are the largest part of what still ties the browser tests to
the articles. Every table test runs on the two articles that have
tables, the hybris article and the After Save Event article, and two of
them name the hybris article and guard it: an edit made to that article
for the text can stop the build. A page of the style guide, with cases
written for the purpose, gives the table's tests a place of their own,
as the code block's page did for the code, and the articles go back to
being content as far as tables are concerned. It is also one of the
pages the style guide needs before it is published.

## Known from the start

- **What the tests check today** (rows in `end2end/README.md`):
  - `table-forms.spec.ts`, on every built page with tables, found by
    walking the built site (`TABLE_ADDRESSES` in `support.ts`), at the
    column widths: exactly one of the two forms shows, the table is not
    wider than its column, the page does not scroll sideways; in the
    record viewer every row is level with its label and the number row
    reads "k / n"; at 320, Tab reaches the viewer and the arrow keys move
    it by one record.
  - `table-forms.spec.ts`, on the hybris article by name
    (`SINGLE_TABLE`): its six-column table is a table at 768 and a
    record viewer at 767.
  - `table-forms.spec.ts`, on the hybris article by name (`AXE_PAGE`),
    at 320 and 768: axe limited to the tables, and the contrast in both
    schemes, with a guard that stops the build if the article no longer
    shows both forms at 320.
  - `tables.spec.ts`, on every built page, tables or not, at the four
    phone widths: a cell with no space in it, such as "63–44", stays on
    one line. 20 of its 28 runs are on pages with no table.
- **The form changes at a threshold the build estimates:** from the
  characters' widths in Literata (`src/table_widths.rs`), so a case
  written to change form at a given width depends on that estimate.
- **The component does not change:** the table and its record viewer are
  written by the Markdown converter (`src/markdown.rs`,
  `table_component`); this card shows them and moves their tests, it
  does not redesign them.
- **What it touches, roughly:** a new page of the style guide (its
  generator in `src/style_guide/`, its text in `content/style-guide/`),
  listed by the index and the sitemap; `table-forms.spec.ts` and
  probably `tables.spec.ts`; their rows in `end2end/README.md`.
- **The page is covered by the net** with no test added, like the other
  pages of the style guide: it is in the sitemap.

## Questions

To be discussed one at a time; if they turn out too many for one card,
the card is cut.

1. Which cases the page needs, written for the purpose, so that each
   test finds what it checks without a guard.
2. Where the general tests run: on the style guide's page only, as for
   the code block, or on the articles too.
3. How the change of form at a given width is checked without the
   hybris article, given that the threshold is an estimate.
4. Whether `tables.spec.ts` becomes a check of the table component, run
   once on its cases (the question left open by the card
   [build a design system](build-a-design-system.md)).
5. The texts for the readers: placeholders written by the agent and
   texts written by the user, as for the pages already made, or
   otherwise.
