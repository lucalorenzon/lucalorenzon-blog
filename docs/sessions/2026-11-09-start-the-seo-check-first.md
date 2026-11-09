# Session 2026-11-09 — Start the SEO check first

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Haiku 5.5 (building the site and measuring the
browser tests, two tasks) and Claude Sonnet 5.5 (the change to the test
configuration, one task).
**Goal:** do the card
[start the SEO check first](../done/2026-11-09_start-the-seo-check-first.md),
so the Lighthouse check stops running last and alone. Technical details
and measurements are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Take the card on starting the SEO check first | user | — | agent |
| 2 | List the `lighthouse` project first, with no other setting, and measure before and after on 2 and 12 workers | user | agent | — |
| 3 | Measure locally only; no push just to measure on the pipeline's machine | user | agent | — |
| 4 | The live check's gain stated by construction, not measured, since measuring it means publishing | user | agent | — |
| 5 | No push at the close of the session | user | agent | — |

## Discussion

1. **The card.** The user asked to take it up. → decision 1
2. **How to start it first.** The agent read Playwright's code: the
   groups of tests are built project by project, in the configuration's
   order, and the workers take them from that queue. It proposed moving
   the project and measuring, flagging that a fresh worker might not
   take the first group of the queue; the measurement settled it. The
   user approved the plan. → decision 2
3. **Where to measure.** The agent noted that whether Lighthouse's own
   Chromium slows the tests beside it shows only on the pipeline's
   machine, which needs a push, and asked the user; the user chose the
   local measurement for now. → decision 3
4. **The live check.** The agent proposed answering from the workflow,
   which runs the same configuration on the same kind of machine; the
   user approved it with the plan. → decision 4
5. **Closing.** Asked whether to push to see the pipeline's
   measurement, the user chose to write the minute and close. →
   decision 5

## Outcome

- On two workers, as in the pipeline, the browser tests went from
  about 49 to about 36 seconds; on this machine's twelve workers, from
  about 34 to about 31.
- Card [start the SEO check first](../done/2026-11-09_start-the-seo-check-first.md):
  started (`3ff3f9c`), change to `end2end/playwright.config.ts`
  (`69d831e`), closed with the answers and the measurements
  (`6d6ceba`).
- The generated site is unchanged; nothing pushed in this session:
  `631569e`, the previous session's minute, and this session's commits
  are local.

## Open points

- The gain on the pipeline's machine, and whether Lighthouse's
  Chromium slows the tests beside it there, are not measured yet.

## Next steps

- Read the browser test step's time and the SEO check's duration in
  the first pipeline run after the next push: card
  [start the SEO check first](../done/2026-11-09_start-the-seo-check-first.md),
  "Does Lighthouse's own Chromium slow the tests beside it?".
