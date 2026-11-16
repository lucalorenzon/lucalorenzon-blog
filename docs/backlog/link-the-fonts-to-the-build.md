# Link the fonts to the build

- **Opened:** 2026-11-16, by the user, closing the card
  [show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md).
  In Next.
- **Cut:** 2026-11-16, by the user. The stylesheet is made parametric
  first, in the card
  [give the fonts three roles in the stylesheet](../wip/give-the-fonts-three-roles-in-the-stylesheet.md);
  this card keeps the build. The fonts page and the measures on a font
  follow in cards of their own
  ([fonts page](build-the-fonts-page-from-the-configuration.md),
  [measures](measure-the-fonts-again-when-they-change.md)).

## Why

The style guide's fonts page draws its samples with the stylesheet's
font classes, so a change of font in the stylesheet shows there; but
the fonts themselves are written by hand in `style/input.css`
(`@font-face` rules and the `@theme` variables), and the page's names
are written by hand too. A change of font made for the style guide is
not a change of the whole site, and the page could name a font the
site no longer uses. In the user's words, otherwise the style guide
loses its purpose. The colours already work the other way: defined
once in `src/style_guide/colours.rs`, from which the build writes both
the colours page and the stylesheet's colours.

## Known from the start

- **The three roles:** after the stylesheet's card, the theme has three
  fonts, `font-text`, `font-code` and `font-logo`, and nothing else
  names a font but the `@theme` block and the `@font-face` rules. This
  card moves their values into a configuration file, read by the build,
  which writes both (the user's choice: the stylesheet cannot read a
  configuration file by itself).
- **The model to follow:** the colours, defined once in the build,
  which writes `target/generated/colours.css` for the stylesheet.
- **What the fonts are made of** (`style/input.css`, `assets/fonts/`):
  three fonts, six files, each with its style, weight and
  `font-display`.
- **A configuration file already planned:** the card
  [use a structured configuration file](use-a-structured-configuration-file.md)
  waits for a second setting shared by the build and the tests; the
  fonts may be that setting, and a structured one (several files per
  font).
- **IBM VGA 8x16 is going:** the user means to replace the logo with an
  image, the font's only use today; the logo's role may go with it.

## Questions

1. What the configuration holds for each role, and in which file.
