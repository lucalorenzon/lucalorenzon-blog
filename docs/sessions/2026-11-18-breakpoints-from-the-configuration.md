# Session 2026-11-18: the breakpoints, read from the configuration

## Participants and goal

- **The user**, as Product Owner.
- **The agent**, Claude Opus 5.5: analysis, cards, baseline and trial,
  review.
- **One sub-agent**, Claude Sonnet 5.5: wrote the code and the tests,
  then three rounds of fixes asked by the agent.

Goal: start the breakpoints, the last page of the style guide before
it is published (card [build a design system](../wip/build-a-design-system.md)).
The session cut them in two cards and finished the first
([card](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md)).
Technical details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The breakpoints' page both explains how the site adapts to the width and is the only place of the breakpoints | The user | The agent | — |
| 2 | The values live in one place, read by the stylesheet, the page and the tests; what changes at each breakpoint stays in the components | The user | The agent | — |
| 3 | The style guide is published with the breakpoints' page as its last; the pages of components still missing are declared as pages to come | The user | The agent | — |
| 4 | The values go in a text file in `config/`, in the fonts' format, not in Rust and not after the structured configuration file | The user | The agent | — |
| 5 | Two cards: the values in the configuration first, then the page | The user | The agent | — |
| 6 | The breakpoints are in pixels: they say which devices the site addresses | The user | The agent | — |
| 7 | The tests take from the file only the two sides of each breakpoint; the devices stay a list of their own, and the page will show which devices fall in each group | The user | The agent | — |
| 8 | The file follows the fonts' rules, and the Rust checks of the classes take their prefixes from it | The user | The agent | — |
| 9 | The seven acceptance tests of the first card | The user | The agent | — |
| 10 | The side at a breakpoint also comes from the file; a device at that exact width stands for it | The agent | — | The user |
| 11 | The launch on LinkedIn and the marketing campaigns become two cards in Later | The agent | — | The user |

## Discussion

1. **What the page is for.** The agent showed the two breakpoints,
   what changes at each and that their widths are written in three
   places, and asked what the page should do. The user wanted both
   an explanation for the reader and the only place of the
   breakpoints, and noticed that a breakpoint needs its changes listed
   too, leading to the other pages. The agent named two tensions: the
   changes are the components' own classes, and most of them, in the
   header and the footer, have no page to lead to. → decisions 1, 2
2. **More pages before publishing?** The user asked whether the
   title, logo, header, footer, text column, list of articles and
   article's box should each have a page. The agent named the
   decision to publish only when complete, the logo's review still
   open, and the overlap between header, title and logo. The user was
   torn between publishing soon and fearing a guide with gaps; the
   agent gave its view, that declared gaps are a work in progress and
   only publishing tells which details readers value, with two ways.
   The user chose to publish as planned, adding that the real launch
   comes later, on LinkedIn, with more articles and the visitors
   observed. → decisions 3, 11
3. **Where and how the values are written.** The agent set Rust, as
   the colours, against `config/`, as the fonts, since the tests must
   read them; the user chose `config/`, then the plain text file over
   waiting for a structured format. On the unit, the agent showed
   that rem in a media query follows the browser's default text size,
   which the site's text ignores; the user chose pixels, because
   breakpoints address devices, and the agent dropped its own idea of
   returning to rem one day, which contradicted that reason. The user
   noted that the breakpoints sort the devices into groups, and that
   the smallest screen the user checks is the later iPhone SE, not
   the tests' 320. → decisions 4, 6, 7, 8
4. **Cutting and agreeing.** The agent described the two pieces, one
   structural and wide, one a new page; the user cut them in two
   cards, values first, and agreed the acceptance tests the agent
   proposed. → decisions 5, 9
5. **Verification.** The agent kept the site built before the change
   and compared it with the sub-agent's: identical HTML, but a stray
   utility and two variables in the stylesheet, and the side at each
   breakpoint still taken from devices by name, which the agent's own
   brief had asked for. The agent said so and sent the fixes; the
   trial with `md` moved to 800 found one unit test tied to the real
   values, also fixed. → decision 10

## Outcome

- The two cards cut, the plan and the design system's card linked
  (`f8647a6`).
- The unit, the rules and the acceptance tests agreed; the note on
  pixels in the card on letting readers adjust the text (`70ff33a`).
- The breakpoints read from `config/breakpoints.txt` by the build,
  the Rust checks and the browser tests (`0ae5d00`).
- The first card closed (`9fe06dd`).
- The cards on the launch and on marketing campaigns opened, in Later
  (`f9bfb93`).

Nothing was pushed.

## Open points

- The tables page's text still names 767 and 768; if `md` moves, it
  becomes false unnoticed (card
  [read the breakpoints from the configuration](../done/2026-11-18_read-the-breakpoints-from-the-configuration.md),
  "Cautions").

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The breakpoints' page, last before the style guide is published:
  card [show the breakpoints in the style guide](../done/2026-11-21_show-the-breakpoints-in-the-style-guide.md).
- The real launch: card [launch the site on LinkedIn](../backlog/launch-the-site-on-linkedin.md).
- Learning about campaigns and banners: card
  [plan marketing campaigns](../backlog/plan-marketing-campaigns.md).
