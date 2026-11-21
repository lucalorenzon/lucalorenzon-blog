# Session 2026-11-21: the breakpoint spike closes

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: analysis, the cards' text, the plan,
  commits. No sub-agent took part.

Goal: the second and last session of the spike on how a component
tells the build what changes in it at each breakpoint, starting from
the point the first session left to check, whether the header and the
footer move to containers; then the spike's finding and its close
(card
[try a breakpoint declaration on the text column](../board/cards/try-a-breakpoint-declaration-on-the-text-column.md)).
The first session's minute:
[2026-11-21, the breakpoint spike turns to container queries](2026-11-21-breakpoint-spike-container-queries.md).
Technical details, sources and reasons are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The page frame stays on the screen's breakpoints; only the text column moves to a container query | The user | The agent | — |
| 2 | The column's text grows smoothly with its container, within a range of sizes the site injects, instead of choosing among fixed sizes | The user | The agent | — |
| 3 | The page frame's text takes its sizes from the same ranges, growing with the screen; its changes of layout stay on the screen's breakpoints | The user | The agent | — |
| 4 | The spike has answered; its open points become four new cards, and the question of where a component's explanation lives moves to the text column's page | The user | The agent | — |
| 5 | The breakpoints page's card is closed in `docs/done/` as superseded, not deleted: it was worked on, and the work it foresaw falls as a consequence of that work, which is not a split | The user | The agent | — |
| 6 | Order of Now: the card on referring to cards by name in frozen documents first, then whether the text column is cut again, then the column's text and page, then the frame's text and page | The user | The agent | — |

## Discussion

1. **The header and the footer.** The agent, checking the first
   session's open point, named three tensions: for the frame the space
   it has is nearly the screen, so a container brings no modularity;
   the screen's breakpoints stay anyway for what only the screen
   knows; and a risk, not verified, for the planned sticky header. The
   user said the expected gains were one mechanism and no breakpoints
   page. The agent observed that the two do not depend on each other,
   proposed `iframe` samples for the frame's page, and weighed two
   options, preferring the frame on the screen's breakpoints; the user
   chose it, the single mechanism not being worth the work before
   publishing. → decision 1
2. **Whether the column needs a threshold.** The agent named the
   tension between text that grows smoothly, as intrinsic design
   suggests, and the user's view of the first session, a set of sizes
   the site offers. The user explained the worry behind it,
   uniformity, and observed that inside a container the space is the
   same for every child, so fixed sizes imposed from outside would be
   too rigid. The agent named the change to the first session's
   decision on the injected style, then offered two ways, bounds set by
   the component or a range the site injects; the user chose the
   range. → decision 2
3. **The frame's text.** The agent named a tension with decision 1:
   one page would mix text sizes that jump with text that grows. The
   user extended the ranges to the frame. → decision 3
4. **Closing the spike.** The agent proposed reading the spike's
   question as answered, nothing being declared, and turning the open
   points into cards; the user agreed. The agent presented the plan of
   the commits and two ways for the superseded breakpoints card,
   preferring to close it in `docs/done/`; the user chose it and gave
   the reason that tells it from a split. → decisions 4 and 5
5. **The order of the work.** The agent proposed an order for Now and
   noted, outside the task, two finished cards whose links to the
   design system's card were already broken. The user put the cut of
   the text column first among the components, having thought of a
   way that may contradict what was decided, or end quickly; and asked
   to take up the card on broken links first, having waited too long.
   → decision 6

## Outcome

- The page frame kept on the screen's breakpoints (`a2a6700`).
- The text growing within ranges the site injects, for the column and
  the frame (`a0bd441`).
- Four new cards in the backlog (`0fca5ae`).
- The breakpoints page's card closed as superseded, the text column's
  card updated, links following the moved card (`1dbcc0c`).
- The spike closed with its finding, the plan and the design system's
  card updated, the throwaway branch `worktree-agent-afb8353149f05acd9`
  and its working folder deleted (`70bacfc`).
- The plan's Now reordered (`7f4de64`).

## Open points

- The user's new idea on how to cut the text column, not told yet: to
  be checked against what was decided, in its card.
- Whether a fixed header inside a page declared a container scrolls
  away with the page, in today's browsers: not verified, and the
  sticky header has no card yet.
- Two finished cards,
  `2026-11-14_link-the-style-guide-pages-to-its-index.md` and
  `2026-11-15_show-the-code-block-in-the-style-guide.md`, link to the
  design system's card as if it were in their own folder.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Stop moved cards from breaking links in minutes and finished cards:
  card
  [refer to cards by name in frozen documents](../board/cards/refer-to-cards-by-name-in-frozen-documents.md).
- Decide whether the text column is cut again, starting from the
  user's new idea: card
  [decide whether to cut the text column again](../board/cards/decide-whether-to-cut-the-text-column-again.md).
- Let the column's text grow within a range the site injects: card
  [let the column's text grow within a range the site injects](../board/cards/let-the-column-text-grow-within-an-injected-range.md);
  then its page: card
  [show the text column in the style guide](../board/cards/show-the-text-column-in-the-style-guide.md).
- Let the frame's text grow within the same ranges: card
  [let the page frame's text grow within the injected ranges](../board/cards/let-the-frame-text-grow-within-the-injected-ranges.md);
  then its page: card
  [show the page frame in the style guide](../board/cards/show-the-page-frame-in-the-style-guide.md).
