# Scroll before the accessibility check

- **Opened:** 2026-11-16, by the user, while building the style guide's
  text page ([card](show-the-text-elements-in-the-style-guide.md)):
  the new link on the style guide's index made the net's accessibility
  test fail. In Now, before that card's first slice was committed.
- **Status:** done, 2026-11-16.

## Why

The net's accessibility test (`end2end/tests/accessibility.spec.ts`,
`look`) opened a page and ran axe at once, from the top of the page.
There the sticky footer covers a band at the bottom of the window, and
a link that falls in that band is reported as too small a target (axe's
`target-size`, WCAG 2.5.8), although a reader scrolls and finds it
whole. The fourth link of the style guide's index, "Text", fell there
at 1280 by 720. Any page with a link in that band at the tested width
would have failed the same way; until then none happened to.

## Answers

- **A fault of the test, not of the page**, decided by the user: if the
  page scrolls, the test must scroll it. Rejected: shortening the
  style guide index's text, which is the user's and would leave the
  test as fragile; changing the layout, for a footer the reader is not
  hindered by.
- **Scroll to the end, once, before axe runs**, as the text checks
  already did for the same footer (`content-layout.spec.ts`,
  `snapshot`). At the end the content stops at least 32 px above the
  footer, a check already in force, so nothing is covered; the links
  higher up are out of the window, covered by nothing. No position in
  between needs a check of its own: the footer always covers the same
  band, and the header is not sticky.

## Acceptance tests

All five pass, on 2026-11-16.

1. `look` scrolls to the end of the page before running axe.
2. With the style guide's index holding the "Text" link, the test
   `accessibility of the style-guide/ at 1280x720 (Laptop)` fails
   before the change and passes after.
3. The contrast rules still pass on every page, in both schemes, and
   the number of elements axe judges for contrast is the same before
   and after the change, on every page of the net.
4. The test's row in `end2end/README.md` is still true, or is
   updated; `readme.spec.ts` passes.
5. The checks of `build.yml` pass.

## Results

- **Before the change**, in the light scheme: `target-size` on the
  "Text" link, "partially obscured (smallest space is 43px by 22px,
  should be at least 24px by 24px)". **After:** the test passes.
- **The contrast was judged on the same elements**: for both contrast
  rules, on the twelve pages of the net and in both schemes, the number
  of elements axe judged (passed, failed or left incomplete) is the
  same before and after; the two lists, sorted, are identical. Measured
  with a temporary log, removed.
- **The README's rows** of the accessibility and contrast tests are
  still true, unchanged.
- **The checks of `build.yml`**, run locally on 2026-11-16, with the
  text page's first slice also in the working tree: format, lint, 343
  unit tests, type check, 247 browser tests passed, one skipped (the
  "page not found" test that runs on the live site only).
