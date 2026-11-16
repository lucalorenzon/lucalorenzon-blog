# Build the fonts page from the configuration

- **Created:** 2026-11-16
- **Status:** In work

## Why

The user opened this card on 2026-11-16, while cutting the card
[link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md).

The fonts page of the style guide writes by hand the fonts it shows
(`FONTS_SHOWN` in `src/style_guide/fonts_page.rs`): their names, the
styles shown and the variant of each sample, whose names carry the
font's (`sample-literata`, `sample-hack`, `sample-pixel`). The styles
the site serves are written elsewhere, in the `@font-face` rules: the
two lists can drift, and a style added to the site would have no
sample without anything noticing. Once the configuration holds the
fonts, the page can take from it the roles, the fonts and their styles.

The user wants this card done before the style guide is published.

## What is known

- **The author's text** (`content/style-guide/fonts.md`) has one
  section per font, titled with its name, and the build stops when a
  section and a font do not match; it explains why each font was
  chosen, so a change of font also asks for a change of the text.
- **The configuration already holds what the samples need**, checked
  on 2026-11-16: `config/fonts.txt` says which font plays each role,
  and each `config/fonts/<folder>.txt` gives the font's family name and,
  for each file, the label the page shows under the sample (`.name`),
  its style and its weight. The label was added for this card.
- **The build already reads the configuration** (`src/style_guide/fonts.rs`,
  called by `write_fonts` in `src/main.rs`) and writes the roles' values
  and the `@font-face` rules to `target/generated/fonts.css`. The fonts
  page does not read it yet.
- **Each sample is a named variant of the stylesheet**
  (`style/input.css`, `sample-literata` to `sample-pixel`), written out
  in full in the Rust code because Tailwind finds the classes it must
  emit by scanning the sources
  ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md)). Each
  variant takes the font from its role (`font-text`, `font-code`,
  `font-logo`), not from a font's name.
- **The pixel font is drawn at 16 pixels**, its native size, where it
  does not blur, and no wider than 80 characters; the other two keep
  the size of the page's text. The configuration holds no size.

## Questions to answer

The ones known when the card was taken into work; there may be more.

- **How a sample gets its style when the styles come from the
  configuration.** Today one variant of the stylesheet exists for each
  sample, named after the font. If the configuration adds a style, or a
  role changes font, a variant named after the old font would be wrong,
  and a new style would have no variant; Tailwind cannot emit a class it
  does not find written in the sources.
- **Where the pixel font's size lives.** It is a property of that font,
  not of the logo role, yet the configuration does not hold it; if the
  logo changed font, the 16 pixels would stay.
- **What happens when two roles use the same font.** The configuration
  does not forbid it, and the page has one section per font.
- **Whether the page shows which role each font plays.** Today the
  sections are titled with the font's name only, and their order is the
  author's.

## What it leaves

- The values measured on a font (table widths, the wrap toggle's
  threshold), taken again when it changes: card
  [measure the fonts again when they change](../backlog/measure-the-fonts-again-when-they-change.md),
  next in the plan.

## References

- [Link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md):
  the card this one was cut from; it put the fonts in the configuration.
- [Show the fonts in the style guide](../done/2026-11-16_show-the-fonts-in-the-style-guide.md):
  the card that built the fonts page as it is today.
- [Measure the fonts again when they change](../backlog/measure-the-fonts-again-when-they-change.md):
  cut from the same card, done after this one.
- [Build a design system](../wip/build-a-design-system.md): the card in
  work this one belongs to; the style guide is published after it.
