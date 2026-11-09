# Check accessibility on every page with the net

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  second of its three parts, after
  [take the net's pages from the sitemap](2026-11-09_take-the-nets-pages-from-the-sitemap.md).
- **Status:** done, 2026-11-09.

## Why

`accessibility.spec.ts` listed its pages by hand: `PAGES`, seven pages
checked with axe at three widths, and `TREE_PAGES`, four information
pages whose accessibility tree was compared with the expected one,
section titles included. Every new article meant editing them, and a
change of text broke the tree test although the site still worked.

## Answers

### At how many widths does axe run on every page?

**One, the laptop's 1280 pixels, on every page of the sitemap** (user;
the width is the agent's proposal, the one ADR-004 gives the 80
characters). The other rules run once, in the light theme; the
contrast rules, as a test of their own tagged `@colours`, in both.

What changes with the width is checked where it belongs:

- **The frame,** whose header changes form with the width (text logo,
  drawn logo, title below or beside), keeps axe at the three widths,
  on the home page and on the "page not found" (agent's proposal,
  user's decision): the frame is the same on every page, so one
  content page suffices. The "page not found" runs locally only, as
  before: only the host serves it for a missing address.
- **The components,** the record viewer of the tables and the code
  blocks' wrap toggle, which appear only at some widths: axe limited
  to the component, in their own tests (`table-forms.spec.ts`,
  `code-blocks.spec.ts`), **once, on the worst case, at two widths**
  (user). Today's worst case is the hybris article at 320 and 768
  pixels: measured on 2026-11-09, at 320 it shows both forms of a
  table and code blocks with and without the toggle, at 768 the table
  form and both states of the code. A guard in each test fails if the
  article stops showing them. The checks move to the components' test
  pages when these exist.

**Rejected:** axe on the components on every article at the five
widths of the text column (the first version of the code): 60 more
tests and about 70 more seconds of work, growing by 15 tests per
component for every new article, the growth ADR-004 is meant to stop.

### What replaces the section titles written in the test?

**The structure of the headings, by three axe rules enabled by name**
(user): `page-has-heading-one`, `heading-order`, `empty-heading`. They
are in axe's "best practice" group, not tied to a WCAG criterion, so
the selection by WCAG tags left them out. The test checks that the
three actually ran on each page.

**The loss is accepted** (user): a section lost while a page is
composed (a text cut after its third section, a section skipped) is no
longer noticed by the tests; each heading is still converted right (a
unit test in `src/markdown.rs`) and the structure stays valid. The
user asked for a periodic consistency check instead:
[check periodically that the published text matches its Markdown](../backlog/check-published-text-against-markdown.md).

**Rejected:** the whole "best practice" group in the tests: a new
version of axe could change its rules and stop the build for a rule
nobody chose. The whole group runs **on demand**, locally, with
`end2end/tools/check-axe-best-practices.mjs` (user). On 2026-11-09 the
seven built pages, at three widths and in both themes (42 runs),
passed all of its 27 rules: 14 passed at least once, 13 found nothing
to check (dialogs, images, frames, access keys, trees, skip links). A
skip link is checked only when there is one; whether the site needs
one belongs to the card on the WCAG criteria axe cannot check.

### What becomes of the banner and footer comparison?

**A net check of their shape, on every page** (agent's proposal,
user's decision): the banner holds the logo link "Lvk@73r, home page"
and one level-1 heading with any non-empty text; `main` holds none;
the page has exactly one; the footer holds the copyright and one link
with any text. The exact texts stay with Rust's unit tests, which
already check each page's heading and footer phrase (`FooterLicence`
in `src/page_frame.rs`). The two expected trees, one per form of the
logo, were identical, since the logo has the same name in both.

### Repeated identifiers, and link texts?

**A check of our own: no identifier used twice in a page** (user).
axe now checks only identifiers referenced by accessibility
attributes (`duplicate-id-aria`): its general rule was retired with
WCAG 2.2's removal of criterion 4.1.1. The built pages have no
identifier today, so the check guards the first component that adds
one. **Rejected:** leaving it to axe (agent's proposal).

Link texts were covered already, by `link-name` and
`identical-links-same-purpose`.

### And an empty header cell in a table?

**Refused by the build, not checked by the tests** (user). A fault
made on purpose, an emptied header cell, passed the tests: axe's rule
for it, `empty-table-header`, is a best practice. The build does not
check it either: Markdown allows `| | Value |`, and the record viewer
would show a value with an empty label. A rule in the build
guarantees it for every article, so it went to the card
[check the pages' text in the build](../backlog/check-page-bodies-in-the-build.md).
**Rejected:** enabling the rule by name in the table's test.

## Results

- **Tests:** 315 became 300, and their summed work 178.9 seconds
  became 170.4 (locally, 14 workers: 40.8 and 41.8 seconds of wall
  time):

  | File | Tests | Work |
  |---|---|---|
  | `accessibility.spec.ts` | 75 → 48 | 58.9 → 37.8 s |
  | `code-blocks.spec.ts` | 16 → 22 | 13.2 → 21.2 s |
  | `table-forms.spec.ts` | 24 → 30 | 10.1 → 17.3 s |

  A new article now adds five accessibility tests to the net.
- **Faults made on purpose,** in the built site:
  - a level-1 heading inside `main` of the about page: the shape test
    fails;
  - the same identifier on two elements of the licence page: the
    identifier check fails;
  - a level-4 heading right after the level-1: the net's axe test
    fails on `heading-order`;
  - an empty link inside a cell of the hybris article's table: the
    table's axe test fails at both widths;
  - an emptied header cell: not caught, see the last answer.
- **The stale comment** saying the articles were not published is
  gone with `PAGES`: the net takes them from the sitemap, also on the
  live site.

Commits: `43f415e` (the on-demand check), `ed27a1d` (the tests).
