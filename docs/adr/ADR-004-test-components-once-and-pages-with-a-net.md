# ADR-004: Test each component once, and every page with tests written once

- **Date:** 2026-11-08
- **Status:** Accepted
- **Cards:** split from "make the browser tests check behaviour, not
  articles" (opened 2026-11-05, split on 2026-11-08) into
  [run in one theme the tests that do not check colours](../backlog/run-colour-free-tests-in-one-theme.md),
  [check at every width only what depends on the width](../backlog/check-at-every-width-only-what-depends-on-it.md)
  and
  [check every page with tests written once](../backlog/check-every-page-with-tests-written-once.md);
  related: [build a design system](../backlog/build-a-design-system.md)

## Context

The site is tested in a real browser: the tests open each page and
check what a reader sees. They grew one page at a time, as pages were
added, and on 2026-11-08 they were 913. About 200 of them belong to
each of the two articles, so every new article would add about 200
more, and in the end the build would no longer finish.

Many of these tests repeat the same check on every page, at 12 screen
widths and in both the dark and the light theme. Some also know facts
about the content: the titles of an article, the list of a page's
sections, whether a page has role headings. When the content changes,
these tests break although the site still works.

This repository holds the software that builds the site, not the
content. The user stated the goal: the tests certify that the site
behaves as expected; they must use real cases, but a component's wrong
behaviour does not depend on the article it appears in, so not every
check needs every page.

## Decision

Each test belongs to one of four classes, and each class is run as
often as its result can change, no more:

- **Frame:** the site's own pages and the frame shared by every page
  (home page, header, "page not found", logo, footer links). They do not
  grow with the articles and keep their checks.
- **Component:** a component's behaviour (code block, table and record
  viewer, lists, headings, contrast of a text on its background, roles)
  is checked once, like a unit test, on the case that stresses it most.
  A component that is right is right wherever it appears.
- **Net:** what only exists in a composed page, or depends on the text
  itself, is checked by tests written once and run on every page, at one
  width and in one theme: title, description, one level-1 heading, axe's
  rules, heading order, link texts, repeated identifiers, sideways
  scroll at the narrowest width, at most 80 characters per line at the
  widest column. The list of pages comes from the site, not from the
  tests.
- **Content facts** are taken out of the tests: rewritten as a
  component or a net check, or dropped.

Two rules go with the classes:

- **Only what can change the result multiplies a test.** The light
  theme changes colours only, so only checks on colours run in both
  themes. The 12 widths are kept for layout, not for keyboard, copying
  or links.
- **Real cases become fixed variants.** When a real article reveals a
  fault, its case becomes a variant of the component, kept as a
  regression test; the test no longer depends on the article.

The components' cases live in test pages built only for the build,
shaped as a catalogue of components (one component per page, with its
variants), so that a design system can take them over later. The work
does not wait for the design system.

## Consequences

**Positive:**

- A new article adds only the net's few checks, not about 200 tests.
- The tests no longer break when the content changes.
- The net takes its pages from the site, so it runs on the published
  site too; the components' test pages are not published and run in the
  build only.

**Negative / Risks:**

- A contrast fault of an element shown only at some widths, on a page
  that is not among the components' cases, would go unnoticed.
- A section lost by a fault of the software is no longer noticed by its
  title, only by the structure of the headings.
