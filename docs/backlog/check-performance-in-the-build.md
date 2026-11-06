# Check performance in the build

- **Opened:** 2026-11-02, by the user, after version 0.5.0.
- **Status:** noticed, not started.

## Why

The site is starting to carry more: version 0.5.0 added the Literata
font, and articles, images and navigation will follow. The build checks
accessibility and layout, but nothing tells whether a choice makes the
site slower or heavier. The user wants a tool in the automatic tests
that flags it, so the project does not drift off course without
noticing: Google Lighthouse, or something similar if it exists.

## Known from the start

- **An example of the drift already happened.** With Literata the page
  went from about 33 KiB (page, stylesheet, pixel font) to about 95 KiB,
  a little under three times as much; acceptable, decided knowingly, but
  no test would have said so.
- **Lighthouse** gives scores from 0 to 100 for performance,
  accessibility, best practices and SEO. Lighthouse CI
  (`GoogleChrome/lighthouse-ci`) runs it in a pipeline and can fail the
  build on thresholds.
- **Scores vary from run to run,** above all the timing-based ones, and
  more on shared CI machines; the usual remedy is several runs and the
  median. A check on sizes (bytes per kind of file, number of requests)
  is deterministic instead: same page, same result.
- **Lighthouse comes first for SEO:** on 2026-11-06 the user chose
  Lighthouse for the search-engine check, knowingly anticipating this
  card's choice of tool
  ([card](../wip/check-seo-with-lighthouse.md)). Once that card is done, the
  performance category may be added to the same run, and the question
  "which tool" below is mostly answered.
- **The build already starts a browser** (Playwright, Chromium) on the
  built site served locally, and again on the live site after each
  publication; a performance check could run in either moment, or both.

## Questions

- Which tool: Lighthouse CI, Lighthouse driven from the Playwright
  tests, or an alternative.
- What is checked: the scores, a budget of sizes and requests, or both;
  and with which thresholds.
- Where it runs: on the local build, on the live site after
  publication, or both.
- How the variance of the scores is kept from making the build fail at
  random.
