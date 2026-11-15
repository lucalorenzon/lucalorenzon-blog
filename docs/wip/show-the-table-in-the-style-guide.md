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

## Answers

- Which cases the page needs (question 1), in part. Answered by the
  user on 2026-11-15:
  - **Two tables serve the change of form, the keyboard and axe**: a
    simple one, two columns and five rows, which is a table at every
    width; and one of six columns and five rows, which is a table at
    768 and a record viewer at 767. Together they show both forms at
    767, as axe needs, and the second gives the keyboard five records.
  - **The rows level with their labels and the single-value cells get
    cases of their own**, each needing a particular content that
    another test's case should not depend on.
  - **The rows level with their labels: one table with the three
    situations that can break them**, a record viewer at least up to
    767: a value that wraps over several lines in one record and is
    short in another, a label that wraps in the label column, and an
    empty value, shown as a dash. If every value fitted on one line the
    rows would be level anyway, and the test would pass without
    checking anything.
  - **The single-value cells: two cases, one per kind of value the
    browser would break**: a range with an en dash, such as "63–44",
    after which the browser always breaks; and a word with a hyphen
    followed by a letter, such as "read-only". Each is a table at
    every phone width, down to 320, with the value in a column narrow
    enough to break it without the rule, beside a column with room to
    spare: the test sees only the cells of the table form, and a value
    that would not break anyway checks nothing. Rejected: a date such
    as "2026-11-15", which the browser does not break, since its hyphen
    is followed by a digit.
- Where the general tests run (question 2). Answered by the user on
  2026-11-15: **on the style guide's page only**, as for the code
  block; the articles stop being test pages for the tables. The risk
  the general tests cover today on the articles, a threshold wrong
  because of a table's own text, is covered by **texts at the limit of
  the estimate** written in the guide's tables. The user accepted the
  risk left: a future table with a content nobody foresaw is no
  longer checked, since the net's check on sideways scrolling leaves
  out tables in table form. The same price was paid for the code
  block, but there the threshold depends only on the number of
  characters, since every character of the code's font is as wide as
  the others; here it depends on which characters there are. What the
  estimate assumes, and a text at the limit can put to the test: bold
  is measured with the bold widths, inline code with the code font's,
  a character outside the measured set as the widest of its weight,
  and italic with the regular widths, an assumption only the browser
  can disprove. **The texts at the limit go in a table of their own**,
  answered by the user on 2026-11-15: added to the six-column case
  they could move its threshold out of its window, and in the simple
  two-column case, far from its limit, an error of the estimate would
  not show. **Its threshold falls just below the column's width at
  767**, measured on the built page, answered by the user on
  2026-11-15: an estimate too low shows only if the table is in table
  form with very little room at one of the widths tested, since with
  plenty of room it fits the column anyway; an estimate too high only
  makes the record viewer come a little early, which the build accepts.
  The price: the case sits a few pixels from its limit, tighter than
  the six-column case's window, and an edit to its text moves it
  easily. **No test of the test here either**, answered by the user on
  2026-11-15, as for the code block: the test expects the case to be a
  table at 767, no wider than the column, so an edit that pushes it
  past its limit fails the test; an edit that moves it away from its
  limit, leaving it plenty of room, is not caught, and is reviewed as
  a change to the tests. **Its texts: italic, inline code and bold**,
  answered by the user on 2026-11-15, the assumptions of the estimate
  that, if wrong, make it too low, which is what the test can catch:
  italic is measured with the regular widths; inline code as 0.85 of
  the text's size in the code font's fixed width; bold with its real
  widths, wrong only if the browser draws a different bold. Rejected:
  a character outside the measured set, counted as the widest of its
  weight, so always estimated too wide: it cannot fail the test.
- How the change of form is checked (question 3). Answered by the user
  on 2026-11-15: **at 767 and 768, for all three tests** (the change
  of form, the keyboard, axe with the contrast). At 768 the text grows
  from 18 to 20 px and the column widens, from about 632 px at 767
  (computed from the 704 px measured at 768, to be measured on the
  built page) to 704 px: the six-column case's threshold must fall in
  that window of about 70 px, not on one pixel. So the tests still
  check where the form changes, that is that the browser agrees with
  the build's estimate where the column widens. Rejected: 320 and 768,
  proposed first by the user, which made the case sturdier but no
  longer checked where the form changes.
- Whether `tables.spec.ts` becomes a check of the table component
  (question 4). Answered by the user on 2026-11-15, as it follows from
  question 2: **yes, it runs on the style guide's page only**, at the
  four phone widths, and states for each of its two cases (the range
  with an en dash, the word with a hyphen) that the value stays on one
  line. Today 20 of its 28 runs are on pages with no table; the note in
  the file that it would check nothing if no page had tables goes away
  with them.
- The texts for the readers (question 5). Answered by the user on
  2026-11-15: **as for the pages already made**: one Markdown file,
  cases included, whose author is the user; the agent writes the cases
  and a placeholder for each piece of text, the user writes the texts
  in Italian, one at a time, and for each the agent proposes an English
  version and its suggested changes, each with its reason. One
  difference: **the content of the tables is the tests' material**,
  written by the agent to precise constraints (the six-column case's
  window, the limit case just below 767, values the browser would
  break); a word changed in a table for style could move a case
  unnoticed, so the tables are part of the tests and the user's texts
  sit around them.
