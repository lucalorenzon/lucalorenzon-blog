# Check the layout on every page with the net

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  "check every page with tests written once", third slice of
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md);
  third of its three parts, after
  [take the net's pages from the sitemap](../done/2026-11-09_take-the-nets-pages-from-the-sitemap.md).
- **Status:** in progress.

## Why

`readability.spec.ts` and `content-layout.spec.ts` run on `TEXT_PAGES`
at the five widths of the text column, and the list carries two facts
about the content: `hasRoles` (the page has role headings, level 3) and
`hasConsecutiveParagraphs`. Two of their checks depend on the text
itself and belong to the net; the rest are component checks.

## Already decided, while splitting

- **Sideways scroll at the narrowest width is net** (user, on the
  agent's correction). The text breaks only at spaces, so a word wider
  than the column scrolls the page whatever the rest of the style does:
  it depends on the text. A style rule breaking long words would make
  it a component check:
  [break long words in the text](break-long-words-in-the-text.md).
- **Sideways scroll with the reader's spacing, at the narrowest
  width, is net too** (user, 2026-11-09). It depends on the text even
  more: with the letters spaced as a reader may set them (WCAG 1.4.12)
  the same word is about a quarter wider, so a long word overflows
  there first. ADR-004's census had made it a component check,
  accepting that loss; with the criterion above it no longer holds,
  and the ADR carries a note.
- **At most 80 characters per line, at the widest column, is net.** It
  is counted on the lines as the browser breaks them, and with a
  proportional font no style rule can bound it, so it depends on the
  text.
- **`hasRoles` and `hasConsecutiveParagraphs` are dropped** (user).
  Each asserts a fact about a page's text, so a change of text breaks
  the test although the site still works; they guard their checks from
  passing with nothing to check, which the components' test pages will
  do instead.

**Rejected:** sideways scroll as a component check today (the agent's
first proposal, which wrongly assumed the text already breaks long
words); bounding it by 80 characters of the widest glyph, which never
fit the column: 80 "W" are 1568 pixels with text at 18 pixels and 1742
at 20, against a column of 304 to 704 pixels.

## Decisions to take

- **Where the component checks of these two files run until the
  components' test pages exist:** on the real pages as today, or moved
  together with the test pages.
