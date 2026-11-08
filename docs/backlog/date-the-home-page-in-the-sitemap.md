# Give the home page a date in the sitemap

- **Opened:** 2026-11-07, by the user, while making the sitemap
  ([card](../done/2026-11-07_add-sitemap.md)), where the home page is listed without
  a date.
- **Status:** noticed, not started.

## Why

In the sitemap, each page's date of last change comes from its Markdown
file. The home page has no Markdown of its own: it lists the articles.
It is therefore in the sitemap without a date, which the format allows.

In the user's words, the home page changes when an article is added or
removed, so its date should be the date of the last addition or removal
of an article. The user chose to deal with it in a card of its own,
apart from the sitemap's first increment.

## Known from the start

- **What the home page shows** of each article: its title, description
  and date, today in the code (`ARTICLES` in `src/article_page.rs`);
  correcting an article's text does not change the home page.
- **The latest date among the articles' Markdown files was rejected**
  for the home page, since it moves when an article's text is corrected
  ([card](../done/2026-11-07_add-sitemap.md), "Which date does the home page get?").
- **A change to an article's title, description or date** also changes
  the home page; once those fields are in the Markdown's front matter
  ([card](../wip/read-article-fields-from-front-matter.md)), it shows in
  git as a change to the Markdown file, not as an addition.

## Questions

- Which changes move the home page's date: additions and removals only,
  or also changes to what the home page shows of an article.
- How the build finds the date from git.
