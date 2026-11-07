# Check SEO with Lighthouse in the build

- **Opened:** 2026-11-06, by the user, splitting the card on SEO and
  Open Graph tags, opened on 2026-11-02; the other parts are
  [add the description and the Open Graph tags to every indexed page](../done/2026-11-06_add-description-and-open-graph-tags.md)
  and [add preview images](../backlog/add-preview-images.md). In Now, after the
  description and the Open Graph tags.
- **Status:** being worked on, since 2026-11-06; on hold from
  2026-11-06, waiting for the cards
  [add a sitemap](../done/2026-11-07_add-sitemap.md), done on
  2026-11-07, and
  [add a robots.txt](../backlog/add-robots-txt.md). It stays in
  `docs/wip/` meanwhile, by the user's choice: half of its answers are
  given.

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
[check performance in the build](../backlog/check-performance-in-the-build.md),
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

### How does it run?

Chosen by the user: Lighthouse driven from the Playwright tests, as a
library inside the suite that already exists.

- **What it reuses:** the local server, the `BASE_URL` variable that
  switches between the two moments, the report in the pipeline, the
  Playwright container image with its Chromium.
- **Its price:** some code of the project's own, since Lighthouse needs
  a browser opened with a dedicated control port, which Playwright does
  not open by default; and the suite runs twice, dark and light theme,
  while these rules need one run.
- **Known tension with the performance card:** there, scores vary and
  need several runs and their median, which Lighthouse CI provides and
  the project will then write itself. The SEO rules do not need it,
  since they always give the same result.
- **Rejected:** Lighthouse CI (`@lhci/cli` 0.16.0), a second program
  beside Playwright with its own way of naming pages and the live site;
  Lighthouse's command line in a script of the project's own, which
  would hold the thresholds, the server and the list of pages outside
  both tools.

### Which package?

Chosen by the user: the `lighthouse` package alone, at version 13.6.0,
plus the project's own few dozen lines that open Chromium with a control
port and hand the page to Lighthouse.

- **The package:** by the Chrome team, Apache 2.0 licence, version
  13.6.0 released on 2026-10-27, the one of the trial run. It goes into
  `end2end/`'s development dependencies at an exact version, as the
  others there, and Dependabot already follows that folder.
- **Its weight:** about 120 packages come with it, among them Puppeteer
  and the Sentry client. Read in its code (`core/lib/sentry.js`): used
  as a library, Lighthouse loads Sentry only when the option
  `enableErrorReporting` is passed, so it sends nothing unless asked.
- **Rejected:** `playwright-lighthouse` 4.0.0 on top of `lighthouse`. It
  would write the link between the two tools for the project, but it has
  one maintainer, its last version dates from 2024-01-15, and it has not
  been checked against recent Lighthouse versions since.

### Which threshold?

Chosen by the user: rule by rule, every rule of the category. Since
every rule is pass or fail, this is as strict as the full score of 100,
but the test names each rule that fails, instead of a number to be
explained by reading the report. A rule that a future Lighthouse version
adds enters the check by itself, since the check takes the category's
rules as they are, not a list of the project's own.

- **"Not applicable" counts as passed,** as Lighthouse does in its score.
  A change in what is not applicable goes unnoticed; it is handled by
  the card
  [watch the rules Lighthouse finds not applicable](../backlog/watch-lighthouse-rules-not-applicable.md).
- **The "manual" rule on structured data is ignored** by the automatic
  check, since Lighthouse never judges it; it is handled by the card
  [check the structured data](../backlog/check-structured-data.md).
- **Rejected:** the full score, which also catches a new rule but says
  only "95" when one fails; a list of rules chosen by the project, which
  a new rule would not enter until someone adds it.

### How is a page exempted?

Chosen by the user: from the page itself. A page that carries
`<meta name="robots" content="noindex">` skips two rules: the rule on
indexing, which `noindex` fails by definition, and the rule on the
description, since the build gives a description only to indexed pages
(`Indexing::NoIndex` in `src/page_frame.rs`). Every other rule still
applies to it: title, links, response status, and the rest.

- **One source:** the build already decides, page by page, whether a page
  is indexed; the check reads that decision instead of repeating it.
- **How:** the test reads the page, and if it carries `noindex` it calls
  Lighthouse with the option `skipAudits` set to those two rules
  (`is-crawlable`, `meta-description`), present in version 13.6.0.
  Lighthouse has no notion of a set of pages: it checks the one address
  it is given, so which pages are checked is always the test's choice.
- **Its price:** a page marked `NoIndex` by mistake would drop out of
  those two rules in silence.
- **The tags go with it:** a page with `noindex` has no description and
  no Open Graph tags, as decided by the card
  [add the description and the Open Graph tags](../done/2026-11-06_add-description-and-open-graph-tags.md);
  this card does not change that.
- **Rejected:** a list kept in the test, which would be a second
  declaration beside the build's, free to disagree with it; a
  `robots.txt` that disallows the "page not found", which Lighthouse
  counts as blocked too, so the rule would fail anyway, and which would
  hide the `noindex` from search engines, as Google warns.

### On which pages, and why on hold?

Chosen by the user: the check takes its list of pages from a sitemap,
which the site does not have yet. The card is on hold until the sitemap
and the `robots.txt` exist, each in a card of its own.

- **The problem:** on the live site there is no folder to walk, so the
  check needs a list of pages, and the only one, `TEXT_PAGES` in
  `end2end/tests/support.ts`, is written by hand; the card
  [make the browser tests check behaviour](../backlog/make-browser-tests-check-behaviour.md)
  wants to get rid of such lists. The "page not found", asked on the
  live site through a missing address, answers with status 404 and
  fails the rule on the response status, rightly, since a missing
  address must answer 404; asked as `/404.html`, it answers 200, at an
  address no search engine meets.
- **Why the sitemap:** it lists the pages the site asks search engines
  to index, exactly those the check is about; the test reads it from
  the local server or from the live site in the same way; the build can
  generate it from the decision it already takes, `Indexing::Indexed`
  or `NoIndex`. The "page not found" is not in it, so the problem of its
  404 on the live site goes away.
- **The version waits:** the canonical link, the descriptions and the
  Open Graph tags, already done, are published together with the
  sitemap, the `robots.txt` and this check, by the user's choice.
- **Rejected:** publishing the tags first, without the check.

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

- **Known from the sitemap's card** ([card](../done/2026-11-07_add-sitemap.md), "How is
  it checked?"): this check, reading its pages from the sitemap, is also
  the sitemap's check in a browser; the Rust tests check that each
  canonical link matches its sitemap address, which Lighthouse does
  not.
- **To be answered when the card is resumed.** With the pages taken from
  the sitemap, the "page not found" is never checked, so the exemption
  derived from `noindex` never applies. Whether the local check also
  runs on the pages found in the built site's folder, where the "page
  not found" is, or the exemption is dropped.
- Whether the site needs a `robots.txt`, and what it says: moved to the
  card [add a robots.txt](../backlog/add-robots-txt.md).
