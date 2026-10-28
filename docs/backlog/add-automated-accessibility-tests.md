# Add automated accessibility tests

- **Opened:** 2026-10-28, asked by the user while the card
  [give the page a header, a footer and a favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

On the previous attempt of the site the user had ruled out colours other
than white for accessibility reasons. For the dark terminal style they
now want to try amber on black. The contrast of the chosen amber,
`#FFB000` on `#000000`, was computed on 2026-10-28 at 11.46:1, above the
WCAG thresholds for normal text (4.5:1 for AA, 7:1 for AAA).

That was a one-off check by hand. Every later change of colour, layout
or content can break accessibility without anyone noticing: the
pipeline should check it at every build, the way the browser test
already checks what a visitor sees.

## Questions

- Which tool: for example an accessibility scanner run inside the
  existing Playwright test, so no new test runner is added.
- Which level the site commits to (WCAG AA or AAA), and whether a
  violation fails the build.
- What a tool cannot check, such as how the ASCII-art logo sounds in a
  screen reader, and how that is covered.
