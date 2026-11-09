# Check periodically that the published text matches its Markdown

- **Opened:** 2026-11-09, by the user, while accepting a loss in the
  card
  [check accessibility on every page with the net](../done/2026-11-09_check-accessibility-on-every-page-with-the-net.md);
  in Later.
- **Status:** noticed, not started.

## Why

The browser tests no longer write each page's section titles
([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)).
So a whole section lost while a page is composed, for example a text
cut after its third section, or a section skipped in the middle, would
go unnoticed: each heading is still converted right (a unit test in
`src/markdown.rs`) and the structure of the headings stays valid. The
user accepted the loss in the tests, and wants a consistency check
between the Markdown used and the published text instead: not a test
run on every build, but a periodic check.

## Known from the start

- **The site already has a scheduled workflow,** `daily.yml`, which
  runs once a day and opens or updates an issue when it finds
  something (the check for newer versions of pinned tools).
- **The published site is the last version tag, not `main`:** the
  Markdown on `main` can be ahead of it. Compared with `main`, the
  check would report a difference every time a text changes before it
  is published.

## Questions

- Which Markdown the published text is compared with: the one of the
  commit of the published tag, which is what the site was built from.
- What "consistent" means: the same headings in the same order, the
  same text with the markup removed, or something in between.
- What it does when it finds a difference: fail the run, so GitHub
  mails it, or open an issue as the version check does.
- How often: daily with the other scheduled jobs, or less.
