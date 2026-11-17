# Session 2026-11-17: the code font's widths in the browsers

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: analysis, cards, measures in the
  browsers, review.
- **Two sub-agents**, Claude Sonnet 5.5: the first measured the code
  page for the cases at the limit and stopped on a finding; the second
  wrote the tool that makes the measuring page.

Goal: take up the card on measuring the fonts again when they change
([card](../wip/measure-the-fonts-again-when-they-change.md)). It led to
a fault of the wrap toggle, opened as a card of its own
([card](../wip/hide-the-wrap-toggle-on-blocks-that-fit.md)). Technical
details and reasons are in the cards.

## Decisions

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The card is taken into work and translated into the cards' new format, its "What is known" filled from what the agent checked in the code | The user | The agent | — |
| 2 | A change of font should break the build when the page comes out wrong, not because the font changed | The user | The agent | — |
| 3 | Estimates made too cautious count only when the result is hard to read: not for the record viewer; the wrap toggle on a block that fits keeps stopping the build | The user | The agent | — |
| 4 | The code font's measure is guarded by the outcome, two cases at the limit on the code page; a tool stopping the build when the measures differ is rejected | The user | The agent | — |
| 5 | The measures are taken from the font of each role, in the build, in the browser, as the script for Literata does | The user | The agent | — |
| 6 | The card is cut: it keeps the cases at the limit; the measures from the configured fonts and the stylesheet's comments go to a new card | The user | The agent | — |
| 7 | The new card goes at the top of Next | The agent | — | The user |
| 8 | The six acceptance tests of the cases at the limit | The user | The agent | — |
| 9 | The agent drafts the text of the two cases, reviewed by the user before the commit | The user | The agent | — |
| 10 | The toggle shown on a block that fits is a fault, handled as a fix in a card of its own; the card of the cases at the limit waits in work for it | The user | The agent | — |
| 11 | Before choosing the fix, measure the pipeline's Chromium and Firefox, then WebKit | The user | The agent | — |
| 12 | The measure on macOS goes to a card in Later | The user | The agent | — |
| 13 | The broken link in a finished card is mended, and the measuring page is kept in the repository | The user | The agent | — |
| 14 | The page is kept as a tool that writes it from the configuration, not as an HTML file with the font inside | The agent | — | The user |

## Discussion

1. **What a change of font should do.** The agent stated the problem:
   nothing ties the measured values to the configured font. The user
   answered that the build should break, then made it precise: it
   depends on the stylesheet and on the nature of the font, so what
   counts is the page. The agent found that the browser tests already
   check the outcome, but only on cases near a threshold. → decisions
   2, 3
2. **How to notice a wrong measure.** The agent proposed two ways, the
   outcome only or a guard on the measures too, and preferred the first
   since it follows the user's rule; the user agreed, and added that
   more precise measures, taken from the configured font, would be
   better. The agent asked which of two readings was meant; the user
   chose the font of each role, and said where to measure. → decisions
   4, 5
3. **The cut.** The agent pointed out that the card now held three
   pieces of work and proposed to keep the cases at the limit only; the
   user agreed. → decisions 6, 7
4. **The cases at the limit.** The agent proposed the acceptance tests;
   the user agreed and asked for a draft of the cases' text. The first
   sub-agent, measuring before writing, found that Chromium draws Hack
   narrower than the build assumes, so the agreed pair of cases does
   not exist, and stopped as instructed. The agent said its acceptance
   tests had taken the build's measure for granted, and read the
   finding as a fault; the user agreed. → decisions 8, 9, 10
5. **The browsers.** The agent measured: the pipeline's container
   (run by the user, Docker needing `sudo`) rounds as this machine
   does; Firefox draws the true width; `text-rendering:
   geometricPrecision` makes Chromium draw it too. The user asked for
   Safari; the agent explained that Playwright's WebKit on Linux is not
   Safari, that it is close to Epiphany, and that Docker on a Mac runs
   Linux; it measured WebKit, which draws the true width, and made a
   page to open on a Mac. The user mentioned BrowserStack, whose free
   terms the agent did not know for certain, then put the check in
   Later. → decisions 11, 12
6. **Before the minute.** The agent offered two pending steps; the
   user asked for both. → decisions 13, 14

## Outcome

- The card on measuring the fonts again taken into work, translated,
  cut, with its acceptance tests and now waiting for the fix
  (`ae33a0e`, `71f3cca`, `6a589f7`).
- New card in Next:
  [take the measures from the configured fonts](../backlog/take-the-measures-from-the-configured-fonts.md)
  (`71f3cca`).
- New card in work, the fault of the wrap toggle, with the measures of
  three browsers (`213c71e`, `e4dbe16`).
- New card in Later:
  [measure the code font on macOS](../backlog/measure-the-code-font-on-macos.md)
  (`e4dbe16`).
- A broken link mended in a finished card (`b8cd3a8`).
- The tool `end2end/tools/make-code-font-page.mjs`, which writes a page
  measuring the code font in any browser (`1f894fc`).

## Open points

- What `text-rendering: geometricPrecision` changes in the look of the
  code in Chromium on Linux, and whether it is the fix: not looked at
  yet.
- Whether Chromium's rounding to whole pixels also affects the table
  widths, estimated from Literata's widths measured at 1000 pixels:
  noticed by the agent, not looked at, and in no card.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Screenshots of the code with and without `geometricPrecision`, side
  by side, then the regression test and the fix: card
  [hide the wrap toggle on blocks that fit](../wip/hide-the-wrap-toggle-on-blocks-that-fit.md).
- Review acceptance tests 1 to 3, then write the cases at the limit:
  card [measure the fonts again when they change](../wip/measure-the-fonts-again-when-they-change.md).
