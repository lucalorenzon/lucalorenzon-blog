# Add a sitemap

- **Opened:** 2026-11-06, by the user, while choosing on which pages the
  SEO check runs
  ([card](check-seo-with-lighthouse.md)). In Now, before the SEO
  check is resumed.
- **Status:** being worked on, since 2026-11-06.

## Why

The site has no sitemap: a file, usually `/sitemap.xml`, listing the
pages the site asks search engines to index. The user had meant to open
a card for it, as for the missing `robots.txt`.

It also answers a need of the SEO check: that check runs on the local
build and on the live site, and on the live site there is no folder to
walk, so it needs a list of pages. Today the only list is written by
hand in the tests. The user chose to put the SEO check on hold, add the
sitemap first, and then take the check's list of pages from it, in both
moments.

## Answers so far

### Which fields, and where the date comes from?

Chosen by the user: each entry has the page's address and the date of
its last significant change (`<lastmod>`). The build computes it from
git, before publication: the latest of the last change to the page's
content and the last change to the code that turns that content into
the page's text, links or data. A change of appearance only (the
stylesheet, the fonts, the layout) does not move the date.

- **Why a date at all:** it is the only optional field search engines
  use. Google's documentation
  ([Build and submit a sitemap](https://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap),
  read on 2026-11-06) says Google ignores `<priority>` and
  `<changefreq>`, and uses `<lastmod>` "if it's consistently and
  verifiably accurate".
- **Why not appearance:** the same page says the date "should reflect
  the date and time of the last significant update to the page": the
  main content, the structured data or the links are significant, the
  copyright date is not. A change of appearance is closer to the
  copyright date; counting it would make the date move when nothing
  significant changed, and risk Google ignoring the field for the site.
- **Why the code too, not the content alone:** part of the code changes
  what the page says: the header's and footer's links, the tags in the
  page's head, how Markdown becomes text.
- **Built before publication:** the published site is always the one
  built and tested from a commit of `main`, through the snapshot or the
  rebuild, so the date is computed there.
- **Full git history in the pipeline:** the build job downloads only the
  last commit today, so every file would get the same date and every
  page would look changed at each build. It must download the full
  history, as the job that detects documentation-only changes already
  does.
- **Rejected:** the address alone, always accurate but silent on
  changes; every change to the rendering code, appearance included,
  which Google does not count as significant; the build's own date,
  which changes at every build.

### How is the significant change found, in this first increment?

Chosen by the user: the date of a page is the date of the last commit
that changed its Markdown file. It is knowingly imprecise for now, and
becomes accurate with the card
[move titles, descriptions and dates into the Markdown's front matter](../backlog/move-page-fields-into-markdown-front-matter.md).

- **The problem:** the code that changes what a page says and the code
  that changes only how it looks are in the same files, often on the
  same line: the Tailwind classes are written in the Rust code next to
  the links and texts (9 in `src/page_frame.rs`, which draws every
  page's frame; 35 in `src/markdown.rs`). Git sees which files and lines
  changed, not whether the change is of content or appearance.
- **Why the Markdown:** it already holds each page's body, the pure
  content. The user wants it to become the only source of content, with
  the titles, descriptions and dates now in the code moved into a YAML
  front matter in each file: a card of its own.
- **The imprecision, accepted for now:** a change to an article's title,
  description or date, to a page's description, or to the links of the
  header and footer does not move the date.
- **Rejected:** a list of files per page, where a file holding both
  content and appearance counts entirely, so a change of colour would
  move every page's date; a fingerprint of each generated page's
  significant parts, compared with the previous version's, accurate but
  needing the previous fingerprints and dates, which the build keeps
  nowhere.

### Which date does the home page get?

Chosen by the user: none, for now. The home page is in the sitemap with
its address only; the date is optional entry by entry.

- **The problem:** the home page has no Markdown of its own; its content
  is the list of articles (title, description and date of each), all in
  the code.
- **When the home page changes:** in the user's words, when a new
  article is added. Its date belongs with how content is edited and
  added, more than with the front matter.
- **Rejected:** the latest date among the articles' Markdown files,
  wrong in the most common case, since correcting an article's text
  would move the home page's date while the home page, which shows only
  title, description and date, does not change; the date of the home
  page's code files, which brings back the mixed files already
  rejected.

## Known from the start

- **The build already decides which pages are indexed:** each page is
  `Indexing::Indexed` or `Indexing::NoIndex` (`src/page_frame.rs`); the
  "page not found" is the only `NoIndex` page today, and would not be in
  the sitemap.
- **The build already knows the site's full address,** `SiteAddress`
  (`src/site_address.rs`), used for the canonical link; a sitemap needs
  full addresses too.
- **The format** is the sitemaps.org protocol: an XML file, one entry
  per page with its full address (`<loc>`), and optional fields such as
  the date of the last change (`<lastmod>`).
- **A page missing from the sitemap** would escape both the search
  engines and the SEO check. Locally a test can compare the sitemap with
  the pages found in the built site's folder.
- **The site is served from the root** of `https://www.lucalorenzon.com/`
  since 2026-11-01, where a sitemap is expected.
- **Pointing search engines to it** is the job of a `robots.txt`
  ([card](../backlog/add-robots-txt.md)).
- **The next version waits for this card,** the `robots.txt` and the SEO
  check: the user chose to publish them together with the canonical
  link, the descriptions and the Open Graph tags already done.

## Questions

- How the build generates it from the pages it already writes.
- How the test checks it, locally and on the live site.
