# Unify the model of the information pages

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../wip/read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

The pages that are not articles and carry information, the licence,
about and curriculum pages, are written as three modules that are
nearly copies of each other: `src/licence_page.rs`,
`src/about_page.rs` and `src/curriculum_page.rs`. Each holds its
Markdown, its description, its title and heading, its address, its
kind of page for search engines and its licence line in the footer;
the component and the function that render the page are the same
code three times, and so are most of their tests.

The articles now have one model, `Article`, built from their file
([card](../wip/read-article-fields-from-front-matter.md)). The user
wants the same for the information pages: one model of data for the
three, instead of three modules.

## Known from the start

- **What differs between the three pages,** today: the Markdown file,
  the address, the title and heading, the description, the kind of page
  (`PageKind::Profile` for about, `PageKind::Website` for the others)
  and the licence in the footer (the curriculum's forbids changes).
- **It overlaps with the card on
  [the other pages' fields](read-page-fields-from-markdown.md):** that
  card moves their title and description into the Markdown. With one
  model, the move is done once rather than three times, so this card
  probably comes first.
- **The home page and the "page not found"** are not information pages:
  the first is the list of articles, the second is not indexed.

## Questions

- What the model holds, and which of its fields come from the Markdown
  and which stay in the code.
- Whether articles and information pages share part of the model, or
  stay two models.
