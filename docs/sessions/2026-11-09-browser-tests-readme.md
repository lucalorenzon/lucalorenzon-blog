# Session 2026-11-09 — The browser tests' README, and the test pages in the design system

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Claude Sonnet 5.5 (two tasks: the refactor of the axe
tests, and the README check).
**Goal:** find what was left of the browser tests' reorganisation
([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)),
place the components' test pages, and do the card
[describe the browser tests in a README](../done/2026-11-09_describe-the-browser-tests-in-a-readme.md).
Technical details are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | No card of their own for the components' test pages: merged into [build a design system](../backlog/build-a-design-system.md), with a revision note in ADR-004 | user | agent | — |
| 2 | The design system's main reason: the two articles serving as test pages; it separates the components from the pages for good, and leaves the pages to the integration checks | user | agent | — |
| 3 | The design system goes to Now, starting with a spike on Holt, after a round of refactors: dates in a module of their own and the errors with thiserror, up from Next | user | agent | — |
| 4 | A README in `end2end/` listing every browser test | user | — | agent |
| 5 | Each row leads to its test through a unique piece of the declaring line, not a line number | user | agent | — |
| 6 | The README is kept in step by a project rule and by a check in the build | user | agent | — |
| 7 | Grouped by what the tests protect from, then by ADR-004's class, one table per class under its own heading | user | agent | — |
| 8 | The class "the net" is titled "Integration (net)"; the code keeps "net" | user | agent | — |
| 9 | FitNesse-shaped tables; the first column named "Find in files" | user | agent | — |
| 10 | The axe tests of the net and of the frame declared apart, behaviour preserved | user | agent | — |
| 11 | The check is a test in the suite, `readme.spec.ts`, skipped on the live site | agent | — | user |
| 12 | A fourth class, "Guard", for the tests that check the suite itself | agent | — | user |
| 13 | Running a test from its README row: a card in Later | user | agent | — |
| 14 | Push at the close of the session | user | — | agent |

## Discussion

1. **What was left of the tests.** The user asked whether most of it
   was done. The agent confirmed: of ADR-004 only the components' test
   pages remained, with no card.
2. **The test pages and the design system.** The user asked whether
   the test pages were not the design system's card. The agent
   recalled that ADR-004 had kept them apart, to stop the suite growing
   without waiting, and that the net had since removed that reason; it
   proposed merging them. The user observed that the After Save Event
   article had in fact become the test page; the agent found the hybris
   article was one too, and changed its preference to a separate card
   that would free them sooner. The user chose merging, holding that
   the design system separates components from pages for good and
   leaves the pages to integration tests. The agent pointed out that
   the reply "yes" answered its last proposal, the opposite, and had
   the reading confirmed. → decisions 1, 2
3. **Priorities.** The user put the design system in Now, but wanted
   the code's waiting refactors first, the dates and thiserror, so the
   code also moves. → decision 3
4. **How the README stays true.** The user asked for the README, with
   an interactive table as a nice-to-have. The agent raised that line
   numbers drift with every change; the user chose a string found with
   "find in files" and a project rule followed by the agent. The agent
   offered a check in the build beside the rule, as its preference; the
   user chose it. → decisions 4, 5, 6, 13
5. **The shape of the tables.** On the agent's questions, one at a
   time, the user chose the grouping, the classes as headings and the
   name "Integration (net)", to avoid thinking of networking; the agent
   named the tension with "net" in the code and the ADR, and the user
   chose to bracket the old name. The user asked where the string to
   search was; the agent pointed to the first column and the user named
   it "Find in files". → decisions 7, 8, 9
6. **What reading the tests showed.** The agent found that the first
   definition of the string (the fixed part of the title) was not
   unique, and that one helper declared the axe tests for two classes;
   it corrected the definition and proposed declaring them apart, which
   the user approved. → decision 10
7. **The agent's own choices.** Where the check lives and the class
   "Guard" were decided by the agent as low in impact and easy to
   change, and reported at the end; "Guard" waits for the user's
   confirmation. The agent also found and corrected a wrong count of
   its own in the design system card. → decisions 11, 12
8. **Closing.** The user asked for the minute, then for a push, being
   away from this machine. → decision 14

## Outcome

- Design system card, ADR-004 revision note and plan (`d74d392`);
  correction of a count in the card (`c0c2a7b`).
- Card [describe the browser tests in a README](../done/2026-11-09_describe-the-browser-tests-in-a-readme.md):
  started (`695f588`); axe tests declared apart, with Playwright's list
  of tests identical before and after (`1bb3432`); `end2end/README.md`
  with 52 tests (`5685342`); the check `readme.spec.ts`, its row, and
  the rule in the project's `CLAUDE.md` (`625d4ea`); closed (`76719ea`).
- Card [run a browser test from its row in the README](../backlog/run-a-test-from-its-readme-row.md),
  in Later (`bb4c349`).
- The full browser suite passes: 227 passed, the live-only test
  skipped. Pushed at the close, with this minute.

## Open points

- The class "Guard", added by the agent, is not confirmed by the user.
- `home.spec.ts` still names the two articles, their dates and their
  order, the content facts ADR-004 meant to remove; no card carries it.

## Next steps

- Give dates a module of their own: card
  [give dates a module of their own](../backlog/give-dates-a-module-of-their-own.md).
- Refactor the errors with thiserror: card
  [refactor the code's errors with thiserror](../backlog/refactor-errors-with-thiserror.md).
- Run the spike on Holt, then build the catalogue that frees the two
  articles: card [build a design system](../backlog/build-a-design-system.md).
