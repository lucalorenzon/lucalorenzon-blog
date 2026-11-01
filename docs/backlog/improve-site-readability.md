# Improve the site's readability

- **Opened:** 2026-11-01, by the user, reading the published site. It
  joins two cards opened the same day, "space out the lines of text"
  and "make the text colours easier to read", and adds the font.
- **Status:** open, in Now, after the card
  [cover the WCAG criteria axe cannot check](cover-wcag-criteria-axe-cannot-check.md).

## Why

Reading the curriculum on the live site, the user found the text hard
to read: the lines sit too close together, and amber on black does not
help. While the line spacing was being discussed, the user asked to
consider the font too.

The three questions depend on one another, so they were joined in one
card:

- **The font sets the rules for the line spacing.** Sizes and spacings
  are multiples of 8px only because IBM VGA 8x16 is a pixel font; with
  another font that rule goes, and the right line spacing depends on the
  new font's proportions.
- **Font and colour together make the old-terminal look** chosen on
  2026-10-28
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md)):
  changing either one decides how much of that look is left.

A card holds one decision, so when this one is picked up it is split
again, probably in the order the dependencies suggest: font, then line
spacing, then colour.

It comes after the card on the WCAG criteria axe cannot check, which
the user asked to widen to level AAA, so readability is judged against
criteria the build or a written check covers, not against impressions
only.

## What the stylesheet does today

In `style/input.css`:

- **Line height equals font size:** 16px on 16px, a ratio of 1.0, set
  on `html, body`. Paragraphs and list items are 8px apart (`mt-2`);
  lines inside a paragraph have no space between them.
- **Amber `#FFB000` on pure black `#000000`,** everywhere, lines
  included.
- **IBM VGA 8x16** for all text, served by the site itself.

## Why the build did not catch it

The build checks accessibility with axe, and no axe rule looks at what
the user noticed:

- **Criterion 1.4.8** (level AAA, visual presentation) asks for line
  spacing of at least 1.5 inside paragraphs, and space between
  paragraphs at least 1.5 times that. axe 4.14.0 has no rule for it.
  The build runs only the three AAA rules axe can check (enhanced
  contrast, links with the same text, no automatic refresh); the card
  [check accessibility in the build](../done/2026-10-30_check-accessibility-in-the-build.md)
  already says the site is not AAA, since most AAA criteria can only be
  judged by a person.
- **Criterion 1.4.12** (level AA, text spacing) does not ask the author
  for a wide line height: it asks that the page still works when the
  reader raises line height and spacing on their own. axe's only rule
  for it, `avoid-inline-spacing`, looks only at spacing written in an
  element's `style` attribute; the page has none, and the stylesheet is
  not examined.
- **Colours:** the only thing WCAG measures is contrast, and amber on
  black passes it at 11.46:1, above the 7:1 of level AAA. A bright
  saturated colour on pure black can still glare ("halation"), more so
  for readers with astigmatism; no WCAG criterion covers it.

## Known from the start

- **The header keeps its own line heights:** the title (24px or 32px),
  the text logo (32px) and the ASCII-art logo (1.0, which its drawing
  needs) set them in the stylesheet, so a change on `html, body` does
  not reach them. It reaches the text column, the roles in inverse
  video and the footer.
- **The footer is sticky:** a taller line makes it take more of a small
  phone's screen (320 by 568 pixels is the smallest tested).
- **Criterion 1.4.8 can be read two ways** for the space between
  paragraphs: as the empty space between them, or as the distance from
  the last line of one to the first line of the next. With a 24px line
  height that is 40px or 16px; today it is 8px.
- **Every colour is bound to 7:1** on normal text by the build's
  accessibility check.
- **History of the colour.** On the previous attempt of the site the
  user had ruled out colours other than white for accessibility
  reasons; amber was a deliberate try.
- **History of the font.** Rejected on 2026-10-28: the system's
  monospace font (different on every device), fonts loaded from Google
  Fonts (fines in Europe over visitors' privacy), Meslo LG and VT323
  (not the 1980s home-computer look).
- **The content layout test** already checks, at 12 screen sizes, that
  consecutive blocks do not overlap; a measure of the line height fits
  there.

## Answers already given

### Which text gets the new line spacing?

**The whole page (chosen by the user on 2026-11-01),** not only the
text column: the footer and any future page that does not use the
column start from the same line spacing. The header is not affected,
as above.

## Questions to answer

- **The font:** whether IBM VGA 8x16 stays, and if not, which font, and
  how much of the old-terminal look is left.
- **How much line height,** depending on the font, and whether the
  space between paragraphs and list items changes with it.
- **What to do with the colour:** softer values of amber and black,
  amber kept only for lines and headings, or other colours.
- **How the build catches it next time:** a test that measures the
  line height in the text column (criterion 1.4.8); a test that raises
  line height and spacing as a reader would and checks that no text
  overlaps or is cut off (criterion 1.4.12). The card on the WCAG
  criteria may already decide this.
- **How the result is judged** beyond the tests: the user's reading,
  on the live site and on a phone, one change at a time so the effects
  are not confused. Images of the page with each option, side by side,
  can help before deciding.
