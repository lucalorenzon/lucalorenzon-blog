# Show the code block in the style guide

- **Opened:** 2026-11-12, by the user, as the second page of the style
  guide in the card [build a design system](build-a-design-system.md),
  the first one about a component; in Now.
- **Status:** being worked on, since 2026-11-12.

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

## Questions

- Where the cases live: in the page's Markdown, as fenced blocks the
  converter turns into components, or in the page's generator, as the
  colour samples are.
- Which cases the page needs, so that each test finds what it checks
  without a guard: a block that scrolls and one that fits, at every
  width tested; indented lines that wrap; what else.
- What the page's text says about the component, and where it is
  written.
