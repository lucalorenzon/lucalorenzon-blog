# Check every page with tests written once

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on making the browser tests check behaviour, not articles
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)); in Now, third of three slices.
- **Status:** noticed, not started.

## Why

The tests list the pages they check by hand (`TEXT_PAGES` in
`end2end/tests/support.ts`, `PAGES` and `TREE_PAGES` in
`accessibility.spec.ts`, `ARTICLES` in `code-blocks.spec.ts`,
`LISTED_ARTICLES` in `home.spec.ts`), with facts about their content:
titles, lists of sections, flags such as `hasRoles`. Every new article
means editing them, and a change of content breaks them although the
site still works. The tables tests walk the built folder instead, but
a live run has no folder and skips them.

[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md) calls "net" the checks written once and run on every
page, at one width and in one theme, for what only exists in a composed
page or depends on the text itself.

## Decisions to take

- **Where the list of pages comes from.** The sitemap is one candidate:
  it exists on the published site too, and the SEO check already reads
  it. It leaves out the "page not found" and any page kept out of
  search engines, which may still need the net.
- **Which checks make the net,** from the census: title, description,
  canonical link and Open Graph tags, one level-1 heading, axe's rules,
  heading order, sideways scroll at the narrowest width, at most 80
  characters per line at the widest column.
- **Which content facts are dropped and which become net checks,** and
  so which of the losses listed in [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md) are accepted.

## Known from the start

- **A check that finds nothing to check passes:** a net check on
  tables would pass on a site with no table. The components' own tests
  close that gap, in the slice on test pages that follows this one.
- **The live check** would run the net only: the components' test
  pages are not published.
