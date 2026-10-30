# Run the browser tests in more browsers

- **Opened:** 2026-10-30, asked by the user while the card
  [check the accessibility tree in the build](../wip/check-accessibility-tree-in-the-build.md)
  was being worked on.
- **Status:** noticed, not started.

## Why

Every browser test runs in Chromium only (`end2end/playwright.config.ts`,
a single project "chromium"). Visitors also use Firefox and Safari, whose
engines (Gecko, WebKit) can lay out the page, load the font or expose
the accessibility tree differently: a break in one of them would go
unnoticed.

Playwright drives three engines: Chromium, Firefox and WebKit (the
engine of Safari), and its official container image, already used by
the build, contains all three.

## Questions

- Which browsers: Firefox and WebKit on the desktop, and whether also
  the mobile profiles Playwright emulates (for example Safari on an
  iPhone).
- Whether every test runs in every browser, or only some (for example
  layout and accessibility), and what that costs in build time.
- Whether a failure in any browser fails the build, as in Chromium.
