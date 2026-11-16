# Show the fonts in the style guide

- **Opened:** 2026-11-15, by the user, cutting the typography page of
  the card [build a design system](build-a-design-system.md) in
  two: the elements of the text come first
  ([card](../done/2026-11-16_show-the-text-elements-in-the-style-guide.md)),
  the fonts follow. In Now.
- **Status:** being worked on, since 2026-11-16.

## Why

The style guide is published only when it is complete, and the
typography is one of the pages it needs. The fonts are mostly to be
shown: which font the site uses for what, with its weights and styles,
so that a change of font, one day, is seen in one place.

## Known from the start

- **The fonts the site serves** (`assets/fonts/`, each with its
  licence and attribution in a `README.md` of its own; the rules in
  `style/input.css`):
  - **Literata**, the text and the page titles: four files, regular,
    italic, bold and bold italic, in Fontsource's latin subset;
    `font-display: swap`, so the text is readable at once in a
    fallback font. Tailwind's `font-sans`.
  - **Hack**, the code, in blocks and inside a line: one file, regular
    only, in the Hack project's own web subset of 386 characters;
    `font-display: swap`. Tailwind's `font-code`.
  - **IBM VGA 8x16**, a pixel font, the header only (the logo written
    in characters); `font-display: block`, since a fallback would
    flash. Tailwind's `font-mono`.
- **Hack has no bold and no italic:** inline code inside bold or
  italic text, a level-2 or level-3 heading or a table's header, would
  be drawn by the browser with a thickened or slanted regular. The
  content has none today (a search of the Markdown, 2026-11-16).
- **"Every character of the site's text is inside it"**, say the
  Literata and Hack notes of their subsets; nothing checks it, so a
  character outside a subset would be drawn in a fallback font
  unnoticed.
- **The widths of the tables depend on the fonts:** the build
  estimates a table's width from Literata's regular and bold
  character widths (`src/literata_widths.rs`, measured in Chromium by
  `end2end/tools/`) and from Hack's.
- **The only test about fonts today** is `loads stylesheet, fonts and
  favicon` (`home.spec.ts`), on the home page: a `.woff` and a
  `.woff2` file are served. Any one `.woff2` satisfies it, so it does
  not tell which of Literata's four files or Hack's was asked for.
  The text checks moved with the elements of the text.
- **What it touches, roughly:** a new page of the style guide (its
  generator in `src/style_guide/`, its text in
  `content/style-guide/`), listed by the index and the sitemap; tests
  depending on the answers below.

## Questions

1. What the page shows of each font: names, weights, styles, the
   characters, the licence.
2. Which tests, if any, belong to the fonts.
