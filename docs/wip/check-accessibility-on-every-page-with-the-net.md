# Check accessibility on every page with the net

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  second of its three parts, after
  [take the net's pages from the sitemap](../done/2026-11-09_take-the-nets-pages-from-the-sitemap.md).
- **Status:** in progress.

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

## Decided so far

- **The net runs axe at one width** (user). The elements that appear
  only at some widths, the record viewer of the tables and the code
  blocks' wrap toggle, are components: their accessibility is checked
  by their own tests, `table-forms.spec.ts` and `code-blocks.spec.ts`,
  with axe limited to the component, at the widths where it changes
  form. Those tests run on the real articles today and move to the
  components' test pages when these exist, so the check is never left
  without a home in between.
- **The structure of the headings is checked by three axe rules,
  enabled by name** (user): `page-has-heading-one`, `heading-order`,
  `empty-heading`. They belong to axe's "best practice" group, not to
  a WCAG criterion, so the selection by WCAG tags left them out.
  **Rejected:** the whole "best practice" group in the tests: a new
  version of axe could change its rules and stop the build for a rule
  nobody chose.
- **The whole "best practice" group can be run on demand, locally**
  (user), to see how the site fares. On 2026-11-09 the seven built
  pages, at the three widths and in both themes (42 runs), passed all
  of its 27 rules: 14 passed at least once, 13 found nothing to check
  (dialogs, images, frames, access keys, trees, skip links). A skip
  link is only checked when there is one; whether the site needs one
  belongs to the card on the WCAG criteria axe cannot check.

- **The banner and footer comparison becomes a net check of their
  shape** (user, on the agent's proposal), on every page of the
  sitemap: the banner holds the logo link "Lvk@73r, home page" and
  one level-1 heading with any non-empty text; `main` holds no
  level-1 heading; the footer holds the copyright and one link with
  any text. This also answers "exactly one level-1 heading". The exact
  texts stay with Rust's unit tests, which already check each page's
  heading and footer phrase (`FooterLicence` in `src/page_frame.rs`).
  The two expected trees, one per form of the logo, were identical,
  since the logo has the same name in both.
- **The loss of the section titles is accepted** (user): a section
  lost while a page is composed would no longer be noticed by the
  tests. The user asked for a periodic consistency check between the
  Markdown and the published text instead:
  [check periodically that the published text matches its Markdown](../backlog/check-published-text-against-markdown.md).

## Decisions to take

- **Repeated identifiers:** axe now checks only those referenced by
  accessibility attributes (`duplicate-id-aria`); its general rule was
  retired with WCAG 2.2's removal of criterion 4.1.1. Link texts are
  covered already (`link-name`, `identical-links-same-purpose`).

## Known from the start

- **The contrast rules run in both themes,** as a test of their own
  tagged `@colours`; every other axe rule runs once.
- **The "page not found" stays out of the net,** as in the first part:
  its axe check stays a frame test.
- **A comment in `accessibility.spec.ts` says the articles are not
  published,** but the sitemap lists them: to be checked when the file
  is changed.
