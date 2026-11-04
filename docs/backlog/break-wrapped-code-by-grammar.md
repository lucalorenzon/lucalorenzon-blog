# Break wrapped code lines by grammar

- **Opened:** 2026-11-04, by the user, as the second slice of the card
  on [wide content](../wip/annotate-markdown-for-wide-content.md),
  moved here when the user judged the code good enough for now; in
  Later.
- **Status:** noticed, not started.

## Why

When a reader turns on "wrap", a long line of code wraps by indent:
the continuation starts 4 columns deeper than the line, and the break
falls wherever the width runs out, sometimes in the middle of a word
(`getSer` / `ialNumberGenerator`). Comparing the two on lines of both
articles at iPhone SE width, the user found breaking by grammar (before
a `.`, after a `(` or a `,`) clearly more readable, and chose "grammar,
with indent as fallback", indent first.

## Known from the start

- **Width varies:** code wraps on every screen, so the breaks cannot be
  computed once for one width. The idea agreed: the build marks where
  the grammar allows a break, and the browser uses only those, with the
  indent wrap as fallback when a piece still does not fit.
- **No script:** a formatter running in the browser was rejected (a
  script and a formatter per language, and weight on the page).
- **Languages of the articles:** Java, Groovy, SQL, XML, properties.
- **Shared need:** recognising the parts of the code per language is
  also what [syntax colours](add-syntax-colours-to-code.md) need; one
  tool could serve both.
- **The component today:** `src/markdown.rs` writes each line as an
  element with its indent; the "wrap" toggle and its threshold are in
  `style/input.css`.

## Questions

- Which tool recognises the code's parts at build time, for these
  languages.
- How a break point is marked without changing the copied text.
- How the tests check that a wrapped line breaks only at those points.
