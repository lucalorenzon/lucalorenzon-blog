# Add icons for the home screen

- **Created:** 2026-11-22
- **Status:** Open

## Why

Cut by the user on 2026-11-22 from the card
[make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md),
which kept only the icon file of 16 and 32 pixels. A reader who adds
the site to a phone's home screen gets today a small image enlarged, or
a letter drawn by the phone, instead of the logo.

## What is known

- **What phones ask for** (the agent's knowledge, not checked against
  current documentation): an iPhone looks for an `apple-touch-icon` of
  180 pixels; Android takes its icons from a web manifest, a file of
  its own linked from each page, usually with images of 192 and 512
  pixels.
- **The frame shared by every page** (`src/page_frame.rs`) links the
  favicon today; more icons mean more links there, the unit tests of
  each page that check the favicon's link, and the browser tests'
  README.
- **The images come from the original photo**, never committed, with
  the logo's recipe (`assets/logo/README.md`); at these sizes the
  sharpening may need its own trial, as it had for the favicon.
- **The rights**: these images, like the logo and the favicon, are
  under all rights reserved.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which icons and which sizes** the site offers, and whether a web
  manifest is worth it for a site that is not an application.
- **Whether PNG images for dense screens** are needed besides the icon
  file, for the browsers' tabs.

## Dependencies

- The original photo, which lives only on the user's machine.

## References

- [Make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md):
  the card this one was cut from.
- [Decide whether to keep the logo images' script](decide-whether-to-keep-the-logo-images-script.md):
  the way these images would be made.
