# Fix the editor errors in the browser test

- **Opened:** 2026-10-29, raised by the user while the card
  [adapt the header to screen width](../done/2026-10-29_adapt-header-to-screen-width.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

The user's editor (Zed) shows errors in the Playwright test under
`end2end/`, although the test runs and passes. Playwright runs
TypeScript without checking types, so the errors never show up in the
build. The user wants them fixed so the editor can be trusted.

## Questions

- Which errors exactly. Likely cause, not yet confirmed: `end2end/` has
  no `tsconfig.json` and no `@types/node`, while the test uses `process`,
  `__dirname`, `node:fs` and `node:path`.
- Whether the build should also check the test's types, so the errors
  cannot come back unnoticed.
