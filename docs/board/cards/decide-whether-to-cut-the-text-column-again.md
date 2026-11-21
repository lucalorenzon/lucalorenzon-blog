# Decide whether to cut the text column again

- **Created:** 2026-11-21
- **Status:** Closed
- **Closed:** 2026-11-21

## Why

Opened by the agent on 2026-11-21, at the close of the spike
[try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md),
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

  **Answered by the user on 2026-11-21.** Neither: the component
  `OneColumnText` is removed. It was made for two reasons: to start
  splitting `PageFrame`, which does many things, into smaller
  components; and to give the change of text size of the wireframe
  page a place of its own, a page that no longer exists. The second
  reason is gone, and the first one took the `<main>` away from the
  frame, which is the only part of the site for which holding
  `<header>`, `<main>` and `<footer>` together makes sense: it is the
  page's skeleton. The component holds almost no code, one line with
  two class names; the look of the column and of its text is in the
  stylesheet, `style/input.css`, and stays there whether the
  component exists or not, since with this stack a component can only
  name the classes. The user saw nothing lost in removing it, unless
  something were added to it later.

  What is given up, named by the agent: the frame grows again by one
  element; a page can no longer ask for a `<main>` without the look
  of the text, which no page wants today; and the frame has to take
  what the component took, the HTML made from a Markdown file or
  other components.
- **Who writes the `<main>`**: the column, the page, or the frame.

  **Answered by the user on 2026-11-21.** The frame, `PageFrame`,
  with the two classes the component gave it, so every page keeps the
  look of the text it has today.
- **Whether the measure and the text size can be parted**, since the
  width is counted in characters of the text.

  **Left without object on 2026-11-21**, by the answers above: with no
  column component, there is nothing to part them from. The question
  can come back if the look of the text is ever given to something
  other than the frame's `<main>`.
- **Who gives the look of the text to the pages**: the frame's
  `<main>` for every page, as today, or something else. Added on
  2026-11-21 by the agent: the stylesheet's rules for the text apply
  also to pages that are not made from Markdown (the home page, the
  "page not found", the style guide's index), and the home page's
  list of boxes already has to win over the column's rule for lists
  (browser test `boxes are 2.25em apart`).

  **Answered for now by the user on 2026-11-21**: the frame's `<main>`
  carries the classes, as the component did; nothing else changes.

## Acceptance tests

Agreed with the user on 2026-11-21, for the removal of the component,
done as a refactor in this card.

- The site built after the change is identical, byte for byte, to the
  site built before it.
- The module `src/one_column_text.rs` no longer exists; `PageFrame`
  writes the `<main>` with the classes `long-text-column` and
  `md:long-text-large`, holding either the HTML made from a Markdown
  file or other components, and its unit tests check it.
- Every check of the pipeline passes, as in
  `.github/workflows/build.yml`.
- The row of the browser test `text column of the style guide's text
  page` in `end2end/README.md`, which names the component, is
  reviewed in the same commit.

## Recap

The column was not cut again: the component `OneColumnText` was
removed, and the page frame writes the `<main>` again, between the
header and the footer.

- The answers and the acceptance tests written in the card
  (`392e900`).
- The refactor (`fa15da1`), written by a sub-agent on Sonnet 5.5 and
  checked by the agent: `PageFrame` writes the `<main>` with the
  classes `long-text-column` and `md:long-text-large`, held in one
  constant, and takes either the HTML made from a Markdown file
  (`html`) or other components (`children`); the component's two
  unit tests moved to the frame. The eight pages that used the
  component give the frame what they gave it. In the style guide,
  `GuideMain` became `GuideBody`, since it now returns only what goes
  inside the `<main>`. The row of the column's browser test in
  `end2end/README.md` and the repository map in `CLAUDE.md` were
  updated in the same commit.
- Every acceptance test passed: the site built after the change was
  compared with the one built before it and is identical byte for
  byte, the stylesheet included; formatting, clippy, 423 unit tests,
  the build, the type check and 275 browser tests passed, with one
  skipped, Lighthouse included.
- The consequences on the other cards (`e487148`): the column's page
  in the style guide was closed, and what it was to show joined the
  page of the frame
  ([show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md)),
  decided by the user; the card on the column's growing text no
  longer names the removed file.

## Cautions

- **The look of the text is still given to every page**, through the
  frame's `<main>`, also to pages not made from Markdown: the home
  page's list of boxes has to win over the column's rule for lists.
  A page that needs a `<main>` without that look needs a change to
  the frame first.
- **The code block takes its spacing from `--long-text-size`**, the
  property the `<main>`'s class declares (`style/input.css`): it
  looks right only inside the frame's `<main>`.

## References

- [Try a breakpoint declaration on the text column](try-a-breakpoint-declaration-on-the-text-column.md):
  the spike where the question came up.
- [Make the text column a component](make-the-text-column-a-component.md):
  the refactor that gave the component its shape.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  the page that would show the column's samples.
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  the next change to the column, now in the frame.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that shows the column, since its own page joined it.
