# Check the structured data

- **Opened:** 2026-11-06, by the user, while choosing the threshold of
  the SEO check
  ([card](../wip/check-seo-with-lighthouse.md)).
- **Status:** noticed, not started.

## Why

Lighthouse's SEO category holds a rule on structured data that it never
judges by itself: it marks it "manual" and leaves it to a person. The SEO
check therefore ignores it, and nothing in the build says whether the
site's structured data is right, or whether it should have any.

Structured data is a description of the page written for machines (for
example who wrote an article and when), which search engines may use to
show a richer result.

## Known from the start

- **The site has no structured data yet.**
- **Lighthouse's own advice** for that rule (version 13.6.0) is to run
  Google's
  [Structured Data Testing Tool](https://developers.google.com/search/docs/appearance/structured-data/)
  on the page, by hand.

## Questions

- Whether the site needs structured data, on which pages, and of which
  kind.
- If it does: how the build checks it, since Lighthouse does not.
