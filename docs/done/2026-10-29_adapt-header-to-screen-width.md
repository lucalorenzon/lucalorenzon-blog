# Adapt the header to screen width

- **Opened:** 2026-10-29, split from the card "Make the site responsive"
  (opened 2026-10-28 while the card
  [give the page a header, a footer and a favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  was being worked on). Its other half is
  [adapt the footer and the articles to screen width](../backlog/adapt-footer-and-articles-to-screen-width.md).
- **Status:** done on 2026-10-29.

## Why

The header showed the narrow ASCII-art logo, the title "Home Page" and an
empty column, on every screen. The logo is 42 characters wide, 336px in
the IBM VGA 8x16 font at 16px; on a 375px-wide phone the title next to it
wrapped onto two lines, squeezed against the right edge. The user accepted
this for the first version and asked for the header to adapt to screen
size in a card of its own.

A navigation menu will come later
([card](../backlog/add-site-navigation.md)) and will need room in the
header too, so the layout must leave space for it without being reworked.

## What the header shows at each width

| Width | Logo | Title |
|---|---|---|
| Below 768px (Tailwind's `md`) | the text "Lvk@73r", 32px | under the header line, 24px |
| 768px to 1279px | ASCII art, 16px | under the header line, 32px |
| 1280px and wider (Tailwind's `xl`) | ASCII art, 16px | beside the logo, 32px, at most two lines |

At every width: the line sits between the logo and the title; 24px of
space under the line; at least 24px between the title and the logo or
the menu; 8px of margin on both sides; a slot 56px wide on the right is
reserved for the menu (two 24px icons and the space between them).
Titles are at most 50 characters.

These values were chosen by the user by looking at them, not computed:
a playground page showed the header in ten device frames at once, from
a 320px phone to a Full HD screen, with controls for each value and
red marks for overflow, overlaps and gaps that were too small.

### Why the title moves under the line on smaller screens

The logo and the menu are the same on every page; the title belongs to
the content and changes length. Under the line, a long title wraps
freely without pushing the logo or the menu. Beside the logo it needs
room: at 1280px a 50-character title at 32px fits in two lines, at
1024px it would take three.

- **Rejected:** title beside the logo from 1024px. Measured with
  50-character titles, it took three lines on an iPad in landscape.
- **Rejected:** the title always in the header row, with the logo
  disappearing on phones (a first idea of the user's). Once the title
  can move under the line, the logo always fits.

### Why the ASCII art from 768px

Without the title beside it, the ASCII art only needs its own 336px plus
the menu, so tablets in portrait can show it. The old site (branch
`old_main`) switched at 768px too, but its title was elsewhere and it
drew the ASCII art at 7pt, too small to read.

### Why the title stays centred only while there is room

At 1280px a title centred on the page has the same room on both sides,
but the menu only uses 56px of the right side, so the title wrapped
early. Centred between the logo and the menu it fits one line, but a
short title like "Home Page" looks off-centre on a wide screen. The
layout keeps the title centred on the page while there is room, and
shifts it toward the menu only as much as needed to fit. This is done by
the grid's column sizes alone, with no calculation.

- **Rejected:** always centred on the page (wraps early at 1280px);
  always centred between logo and menu (off-centre on wide screens).

### Why the line is between the logo and the title

Two versions were compared side by side: the line under the title
(the title inside the header) and the line between the logo and the
title. The user preferred the second.

### Sizes

- Title 24px on phones: at 32px a 50-character title took three lines,
  close to 100px before the content, and a 21-character word such as
  "dell'interoperabilità" did not fit a 320px phone.
- Text logo 32px, larger than the title on phones: the user's choice,
  made after seeing it, since at 16px the text logo was hard to read.
- The bitmap font is sharp at 16, 32 and 48px; other sizes are sharp
  only on high-density screens (phones, tablets).

### How it is built

The title is inside the header at every width; the header is a grid
whose named areas change at 1280px, from two rows ("logo, menu" over
"title") to one ("logo, title, menu"). Below 1280px the line is the top
edge of the title row. A first attempt kept the title outside the header
and laid it over the header row on wide screens: it worked, but the
user rejected it as a workaround, along with Tailwind's classes written
straight in the page; how styles are named is now decided in
[ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md).

### How the browser test checks it

The test was written before the code and checks what the reader sees,
not how it is built, at twelve widths: the ten devices of the playground
(320 to 1920px) and the two thresholds' neighbours, 767 and 1279px. At
each width it checks which logo shows and its size, where the title is
and its size, the 24px under the line, the gaps, the 8px margins, the
menu slot, no overlap and no horizontal scroll; it repeats the checks
with two 50-character titles put into the page, and attaches a
screenshot of each width to the report. It failed on the previous site
and passed on the playground before any code was written.
