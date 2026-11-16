# Link the fonts to the build

- **Opened:** 2026-11-16, by the user, closing the card
  [show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md).
  In Next.

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

- **The model to follow:** the colours, defined once in the build,
  which writes `target/generated/colours.css` for the stylesheet and
  the samples of the colours page.
- **What the fonts are made of** (`style/input.css`, `assets/fonts/`):
  three fonts, six files, each with its style, weight and
  `font-display`, and the three `@theme` variables `font-sans`,
  `font-code`, `font-mono`; the six named variants of the samples.
- **A bond that would not follow by itself:** the table widths are
  estimated from Literata's character widths (`src/literata_widths.rs`),
  measured in Chromium by a tool in `end2end/tools/`; a change of the
  text font would need them measured again.
- **IBM VGA 8x16 is going:** the user means to replace the logo with an
  image, the font's only use today; it may not need a definition of
  its own.

## Questions

1. What the definition holds for each font, and what the build writes
   from it.
2. Whether a change of the text font should stop the build until
   Literata's widths are measured again for the new font.
