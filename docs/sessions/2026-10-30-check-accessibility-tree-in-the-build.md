# Session 2026-10-30 — Check the accessibility tree in the build

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Sonnet 5.5 (the accessibility tree test).
**Goal:** take the card "Check accessibility by hand" and close its
first part:
[check the accessibility tree in the build](../done/2026-10-30_check-accessibility-tree-in-the-build.md).
Technical details and reasons are in the card. The session came right
after the session
[check accessibility in the build](2026-10-30-check-accessibility-in-the-build.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | First an automated check of the accessibility tree, with Playwright's own tool | user | agent | — |
| 2 | The check by hand must start with one local command, repeatable to look for regressions | user | — | agent |
| 3 | Split the card into those two | agent | — | user |
| 4 | Strict match (`/children: equal`), not `deep-equal` | agent | — | user |
| 5 | New card: run the browser tests in more browsers, placed in Later | user | — | agent |

## Discussion

1. **Can a screen reader be checked automatically?** The agent had
   proposed starting with a check by hand, with Orca and Firefox,
   already on the user's machine. The user asked about automated tools.
   The agent found three kinds (accessibility tree, simulated screen
   reader, real screen readers driven by a program, the last only on
   macOS and Windows), and noted that none hears how the page sounds.
   The user chose the tree first, then the check by hand, adding that it
   must start with one command. → decisions 1, 2, 3
2. **A tension left open.** The agent pointed out that a check by hand
   is a person listening, while a regression check compares with an
   earlier result; how the command reconciles the two is the first
   question of the card that carries it.
3. **How strict.** The sub-agent tried both strict modes: `deep-equal`
   would also demand the ASCII art inside the logo image, which a screen
   reader does not read. The agent kept `/children: equal`, and checked
   it with a deliberate change of the title's level, which axe does not
   catch and the new test does. → decision 4
4. **More browsers.** While the test was being reviewed, the user asked
   for a card on running the tests also outside Chromium. The agent
   placed it at the top of Later, telling the user it can move. →
   decision 5

## Outcome

- The browser test compares the accessibility tree at 393, 768 and
  1280 pixels; 19 tests out of 19 pass in the Playwright 1.64.1 image
  and on GitHub, run
  [41014138735](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41014138735).
- Commits: `6375d65` card split and started, `d1d0f47` browsers card,
  `082d32d` accessibility tree test, `db09bcf` card closed, plus this
  minute.

## Open points

- How the on-demand screen reader check compares one run with the
  previous: recorded answers, captured speech, or both.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Make the screen reader check start with one local command (decision
  2): card
  [run a screen reader check on demand](../backlog/run-screen-reader-check-on-demand.md).
- Run the browser tests in more browsers (decision 5): card
  [run the browser tests in more browsers](../backlog/run-browser-tests-in-more-browsers.md).
