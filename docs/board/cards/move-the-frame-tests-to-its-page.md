# Move the frame's tests to its page

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

## Why

Cut by the user on 2026-11-22 from the card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md).
There the user decided that the tests that check the frame as a
component leave the home page for the frame's page, as the
components' tests left the articles when the style guide took them
over
([ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md)).
The page itself is the frame's card's slice; moving the tests onto it
is this card. Left as it is, the home page keeps doing for the frame
what the articles did for the text's components: a page of content
used as a test page.

## What is known

- **The tests that move**, as agreed on 2026-11-22 (names as in
  `end2end/README.md`):
  - in `header-layout.spec.ts`: `header layout at ${width}`,
    `the header's line under the name at ${width}`,
    `footer text size at ${width}`, `header with the reader's text spacing`;
  - in `accessibility.spec.ts`: `accessibility of the frame, ${label}`
    and `contrast of the frame, ${label}`, for their home page's part.
- **What stays where it is:** the net on every page of the sitemap
  (`the logo on every page at ${width}`, `shape of the ${pageName}`),
  the tests of the home page's own content (its list of articles and
  their links), and the "page not found", tested today with the home
  page for the frame's accessibility and contrast: a page of its own,
  with links written for addresses of any depth.
- **The longest titles need nothing of the build:** the header's test
  puts its two invented titles in place of the heading in the browser
  (checked by the agent on 2026-11-22).
- **Each moved test's row** in `end2end/README.md` changes in the same
  commit (project rule, "The browser tests' README").

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether a moved test still guards what it guarded on the home
  page**, which has a list of articles under the header: whether any
  of these checks depended on what the home page holds below the
  frame.

  **Answered by the agent on 2026-11-22, the last point accepted by
  the user the same day.** Four of the six tests look only at the
  frame: the header's line under the name, the footer's text size,
  and, apart from one rule, the header's layout and the header with
  the reader's text spacing. That one rule, in both, is that the page
  does not scroll sideways, which looks at the whole page; for the
  home page the net already checks it on every page of the sitemap at
  320 pixels, with and without the reader's spacing
  (`content-layout.spec.ts`). The header's layout uses the home page's
  real title, "Home Page", and checks it centred; the frame page's
  "Page frame" is as short, so the rule holds. What is lost is in the
  frame's accessibility and contrast: axe examines today the whole
  home page at the three width bands, its list of articles included,
  and after the move only at the laptop's width, through the net. The
  list changes form with the width; its layout stays checked at every
  width of the column by the home page's own tests in
  `content-layout.spec.ts`. The user accepted that the list is
  examined by axe at the laptop's width only, as every other page of
  content (ADR-004).

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  which builds the page the tests move to.

## Acceptance tests

Agreed with the user on 2026-11-22.

1. The four tests of the header and the footer (`header layout at
   ${width}`, `the header's line under the name at ${width}`,
   `header with the reader's text spacing`, `footer text size at
   ${width}`) open the frame's page, `style-guide/frame.html`, not the
   home page, still on every device.
2. Where they use the home page's real title, "Home Page", they use
   the frame page's, "Page frame"; the header's layout still checks it
   centred, together with the two invented titles as long as the build
   accepts.
3. The frame's accessibility and contrast tests run on the frame's
   page and on the "page not found", at the three width bands; the
   home page leaves that list and stays in the net only, at the
   laptop's width.
4. No other test changes: neither the net, nor the home page's own
   tests, nor those of the "page not found"; the number of tests stays
   the same.
5. In the same commit, the rows of the six tests in
   `end2end/README.md` (the page they run on, and the text of "Why")
   and the comments at the top of the two test files that name the
   home page as the frame's page change with them.
6. Every check of the pipeline passes, as in `build.yml`: formatting,
   clippy, unit tests, type check and Playwright, the README's test
   included.

## Recap

Done on 2026-11-22: the card taken into work with its acceptance tests
(`cb076fc`), the tests moved (`cb80c02`).

- The four tests of the header and the footer in
  `header-layout.spec.ts` open `style-guide/frame.html`, named once in
  the constant `FRAME_PAGE`, and use the page's title, "Page frame",
  where they used "Home Page".
- In `accessibility.spec.ts`, the frame's list of pages holds the
  frame's page and the "page not found"; the home page is in the net
  only.
- The rows of the six tests in `end2end/README.md`, and the comments
  at the top of the two files, name the frame's page.
- Every acceptance test passed: the moved tests pass on the frame's
  page with no rule changed, which confirms they did not depend on the
  home page's content; the suite still counts 332 tests (331 passed,
  1 skipped, the "page not found" check that runs on the live site
  only); formatting, clippy, the 434 unit tests and the type check
  pass.

## Cautions

- The home page's list of articles is examined by axe at the laptop's
  width only. Should it become a component of its own, its page in the
  style guide is where it would be examined at every width.
- The README's class "Frame" still lists the home page among the
  site's own pages: its own tests (`home.spec.ts`) stay in that class.

## References

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card this one was cut from, where the move was decided.
- [Move the text column's tests to the frame's page](move-the-column-tests-to-the-frame-page.md):
  the other slice cut from the same card.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
- [ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md):
  components tested once, pages with a net.
