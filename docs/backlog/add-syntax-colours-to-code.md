# Add syntax colours to code

- **Opened:** 2026-11-04, by the user, after giving code blocks their
  box ([look card](../wip/design-look-of-articles.md)); in Later.
- **Status:** noticed, not started.

## Why

Code in the articles is one colour, off-white on dark grey. Colouring
its parts (keywords, strings, comments) helps a reader find their way
in a long block, as in a code editor or on most technical sites.

## Known from the start

- **The base was chosen for it:** off-white code (`#e6e6e6`) on
  `#1a1a1a` was preferred to amber partly because a neutral base takes
  other colours better.
- **Contrast:** the site checks enhanced contrast (WCAG 1.4.6, at least
  7:1). Every colour must keep it on `#1a1a1a`, which rules out many
  usual themes' darker hues; and colour must not be the only way a
  meaning is shown (WCAG 1.4.1).
- **No script:** the colouring is done at build time, by the Rust
  build, not in the browser.
- **Languages of the articles:** Java, Groovy, SQL, XML, properties.
- **Shared need:** recognising the parts of the code per language is
  also what [breaking wrapped lines by grammar](break-wrapped-code-by-grammar.md)
  needs; one tool could serve both.
- **Named styles:** the page carries only our own style names
  ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md)); the
  colour classes a highlighter writes must fit that rule, or be an
  exception to it.

## Questions

- Which tool, at build time, for these languages.
- Which colours, within 7:1 on the code's surface, and how they fit the
  site's amber.
- How the copied code stays exactly the article's code.
