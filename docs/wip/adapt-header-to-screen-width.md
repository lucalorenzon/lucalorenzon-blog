# Adapt the header to screen width

- **Opened:** 2026-10-29, split from the card "Make the site responsive"
  (opened 2026-10-28 while the card
  [give the page a header, a footer and a favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  was being worked on). Its other half is
  [adapt the footer and the articles to screen width](../backlog/adapt-footer-and-articles-to-screen-width.md).
- **Status:** being worked on since 2026-10-29.

## Why

The header shows the narrow ASCII-art logo, the title "Home Page" and an
empty column, on every screen. The logo is 42 characters wide, 336px in
the IBM VGA 8x16 font at 16px; on a 375px-wide phone the title next to it
wraps onto two lines, squeezed against the right edge. The user accepted
this for the first version and asked for the header to adapt to screen
size in a card of its own.

A navigation menu will come later
([card](../backlog/add-site-navigation.md)) and will need room in the header too, so
the layout chosen here must leave space for it without being reworked.

## Decisions

### What the header shows at each width

| Width | Header | Page title |
|---|---|---|
| 1024px and wider (Tailwind's standard `lg` threshold) | ASCII-art logo, title, menu (when it exists) | in the header, centred |
| Narrower than 1024px | the text "Lvk@73r", menu (when it exists) | in the body of the page |

- **Why:** the logo and the menu are the same on every page, the title
  belongs to the content. On small screens moving the title into the
  page leaves the header with only the site's identity and its actions,
  so its width no longer depends on the page.
- **Why 1024px:** the header grid keeps the title centred with two side
  columns of equal width, so it needs twice the widest side element plus
  the title: 2 × 336 + 144 = 816px with the ASCII-art logo. The first
  standard Tailwind threshold above that is 1024px. The menu will go in
  the right-hand column, as wide as the logo's, so it does not change
  this.
- **Rejected:** the old site (branch `old_main`) switched from text to
  ASCII art at 768px, with its own redefined thresholds (480, 768, 976,
  1440). At 768px the ASCII-art logo and a centred title do not fit.
  It also drew the ASCII art at 7pt, which made it hard to read.
- **Rejected:** three states, ASCII art → text → no logo, with the title
  always in the header. On the smallest screens the title in the browser
  tab, the favicon and the footer are enough to recognise the site, so
  hiding the logo was acceptable; but the text logo and the menu icons
  fit even on the narrowest phone once the title moves into the page, so
  the third state is not needed.

### The title appears once

The page has a single title element; the style moves it into the header
or into the body depending on the width. Duplicating it and hiding one
copy would make a screen reader or a page read without its style show
it twice.

### How the browser test checks it

The browser test checks the page at two widths: at 1280px the ASCII-art
logo is visible and the title is inside the header; at 375px the text
"Lvk@73r" is visible, the ASCII art is hidden and the title is outside
the header.
