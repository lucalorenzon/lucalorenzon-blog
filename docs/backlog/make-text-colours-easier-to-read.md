# Make the text colours easier to read

- **Opened:** 2026-11-01, by the user, reading the published site.
- **Status:** open, in Now.

## Why

Reading the curriculum on the live site, the user found the text hard
to read: besides the lines sitting too close together (card
[space out the lines of text](../wip/space-out-lines-of-text.md)), amber on
black does not help.

What the stylesheet does today (`style/input.css`): amber `#FFB000` on
pure black `#000000`, everywhere, lines included, chosen on 2026-10-28
for the old-terminal look
([card](../done/2026-10-28_add-header-footer-and-favicon.md)).

## Why the build did not catch it

The only thing WCAG measures about colours is contrast, and amber on
black passes it: 11.46:1, above the 7:1 of level AAA, checked by axe at
every build
([card](../done/2026-10-30_check-accessibility-in-the-build.md)). A
bright saturated colour on pure black can still glare ("halation"),
more so for readers with astigmatism; no WCAG criterion covers it, and
no automatic test can say whether a text reads well.

## Known from the start

- **Every colour is bound to 7:1** on normal text by the build's
  accessibility check.
- **History of the colour.** On the previous attempt of the site the
  user had ruled out colours other than white for accessibility
  reasons; amber was a deliberate try.
- **The font may play a part** on a long text (the curriculum is about
  1,800 words): IBM VGA 8x16 is a 1980s screen font. The user did not
  raise it.

## Questions to answer

- **What to do with the colour:** softer values of amber and black,
  amber kept only for lines and headings, or other colours; and what is
  left of the old-terminal look.
- **Whether the font is part of this card** or a separate one.
- **How the result is judged:** the user's reading, on the live site
  and on a phone, after the line spacing has changed, so the two
  effects are not confused.
