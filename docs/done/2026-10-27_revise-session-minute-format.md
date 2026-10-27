# Make the session minute record who contributed what

- **Opened:** 2026-10-27
- **Status:** done on 2026-10-27. The rules are in `CLAUDE.md`, sections
  "Session minutes" and "Commit attribution"; each answer below was
  settled with the user, one at a time.

## Why it was opened

The rule in `CLAUDE.md` ("Session minutes") asked for the goal, the
decisions and why, what was done, open points and the next step. In
practice the minute repeated the card that moved to `docs/done/` in the
same session: the minute of the pipeline session restated the card's
decision almost point by point.

What the minute did not tell is how the session went: which ideas came
from the user and which from the agent, which proposals the user accepted
as they were, changed or rejected, and where the user stepped in to
redirect the work. This showed up only by chance ("Asked by the user",
"was the user's choice"), so a reader could not tell the author's
contribution apart from the agent's. For a blog meant to showcase the
author's professional experience, that is the part worth recording.

## Answers

### What shape does the attribution take?

**Decisions first, then a short discussion.** The table lists the
decisions with the MADR roles: decision-makers, consulted, informed. Then
one short point per topic says who raised it, who proposed what and how
it ended, with a reference to the decisions it led to. Only what shaped
the decisions goes in; technical details stay in the card, and what a
card already says is linked, not repeated.

How it was reached, all tried on the Playwright image minute:

- A chronological account of the dialogue answered the question, but the
  user found it unreadable and asked for a workshop-style minute.
- Asked for existing practices, the agent found no standard for minutes
  of sessions between a person and an AI agent, only practices covering
  part of the problem (see "References"). Of the decision-role schemes,
  the user chose MADR, the most used and the most readable. Rejected:
  DACI, whose "Driver" means who moved the process rather than whose idea
  it was, and leaves half the cells empty in a two-party session; and a
  mixed table (Proposed by, Decided by, Informed) proposed by the agent.
- A table alone lost who proposed what. The user asked for a short
  discussion, as in meeting minutes, and for the table to come first.

### What are a minute's next steps?

**The actions that came out of the session, each linked to its card.**
When they are done is decided by the plan, not by the minute: a minute
records a session and is not edited afterwards, so if it held the order
of the work it would have to be touched whenever priorities change. The
user first dropped the section, then restored it with this meaning,
noticing that the old minutes used it for "what to do next". Open points
stay a separate, optional section for questions left unresolved.

### Are the existing minutes rewritten?

**No, except the Playwright image minute.** A minute is not edited once
committed, so the minutes of 2026-10-26 and of the pipeline session keep
their earlier format. The Playwright image minute was already declared a
trial for this card; rewritten, it is now the reference example.

### How do commits tell the author apart from the agent?

**`Assisted-by:` instead of `Co-Authored-By:`**, as in Fedora's policy:
the user stays the only author and accountable, the line names the model
that did the work, and a sub-agent names its own model. It came up
because `acd67ed`, made by the Haiku sub-agent, named Opus: the agent had
written that line into its instructions. The `Claude-Session:` line
stays, to trace a commit back to its session.

### Where do these rules live?

**In this repository's `CLAUDE.md`.** Whether they also go to
`ll-dev-setup` is left to the user, after seeing how they work here.
Recording the order of the work and bringing `docs/plan.md` up to date
went to a card of their own:
[define the rules for planning and project management](2026-10-27_define-planning-rules.md).

## References

Found on 2026-10-27; each covers a part of the problem.

- **Who decided.** MADR, a common template for decision records, has the
  fields `decision-makers`, `consulted` and `informed` (RACI-like).
  Atlassian's DACI names a Driver, an Approver, Contributors and the
  Informed.
  [MADR notes](https://cdn.jsdelivr.net/npm/@harness-forge/cli@1.5.5/skills/madr-decision-log/references/madr-reference.md),
  [DACI](https://www.atlassian.com/team-playbook/plays/daci).
- **Who did what, per commit.** Fedora's AI-assisted contributions policy
  (October 2025) recommends an `Assisted-by:` commit trailer; the human
  contributor stays the author and fully accountable.
  [Fedora policy](https://docs.fedoraproject.org/ca/council/policy/ai-contribution-policy/).
- **How much the AI contributed, per piece of work.** IBM Research's AI
  Attribution Toolkit (a research prototype, inspired by the CRediT
  contributor roles used in science) produces a short statement: the AI
  system used, what it contributed, how much the human changed or curated
  it, and who holds final accountability.
  [IBM blog](https://www.research.ibm.com/blog/AI-attribution-toolkit),
  [repository](https://github.com/aiattribution/aiattribution.github.io).
