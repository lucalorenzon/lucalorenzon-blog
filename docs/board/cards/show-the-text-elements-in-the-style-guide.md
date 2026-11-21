# Show the text elements in the style guide

- **Opened:** 2026-11-15, by the user, as the fourth page of the style
  guide in the card [build a design system](build-a-design-system.md),
  after [the table](show-the-table-in-the-style-guide.md).
  The card's typography page was cut in two by the user: this one, the
  elements of the text, comes first; the fonts have
  [a card of their own](show-the-fonts-in-the-style-guide.md).
  In Now.
- **Status:** done, 2026-11-16. Not published yet: the style guide goes
  live when it is complete.

## Why

The text checks are the last browser tests that use an article as a
test page. They run on the After Save Event article, chosen as the
worst case because it has every kind of element they look at, and a
guard stops the build if an edit made for the text takes one of them
away. A page of the style guide, with cases written for the purpose,
gives the text checks a place of their own, as the code block's and the
table's pages did, and the article goes back to being content. It is
also one of the pages the style guide needs before it is published.

Why the elements of the text come before the fonts: they free the
article; the fonts are mostly to be shown and move few tests.

## Known from the start

- **What the tests check today on the article** (rows in
  `end2end/README.md`), at the column widths (`COLUMN_WIDTHS`), each
  after the guard (`expectWorstCaseStillWorst` in `support.ts`):
  - `readability.spec.ts`, visual presentation (WCAG 1.4.8): line
    height of paragraphs and list items at least 1.5 times the font
    size, space between consecutive paragraphs at least 1.5 times the
    line height, no justified text;
  - `readability.spec.ts`, text spacing (WCAG 1.4.12): with the
    reader's spacing, consecutive blocks do not overlap, no element cuts
    off its content, the content ends at least 32 px above the footer;
  - `content-layout.spec.ts`: section headings larger than the body
    text, no overlap of consecutive headings, paragraphs and list items,
    the content clear of the footer;
  - `content-layout.spec.ts`, at one width and in both schemes: section
    and role headings in a colour other than the body text's.
- **Two tests name the article without the guard**, in
  `content-layout.spec.ts`: the numbered list shows its numbers and is
  indented and spaced like the bulleted list; the list markers take the
  amber accent while the item text keeps its colour.
- **The guard asks for more than text:** two consecutive paragraphs, a
  level-2 and a level-3 heading, a bulleted and a numbered list, and
  also a code block and a table. The check that nothing cuts off its
  content makes exceptions for the code's scroller and for the record
  viewer, which exist because the article has both.
- **A gap:** level-4 headings are only in the curriculum, so no text
  check sees them: the net has no overlap check, and the worst case has
  no level-4 heading.
- **How the headings stand out from the text** (`style/input.css`):
  level 2 larger, bold, in the heading accent; level 3 a little larger,
  bold italic, in the heading accent; level 4 of the text's size and
  colour, underlined only, like the links in the text.
- **Not part of this card:** the net's checks (the 80-character line,
  no sideways scroll), which run on every page and stay there; the
  home page's boxes test, in `content-layout.spec.ts`, which is about
  the home page.
- **The components do not change:** the text is written by the
  Markdown converter (`src/markdown.rs`); this card shows the elements
  and moves their tests, it does not redesign them.
- **What it touches, roughly:** a new page of the style guide (its
  generator in `src/style_guide/`, its text in `content/style-guide/`),
  listed by the index and the sitemap; `readability.spec.ts`,
  `content-layout.spec.ts` and the guard in `support.ts`; their rows in
  `end2end/README.md`.
- **The page is covered by the net** with no test added, like the other
  pages of the style guide: it is in the sitemap.

## Acceptance tests

Agreed with the user on 2026-11-16.

Slice 1, the page and its tests:

1. The build writes the style guide's text page,
   `style-guide/text.html`, titled "Text", linked from the index and
   listed in the sitemap, with its heading, description and canonical
   address: unit tests, as for the code blocks and tables pages.
2. The page holds the cases: headings of levels 2, 3 and 4 that wrap,
   two consecutive paragraphs, a bulleted and a numbered list whose
   items wrap; and bold, italic, inline code and a link, which the page
   says are shown only. It holds no code block and no table: unit test
   of the page.
3. The guard: before measuring, every case takes at least two lines at
   every column width, or the test fails naming the case: browser
   test. Proof that it bites: a case shortened in a throwaway run.
4. On the guide's page, at every column width, the checks that ran on
   the article pass: visual presentation (WCAG 1.4.8: line height,
   space between paragraphs, no justified text); text spacing (WCAG
   1.4.12); level-2 headings larger than the text, no overlap, the gap
   above the footer.
5. The text spacing check (WCAG 1.4.12) also runs on the code blocks
   and tables pages, at the same widths, with its exceptions for the
   code's scroller and the record viewer kept.
