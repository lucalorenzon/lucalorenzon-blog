# Show the table in the style guide

- **Opened:** 2026-11-15, by the user, as the third page of the style
  guide in the card [build a design system](../wip/build-a-design-system.md),
  the second one about a component, after
  [the code block](2026-11-15_show-the-code-block-in-the-style-guide.md);
  in Now.
- **Status:** done, 2026-11-15. Not published yet: the style guide goes
  live when it is complete.

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

## Acceptance tests

Agreed with the user on 2026-11-15; all thirteen pass, on 2026-11-15.

Slice 1, the page and its tests:

1. The build writes the style guide's tables page, linked from the
   index and listed in the sitemap, with its heading, description and
   canonical address: unit tests, as for the code blocks page.
2. The page holds six cases, each in a section found by its title: the
   simple table (two columns, five rows), the six-column table (five
   rows), the rows of different heights, the range with an en dash, the
   word with a hyphen, the texts at the limit (italic, inline code,
   bold): unit test of the page.
3. At 767 and 768: the simple table is a table; the six-column table
   is a table at 768 and a record viewer at 767; the limit table is a
   table, no wider than its column: browser test, one expectation per
   case.
4. At every column width, exactly one form of each table shows, no
   table is wider than its column, and the page does not scroll
   sideways: browser test on the guide's page.
5. In the case of the rows of different heights, at every column width
   where it is a record viewer, every row of every record is level with
   its label and as tall, and the number row reads "k / n": browser
   test.
6. At 767, Tab reaches the six-column table's viewer, the down arrow
   moves it by one record and Page Down by one more: browser test.
7. At 767 and 768, axe limited to the tables finds no violation, and
   the contrast holds in both schemes, with no guard: browser tests.
8. At 320, 393, 412 and 767, the range and the hyphenated word each
   stay on one line: `tables.spec.ts`, on the guide's page only.
9. No table test names an article, and the guards on the hybris article
   are gone: review of the diff.
10. The rows of the moved tests in `end2end/README.md` say what they
    now check and where; `readme.spec.ts` passes.
11. The column's width at 767 and 768 and the limit table's threshold
    are measured on the built page and written in the card; the
    threshold falls just below the column at 767.
12. The checks of `build.yml` pass: format, lint, unit tests, type
    check, browser tests.

Slice 2, the texts:

13. Every placeholder holds the user's text, and no table changed in
    this slice: review of the diff.

## Results

- **The page and its tests** (commit `504e488`):
  `style-guide/tables.html` with six cases in
  `content/style-guide/tables.md`; `table-forms.spec.ts` and
  `tables.spec.ts` run on it instead of the articles, with one
  expectation for each case, found by its section's title, and without
  the guards on the hybris article.
- **The texts** (commits `fa0051a`, `79e8d1e`, `5e94f2d`, `92cb8f7`,
  `c3b25a7`), and no table changed while they were written.
- **The checks of `build.yml`**, run locally on 2026-11-15 after the
  last text: format, lint, 334 unit tests, type check, 228 browser
  tests passed, one skipped (the "page not found" test that runs on
  the live site only).
- **The articles are no longer test pages for the tables**, except in
  one place: `end2end/tests/content-layout.spec.ts` and a guard in
  `end2end/tests/support.ts` still need the After Save Event article to
  show a table, for the text checks; left to the typography page of the
  card [build a design system](../wip/build-a-design-system.md).
- **Measured on the built page** (acceptance test 11), on 2026-11-15:
  - the text column's content box: 584 px at 767 (text 18 px), 704 px
    at 768 (text 20 px);
  - the six cases' estimated thresholds: the simple table 10.254 em,
    the six-column table 33.904 em (inside its window of 32.444 to
    35.2 em), the rows of different heights 33.643 em, the range
    12.83 em, the hyphenated word 16.224 em, the limit table 32.292 em;
  - the limit table at 767: estimate 581.3 px, real minimum width
    569 px (the table set to `min-content`), column 584 px. The
    estimate is 12 px above the real minimum (widths rounded up,
    kerning ignored), so the test catches an estimate too low by more
    than about 15 px. Proof that it bites: with the letter spacing of
    its italic, code and bold widened by 0.05 em, in a throwaway run,
    the table grows to 605 px, wider than its column;
  - the range and the hyphenated word break at 320 without the
    `one-line` rule (measured with the rule switched off in a throwaway
    run); at 767 the ranges still break, and three words of four. The
    range case needed a header shorter than its values ("Pts"), which
    otherwise held the column open.
- **The tests:** 267 browser tests became 228, the per-article runs of
  the table tests now one run on the guide's page.

## Answers

- Which cases the page needs (question 1). Answered by the user on
  2026-11-15:
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
  **Every cell of the limit table is a single word**, headers included,
  decided by the user on 2026-11-15 after the first version was
  measured. In the first version the cells held several words: for such
  a cell the build asks for a readable line, up to about 17 characters,
  not for its longest word, so the estimate sat far above what the
  browser cannot shrink (431 px of real minimum width in a 584 px
  column), and a table in table form always fills its column: an
  estimate too low would have been absorbed by that room, and the test
  could not fail. The agent had designed the case wrongly and saw it
  only on the measurement. The user first asked to remove the case,
  since an estimate too low mostly leaves a table cramped but readable,
  then chose to redo it, as a case at the limit shows the component's
  behaviour. With single words, each column's need is its longest word,
  which is also what the browser cannot shrink, so the estimate is
  close to the real minimum width.
- How the change of form is checked (question 3). Answered by the user
  on 2026-11-15: **at 767 and 768, for all three tests** (the change
  of form, the keyboard, axe with the contrast). At 768 the text grows
  from 18 to 20 px and the column widens, from 584 px at 767 to 704 px
  (measured on the built page on 2026-11-15, the content box of the
  text column; 632 px had been computed beforehand, wrongly). The
  threshold is in em of the table's own text, so the window is between
  32.444 em (584 / 18) and 35.2 em (704 / 20): 2.756 em, about 50 px of
  the text at 767. The six-column case's threshold must fall in that
  window, not on one pixel. So the tests still
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

## The texts

The user wrote the seven texts on 2026-11-15, in Italian, one at a
time; for each one the agent proposed an English version and the
changes it suggested, each with its reason, and the user chose which to
apply: all of them. Changes made for accuracy, beyond the language:

- the record viewer comes when the columns would be too narrow to read,
  not when the table overflows its column;
- the width where a table changes form is estimated, not computed;
- nothing computes the rows' heights: the values of a field sit in the
  same cell, hidden, and the browser makes the row as tall as the
  tallest of them;
- the range is written with an en dash, as in the case: a hyphen
  followed by a digit is not a place where the browser breaks the line;
- the limit table is estimated at 581 px, not 681, which would not fit
  its column, and the estimate is not the table's real width.
