# Check SEO with Lighthouse in the build

- **Opened:** 2026-11-06, by the user, splitting the card on SEO and
  Open Graph tags, opened on 2026-11-02; the other parts are
  [add the description and the Open Graph tags to every indexed page](../done/2026-11-06_add-description-and-open-graph-tags.md)
  and [add preview images](../backlog/add-preview-images.md). In Now, after the
  description and the Open Graph tags.
- **Status:** being worked on, since 2026-11-06; on hold from
  2026-11-06 to 2026-11-07, waiting for the cards
  [add a sitemap](../done/2026-11-07_add-sitemap.md) and
  [add a robots.txt](../done/2026-11-07_add-robots-txt.md), both done on
  2026-11-07; it stayed in `docs/wip/` meanwhile, by the user's choice.
  Resumed on 2026-11-07; done, on 2026-11-07.

## Why

The user wants the build to check automatically that every page meets
the search-engine rules, so a page published without its description,
its title or its canonical link does not go unnoticed. Some pages must
be able to stay out of it, through a block-list.

## Answers

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

### How many times does it run?

Chosen by the user on 2026-11-07: once, in a third group of tests
(a Playwright "project") of its own, beside the dark and the light
theme. That group runs only the SEO check's file, and the two theme
groups leave it out.

- **Why once:** the search-engine rules do not depend on the colour
  scheme, so two runs would give the same result in twice the time.
  Lighthouse also opens a Chromium of its own, with its control port,
  so it does not use the browser Playwright prepares for each theme.
- **Why a group of its own:** the rule "it runs once" is written where
  the reader looks to see how the suite runs, the configuration, and
  the report shows the check as an entry of its own.
- **Its price:** the two theme groups each carry the exclusion of the
  SEO check's file.
- **Rejected:** the SEO check skipping itself on the light theme, which
  leaves the configuration unchanged but fills the report with skipped
  tests and hides in the test the choice of the dark theme, which has
  no reason behind it.

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
- **Audited when installed, on 2026-11-07:** 135 packages came with it,
  more than the trial run's estimate; `npm audit` found no
  vulnerability. Every licence is permissive (MIT, Apache 2.0, BSD, ISC,
  BlueOak and similar) but one: the package `sentry` 0.45.0, Sentry's
  command line, brought in by `@sentry/node` through
  `@sentry/bundler-plugins`, under the Functional Source License 1.1,
  which forbids only using it to build a product competing with Sentry
  and becomes Apache 2.0 after two years. Accepted, with the user
  informed: it has no script that runs at installation, the check never
  calls it, and it stays a development dependency, never part of the
  published site.
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

Chosen by the user on 2026-11-07, revising the answer of 2026-11-06: no
page is exempted. The check runs only on the pages of the sitemap, both
on the local build and on the live site, and every rule of the category
applies to each of them. A page stays out of the check by staying out
of the sitemap, which the build decides from `Indexing::Indexed` or
`NoIndex` (`src/page_frame.rs`, `src/sitemap.rs`): today only the
"page not found" is `NoIndex`.

- **Why revised:** the first answer let a page that carries
  `<meta name="robots" content="noindex">` skip the rules on indexing
  and on description (`is-crawlable`, `meta-description`, through
  Lighthouse's option `skipAudits`). Once the pages came from the
  sitemap, no page with `noindex` would ever reach the check, so that
  code would never run.
- **One list, the one search engines read:** the check is about the
  pages the site asks to have indexed, and the sitemap names exactly
  those. Lighthouse has no notion of a set of pages: it checks the one
  address it is given, so which pages are checked is always the test's
  choice.
- **What is left unchecked, and why it is accepted:** the "page not
  found" never goes through Lighthouse. Its remaining rules (title,
  links, response status) serve search engines, which that page asks
  not to index it; the alternative text of images, the one rule that
  matters outside search too, is already checked by axe in the
  accessibility test.
- **Its price:** a page marked `NoIndex` by mistake leaves both the
  sitemap and this check, in silence.
- **The tags go with it:** a page with `noindex` has no description and
  no Open Graph tags, as decided by the card
  [add the description and the Open Graph tags](../done/2026-11-06_add-description-and-open-graph-tags.md);
  this card does not change that.
- **Rejected:** on the local build, the sitemap's pages plus those found
  in the built site's folder, keeping the exemption for the "page not
  found": every built page would be checked, but locally the pages would
  come from two sources, and the exemption's code would serve one page
  that asks not to be found. Rejected on 2026-11-06 already: a list of
  exempted pages kept in the test, which would be a second declaration
  beside the build's, free to disagree with it; a `robots.txt` that
  disallows the "page not found", which Lighthouse counts as blocked
  too, so the rule would fail anyway, and which would hide the
  `noindex` from search engines, as Google warns.

### On which pages, and why on hold?

Chosen by the user: the check takes its list of pages from a sitemap,
which the site does not have yet. The card is on hold until the sitemap
and the `robots.txt` exist, each in a card of its own.

- **The problem:** on the live site there is no folder to walk, so the
  check needs a list of pages, and the only one, `TEXT_PAGES` in
  `end2end/tests/support.ts`, is written by hand; the card
  [make the browser tests check behaviour](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
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
  Revised since: no page is exempted, see "How is a page exempted?".
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

## Results

- **Built in one commit** (`78a2765`): the `lighthouse` package at
  13.6.0 in `end2end/`, a third Playwright project `lighthouse` beside
  the two themes, which leave its file out, and the test
  `end2end/tests/seo.spec.ts`.
- **How the test reads the sitemap:** it asks the server under test for
  `sitemap.xml` and keeps the path of each address, since the addresses
  always name `https://www.lucalorenzon.com/`; locally it opens that
  path on the local server. On the live site only the reading of the
  sitemap is retried, for up to a minute, while GitHub Pages serves the
  new version: six Lighthouse runs one after the other would not fit in
  that minute, and the sitemap and the pages are published together.
  This path runs for the first time with the next publication.
- **On the local build, 2026-11-07:** the six pages of the sitemap
  (home, curriculum, about, licence, the two articles) give the same
  result: nine rules pass (`is-crawlable`, `document-title`,
  `meta-description`, `http-status-code`, `link-text`,
  `crawlable-anchors`, `robots-txt`, `hreflang`, `canonical`),
  `image-alt` is not applicable, since there are no images yet, and
  `structured-data` is manual, so ignored. The rule on `robots.txt`,
  not applicable in the trial run, now applies, since the site has one.
  About 5 seconds a page, 30 seconds in all.
- **It can fail:** with the description removed by hand from the built
  `about.html`, the test stopped with
  `http://127.0.0.1:4173/about.html: meta-description (Document does not
  have a meta description): mode binary, score 0`.
- **Whole suite:** 849 browser tests passed and 2 skipped, the SEO check
  included; 199 Rust tests passed. The workflows needed no change: both
  run `npx playwright test`, which runs the new project too.
- **It is also the sitemap's check in a browser,** as decided in the
  sitemap's card ([card](2026-11-07_add-sitemap.md), "How is it
  checked?"): a page listed there that does not answer, or answers
  without its title or description, fails here. The Rust tests check
  that each canonical link matches its sitemap address, which
  Lighthouse does not.
