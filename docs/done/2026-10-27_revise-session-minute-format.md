# Backlog: make the session minute record who contributed what

- **Opened:** 2026-10-27
- **Status:** done on 2026-10-27, see "Decision".

## Decision

The new rules are in `CLAUDE.md`, sections "Session minutes" and "Commit
attribution". Each point below was settled with the user, one at a time.

- **Shape: decisions first, then a short discussion.** A chronological
  account of the dialogue, tried first, answered the question but was
  unreadable. A workshop-style minute with only a decisions table was
  readable but lost who proposed what and where the user changed course.
  The final shape keeps both: the table with the MADR roles (chosen by
  the user as the most used and most readable), followed by one short
  point per topic saying who raised it, who proposed what and how it
  ended, with a reference to the decisions. Only what shaped the
  decisions goes in; technical details stay in the card. The order,
  table first, was the user's choice.
- **Next steps are actions, not the order of the work.** They list the
  actions that came out of the session, each linked to its card; when
  they are done is decided by the plan. The user first dropped the
  section, then restored it with this meaning: the old minutes used it
  for "what to do next", which belongs to the plan and would force a
  minute to be edited whenever priorities change. Open points stay a
  separate, optional section for questions left unresolved.
- **Minutes are not edited once committed.** The minutes of 2026-10-26
  and of the pipeline session keep their earlier format. Only the
  Playwright image minute was rewritten, since it was already declared
  a trial for this card; it is now the reference example.
- **`Assisted-by:` instead of `Co-Authored-By:`**, as in Fedora's policy:
  the user stays the only author and accountable, the line names the
  model that did the work, and a sub-agent names its own model. This
  came up because `acd67ed`, made by Haiku, named Opus. The
  `Claude-Session:` line stays, to trace a commit back to its session.
  The rule is local to this repository; whether it goes to
  `ll-dev-setup` is left to the user, after seeing how it works here.
- **Planning moved out.** Recording the order of the work, and bringing
  `docs/plan.md` up to date, went to a card of its own:
  [define the rules for planning and project management](../wip/define-planning-rules.md).

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

## References found on 2026-10-27

No standard was found for minutes of sessions between a person and an
AI agent; these practices each cover a part of the problem.

- **Who decided.** MADR, a common template for decision records, has
  the fields `decision-makers`, `consulted` and `informed` (RACI-like).
  Atlassian's DACI names a Driver, an Approver, Contributors and the
  Informed.
  [MADR notes](https://cdn.jsdelivr.net/npm/@harness-forge/cli@1.5.5/skills/madr-decision-log/references/madr-reference.md),
  [DACI](https://www.atlassian.com/team-playbook/plays/daci).
- **Who did what, per commit.** Fedora's AI-assisted contributions
  policy (October 2025) recommends an `Assisted-by:` commit trailer; the
  human contributor stays the author and fully accountable. More
  precise than the `Co-Authored-By:` used so far, which puts the agent
  on the same level as the author.
  [Fedora policy](https://docs.fedoraproject.org/ca/council/policy/ai-contribution-policy/).
- **How much the AI contributed, per piece of work.** IBM Research's AI
  Attribution Toolkit (a research prototype, inspired by the CRediT
  contributor roles used in science) produces a short statement: the
  AI system used, what it contributed, how much the human changed or
  curated it, and who holds final accountability.
  [IBM blog](https://www.research.ibm.com/blog/AI-attribution-toolkit),
  [repository](https://github.com/aiattribution/aiattribution.github.io).

The minute of 2026-10-27
(`docs/sessions/2026-10-27-run-browser-tests-in-playwright-image.md`)
was used to try the shape: a workshop-style minute with a short
decisions table.

## When to pick this up

Whenever the user wants; ideally before the next minute is written, so
that minute already follows the new format.
