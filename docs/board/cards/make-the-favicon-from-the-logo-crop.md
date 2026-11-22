# Make the favicon from the logo's crop

- **Created:** 2026-11-22
- **Status:** In work

## Why

Cut by the user on 2026-11-22 from the card
[review the logo](review-the-logo.md). The new logo is the photo of the
favicon cut to a circle, with the same framing, so that each recalls
the other. The favicon in the repository was made earlier, by another
crop, and is one image only: it may not match the logo's crop, and it
holds nothing for screens with dense pixels or for the places that ask
a site for larger icons.

## What is known

- **Today's favicon** is `assets/favicon.ico`, one image of 16 by 16
  pixels, a circular crop of the same photo (checked by the agent on
  2026-11-22 against the original).
- **The original photo is never committed**: it stays outside git
  (`.git/info/exclude`), and only images made from it enter the
  repository. Decided in the card [review the logo](review-the-logo.md).
- **The rights**: the favicon, like the logo, is out of every licence
  of the repository and the site, all rights reserved, with a notice
  that names the author of the photo. Decided in the logo's card, where
  the notice is written.
- **The logo's icon is sharpened** after reduction (unsharp mask,
  radius 1.2, 80 per cent, threshold 2, colour raised by 15 per cent
  and contrast by 8), since at small sizes the photo looked grainy.
- **The favicon is linked once**, by the frame shared by every page
  (`src/page_frame.rs`, one `<link rel="icon">` to `favicon.ico`), and
  the home page's browser test checks only that the file is served.
- **The icon file cannot carry the rights notice inside it**, unlike
  the logo's WebP files: the format has no place for metadata. The
  notice stays beside the files (`assets/logo/README.md`) and on the
  site's licence page, which already name the favicon.
- **The original photo was put back** in `assets/logo-original.jpg` by
  the user on 2026-11-22, for this card. The user had moved it to the
  trash earlier the same day, since a local build copied it into
  `target/site`; while it is in `assets/`, every local build copies it
  again.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which formats and sizes** the favicon is made in: the icon file,
  images for dense screens, an icon for phones' home screens.

  **Decided by the user on 2026-11-22.** This card remakes
  `favicon.ico` from the logo's crop, with two images in it, of 16 and
  32 pixels; the 32 serves screens of double density. The rest went to
  cards of their own (see "What it leaves").
- **Whether the same sharpening suits a favicon**, which is smaller
  than the logo.

  **Chosen by the user on 2026-11-22**, on screenshots of three
  variants made by the agent from the logo's circle, enlarged and in a
  mock browser tab at real size, light and dark, at normal and double
  density: the logo's recipe (A), a lighter one (radius 0.6, 50 per
  cent) and none. The user chose A, the logo's recipe: at 32 pixels the
  most defined, with a faint light rim along the circle's edge; in the
  tab at real size the three are hard to tell apart.

## Dependencies

- The card [review the logo](review-the-logo.md), which makes the
  crop and writes the rights notice.

## Acceptance tests

Agreed with the user on 2026-11-22.

1. `assets/favicon.ico` holds exactly two images, of 16 and 32 pixels,
   at 32 bits per pixel, so transparent outside the circle; a unit test
   reads the file's header and checks it.
2. Each image is made from the logo's square with the logo's recipe,
   each at its own size (the 16 is not the 32 reduced by the format);
   the favicon's recipe is written in `assets/logo/README.md`, beside
   the icons'.
3. Before the commit, the user sees the final file, opened as a
   favicon, beside the variant A they chose.
4. The site still serves `favicon.ico` (the home page's browser test),
   and every check of the pipeline passes.

## What it leaves

Cut by the user on 2026-11-22.

- The icons for phones' home screens, and images for dense screens
  beyond the icon file:
  [add icons for the home screen](add-icons-for-the-home-screen.md).
- Whether the script that makes the logo's and the favicon's images is
  kept in the repository:
  [decide whether to keep the logo images' script](decide-whether-to-keep-the-logo-images-script.md).

## References

- [Review the logo](review-the-logo.md): the card this one was cut
  from.
- [Add preview images](add-preview-images.md): the other images a page
  offers to the places that link to it.
- [Add icons for the home screen](add-icons-for-the-home-screen.md) and
  [decide whether to keep the logo images' script](decide-whether-to-keep-the-logo-images-script.md):
  cut from this card.
