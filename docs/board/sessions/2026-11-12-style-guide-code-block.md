# Session 2026-11-12 — The style guide's code block

Held from 2026-11-12 to 2026-11-15.

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Claude Sonnet 5.5 (two tasks: the code of the code block
page with its tests, and the code of the breadcrumb and "Back to top",
corrected once on the agent's request).
**Goal:** start again from the plan, and do the style guide's second
page,
[show the code block in the style guide](../cards/show-the-code-block-in-the-style-guide.md),
within the card [build a design system](../cards/build-a-design-system.md).
Technical details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Push the commits waiting since the last version, without a version tag | user | agent | — |
| 2 | Publish the style guide only when complete: colours, code block, table, typography, breakpoints | user | agent | — |
| 3 | The article on the style guide is the site's third article and the readers' way into it; the style guide stays a section of its own, not the article itself | user | agent | — |
| 4 | The pages in this order: code block, table, typography (fonts and the elements of the text), breakpoints | user | agent | — |
| 5 | The code block page's cases live in its Markdown, through the same converter as the articles | user | agent | — |
| 6 | No test of the test: each test states what it expects of each known case | user | agent | — |
| 7 | The cases of today's tests plus a tab and a blank line, with no new test; escaping and the language's class stay with the unit tests | user | agent | — |
| 8 | The page's Markdown holds both the text and the cases, and its author is always the user | user | agent | — |
| 9 | A breadcrumb at the top of each page of the style guide, starting from "Style guide", and "Back to top" at the bottom; for the style guide only | user | agent | — |
| 10 | The articles get a breadcrumb and a way back to the top in a card of their own | user | agent | — |
| 11 | The bottom link says "Back to top", not "Up" | user | agent | — |
| 12 | The breadcrumb outweighs the text column's list rules by specificity, as the colour samples do, not by `!important` | agent | — | user |
| 13 | Turn the wrap toggle into an icon, before publishing, in a session of its own | user | agent | — |
| 14 | The code block page's texts: the user's versions, with the changes the user chose among the agent's suggestions | user | agent | — |

## Discussion

1. **Where the work stood.** The user asked to start again from the
   plan. The agent summarised it: the refactors and the colour page done
   but not published, and the article waiting for the colour page to be
   live; it pointed out that publishing would put the style guide online
   with no link to it. The user chose to push without publishing, and to
   publish only once the style guide is complete. → decisions 1, 2
2. **The style guide as an article.** The user proposed publishing the
   style guide as a third article. The agent set out two readings: an
   article that leads into the style guide, or the style guide's pages
   as the article, which would turn its test cases into editorial
   content again. The user meant the first. → decision 3
3. **The order of the pages.** Asked what the agent would choose, it
   proposed the code block first, as the smallest case of moving tests
   off the articles, then the table, the fonts, the breakpoints; and
   pointed out that none of the four pages covered the text elements
   the After Save Event article is tested on. The user confirmed the
   whole aim is isolating components, and the typography page takes
   them. → decision 4
4. **Where the cases live.** The agent set the cases in the Markdown
   against the cases in the generator. The user chose the Markdown,
   written for the purpose. → decision 5
5. **A test that passes without checking.** The agent pointed out that
   the toggle's rule ("shown if and only if the code scrolls") passes
   without checking anything if a case is shortened. The user answered
   that nobody but the tests' author edits that file, and that a test
   of the test makes no sense. The agent agreed, and proposed writing
   each test with one expectation per known case, which the general
   rule only needed on unknown articles. The user agreed. → decision 6
6. **Which cases.** The agent listed what the component does beyond
   today's tests. The user doubted it was worth testing the library;
   the agent separated what the unit tests already cover from the tab,
   the one point only the browser can check, and proposed adding the
   tab and a blank line with no new test. The user agreed. → decision 7
7. **Text and cases in one file.** The agent saw a tension between the
   file as test material and the file as text for the readers. The user
   answered that the Markdown is the readers' text and its author is
   always the user, so there is no conflict. → decision 8
8. **The way back to the index.** The user noticed the pages need a
   link to the index. The agent pointed out it would be the site's
   first link of its kind; the user kept it to the style guide, the
   menu coming later. The user first thought of the bottom of the page;
   the agent suggested a breadcrumb at the top, without "Home"; the
   user added "Back to top" at the bottom. The agent noted that the
   longest pages are the articles; the user kept it to the style guide
   for now and asked for a card for the articles. The user proposed
   "Up", then "Return at begin"; the agent explained why "Up" reads as
   the level above and suggested the usual phrase. → decisions 9, 10,
   11
9. **The sub-agent's `!important`.** Reviewing the sub-agent's work, the
   agent found `!important` where the stylesheet already solves the
   same problem by specificity, and had it aligned before the commit;
   the screenshots before and after were identical. The user was told
   and approved the commit. → decision 12
10. **The wrap toggle as an icon.** The user asked for a card, then
    placed it before publishing, in a session of its own. → decision 13
11. **The texts.** The user wrote each text in Italian; the agent
    translated it and suggested changes, each with its reason, several
    of them for accuracy; the user chose which to apply, every one of
    them in the end. → decision 14

## Outcome

- The publishing decision and the article's role recorded (`0f051b1`).
- The code block page's card started and answered (`3d3c950`,
  `d347597`); the page and its tests moved off the articles
  (`4f28717`); the texts (`255ce37`); the card closed (`0ac48df`).
- The breadcrumb and "Back to top": card started and answered
  (`5299cf1`, `b231db4`), code (`c8b9c91`), card closed (`9c33df1`).
- The icon card noted and placed before publishing (`071730c`,
  `2ab94f7`); the articles' card noted with `b231db4`.
- 254 browser tests became 263; 311 unit tests became 318.
- Pushed at the start of the session, up to `06a34c1`; the commits of
  this session are not pushed.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Turn the wrap toggle into an icon, in a session of its own: card
  [turn the code wrap toggle into an icon](../cards/turn-the-code-wrap-toggle-into-an-icon.md).
- The table page, then the typography and the breakpoints: card
  [build a design system](../cards/build-a-design-system.md).
- The article on the style guide, once it is complete: card
  [write an article on the style guide](../cards/write-an-article-on-the-style-guide.md).
- A breadcrumb and a way back to the top for the articles: card
  [add breadcrumbs and a way back to the top to the articles](../cards/add-breadcrumbs-and-back-to-top-to-articles.md).
