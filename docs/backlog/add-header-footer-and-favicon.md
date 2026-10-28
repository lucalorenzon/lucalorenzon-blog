# Give the page a header, a footer and a favicon, in a dark terminal style

- **Opened:** 2026-10-28, from the light pass of `residuality`
  ([card](../done/2026-10-28_challenge-skeleton-with-residuality.md),
  stressor 7).
- **Status:** noticed, not started. The next card in the plan.

## Why

The site exists to tie its content to its author. Today a visitor sees
only "Sito in costruzione": the page could belong to anyone. Before any
article, the page has to say whose site it is.

The audience is mostly technical people who appreciate simplicity that
does not become trivial. The style chosen by the user, dark like an old
terminal, tells where the author comes from (a photo of them aged twelve,
in 1985, in front of a terminal in a server room; the photo explains the
style, it is not part of this card).

## What it holds

One slice, as decided by the user: the style is a stylesheet and the
favicon an existing file, so they are not separate decisions.

- **Header** with the ASCII-art logo.
- **Footer** with a copyright line, for example "© Luca Lorenzon 2026".
- **Dark, minimal style** like an old terminal.
- **Favicon.**

Existing material on the branch `old_main`, to reuse:

- favicon: `favicon.ico` and `assets/favicon.ico`;
- logo: `src/components/logos/logo.rs`, with two ASCII-art versions of
  "Lvk@73r" (a large one and a narrower one) and a plain-text version
  used on small screens; also `assets/logoIcon.png` and
  `assets/logoIconSmall.png`;
- header and footer: `src/components/headers/header.rs`,
  `src/components/footers/footer.rs`. The old header also had a search
  icon and a menu, which are not part of this card.

Publishing it changes what a visitor sees, so it makes the next version;
that version also carries the card
[verify the publication at the next version](verify-publication-at-next-version.md).

## Questions to settle when it starts

- Which ASCII-art version, and what a phone shows: on a narrow screen the
  wide logo does not fit.
- Whether "Sito in costruzione" stays between header and footer.
- The old components referred to files by root-absolute paths
  (`/assets/...`). The site is served under a sub-path, and a unit test
  forbids root-absolute references: the favicon and any image need
  relative paths.
- The logo is drawn with characters: a screen reader would read them one
  by one, so the logo needs a text alternative.
- The year in the footer is fixed at build time, so it stays the year of
  the last published version.
