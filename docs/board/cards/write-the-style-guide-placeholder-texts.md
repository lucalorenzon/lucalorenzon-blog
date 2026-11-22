# Write the style guide's placeholder texts

- **Created:** 2026-11-22
- **Status:** Closed
- **Closed:** 2026-11-22

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

  **Answered by the user on 2026-11-22**, by the texts written. The
  marks are gone entirely. Of the draft, only the frame page's
  opening says that the site is still a draft, as the reason the
  frame is one piece.

- **How the texts are written.**

  **Answered by the user on 2026-11-22.** One placeholder at a time:
  the agent shows it in the chat, the author writes the text, the
  agent reviews it, and together they decide what goes in.

- **Which sources the frame page cites.**

  **Answered by the user on 2026-11-22.** Jen Simmons for intrinsic
  web design, tied to the layout as a whole, through the transcript
  of her interview on The Big Web Show (2018) rather than the slides
  of her talk, which need the video. Fluid typography is named too,
  since the frame's text uses it, through Maxwell Barvian's article
  on its accessibility, the one read when the column's text was made
  to grow
  ([card](let-the-column-text-grow-within-an-injected-range.md)).
- **Whether the header's text explains the nickname.**

  **Answered by the user on 2026-11-22.** No: the nickname is named,
  not explained.

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

All four pass, checked on 2026-11-22: the frame page's text keeps
three headings of level two and one of level three; the unit tests,
which check that the frame page's text holds no HTML, pass with the
rest (433); no placeholder mark is left in `content/` or in
`src/page_frame.md`; formatting, lint, unit tests, the Python
scripts' tests, the build, the type check and the browser tests (331
passed, one skipped because it runs only on the live site) pass, run
by a sub-agent (Claude Haiku 5.5).

## Recap

The texts were written one at a time in the chat: the agent showed
each placeholder or each section of the draft, the author wrote the
text, often in Italian, the agent reviewed it against the code and
proposed an English version, and the author accepted or changed it.
Taken into work in commit `39032ed`; the texts and this card's close
are in the commit that follows it.

### The colours page

- The section on the code surface, in place of its placeholder: why
  code blocks have a background of their own, and how the wrap
  toggle's icon shows its state by swapping the code accent and the
  code surface. It names no contrast value, since the samples print
  them and a test of the page keeps them out of the text.
- The page's opening, simplified at the author's request: the
  passage on egocentrism is gone, and what is left says that the
  author changed their mind to look for something easier to read.

### The frame page

- An opening of two paragraphs: the frame is one component, left
  whole while the site is a draft, rather than divided too early;
  and it carries the responsive behaviour of the page's layout,
  following intrinsic web design and fluid typography, each with its
  link.
- The header: the story of the logo's photo, the line that runs
  under the stroller's wheels, the nickname that breaks the line;
  then the draft's paragraph on the menu's room and the header's two
  rows below 1280 pixels, kept.
- The text that grows: the logo added to the list of sizes (48 to 64
  pixels), and the first sentence cut, since the opening already says
  the text grows continuously.
- The column: named as the content-sized piece of the layout, and one
  paragraph on the footer, which stays at the bottom of the window and
  links the terms of reuse to the about page.

`CLAUDE.md` no longer says that two texts are placeholders.

## Cautions

- The frame page's words "like the one above the list" and "like this
  one" point at the headings around them: moving or renaming a heading
  means reviewing them.
- The header's section describes the logo and the header's layout in
  words: a change of the logo or of the 1280-pixel breakpoint makes it
  wrong, and no test notices.

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
- [Let the column's text grow within a range the site injects](let-the-column-text-grow-within-an-injected-range.md):
  where the article on fluid typography was read.
