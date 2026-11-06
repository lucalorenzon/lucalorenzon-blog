# Add a sitemap

- **Opened:** 2026-11-06, by the user, while choosing on which pages the
  SEO check runs
  ([card](../wip/check-seo-with-lighthouse.md)). In Now, before the SEO
  check is resumed.
- **Status:** noticed, not started.

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
  ([card](add-robots-txt.md)).
- **The next version waits for this card,** the `robots.txt` and the SEO
  check: the user chose to publish them together with the canonical
  link, the descriptions and the Open Graph tags already done.

## Questions

- Which fields each entry has besides the address; and if a date of the
  last change, where it comes from.
- How the build generates it from the pages it already writes.
- How the test checks it, locally and on the live site.
