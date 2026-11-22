# Move the frame's tests to its page

- **Created:** 2026-11-22
- **Status:** Open

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

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  which builds the page the tests move to.

## References

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card this one was cut from, where the move was decided.
- [Move the text column's tests to the frame's page](move-the-column-tests-to-the-frame-page.md):
  the other slice cut from the same card.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
- [ADR-004](../../adr/ADR-004-test-components-once-and-pages-with-a-net.md):
  components tested once, pages with a net.
