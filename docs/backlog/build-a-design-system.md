# Build a design system

- **Opened:** 2026-11-08, by the user, while classifying the browser
  tests in the card on making them check behaviour, not articles, since
  split ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  in Next.
- **Merged:** 2026-11-09, by the user: the components' test pages of
  ADR-004, which had no card yet, are built by this card's catalogue
  instead of by hand before it. Moved to Now, after a round of the
  refactors left waiting.
- **Status:** noticed, not started.

## Why

Three reasons, one older and two newer.

- **The look of the site.** When the curriculum went on the home page,
  the card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md)
  left a note: at the third article, the look is tied to a design
  system. The site now has two articles.
- **The tests.** The browser tests used to repeat the same checks on
  every page, and a new article added about 200 of them. ADR-004 split
  them into components, checked once, and a net of checks written once
  and run on every page; 913 tests became 227. A design system finishes
  the job: what makes a component right (the contrast of its text on
  its background, its roles, the logo) is the same wherever it appears,
  so it is tested once, like a unit test.
- **Two articles have become test pages.** Until a catalogue exists,
  the components are checked on the real article that stresses them
  most, so the tests depend on the content, which ADR-004 meant to
  avoid (see "Known from the start"). Editing one of these articles for
  editorial reasons can stop the build. The user's view, which this
  card adopts: the design system separates the components from the
  pages for good. Each component lives in the catalogue and is tested
  there, on cases written for the purpose; the pages, articles
  included, are left to the integration checks of the net.

Why the test pages are not built by hand first, as ADR-004 first
decided: they were meant to stop the suite growing with every article,
and the net already did that. What is left is a question of method and
two small gaps, which can wait for the catalogue rather than be built
twice. The price: until then, the two articles stay test pages.

## Known from the start

- **Where the tests depend on an article today** (2026-11-09):
  - `end2end/tests/support.ts`, `WORST_CASE`: the After Save Event
    article, for the text checks of `readability.spec.ts` and
    `content-layout.spec.ts`, with a guard
    (`expectWorstCaseStillWorst`) that stops the build if the article
    loses one of the kinds of element those checks look at;
  - `end2end/tests/content-layout.spec.ts`: the numbered list and the
    list markers, on the After Save Event article by name;
  - `end2end/tests/code-blocks.spec.ts`: the wrap toggle, on both
    articles by name (`ARTICLES`), and axe on the code blocks of the
    hybris article (`AXE_ARTICLE`), with a guard of its own;
  - `end2end/tests/table-forms.spec.ts`: the six-column table of the
    hybris article at the width where it turns into a record viewer,
    and axe on its tables (`AXE_PAGE`).
- **A gap of the worst case:** level-4 headings are only in the
  curriculum, so the text checks do not see them; they enter only the
  overlap check of the net
  ([card](../done/2026-11-09_check-the-layout-on-every-page-with-the-net.md)).
- **A component check still run on every page:**
  `end2end/tests/tables.spec.ts` (single-value cells on one line) runs
  on every page of the built site at the four phone widths, 28 runs,
  20 of them on the five pages with no table, although ADR-004's
  census classes it as a component check.
- **The catalogue is not published:** the check on the live site keeps
  to the net, which takes its pages from the sitemap.

## Options known

- **[Holt](https://holt.rs/),** the user's first choice: a component
  library (Holt Kit, with behaviour kept apart from presentation) and a
  catalogue of components to develop and show them in isolation (Holt
  Book), for Leptos 0.8 and later, styled with Tailwind through
  `tailwind_fuse`.
- **Storybook,** as a fall-back: the user has used it at work. It does
  not know Leptos, so it would show the HTML the site's components
  produce, through a bridge to be written.

## Questions

To be answered by a timeboxed spike on Holt before choosing, the
card's first step:

- Whether Holt's components and catalogue work for a site rendered to
  static HTML at build time, and with the WebAssembly islands the site
  will have soon (the menu is likely the first).
- Holt's licence, read at its source, and its maturity: version,
  releases, who maintains it.
- How the browser tests reach the catalogue's pages, in the build only.
- What `tailwind_fuse` and Holt's way of composing classes mean for
  the named visual variants the site uses today
  ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md)).

To be answered when the catalogue is built:

- Which cases each component needs, written for the purpose, so that
  the two articles stop being test pages and their guards go away.
- Whether `tables.spec.ts` becomes a check of the table component, run
  once on its cases.
