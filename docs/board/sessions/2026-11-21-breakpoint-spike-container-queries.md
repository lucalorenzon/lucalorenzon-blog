# Session 2026-11-21: the breakpoint spike turns to container queries

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: searches on the web, analysis, the
  card's text, the experiment's brief and the check of its results,
  commits.
- **One sub-agent**, Claude Sonnet 5.5: ran the experiment with a
  container query on a throwaway branch, with every check of the
  build.

Goal: the first session of the spike on how a component tells the
build what changes in it at each breakpoint, tried on the text column
(card
[try a breakpoint declaration on the text column](../cards/try-a-breakpoint-declaration-on-the-text-column.md)).
The session started on 2026-11-19 and ended on 2026-11-21. Technical
details, sources and reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The spike is taken into work with a time limit of one session, at most 200,000 tokens of the agent's context | The user | — | — |
| 2 | The spike is extended to two sessions, each within the same limit | The user | The agent | — |
| 3 | The spike tries the column with a container query, sized by the space the component has in the page, instead of the declaring trait | The user | The agent | — |
| 4 | The experiment answers six questions, on a throwaway branch, run by a sub-agent; the main branch stays untouched | The agent | — | The user |
| 5 | The thresholds belong to the component; the column's page would show the component at each of them, with a test that checks what it shows, instead of a declared list of changes; the breakpoints page may lose its sense | The user | The agent | — |
| 6 | The site defines the style it injects: the values the components choose from (text sizes, spacing, container sizes) | The user | The agent | — |

## Discussion

1. **Whether the problem is real.** The user asked whether others had
   faced it, or whether it was not a problem. The agent searched and
   found three ways: documentation beside the code, components
   observed at each width, and the breakpoint as a parameter of the
   component; none reads the list of changes from the code. The user
   found the third illuminating and said the first was where they
   were going anyway. → decision 2
2. **Untangling the questions.** The agent gave two readings of the
   user's "breakpoint injected", the component declaring its own
   changes or the caller passing them; the user said they were
   getting tangled. The agent separated three questions (who decides,
   how the declaration stays in step, where the explanation lives).
   The user asked to extend the spike, and asked whether their
   direction went against common practice, wanting either to change
   position or to have a good reason for it. → decision 2
3. **Primitives and the column's cut.** The agent found that the
   responsive properties are given only to generic layout primitives;
   the user asked which would be the primitive here. The agent
   answered: the stylesheet's classes, while `OneColumnText` holds the
   site's choices, the look of the whole text among them. The user
   observed that the component is then cut wrong, taking over more
   than "one column"; the agent named two tensions (the name says
   "text"; width and text size are one thing for the stylesheet). The
   user's view: the site sets the rules and the values, the component
   chooses among them for the space it has. → decision 6, open points
4. **The space the component has.** The agent pointed out that "the
   space it has" is not the screen. The user confirmed it was meant,
   though not fully convinced; the agent searched, confirmed the
   reading (container queries) and listed the costs it saw; the user
   asked to try. → decision 3
5. **The experiment.** The agent wrote the six questions; the
   sub-agent answered them with evidence; the agent checked the
   generated rule and the samples' screenshot. It works with no side
   effect, but moves the switch with a classic scrollbar, needs the
   class checks to know the container sizes, and samples of the column
   would each be a duplicate `<main>`. → decision 4
6. **What it means for the breakpoints.** The agent proposed reading
   it as thresholds of the component, shown and tested on its page.
   The user agreed and went further: the header and the footer would
   move to containers too, so the breakpoints page would have nothing
   left to show, and the site should rather define the style it
   injects. The agent named the consequences on the breakpoints page's
   card and on `config/breakpoints.txt`, and that moving the page's
   skeleton goes further than the sources; the user asked to check
   that point first in the next session. → decisions 5 and 6, open
   points

## Outcome

- The spike taken into work, with its time limit (`c84eaac`).
- The search on how others face the problem, the questions regrouped
  and the spike extended (`bdea3cb`).
- The discussion on who decides, and the turn to container queries
  (`6ed629e`).
- The experiment's findings, with the throwaway branch
  `worktree-agent-afb8353149f05acd9` (commit `d018d05`), kept until
  the spike closes (`22816a9`).
- The user's view on the breakpoints and the injected style
  (`3ee1298`).

## Open points

- Whether the header and the footer move to containers: the user's
  view at the end of this session, to be checked first.
- Whether `OneColumnText` is cut wrong, holding the look of the whole
  text besides the column, and whether to cut it again.
- Where the explanation of a component lives: beside the code, or in
  the Markdown of `content/` as today.
- Whether the change of behaviour with a classic scrollbar is
  accepted, and how a test would see it.
- What the card of the breakpoints page becomes, and with it the
  plan's last step before publishing the style guide.
- How a sample of the column avoids a second `<main>`, and how a
  mistyped container size, which Tailwind leaves out silently, is
  caught.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The second session of the spike, starting from whether the header
  and the footer move to containers, then writing the finding and
  deleting the throwaway branch: card
  [try a breakpoint declaration on the text column](../cards/try-a-breakpoint-declaration-on-the-text-column.md).
