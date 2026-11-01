# Space out the lines of text

- **Opened:** 2026-11-01, by the user, reading the published site.
- **Status:** being worked on since 2026-11-01.

## Why

Reading the curriculum on the live site, the user found the text hard
to read, first of all because the lines sit too close together. The
colours are the other half of the same remark, in the card
[make the text colours easier to read](../backlog/make-text-colours-easier-to-read.md).

What the stylesheet does today (`style/input.css`): the line height
equals the font size, 16px on 16px, a ratio of 1.0, for the whole page
(`html, body`). Paragraphs and list items are 8px apart (`mt-2`); lines
inside a paragraph have no space between them.

## Why the build did not catch it

The build checks accessibility with axe, but no axe rule looks at this:

- **Criterion 1.4.8** (level AAA, visual presentation) asks for line
  spacing of at least 1.5 inside paragraphs, and space between
  paragraphs at least 1.5 times that. axe 4.14.0 has no rule for it.
  The build runs only the three AAA rules axe can check (enhanced
  contrast, links with the same text, no automatic refresh); the card
  [check accessibility in the build](../done/2026-10-30_check-accessibility-in-the-build.md)
  already says the site is not AAA, since most AAA criteria can only be
  judged by a person.
- **Criterion 1.4.12** (level AA, text spacing) does not ask the author
  for a wide line height: it asks that the page still works when the
  reader raises line height and spacing on their own. axe's only rule
  for it, `avoid-inline-spacing`, looks only at spacing written in an
  element's `style` attribute; the page has none, and the stylesheet is
  not examined.

The card
[cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md)
is limited to levels A and AA, so it would not have covered 1.4.8
either.

## Known from the start

- **A line height of 24px is 1.5 and still a multiple of 8px.** Line
  height does not scale the glyphs of the pixel font, which stay 16px
  and sharp.
- **The header's heights are measured** by the layout test: a change on
  `html, body` would reach the header too, not only the text column.

## Questions to answer

- **Which text.** Only the text column (`long-text-column`), or the
  header and footer too.
- **How much line height,** and whether the space between paragraphs
  and list items changes with it.
- **How the build catches it next time.** Two candidates: a test that
  measures the line height in the text column and fails below 1.5
  (criterion 1.4.8); a test that raises line height and spacing as a
  reader would, and checks that no text overlaps or is cut off at the
  tested widths (criterion 1.4.12).
- **How the result is judged** beyond the tests: the user's reading, on
  the live site and on a phone.
