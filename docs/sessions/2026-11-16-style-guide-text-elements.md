# Session 2026-11-16 — The text elements in the style guide

**Participants:** the user (author); the agent (Claude Opus 5.5), with
two sub-agents Claude Sonnet 5.5 (one built the style guide's text page
and moved the text checks onto it; the other made the accessibility
check scroll the page, and measured that the contrast is judged on the
same elements).
**Goal:** take up the typography page of the card
[build a design system](../wip/build-a-design-system.md): agree its
scope and build its first slice, in the card
[show the text elements in the style guide](../wip/show-the-text-elements-in-the-style-guide.md).
The session began on 2026-11-15 and ended on 2026-11-16. Technical
details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The typography is cut in two pages and two cards: the elements of the text first, the fonts after | user | agent | — |
| 2 | The page's cases are the kinds of element the text checks looked at on the article, plus the level-4 heading | user | agent | — |
| 3 | The cases are written on purpose to put the checks to the test, with headings and list items that wrap | user | agent | — |
| 4 | A guard checks that every case of the guide still wraps, instead of measures written in the card only | user | agent | — |
| 5 | No code block and no table on the text page; the text spacing check also runs on the code blocks and tables pages | user | agent | — |
| 6 | Bold, italic, inline code and links are on the page to be shown only, without checks of their own | user | agent | — |
| 7 | The level-4 heading gets a check of its own, on its underline as it is today | user | agent | — |
| 8 | The text checks run on the guide's page only, as for the code block and the table | user | agent | — |
| 9 | The agent writes the cases, for the tests; the user writes the explanations | user | agent | — |
| 10 | The card's eleven acceptance tests, the page at `style-guide/text.html` titled "Text", and the correction of the design system card's sentence on level-4 headings | user | agent | — |
| 11 | The guard tells an explanation from a case by its place: the first paragraph after each heading, and the introduction, explain; everything else is a case | sub-agent (Sonnet) | agent | user |
| 12 | The text checks leave out the breadcrumb, and the check of the end of the content counts the record viewer's box only, not what it hides | sub-agent (Sonnet) | agent | user (by this minute) |
| 13 | The accessibility check failing on the style guide's index is a fault of the test: if the page scrolls, the test scrolls it | user | agent | — |
| 14 | The test scrolls to the end once before axe runs, in a fix card of its own committed before the text page | user | agent | — |
| 15 | The likeness of the level-4 heading to the links is not noted in the backlog | user | agent | — |
| 16 | The session closes after the first slice; the explanations are written in another session | user | agent | — |

## Discussion

1. **One page or two.** The user asked to take up the typography card;
   the agent found no card of its own, only an item of the design
   system card, and read it as two pages of different nature, the
   fonts mostly to be shown, the text elements freeing the article
   from its role of test page. The user agreed and chose the text
   elements first. → decision 1
2. **Which cases.** The agent raised that on the article some of what
   puts the checks to the test happens by chance, while the old guard
   only counted the elements. The user answered with the article's
   kinds of element, then chose cases that wrap on purpose. The agent
   raised that a case shortened for the text would leave its check
   with nothing to check; the user chose a guard on the guide's cases.
   The agent pointed out it was not the kind of guard earlier cards had
   removed, since it guards cases written for the tests. → decisions 2,
   3, 4
3. **Code and tables beside the text.** The agent found that the text
   spacing check met the code block and the table only on the article;
   the user moved the check to their pages too. → decision 5
4. **Inside a line, and the level-4 heading.** The agent raised that no
   text check looks at bold, italic, inline code or links; the user
   chose to show them only. The agent found, in the stylesheet, that
   the level-4 heading stands out by its underline alone, like the
   links; the user wanted a check of its own, on the underline as it
   is. The agent also found that the design system card wrongly said
   level-4 headings entered an overlap check of the net. → decisions 6,
   7, 10
5. **Where, and who writes.** The user kept the text checks on the
   guide only. The agent raised that the guard makes the cases' text
   part of the test; the user left the cases to the agent and kept the
   explanations. → decisions 8, 9, 10
6. **The first slice.** The first sub-agent built the page and moved
   the checks, chose how the guard tells explanations from cases, and
   scoped the checks around the breadcrumb and the record viewer; the
   agent reviewed the changes and showed the user screenshots on a
   small phone and a laptop. The agent told the user that each
   explanation must stay one paragraph. → decisions 11, 12
7. **A net test failing.** The sub-agent reported that the
   accessibility check failed on the style guide's index, whose new
   link fell under the sticky footer at 1280 by 720, and left the
   choice open. The agent confirmed the failure and asked whether it
   was a fault of the page or of the test; the user asked whether the
   test scrolls. The agent found it does not, while the text checks
   do; the user called it a fault of the test. The agent proposed a
   fix card of its own, as the project asks for every fix; the second
   sub-agent made the change and showed the contrast judged on the same
   elements before and after. → decisions 13, 14
8. **The likeness to the links.** Asked whether to note it, the user
   recalled that the level-4 heading is the curriculum's "Main work";
   the agent confirmed it is two words above a list there, unlike the
   guide's long case, and the user dropped the note. → decision 15

## Outcome

- The card on the text elements opened, the fonts noted in the
  backlog, the plan and the design system card updated (`46cab20`).
- The card's answers and acceptance tests agreed, and the design
  system card's sentence on level-4 headings corrected (`595593e`).
- The accessibility check scrolls to the end of the page before axe
  runs; its card is closed
  ([card](../done/2026-11-16_scroll-before-the-accessibility-check.md))
  (`58774b8`).
- The style guide's text page, with the text checks on it instead of
  the After Save Event article, which is no longer a test page; the
  text spacing check also on the code blocks and tables pages
  (`844a954`). Checks of `build.yml` run locally: 343 unit tests, 247
  browser tests passed, one skipped (the "page not found" test that
  runs on the live site only).

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The user's explanations in place of the placeholders, one paragraph
  each, and the card's closing, with the dropped note on the likeness
  to the links: card
  [show the text elements in the style guide](../wip/show-the-text-elements-in-the-style-guide.md).
- The fonts' page: card
  [show the fonts in the style guide](../backlog/show-the-fonts-in-the-style-guide.md).
