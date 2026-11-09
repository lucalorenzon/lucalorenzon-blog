# Let Tailwind read only the presentation

- **Opened:** 2026-11-09, by the user, while doing the card
  [refactor the code's errors with thiserror](../done/2026-11-09_refactor-errors-with-thiserror.md).
- **Status:** noticed, not started.

## Why

Tailwind writes a rule into the stylesheet for every word it reads that
matches one of its class names. What it reads is set in
`style/input.css`: today every Rust file in `src/` (`@source
"../src/**/*.rs"`), comments and names included, and the list of the
classes the build generates.

While the build's errors were being refactored, a sub-agent named a
parameter `contents`, and the stylesheet gained `.contents{display:contents}`,
a rule no page uses. The comparison of the built site before and after
caught it, but outside a refactor checked byte by byte nothing would.
Any word of the Rust code can do the same.

It is the third time:

- on 2026-10-29 Tailwind read the whole repository and about fifteen
  classes came from the documentation (`.container`, `.table`,
  `.mt-6`…); the user asked for Tailwind to read only the files that
  matter, and it was limited to `src/**/*.rs`
  ([card](../done/2026-10-29_name-styles-in-the-stylesheet.md));
- on 2026-11-04 the generated HTML would have turned words of the prose
  into classes; the build lists the classes it generates instead, in
  `target/generated/code-classes.txt`.

The limit of 2026-10-29 is still too wide: in `src/` the presentation
(Leptos views carrying the named styles of
[ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md), such as
`title-below-line md:title-below-line-large`) sits in the same files as
the domain. As the user noted, the missing separation of the domain
from the adapters is part of the cause.

## Known from the start

- **Pages carry only named styles** (ADR-001), defined in
  `style/input.css`; Tailwind needs to read only where they are used.
- **The separation of the domain from the adapters**
  ([card](separate-the-domain-from-the-adapters.md)) would make the
  presentation an adapter, so Tailwind could read its files only.
- **Not every answer waits for that card:** the build could list every
  class it puts in a page, as it already does for code blocks and
  tables, and Tailwind would read only that list, no Rust at all.

## Questions

- Whether the stylesheet published today already holds rules born from
  words of the Rust code, and which.
- How to keep Tailwind from reading anything but the presentation: the
  build's list of classes, the presentation's own files after the
  separation, or something else.
- How a stray rule is caught from now on, without a byte-by-byte
  comparison: for example, a check that every rule of the stylesheet is
  used by some page.
