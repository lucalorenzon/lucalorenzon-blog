# Session 2026-11-18: the wrap toggle's fix and the cases at the limit

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: analysis, screenshots, cards, review.
- **Three sub-agents**: Claude Haiku 5.5 built the site; Claude Sonnet
  5.5 wrote the two cases at the limit, the test and the fix; Claude
  Haiku 5.5 ran the proof that the guard works.

Goal: restart from the plan, fix the wrap toggle shown on code blocks
that fit
([card](../done/2026-11-18_hide-the-wrap-toggle-on-blocks-that-fit.md)),
then finish the card it blocked
([card](../done/2026-11-18_measure-the-fonts-again-when-they-change.md)).
Technical details and reasons are in the cards.

## Decisions

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Look at the code with and without `text-rendering: geometricPrecision`, in screenshots at two widths, before choosing the fix | The user | The agent | — |
| 2 | The change in the look is practically invisible, so the rule on the code block is the fix, and the build's threshold stays as it is | The user | The agent | — |
| 3 | The two cases at the limit move to the fix's card as its regression test; the card that waited keeps only the proof that the guard works | The user | The agent | — |
| 4 | The seven acceptance tests of the fix's card, and the way to proceed | The user | The agent | — |
| 5 | The text of the two cases, drafted by the agent | The user | The agent | — |
| 6 | The fix is committed and its card closed | The user | The agent | — |
| 7 | Hack's advance moved to 1234 and to 1215 over 2048 for the proof | The agent | — | The user |
| 8 | The card on measuring the fonts again is closed | The user | The agent | — |
| 9 | A card on the table widths and Chromium's rounding, in Next | The user | The agent | — |
| 10 | The new card goes at the top of Next | The agent | — | The user |
| 11 | The session ends here; the breakpoints come next session | The user | — | — |

## Discussion

1. **Where to restart.** The agent read the plan, the two cards in
   work and the last minute, and proposed the step the minute named:
   screenshots of the code with and without the rule. The user agreed.
   The agent showed that the rule makes Chromium draw the width the
   build assumes, and that magnified at one device pixel per CSS pixel
   some strokes are a little softer; the user found the change
   practically invisible. → decisions 1, 2
2. **Which case is the regression test.** The agent proposed that the
   case that fits the column be the regression test, moving from the
   waiting card to the fix's card. The user agreed with moving it, and
   rejected the alternative of leaving the other card intact.
   Computing the numbers for the acceptance tests, the agent found its
   own proposal wrong: the case that fits passes before the fix too,
   and only the case one character longer fails. It said so and
   brought the correction, both cases and five of the six acceptance
   tests moving to the fix's card; the user agreed. → decisions 3, 4
3. **The fix.** The Sonnet sub-agent showed the test failing before
   the fix, on the line one character too long at 768 pixels, then
   every check of the pipeline passing after it. The agent noted that
   the failed run stopped at the scrolling check, before the toggle's,
   and took screenshots of the two cases before asking for the commit.
   The user approved the text and the commit. → decisions 5, 6
4. **The proof that the guard works.** The agent chose the two values
   that move the threshold by one character in each direction, and
   pointed out that the guard is not symmetrical: one unit wider stops
   the build, while a narrower font must be about 1.5% narrower. The
   Haiku sub-agent ran both; each failed as expected, and the test
   passed again with the value put back. The user approved closing
   the card. → decisions 7, 8
5. **What next.** The agent recalled the open point of the last
   session on the table widths, and offered the breakpoints, the card
   or the minute; the user asked for the card in Next and for the
   minute. The agent did not mend the links of the last minute, which
   is frozen. → decisions 9, 10, 11

## Outcome

- The fix's card given its answers and acceptance tests, the limit
  cases moved to it, the plan updated (`ec5c2a9`).
- The fix and its regression test: two cases at the limit on the
  style guide's code page, the test extended, the rule on the code
  block, the test's row in the README (`65d9b1c`).
- The fix's card closed (`5420295`).
- The card on measuring the fonts again closed, with the proof that
  the guard works; links in the other cards and the plan updated
  (`36732da`).
- New card in Next:
  [check the table widths against Chromium's rounding](../backlog/check-table-widths-against-chromium-rounding.md)
  (`c3d086d`).
- Nothing pushed.

## Open points

- The minute of 2026-11-17 links to the two cards through `docs/wip/`,
  which no longer holds them; it is not edited, as minutes are frozen.
  The card
  [refer to cards by name in frozen documents](../backlog/refer-to-cards-by-name-in-frozen-documents.md)
  covers the cause.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The breakpoints, last page of the style guide before it is
  published: card [build a design system](../wip/build-a-design-system.md).
- Whether Chromium's rounding can make a table overflow its column:
  card
  [check the table widths against Chromium's rounding](../backlog/check-table-widths-against-chromium-rounding.md).
