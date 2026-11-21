# Link the fonts to the build

- **Opened:** 2026-11-16, by the user, closing the card
  [show the fonts in the style guide](show-the-fonts-in-the-style-guide.md).
  In Next; in Now from 2026-11-16.
- **Status:** done, 2026-11-16 (`f2640ca`, `eb44029`).
- **Cut:** 2026-11-16, by the user. The stylesheet is made parametric
  first, in the card
  [give the fonts three roles in the stylesheet](give-the-fonts-three-roles-in-the-stylesheet.md);
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

## Answers

1. **Only the roles in the roles' file.** `config/fonts.txt` says which
   font plays each role, `text`, `code` and `logo`, and nothing else.
   Chosen by the user: a change of font is a change of one name.
2. **Each font described in a file of its own, in `config/`.** The
   user first meant the description to sit beside the font's files in
   `assets/fonts/`, then kept it in `config/`, with the rest of the
   configuration; `assets/` is copied into the published site as it
   is, `config/` is not.
3. **Plain text, for now.** The user chose `.txt` files over a
   structured format: the card
   [use a structured configuration file](use-a-structured-configuration-file.md)
   will review how every configuration file is organised.
4. **A font is named by its folder.** The description of a font is
   `config/fonts/<folder>.txt`, `<folder>` being the font's folder in
   `assets/fonts/`; the roles' file names the font the same way; the
   family name the stylesheet uses, with its spaces and capitals, is
   written inside the description.
5. **The pixel font gets a folder of its own**, like the other two, as
   the card's first slice, before the configuration: the user's choice,
   for uniformity, so the configuration finds three fonts of the same
   shape.
6. **The description's form**, proposed by the user: one `key=value`
   per line; keys without a prefix for the whole font (family name,
   fallback, `font-display`), keys with a prefix for one file, the
   prefix being a free label (`italic.file`, `italic.name`,
   `italic.style`, `italic.weight`). Style and weight are written, not
   inferred from the prefix, so a new weight needs no change to the
   build. `.name` is the label the fonts page shows under the sample,
   for the card
   [build the fonts page from the configuration](build-the-fonts-page-from-the-configuration.md).

   ```
   family=Literata
   fallback=serif
   display=swap
   regular.file=literata-latin-400-normal.woff2
   regular.name=Regular
   regular.style=normal
   regular.weight=400
   italic.file=literata-latin-400-italic.woff2
   italic.name=Italic
   italic.style=italic
   italic.weight=400
   ...
   ```

## Acceptance tests

Agreed with the user on 2026-11-16.

**First slice, the pixel font's folder**

1. The pixel font, with its README and licence, is in
   `assets/fonts/ibm-vga-8x16/`; `assets/fonts/` holds only folders.
2. The logo still takes the pixel font (the font roles' browser test
   checks it), the font file is served at its new address, and the
   documentation's references to the old path are updated.

**Second slice, the configuration**

3. `config/fonts.txt` holds three lines in the descriptions' own
   `key=value` form (`text=literata`); each font has its description in
   `config/fonts/<folder>.txt`, in the agreed form.
4. The build writes the `@font-face` rules and the three roles to
   `target/generated/fonts.css`, which `style/input.css` imports; the
   hand-written stylesheet names no font any more, and the unit test of
   the card
   [give the fonts three roles in the stylesheet](give-the-fonts-three-roles-in-the-stylesheet.md)
   is narrowed accordingly: no font name anywhere in `style/input.css`.
5. The build stops, with a message naming the file and the line, when a
   role is missing, repeated or unknown; a role names a font with no
   description; a description lacks a required key, has an unknown or
   repeated key, or a line not in the `key=value` form; a style, a
   weight or a `font-display` has a value not allowed; a described file
   does not exist in the font's folder. Each case has its unit test.
6. A unit test shows that changing a role's font in the configuration
   changes the role's value in the generated CSS (with `text=hack`,
   `--font-text: "Hack", monospace`).
7. Nothing visible changes: the computed font of every element of every
   page of the sitemap is the same before and after, and screenshots on
   a small phone and a laptop.
8. Every check of `build.yml` passes; the browser tests' README and the
   repository map in `CLAUDE.md` (`config/`, the new generated file) are
   up to date.

## Results

All eight acceptance tests pass.

**First slice (`f2640ca`).** The pixel font is in
`assets/fonts/ibm-vga-8x16/`. No test checked that the font files
load: the role tests compare family names, which stay the same when a
file is missing. A browser test now loads every font face the
stylesheet declares and fails on any that does not load; broken on
purpose, it failed on the pixel font. Screenshots of the home page
byte-identical before and after.

**Second slice (`eb44029`).** The build reads `config/fonts.txt` and
`config/fonts/`, refuses a configuration that does not hold together
(each refusal with its unit test, with a few more than agreed: two
files of one font with the same style and weight, names that could
point outside `assets/fonts/`, a family name with a quote) and writes
`target/generated/fonts.css`. Two things learned on the way:

- `--font-*: initial` clears only what was defined before it, so it
  moved into the generated `@theme`, before the roles: left in
  `style/input.css`, after the import, it wiped them.
- Tailwind reads every word of `src/`: a first error message with the
  word "lowercase" became a stray utility in the stylesheet, and was
  reworded. The card
  [let Tailwind read only the presentation](let-tailwind-read-only-the-presentation.md)
  is about this.

Checks of `build.yml`: 392 unit tests; 270 browser tests passed, one
skipped (the "page not found" test that runs on the live site only).

**Acceptance test 7, second slice.** The computed font
family, style and weight of every element of every page of the sitemap
(12 pages, 3638 elements, Chromium at 1366x768, light scheme), captured
before and after the configuration: no difference. Screenshots of the
home page, an article, the style guide's fonts page and its code blocks
page, at 320x568 and 1366x768 (full page): the eight pairs are
byte-identical. The only change in the built `style.css` is that the
pixel font's `@font-face` rule now says `font-style: normal` and
`font-weight: 400`, which are the defaults it had before.
