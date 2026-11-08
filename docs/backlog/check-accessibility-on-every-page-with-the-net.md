# Check accessibility on every page with the net

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  second of its three parts, after
  [take the net's pages from the sitemap](../wip/take-the-nets-pages-from-the-sitemap.md).
- **Status:** noticed, not started.

## Why

`accessibility.spec.ts` lists its pages by hand: `PAGES`, seven pages
checked with axe at three widths, and `TREE_PAGES`, four information
pages whose accessibility tree is compared with the expected one,
section titles included. Every new article means editing them, and a
change of text breaks the tree test although the site still works.

ADR-004's census sorts them so:

- **axe:** net on every page at one width; at three widths only on a
  set of pages covering every kind of element, which belongs to the
  components' test pages;
- **accessibility tree:** frame for the banner and the footer, at three
  widths; net for the structure of the headings, in place of the
  section titles written in the test.

## Decisions to take

- **Which losses ADR-004 lists for this part are accepted:** a contrast
  fault of an element shown only at some widths, on a page outside the
  covering set; a section lost by a fault of the software, no longer
  noticed by its title but only by the structure of the headings.
- **What "structure of the headings" checks:** one level-1 heading,
  no level skipped, or more.
- **Which of the other net checks named by ADR-004 go here:** link
  texts and repeated identifiers, if axe does not cover them already.

## Known from the start

- **The contrast rules run in both themes,** as a test of their own
  tagged `@colours`; every other axe rule runs once.
- **The "page not found" stays out of the net,** as in the first part:
  its axe check stays a frame test.
- **A comment in `accessibility.spec.ts` says the articles are not
  published,** but the sitemap lists them: to be checked when the file
  is changed.
