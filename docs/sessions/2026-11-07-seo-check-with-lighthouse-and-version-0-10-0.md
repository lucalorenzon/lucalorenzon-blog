# Session 2026-11-07 — The SEO check with Lighthouse, and version 0.10.0

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Sonnet 5.5 (the code of the SEO check, its dependency
audit and its checks).
**Goal:** resume the card
[check SEO with Lighthouse in the build](../done/2026-11-07_check-seo-with-lighthouse.md),
on hold since the previous session for the sitemap and the
`robots.txt`, answer its last questions, build it, and publish it with
the version that was waiting for it. Reasons and rejected options are in
the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | No page is exempted: the check runs only on the sitemap's pages, in both moments; the exemption derived from `noindex` is dropped | user | agent | — |
| 2 | The check runs once, in a Playwright project of its own; the two theme projects leave its file out | user | agent | — |
| 3 | The implementation plan: the dependency with its audit, the project, the test, no change to the workflows, the checks of the pipeline, then the card closed | user | agent | — |
| 4 | On the live site only the reading of the sitemap is retried, not the six Lighthouse runs | Sonnet | — | agent, user |
| 5 | The package `sentry`, Sentry's command line under the Functional Source License 1.1, is accepted as a development dependency that is never called | user | agent, Sonnet | — |
| 6 | Lighthouse is listed in the stack of `CLAUDE.md` | agent | — | user |
| 7 | Publish the waiting work as version 0.10.0 | user | agent | — |
| 8 | The minute is written after the publication, so it holds the first live run of the check | user | agent | — |

## Discussion

1. **The open question.** The agent set out the tension left by the
   previous session: with the pages taken from the sitemap, the "page
   not found" never reaches the check, so the exemption would never
   apply. It proposed two ways, the sitemap plus the built folder
   locally, or no exemption, and preferred the second; the user chose
   it. → decision 1
2. **One run instead of two.** The agent raised that the suite runs
   twice, once per theme, while the rules need one run, and proposed a
   project of its own or a test that skips itself; the user chose the
   project. → decision 2
3. **The plan.** The agent presented the steps before any code; the
   user approved them. Sonnet wrote the code and ran the checks, the
   agent reviewed it and committed it. Sonnet changed one point of the
   brief, the retry on the live site, and gave the reason; the agent
   kept it and reported it. → decisions 3, 4
4. **Sentry.** The audit found one licence that is not permissive. The
   user asked whether Sentry was not meant to stay unused; the agent
   confirmed it is never loaded, explained why its packages are
   installed anyway, and read the package: no script at installation,
   never called, never part of the site. The user accepted it.
   → decision 5
5. **Closing and publishing.** Before the minute the agent offered to
   follow the pipeline and to prepare the publication; the user asked
   for both. The agent proposed to write the minute after the
   publication; the user pushed the tag. → decisions 6 to 8

## Outcome

- The card's answers: `a873c5c` no exemption, `589a13a` one run in a
  project of its own.
- The code: `78a2765`, written by Sonnet: `lighthouse` 13.6.0 in
  `end2end/`, the project `lighthouse` in `playwright.config.ts`, the
  test `end2end/tests/seo.spec.ts`.
- `71e6218` card closed, with the plan and the live cards that linked
  to it; `7cb9346` the changelog entry; `6360de7` Lighthouse in the
  stack; `3528b7d` version 0.10.0 in the changelog; `a196360` the plan
  after the publication.
- Checks: 199 Rust tests and 849 browser tests passed locally, 2
  skipped; the SEO check passed on the six pages of the sitemap in
  about 30 seconds, and failed as expected with a description removed
  by hand. The pipeline passed on `6360de7` and on `3528b7d`.
- Version 0.10.0 published from tag `v0.10.0` on `3528b7d`: the
  deploy rebuilt and tested the site, since the tag came before the
  build of that commit finished, published it, and created the Release
  with `site-v0.10.0.zip`. The test on the published site passed: 611
  passed, 26 skipped, the SEO check included, its first live run, in
  32 seconds. The live `sitemap.xml` and `robots.txt` answer.

## Open points

- GitHub warns, on every job, that the label `ubuntu-latest` moves to
  Ubuntu 26 from 2026-10-19; the agent offered a card to pin it or
  watch it, and none was opened in this session.
- `actionlint` is still not installed.

## Next steps

- Choose the next card from Next: [plan](../plan.md).
- Watch the rules Lighthouse finds not applicable: card
  [watch the rules Lighthouse finds not applicable](../backlog/watch-lighthouse-rules-not-applicable.md).
- Add the performance category to the same Lighthouse run: card
  [check performance in the build](../backlog/check-performance-in-the-build.md).
- Structured data, the rule Lighthouse leaves to a person: card
  [check the structured data](../backlog/check-structured-data.md).
