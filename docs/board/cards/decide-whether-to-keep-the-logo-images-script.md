# Decide whether to keep the logo images' script

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md),
which received the question from the card
[review the logo](review-the-logo.md). The logo's icons and the
favicon are made by throwaway scripts kept in `target/`, which a clean
build wipes; only the recipe is written, in prose, in
`assets/logo/README.md`. Every new size, or a change of the recipe,
means writing the script again.

## What is known

- **The original photo is never committed**, since the repository is
  public: a script in the repository could only be run on the machine
  that holds the photo.
- **The recipe** is in `assets/logo/README.md`: the square of 760
  pixels at 0 from the left and 110 from the top, cut to a circle,
  reduced with Lanczos, sharpened, colour and contrast raised, saved
  as WebP with the rights notice inside, or as an icon file.
- **The tools**: Pillow 12.1.1 on the user's machine, the same as the
  scripts that made the images on 2026-11-22.

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether a script in the repository is worth it**, when it cannot
  run in the pipeline and the images change rarely.
- **Where it would live and how it is tested**, if kept: the other
  Python steps in `scripts/` have unit tests and pass the user's
  editor checks.

## References

- [Make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md):
  the card this one was cut from.
- [Review the logo](review-the-logo.md): where the question was first
  asked.
- [Add icons for the home screen](add-icons-for-the-home-screen.md):
  more images that the script would make.
