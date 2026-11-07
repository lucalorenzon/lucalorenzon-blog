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

Added by the user on 2026-11-07, while writing the `robots.txt`
([card](../done/2026-11-07_add-robots-txt.md)): the user would like the robots of
artificial intelligence that read the site to cite it in their answers.
No mechanism makes them do it; the site can only declare its terms in a
form machines read, and one of the ways goes through structured data.
The user chose to keep it in this card, and to split it here later if
needed.

## Known from the start

- **The site has no structured data yet.**
- **Lighthouse's own advice** for that rule (version 13.6.0) is to run
  Google's
  [Structured Data Testing Tool](https://developers.google.com/search/docs/appearance/structured-data/)
  on the page, by hand.

- **Ways to declare the licence and ask for a citation,** known on
  2026-11-07: a `<link rel="license">` in each page, or the `license`
  and `author` fields in the structured data, so whoever reads a page
  knows the articles are under CC BY 4.0, which asks to credit the
  author ([licences](../../content/README.md)); `llms.txt`, a proposed
  Markdown file at the site's root that describes the site to language
  models, not a standard, its use unknown. In practice, citations come
  from the search products and assistants that show their sources with
  a link, the robots the `robots.txt` lets through.

## Questions

- Whether the site needs structured data, on which pages, and of which
  kind.
- If it does: how the build checks it, since Lighthouse does not.
- How the site declares its licence and asks for a citation, in a form
  machines read; and whether that part becomes a card of its own.
