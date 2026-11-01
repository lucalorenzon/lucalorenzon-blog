# Session 2026-11-01 — Cards on the site's readability

**Participants:** the user (author); the agent (Claude Opus 5.5); one
sub-agent (Claude Haiku 5.5), which moved a card to `docs/wip/`.
**Goal:** open a card to make the published text easier to read; it
became the card
[improve the site's readability](../backlog/improve-site-readability.md),
with the card
[cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md)
put before it.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Open two cards, line spacing and text colours, both in Now | user | agent | — |
| 2 | Record in the cards why the build's accessibility check did not catch the problem | user | agent | — |
| 3 | Start with the line spacing card | user | — | agent |
| 4 | The new line spacing applies to the whole page, not only the text column | user | agent | — |
| 5 | Join font, line spacing and colours in one readability card, to be split again when picked up | user | agent | — |
| 6 | Widen the WCAG card to level AAA and do it before the readability card, in Now | user | agent | — |
| 7 | The widening covers the whole AAA level, with the criteria the readability card needs named first | agent | — | user |

## Discussion

1. **The text is hard to read.** The user, reading the live site, found
   the lines too close together and amber on black unhelpful. The agent
   found the line height equal to the font size (1.0) and recalled that
   the contrast passes at 11.46:1; it proposed two cards, one decision
   each, which the user accepted. → decision 1
2. **Why the tests did not see it.** The user asked why the
   accessibility rules had not caught it. The agent checked axe 4.14.0:
   no rule for criterion 1.4.8 (AAA), and the only rule for 1.4.12 (AA)
   looks at inline styles, which the page does not have; glare is not a
   WCAG criterion. It corrected its own earlier wording, which had
   suggested the build checked these criteria. → decision 2
3. **Which text.** The agent set out that the header already has its own
   line heights and that the sticky footer would grow on small phones;
   the user chose the whole page. → decisions 3 and 4
4. **The font.** Asked to choose between 24px and 32px, the user raised
   changing the font. The agent pointed out that the font sets the rules
   for the line spacing and, with the colour, makes the old-terminal
   look, and proposed a separate font card first. The user saw the three
   cards depending on one another and proposed one general card, after
   the WCAG card. The agent noted that a joined card will need splitting
   again, and that the WCAG card stopped at level AA while 1.4.8 is AAA.
   → decisions 5 and 6
5. **How far AAA.** The user said to widen the card to AAA; the agent
   read it as the whole level. → decision 7

## Outcome

- `9169c28`: the two cards, line spacing and text colours, in Now.
- `2816940`: the line spacing card moved to `docs/wip/` (sub-agent).
- `4aa88e2`: the two cards joined into
  [improve the site's readability](../backlog/improve-site-readability.md),
  font included; the earlier cards removed. A pull with automatic stash
  had left the removals and the plan out of the commit; the agent saw it
  and amended the commit before pushing.
- `e3c44fa`: the WCAG card widened to level AAA and put in Now.
- Not pushed: these four commits and this minute.

## Open points

- Whether the WCAG card covers the whole AAA level, as written, or only
  the criteria about readability (decision 7): to be confirmed by the
  user.
- How criterion 1.4.8's "space between paragraphs" is read: as the empty
  space between paragraphs, or as the distance between their lines.
  Recorded in the readability card.

## Next steps

- Go through the WCAG criteria not covered, levels A to AAA: card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md).
- Split and carry out the readability work, font first: card
  [improve the site's readability](../backlog/improve-site-readability.md).
