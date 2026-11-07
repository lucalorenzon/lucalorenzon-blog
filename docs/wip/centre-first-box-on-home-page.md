# Show the first box of the home page at mid-height of the screen

- **Opened:** 2026-11-07, by the user, right after the card
  [frame each article on the home page in a box](../done/2026-11-07_frame-home-page-articles-in-boxes.md);
  in Now.
- **Status:** being worked on.

## Why

The first thing a reader should meet on the home page is the first
thing to read, and it should meet it where the eye goes first: the
middle of the screen. Today the boxes start right below the page
title, at the top of the screen; on a laptop most of the screen below
them stays empty.

The user wants the first box to appear at mid-height of the screen
when the page opens. Scrolling then works as usual: the page does not
move by itself. The box is already centred across the page, since the
text column is.

## Known from the start

- **What the screen holds today** (from the screenshots of the
  previous card): on a laptop, 1280 by 800 pixels, both boxes end
  about a third of the way down; on a phone, 320 by 568, the header
  and the title take about 130 pixels and the two boxes fill the rest
  of the screen.
- **Pushing the first box down pushes the others too.** On a phone
  the second article would leave the screen; on a laptop, as the
  articles grow, fewer of them are seen without scrolling.
- **What it touches:** the `article-list` style in `style/input.css`,
  possibly the home page's layout in `src/home_page.rs` and the page
  frame shared by every page (`src/page_frame.rs`), which must not
  change for the other pages; and the layout tests, among them the one
  that keeps 32 pixels between the text and the footer, which stays
  in view at the bottom of the screen.
- **How the result is judged:** as for the boxes, on the real page at
  phone and laptop width, light and dark, with alternatives side by
  side before a commit, and with a layout test on the range of
  devices.

## Questions

- What exactly sits at mid-height: the middle of the first box, or its
  top.
- On which screens: every screen, or only those where the boxes leave
  the screen half empty, such as a laptop.
- Where the page title goes: it stays at the top, or moves down with
  the box.
