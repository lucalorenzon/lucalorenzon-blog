# Make the browser tests check behaviour, not articles

- **Opened:** 2026-11-05, by the user, reviewing the browser test of
  the rule "a table cell with no space never wraps", in the card
  [let the Markdown ask for components for wide code and tables](../done/2026-11-05_annotate-markdown-for-wide-content.md);
  in Next.
- **Status:** being worked on, since 2026-11-08.

## Why

The first version of that test looked for the PK bits table of the
hybris article and for its values ("63–44", "43", "42–15", "14–0").
The user found it wired to one article rather than checking that the
behaviour is right: if the article changed, the test would break with
the rule still working, and it would say nothing about a table in
another article. It was rewritten before its commit as a general rule
over the site's pages; the same weakness is in the tests already
there.

The unit tests in Rust do not have it: they check the conversion on
small invented Markdown. The browser tests do:

- `end2end/tests/support.ts` lists the pages the content and
  readability tests check (`TEXT_PAGES`), each with flags about its
  content (`hasRoles`, `hasConsecutiveParagraphs`), and a comment on
  which article stays out and why.
- `code-blocks.spec.ts` lists the two articles (`ARTICLES`) and skips
  one check for the hybris article by name.
- `accessibility.spec.ts` has its own list of pages (`PAGES`) and of
  expected accessibility trees per page (`TREE_PAGES`).

Each new article means editing these lists, and the tests know facts
about the content that belong to the content.

## Proposal

Open, to be decided when the card is picked up. Two roads, not
exclusive:

- **Test pages built only for the tests,** with content made to
  exercise each rule (a table with single values, a wide table, wide
  code, consecutive paragraphs), kept out of the published site. The
  tests then know their fixtures, not the articles.
- **General properties over every page,** found without naming them
  (for example from a list the build writes, or by following the
  site's links once the navigation exists): every page has no
  sideways scroll, every single-value cell is on one line, and so on.

## Known from the start

- **A general property can pass by checking nothing:** if no page had
  a table, the table test would pass with no cell checked. Test pages
  built for the tests would close that gap; a general property alone
  needs a guard.
- **Pages left out on purpose** (today the hybris article, until its
  wide table is handled) need a way to be declared that does not live
  inside each test.
- **The live check:** the same suite runs on the published site after
  each publication (`BASE_URL`); test pages kept out of the site would
  not exist there.

## Census of the browser tests (2026-11-08)

Working section, to be refined in the discussion and rewritten as answers
when the card closes.

### The problem, restated by the user

The repository holds the software that builds the site, not the content.
The browser tests grew page by page as pages were added: many repeat the
same check on each page, and several know facts about one article. So
every new article makes the suite larger, and in the end the build would
no longer finish. The tests must still use the real pages, because only
they give concrete cases; but a component's wrong behaviour does not
depend on the content, so not every check needs every page.

### Numbers

- 913 tests: 456 in the dark theme, 456 in the light theme, plus the SEO
  check once.
- About 400 of them belong to the two articles, about 200 each: a new
  article would add about 200.
- The multipliers: page × 12 screen widths × 2 themes.
- Last build on `main` (run 42496141109): the browser test job took
  189 seconds, the longest job of the build (the site's build job took
  40).
- The light theme changes colours only (`style/input.css`, the
  `prefers-color-scheme: light` block sets colour variables and nothing
  else): every check of size, position, keyboard or address gives the
  same result in both themes.

### Classes

- **Frame (F):** behaviour of the site's own pages and of the frame
  (home page, header, "page not found", logo, footer links). They do not
  grow with the articles.
- **Component, worst case (C):** a component's behaviour, checked once on
  the real element of the site that stresses it most, which the test
  finds by itself (the longest code line, the table with the most
  columns), not by naming the article.
- **Net (N):** a cheap check run on every page, at one width and in one
  theme, for what the content itself can break.
- **Content fact (K):** the test holds facts about the content (titles,
  lists of sections, flags about a page): to be rewritten as C or N, or
  dropped.

### Census

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

### First observations

- **The themes double almost everything for nothing.** Only the checks
  on colours need both: axe's contrast, the heading and marker colours,
  the home page box border.
- **The 12 widths are needed for layout, not for behaviour.** Keyboard,
  copy and links do not change with the width.
- **The test of the header already works the "worst case" way,** on
  invented titles rather than on articles: whether the worst case is
  invented or taken from the real pages is a question still open.
- **The live check** runs today every test that does not skip itself;
  the net is the natural candidate for it, to be decided.

### Agreed so far

1. **The classification above holds** (user): frame, component on its
   worst case, net, content facts to rewrite.
2. **Components are tested once, like unit tests; pages are tested by a
   parametric net** (user): a component that is right (contrast of its
   text on its background, its roles, the logo) is right everywhere, so
   its tests do not repeat per article; what remains is integration,
   written once and run on every page (title, description, one level-1
   heading and so on). The agent added that some properties only exist
   in the composed page (heading order, link texts, repeated
   identifiers, keyboard order) and stay in the net, as do the two
   checks that depend on the text (80 characters, sideways scroll at
   320).
3. **Real cases become fixed variants** (agent, accepted with the
   order below): when a real article reveals a fault, that case becomes
   a variant of the component, as a regression test, instead of the
   test depending on the article.
4. **This card does not wait for a design system** (agent's proposal,
   user's decision): it cuts the multipliers that do not change the
   result, brings in the net, removes the content facts, and puts the
   worst cases in test pages built only for the build, shaped as a
   catalogue of components (one component per page, with its variants)
   so that a design system can take them over. The design system has a
   card of its own:
   [build a design system](../backlog/build-a-design-system.md).

Still open: which of the three losses listed in "First observations"
are acceptable.
