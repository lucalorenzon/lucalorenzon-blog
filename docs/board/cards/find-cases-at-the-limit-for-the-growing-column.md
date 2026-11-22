# Find cases at the limit for the growing column

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md).
There the column's text stops jumping from 18 to 20 pixels at a
screen of 768 pixels and grows smoothly up to 1366, so the column
widens gradually instead of at once. The tables' cases at the limit,
placed on the two sides of that jump (767 and 768), lost their
meaning and were taken out with that card.

Those cases checked, where the column widened, that the threshold the
build estimates puts each table in the form written for it: a table
where its columns fit, a record viewer below. Without them the
estimate is checked only by the rule that no table may be wider than
its column, at every width.

## What is known

- **What the cases were**, in the style guide's tables page
  (`content/style-guide/tables.md`) and their tests
  (`end2end/tests/table-forms.spec.ts`, rows in `end2end/README.md`):
  the six-column table, a table at 768 and a record viewer at 767;
  the rows of different heights, a record viewer below 768; and the
  table of texts at the limit, a table at 767 and 768 and not wider
  than its column, made of single words so that its estimate is close
  to its real smallest width. Measurements in
  [show the table in the style guide](show-the-table-in-the-style-guide.md).
- **The code block kept its case at the limit**, moved from 768 to
  1366, where the column reaches its widest; it is the check that
  stops the build when a code font moves the wrap toggle's threshold
  ([measure the fonts again when they change](measure-the-fonts-again-when-they-change.md)).
- **With a growing column**, the column's width follows the page's
  width between 320 and 1366 pixels, so a case at the limit can be
  placed at any width, not only at a breakpoint; smoothly in Firefox,
  by steps in Chromium on Linux (below).
- **The column steps in Chromium on Linux** (measured on 2026-11-22,
  at the close of
  [let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md)):
  its maximum, 60 characters of the digit zero, is 600, 660 or 720
  pixels as the text grows, since Chromium rounds the zero's width to
  a whole pixel; in Firefox it grows smoothly. The six-column table,
  whose threshold (33.904 em) lies close to the column's widest, is a
  record viewer up to 412, a table from 659 to 810, a viewer from 843
  to 1280 and a table from about 1300. Details in that card's recap.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which cases at the limit are worth having** for the tables, and
  at which widths.
- **Whether the table of texts at the limit comes back**, since it
  checked that the estimate is not too low.
- **Whether other components need cases at the limit** besides the
  tables and the code block.

## Dependencies

- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md),
  to be done first.

## References

- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the card this one was cut from.
- [Show the table in the style guide](show-the-table-in-the-style-guide.md):
  where the tables' cases at the limit were made and measured.
- [Measure the fonts again when they change](measure-the-fonts-again-when-they-change.md):
  the check the code block's case at the limit carries.
- [Check the table widths against Chromium's rounding](check-table-widths-against-chromium-rounding.md):
  the table's widths, which a case at the limit would test.
