# Cover the WCAG criteria axe cannot check

- **Opened:** 2026-10-30, asked by the user while the card
  [run a screen reader check on demand](../done/2026-10-30_run-screen-reader-check-on-demand.md)
  was being worked on.
- **Status:** noticed, not started; in Now since 2026-11-01. Widened
  to level AAA on 2026-11-01.

## Why

The build runs every axe rule tagged for WCAG levels A and AA, plus the
three AAA rules axe can check
([card](../done/2026-10-30_check-accessibility-in-the-build.md)). That
is only part of the WCAG criteria: many of them, such as keyboard use,
focus order, zoom to 400% or text that makes sense, can only be judged
by a person. The user's aim is the WCAG criteria themselves, not a tool's
list of rules.

Some criteria are already covered beyond axe: the accessibility tree
test (structure, names, heading levels,
[card](../done/2026-10-30_check-accessibility-tree-in-the-build.md)) and
the on-demand screen reader check (how the page sounds, the language of
its parts). Others are not covered at all.

axe also has rules marked "best practice", off in the build because
they are not WCAG criteria (one main region, all content inside
regions, one level-1 heading, heading order). They help with some
criteria, such as info and relationships (1.3.1), but are a means, not
the goal.

### Widened to level AAA

On 2026-11-01 the user found the published curriculum hard to read
(card [improve the site's readability](improve-site-readability.md)).
The criterion that speaks to it most, 1.4.8 visual presentation (line
spacing, width of the text, space between paragraphs), is level AAA,
outside this card as first written; axe has no rule for it. The user
asked to widen the card to level AAA and to do it before the
readability card, so that card is judged against criteria the build or
a written check covers.

The criteria the readability card needs first: 1.4.8 (AAA) and 1.4.12
text spacing (AA). Other AAA criteria about reading, such as unusual
words (3.1.3), abbreviations (3.1.4) and reading level (3.1.5), concern
the content more than the look.

## Questions

- Which WCAG 2.2 criteria of levels A, AA and AAA are not covered by the
  build or by the screen reader check, criterion by criterion. The
  W3C's method for evaluating a site, WCAG-EM, is a possible guide.
- How each uncovered criterion is checked: automatically (for example
  axe's best-practice rules, or a test written for it), on demand like
  the screen reader check, or by hand with a written result.
- Whether AAA criteria that cannot be met, or that a personal site
  does not aim at, are listed with the reason rather than checked.
- When: the page today has no links, forms or keyboard targets, so
  several criteria have nothing to apply to yet.
