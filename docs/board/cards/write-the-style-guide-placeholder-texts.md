# Write the style guide's placeholder texts

- **Created:** 2026-11-22
- **Status:** In work

## Why

Asked by the user on 2026-11-22, while closing the card
[declare the pages to come in the style guide's index](declare-the-pages-to-come-in-the-style-guide-index.md):
the texts of the style guide still marked as placeholders are written
by the author in one go, and then the style guide is published. Left
as they are, the published guide would show a reader a draft written
by the agent and a section that says only what it should hold.

## What is known

- **The texts to write**, as the agent read the user's request
  ("all three texts") on 2026-11-22, confirmed by the user the same
  day:
  1. the frame page's text, `src/page_frame.md`, a whole draft by the
     agent, written while building the page
     ([card](show-the-page-frame-in-the-style-guide.md)), with its
     sub-section on the headings added later
     ([card](move-the-column-tests-to-the-frame-page.md));
  2. one section of the colours page, `content/style-guide/colours.md`,
     a placeholder that says what the author is to explain: why the
     code surface, a background everywhere else, becomes the colour of
     a graphic on the code accent, and where the site uses it (the
     wrap toggle's icon, when the lines wrap);
  3. in the frame page's text, a statement of the present only: the
     frame is one piece today, and the site, still a draft, needs no
     other division for now. Decided by the user on 2026-11-22 (index
     card), in place of declaring a division to come.
- **No other placeholder** is in `content/` or beside the code: checked
  by the agent on 2026-11-22 by searching for "placeholder" and
  "draft".
- **The author writes them**; the agent's part is to check them against
  what the tests need:
  - the frame page's text must keep at least one heading of level two
    and one of level three, or the column's test fails on purpose
    ([card](move-the-column-tests-to-the-frame-page.md));
  - the Markdown of the site holds no HTML (a unit test checks the
    frame page's text; the articles have their own check);
  - the page is dated in the sitemap by its text, so the new date
    comes by itself.
- **The licence**: the frame page's text is text of the site, under
  CC BY 4.0, though it lives beside the code
  ([ADR-005](../../adr/ADR-005-explain-a-component-beside-its-code.md)).

## Questions to answer

The ones known when the card was written; there may be more.

- **Whether the three texts are the ones the user meant.**

  **Answered by the user on 2026-11-22.** Yes, the three listed above.
- **Whether the placeholder marks go** once the author's text is in,
  or something of them stays.

- **How the texts are written.**

  **Answered by the user on 2026-11-22.** One placeholder at a time:
  the agent shows it in the chat, the author writes the text, the
  agent reviews it, and together they decide what goes in.

## Dependencies

- The author's time to write the texts.

## Acceptance tests

Agreed with the user on 2026-11-22, when the card was taken into work.

1. The frame page's text, `src/page_frame.md`, keeps at least one
   heading of level two and one of level three.
2. The Markdown of the three texts holds no HTML.
3. No placeholder mark is left in `content/` or beside the code,
   unless the answer to the question on the marks keeps one on
   purpose.
4. The build and every test pass, as the pipeline runs them.

## References

- [Declare the pages to come in the style guide's index](declare-the-pages-to-come-in-the-style-guide-index.md):
  where this card was asked for.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  where the frame page's draft was written.
- [Move the text column's tests to the frame's page](move-the-column-tests-to-the-frame-page.md):
  the headings the frame page's text must keep.
- [Show the colours in the style guide](show-the-colours-in-the-style-guide.md):
  the colours page, with its placeholder section.
- [Build a design system](build-a-design-system.md): the card the
  style guide belongs to.
