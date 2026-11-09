# Check the pages' text in the build

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

With the articles' card, an article is built only from a file whose
front matter is valid, but its text, the Markdown after the front
matter, is a plain string that nothing checks:

- **an empty text** is not checked anywhere: an article with a valid
  front matter and no text would become an empty page;
- **HTML in the text** is checked only by unit tests on the real files
  (`src/markdown.rs`, `src/article_page.rs`). In the pipeline the tests
  run before the build, so a page with HTML cannot be published; on the
  author's machine the build writes the page anyway, and only running
  the tests shows the problem;
- **an empty header cell of a table** is not checked anywhere (added
  on 2026-11-09, by the user, in the card
  [check accessibility on every page with the net](../wip/check-accessibility-on-every-page-with-the-net.md)).
  Markdown allows a header row such as `| | Value |`; the build
  writes an empty header, and in the record viewer an empty label, so
  a screen reader hears a value without knowing what it is. Empty data
  cells stay allowed, as today (the viewer shows a dash). axe's rule
  for it, `empty-table-header`, is a best practice, run only by the
  on-demand check (`end2end/tools/check-axe-best-practices.mjs`), not
  by the tests: a rule in the build guarantees it for every article,
  so no browser test is needed.

The user wants all three checked in the build, with the same rule as a
missing or wrong field: on the author's machine the article is skipped
and listed, in the pipeline the build stops
([card](../done/2026-11-08_read-article-fields-from-front-matter.md), "What does
the build do when a field is missing or wrong?").

## Known from the start

- **Following "parse, don't validate",** as for the front matter's
  fields: a type for the text, built with `TryFrom`, which can hold
  only a text that is not empty and has no HTML; the article's error
  gains the text's errors next to the front matter's.
- **The two checks were kept together,** in this card, rather than
  checking the empty text in the articles' card: the type for the text
  is born once, with both rules.
- **The check on HTML concerns every page,** the curriculum, about and
  licence pages too, not only the articles: it is linked to the card on
  [the other pages' fields](read-page-fields-from-markdown.md).

## Questions

- How the error names the table and the column with the empty header,
  so the author can find it in the Markdown.

- Whether the other pages get the same type, and what skipping one of
  them means.
- What happens to the unit tests that check HTML today: removed, or
  kept as a check on the real files.
