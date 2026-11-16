# Show the fonts in the style guide

- **Opened:** 2026-11-15, by the user, cutting the typography page of
  the card [build a design system](../wip/build-a-design-system.md) in
  two: the elements of the text come first
  ([card](2026-11-16_show-the-text-elements-in-the-style-guide.md)),
  the fonts follow. In Now.
- **Status:** done, 2026-11-16. Not published yet: the style guide goes
  live when it is complete.

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

## Acceptance tests

Agreed with the user on 2026-11-16. One slice: the page, its texts and
its tests.

1. The build writes the style guide's fonts page,
   `style-guide/fonts.html`, titled "Fonts", linked from the index and
   listed in the sitemap, with its heading, description and canonical
   address: unit tests, as for the other pages of the style guide.
2. The page holds an introduction and one section for each font,
   Literata, Hack and IBM VGA 8x16, each with its explanation followed
   by its samples: Literata regular, italic, bold and bold italic;
   Hack regular; IBM VGA 8x16 regular. Six samples, each named by its
   style: unit test of the page.
3. Every sample is drawn with the stylesheet's classes (`font-sans`,
   `font-code`, `font-mono`, with italic and bold) and never names a
   font, so a change to the stylesheet shows in the samples too: unit
   test. Corrected at closing: the page uses named variants only
   ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md)), so
   the samples use six variants, `sample-literata` to `sample-pixel`,
   each built from those classes; the agent had written the test
   without the ADR in mind.
4. Every sample shows the same English text, written by the agent.
   Literata and Hack at the size of the page's text; IBM VGA 8x16 at
   16 px, its native size and the drawn logo's, since a pixel font
   blurs at other sizes.
5. The explanations are the user's, translated by the agent and
   approved by the user in the chat before the commit.
6. No browser test is added: the page is covered by the net. The
   browser tests' README does not change, and `readme.spec.ts` passes.
7. The checks of `build.yml` pass: format, lint, unit tests, type
   check, browser tests.
8. Before the commit, the user has seen the page on a small phone and
   on a laptop.

## Results

- **The page** (commit `54d4f2f`): `style-guide/fonts.html`, its text
  in `content/style-guide/fonts.md`, its samples placed by the build
  after each font's section (`src/style_guide/fonts_page.rs`, on the
  model of the colours page), six named variants in
  `style/input.css`.
- **The pixel font's sample is no wider than 80 characters:** at
  16 px, on a laptop, a line held 88, and the net's check of long
  lines failed; the text did not change.
- **The labels sit below their samples**, as captions: above, they
  read as captions of the explanation before them. A screen reader
  reads the sample before its label.
- **The checks of `build.yml`**, run locally on 2026-11-16: format,
  lint, 361 unit tests, type check, 256 browser tests passed, one
  skipped (the "page not found" test that runs on the live site only).
- **Seen by the user** on a small phone and a laptop, light and dark,
  before the commit.

## Answers

All answered by the user on 2026-11-16.

- **What the page shows** (question 1): a sample for each style the
  site serves, six in all, with the user's explanation of each font:
  Literata chosen for its readability; Hack, the lightest font the
  user found to show code in the style of Meslo LG, its close
  relative, Meslo LG itself being left out because of its licence; IBM
  VGA 8x16, the first choice, for the look of the old BBS, a wrong
  choice left for readability, kept today by the logo only and going
  when the logo becomes an image. Rejected: a sample for each element
  of the page that uses a font (heading, paragraph, code, logo), which
  would repeat the text elements' page.
- **The samples' text:** plain English, neutral and without a message,
  written by the agent, the same in every sample so that only the font
  changes. Rejected: Lorem ipsum, which a screen reader reads with the
  page's English voice as meaningless sounds; text from
  nietzsche-ipsum.com, whose source has no licence and whose word list
  can put words together that read, on the author's site, as a
  statement.
- **No tests about the fonts** (question 2): the page is only to be
  shown. The page's own unit tests stay, as for every page of the
  style guide.
- **Not noted in the backlog:** three things no test protects, found
  while opening the card: characters outside the subsets, which file
  is actually loaded, inline code inside bold or italic text, which
  the browser would thicken or slant from Hack's regular. Hypothetical
  today, none of them a real problem; if inline code reads badly one
  day, the text is changed where it happens.
- **The page and the stylesheet:** the samples use the stylesheet's
  classes, so a change of font in the stylesheet shows on the page;
  the names and the explanations are written by hand. Defining the
  fonts once in the build, as the colours are, so that a change made
  for the style guide is a change of the whole site, is left to a card
  of its own, opened when this one closed
  ([card](../wip/link-the-fonts-to-the-build.md)): otherwise the style guide
  loses its purpose. Its known bond: the table widths are estimated
  from Literata's character widths, measured by a tool of their own
  (`src/literata_widths.rs`), which a change of the text font would
  need measured again.
