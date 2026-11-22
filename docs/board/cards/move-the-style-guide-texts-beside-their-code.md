# Move the style guide's texts beside their code

- **Created:** 2026-11-22
- **Status:** Open

## Why

Asked by the user on 2026-11-22, in the card
[show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
to be done after the style guide is published. There the user decided
that the frame's explanation lives in a Markdown file beside the
component's code (`src/page_frame.md`, taken into `src/page_frame.rs`
both as the module's documentation and as the page's text), instead of
in `content/style-guide/` as every other page of the style guide. Left
as it is, the style guide follows two rules, and the pages written
before the frame's keep a text that nothing ties to the code it
describes.

## What is known

- **The pages written before the frame's**, and where their text is
  today (checked by the agent on 2026-11-22): the index
  (`content/style-guide/index.md`), colours, code blocks, tables, text
  and fonts (`content/style-guide/<name>.md`), each read with
  `include_str!` by its page in `src/style_guide/`.
- **The model** is the frame's page, once it is done; its card holds
  the answers on the licence of a text beside the code and on the
  page's date in the sitemap, which this card reuses.
- **Not every page describes a component with code of its own.** The
  code block and the table are components (`src/markdown.rs`); the
  colours, the fonts and the breakpoints are settings of the build
  (`src/style_guide/`); the text page shows the elements the Markdown
  produces. Supposed by the agent, not discussed.
- **Some of these texts hold the cases the browser tests run on**
  (`code.md`, `tables.md`, `text.md`), and `fonts.md` is cut in
  pieces by headings the build places.

## Questions to answer

The ones known when the card was written; there may be more.

- **Which pages move**, and beside which code a page goes when it
  describes a setting or the elements of the text rather than a
  component.
- **Where the cases of the browser tests go**, which are content
  written for the tests rather than explanation.
- **What the index becomes**, which describes the style guide rather
  than one component.

## Dependencies

- The card
  [show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md),
  which sets the model.
- **The publication of the style guide**: decided by the user on
  2026-11-22, this card comes after it.

## References

- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the card where the model was decided.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
- [ADR-005](../../adr/ADR-005-explain-a-component-beside-its-code.md):
  the record of the decision this card applies.
