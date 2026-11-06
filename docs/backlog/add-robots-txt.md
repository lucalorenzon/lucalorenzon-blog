# Add a robots.txt

- **Opened:** 2026-11-06, by the user, while choosing on which pages the
  SEO check runs
  ([card](../wip/check-seo-with-lighthouse.md)), where it was the last
  open question. In Now, after the [sitemap](add-sitemap.md).
- **Status:** noticed, not started.

## Why

The site has no `robots.txt`: the file at the root of a site that tells
search engines which addresses they may read, and where the sitemap is.
The user chose to add it, in a card of its own, separate from the
sitemap's: whether the site needs one, and what it says, is a decision of
its own.

## Known from the start

- **There is nothing to disallow today.** The "page not found" is kept
  out of search results by `noindex`. Disallowing it in `robots.txt`
  would not help: Lighthouse counts a page disallowed there as blocked
  too, so its rule on indexing would fail anyway; and search engines
  could no longer read the page, so they would not see its `noindex`, as
  Google warns
  ([card](../wip/check-seo-with-lighthouse.md), "How is a page
  exempted?").
- **Its main use here** would be the `Sitemap:` line, pointing search
  engines to the [sitemap](add-sitemap.md).
- **Lighthouse's rule on `robots.txt`** is "not applicable" today, since
  the file is missing; with the file it becomes active and checks it is
  valid. That change is one of those the card
  [watch the rules Lighthouse finds not applicable](watch-lighthouse-rules-not-applicable.md)
  is about.
- **It must be at the root of the host,** `/robots.txt`; the site is
  served from the root of `https://www.lucalorenzon.com/` since
  2026-11-01.

## Questions

- Whether the site needs one, and what it says.
- How the build writes it, and how the test checks it.
