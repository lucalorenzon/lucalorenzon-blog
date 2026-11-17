# Give the fonts three roles in the stylesheet

- **Opened:** 2026-11-16, by the user, cutting the card
  [link the fonts to the build](2026-11-16_link-the-fonts-to-the-build.md):
  the stylesheet is made parametric first, the build and the fonts
  page follow. In Now.
- **Status:** done, 2026-11-16 (`efb5e57`).

## Why

A change of font made for the style guide must be a change of the whole
site. Before this card it was not, even inside the stylesheet: the running text of
every page (paragraphs, lists, tables) takes its font from
`html, body`, where `"Literata", serif` is written by hand
(`style/input.css`), while only the page titles and the fonts page's
samples use `font-sans`. Changing `--font-sans` would change the
samples and the titles, and leave the text in Literata: the style guide
would show a font the site does not use.

## Answers

1. **Three fonts, named by role.** The theme defines `font-text`,
   `font-code` and `font-logo`, today Literata, Hack and IBM VGA 8x16;
   everything else (the stylesheet, later the build and the fonts page)
   refers to the three roles, never to a font's name. Chosen by the
   user: a font is what it does in the site.
2. **The stylesheet first, alone.** The values are written in the
   `@theme` block and in the `@font-face` rules, the only places that
   name a font. A configuration file comes later: the stylesheet cannot
   read one, so it arrives with the build that writes the CSS from it
   (the card [link the fonts to the build](2026-11-16_link-the-fonts-to-the-build.md)).
3. **The order of what follows**, chosen by the user: the build reads
   the fonts from a configuration file; then the fonts page is built
   from the same configuration
   ([card](../done/2026-11-17_build-the-fonts-page-from-the-configuration.md));
   then what is measured on a font
   ([card](../backlog/measure-the-fonts-again-when-they-change.md)).

## Known from the start

- **Tailwind's own fonts:** `font-sans`, `font-serif` and `font-mono`
  exist unless the theme clears them with `--font-*: initial`, as it
  already does for the breakpoints. Cleared, no rule can use a font
  outside the three roles by mistake.
- **Tailwind's default fonts:** its base layer draws `html` with
  `--default-font-family` and `code`, `kbd`, `samp` and `pre` with
  `--default-mono-font-family`, which point today to `--font-sans` and
  `--font-mono`. So the elements of code fall back to the pixel font
  wherever no rule of ours sets `font-code`. With the roles they point
  to `--font-text` and `--font-code`.
- **The variants' names** of the fonts page (`sample-literata`,
  `sample-hack`, `sample-pixel`) still carry the fonts' names: they
  change with the fonts page's card, not here.
- **A CSS limit:** a `@font-face` rule accepts no variable, neither in
  its family name nor in its file; the name stays in each rule of its
  font until the build writes them.
- **A mistaken report, corrected:** the sub-agent that made the change
  reported that Tailwind's default fonts were not defined before it;
  the stylesheet built from the previous commit shows they were, as
  written above. Nothing on the pages depended on them (result 1).

## Acceptance tests

Agreed with the user on 2026-11-16.

1. The theme defines exactly three fonts, `font-text`, `font-code` and
   `font-logo`; the built stylesheet has no `font-sans`, `font-serif`
   or `font-mono`, neither as variables nor as classes.
2. A browser test checks that each role reaches its elements, comparing
   the font the browser computes with the variable's value: the running
   text (paragraphs, lists, tables), the page titles and the Literata
   samples take `font-text`; the code blocks, the code inside a line
   and the Hack sample take `font-code`; the logo and the pixel sample
   take `font-logo`.
3. Outside the `@theme` block and the `@font-face` rules,
   `style/input.css` names no font; an automatic test checks it.
4. Nothing visible changes: on every page of the sitemap, every element
   has the same computed font before and after the change, checked once
   and reported here; and screenshots on a small phone and a laptop.
5. Every check of `build.yml` passes, and the browser tests' README is
   up to date.

## Results

All five acceptance tests pass.

1. **Acceptance test 4, computed fonts:** on the 12 pages of the sitemap
  (3 638 elements, at 1366x768, light theme), `font-family`, `font-style`
  and `font-weight` of every element were captured before and after the
  change: 0 differences, not even in the family strings. Screenshots of the
  home page, an article and the style guide's fonts and code pages, at
  320x568 and 1366x768, were taken before and after: identical, byte
  for byte.
2. **Checks of `build.yml`:** 363 unit tests; 269 browser tests passed,
   one skipped (the "page not found" test that runs on the live site
   only).
