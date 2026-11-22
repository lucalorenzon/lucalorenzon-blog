# ADR-005: Explain a component in a text beside its code

- **Date:** 2026-11-22
- **Status:** Accepted
- **Cards:** decided in
  [show the page frame in the style guide](../board/cards/show-the-page-frame-in-the-style-guide.md);
  applied to the pages written before it by
  [move the style guide's texts beside their code](../board/cards/move-the-style-guide-texts-beside-their-code.md);
  related: [build a design system](../board/cards/build-a-design-system.md)

## Context

The site has a style guide: a set of pages, published with the site,
that shows each of its building blocks (the colours, the code block,
the table, the text, the fonts) and explains how each one behaves.
Each page's explanation is written by the author in a text file kept
with the rest of the site's text, in a folder apart from the program
that builds the site.

The next page shows the page frame: the header, the footer and the
column of text that every page of the site shares. It is the first
page about a building block of the page itself rather than of the
content, and more such blocks will come as the site gains behaviour
(a header that changes as the reader scrolls, for example).

The author's view, since 2026-11-19, is that the explanation of a
building block belongs with the program that makes it, but in a place
of its own, so that changing the program does not mean scrolling
through one long file. With the explanation in the text folder,
nothing reminds whoever changes a building block that its explanation
may no longer be true.

## Decision

The explanation of a component lives in a text file beside the
component's code, with the same name, and that one file serves twice:
as the documentation a programmer reads with the code, and as the text
of the component's page in the style guide.

Why: whoever changes a component sees its explanation in the same
folder and in the same change, so the explanation is more likely to
stay true; the code file stays short, as the author wished; and there
is one text, not a documentation for programmers and a page for
readers that could disagree.

The frame's page is the first to follow it. The pages written before
it keep their text in the text folder until the style guide is
published, and are brought to this model afterwards, by the card
named above.

## Consequences

**Positive:**
- The explanation and the component change together, in the same
  folder and the same commit.
- One text is both the programmer's documentation and the reader's
  page.
- The code file stays short; the explanation has a place of its own.

**Negative / Risks:**
- **Licence.** The site's text has licences of its own, different from
  the code's (`content/README.md`); an explanation beside the code
  falls under the code's licence unless an exception is declared. Which
  one applies is a question of the frame's card.
- **Date in the sitemap.** The step that dates the pages reads only the
  text folder (`scripts/content-dates/`), so a page whose text is
  beside the code has no date until the step reaches it. A question of
  the frame's card.
- **Two rules for a while.** Until the older pages are moved, the style
  guide keeps its texts in two places.
- **The checks on the Markdown** (no HTML in it) must reach the new
  files, or they are not checked.
- Not every page of the style guide describes a component: the
  colours, the fonts and the text's elements may not fit the model.
  Left to the card that moves the older pages.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Keep the explanation in `content/style-guide/`, as every page before | Nothing ties the text to the code it describes; whoever changes a component is not reminded of it |
| Write the explanation as documentation comments inside the component's code file | One long file to open and scroll for every change, which the author wanted to avoid; and the page's text would be extracted from comments |
| A folder of documentation apart from both the code and the text | Still apart from the code, so it loses the reason for the change |

## Technical Notes

For the frame, the component is `src/page_frame.rs` and its
explanation `src/page_frame.md`, taken in twice by the compiler:

```rust
#![doc = include_str!("page_frame.md")]

pub(crate) const EXPLANATION: &str = include_str!("page_frame.md");
```

The first line makes the file the module's documentation; the second
gives its text to the style guide's page, which converts it from
Markdown as the other pages do (`src/markdown.rs`). The names are the
agent's example of 2026-11-22, not fixed by this record.

## References

- Cards:
  [show the page frame in the style guide](../board/cards/show-the-page-frame-in-the-style-guide.md),
  [move the style guide's texts beside their code](../board/cards/move-the-style-guide-texts-beside-their-code.md),
  [build a design system](../board/cards/build-a-design-system.md)
- Related ADRs:
  [ADR-004](ADR-004-test-components-once-and-pages-with-a-net.md), one
  page per component in the style guide, where each component is
  tested once
