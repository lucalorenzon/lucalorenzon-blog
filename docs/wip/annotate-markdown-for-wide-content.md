# Let the Markdown ask for components for wide code and tables

- **Opened:** 2026-11-04, by the user, while giving the articles a page
  of their own ([card](../done/2026-11-04_add-article-pages.md)); in Now.
- **Status:** in progress since 2026-11-04.

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
  [navigation](../backlog/add-site-navigation.md) links to them.

## Slice 1: wide code (2026-11-04)

### How thin

**Chosen, by the user: code only,** with "After Save Event" as the
article that goes back into the checks. Its only table has two columns
and fits a phone, so the page could pass the readability checks at the
end of the slice; the hybris PK article has the harder code (Java,
Groovy) but also a six-column table, which keeps it out of the checks
until tables are done. Rejected: the hybris PK article first, a better
test of the component that could not close the loop.

### How code asks for the component

**Chosen, by the user: it does not ask. The code block itself is the
signal.** Every code block becomes the component; Markdown already
tells code from prose with its fences, and the article's paragraphs
are never touched.

The road to it, each step rejected for a reason:

- **The comment `[//]: <> (...)`** proposed when the card was opened:
  pulldown-cmark reads it as a link reference definition and removes
  it from the text; worse, of several definitions with the same label
  (`//`) it keeps only the first (`firstpass.rs`, line 374, in
  pulldown-cmark 0.13.4), so a second comment would vanish.
