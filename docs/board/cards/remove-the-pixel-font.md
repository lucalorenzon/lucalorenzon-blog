# Remove the pixel font

- **Created:** 2026-11-22
- **Status:** Open

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

- **What ADR-003 says once the pixel font is gone**: a note on the
  record, or a new record that supersedes the part on the logo.
- **Whether `font-display: block` is still needed** by any font, once
  the only font that used it is gone.

## Dependencies

- The card [review the logo](review-the-logo.md), which takes the role
  `logo` away from the pixel font.

## References

- [Review the logo](review-the-logo.md): the card this one was cut
  from.
