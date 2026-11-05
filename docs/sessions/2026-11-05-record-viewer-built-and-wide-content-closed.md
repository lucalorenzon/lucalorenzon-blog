# Session 2026-11-05 — The record viewer built, the wide content and look cards closed

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (the rebuilt prototype and its fixes, the record
viewer in the site with its tests, the hybris article back in the
checks).
**Goal:** slices 2 and 3 of the card
[let the Markdown ask for components for wide code and tables](../done/2026-11-05_annotate-markdown-for-wide-content.md):
the record viewer and its threshold in the build, then the hybris PK
article back in the checks, which closes the card. Then, on
2026-11-06, the user's validation of the article pages, which closes
the card
[give the articles' code, quotes and tables a look](../done/2026-11-06_design-look-of-articles.md).
Reasons and rejected options are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Slice 2's plan: both forms of a table in the page, one shown; Literata's widths measured once and written in the code | user | agent | — |
| 2 | The readability rule lets the record viewer's box hide its other records vertically | user | agent | — |
| 3 | The spike's prototype rebuilt in one file, for the user to read its code | user | agent | Sonnet |
| 4 | The copies of each value in the viewer accepted, as the price of no script | user | agent | — |
| 5 | The label column as wide as the longest label, measured by the build; beyond a cap, labels wrap with hyphenation | user | agent | — |
| 6 | The cap is 45% of the viewer | user | agent, Sonnet | — |
| 7 | Literata's widths measured in Chromium by a script in `end2end/tools/`, no font library added | agent | Sonnet | user |
| 8 | M measured with the average character of English text, and lowered from 18 to 17 | user | agent, Sonnet | — |
| 9 | A header cell asks only for its longest word | user | agent | — |
| 10 | A margin of 0.1 em on the label column | agent | Sonnet | user |
| 11 | A 1 ms animation makes Chromium attach the record number's scroll timeline | Sonnet | agent | user |
| 12 | The built article page tried on a phone before the commit | user | agent | — |
| 13 | A table overflowing under the reader's text spacing, 768 to about 880 pixels, is a known limit, as for code | user | agent, Sonnet | — |
| 14 | That exception covers only the table form, not the record viewer | agent | — | user, Sonnet |
| 15 | A card in Later for long tables: a header in view, and the viewer's copies | user | agent | — |
| 16 | A question on text set in pixels (WCAG 1.4.4) noted in the WCAG card | agent | — | user |
| 17 | The two private pages on claude.ai deleted | user | — | agent |
| 18 | The article pages validated in the browser | user | — | agent |
| 19 | Firefox's warning on the record viewer's box kept as known, with the reason next to the code | user | agent | — |
| 20 | The look card closed | user | agent | — |
| 21 | This minute corrected in place of the one already pushed, rewriting the published history | user | agent | — |

## Discussion

1. **The readability exception.** The agent's plan said the test rule
   would change; the user did not see why. The agent explained it with
   the code's precedent; the user's picture was all records in one
   place, one shown at a time, and they asked for the code. The agent
   explained the two mechanisms, the scrolling roll and the invisible
   stacks. → decision 2
2. **The copies.** The user asked whether each value is written at
   least three times; the agent counted seven for the hybris table and
   said they grow with the square of the records, the price of no
   script. The prototype, lost with the earlier session, was rebuilt;
   the user read it and went on without a script. → decisions 3, 4
3. **The label column.** On the rebuilt prototype the user saw
   "Description" broken inside the word. The agent proposed a width
   measured by the build with a 40% cap; Sonnet's measurements showed
   the cap too low on an iPhone SE, an error of the agent's, and the
   user chose 45%. → decisions 5, 6, 10
4. **The plan approved.** With the prototype working on the phone, the
   user approved the plan. With no font library on the machine, the
   agent had the widths measured in Chromium. → decisions 1, 7
5. **M, measured.** Sonnet stopped as briefed: the hybris table became
   a record viewer even on a computer. The agent found its own brief at
   fault (the alphabet's average, not running text's), showed that even
   with English text M = 18 missed 768 pixels by one pixel, and
   proposed 17; the user agreed. → decision 8
6. **Headers.** The same measurements turned the "After Save Event"
   table into a record viewer on an iPhone SE, because of a two-word
   header; the agent proposed counting headers by their longest word,
   and the user chose it. → decision 9
7. **The phone test.** The user tried the built page, published as a
   private page, and found everything working, including the Chromium
   workaround Sonnet had needed. → decisions 11, 12
8. **Text spacing.** Back in the checks, the hybris page failed four
   text spacing tests: the table in table form grows wider than the
   column. The agent laid out three options and proposed the known
   limit; the user chose it. Reviewing Sonnet's change, the agent
   narrowed it to the table form. → decisions 13, 14
9. **Closing the card.** The user chose a card in Later for the header
   idea, which the previous session left undecided; the agent added the
   copies of long tables to it, and noted the pixel text sizes found by
   Sonnet's doubled-text check in the WCAG card. → decisions 15, 16
10. **The first minute.** Right after the minute's commit the user
    asked to delete the two private pages and to validate the article
    pages, and pointed out that the agent should have asked both
    before writing the minute. The agent deleted the pages and served
    the built site locally. → decisions 17, 18
11. **Validation.** The user validated both pages. The browser's
    accessibility check (Firefox's inspector, by its wording) warned
    that the viewer's box takes focus without being a control; the
    agent explained why it is a Tab stop and proposed keeping it, the
    user agreed. Asked before the new minute, the user had the look
    card closed. → decisions 19, 20
12. **Rewriting the minute.** The user had already pushed the first
    minute; the agent, having pulled it back locally, proposed a
    correction on top; the user chose to replace it, with a forced
    push. → decision 21

## Outcome

- On the site: `832a735` a wide table becomes a record viewer below a
  readable width; `974205c` the hybris PK article back in the
  readability checks. 126 unit tests and 720 browser tests pass (2
  skipped, as before).
- The card: `e5e0ea9` slice 2's decisions and results; `336cfc8` the
  card closed in `docs/done/`, and the card
  [handle long tables](../backlog/handle-long-tables.md) opened in
  Later; `68165ea` the 1.4.4 question in the
  [WCAG card](../backlog/cover-wcag-criteria-axe-cannot-check.md).
- [ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md) extends
  the container query exception to tables (`e5e0ea9`).
- `1ab20fd` the reason for the viewer's Tab stop, in the code;
  `5ba7f57` the
  [look card](../done/2026-11-06_design-look-of-articles.md) closed,
  the plan updated: the navigation's condition is met.
- Plus this minute, which replaces `33e18aa`, pushed earlier.

## Open points

- Not verified: a real mouse wheel in Chromium; the contrast finding
  axe reported once in the spike with the viewer on a switch point.

## Next steps

- The site's navigation, whose condition is now met, in Next:
  [add site navigation](../backlog/add-site-navigation.md).
- Whether text set in pixels fails WCAG 1.4.4, in the card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md).
