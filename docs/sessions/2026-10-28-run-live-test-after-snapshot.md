# Session 2026-10-28 — Run the live-site test after a snapshot publication

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Haiku 5.5 (card move; push, tag and watching the runs) and
Sonnet 5.5 (workflow fix).
**Goal:** close the card
[run the live-site test when the snapshot is published](../done/2026-10-28_run-live-test-when-snapshot-is-published.md),
handled as a malfunction with the `fix` skill. Technical details and
reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | `verify-live` gets the same explicit condition as `release` | user | agent | — |
| 2 | The 0.2.0 run is the failing case, the 0.2.1 deploy the proof | user | agent | — |
| 3 | The closed card is the fix record, not a file in `docs/fixes/` | user | agent | — |
| 4 | Push and tag of 0.2.1 done by the agent, with stop conditions | user | agent | — |
| 5 | No local install of actionlint; a backlog card for running it in the pipeline | user | agent | — |
| 6 | The actionlint card goes in Later | agent | — | user |
| 7 | The link in a committed minute is updated when its card moves | user | agent | — |

## Discussion

1. **Plan and scope.** The agent confirmed the card's cause on the 0.2.0
   run, checked every other job in both workflows and found no other
   gap, and proposed a plan in six steps, stopping before push and tag.
   The user approved it. → decisions 1, 3
2. **Proving a fix that cannot run locally.** The agent pointed out that
   the bug lives in how GitHub skips jobs, so it cannot be reproduced on
   a machine; it proposed the 0.2.0 run as the failing case and the next
   patch version as the proof, after checking that a `CHANGELOG.md`
   commit produces a snapshot. → decision 2
3. **actionlint.** The user noted that the agent had named actionlint
   more than once, and offered to install it. The agent answered that it
   would not have found this bug, and proposed to run it in the pipeline
   rather than on one machine, through a card written at the end of the
   session. The user agreed. → decisions 5, 6
4. **Publication.** The user asked the agent to push and tag. The agent
   gave the Haiku sub-agent three stop conditions: the build of the
   commit passed, its snapshot exists, and the date still matches the
   one written in `CHANGELOG.md`. None fired. → decision 4
5. **Links to a moved card.** Closing the card, the agent found that
   starting it had left links pointing at `docs/backlog/`, because its
   own instructions to the Haiku sub-agent named only the plan. One of
   those links is in a committed minute, and two rules of `CLAUDE.md`
   disagree on it: links follow a moved card, a minute is not edited.
   The agent recommended updating the link only; the user agreed.
   → decision 7

## Outcome

- Workflow fixed (`ed2df23`). Version 0.2.1 published from the snapshot
  ([deploy run 40782792212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40782792212)):
  `build` skipped, `verify-live` ran and passed.
- Commits: `0f6b39e` card to `wip`, `ed2df23` fix, `517d739` changelog
  for 0.2.1, `cf5b7ff` card closed and links updated, `4ab08d4` actionlint
  card, plus this minute.

## Open points

- The two rules of decision 7 are still both written in `CLAUDE.md`
  without saying which wins; this session settled one case.
- Pushing to `main`, GitHub reported "Bypassed rule violations … 2 of 2
  required status checks are expected": direct pushes skip the ruleset's
  required checks. Not discussed in this session.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Check the workflows with actionlint (decision 5): card
  [check the workflows with actionlint](../backlog/lint-workflows-with-actionlint.md).
