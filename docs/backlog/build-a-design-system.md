# Build a design system

- **Opened:** 2026-11-08, by the user, while classifying the browser
  tests in the card on making them check behaviour, not articles, since
  split ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  in Next.
- **Status:** noticed, not started.

## Why

Two reasons, one older and one new.

- **The look of the site.** When the curriculum went on the home page,
  the card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md)
  left a note: at the third article, the look is tied to a design
  system. The site now has two articles.
- **The tests.** The browser tests repeat the same checks on every page,
  and a new article adds about 200 of them. A design system separates
  the components one by one: what makes a component right (the contrast
  of its text on its background, its roles, the logo) is the same
  wherever it appears, so it is tested once, like a unit test. What
  remains on the pages is integration, checked by tests written once
  and run on every page.

The browser tests do not wait for this one: they put the components'
worst cases in test pages of their own (ADR-004), shaped as a catalogue
of components (one component per page, with its variants), so that the
design system can take them over.

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

To be answered by a timeboxed spike before choosing:

- Whether Holt's components and catalogue work for a site rendered to
  static HTML at build time, and with the WebAssembly islands the site
  will have soon (the menu is likely the first).
- Holt's licence, read at its source, and its maturity: version,
  releases, who maintains it.
- How the browser tests reach the catalogue's pages, in the build only:
  the catalogue is not published, so the check on the live site keeps
  to the net of tests over every page.
- What `tailwind_fuse` and Holt's way of composing classes mean for
  the named visual variants the site uses today
  ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md)).
