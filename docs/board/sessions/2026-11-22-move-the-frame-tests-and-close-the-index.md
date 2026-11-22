# Session 2026-11-22 — move the frame's tests and close the index

**Participants:** the user (author); the agent (Claude Opus 5.5), with
two sub-agents on Claude Sonnet 5.5 (the first moved the frame's tests,
the second moved the column's tests, added the headings' ratios and
took the screenshots; both ran the pipeline's checks).
**Goal:** pick up the plan and take the three cards left before
publishing the style guide:
[move the frame's tests to its page](../cards/move-the-frame-tests-to-its-page.md),
[move the text column's tests to the frame's page](../cards/move-the-column-tests-to-the-frame-page.md)
and
[declare the pages to come in the style guide's index](../cards/declare-the-pages-to-come-in-the-style-guide-index.md).
Technical details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Start from moving the frame's tests, the next and smallest card | user | agent | — |
| 2 | The home page's list of articles is examined by axe at the laptop's width only, as every page of content | user | agent | — |
| 3 | Six acceptance tests for moving the frame's tests | user | agent | — |
| 4 | Undo the sub-agent's change to the README's class "Frame", which still covers the home page | agent | — | user |
| 5 | Commit the moved frame tests and close their card | user | agent | — |
| 6 | Go on with moving the column's tests | user | agent | — |
| 7 | The sub-section headings' ratio, 1.1 times the text, belongs to the frame too, and is checked in this card; with a doubt declared | user | agent | — |
| 8 | Eight acceptance tests for the column's card, among them a real sub-section on the frame's page instead of a sample | user | agent | — |
| 9 | One constant for the frame page's address, shared by every test file | agent | — | user |
| 10 | Commit the moved column tests with the new sub-section, and close their card | user | agent | — |
| 11 | Go on with the index's card | user | agent | — |
| 12 | The index describes only what exists; no page is declared to come, changing the decision of 2026-11-18 | user | agent | — |
| 13 | Of declaring the components not yet cut (2026-11-19), only the present stays, written by the author in the frame page's text | user | agent | — |
| 14 | Close the index's card with no change to the code; one card for the three placeholder texts, written in one go before publishing | user | agent | — |
| 15 | Close the session | user | — | — |

## Discussion

1. **Where to start.** The agent read the plan and the last minute and
   proposed the frame's tests, next in the plan's order and the
   smallest; the user agreed. → decision 1
2. **What the frame's tests guarded on the home page.** The agent read
   the six tests and found four looking only at the frame, the one rule
   that looked at the whole page already covered by the net; what the
   move lost was axe on the home page's list of articles at the phone's
   and tablet's widths. It asked whether the list deserved a check of
   its own; the user chose the net only. → decisions 2 and 3
3. **The sub-agent's extra change.** Reviewing the diff, the agent
   found that the sub-agent had also rewritten the README's class
   "Frame", dropping the home page, whose own tests still belong
   there; it put the line back and told the user, who approved the
   commit. → decisions 4 and 5
4. **The sub-section headings.** Reading the column's tests, the agent
   answered the card's question (the column's width does not depend on
   its text) and raised a new one: the same rule of the stylesheet sets
   the sub-section headings at 1.1 times, untested, and the frame's
   page had none. The user, not sure, placed them in the frame. The
   agent pointed out that the rule agreed earlier measured the page's
   own headings, with no sample, and proposed a real sub-section in the
   draft text; the user accepted it with the acceptance tests. →
   decisions 6, 7 and 8
5. **The column's build.** A sub-agent wrote the tests; the agent's
   brief had it share one constant for the page's address. The user
   saw the new sub-section in screenshots on the iPhone SE and the
   laptop; the agent pointed out the large space under a heading as
   the column's spacing of paragraphs, outside the card. The user
   approved. → decisions 9 and 10
6. **The pages to come.** The agent set out how the list of pages
   wished for on 2026-11-18 had moved, and the tension between
   announcing the list of articles and the user's choice earlier in
   the session. The user chose to describe only what exists, not
   knowing when or which pages will come. The agent noted the change
   of the earlier decision and what it loses, and asked what was left
   of the decision of 2026-11-19; the user kept only the present, to
   write it in the frame page's text. → decisions 11, 12 and 13
7. **The texts before publishing.** The agent proposed closing the
   card with no code and a card for the frame page's draft; the user
   asked for one card for all three texts, so as to publish after it.
   Finding only two placeholders, the agent wrote its reading of the
   three texts into the card as unconfirmed; the user confirmed it. →
   decision 14
8. **The agent's own slips.** The agent reported a commit identifier it
   had not read from the repository, and corrected it in the next
   message; and a closing commit missed the plan's line, which the
   agent added to the same commit, not yet pushed, before reporting.
   The user then closed the session. → decision 15

## Outcome

- The frame's tests moved to its page: card taken into work with its
  acceptance tests (`cb076fc`), the tests (`cb80c02`), card closed
  (`3e2d1fb`).
- The column's tests moved to the frame's page, with the headings'
  ratios and a sub-section of the frame page's text: card taken into
  work (`7b60f28`), the tests and the text (`bd3abf3`), card closed
  (`ccae0d5`).
- The index's card closed with no change to the code, and the card on
  the placeholder texts opened (`76d0797`), its three texts confirmed
  (`e94fb17`).
- New card:
  [write the style guide's placeholder texts](../cards/write-the-style-guide-placeholder-texts.md).
- Not pushed: the local branch is ahead of the remote.
- Screenshots, not committed, in `target/frame-headings/`.

## Open points

- Whether the placeholder marks go entirely once the author's texts are
  in: a question of the texts' card.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The author writes the three placeholder texts:
  [card](../cards/write-the-style-guide-placeholder-texts.md).
- Then the style guide is published:
  [card](../cards/build-a-design-system.md).
