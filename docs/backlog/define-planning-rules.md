# Backlog: define the rules for planning and project management

- **Opened:** 2026-10-27
- **Status:** proposal, open; to be picked up right after the card
  [make the session minute record who contributed what](../wip/revise-session-minute-format.md),
  in the same session, as the user decided.

## Why this was asked

While reworking the session minute format, the user set what the
minute's "next steps" mean: the actions that came out of the session,
not the order in which work is done. When an action is done is a
planning matter. A minute records how a session went and is not edited
afterwards; if it also held the order of the work, it would have to be
touched every time that order changed.

That leaves a gap. Today the changes of order are written down only in
the minutes:

- `docs/plan.md` was last updated on 2026-10-26. Its "Next" section still
  names only the `residuality` challenge.
- Since then, three cards were picked up instead: the pipeline card, the
  Playwright image card and the minute format card. None of them appears
  in the plan. The choice to take the pipeline card ahead of the planned
  one is recorded only in the minute of 2026-10-27.

Once the minute stops carrying the order of the work, nothing records
these choices unless the planning rules say where they go.

## Already raised, to be settled here

These two points came up in the minute format session as a consequence
of that decision, so they would not be lost. They belong to this card:

- **A rule proposed by the agent:** when the user picks a card out of the
  planned order, `docs/plan.md` is updated in the same commit that moves
  the card to `docs/wip/`. The user agreed with the idea; the wording
  and its place in `CLAUDE.md` are still open.
- **Bringing `docs/plan.md` up to date** with the work done and the cards
  in the backlog.

## Questions to settle when picking this up

- What the plan holds and what the cards hold, so that the same fact is
  not kept in two places.
- When the plan is updated, and by whom.
- How the plan's Now / Next / Later relates to the three card folders
  (`docs/backlog/`, `docs/wip/`, `docs/done/`).
