# Measure the fonts again when they change

- **Opened:** 2026-11-16, by the user, cutting the card
  [link the fonts to the build](../done/2026-11-16_link-the-fonts-to-the-build.md). In
  Next, after the fonts page.

## Why

Some values of the build are measured on a font, and would not follow
a change of font made in the configuration:

- the table widths are estimated from Literata's character widths
  (`src/literata_widths.rs`), measured in Chromium by
  `end2end/tools/measure-literata-widths.mjs`;
- the wrap toggle of the code blocks shows from a threshold computed
  with Hack's advance, 1233/2048 of an em.

Comments of the stylesheet also name the fonts where they explain a
measure.

## Questions

1. Whether a change of the text font or of the code font should stop
   the build until its measures are taken again.
