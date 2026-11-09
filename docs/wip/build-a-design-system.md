# Build a design system

- **Opened:** 2026-11-08, by the user, while classifying the browser
  tests in the card on making them check behaviour, not articles, since
  split ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  in Next.
- **Merged:** 2026-11-09, by the user: the components' test pages of
  ADR-004, which had no card yet, are built by this card's catalogue
  instead of by hand before it. Moved to Now, after a round of the
  refactors left waiting.
- **Changed:** 2026-11-10, by the user: the catalogue is a style guide
  of our own, built with Leptos and published for the readers, instead
  of Holt or Storybook (see "Which catalogue"); the spike on Holt is no
  longer needed.
- **Status:** being worked on, since 2026-11-10.

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
- **The catalogue is published** (since 2026-11-10; until then it was
  meant to stay out of the site): its pages go in the sitemap, pass the
  SEO check and are covered by the net on the live site too, like every
  other page.
- **Cases written for the tests, pages written for the readers:** a
  case that stresses a component (a very long word, a six-column table
  on a narrow screen) can look odd next to the articles. Each case says
  what it puts to the test, so an extreme case reads as one.

## Which catalogue

- **Chosen: a style guide of our own.** Built with Leptos in the same
  build as the site, and published under `/style-guide`, with the title
  "Style guide", for the readers like the articles. One page per
  component, each about one aspect, without frills, on made-up texts
  that show the result. Why: each component gets tests aimed at it, on
  cases written for the purpose, rendered by the same code and the same
  stylesheet as the real pages; and a change of style, one day, is made
  in one place. The card keeps the name "design system" for the work
  behind the pages: the components set apart from the pages and tested
  once.
- **Rejected: [Holt](https://holt.rs/),** the user's first choice, a
  component library (Holt Kit) and a catalogue (Holt Book) for Leptos
  0.8 and later. The licence fits: Apache 2.0 or MIT, at the user's
  choice, the same in the workspace's `Cargo.toml`, the README, the
  licence files and on crates.io. The maturity does not (checked on
  2026-11-10):
  - Holt Kit is not published: it is a reference to copy into one's own
    project, as shadcn does
    ([architecture](https://github.com/aonyx-ai/holt/blob/main/docs/docs/explanation/architecture.md)).
    The only dependency left would be Holt Book, the catalogue.
  - Versions 0.1.0 (2026-02-16), 0.2.0 and 0.2.1 (2026-03-11), none
    since ([changelog](https://github.com/aonyx-ai/holt/blob/main/CHANGELOG.md)).
  - One maintainer: 93 of the 103 commits written by people are by one
    person, and the second stopped in July 2025. The last commit by a
    person is of 2026-03-19; since then only bots updating the
    dependencies. The project is not abandoned: its author opened an
    issue on 2026-10-29.
  - 2 stars, no fork, 67 open issues
    ([repository](https://github.com/aonyx-ai/holt)); Holt Book
    downloaded about 1,100 times
    ([crates.io](https://crates.io/crates/holt-book)).
  - The crate named `holt` on crates.io is another project; Holt's
    crates are `holt-book`, `holt-cli` and `holt-macros`.
- **Rejected: Storybook,** the fall-back, which the user has used at
  work. It does not know Leptos, so it would need a bridge to show the
  HTML the site's components produce; a catalogue of our own renders
  them directly.
- **Rejected: the name "Design system" for the readers:** too big a
  name for a site of this size. "Style guide" says the same with a
  sense of proportion.

## Questions

The spike on Holt is no longer needed. Its four questions were about
adopting Holt: licence and maturity are answered above; static HTML
and islands, and how the browser tests reach the catalogue, are the
same as for any other page of the site; `tailwind_fuse` against
[ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md) does not
arise without Holt.

To be answered while the style guide is built:

- Which component comes first.
- How the reader reaches the style guide, while the site has no menu
  yet.
- Which cases each component needs, written for the purpose, so that
  the two articles stop being test pages and their guards go away.
- Whether `tables.spec.ts` becomes a check of the table component, run
  once on its cases.
