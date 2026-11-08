# Move titles, descriptions and dates into the Markdown's front matter

- **Opened:** 2026-11-06, by the user, while choosing where the
  sitemap's dates come from
  ([card](../done/2026-11-07_add-sitemap.md)).
- **Status:** being worked on, since 2026-11-08.

## Why

The user wants the Markdown to be the only source of the site's
content. Today the body of each page is in its Markdown file under
`content/`, but part of what the page says lives in the Rust code:

- the title, the tab title, the description and the date of each
  article, in the list `ARTICLES` of `src/article_page.rs`;
- the description of the other pages, a constant in each page's file
  (for example `src/about_page.rs`).

The user chose to move them into a front matter in YAML at the top of
each Markdown file.

It matters for the sitemap: its date of a page's last change comes from
git. As long as titles, descriptions and dates are in the code, a change
to them does not move the date, which, as a first increment, is taken
from the page's Markdown file only
([card](../done/2026-11-07_add-sitemap.md)). With this card the date becomes
accurate for those fields too.

## Known from the start

- **The model of an "article" was left for later** when the home page's
  list was made: to the
  [run of `residuality`](../backlog/run-full-residuality-round.md) and to the card
  on search-engine tags
  ([card](../done/2026-11-06_list-articles-on-home-page.md), "What each
  entry shows"). The user noted that this card anticipates part of that
  run.
- **The home page has no Markdown of its own:** its content is the list
  of articles, and its description is a constant in `src/home_page.rs`.
- **The Markdown holds no HTML,** checked by a unit test; a front matter
  is not HTML, but the conversion with pulldown-cmark has to skip it.
- **The model the user aims at** (2026-11-07, while making the
  [sitemap](../done/2026-11-07_add-sitemap.md)): the build goes through every `.md`
  file under `content/` and its folders, reads its front matter (title,
  description, slug and the rest) and its date of last change, and from
  them writes both the page and its sitemap entry. The page's address
  comes from the slug, which follows the marketing campaigns and may
  differ from the file's name. A list of old slugs, for redirects, may
  follow later.
- **Files in `content/` that are not pages,** `README.md` and
  `LICENSE-MIT-0`, are on an exclusion list, chosen by the user for the
  sitemap's card.
- **What stays in the code:** the links of the header and the footer,
  in `src/page_frame.rs`. A change to them does not move the sitemap's
  dates.

## Questions

- Which fields the front matter has, for articles and for the other
  pages.
- Where the home page's description goes.
- Which tool reads YAML, a new dependency to be audited as any other.
- What the build does when a field is missing or wrong.
