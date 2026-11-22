# Declare the pages to come in the style guide's index

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

## Why

Cut by the user on 2026-11-22 from the card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
which had inherited the question from the superseded card
[show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md).
It concerns the index, not the frame's page, but it has to be answered
before the style guide is published: the user decided on 2026-11-18 to
publish it with its gaps declared, and the index is where a reader
learns which pages are still to come.

## What is known

- **Published with the gaps declared**, decided by the user on
  2026-11-18 (breakpoints page's card): the index lists the pages to
  come. Why: publishing early tells which details the readers value,
  and the real launch of the site comes later, with more articles.
- **The components not yet cut in the code** are declared as a limit
  of the site as it is today, a first version kept small on purpose,
  with a refactor into components planned; decided by the user on
  2026-11-19 (same card).
- **The pages the user wished for later**, named on 2026-11-18: the
  title, the logo, the header, the footer, the text column, the list
  of articles and the article's box in that list.
- **That list has moved since.** The text column went back into the
  frame and its page joined the frame's
  ([show the text column in the style guide](show-the-text-column-in-the-style-guide.md));
  and on 2026-11-22 the user decided that the frame stays one page,
  its parts not cut until the header's dynamic behaviours give them
  their shape (frame's card). Whether the title, the logo, the header
  and the footer are still pages to come, or only parts of the frame
  for now, is open.
- **The index** is built from `content/style-guide/index.md` and
  `src/style_guide/index_page.rs`; it lists the pages that exist
  today. Checked by the agent on 2026-11-22; how it would list a page
  that does not exist is not looked at yet.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which pages are to come**, now that the frame stays one page.

  **Answered by the user on 2026-11-22.** None is declared. The index
  describes what exists now; a page is listed when it is made. The
  user does not know when the next pages will come, nor which ones.
  This changes the decision of 2026-11-18 to publish with the gaps
  declared: what is lost is that a reader cannot ask for a page they
  do not know is coming; what is gained is an index that promises
  nothing that may not come.
- **How the index declares the pages to come.**

  **Answered by the user on 2026-11-22, with the question above.** It
  does not: with no page to come, the index stays as it is, the
  author's text and the list of the pages that exist.
- **What is left of declaring the components not yet cut**, a question
  that came up during the work: on 2026-11-19 the user had decided to
  declare them a limit of the first version, with a division planned.

  **Answered by the user on 2026-11-22.** Only the part on the present
  stays: the frame is one piece today, and at most the site, still a
  draft, needs no other division for now. The user writes it in the
  frame page's text; the announcement of a division to come goes.

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  the last page before publishing, which may change what is left to
  come.

## Recap

Closed on 2026-11-22 with no change to the code: the index already
lists only the pages that exist (`src/style_guide/index_page.rs`, after
the author's text in `content/style-guide/index.md`). The statement on
the present goes into the frame page's text, which the author rewrites
with the other placeholders
([card](write-the-style-guide-placeholder-texts.md)).

## Cautions

- When a page of the style guide is made, it is added to the index's
  list in the same slice; nothing announces it before.

## References

- [Write the style guide's placeholder texts](write-the-style-guide-placeholder-texts.md):
  where the statement on the present is to be written.

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card this one was cut from.
- [Show the breakpoints in the style guide](show-the-breakpoints-in-the-style-guide.md):
  where publishing with the gaps declared was decided.
- [Show the text column in the style guide](show-the-text-column-in-the-style-guide.md):
  a page wished for, joined to the frame's.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
