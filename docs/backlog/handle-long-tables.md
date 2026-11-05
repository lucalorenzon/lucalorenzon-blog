# Handle long tables

- **Opened:** 2026-11-05, by the user, while closing the card
  [let the Markdown ask for components for wide code and tables](../done/2026-11-05_annotate-markdown-for-wide-content.md);
  in Later.
- **Status:** noticed, not started. To be picked up with the first
  article that has a long table.

## Why

The tables in the articles today have at most eight rows. Two
questions wait for the first long one.

- **A header that stays in view:** while a long table scrolls with the
  page, its header leaves the screen, and the reader no longer sees
  which column a value belongs to. The idea is the user's.
- **The record viewer's copies:** on narrow screens a wide table
  becomes a record viewer, built with HTML and CSS only. To keep its
  rows aligned without a script, each value is written once in the
  table and once more for every record, plus one: seven times for the
  hybris table's five records, twenty-two for a table of twenty. The
  copies grow with the square of the records. The user accepted them
  for today's tables as the price of no script; a long table reopens
  the choice of a script, a
  [Leptos island](measure-weight-of-a-leptos-island.md), which would
  write each value once.

## Known from the start

- **Both forms are involved:** a long table may be a table on a wide
  screen and a record viewer on a phone, so the header question has an
  answer for each form; in the record viewer the labels already stay
  in view, but the record number and the page's own scrolling may not.
- **How the record viewer works** and why it was chosen: the closed
  card above, sections "Spike A" and "The copies of each value".

## Questions to answer

- From how many rows a table counts as long, for each question.
- How the header stays in view in the table form (for example a
  sticky header) and whether the record viewer needs anything.
- Whether a long table keeps the record viewer without script, with
  its copies, or moves to an island; measured on the page's weight.
