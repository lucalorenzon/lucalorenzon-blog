# Make the favicon from the logo's crop

- **Created:** 2026-11-22
- **Status:** Open

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

## Questions to answer

The ones known when the card was written; there may be more.

- **Which formats and sizes** the favicon is made in: the icon file,
  images for dense screens, an icon for phones' home screens.
- **Whether the same sharpening suits a favicon**, which is smaller
  than the logo.

## Dependencies

- The card [review the logo](review-the-logo.md), which makes the
  crop and writes the rights notice.

## References

- [Review the logo](review-the-logo.md): the card this one was cut
  from.
- [Add preview images](add-preview-images.md): the other images a page
  offers to the places that link to it.
