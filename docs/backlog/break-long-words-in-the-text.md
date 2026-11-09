# Break long words in the text, so no text can scroll the page sideways

- **Opened:** 2026-11-08, by the user and the agent, while deciding
  which checks make the net, in what became
  [check the layout on every page with the net](../done/2026-11-09_check-the-layout-on-every-page-with-the-net.md)
  ([ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md));
  in Later.
- **Status:** noticed, not started.

## Why

Today the browser breaks the text of paragraphs, lists and headings
only at spaces. A single word wider than the text column, such as a web
address or a full Java name, makes the whole page scroll sideways. For
example `de.hybris.platform.core.PK.createCounterPK` has 42 characters
and is about 395 pixels wide at 18 pixels, while the column of the
narrowest screen is 304 pixels wide.

So whether a page scrolls sideways depends on its text, and the check
stays in the net, run on every real page. With a style rule that
breaks any word too long for the column, no text could make the page
scroll: the check would become a component check, run once on the
worst case (a very long word), as ADR-004 does for the other
components.

## Known from the start

- **At 320 pixels, 26 letters are enough with the reader's spacing:**
  a word of 26 "a" scrolls the page sideways once the letters are
  spaced as WCAG 1.4.12 allows, while 24 do not (measured on
  2026-11-09, in the card
  [check the layout on every page with the net](../done/2026-11-09_check-the-layout-on-every-page-with-the-net.md)).
  The net checks it on every page, so such a word stops the build
  rather than reaching a reader.
- **The rule exists already in two places,** in `style/input.css`: the
  code blocks, when the reader turns wrapping on
  (`overflow-wrap: anywhere`), and the labels and values of the record
  viewer (`hyphens: auto` with `overflow-wrap: break-word`, or
  `anywhere`).
- **The limit of 80 characters per line does not prevent it:** it is
  measured on the lines the browser has already broken, and a line of
  42 characters can still be wider than the column.
- **No length limit on words could replace the rule:** a word made
  only of the widest character, "W", fits in the narrowest column 15
  times, and "responsibilities" already has 16 letters.
- **The 80 characters per line stay in the net anyway:** no style rule
  can bound the characters of a line with a proportional font, so that
  check keeps depending on the text.

## Questions

- How words are broken: hyphenated where the language allows it
  (`hyphens: auto`, the pages declare English), broken anywhere
  (`overflow-wrap: anywhere`), or both, as in the record viewer.
- Which elements take the rule: paragraphs, list items, headings, and
  the code written inside a sentence, where the long Java names
  usually are.
- What the change does to the look of the existing articles, shown on
  the real pages before deciding.
