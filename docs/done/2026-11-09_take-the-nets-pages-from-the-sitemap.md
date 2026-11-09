# Take the net's pages from the sitemap

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  first of its three parts, the other two being
  [check accessibility on every page with the net](2026-11-09_check-accessibility-on-every-page-with-the-net.md)
  and
  [check the layout on every page with the net](../backlog/check-the-layout-on-every-page-with-the-net.md).
- **Status:** done, 2026-11-09.

## Why

The tests listed the pages they check by hand (`TEXT_PAGES` in
`end2end/tests/support.ts`, `PAGES` and `TREE_PAGES` in
`accessibility.spec.ts`), so every new article meant editing them.
[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
calls "net" the checks written once and run on every page, and says the
list of pages comes from the site, not from the tests.

This card decided where that list comes from, and moved to it the two
checks that ADR-004's census marks as net with nothing lost: the
canonical link (`canonical-link.spec.ts`) and the description with the
Open Graph tags (`open-graph.spec.ts`).

## Answers

### Where does the list of pages come from?

**From the sitemap the build writes, `target/site/sitemap.xml`, in
both moments** (user). What is tested is what the build generated: on
every push the browser tests run on the build's own output, which
holds the sitemap; after a publication, the live check takes the
sitemap from the same output of the build that was published, and
opens the pages on the real server. The sitemap lists every page the
build decides to have indexed: today the home page, the curriculum,
the about and licence pages, and the two articles. The "page not
found" is left out on purpose: in ADR-004 it is a frame page, with
tests of its own, which stay as they were.

The live check copies **only `sitemap.xml`** into `target/site`, not
the whole built site (user, on the agent's finding): the table tests
walk that folder, and with the whole site there they would start
running on the published site, an effect nobody had decided.

**Rejected:**

- the sitemap served by the published site: the list would not come
  from the build, and it cannot be read while Playwright lists the
  tests (see below);
- generating the sitemap again in the live check (the user's first
  reading): it needs Rust and the git history in the job, while the
  build's output is already kept in the pipeline;
- walking the built folder, as the table tests do: it finds nothing on
  a live run, so those tests skip there.

### One test per page, or one test with a step per page?

**One test per page, the page's address being the parameter** (user).
Playwright lists the tests before running any, so the list is read from
the disk, synchronously, when `support.ts` is loaded (`NET_PAGES`). A
failure names its page, and the pages run in parallel.

A missing or empty sitemap throws: a net check with no page to check
must not pass. Since every test file loads `support.ts`, the whole
suite stops then, not only the net; without a built site most tests
failed already.

**Rejected:** one test with a step per page, as the SEO check does: the
pages run one after the other and the report shows a single test.

### How is the expected Open Graph type known?

**From the address, as before:** an article by its folder
`articles/`, a profile page by a list of two addresses. Reading it
from elsewhere would be a decision of its own, not taken here.

## Results

- **Tests:** 315, as before: the sitemap lists the same six pages as
  `TEXT_PAGES`. 314 pass; the one skipped is the "page not found"
  reached through a missing address, which runs on the live site only,
  as before.
- **The failure was seen:** with the sitemap moved away, the canonical
  link test stops with the message naming the file and how to build it.
- **The SEO check** keeps reading the sitemap from the server under
  test; the conversion from production address to path is shared, in
  `support.ts`.
- **Not yet seen running:** the live check's new steps
  (`.github/workflows/deploy.yml`, job `verify-live`) run only when a
  version tag is pushed. Reading the workflow, the built site is
  available to that job both when the snapshot is reused and when the
  site is rebuilt.

Commit: `5cc93a7`.
