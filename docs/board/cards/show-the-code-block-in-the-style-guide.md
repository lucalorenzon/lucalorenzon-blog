# Show the code block in the style guide

- **Opened:** 2026-11-12, by the user, as the second page of the style
  guide in the card [build a design system](build-a-design-system.md),
  the first one about a component; in Now.
- **Status:** done, 2026-11-15. Not published yet: the style guide goes
  live when it is complete.

## Why

The code block is the smallest case of what the style guide is for:
taking a component's tests away from the articles. Today
`end2end/tests/code-blocks.spec.ts` checks the wrap toggle on both
articles, named one by one (`ARTICLES`), and runs axe on the code
blocks of the hybris article (`AXE_ARTICLE`), with a guard that stops
the build if that article no longer shows a block with the toggle and
one without. An edit made to an article for the text can stop the
build. A page of the style guide, with cases written for the purpose,
gives these tests a place of their own, and the two articles go back
to being content as far as code is concerned.

It is chosen before the table, which is larger, so the way a
component's page is made (its cases, the tests moved onto it, the
guards removed) is settled on a small case first and then reused.

## Known from the start

- **What the tests check today** (`code-blocks.spec.ts`, rows in
  `end2end/README.md`):
  - the toggle shows exactly when the code scrolls sideways, and
    checking it stops the scrolling, at the column widths;
  - Tab reaches the toggle and Space switches it;
  - a wrapped line continues four columns deeper than its own indent;
  - copying a block, wrapped or not, gives the code and nothing else;
  - axe, limited to the code blocks, finds no violation, and the
    contrast holds in both schemes, at 320 and 768.
- **The component does not change:** the code block is written by the
  Markdown converter (`src/markdown.rs`, `code_component`) from a
  fenced block; this card shows it and moves its tests, it does not
  redesign it.
- **What it touches, roughly:** a new page of the style guide (its
  generator in `src/style_guide/`, its text in `content/style-guide/`),
  listed by the index and the sitemap; `code-blocks.spec.ts` pointed at
  that page instead of the articles; the rows of `end2end/README.md`.
- **The page is covered by the net** with no test added, like the
  colours page: it is in the sitemap.

## Answers

- Where the cases live: in the page's Markdown, as fenced blocks the
  converter turns into components, or in the page's generator, as the
  colour samples are. Answered by the user on 2026-11-13: **in the
  Markdown**, written for the purpose, so the component is shown the
  way it works on every other page, through the same converter.
  Rejected: the cases in the generator, which would keep them fixed
  but show a code block that no author ever writes that way.
- Whether the tests check that the cases are still what they were
  written for. Answered by the user on 2026-11-13: **no test of the
  test.** The page's Markdown is the tests' material, not content: only
  someone changing the tests edits it, and that edit is reviewed as a
  change to the tests. What follows, agreed with the agent: since the
  cases are known, each test states what it expects of each case (the
  block with the long line scrolls and shows the toggle, the short one
  fits and does not), instead of today's general rule ("the toggle
  shows if and only if the code scrolls"), which was needed on articles
  the test did not know, and which a shortened line would pass without
  checking anything.
- Which cases the page needs. Answered by the user on 2026-11-13,
  from the agent's proposal: **the cases the tests of today need, plus
  a block indented with tabs and a blank line inside a block**, with no
  new test: the tests of the toggle, of the wrapped indent and of the
  copy run on every block of the page, so they cover the two new cases
  too. The tab is the one behaviour that only the browser can check:
  the code counts a tab up to the next multiple of four, the browser
  draws it with Tailwind's base style (`tab-size: 4`), and if the two
  disagree the toggle shows at the wrong width and a wrapped line
  restarts from the wrong indent. Rejected: browser tests for the
  escaping of `<` and `&` and for the language's class, already
  checked by the unit tests in `src/markdown.rs` (`code_is_escaped`,
  `fenced_code_becomes_the_component_with_its_language`); in the
  browser they would test the browser, not the site.
- Which cases the page needs, so that each test finds what it checks
  without a guard: a block that scrolls and one that fits, at every
  width tested; indented lines that wrap; what else.
- What the page's text says about the component, and where it is
  written. Answered by the user on 2026-11-14: **the text for the
  readers is the page's Markdown, cases included, and its author is
  always the user**, so one file with one author holds no conflict
  between text and tests. As for the colours page, the agent writes
  the cases and a placeholder for each piece of text, saying what kind
  of text goes there; the user writes the texts.

## The texts

The agent wrote the cases and a placeholder for each piece of text; the
user wrote the texts on 2026-11-14 and 2026-11-15, in Italian, one at a
time, and for each one the agent proposed an English version and the
changes it suggested, each with its reason; the user chose which to
apply. Changes made for accuracy, beyond the language:

- the 80-character limit is said to be the guidelines' highest level
  (WCAG 1.4.8, AAA), and counted in characters, the guidelines' word,
  not in graphemes;
- the toggle shows when the code does not fit the column, which changes
  with the screen, not when it exceeds those 80 characters;
- a tab is one character drawn as wide as the editor decides, not a
  number of spaces; and it is the site that tells the browser its
  width, four columns, and counts the same way;
- the toggle is switched, not clicked, since the keyboard switches it
  too.

## Results

- **The page and its tests** (commit `4f28717`): `style-guide/code.html`
  with five cases in `content/style-guide/code.md`;
  `end2end/tests/code-blocks.spec.ts` runs on it instead of the two
  articles, with one expectation for each case, found by its section's
  title, and without the guard on the hybris article. The cases measured
  on the built page: the text column is 704 px wide from 768 px up, the
  long line about 1,700 px. 254 browser tests became 259.
- **The texts** (commit `255ce37`).
- **What came out of it:** a breadcrumb and "Back to top" on every page
  of the style guide
  ([card](link-the-style-guide-pages-to-its-index.md)), and
  the wrap toggle to be turned into an icon before publishing
  ([card](turn-the-code-wrap-toggle-into-an-icon.md)).
- **The articles are no longer test pages for the code block.** They
  still are for the text and the tables, which the typography and table
  pages will take over.