- **A word after the language** (```` ```java responsive ````): the
  info string is the place CommonMark gives for such data, invisible in
  a Markdown preview, and pulldown-cmark passes it through. The user
  preferred it to the comment, "better follow the standard than invent
  our own". CommonMark defines only the place, though; the word itself
  stays our convention.
- **A word naming the component:** it ties every article to one
  implementation. The user chose a word describing the text instead,
  so articles do not change when the presentation does, which was the
  card's purpose.
- **A word describing the text:** what it would describe ("this code
  is wide") the build can measure by itself; written by hand it can
  drift from the truth. The only thing worth annotating was the
  difference between code and prose, which the fence already marks.

For tables the question stays open: Markdown has no place for such
data in a table.

### What wide code does

**Chosen, by the user: it scrolls sideways by default, and a "wrap"
toggle wraps its long lines;** a wrapped line continues 4 columns
deeper than its own indent.

Why, from a search of user-experience and accessibility sources (no
controlled study of code blocks on phones was found):

- WCAG 1.4.10 (reflow) forbids scrolling in two directions at 320
  pixels; code is not among its named exceptions. Technique G224 lets
  a code block pass if its lines fit, or wrap, or a mechanism lets
  them wrap, or its line breaks are meaningful to the language.
- Nielsen Norman Group finds sideways scrolling painful and content
  out of view often missed (on desktop pages).
- Practice is split: some projects wrap for accessibility, others
  (MDN) scroll because wrapped code reads worse; mixed solutions offer
  a per-block toggle.
- E-readers never scroll sideways: Kindle wraps code, badly by its
  readers' account; old Kobo models cut it off. Technical publishers
  keep code lines short when writing (an older O'Reilly style sheet:
  82 characters, 73 in the smaller books), still more than an iPhone
  SE shows: about 35 characters of the site's code font in a box.

The user judged code wrapped where it should not "rather illegible",
hence scrolling by default and wrapping on request.

How a wrapped line breaks was compared on lines of both articles at
iPhone SE width: by indent (as code editors do: VS Code's "Wrapping
Indent") and by grammar (as formatters such as google-java-format do).
Grammar reads better almost always; by indent breaks method chains in
the middle of a word. But grammar is computed for one width, and at 35
characters even grammar needs the indent fallback.
**Chosen, by the user: grammar with indent as fallback, in two slices:**
indent now, the grammar break points in a later slice, for the
languages of the articles. Rejected: a formatter running in the
browser, which needs a script and a formatter per language, and adds
weight to the page, a worry of the user's.

**4 columns of continuation**, chosen by the user, as Google's Java
style; none makes a continuation look like a new line (a method's
parameters looked like its body), 8 leaves about 19 characters on an
iPhone SE inside a method.

Rejected without a trial: a button opening the code full screen; the
toggle covers the need with no script.

### Where the toggle shows

**Chosen, by the user: only where needed,** that is when the block's
longest line does not fit its box. The build knows the line length,
the browser knows the screen; without a script they meet in a CSS
container query, one threshold per block. A spike (a short trial page,
measured in Chromium and WebKit at six widths and two text sizes)
showed the toggle visible exactly when the code overflowed, in all 36
cases per browser, and no overflow left once wrapped. What it taught:
the threshold is measured in the code font, as hoped; and `pre` takes
the browser's own font unless told to inherit the block's.

Two changes came with the code:

- **The threshold is written in `em`, not `ch`:** on a first visit,
  before the font is cached, Chromium did not evaluate `ch` again in
  the container query once the font arrived, and showed wrong toggles.
  The code font's characters are exactly half an em wide, so the
  result is the same and still follows enlarged text. The cost: the
  factor 0.5 belongs to the font IBM VGA 8x16; if the
  [look card](design-look-of-articles.md) changes the code
  font, the factor changes too, and the browser tests notice it.
- **ADR-001 has an exception:** the threshold cannot be one of our
  named styles, because Tailwind does not accept a value inside a
  container query's condition (it drops the style without an error).
  The block carries Tailwind's prefix with our style behind it,
  `@max-[66em]/code:wrap-shown`
  ([ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md), note of
  2026-11-04). Rejected: the Rust build writing a file of CSS rules,
  which would split the presentation between the stylesheet and the
  code; a CSS rule that detects overflow by itself ("scroll-state"),
  supported by Chromium but not by WebKit, that is Safari.

### Keyboard

**Chosen, by the user: the toggle is the keyboard's way in,** rather
than making every block a Tab stop, which would be useless on screens
where the block fits. Checked with axe in a second spike: with the
toggle outside the scrolling box, axe still reports a scrolling region
no keyboard can reach, and Chromium adds its own Tab stop while Safari
does not; with the toggle inside the box, sticky at its left edge, axe
reports nothing in both browsers. A keyboard user cannot scroll the
code with the arrows, but can wrap it and read it whole.

### The toggle's name

**Chosen, by the user:** `wrap` on screen; "Wrap long lines of code"
for screen readers, since "wrap" alone, heard out of context, does not
say what wraps (WCAG 2.4.6). The longer name contains the visible word,
so voice control users can still say "wrap" (WCAG 2.5.3).

### A known limit

When the reader widens the letter spacing (WCAG 1.4.12, simulated by
the readability test), code lines grow about 24% wider than the
threshold computed without it: the code may scroll while the toggle
stays hidden. Pointer and touch users still scroll it; a keyboard-only
user on Safari cannot. **Accepted, by the user,** as a narrow case;
alternatives belong to the
[look card](design-look-of-articles.md). Rejected: a
threshold 24% higher, which would show a toggle that does nothing in a
band of widths, against "only where needed". The readability test's
exception is therefore wider than planned: the code's scrolling box
may scroll sideways whether or not the toggle shows; the test of the
code blocks checks the toggle under normal spacing.

### The readability rule

The rule "nothing cut off by its own box" now lets the code's scrolling
box scroll sideways, with the reason in the test: line breaks are
meaningful in code (G224) and the toggle offers wrapping. Vertical
cutting and the page's own sideways scroll stay forbidden. One more
change was needed for the page to pass: a long word in a role heading
(`AfterSaveEventPublisherThread`) may now break, rather than push the
page sideways under the reader's spacing.

## Questions left

- Tables: the form of their annotation, since Markdown has no place
  for it, and what the component does on a phone and on a wide screen.
- The grammar break points for code, a slice of their own.
- The hybris PK article back in the readability and content-layout
  checks, which ends this card.
