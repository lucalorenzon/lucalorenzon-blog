# Let the Markdown ask for components for wide code and tables

- **Opened:** 2026-11-04, by the user, while giving the articles a page
  of their own ([card](../done/2026-11-04_add-article-pages.md)); in Now.
- **Status:** noticed, not started.

## Why

The two articles from Medium hold code with lines well beyond 80
characters and tables too wide for a phone. On Medium both scroll
sideways on a phone; the author remembers trying, without success, to
solve it at the time. On this site the readability test forbids both
a page that scrolls sideways and a box that scrolls inside itself
(counted as cut-off text), so the article pages were left out of the
readability and content-layout checks until this card is done.

The user sees a need that was bound to come: telling the presentation
layer how to show a piece of text, where Markdown alone is no longer
enough. Without putting HTML into the Markdown: that would mix
presentation with text, and split the presentation logic between the
layer meant for it and each article.

## Proposal

From the user: annotations in the Markdown name a custom component of
the presentation layer, which reads them to configure the component
and show the text. For example, components such as
`markdown_code_responsive` or `table_layout_responsive`, with the text
wrapped in markers like
`[//]: <> (markdown_code_responsive#start)` … `(markdown_code_responsive#end)`.

What each component does is part of this card. Ideas from the user:

- **Code:** accept scrolling sideways; or a button opening the code in
  a full-screen window, perhaps along the longer side of the screen.
- **Tables:** split the table by columns, repeating key columns, which
  the annotation names; or a full-screen window; or other solutions.

## Known from the start

- **It replaces part of the look card:** what wide code and tables do
  on a phone moved here from the card
  [give the articles' code, quotes and tables a look](design-look-of-articles.md),
  which keeps font, colours and quotes.
- **The readability test may change:** if a component lets code or a
  table scroll inside its box, the rule that counts that as cut-off
  text is to be revised, with the reason.
- **The annotation's form is to be checked:** pulldown-cmark reads
  `[//]: <> (...)` as a link reference definition, which it consumes
  without leaving an event where it stands, so the presentation layer
  may not see it in the flow of the text. For code there is a native
  place for such data: the info string of a fenced block, after the
  language (```` ```rust responsive ````). Tables have no such place.
- **No HTML in the content** stays: the unit test that forbids it
  applies to the annotations too.
- **When the articles go back into the checks:** this card ends with
  the article pages back in the readability and content-layout checks,
  passing.
- **Before the navigation:** the user wants the article pages
  accessible and in as good a state as the curriculum, automatic
  checks and the user's own validation, before the
  [navigation](add-site-navigation.md) links to them.

## Questions

- The form of the annotations, given how pulldown-cmark reads the
  Markdown.
- What each component does, for code and for tables, on a phone and on
  a wide screen.
- How the tests check it, and what changes in the readability rule.
- How thin the first slice can be: for example code only, in one
  article.
