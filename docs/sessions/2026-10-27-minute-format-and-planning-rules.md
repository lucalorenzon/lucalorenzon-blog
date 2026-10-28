# Session 2026-10-27 — Session minute format and planning rules

**Participants:** the user (author); the agent (Claude Opus), with a
Haiku sub-agent for the two card moves (`427d046`, `df3d3c6`), each
checked by the agent on the resulting commit.
**Goal:** close the card
[make the session minute record who contributed what](../done/2026-10-27_revise-session-minute-format.md),
then the card it raised,
[define the rules for planning and project management](../done/2026-10-27_define-planning-rules.md).
Reasons and rejected options are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Minute: decisions table first, then a short discussion of who proposed what | user | agent | — |
| 2 | A minute's next steps are the actions that came out of the session; when they are done is up to the plan | user | agent | — |
| 3 | Earlier minutes stay as they are; only the Playwright image minute is rewritten | user | agent | — |
| 4 | `Assisted-by:` instead of `Co-Authored-By:`, in this repository's `CLAUDE.md`; `ll-dev-setup` later | user | agent | — |
| 5 | The `Claude-Session:` line stays in commits | user | agent | — |
| 6 | Planning rules and the plan update go to a card of their own, taken right after | user | agent | — |
| 7 | The plan is the project's board (Now, Next, Later), with no fixed order and no decisions | user | agent | — |
| 8 | Now holds the card in progress plus a few well-defined priority cards; one card in progress is usual, not a rule | user | agent | — |
| 9 | A split card is deleted in the commit that creates its parts, and the minute records the split; superseded cards wait for a cleanup card | user | agent | — |
| 10 | The plan's decisions move to a finished card for the skeleton, dated 2026-10-26 | user | agent | — |
| 11 | Card titles lose the "Backlog:" prefix; finished cards read as answers; their date is the day the work finished | user | agent | — |
| 12 | The build and deploy card goes to Now | user | agent | — |
| 13 | A decision that belongs to no card goes to a decision record in `docs/adr/` | user | agent | — |

## Discussion

1. **Shape of the minute.** The agent pointed out three gaps in the MADR
   table tried in the previous session: it did not say who proposed, what
   happened to the agent's proposals, or what the user started alone. The
   user recalled that a chronological account had solved this but was
   unreadable, and proposed a short meeting-style discussion. The agent
   rebuilt one for the Playwright image session from its transcript; the
   user approved it and put the table first. → decision 1
2. **Next steps.** The user first dropped them from the minute as a
   planning matter, agreeing that the plan, not the minute, should record
   changes of order, after the agent noted that the plan had fallen
   behind. Rereading the minute, the user reversed this: next steps are
   the actions that came out of a session, and the old "challenge with
   `residuality`" step had been wrong in kind. Asked by the agent how they
   differ from open points, the user answered that next steps are
   actions. → decision 2
3. **Existing minutes and commit lines.** The agent set out the cost and
   the mixed formats against the principle that minutes are not edited;
   the user kept the old ones and had only the trial minute rewritten.
   Asked about the scope of `Assisted-by:`, the user kept it in this card
   and this repository, leaving `ll-dev-setup` for later; the session link
   stays to trace commits. → decisions 3, 4, 5
4. **A planning card.** The user asked for a new card on planning rules,
   to be taken right after. The agent pointed out that the plan update and
   the rule it had proposed were already in the minute card; the user
   moved them to the new card, noting they had been anticipated so as not
   to lose them. → decision 6
5. **What the plan is for.** Asked by the agent, the user described the
   plan as a board built by successive approximations, with the backlog
   collecting ideas, the user prioritising as Product Owner, and three
   questions for cutting large cards before work starts. The agent noted
   that its earlier rule assumed a fixed order and dropped it. On the
   agent's question about split and superseded cards, the user chose
   cleanup cards and recording splits in the minute; the agent's reading
   that the original file is deleted was confirmed. → decisions 7, 8, 9
6. **Decisions in the plan.** The agent noted that three of them had no
   card. The user asked for suggestions; the agent proposed a finished
   card for the skeleton or a folder of decision records, preferring the
   card, and the user chose it. The user then added the rules on titles
   and on finished cards written as answers; the agent spelled them out,
   with the date rule, and the user confirmed. → decisions 10, 11
7. **Priorities.** The agent had placed two backlog cards in Later
   without a decision and asked; the user moved the build and deploy card
   to Now. → decision 12
8. **Checking the understanding.** Before the minute was committed, the
   user asked the agent where future decisions go, to see whether the two
   had understood each other. The agent answered: why in the card, who in
   the minute, the plan showing only the effect; and named the gap it saw,
   decisions that belong to no card. The user confirmed the rest and
   filled the gap with decision records. → decision 13

## Outcome

- Rules in `CLAUDE.md`: "Session minutes", "Commit attribution" (in
  `66d1ad0`), "The plan" and the completed "Backlog cards" (`772cea1`).
- `docs/plan.md` is a short board; its decisions are in
  [the skeleton card](../done/2026-10-26_put-working-skeleton-live.md)
  (`11b63f1`). The three finished cards of 2026-10-27 are rewritten as
  answers (`99415b4`).
- Commits: `427d046` and `66d1ad0` minute card started and closed,
  `a34e48d` planning card created, `df3d3c6` and `57b241f` planning card
  started and closed, plus `772cea1`, `11b63f1`, `99415b4`, `e92a99b`
  (decision records) and this minute. The first push (run
  [40562747395](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40562747395))
  touched only documentation: build, test and publication were skipped.
- Agent's mistakes, corrected before publishing: `a34e48d` was amended
  after decision 2 changed its premise; `66d1ad0` first left the old
  card file in `docs/wip/`; the planning card first claimed that none of
  the cards taken on 2026-10-27 was in the old plan, while the Playwright
  image card was.

## Open points

- `Assisted-by:` is written with an email address by the Haiku
  sub-agent, without one in the `CLAUDE.md` example; one form is still to
  be fixed.
- The minute of 2026-10-26 still points to `docs/plan.md` for the
  reasoning of its decisions 5–8 and 10; that text now lives in the
  skeleton card.
- The build and deploy card is in Now while it still has open questions,
  although Now is meant for well-defined cards.

## Next steps

- Decide whether the minute, commit and planning rules also go to
  `ll-dev-setup`, once they have been seen at work. Left to the user; no
  card yet.
