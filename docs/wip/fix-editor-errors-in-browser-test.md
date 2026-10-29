# Fix the editor errors in the browser test

- **Opened:** 2026-10-29, raised by the user while the card
  [adapt the header to screen width](../done/2026-10-29_adapt-header-to-screen-width.md)
  was being worked on.
- **Status:** being worked on since 2026-10-29. The user asked for both
  answers in this card: fix the errors, and check the types in the
  build.

## Why

The user's editor (Zed) shows errors in the Playwright test under
`end2end/`, although the test runs and passes. Playwright runs
TypeScript without checking types, so the errors never show up in the
build. The user wants them fixed so the editor can be trusted.

## Questions

- Which errors exactly. Reproduced on 2026-10-29 by running the
  TypeScript compiler on `end2end/`: 108 errors, 8 of them in the
  project's own files (`process`, `Buffer`, `__dirname`, `node:fs`,
  `node:path` unknown), the rest inside Playwright's own type files for
  the same reasons. The likely cause is confirmed, with one addition:
  the functions passed to `page.evaluate` run in the browser, so the
  browser's types (`document`, `Element`, ...) are needed too. A trial
  copy with `@types/node` and a `tsconfig.json` giving both sets of
  types showed no errors, with strict checking and without skipping
  Playwright's files.
- Whether the build should also check the test's types, so the errors
  cannot come back unnoticed.
