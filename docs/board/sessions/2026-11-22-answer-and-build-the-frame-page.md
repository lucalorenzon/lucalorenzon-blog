# Session 2026-11-22 — answer and build the frame's page

**Participants:** the user (author); the agent (Claude Opus 5.5), with
a sub-agent on Claude Sonnet 5.5 (wrote the frame page's code and unit
tests and ran the pipeline's checks), and a web-fetch sub-agent, its
model not recorded (read Creative Commons' frequently asked questions
on software).
**Goal:** pick up the plan after the logo, and take the card
[show the page frame in the style guide](../cards/show-the-page-frame-in-the-style-guide.md)
from its open questions to a built page. Technical details and reasons
are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Go on with the frame's page, before the pieces the logo left | user | agent | — |
| 2 | The frame stays one page; its parts are not cut into components until the header's dynamic behaviours shape them; the card on the logo as a component waits for them | user | agent | — |
| 3 | The frame's component tests move from the home page to its page; the net, the home page's own tests and the "page not found" stay | user | agent | — |
| 4 | No `iframe`: the page explains what changes, inside the real frame | user | agent | — |
| 5 | Cut from the card the device list and the index's pages to come, into two cards | user | agent | — |
| 6 | A component's explanation lives in a Markdown file beside its code; the other pages follow after publishing, in a card of their own | user | agent | — |
| 7 | Record decision 6 in ADR-005 | user | agent | — |
| 8 | The code stays under MIT; the text beside the code keeps the site text's licence, CC BY 4.0 | user | agent | — |
| 9 | The page is dated in the sitemap by its text, as the other pages | user | agent | — |
| 10 | The section headings' size belongs to the frame, their look to the text page | user | agent | — |
| 11 | Cut the card to the page itself; the moves of the tests go to two cards | user | agent | — |
| 12 | The agent writes a draft of the page's text, declared a placeholder; the user rewrites it later | user | agent | — |
| 13 | Nine acceptance tests for the page | user | agent | — |
| 14 | The tab title takes the form of the other style guide pages | agent | — | user |
| 15 | The page, the draft and the licence lines are approved; commit, close the card and the session | user | agent | — |

## Discussion

1. **Where to start.** The agent read the plan and the last minute,
   noted that the frame's card still described the logo's change at
   768 pixels, gone with the new logo, and that the card on the logo
   as a component asked the same question as the frame's card; it
   proposed going on with the frame's page. The user agreed. →
   decision 1
2. **One page or several.** The agent stated that the frame is one
   component in the code and that pages for its parts meant either
   cutting the code or describing parts that are not components. The
   user judged the cut premature on static HTML, and described the
   behaviours expected soon (background images, a header turning
   opaque, parallax, the title moving into the header, cards
   scrolling sideways, two columns on wide screens), which should give
   the components their shape. The agent agreed, adding that the
   page's tests check what the visitor sees and would guard a later
   cut. → decision 2
3. **What the page tests.** The agent noted that the home page did for
   the frame what the articles once did for the text's components. The
   user chose to move the tests; the agent checked its reading against
   ADR-004 and the user confirmed it. → decision 3
4. **The `iframe`.** The agent set out samples side by side against an
   explanation; the user chose the explanation, an `iframe` being
   awkward on a phone. → decision 4
5. **Keeping the card on publishing.** The agent proposed which
   questions leave the card; the user agreed. → decision 5
6. **Where the explanation lives.** The agent stated the tension
   between the other pages' Markdown in `content/` and the user's
   principle of documentation with the code; asked for an example, it
   proposed a Markdown file beside the component, read both as the
   module's documentation and as the page, and named four
   consequences. The user chose it and asked for a card to align the
   other pages after publishing; the agent read the project rule on
   decisions spanning several cards as calling for a record, and the
   user confirmed. → decisions 6 and 7
7. **The licence.** The user proposed putting the code under Creative
   Commons too, to simplify. The agent advised against it, from
   Creative Commons' own frequently asked questions, read by a
   sub-agent, part verbatim and part only in summary, and proposed
   declaring the explanation files as text of the site instead; the
   user accepted. → decision 8
8. **The date.** The agent set out the text's date against the
   component's; the user chose the text's. → decision 9
9. **The boundary with the text page.** The agent found two cases
   across it; the user saw the headings' size as a rule the frame
   injects, with a confidence of 60%. The agent agreed, with the rule
   that the place goes to whoever decides the value, and proposed
   splitting the heading in two; the user accepted. → decision 10
10. **The size of the slice.** With every question answered, the agent
    said the work was too large for one slice and proposed three; the
    user agreed. → decision 11
11. **The text and the acceptance tests.** The agent asked who writes
    the text; the user asked for a draft. Before proposing the tests,
    the agent corrected a claim it had made earlier: the check that
    Markdown holds no HTML reads only the articles, not `content/`.
    The user accepted the tests. → decisions 12 and 13
12. **The build.** A sub-agent wrote the code; the agent noticed that
    its own brief had given a tab title in another form than the other
    pages, had it corrected, and renamed two leftovers copied from the
    code page. The agent's first screenshots, taken of the whole page,
    drew the sticky footer over the text; it retook them as the reader
    sees them. The user approved and closed the session. → decisions
    14 and 15

## Outcome

- The frame's card brought up to the new logo and its questions
  answered (`9e429a4`, `c4ae12a`, `f9674b5`, `741e654`, `5706e12`,
  `b7d28e3`, `5b56cea`, `e0f1589`), cut twice (`de92349`, `4ba3e35`),
  its acceptance tests (`5a6339e`).
- ADR-005, a component explained beside its code (`5e8f37c`).
- The page frame's page built (`b5a0cfa`); the card closed
  (`66f49fc`).
- New cards:
  [follow a known device list in the browser tests](../cards/follow-a-known-device-list-in-the-tests.md),
  [declare the pages to come in the style guide's index](../cards/declare-the-pages-to-come-in-the-style-guide-index.md),
  [move the style guide's texts beside their code](../cards/move-the-style-guide-texts-beside-their-code.md),
  [move the frame's tests to its page](../cards/move-the-frame-tests-to-its-page.md),
  [move the text column's tests to the frame's page](../cards/move-the-column-tests-to-the-frame-page.md).
- Not pushed: the local branch is ahead of the remote.
- Screenshots, not committed, in `target/frame-look/`.

## Open points

- The page's text is a draft by the agent, for the user to rewrite;
  it has no card, and is recorded in the closed card's cautions.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Move the frame's tests to its page:
  [card](../cards/move-the-frame-tests-to-its-page.md).
- Move the text column's tests to the frame's page:
  [card](../cards/move-the-column-tests-to-the-frame-page.md).
- Declare the pages to come in the index, before publishing:
  [card](../cards/declare-the-pages-to-come-in-the-style-guide-index.md).
- After publishing:
  [move the style guide's texts beside their code](../cards/move-the-style-guide-texts-beside-their-code.md)
  and
  [follow a known device list in the browser tests](../cards/follow-a-known-device-list-in-the-tests.md).
