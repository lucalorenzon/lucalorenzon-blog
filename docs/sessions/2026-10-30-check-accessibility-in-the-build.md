# Session 2026-10-29/30 — Check accessibility in the build

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Sonnet 5.5 (the accessibility test).
**Goal:** take the card "Add automated accessibility tests" and close
its automated part:
[check accessibility in the build](../done/2026-10-30_check-accessibility-in-the-build.md).
Technical details and reasons are in the card. The session started on
2026-10-29, right after the session
[stop the generic CSS checker on the stylesheet](2026-10-29-stop-generic-css-checker.md),
and ended on 2026-10-30.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Split the card: a check in the build, and a check by hand | user | agent | — |
| 2 | The AAA level goes into the build check, not into a card of its own | user | agent | — |
| 3 | axe-core inside the Playwright test, at one width per layout band, any violation failing the build | user | agent | — |
| 4 | Prove the check fails with a lowered amber (5.63:1) in a copy of the site | agent | — | user |
| 5 | Push and close once the tests passed | agent | — | user |

## Discussion

1. **Splitting the card.** The agent found several decisions in the
   card and proposed three cards: the build check at levels A and AA,
   the AAA level, the checks by hand. The user asked whether the AAA
   level could go with the first. The agent checked which AAA rules axe
   knows (three), agreed it fits one check, and spelled out the
   commitment it brings: every future colour at 7:1, without the site
   being "AAA" as a whole. The user accepted. → decisions 1, 2, 3
2. **A check that could pass silently.** The AAA rules are off by
   default in axe. The agent had the test require the contrast rules to
   have run, and added to the Docker command a second round, inside the
   container only, with the amber lowered below 7:1; it failed on
   enhanced contrast only, as expected. → decision 4
3. **Publishing.** Following the earlier cards of the same sitting, the
   agent pushed after closing the card and told the user. → decision 5

## Outcome

- The browser test runs axe at 393, 768 and 1280 pixels; 16 tests out
  of 16 pass in the Playwright 1.64.1 image and on GitHub, run
  [41012354243](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/41012354243).
- Commits: `5a92312` card split, `47a5ce6` card to `wip`, `9b259f9`
  accessibility test, `8e7a505` card closed, plus this minute.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Decide which checks are done by hand, such as a screen reader
  (decision 1): card
  [check accessibility by hand](../backlog/check-accessibility-by-hand.md).