- A fault that only one article's content triggers, and that is not the
  worst case of its component (a long word that overflows only with the
  reader's text spacing, a table that breaks in another way), would go
  unnoticed until it becomes a variant.
- The test pages may be built twice: by hand now, then in the design
  system.

Which of these losses are acceptable is decided in the card of the net
and in the card of the test pages, check by check.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Test a random sample of articles | The same commit could pass today and fail tomorrow; and the published site is the tested one, so an article never drawn would go live looking checked. |
| Run every general check on every page found in the built site (the card's second proposal, partly in place for tables) | It removes the hand-written lists but not the growth: every article still multiplies every check by 12 widths and 2 themes. |
| Test pages invented for the tests only, with no real content | Without real articles there are no concrete cases; kept only as the place where real cases become variants. |
| Wait for the design system | The suite would keep growing by about 200 tests per article meanwhile. |

## Technical Notes

### Numbers on 2026-11-08

- 913 tests: 456 in the dark theme, 456 in the light theme, plus the SEO
  check once (`end2end/playwright.config.ts`, three projects).
- 398 of them name one of the two articles: 198 After Save Event, 200
  hybris PK.
- Build run 42496141109 on `main`: the browser test job took 189
  seconds, the longest job (the site's build job took 40).
- The light theme is the `prefers-color-scheme: light` block of
  `style/input.css`, which sets colour variables and nothing else.

### Census

Classes: F frame, C component, N net, K content fact. "Today" counts
both themes.

| Test (file:line) | Today | Pages | Class | Themes needed | Widths needed | What would be lost |
|---|---|---|---|---|---|---|
| readability:244, line height, paragraph gap, 80 characters, no justified text | 144 | `TEXT_PAGES` (6) | C, except the 80 characters | one | line height and gap: one; 80 characters: the column is widest from 1280 up | 80 characters depends on the glyphs of the text, so it is N, at one wide width |
| readability:275, text spacing of WCAG 1.4.12 | 144 | `TEXT_PAGES` | C | one | the 12 on the worst case | a long word of one article that overflows only with the reader's spacing, if not the worst case |
| content-layout:126, scroll, heading size and colour, footer gap, overlaps | 144 | `TEXT_PAGES`, with the flag `hasRoles` (K) | C; sideways scroll is N | colours: two; the rest: one | the 12 on the worst case; scroll: N at 320 | nothing, if sideways scroll moves to the net |
| content-layout:141, numbered list indented like a bulleted one | 2 | After Save Event, by name (K) | C | one | one | nothing: find the first page with both kinds of list |
| content-layout:173, list marker colours | 2 | After Save Event, by name (K) | C | two | one | nothing, as above |
| content-layout:210 and 263, home page boxes | 48 | home | F | one | 12 | nothing |
| header-layout:235 | 24 | home, with two invented 50-character titles | F | one | 12 | nothing; it already tests on invented worst cases, not on articles |
| accessibility:69, axe | 42 | 7 pages, listed (K) | N on every page; C on a set covering every kind of element | contrast: two; the rest: one | 3 on the covering set; one for the net | a contrast fault of an element shown only at some widths, on a page outside the covering set |
| accessibility:229, accessibility tree | 24 | 4 information pages, with their exact section titles (K) | F for the frame (banner, footer); N for the structure of the headings | one | 3 for the frame; one for the net | a section lost by a fault of the software, no longer noticed by its title |
| tables:30, single-value cells on one line | 70 | every page, with or without tables | C | one | the phone widths, on the worst case | nothing; today 50 of the 70 run on pages without tables |
| table-forms:74, 99, 164, the two forms, the viewer's rows, the keyboard | 144 | pages with tables | C | one | forms: 12; viewer rows: the widths where a viewer shows; keyboard: one | a table of another article, if not the worst case |
| table-forms:209 and 217, the hybris six-column table at 768 and 767 | 4 | hybris article, by name (K) | C | one | the threshold the build writes into the page, and one pixel below | nothing, if the test reads the threshold from the page instead of knowing it |
| code-blocks:24, the wrap toggle shows exactly when the code scrolls | 48 | 2 articles, listed (K) | C | one | 12 | nothing, if the worst case has both a block that scrolls and one that fits |
| code-blocks:54, 75, 122, keyboard, wrapped indent, copy | 12 | 2 articles, listed (K) | C | one | one | nothing |
| home:56 and 80, the articles listed, newest first, and their links | 6 | the two titles written in the test (K) | F, comparing with the articles in the built site | one | one | nothing |
| home:14, 92, 104, 130, 172 to 249 | 22 | home, about, licence; `FIRST_ARTICLE` (K) | F | colours (220): two; the rest: one | as today | nothing |
| canonical-link, open-graph | 28 | `TEXT_PAGES` (K) | N | one | one | nothing |
| not-found | 6 | "page not found" | F | one | one | nothing |
| seo | 1 | every page of the sitemap, in one test | N, already | one | one | nothing |

## References

- Related ADRs: [ADR-001](ADR-001-name-styles-by-visual-variant.md)
  (named visual variants, the starting point of a design system),
  [ADR-003](ADR-003-readability-first-amber-as-accent.md) (the colours
  the contrast checks protect)
- Session minute: `docs/sessions/2026-11-08-browser-tests-check-behaviour.md`
