# Remove the pixel font

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

## Why

Cut by the user on 2026-11-22 from the card
[review the logo](review-the-logo.md). There the role `logo` moves from
the pixel font, IBM VGA 8x16, to Literata, so the pixel font plays no
role any more: the build stops declaring it in the stylesheet and
showing it on the fonts page by itself. Its files stay in the
repository, unused, and the decision record that gave it to the logo
still says it is there. The user dropped for good the reference to the
BBS of 1985 that the pixel font kept in the logo.

## What is known

Checked by the agent on 2026-11-22, before the logo's card is built:

- **Its files**: `assets/fonts/ibm-vga-8x16/` (the web font, its
  `LICENSE.TXT` and a `README.md` with the attribution), and its
  description `config/fonts/ibm-vga-8x16.txt`. `assets/` is copied as
  it is into the site, so the font would still be published, unused.
- **Unit tests that use it as a sample**: `src/style_guide/fonts_page.rs`
  and `src/style_guide/fonts.rs` name `ibm-vga-8x16` in their cases,
  some with the real descriptions in `config/fonts/`; they need another
  font, or a made-up one, before its description goes.
- **Comments that name it**: `style/input.css` (the header "keeps the
  pixel font", the title "not the pixel font of the header", Hack
  "wider than the pixel font was"), and `src/style_guide/fonts.rs` (the
  logo's pixel font uses `font-display: block`). The logo's card
  rewrote the first two; the other two were left, and noticed by its
  sub-agent on 2026-11-22.
- **The fonts' unit tests** that need a font with a native size use,
  since the logo's card, a made-up configuration with the pixel font
  (`pixel()` in the tests of `src/style_guide/`), which reads its real
  description: they need another font before the description goes.
- **ADR-003** (`docs/adr/ADR-003-readability-first-amber-as-accent.md`)
  calls the logo in the pixel font "the one sign of 1985 left in the
  frame", and rejects Literata for the logo because a logo drawn in
  characters needs a monospaced font. The photo answers that reason.
- `CHANGELOG.md` names the pixel font in versions already published;
  those entries stay as they are.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the native size stays in the build.** A font's description
  may give an optional `size`, its native size in pixels, which the
  build reads, checks and writes into the fonts page's samples
  (`src/style_guide/fonts.rs`). The pixel font was the only font with
  one: once it is gone, no configuration uses it, and only the tests
  would keep it alive with a made-up font. Raised by the agent on
  2026-11-22, when the card was taken into work.

  **Decided by the user on 2026-11-22.** It goes with the pixel font:
  the key `size` is no longer part of a font's description.
- **Whether `font-display: block` is still needed** by any font, once
  the only font that used it is gone.

  **Decided by the user on 2026-11-22.** The key `display` stays, with
  all five values of the CSS property (`auto`, `block`, `swap`,
  `fallback`, `optional`). Unlike the native size, the key is used by
  every font; only some of its values are unused, and the build only
  copies them into the stylesheet. The comment that justifies `block`
  with the pixel font is rewritten.
- **What ADR-003 says once the pixel font is gone**: a note on the
  record, or a new record that supersedes the part on the logo.

  **Decided by the user on 2026-11-22.** A new record, ADR-006,
  supersedes the part of ADR-003 on the logo; in ADR-003 only the
  status line changes, and the rest stays as the record of what was
  decided then. Why not a note, as for ADR-003's earlier corrections
  and extensions: this time a part of the decision is reversed (the
  logo kept the pixel font, and "Literata everywhere, logo included"
  was rejected), and the reason lies in the logo's card, not in a
  record.

## Dependencies

- The card [review the logo](review-the-logo.md), which takes the role
  `logo` away from the pixel font.

## Acceptance tests

Agreed with the user on 2026-11-22.

1. No file of the pixel font is left in the repository
   (`assets/fonts/ibm-vga-8x16/`, `config/fonts/ibm-vga-8x16.txt`), nor
   in the built site.
2. A font's description with the key `size` stops the build, with the
   error that it is not a key of a font's description; a unit test
   checks it.
3. The fonts' stylesheet the build writes has no `font-size` in the
   samples; the fonts page stays as it is today, since the pixel font,
   with no role, was no longer on it.
4. No unit test reads the pixel font's description; those that need a
   second font use a made-up one, written in the test.
5. No comment and no rule of the code names the pixel font, except the
   guard of the header's browser test against the old logo, which
   stays.
6. ADR-006 exists and records the decision on the logo; ADR-003 says in
   its status line that this part of it is superseded.
7. Every check of the pipeline passes: formatting, clippy, unit tests,
   type check, browser tests and the check of the browser tests'
   README.

## Recap

Done on 2026-11-22: the card taken into work with its acceptance tests
(`96c4d90`), ADR-006 written (`462ae6c`), and the pixel font removed in
the commit that closes this card.

- **The pixel font's files are gone**: `assets/fonts/ibm-vga-8x16/`
  (the web font, its licence and its attribution) and its description
  `config/fonts/ibm-vga-8x16.txt`. The built site holds no file of it.
- **The native size is gone** from `src/style_guide/fonts.rs`: the
  field, its reading and its check, and the `font-size` and
  `max-width` it wrote into the fonts page's samples. A description
  with `size` now stops the build as any unknown key does, and a unit
  test checks it; another checks that no sample sets a size or a
  width. The repository map in `CLAUDE.md` no longer lists the key.
- **The key `display` keeps its five values**; the comment that
  justified `block` with the pixel font now says the build copies the
  property's values as they are.
- **The fonts' unit tests** that read the pixel font's description use
  a made-up font, `tiny`, written in the tests, for the third role; the
  fonts page's test of a font with two roles uses Hack, for the code
  and the logo. Two checks that a font with no role is not shown were
  taken out: the real configuration no longer describes a font without
  a role, so they could not fail.
- **The comment in `style/input.css`** that compared Hack with the
  pixel font lost that comparison.
- **ADR-006** records the logo as a photo and a name, and supersedes
  ADR-003's part on the logo; ADR-003 says so in its status line.
- Every acceptance test passed: no file and no mention of the pixel
  font is left outside the records, the changelog and the header's
  guard; formatting, clippy, the 432 unit tests, the build, the type
  check and the browser tests (331 passed, 1 skipped, the "page not
  found" check that runs on the live site only) pass.

## Cautions

- **The build does not empty `target/site`** before writing it: on a
  machine that built the site before this card, the pixel font's
  folder stays in `target/site/fonts/` until the folder is removed by
  hand. The pipeline builds from scratch, so the published site is not
  affected.
- **The header's browser test still looks for the pixel font** (a font
  family with "VGA" or "pixel" in the header), on purpose: it guards
  against the old logo coming back, and its rows in
  `end2end/README.md` stay as they are.
- **No font is described without a role** any more: a font added to
  `config/fonts/` and given no role is not tested by the real
  configuration, only by the made-up ones in the unit tests.

## References

- [Review the logo](review-the-logo.md): the card this one was cut
  from.
