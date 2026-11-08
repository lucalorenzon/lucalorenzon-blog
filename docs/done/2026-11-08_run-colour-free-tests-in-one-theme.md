# Run in one theme the tests that do not check colours

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on making the browser tests check behaviour, not articles
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  first of three slices.
- **Status:** done, 2026-11-08.

## Why

Every browser test ran twice, in the dark and in the light theme
(`end2end/playwright.config.ts`, projects `chromium-dark` and
`chromium-light`). The light theme changes colours only: the
`prefers-color-scheme: light` block of `style/input.css` sets colour
variables and nothing else. So every check of size, position, keyboard,
address or tag gave the same result in both themes, and half of the 913
tests repeated the other half.

## Answers

### Which tests check colours?

Four, from the census in ADR-004; they run in both themes, tagged
`@colours`:

- **axe's contrast rules** (`color-contrast`, `color-contrast-enhanced`),
  split from axe's other rules into a test of their own, `contrast of
  ...`, on the same pages and at the same three widths. The other rules
  run in one theme, with the contrast rules disabled, so nothing is
  checked twice.
- **The colours of the section and role headings,** split from the
  content layout test into `heading colours of ...`, at one width
  (1280x720): a colour does not change with the width. The layout part
  keeps its 12 widths, a question for the next slice.
- **The colours of list markers.**
- **The border of the home page's boxes** (width, style, colour and
  corners in one test, too small to split).

The "page not found" test, which reads the page background to see that
the stylesheet is applied, stays in one theme: it adapts its expected
colour to the theme, and one theme is enough to see the stylesheet.

### Which theme runs everything else?

**The light one** (user, on the agent's proposal): it is what readers
with no preference see
([ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md)). The
dark project runs only the tests tagged `@colours` (`grep` on the
project).

**Rejected:** the dark one, the site's original look, seen only by
readers who ask for it.

### Does the assumption "the themes differ in colours only" need a guard?

**Yes** (user, on the agent's proposal): `end2end/tests/themes.spec.ts`
opens every page of the built site at the three widths of the header
(393, 768, 1280) twice, once preferring dark and once preferring light,
in two fresh browser contexts, and compares the position and size of
every element. It is one test, not one per page, so it does not grow
with the articles; a live run has no folder to walk and skips it.

**Rejected:** checking the stylesheet's text (that the light block sets
only colour variables): it would check how the page is built, not what
the reader sees, which the browser tests avoid.

## Results

- **Tests:** 913 before, 514 after: 484 in the light theme, 29 in the
  dark theme (21 contrast, 6 heading colours, the list markers, the box
  border), the SEO check once.
- **The guard was seen failing:** a rule moving `main` 3 pixels down in
  the light theme only, injected into the built stylesheet, made it fail
  with the elements and both positions; without it, it passes.
- **Pipeline** (build workflow, `main`): before, run 42496141109, 911
  passed in 2.6 minutes, job 189 seconds; after, run 42508735255, 513
  passed and 1 skipped in 1.5 minutes, job 127 seconds. The time falls
  less than the count: the tests removed were the dark copies, while the
  costly ones (readability and tables at 12 widths) remain, which is the
  next slice's subject.
- **Type of axe's results** taken from `@axe-core/playwright`, which the
  project declares, not from `axe-core`, which only comes with it.

Commit: `8641bca`.
