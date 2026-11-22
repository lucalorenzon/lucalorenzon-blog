# ADR-006: A photo and a name for the logo, without the pixel font

- **Date:** 2026-11-22
- **Status:** Accepted. Supersedes the part of
  [ADR-003](ADR-003-readability-first-amber-as-accent.md) on the logo:
  the first item of its point 3, the alternative "Literata everywhere,
  logo included", and the pixel logo among its positive consequences.
- **Cards:** decided in [review the logo](../board/cards/review-the-logo.md);
  recorded, with the pixel font taken out of the site, in
  [remove the pixel font](../board/cards/remove-the-pixel-font.md)

## Context

On 2026-11-05, ADR-003 put readability first and kept identity in a few
signs around the text. One of them was the logo: written in the pixel
font of the IBM PC, IBM VGA 8x16, "the one sign of 1985 left in the
frame", a reminder of the bulletin board system at whose terminal
the author sat in 1985. On wide screens the logo was drawn in
characters, and a drawing made of characters needs a font whose
letters all have the same width; so ADR-003 rejected Literata, the
font of the text, for the logo.

The same day the site got a light background, and on it the drawn logo
became hard to read: its strokes are slashes and underscores, which on
paper-white stay separate lines instead of forming letters. A change of
colour did not help. The logo needed another form, and the card that
looked for it found that the reason for the pixel font had gone with
the drawing.

## Decision

1. **The logo is a photo and a name.** The icon is the favicon's photo
   of the author, cut to a circle; beside it the name "Lvk@73r" is
   written as text, in Literata bold italic, at every width.
2. **The pixel font leaves the site.** No role uses it any more, so its
   files, its description and its licence are taken out of the
   repository.
3. **The reference to 1985 is dropped for good**, by the author's
   choice.

Why: the photo is recognised at the sizes the header gives it, from 48
to 64 pixels, and reads the same on both backgrounds, while a drawing
in characters does not. With no drawing left, nothing needs a font of
equal widths in the header, and the name can be written in the text's
font. The author compared the pixel font, Literata bold, Literata
regular and Literata bold italic on screenshots of the real header,
and chose the italic: bold looked like a second title beside the
page's, regular looked thin beside the icon.

## Consequences

**Positive:**
- The logo reads on the light and on the dark background alike.
- One font fewer to serve and to keep with its licence.

**Negative / Risks:**
- The photo cannot follow the theme's colours, as the site's icons do:
  it is a fixed image, and it stays the same in both themes.
- The photo is of a person: it is published with the agreement of its
  author, under all rights reserved, out of the licences of the code
  and of the text (see the logo's card).
- The link to the author's beginnings in 1985, which ADR-003 kept in
  the logo, is gone from the site; it is a choice, not a loss to be
  made good later.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Keep the logo drawn in characters, in a darker shade on the light page | Tried in three shades of amber: the drawing stayed hard to read, since the problem is its strokes, not their colour |
| A black header on the light page, so the drawn logo keeps its background | Tried in three forms (the header, the header and the footer, the band above the line): rejected by the author |
| The name alone, in the pixel font, at every width | The author's leaning until 2026-11-22, since that form read well on both backgrounds; set aside when the idea of the photo came up, and the reference to 1985 was dropped with it |
| The name in Literata bold or regular beside the photo | Bold looked like a second title beside the page's; regular looked thin beside the icon |

## Technical Notes

- The icon and how it is made: `assets/logo/` and its `README.md`; the
  header that places it: `src/page_frame.rs` and `style/input.css`.
- The role `logo` in `config/fonts.txt` names Literata; the fonts page
  shows Literata with its role `logo` among the others.
- The header's browser test (`end2end/tests/header-layout.spec.ts`)
  fails if a logo drawn in characters or in the pixel font comes back.

## References

- Cards: [review the logo](../board/cards/review-the-logo.md),
  [remove the pixel font](../board/cards/remove-the-pixel-font.md)
- Related ADRs: [ADR-003](ADR-003-readability-first-amber-as-accent.md),
  partly superseded by this one
