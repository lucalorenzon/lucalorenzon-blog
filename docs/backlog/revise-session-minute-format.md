# Backlog: make the session minute record who contributed what

- **Opened:** 2026-10-27
- **Status:** proposal, open; picked up when the user decides.

## Why this was asked

The rule in `CLAUDE.md` ("Session minutes") asks for the goal, the
decisions and why, what was done, open points and the next step. In
practice the minute repeats the card that moves to `docs/done/` in the
same session: the minute of 2026-10-27 restates the card's "Decision"
section almost point by point (option B, skip per job, safe default,
renames, option C dropped).

What the minute does not tell is how the session went: which ideas came
from the user and which from the agent, which proposals the user
accepted as they were, changed or rejected, and where the user stepped in
to redirect the work. Today this shows up only by chance (for example
"Asked by the user", "was the user's choice"), so a reader cannot tell
the author's contribution apart from the agent's. For a blog meant to
showcase the author's professional experience, that is the part worth
recording.

## Proposal

The minute stops repeating what the cards already say and documents the
interaction instead:

- **Content already in a card:** a link to the card, not a copy.
- **For each decision:** who raised it, who proposed the options, who
  chose, and whether the agent's proposal was accepted, changed or
  rejected, with the reason when the user changed course.
- **User's initiatives:** requests, corrections and redirections that did
  not come from an agent proposal.
- **Agent's work:** what was delegated (to which model) and how it was
  checked.

## Questions to settle when picking this up

- The exact shape of the attribution (a column in a decisions table, a
  short label per decision, a chronological account).
- Whether the existing minutes of 2026-10-26 and 2026-10-27 are rewritten
  or left as they are.
- The new wording of the "Session minutes" section in `CLAUDE.md`. The
  rule is local to this repository, so the change stays here and does not
  go to `ll-dev-setup`.

## When to pick this up

Whenever the user wants; ideally before the next minute is written, so
that minute already follows the new format.
