# Frame each article on the home page in a box

- **Opened:** 2026-11-07, by the user, looking at the home page after
  version 0.10.0; in Now.
- **Status:** being worked on.

## Why

The home page lists the articles, newest first: for each one the title
as a heading, the description and the date
([card](../done/2026-11-06_list-articles-on-home-page.md)). The items
are kept apart only by the space between them. The user proposed
framing each item in a box, its border in the amber accent already used
for the lines of the page.

## Known from the start

- **The colour is already decided.** ADR-003 keeps amber for the lines
  and frames of the page; a box is a frame, so its border takes the
  tone of lines and frames, `accent-line` (`#dd9911` on the dark page,
  `#c06d0f` on the light page, 3.7:1 on paper-white, above the 3:1
  asked of graphics). The ADR lists the frame of code blocks among the
  uses of amber, not frames in general: whether the boxes join that
  list is to be recorded in the ADR when this card is done.
- **What the list touches today:** the component in `src/home_page.rs`
  (an `ul` with the class `article-list`, one `li` per article), the
  `article-list` style in `style/input.css`, the unit tests in
  `src/home_page.rs` and the browser test `end2end/tests/home.spec.ts`.
  The readability test checks spacing between paragraphs, which the
  padding of a box may affect.
- **How the result is judged:** on the real page, at phone and laptop
  width, on the light and the dark background, with alternatives side
  by side before a commit; and with a layout test on a range of
  devices (no overflow, no overlap, text legible), not just the rule.

## Questions

- How the box looks: border width, corners square or rounded, inner
  space, space between boxes.
- What the box is for a reader: only a frame around the item, or a
  link as a whole, the entire box leading to the article.
- Whether the date and the description keep their place inside the box
  or move (for example, the date on the border line).
- Whether the box reacts to the pointer or to the keyboard focus, and
  how that relates to the focus outline, which is amber too.
