# Run in one theme the tests that do not check colours

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on making the browser tests check behaviour, not articles
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)); in Now, first of three slices.
- **Status:** being worked on, since 2026-11-08.

## Why

Every browser test runs twice, in the dark and in the light theme
(`end2end/playwright.config.ts`, projects `chromium-dark` and
`chromium-light`). The light theme changes colours only: the
`prefers-color-scheme: light` block of `style/input.css` sets colour
variables and nothing else. So every check of size, position, keyboard,
address or tag gives the same result in both themes, and half of the
913 tests repeat the other half.

## Decision to take

Which tests check colours, and so keep both themes. From the census in
[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md): axe's contrast rules, the colours of the section and
role headings, the colours of list markers, the border of the home
page's boxes. Everything else runs in one theme, and which one.

## Known from the start

- **No check is lost,** as long as no theme starts to change something
  other than colours; whether that assumption needs a guard of its own
  is part of the decision.
- **axe mixes both kinds:** its contrast rules depend on the theme, its
  other rules do not.
