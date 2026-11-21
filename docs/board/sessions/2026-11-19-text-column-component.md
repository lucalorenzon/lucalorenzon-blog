# Session 2026-11-19: the breakpoints page, and the text column made a component

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: analysis, cards, the baseline of the
  built site, review of the sub-agent's work, commits.
- **One sub-agent**, Claude Sonnet 5.5: wrote the browser test of the
  column, the component and its unit tests, and ran every check.

Goal: start the page of the breakpoints, the last page of the style
guide before it is published (card
[show the breakpoints in the style guide](../cards/show-the-breakpoints-in-the-style-guide.md)).
The session answered how that page is built, found that the text
column had to become a component first, and did it
([card](../cards/make-the-text-column-a-component.md)).
It then took up the column's own page
([card](../cards/show-the-text-column-in-the-style-guide.md)) and stopped
on a spike. The session started late on 2026-11-18 and ended on
2026-11-19. Technical details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The build finds what changes at each breakpoint, so the list cannot get out of step with the code; the explanation comes from the author's Markdown, and the build puts the two together | The user | The agent | — |
| 2 | The build asks each component what changes in it, grouped by component; the Markdown is cut in pieces marked with the component, as `fonts.md` is; the breakpoints page links each component to its own page | The user | The agent | — |
| 3 | The header's parts, which all live in the page frame, are declared on the page as a limit of the site as it is today, with a refactor into components planned and the components to come listed | The user | The agent | — |
| 4 | The text column becomes a component, `OneColumnText`, with a page of its own in the style guide, before the breakpoints page; this changes knowingly the decision of 2026-11-18 to add no page of a component before publishing | The user | The agent | — |
| 5 | Two cards: the refactor of the column first, then its page | The user | The agent | — |
| 6 | The refactor is guarded by the column's behaviour, not by each page: unit tests on the component's classes, and a browser test on the style guide's text page, written before the change, which moves to the column's page later | The user | The agent | — |
| 7 | The module lives in `src/one_column_text.rs`, and the component takes either the Markdown's HTML or children | The agent | — | The user |
| 8 | The six acceptance tests of the refactor, among them a comparison of the site built before and after, made once | The user | The agent | — |
| 9 | A component that would get both HTML and children keeps only the HTML; recorded as a caution, the code left as it is | The agent | — | The user |
| 10 | How a component tells the build what changes in it is tried first in a spike on the text column; the column's page waits for the finding | The user | The agent | — |
| 11 | The unit of a declaration is a named change, with the classes that make it, sometimes none | The user | The agent | — |

## Discussion

1. **Who writes the list of changes.** The agent showed that a class
   carries only its name, while the decision of 2026-11-18 asks the
   page to say what happens and lead to the component's page. The user
   answered that the build finds the changes and the Markdown explains
   them. The agent at first read it as a Markdown explaining classes,
   and asked one change per class or one per thing the reader sees;
   the user corrected it: the component knows what its classes mean,
   so the build asks the components. → decisions 1 and 2
2. **Components that are not components.** The agent found that the
   header, the logo, the title and the footer all live in the page
   frame, and that the text column was written by hand in eight
   files. The user chose to declare the frame's case as a limit, and
   asked whether a card could make the column a component; then that
   the component should have its own page, which the breakpoints page
   links to. The agent noted the change from the decision of
   2026-11-18 and proposed cutting the work in two, a refactor and a
   page; the user agreed. → decisions 3, 4 and 5
3. **What guards the refactor.** The agent found that no test checked
   the column's behaviour, and that a test on a page of content would
   go against ADR-004. The user chose to test the component, not the
   pages: unit tests on the classes and a browser test on the style
   guide's text page. The agent proposed the acceptance tests,
   adding a comparison of the built site made once; the user accepted
   them. → decisions 6, 7 and 8
4. **What the browser test found.** The sub-agent measured the column
   600 pixels wide below 768 and 720 from 768; the figures of 584 and
   704 written in the tests' support file are the width of its text,
   without its padding. The agent clarified that comment and corrected
   its own wrong claim, made when opening the card, that the browser
   tests already checked the column's width. → decision 9
5. **How a component declares its changes.** The agent asked whether
   the mechanism is built with the column's page or with the
   breakpoints page. The user sketched a trait the build calls,
   giving a component code, the classes that change and their
   breakpoint, with Markdown headings `## <component> | <what it
   documents>`, and observed that what changes may be a behaviour, not
   only a class; the user said it was a brainstorm. The agent read the
   unit as a named change, raised three tensions (the explanation in
   two places, the declaration written by hand, two kinds of work in
   one card) and proposed a spike; the user agreed. At the end of the
   session the user confirmed the agent's reading of the unit, and
   asked to add the confirmation to this minute. → decisions 10 and 11

## Outcome

- The breakpoints page taken into work, with the build finding the
  changes (`84ee8f5`).
- The text column cut out of the breakpoints page, in two cards
  (`1a121f2`).
- The refactor's guard and acceptance tests agreed (`2cdadc4`).
- The text column made a component, `OneColumnText`, with its unit
  tests and the browser test `text-column.spec.ts`; the site built
  before and after is the same, file by file (`2b77e75`).
- The refactor's card closed; the repository map names the new module
  (`7a541b0`).
- The column's page taken into work (`da949e2`).
- The spike on the breakpoint declaration opened, and the plan
  updated (`8fe4140`).

## Open points

- The other questions of the breakpoints page are still open: what
  the page puts to the test, whether the list of devices can come from
  Firefox's, what the tests' 320 is called, and how the index declares
  the pages to come.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Try how a component declares what changes in it at each breakpoint,
  on the text column: card
  [try a breakpoint declaration on the text column](../cards/try-a-breakpoint-declaration-on-the-text-column.md).
- Show the text column in the style guide, after the spike: card
  [show the text column in the style guide](../cards/show-the-text-column-in-the-style-guide.md).
- Show the breakpoints in the style guide, after the column's page:
  card [show the breakpoints in the style guide](../cards/show-the-breakpoints-in-the-style-guide.md).
