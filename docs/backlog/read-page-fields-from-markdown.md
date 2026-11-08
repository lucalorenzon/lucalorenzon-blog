# Read the other pages' title and description from the Markdown

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on moving titles, descriptions and dates into the Markdown's front
  matter, opened on 2026-11-06 while choosing where the sitemap's dates
  come from ([card](../done/2026-11-07_add-sitemap.md)). The articles
  come first ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

The user wants the Markdown to be the only source of the site's
content. The articles' fields move into their front matter in a card of
their own; the other pages keep theirs in the code:

- the description of the curriculum, about and licence pages, a
  constant in each page's file (for example `src/about_page.rs`);
- their title and tab title, written in the same files.

As for the articles, a change to these fields does not move the
sitemap's date of the page, which comes from its Markdown file only
([card](../done/2026-11-07_add-sitemap.md)).

## Known from the start

- **The articles' card sets the pattern:** the fields read with
  serde-saphyr, the rule for a missing or wrong field
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **The home page has no Markdown of its own:** its content is the list
  of articles, and its description is a constant in `src/home_page.rs`.
- **The tab titles do not all follow one rule:** the home page's is
  "Luca Lorenzon - Product Engineer", the others' "title - Luca
  Lorenzon".
- **What stays in the code:** the links of the header and the footer,
  in `src/page_frame.rs`. A change to them does not move the sitemap's
  dates.

## Questions

- Which fields the front matter has for these pages, and whether the
  tab title is computed as for the articles.
- What a missing or wrong field does to a page that is not an article:
  skipping the about or licence page is not the same as skipping one
  article.
- Where the home page's description goes.