6. At one width, in both schemes: level-2 and level-3 headings differ
   in colour from the text; the level-4 heading is underlined, and the
   paragraphs' text is not.
7. On the guide's page: the numbered list shows its numbers and is
   indented and spaced like the bulleted list; the list markers take
   the amber accent while the items' text keeps its colour.
8. No text test names an article; `WORST_CASE` and
   `expectWorstCaseStillWorst` are gone: review of the diff.
9. The rows of the moved tests in `end2end/README.md` say what they
   now check and where; `readme.spec.ts` passes.
10. The checks of `build.yml` pass: format, lint, unit tests, type
    check, browser tests.

Slice 2, the texts:

11. Every placeholder of the explanations holds the user's text, and
    no case changed in this slice: review of the diff.

## Results

- **The page and its tests** (commit `844a954`): `style-guide/text.html`
  with its cases in `content/style-guide/text.md`;
  `readability.spec.ts` and `content-layout.spec.ts` run the text
  checks on it instead of the After Save Event article, after a guard
  that every case still takes two lines; the text spacing check also
  runs on the code blocks and tables pages; the level-4 heading is
  checked for its underline. No text test names an article any more,
  and `WORST_CASE` and `expectWorstCaseStillWorst` are gone.
- **A fault of the test it brought up** (commit `58774b8`): the
  accessibility check now scrolls to the end of the page before axe
  runs ([card](scroll-before-the-accessibility-check.md)).
- **The explanations** (commit `d4b2ffb`), written by the user, one
  paragraph each, and no case changed while they were written.
- **The guard bites**, proved on 2026-11-16 in a throwaway run: with
  the second item of the bulleted list shortened to "Short item.", the
  three text checks failed at all five column widths, 15 failures,
  each naming the case and the width ("at 320px, these cases of
  ./style-guide/text.html take one line, …: `<li> "Short item."`").
- **The checks of `build.yml`**, run locally on 2026-11-16 after the
  explanations: format, lint, 343 unit tests, type check, 247 browser
  tests passed, one skipped (the "page not found" test that runs on
  the live site only).
- **The articles are no longer test pages:** the text checks were the
  last browser tests that used one; the articles are still checked by
  the net, like every page.

## Answers

All answered by the user on 2026-11-16.

- **Which cases the page needs** (question 1): the kinds of element
  the text checks look at on the article today, plus the level-4
  heading. **Written on purpose to put the checks to the test**, with
  headings and list items that wrap: on the article some of that
  happens by chance, and the old guard only counted the elements.
  Rejected: short cases, with the elements merely present, which would
  let the overlap checks pass with nothing to check.
- **A guard aimed at the guide's cases:** every case still wraps at
  every column width, checked by the tests before they measure.
  Rejected: wrapping cases with their measures written in the card
  only, as for the tables page, since a case shortened for the text
  would leave its check with nothing to check unnoticed. This is not
  the guard ADR-004's cards removed: that one stopped the build for an
  edit made to an article for its readers; this one guards cases that
  exist for the tests.
- **No code block and no table beside the text** (question 2): the
  text spacing check, which today meets them only on the article,
  moves to the code blocks and tables pages too, so each component is
  checked once, on its own page. Rejected: a code block and a table on
  the text page, which would make it depend on two components that
  have pages of their own.
- **The elements inside a line** (question 3), bold, italic, inline
  code and links, are on the page **to be shown only**, without checks
  of their own, and the page says so, since every other case says what
  it puts to the test. Rejected: checks of their own, new decisions no
  text check needs today.
- **How the level-4 heading stands out:** a check of its own, on the
  underline as it is today. Accepted: the check also fixes its
  likeness to the links in the text, which are underlined too.
  Rejected: a check that it differs from the text in anything, which
  would accept any difference however small; redesigning it, which
  would be a card of its own.
- **Where the text checks run** (question 4): on the guide's page
  only, as for the code block and the table. Already today they ran on
  one page, so the other pages lose nothing; what goes is what the
  article put to the test by chance.
- **The texts** (question 5): the cases are written by the agent, for
  the tests; the explanations are written by the user. Why: the guard
  makes a case's text part of the test, so rewriting it shorter would
  stop the build.
- **The likeness of the level-4 heading to the links** is not noted in
  the backlog, decided by the user on 2026-11-16: the level-4 heading
  is used only in the curriculum, where it is two words above a list
  ("Main work"), not a long line in running text like the guide's case,
  so it is not taken for a link there.
- **The level-3 heading is called a subheading on the page**, decided
  by the user on 2026-11-16 while writing the explanations: "role
  heading" fits only the curriculum, while the articles use it for
  subsections. The browser tests' README says the same since this
  card's closing.
