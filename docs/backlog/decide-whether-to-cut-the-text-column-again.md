# Decide whether to cut the text column again

- **Created:** 2026-11-21
- **Status:** Open

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](../wip/try-a-breakpoint-declaration-on-the-text-column.md),
with the user's agreement. In the spike the user observed that
`OneColumnText` is not generic because it is cut wrong: it took over
things that are not about "one column". The refactor that made it a
component kept the published HTML byte for byte, so the component
took the shape of the class that was already there.

## What is known

- **What the component holds** (described by the agent in the spike):
  the measure of 60 characters, the text size, the page's `<main>`,
  and the look of every element of the text, over a hundred lines of
  the class `long-text-column` in `style/input.css`. The generic part
  is a centred column of a maximum width.
- **Two tensions named by the agent**: the name the user chose says
  "text" too; and the column's width is counted in characters of its
  own font, so width and text size are one thing for the stylesheet.
- **The spike's experiment found** that samples of the column on one
  page are each a `<main>`, which axe's best practices report; a
  sample would need the component to write another element.

## Questions to answer

The ones known when the card was written; there may be more.

- **What belongs to "one column"** and what to the text inside it,
  and whether the two become two components.
- **Who writes the `<main>`**: the column, the page, or the frame.
- **Whether the measure and the text size can be parted**, since the
  width is counted in characters of the text.

## References

- [Try a breakpoint declaration on the text column](../wip/try-a-breakpoint-declaration-on-the-text-column.md):
  the spike where the question came up.
- [Make the text column a component](../done/2026-11-19_make-the-text-column-a-component.md):
  the refactor that gave the component its shape.
- [Show the text column in the style guide](../wip/show-the-text-column-in-the-style-guide.md):
  the page that would show the column's samples.
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the next change to the component.
