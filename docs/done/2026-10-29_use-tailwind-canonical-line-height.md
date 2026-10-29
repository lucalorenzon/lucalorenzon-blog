# Use Tailwind's canonical class for the 32px line height

- **Opened:** 2026-10-29, raised by the user while closing the card
  [fix the editor errors in the browser test](2026-10-29_fix-editor-errors-in-browser-test.md).
- **Status:** done on 2026-10-29, in commit `5eaaeae`.

## Why it was opened

With the TypeScript errors gone, the editor (Zed, through Tailwind's
language server) still showed 3 warnings in `style/input.css`:
`leading-[32px]` can be written as `leading-8`. The card that fixed the
TypeScript errors wanted an editor that can be trusted; warnings left in
place teach to ignore them.

## Answers

### Does the change keep the page exactly as it is?

**Yes.** `leading-8` is 8 times Tailwind's spacing unit of 0.25rem,
that is 2rem, and the stylesheet fixes the root font size at 16px, so
the line height stays 32px. The header layout test measures these
heights; it passed 13 out of 13 in the Playwright 1.64.1 image.

`text-[32px]`, on the same lines, stays as it is: Tailwind has no named
font size of exactly 32px, and the editor gives no warning for it.

## Results

- Tailwind's 3 warnings are gone from the editor.
- Other 16 messages on `style/input.css` turned out to come from the
  editor's generic CSS checker, which does not know Tailwind 4's syntax
  (`@utility`, `@theme`, `@source`, `source(none)`). They were there
  before this change, identical, and are a separate question: card
  [stop the generic CSS checker on the Tailwind stylesheet](../backlog/stop-generic-css-checker-on-tailwind-stylesheet.md).
