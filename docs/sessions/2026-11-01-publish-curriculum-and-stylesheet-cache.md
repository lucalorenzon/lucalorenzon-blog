# Session 2026-11-01 — Publish the curriculum, stylesheet cache

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agent Claude Haiku 5.5 (push, tag and publication).
**Goal:** publish version 0.4.0, prepared in the session
[curriculum on the home page](2026-10-31-curriculum-on-home-page.md),
whose minute was committed before the publication. Then, raised by the
user, open the card
[refresh the stylesheet after a new version](../backlog/refresh-stylesheet-after-a-new-version.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The agent pushes `main` and tags `v0.4.0` once the build passes | user | agent | — |
| 2 | Continue past the "Bypassed rule violations" message of the push, as documented | agent | — | user |
| 3 | The stale stylesheet becomes a card in Later; the fix comes later | user | agent | — |

## Discussion

1. **Publication.** The user asked the agent to publish. The sub-agent
   stopped after the push on GitHub's message about bypassed required
   checks; the agent found it documented as the expected effect of a
   direct push by the author's account, and resumed it. → decisions 1, 2
2. **Old stylesheet.** The user noticed that a browser which had visited
   the site kept the old stylesheet until its cache was cleared, and
   proposed a version tied to the stylesheet's name. The agent checked
   the headers GitHub Pages sends (ten minutes of cache, not
   configurable) and wrote them in the card. → decision 3

## Outcome

- `e5d1c55` pushed; build passed; tag `v0.4.0` pushed; publication and
  the test on the live site passed; Release `v0.4.0` created. The site
  shows the curriculum.
- `f41fd69`: card on refreshing the stylesheet, in Later.
- Not pushed: this minute and `f41fd69`.

## Next steps

- Choose the site's public address: card
  [choose the site's public address](../backlog/choose-site-address.md).
- Run the screen reader check on the English site: card
  [update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md).
