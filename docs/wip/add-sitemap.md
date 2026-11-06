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

- How the build tells, for each page, which files make its content,
  links and data, and which only its appearance; some files hold both.
- How the build generates it from the pages it already writes.
- How the test checks it, locally and on the live site.
