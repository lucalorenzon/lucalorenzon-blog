# Watch the rules Lighthouse finds not applicable

- **Opened:** 2026-11-06, by the user, while choosing the threshold of
  the SEO check
  ([card](../wip/check-seo-with-lighthouse.md)).
- **Status:** noticed, not started.

## Why

The SEO check counts a rule that Lighthouse finds "not applicable" as
passed. That keeps it simple: the rule on images' alternative text
becomes active by itself with the first image, with nothing to update.
The price is that a change in "not applicable" goes unnoticed, both
ways:

- **A rule that stops applying by mistake passes in silence.** For
  example, if a page loses its canonical link, the rule on the canonical
  link becomes "not applicable" on that page. Today the Playwright test
  on the canonical link would still catch it; another rule might have no
  such test behind it.
- **A rule that starts applying is not announced.** The first image, or
  a `robots.txt`, turns a rule on without anyone deciding to check it.

## Known from the start

- **Not applicable on 2026-11-06:** the rule on `robots.txt` (the site
  has none), the rule on images' alternative text (the content has no
  images) on every page; the rule on the canonical link on the "page not
  found", which has no canonical link
  ([trial run](../wip/check-seo-with-lighthouse.md)).

## Questions

- Whether the check should declare which rules are expected to be "not
  applicable", on which pages, and fail when the list and the result
  disagree; or report the change without failing.
- Who updates that list, and when: with the first image, with a
  `robots.txt`.
