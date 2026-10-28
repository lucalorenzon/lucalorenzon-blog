# Make the site responsive

- **Opened:** 2026-10-28, asked by the user while the card
  [give the page a header, a footer and a favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

The header shows the narrow ASCII-art logo, about 43 characters wide, on
every screen. In the IBM VGA 8x16 font at 16px it measures about 344px,
so on a 375px-wide phone it fits, but the title "Home Page" next to it
wraps onto two lines, squeezed against the right edge. The user chose to
accept this for the first
version and to make the site adapt to screen size in a card of its own.

## Questions

- What a phone shows in place of the logo, or how the logo shrinks. The
  old site (branch `old_main`, `src/components/logos/logo.rs`) switched
  to the plain text "Lvk@73r" on small screens, and could show the wide
  ASCII-art version on large ones.
- Which other parts of the page need to adapt: header, footer, and later
  articles.
- How the browser test checks the page at a phone's width.
