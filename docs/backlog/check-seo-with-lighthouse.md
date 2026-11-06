# Check SEO with Lighthouse in the build

- **Opened:** 2026-11-06, by the user, splitting the card on SEO and
  Open Graph tags, opened on 2026-11-02; the other parts are
  [add the description and the Open Graph tags to every indexed page](../wip/add-description-and-open-graph-tags.md)
  and [add preview images](add-preview-images.md). In Now, after the
  description and the Open Graph tags.
- **Status:** noticed, not started.

## Why

The user wants the build to check automatically that every page meets
the search-engine rules, so a page published without its description,
its title or its canonical link does not go unnoticed. Some pages must
be able to stay out of it, through a block-list.

## Answers so far

### Which rules?

Chosen by the user: those of Lighthouse's SEO category, a recognised
tool, rather than a list of rules of the project's own checked with
Playwright. The rules then carry the tool's authority and are not
maintained by the project; the price is a new dependency, to be audited
as any other.

This anticipates the choice of tool of the card
[check performance in the build](check-performance-in-the-build.md),
knowingly: that card will find Lighthouse already in the build, and its
performance category can be added to it.

### Why a card of its own?

Split off by the user from the description and the Open Graph tags,
once it was clear that this check holds several decisions of its own,
while those tags were almost decided; each part can be published alone.

## Known from the start

- **Lighthouse's SEO category** checks, among others: the document has
  a title and a description; the response status; links with
  descriptive text and that can be crawled; the page is not blocked
  from indexing; the canonical link is valid; `robots.txt` is valid;
  images have alternative text. It does not check Open Graph: those
  tags are checked by the Playwright test of their own card.
- **The "page not found" would fail the rule on indexing,** on
  purpose: it carries `noindex`, since `/404.html` asked directly
  answers with status 200
  ([card](../done/2026-11-06_add-canonical-link-to-every-page.md)). So
  the block-list is needed from the start
  ([card](../done/2026-11-02_serve-own-not-found-page.md) of that page).
- **The site has no `robots.txt` yet.**
- **The build already starts a browser** (Playwright, Chromium, in
  Playwright's container image) on the built site served locally, and
  again on the live site after each publication.
- **Lighthouse CI** (`GoogleChrome/lighthouse-ci`) runs Lighthouse in a
  pipeline and can fail the build on thresholds; Lighthouse can also be
  driven from the Playwright tests.

## Questions

- How it runs in the pipeline: Lighthouse CI, Lighthouse driven from
  Playwright, which package and version, in which container.
- On which pages and when: the local build, the live site after
  publication, or both.
- The threshold: the full score, or rule by rule.
- The block-list: a list kept in one place, or a field of each page;
  and whether a page on it drops the check only, or the tags too.
- Whether the site needs a `robots.txt`, and what it says.
