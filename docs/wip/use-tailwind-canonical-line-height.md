# Use Tailwind's canonical class for the 32px line height

- **Opened:** 2026-10-29, raised by the user while closing the card
  [fix the editor errors in the browser test](../done/2026-10-29_fix-editor-errors-in-browser-test.md).
- **Status:** being worked on since 2026-10-29.

## Why

With the TypeScript errors gone, the editor (Zed, through Tailwind's
language server) still shows 3 warnings in `style/input.css`:
`leading-[32px]` can be written as `leading-8`. The card that fixed the
TypeScript errors wanted an editor that can be trusted; warnings left in
place teach to ignore them.

## Questions

- Whether the change keeps the page exactly as it is. `leading-8` is
  2rem, and the stylesheet sets the root font size to 16px, so it should
  stay 32px; the header layout test measures these heights.
