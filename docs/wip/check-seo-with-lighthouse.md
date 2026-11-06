# Check SEO with Lighthouse in the build

- **Opened:** 2026-11-06, by the user, splitting the card on SEO and
  Open Graph tags, opened on 2026-11-02; the other parts are
  [add the description and the Open Graph tags to every indexed page](../done/2026-11-06_add-description-and-open-graph-tags.md)
  and [add preview images](add-preview-images.md). In Now, after the
  description and the Open Graph tags.
- **Status:** being worked on, since 2026-11-06.

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

### When does it run?

Chosen by the user: in both moments, as the Playwright test already
does: on the built site served locally, on every push and pull request,
and on the live site after each publication.

- **The local build** catches a page that breaks the rules before it is
  published, on every change.
- **The live site** is what search engines actually see: GitHub Pages'
  responses, and a `robots.txt` served by the real server, if the site
  ever has one. Like the Playwright test there, it can only report
  after publication, not prevent it.
- **The live check failing today is not a problem.** The published
  version still lacks the descriptions (the home page scores 90), but
  the live check runs only after a publication, and the next one brings
  the descriptions.
- **Rejected:** the local build alone, which never sees the real
  server; the live site alone, which finds a broken page only once it is
  public.

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

## Found by a trial run

On 2026-11-06 Lighthouse 13.6.0 was run once, SEO category only, with
the Chromium that Playwright 1.64.1 installs (Chrome 156), on every page
of the site built from commit `99c83d4` and served locally by Python's
server, as the Playwright test does; and on the live home page. Nothing
was added to the repository.

- **Every page scores 100 except the "page not found",** which scores 50:
  it fails the rule on indexing (`noindex`, as expected) and the rule on
  the description, which only indexed pages have
  ([card](../done/2026-11-06_add-description-and-open-graph-tags.md)).
- **The canonical link passes locally,** although it points to
  `https://www.lucalorenzon.com/` while the page is served from
  `127.0.0.1`: the rule does not compare domains. The check can run on
  the local build.
- **A missing `robots.txt` is "not applicable",** locally and on the live
  site: its absence fails nothing.
- **The rule on images' alternative text is "not applicable"** too, since
  the content has no images yet; it becomes active by itself with the
  first one.
- **Every rule of the category is pass or fail,** none depends on
  timing: the same site always gives the same result, unlike the
  performance scores.
- **The live home page scores 90:** the published version still lacks
  the description.

## Questions

- How it runs in the pipeline: Lighthouse CI, Lighthouse driven from
  Playwright, which package and version, in which container.
- The threshold: the full score, or rule by rule.
- The block-list, which also decides on which pages the check runs: a
  list kept in one place, or a field of each page; and whether a page on
  it drops the check only, or the tags too.
- Whether the site needs a `robots.txt`, and what it says.
