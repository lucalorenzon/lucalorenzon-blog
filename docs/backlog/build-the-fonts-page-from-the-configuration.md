# Build the fonts page from the configuration

- **Opened:** 2026-11-16, by the user, cutting the card
  [link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md). In
  Next, after it.

## Why

The fonts page of the style guide writes by hand the fonts it shows
(`FONTS_SHOWN` in `src/style_guide/fonts_page.rs`): their names, the
styles shown and the variant of each sample, whose names carry the
font's (`sample-literata`, `sample-hack`, `sample-pixel`). The styles
the site serves are written elsewhere, in the `@font-face` rules: the
two lists can drift, and a style added to the site would have no
sample without anything noticing. Once the configuration holds the
fonts, the page can take from it the roles, the fonts and their styles.

## Known from the start

- **The author's text** (`content/style-guide/fonts.md`) has one
  section per font, titled with its name, and the build stops when a
  section and a font do not match; it explains why each font was
  chosen, so a change of font also asks for a change of the text.
