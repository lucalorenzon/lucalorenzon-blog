# Take the net's pages from the sitemap

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  first of its three parts, the other two being
  [check accessibility on every page with the net](../backlog/check-accessibility-on-every-page-with-the-net.md)
  and
  [check the layout on every page with the net](../backlog/check-the-layout-on-every-page-with-the-net.md).
- **Status:** in progress.

## Why

The tests list the pages they check by hand (`TEXT_PAGES` in
`end2end/tests/support.ts`, `PAGES` and `TREE_PAGES` in
`accessibility.spec.ts`), so every new article means editing them.
[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
calls "net" the checks written once and run on every page, and says the
list of pages comes from the site, not from the tests.

This part decides where that list comes from, and moves to it the two
checks that ADR-004's census marks as net with nothing lost: the
canonical link (`canonical-link.spec.ts`) and the description with the
Open Graph tags (`open-graph.spec.ts`), today run on `TEXT_PAGES`.

## Known from the start

- **The sitemap lists six pages:** the home page, the curriculum, the
  about and licence pages, and the two articles. It exists on the
  published site too, so a live run can read it, while walking the
  built folder (`pagesOf` in `support.ts`) finds nothing when live.
- **The SEO check reads it already** (`sitemapPaths` in
  `seo.spec.ts`), and maps each production address to the server under
  test by keeping its path.
- **The "page not found" is not in the sitemap.** In ADR-004 it is a
  frame page, with tests of its own (`not-found.spec.ts`, and its own
  test in both files of this part), so the net leaving it out is
  consistent with the decision already taken.
- **Playwright lists the tests before running them.** Reading the
  sitemap over the network to create one test per page is not
  straightforward; the SEO check runs one test with a step per page.
- **The Open Graph test decides the expected type from the address:**
  an article by its folder, a profile page by a list of two addresses.

## Questions

- One test per page, read from the sitemap file when it is local, or
  one test with a step per page, the same locally and live.
- Whether the expected Open Graph type can be read from something other
  than a list of addresses.
