# Session 2026-11-05 — Tables, light background and licence note

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (code and tests of each slice).
**Goal:** the slices left in the card
[give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md),
starting from tables. Reasons and rejected options are in that card
and in [ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Tables with rows in stripes | user | agent | — |
| 2 | Tables: amber lines above, under the header, at the bottom and between columns, not thinner or dimmer | user | agent | — |
| 3 | A slice for the licence note at the end of an article | user | agent | — |
| 4 | Follow the browser's preference at once, so most readers get the light page | user | agent | — |
| 5 | Light page: paper-white, beige grey surface, amber turned towards red (32°) | user | agent | — |
| 6 | The browser tests run on both backgrounds | agent | Sonnet | user |
| 7 | The accessibility test drops contrast findings on the drawn logo only | user | agent, Sonnet | — |
| 8 | The drawn logo stays as it is in the light-background slice; a card of its own, in Now | user | agent | — |
| 9 | The Markdown conversion recognises the closing note | user | agent | — |
| 10 | The note is a `<footer>`, its line a top border | agent | — | user |
| 11 | The note at 14/16 pixels, 3.5 lines of space and an amber line above | user | agent | — |
| 12 | The note 60ch wide in its own font, for the 80-character rule | user | agent, Sonnet | — |
| 13 | The browser tests find the page footer outside the text column | agent | Sonnet | user |

## Discussion

1. **Stripes.** The agent proposed three sets of lines for tables; the
   user added stripes on alternate rows before the trials. The agent
   found the lighter stripe capped by the amber code in a cell, which
   falls under 7:1 above the code surface grey. → decision 1
2. **Lines.** The user chose three lines with lines between columns,
   then asked for those thinner or dimmer; the agent showed half a
   pixel and two dimmer ambers, warning that half a pixel shows only
   on dense screens. The user kept full amber. → decision 2
3. **The note.** Before the light background the user asked for a
   slice on the licence note: a white line, text too large, no space
   above. → decision 3
4. **Who gets the light page.** The agent reported that a stylesheet
   cannot tell no preference from light, so the real choice was to
   follow the preference now or postpone the light page; the user
   chose now. → decision 4
5. **Light shades.** The agent found the ADR's shades held only on the
   page, not on the code surface, and showed darker ones; the user
   found them brown and asked for a redder amber; the agent computed
   two hues and two surfaces, and the user chose. → decision 5
6. **Tests and logo.** Sonnet stopped on axe flagging the drawn logo
   on the light page; the agent proposed relying on WCAG's exemption
   for logos, and the user agreed. → decisions 6, 7
7. **The logo read badly.** Seeing the page, the user found the drawn
   logo illegible on white; darker shades did not help, and the user
   saw it was the drawing, not the colour. The user proposed a black
   header, then only above the line, rejected both, and leaned towards
   the written logo everywhere; asked how to record it, the user
   committed the light background as it was and opened a card.
   → decision 8
8. **Recognising the note.** The agent asked how the page knows the
   note is a note and proposed the conversion over a stylesheet rule;
   the user agreed. → decisions 9, 10
9. **Size and width.** The user chose the smallest trial; Sonnet
   stopped on lines of 85 characters at that size, and the agent
   proposed narrowing the note over enlarging it. → decisions 11, 12,
   13

## Outcome

- Tables with amber lines and striped rows; a light background when
  the browser prefers light pages, colours now tokens by role; the
  licence note set apart at the end of an article. 95 unit tests and
  440 browser tests pass (Chromium, dark and light).
- Commits: `45d67fa` tables, `7201a2b` slice for the note,
  `7d5a7fb` light background, `e87a6d9` card on the logo, `7f20322`
  licence note, plus this minute.

## Open points

- The commits of this session are not pushed.
- The look card closes once the user validates the article pages in
  the browser, both backgrounds; the user was away from the laptop.
- The two backgrounds have accents of slightly different hue: whether
  the dark page's amber turns red too is not decided.

## Next steps

- Validate the article pages and close the card
  [give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md).
- Decide the logo, likely the written logo everywhere. Card
  [review the logo](../backlog/review-the-logo.md).
- What a table too wide for a phone does, and the hybris PK article
  back in the checks. Card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md).
