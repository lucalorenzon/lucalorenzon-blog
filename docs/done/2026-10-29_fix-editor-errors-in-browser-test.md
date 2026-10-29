# Fix the editor errors in the browser test

- **Opened:** 2026-10-29, raised by the user while the card
  [adapt the header to screen width](2026-10-29_adapt-header-to-screen-width.md)
  was being worked on.
- **Status:** done on 2026-10-29, in commits `88905d4` (the errors) and
  `9c21aab` (the check in the build).

## Why it was opened

The user's editor (Zed) showed errors in the Playwright test under
`end2end/`, although the test ran and passed. Playwright runs TypeScript
without checking types, so the errors never showed up in the build. The
user wanted them fixed so the editor can be trusted.

## Answers

### Which errors, and why?

**108 errors, all from missing type definitions.** Running the
TypeScript compiler on `end2end/` reproduced them: 8 in the project's
own files, the rest inside Playwright's own type files.

- `process`, `Buffer`, `__dirname`, `node:fs` and `node:path` were
  unknown: the test had no `@types/node`, the types of Node.js.
- The test had no `tsconfig.json`, so the editor guessed the setting.
  It could not know that the test runs in Node, nor that the functions
  passed to `page.evaluate` run in the browser, where `document` and
  `Element` exist.

The fix adds `@types/node`, pinned to 24.19.2 to match the Node 24
runtime, and an `end2end/tsconfig.json` that gives both Node's and the
browser's types, with strict checking. The compiler then reports no
errors, without skipping Playwright's files.

### Should the build also check the test's types?

**Yes (asked by the user), in `build.yml` only.** The job `browser-test`
runs `npm run typecheck` before the test, so an error in the types fails
the build. A deliberate type error was used to see the check fail before
trusting it. `deploy.yml` needs no change: when it rebuilds, it reuses
`build.yml`.

TypeScript is pinned to **6.0.3**. Versions 5.9.3, 6.0.3 and 7.0.2 were
tried, and all three found the same errors and passed the same fixed
code. Version 7.0.2, the latest, was rejected: it no longer ships
`tsserver`, the program the editor's TypeScript support (vtsls) uses,
so the editor and the build would have checked with different tools.
6.0.3 is the newest version that still ships it.

## Results

- In the Playwright 1.64.1 image, after each of the two commits: type
  check with no errors, 13 browser tests out of 13 passed.
- Whether Zed now shows no errors is for the user to see, after
  `npm ci` in `end2end/`.
