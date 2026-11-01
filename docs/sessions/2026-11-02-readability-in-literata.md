# Session 2026-11-01/02 — The text set in Literata, version 0.5.0

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Sonnet 5.5 (the playground page, the stylesheet, font
and tests) and Claude Haiku 5.5 (card moves, builds and test runs, one
commit).
**Goal:** start the card on the WCAG criteria axe cannot check; it led
to the card
[improve the site's readability](../done/2026-11-02_improve-site-readability.md),
carried out and published in version 0.5.0. Technical details and
reasons are in that card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The WCAG card covers the whole of level AAA; readability's two criteria, 1.4.8 and 1.4.12, come first | user | agent | — |
| 2 | Readability stays one decision with three parameters (font and size, line spacing, colour); the planned split is withdrawn | user | agent | — |
| 3 | The checks for 1.4.8 and 1.4.12 belong to the readability card and land with the choice; the WCAG card comes after it | user | agent | — |
| 4 | The result is judged by comparing complete combinations | user | agent | — |
| 5 | Earlier decisions, the old-terminal look included, are open to question; the combination is searched in a playground page with the criteria as limits | user | agent | — |
| 6 | Fonts tried beyond the first four: Olivetti Thin, two serif fonts, then proportional ones | user | agent | — |
| 7 | Which ones: Courier Prime and Literata; IBM Plex Sans, IBM Plex Serif, Atkinson Hyperlegible Next | agent | — | user |
| 8 | A control for the column width, since with a proportional font 80ch holds more than 80 characters | user | agent | — |
| 9 | The combination: Literata, header in IBM VGA 8x16, line height 1.5, 36px between paragraphs, 12px between list items, 60ch column, `#DD9911` on black | user | agent | — |
| 10 | Literata shows a fallback font until it arrives (`font-display: swap`); the header keeps waiting for its own | agent | — | user |
| 11 | Literata's weight accepted: about 62 KiB per visit | user | agent | — |
| 12 | Colours chosen by the reader and text enlarged to 200% (the rest of 1.4.8) go back to the WCAG card | user | agent | — |
| 13 | Publish now, as version 0.5.0 | user | agent | — |
| 14 | Close the readability card; the empty space on wide screens is a new card | user | agent | — |
| 15 | The new card in Next | agent | — | user |
| 16 | Three cards: performance in the build (Next), badges in the README (Later), CO2 of a page view (Later) | user | agent | — |

## Discussion

1. **Cutting the WCAG card.** The agent proposed three cards (1.4.12,
   1.4.8, the other criteria). The user rejected it with the header's
   precedent: font, line spacing and colour are one compromise, and
   splitting means fixing one, adjusting another, then changing the
   first again. The agent agreed, noted that the same argument withdrew
   the split planned the day before, and that the two criteria measure
   that same choice. → decisions 1, 2
2. **When the checks come.** The agent set out the tension: checks
   written first keep the build red, since the page did not meet 1.4.8,
   blocking every publication. The user put them with the choice; the
   agent mentioned, after the answer, a test marked as an expected
   failure. → decisions 3, 4
3. **The old-terminal look.** Asked about the font, the user said the
   letters of old terminals were never really readable. The agent asked
   whether the look was still a goal; the user asked not to get stuck on
   earlier decisions and proposed a playground like the header's, with
   commands to change the look and the criteria as limits. → decision 5
4. **Fonts.** The user found int10h.org, the source of today's font, and
   asked for Olivetti and a serif font, believing serifs read better.
   The agent said the screen studies it knows show no clear difference,
   and one on dyslexia favours fonts without serifs, but that the eye
   decides. Literata won; the agent noticed it differs in two ways,
   serifs and proportional spacing, and the user asked for proportional
   fonts to tell them apart: proportional spacing made most of the
   difference. No proportional Olivetti font with a usable licence was
   found. → decisions 6, 7
5. **Line width.** Counting characters on the rendered lines, the agent
   found 94 at 80ch with Literata. Columns of 67ch reached exactly 80;
   the agent pointed out a width tuned to today's text leaves no room
   for the next one, and the user chose 60ch (72 characters). The agent
   asked to check 1.4.12 again at that width: all green. → decisions 8, 9
6. **Implementation.** A sub-agent wrote the stylesheet, the font with
   its licence and the test, and showed the test failing with the old
   line height and the old column. The agent put back the section
   headings' own line height, removed by the sub-agent but kept in the
   playground, and corrected the changelog's "60 characters". The user
   asked what Literata weighs. → decisions 10, 11
7. **The rest of 1.4.8.** The agent noticed that two parts of 1.4.8 had
   no test and no card. → decision 12
8. **Publication.** The agent asked for the version number, since a tag
   is never reused; the user chose 0.5.0, the domain being in it too.
   → decision 13
9. **Wide screens.** Reading the live site, the user found the text much
   more readable, but the narrow column wastes wide screens. The agent
   asked whether the empty space or small text weighed: the empty
   space. It proposed a separate card, on the layout. → decisions 14, 15
10. **Performance.** The user asked for Lighthouse or similar in the
    tests, since more is coming onto the site, the results as badges,
    and a CO2 measure later. The agent made three cards, one decision
    each. → decision 16

## Outcome

- `37ed8a8`: readability's criteria moved into the readability card;
  the WCAG card after it.
- `6cd04b3`: the readability card started (sub-agent).
- `62065f3`: the combination chosen in the playground recorded in the
  card.
- `0f2a49d`: Literata, spacing, column and colour in the stylesheet;
  the readability test for 1.4.8 and 1.4.12 (sub-agents).
- `ad9479a`: the rest of 1.4.8 back to the WCAG card.
- `e2cefc6`: version 0.5.0 in the changelog; tag `v0.5.0` pushed,
  published, live test passed.
- `1bf1783`: the readability card closed; the card
  [use the space beside the text on wide screens](../backlog/use-space-beside-text-on-wide-screens.md)
  opened (sub-agent).
- `3332398`: the cards
  [check performance in the build](../backlog/check-performance-in-the-build.md),
  [show the check results as badges in the README](../backlog/show-check-results-as-readme-badges.md)
  and [measure the CO2 of a page view](../backlog/measure-co2-of-a-page-view.md).
- The playground page stayed outside the repository, as the header's
  did.

## Open points

- The card planned a reading on a phone as well as on the live site;
  the user reported the live site. Whether the phone reading was done is
  not recorded.

## Next steps

- Serve the site's own "page not found": card
  [serve the site's own "page not found"](../backlog/serve-own-not-found-page.md).
- Go through the WCAG criteria not covered, levels A to AAA, the two
  parts of 1.4.8 included: card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md).
- Fill the space beside the text on wide screens: card
  [use the space beside the text on wide screens](../backlog/use-space-beside-text-on-wide-screens.md).
- Check performance in the build: card
  [check performance in the build](../backlog/check-performance-in-the-build.md).
