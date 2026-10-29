# Check accessibility in the build

- **Opened:** 2026-10-29, from the split of the card "Add automated
  accessibility tests", opened on 2026-10-28 by the user while the card
  [give the page a header, a footer and a favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  was being worked on.
- **Status:** being worked on since 2026-10-29.

## Why

On the previous attempt of the site the user had ruled out colours other
than white for accessibility reasons. For the dark terminal style they
now want to try amber on black. The contrast of the chosen amber,
`#FFB000` on `#000000`, was computed on 2026-10-28 at 11.46:1, above the
WCAG thresholds for normal text (4.5:1 for AA, 7:1 for AAA).

That was a one-off check by hand. Every later change of colour, layout
or content can break accessibility without anyone noticing: the build
should check it every time, the way the browser test already checks
what a visitor sees.

## Agreed scope

Decided with the user on 2026-10-29, when the original card was split:

- **Tool:** axe-core, through `@axe-core/playwright`, inside the
  existing Playwright test: no new test runner. Its licence, MPL-2.0,
  applies file by file and the package is used only by the tests, so it
  never reaches the published site.
- **Level:** the WCAG 2.2 rules of levels A and AA, plus the three
  level-AAA rules axe can check (enhanced contrast of at least 7:1, links
  with the same text leading to the same purpose, no automatic page
  refresh). The build cannot claim the whole of AAA, most of which can
  only be checked by hand; it commits every future colour to at least
  7:1.
- **Widths:** the page differs by screen width (text logo on phones,
  ASCII-art logo from 768px, title beside the logo from 1280px), and
  axe checks only what is shown, so it runs at one width per band.
- **Failure:** any violation fails the build.

What a tool cannot check is in the card
[check accessibility by hand](../backlog/check-accessibility-by-hand.md).
