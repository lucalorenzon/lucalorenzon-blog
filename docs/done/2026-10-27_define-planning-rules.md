# Define the rules for planning and project management

- **Opened:** 2026-10-27
- **Status:** done on 2026-10-27. The rules are in `CLAUDE.md`, sections
  "The plan" and "Backlog cards"; each answer below was settled with the
  user, one at a time. Taken in the same session as the
  [session minute format card](2026-10-27_revise-session-minute-format.md),
  which raised it.

## Why it was opened

While reworking the session minute format, the user set what a minute's
"next steps" mean: the actions that came out of the session, not the
order in which work is done. When an action is done is a planning matter.

That left a gap. Changes in the order of the work were written down only
in the minutes, and `docs/plan.md` had fallen behind: last updated on
2026-10-26, its Next section still named the `residuality` challenge as
the next thing, and two of the three cards taken on 2026-10-27 instead
(the pipeline card and the minute format card) did not appear in it. The
third, the Playwright image card, did. The plan also held technical
decisions that partly repeated the finished cards.

## Answers

### What is the plan for?

**It is the project's board**, the only file updated continuously. It
maps the road toward the project's outcome in three horizons: Now (what
is being tackled), Next (what is expected right after), Later (what is
known or wished for, but not for now). It is kept short on purpose: its
value is in making it, and detail is dispersive.

Rejected: a fixed order of execution. Software development is learning,
so the plan is reworked by successive approximations; what matters most is
not doing too many things at once.

### What does Now hold, and when does the plan change?

**The card being worked on, plus the few cards that are well defined and
clearly come first.** The plan changes whenever priorities change, also
during a session. This card is the example: it was not planned, it was
created because the work showed a direction was needed, and it was taken
ahead of `residuality`, which is urgent too, because working without these
rules would make the following work worse.

The rule first proposed by the agent, "update the plan in the same commit
that moves a card taken out of the planned order", was dropped: with no
fixed order there is no "out of order", only a plan kept up to date.

### Who sets the priorities?

**The user, acting as Product Owner**, possibly discussing them with the
agent. The backlog collects every idea or question; a card may never be
done and end up superseded by others.

### How is a card too large for one session handled?

**The user and the agent discuss it together**, asking what can be done
right away and gives value at once, which activities increase learning
about where to go, and whether it can be cut vertically, for example by
limiting the kind of data handled. Once a minimal slice is found and judged
worth doing, its card is created (two or three of the same size may be
created together) and work starts. Planning stops there: validating its
assumptions with code matters more than settling every detail in advance.
This is what the global development pipeline missed when it was applied
before the restart: it tried to decide everything first.

### How do cards move?

**Backlog, then work in progress, then done, as a general line.**

- Usually only one card is in `docs/wip/`, the session's card; the user
  did not make it a fixed rule, not having enough evidence yet.
- A card split into smaller ones does not go to `docs/done/`. The new
  cards go to the backlog and the original file is deleted in the same
  commit; the session minute records the split with that commit, and the
  git history keeps the original text.
- Superseded cards stay in the backlog until a cleanup card clears it,
  from time to time.

### Where do the decisions held in the plan go?

**Into a finished card for the working skeleton**, written afterwards and
dated 2026-10-26, the day that work finished:
[put a working skeleton of the site live](2026-10-26_put-working-skeleton-live.md).
Decisions keep a single home, the card, while the minute says who took
them. The open decision on the site address already had its card, so it
simply left the plan.

Rejected: a separate folder of decision records, one file per decision.
It is a recognised practice, but it would be a third place for decisions
after cards and minutes: the kind of ceremony the restart set out to
avoid.

### How are cards written?

- **The title names the subject**, with no "Backlog:" prefix: the folder
  tells the state.
- **A finished card reads as the answer to the questions it raised**: why
  it was opened, the answers with the rejected options and their reason,
  and the results. Proposal-time sections are removed. Asked by the user,
  who noticed that finished cards still read as if the work were to be
  done.
- **The date in a finished card's name is the day the work finished**,
  not the day the file moved to `docs/done/`.

## Results

- Rules in `CLAUDE.md`: `772cea1`.
- Plan turned into a board, skeleton card written, references updated:
  `11b63f1`. The user placed the build and deploy card in Now.
- Titles and the three finished cards of 2026-10-27 rewritten: `99415b4`.
