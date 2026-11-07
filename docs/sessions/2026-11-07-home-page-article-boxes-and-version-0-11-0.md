# Session 2026-11-07: boxes for the home page's articles, and version 0.11.0

## Participants and goal

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents on Claude Sonnet 5.5 (the prototypes and comparison
screenshots, the build of the boxes and their tests).

**Goal:** frame each article on the home page in a box, its border in
the amber of the page's lines. Card:
[frame each article on the home page in a box](../done/2026-11-07_frame-home-page-articles-in-boxes.md).
A second card opened during the session:
[show the first box of the home page at mid-height of the screen](../done/2026-11-07_centre-first-box-on-home-page.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | A card for the boxes, straight into `docs/wip/` | user | — | agent |
| 2 | The whole box leads to the article; the link stays on the title, its clickable area stretched over the box | user | agent | — |
| 3 | The box shows it is active by a thicker border, under the mouse and with the keyboard focus | user | agent | — |
| 4 | On touch screens the active state is never seen, and that is accepted | user | agent | — |
| 5 | Border 2 pixels at rest, 4 when active | user | agent | — |
| 6 | Corners rounded, 8 pixels; the boxes and the code blocks keep different shapes | user | agent | — |
| 7 | Inner space 0.75 times the text size; space between boxes 2.25, as before | user | agent | — |
| 8 | The date as a label on the bottom border, at the right, in italic capitals at 0.875 times the text size | user | agent | — |
| 9 | The rule spacing the boxes wins by specificity, and a layout test measures the space | agent | — | user |
| 10 | ADR-003 counts the frame of the boxes among the uses of amber | user | agent | — |
| 11 | Moving the first box to mid-height of the screen is not adopted; its card is closed as tried | user | agent | — |
| 12 | Stop the trial servers, push, publish version 0.11.0, then this minute | user | agent | — |

## Discussion

1. **The card.** The user proposed boxes in the amber of the lines and
   asked for a card in `docs/wip/`. The agent noted that ADR-003 already
   settles the colour, but lists the frame of code blocks, not frames in
   general. → decisions 1, 10
2. **What the box is.** The agent asked whether the box is only a frame
   or something to click. The user wanted the whole box clickable. The
   agent set out two ways to build it, which look the same but sound
   different to a screen reader, and recommended keeping the link on the
   title. The user agreed. → decision 2
3. **The active state.** The user asked for alternatives side by side.
   The agent showed three, with the contrast between active and resting
   box: the tinted fill fell far below 3:1 and broke a rule of ADR-003.
   The user chose the thicker border. On a simulated phone the user saw
   no change; the agent explained that a touch screen has no hovering,
   and the user accepted it. → decisions 3, 4
4. **Border, corners and spaces.** For each, the user asked to see
   alternatives side by side. The agent added measurements: 1.5 pixels
   is painted as 1 by Chromium; with a 2-pixel border at rest the active
   one goes to 4, to keep the change visible; a 4-pixel radius almost
   vanishes when the border thickens. The agent also corrected its own
   claim that the inner space would change the number of lines: it did
   not. The user chose each value, and accepted that the rounded boxes
   differ from the square code blocks. → decisions 5, 6, 7
5. **The date.** The user asked to see the date as a label on the
   border. Three positions were compared. The user first chose the top
   left; the agent pointed out that the date would then be seen first
   but read last by a screen reader, and offered two ways out. The user
   changed to the bottom right, which removes the tension, and asked for
   a smaller italic in capitals. A grid of sizes and styles showed that
   the served Literata has no small caps, and that a click on the half
   of the label below the box did not open the article. The user chose
   italic capitals at 0.875. → decision 8
6. **The lost space.** In the build's screenshots the agent saw the
   boxes almost touching, while every test passed. It found that the
   rule for the space had been winning only by its place in the
   stylesheet, which Tailwind had changed. The rule was made heavier, a
   test was added, and the sub-agent saw it fail before the fix.
   → decision 9
7. **The first box at mid-height.** The user asked for the first box to
   appear at mid-height of the screen. The agent checked the reading,
   wrote the card and raised the risk of pushing the other articles off
   a phone's screen. The user asked to see it: on screen the page looked
   much worse, and the trial had moved the header's line, which the user
   wants at the top. The user felt something else is not right with the
   page, and chose to keep it as it was. The agent proposed closing the
   card as tried rather than deleting it, so the reason stays written.
   → decision 11
8. **Closing.** Before the minute the agent offered the pending steps:
   the trial servers, the push, a version. The user asked for all of
   them. → decision 12

## Outcome

- `be712ca`: the card on the boxes, in `docs/wip/`.
- `d653de2`: the boxes, with browser tests for the clicks, the active
  state, the space between boxes and the label's place on twelve
  devices.
- `1913968`: ADR-003 extended.
- `cfb9277`: the card on the boxes, in `docs/done/`.
- `2076438`: the card on the first box at mid-height, in `docs/wip/`.
- `594977a`: that card, closed in `docs/done/` as tried and not adopted.
- `f36597a`: `CHANGELOG.md`, version 0.11.0; tagged `v0.11.0`.
- `77505a8`: the plan, with version 0.11.0 published.
- Every check of the pipeline passed locally; of the browser tests, 911
  passed and 2 were skipped, which run only on the live site. The build
  of `f36597a` passed on GitHub; the deploy published version 0.11.0
  and the browser test on the public site passed (673 passed, 26
  skipped as designed for the live site).
- The three local servers of the trials were stopped.

## Open points

- The user feels something is not right with the home page, and that it
  may not be the height of the boxes: not named yet
  ([card](../done/2026-11-07_centre-first-box-on-home-page.md), "Open
  point").

## Next steps

- Choose the next card from Next: [plan](../plan.md).
- Open a card on the home page once what is not right can be named:
  [card](../done/2026-11-07_centre-first-box-on-home-page.md).
