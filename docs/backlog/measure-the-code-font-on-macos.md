# Measure the code font on macOS

- **Created:** 2026-11-17
- **Status:** Open

## Why

The user opened this card on 2026-11-17, while working on the card
[hide the wrap toggle on blocks that fit](../wip/hide-the-wrap-toggle-on-blocks-that-fit.md),
and put it in Later: the check matters less than the fix.

The wrap toggle of the code blocks shows from a threshold the build
computes from Hack's advance, 1233/2048 of an em. Whether the toggle
is right depends on how wide the browser really draws Hack, and that
was measured on Linux only. Readers on macOS, with Safari or Chrome,
may see the toggle on a block that fits, or a block that overflows
without it, and nothing would show it.

## What is known

- **On Linux**, measured on 2026-11-17 (details in the card above):
  Chromium rounds each character to whole pixels, in both directions,
  unless the code has `text-rendering: geometricPrecision`; Firefox
  and WebKit draw the true width.
- **Playwright's WebKit on Linux is not Safari.** It draws fonts with
  Linux's tools; it is close to Epiphany, GNOME's browser, which uses
  the same port, and says nothing certain of Safari on macOS.
- **Docker on a Mac does not help.** It runs a Linux machine, so the
  browsers inside it draw as on Linux.
- **A page that measures in any browser was built and checked** on
  2026-11-17, in the session's scratchpad, not kept: one HTML file
  with Hack embedded, which draws a line of 100 characters at every
  size from 10 to 30 pixels, in tenths, with and without
  `geometricPrecision`, and prints a summary and the full result as
  text to copy back. In Chromium and WebKit on Linux it gave the same
  numbers as the measures on the built site.

## Questions to answer

The ones known when the card was written; there may be more.

- **How Safari and Chrome on macOS draw Hack**, with and without
  `text-rendering: geometricPrecision`.
- **Where to measure**: on a Mac at hand, opening the page above, or on
  a service of remote browsers such as BrowserStack, whose free terms
  were not checked (a trial with few minutes, and a programme for open
  source projects, as far as was known on 2026-11-17).

## References

- [Hide the wrap toggle on blocks that fit](../wip/hide-the-wrap-toggle-on-blocks-that-fit.md):
  the card whose measures this one completes.
