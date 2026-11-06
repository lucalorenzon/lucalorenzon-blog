# Let the Markdown ask for components for wide code and tables

- **Opened:** 2026-11-04, by the user, while giving the articles a page
  of their own ([card](2026-11-04_add-article-pages.md)); in Now.
- **Status:** done on 2026-11-05. Wide code scrolls sideways with a
  "wrap" toggle; a wide table becomes a record viewer below a readable
  width; both article pages are back in the readability and
  content-layout checks, passing. No annotation is written in the
  Markdown: the build measures what it needs.

## Why

The two articles from Medium hold code with lines well beyond 80
characters and tables too wide for a phone. On Medium both scroll
sideways on a phone; the author remembers trying, without success, to
solve it at the time. On this site the readability test forbids both
a page that scrolls sideways and a box that scrolls inside itself
(counted as cut-off text), so the article pages were left out of the
readability and content-layout checks until this card was done.

The user saw a need that was bound to come: telling the presentation
layer how to show a piece of text, where Markdown alone is no longer
enough. Without putting HTML into the Markdown: that would mix
presentation with text, and split the presentation logic between the
layer meant for it and each article. The starting idea, the user's:
annotations in the Markdown, such as
`[//]: <> (markdown_code_responsive#start)`, naming a component of the
presentation layer; for code, scrolling sideways or a full-screen
window; for tables, splitting by columns or a full-screen window. The
slices below say where each idea ended.

What wide code and tables do on a phone moved here from the card
[give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md),
which kept font, colours and quotes. The user wanted the article pages
accessible and in as good a state as the curriculum before the
[navigation](../wip/list-articles-on-home-page.md) links to them.

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
languages of the articles. Once the box and colours of code were in
place, the user judged code good enough for now and moved the grammar
slice to a card of its own, in Later:
[break wrapped code lines by grammar](../backlog/break-wrapped-code-by-grammar.md).
Rejected: a formatter running in the
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
  [look card](../wip/design-look-of-articles.md) changes the code
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
[look card](../wip/design-look-of-articles.md). Rejected: a
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

## Slice 2: wide tables (2026-11-05)

### How a table asks for the component

**Chosen, by the user: it does not ask.** As for code, whether a table
is too wide is something the build can measure, so no annotation is
written in the Markdown, and the question of where a table could carry
one is closed. Two differences from code, raised by the agent, remain
for the slice's design: table text is in a proportional font
(Literata), so the build can only estimate widths, with a margin the
browser tests must confirm; and a table becomes unreadable well before
it overflows (six columns on a phone give one word a line), so the
threshold is a minimum readable column width, not only the point where
a word no longer fits.

**A cell with no space never wraps, chosen by the user,** rather than an
optional annotation giving each column a minimum number of characters,
which the user proposed after noticing that in the PK bits table, at
320 pixels, "63–44" breaks after the en dash (a break point for
browsers) in a first column 59 pixels wide, while the second column
has room to spare. The agent noted that the annotation would undo the
choice above and bring back the question of where a table could carry
it, and that the build can tell a single value from text by itself: a
cell with no space is one value. The annotation stays an idea, for
the day a real table shows the rule wrong; where to write it would be
decided then.

Built on 2026-11-05: the conversion wraps the content of such a cell
(header, data, or only inline code) in the style `one-line`, which
keeps it on one line; "63–44" stays whole at 320 pixels. The browser
test was first written for the PK bits table and its values; the user
found it wired to one article, so it became a general rule: on every
page of the built site, at phone widths, every cell whose visible text
has no space is on one line. The same weakness in the older tests went
to a card of its own,
[make the browser tests check behaviour, not articles](../backlog/make-browser-tests-check-behaviour.md).

### What a wide table does on a phone

**Chosen, by the user: a record viewer.** Two columns: the first holds
the header names, one a line, and stays still; the second shows one
record at a time, and one vertical gesture over it moves to the next
record as a whole. Every field is as tall as its tallest value across
all records, so labels and values stay aligned and the box never
changes height. **No sideways scrolling in tables, ever** (the user's
rule; code keeps its sideways scrolling). A gesture over the first
column scrolls the page.

The road to it:

- **Ideas found by the user on other sites:** a table that overflows
  is "pivoted". The agent named the two forms the word covers: the
  transposed table, records as columns scrolling sideways, and one
  block of "name: value" pairs a record, which never scrolls. The user
  wanted a third: transposed, but one record at a time, scrolling
  vertically.
- **Each field at the height it has in the shown record**, rejected by
  the user for the tallest-value rule: with the labels outside the
  scrolling box, keeping them aligned needs a script measuring the
  record on screen, and either the box keeps the tallest record's
  height, leaving empty space under shorter ones, or the text under
  the table jumps at every gesture.
- **A script is acceptable** to the user: Leptos was chosen to allow
  one, and this would be the site's first island, others are wished
  for. Without one is better, hence spike A; what an island weighs is
  spike B.

Noted by the agent and not settled: the record box takes the vertical
gesture from the page over about two thirds of a phone's width, so a
reader scrolling the article with a thumb in the middle may page
through records instead.

### Spike A: the record viewer without script

**Question:** can the record viewer above be built with HTML and CSS
only? Timeboxed to one hour; done in about sixteen minutes by a
Sonnet sub-agent, on a throwaway copy of the hybris PK article page
with its six-column table replaced, measured with Playwright in
Chromium and WebKit at 320, 360, 393, 412, 659 and 768 pixels, with
the text at its size and doubled.

**Answer: yes, with the swipe judged by the user on a real phone; the
mouse wheel in Chromium is the open limit.**

How it works: all the values of a field sit in the same grid cell,
invisible and stacked, so the row takes the tallest one's height by
itself; the labels sit in the same rows. A scrolling box over the
second column holds one pane a record, each with the same invisible
stacks, so its rows get the same heights; the box snaps one pane at a
time (`scroll-snap-type: y mandatory`, `scroll-snap-stop: always`) and
lets the page take over at the first and last record. Three changes
were needed: the box does not count in the outer row heights
(`container-type: size`), the label column has a fixed width with a
copy of each label invisible in the panes (a label wrapped under
doubled text otherwise misaligned the rows by 38 pixels), and the
scrollbar is hidden, so the panes are as wide as the cells they match.
A first row shows "1 / 5", the only sign without script that the box
pages.

**The record number, chosen by the user after trying it on the phone:
one row above the box, still and opaque, whose number changes at once.**
The first version put the number in each record's pane: it showed
through while the box moved, and once made opaque and sticky, the next
record's number was seen arriving from below. Now the number sits
outside the box, as the labels do; a strip of the five numbers moves
behind a one-line window, driven by the box's scroll position
(`scroll-timeline`, `animation-timeline`), in steps (`step-end`), so
it switches halfway between two records, where the snap decides, and
never slides. Checked in Chromium and WebKit at 320 and 393 pixels,
text at its size and doubled: the right number at every record and
halfway, alignment within half a pixel. Firefox does not run
scroll-driven animations by default, as far as known: there the page
falls back (`@supports`) to the number inside each pane, checked by
forcing the fallback in Chromium; Firefox itself was not tried. One
axe run in Chromium reported a contrast violation on one element while
the box stood exactly on a switch point; a run at rest reported none;
to be checked again on the real component.

What was observed:

- **Alignment:** labels and values within half a pixel, every record,
  width and text size, both browsers.
- **Gesture over the labels:** the page scrolls, the box does not.
- **Keyboard:** the box takes focus; arrows and page keys move one
  record and snap, both browsers.
- **axe:** no violations. Screen readers get one group a record
  ("Record 2 of 5") with name and value pairs; the invisible copies
  stay hidden. The data is no longer exposed as a table.
- **Mouse wheel:** WebKit moves one record a wheel step. Chromium
  snaps back to the same record with steps under half the box's
  height (about 160 to 200 pixels), so one notch of a real mouse may
  not move it; simulated steps, a real mouse was not tried.
- **Finger:** not reproducible in the automated browsers. The user
  tried the page on a phone, published as a private page on
  claude.ai, and found it works.
- **Doubled text at 320 pixels:** the label column is 128 pixels wide,
  "Description" breaks inside the word, and the box is taller than
  the screen.
- **Unrelated:** at 320 pixels with doubled text the hybris PK page is
  a few pixels too wide even without the table; to be found when the
  article goes back into the checks.

### When a table becomes a record viewer

**Chosen, by the user: when its columns no longer get a readable
width.** Each column needs the larger of its longest word (its whole
cell, for cells with no space) and the smaller of M and its content
on one line; the table becomes a record viewer when the text column is
narrower than the sum of these needs plus the cells' padding. Columns
of short values (YES, PRI) ask only for their own width; columns of
running text ask for at least M. **M is 18 characters,** in
Literata's average character width; once measured by the build, M
became 17 and header cells ask only for their longest word (see
[M, measured by the build](#m-measured-by-the-build) and
[Headers](#headers)). Rejected: switching only when the
table overflows, simpler and exact, but late: just above it the hybris
table has six columns with a word a line. The agent proposed both;
the user chose. Why this rule needs no imitation of how the browser
shares the width among columns: it uses only widths of words and of
lines, which the build measures well.

How M was chosen: by eye, not on paper. A Sonnet sub-agent rendered
the hybris table, with the no-wrap rule, beside the record viewer at
eight widths from 320 to 768 pixels. The text column has a maximum
width, so the choice came down to two cases:

- **Below 768 pixels** the column is at most 584 pixels wide, text at
  18 pixels; the table needs at least 589 (its no-wrap cells
  "varchar(255)", "currentValue" and the header "Description"), so it
  overflows at every such width, by 5 pixels at 659. The record viewer
  is forced there, whatever M; at 659 the user preferred it anyway.
- **From 768 pixels up** the column is 704 pixels at any screen width,
  text at 20 pixels, so the table always looks the same: "Description"
  gets about 18.6 characters a line and up to five lines a cell. The
  user judged it "slightly at the limit, but acceptable".

Hence M just under 18.6: the hybris table stays a table on tablets and
computers. With the no-wrap rule the two-column tables fit at 320
pixels, "63–44" on one line. The margin between M and 18.6 is small;
the browser tests must hold the hybris table as a table at 768 pixels
and as a record viewer just below.

### The copies of each value

The prototype of spike A was not kept in the repository; the agent
had it rebuilt from this card, so the user could read its code. The
user asked whether the same value is written several times. It is:
each value appears once in the table and, in the record viewer, once
in the label column's invisible stack and once in every record's
stack, two times plus the number of records (seven for the hybris
table: about 3,000 characters of text instead of 430). The stacks are
what lets the browser give each row its height without a script,
since the height depends on the screen width. The copies are hidden
from screen readers and compress well; they grow with the square of
the records. **Accepted, by the user,** after reading the code, as the
price of no script. A long table would reopen the choice of a Leptos
island: [handle long tables](../backlog/handle-long-tables.md).

### The label column

**Chosen, by the user: as wide as the longest label, at most 45% of
the viewer;** a label wider than that wraps with the language's
hyphenation. The build measures the labels in bold, adds the padding
and 0.1 em of margin, and rounds up to a tenth of an em.

The road to it:

- **A fixed width** in the rebuilt prototype (7rem) broke
  "Description" inside the word, with no hyphen, at normal text size.
- **A cap of 40%,** proposed by the agent without measuring: on an
  iPhone SE the viewer is 288 pixels wide, so 40% left about 97
  pixels for a word needing 110. The user chose 45%: 129.6 pixels,
  enough for "Description" with its padding.
- **The 0.1 em margin,** added by the agent: Chromium at a pixel ratio
  of 1 rounds each glyph to whole pixels, and "Description" broke by
  one pixel.

### M, measured by the build

To turn M into a width the build needs Literata's average character
width. The agent first defined it as the mean of the 26 lowercase
letters (0.555 em): the hybris table then needed 36.68 em and became
a record viewer even at 1920 pixels, while the card asks for a table
from 768 pixels, where the column holds 35.2 em. Running text is
narrower, because of spaces and frequent narrow letters: English
letter frequencies with a space every 4.7 letters give 0.476 em,
close to the hybris article's own text (0.475). With that average and
M = 18 the hybris table needs 35.25 em, one pixel too many at 768,
within the build's own estimate error.

**Chosen, by the user: the average of English text, and M = 17:** the
hybris table needs 34.777 em, about 8 pixels of margin at 768, and M
stays just under the 18.6 characters judged acceptable. Rejected:
M = 18, with the hybris table a record viewer on tablets held upright.
The average is computed by the build from the widths and the
frequency table, not written as a number, so it follows the font.

### Headers

**Chosen, by the user: a header cell asks only for its longest word.**
With the rule as it was, the "After Save Event" table became a record
viewer on an iPhone SE, eight records of two short fields: its header
"Event generated" counted as running text, so a column of values such
as CREATE asked for M. Headers are short bold labels that read well
on two lines. With this rule that table needs 15.357 em and fits at
320 pixels (16.89 em). Rejected: the rule as it was.

### Built on 2026-11-05

Every table is written in two forms in one box, a container named
`table`: the table as before and the record viewer of spike A; the
form not shown is hidden with `display: none`, so screen readers get
one. Rejected: one structure restyled both ways, since the viewer's
stacks are not a table. The threshold is a container query, as for
code (`@max-[34.777em]/table:viewer-shown`, an exception recorded in
[ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md), note of
2026-11-05); the label width is a named style with a number,
`viewer-label-width-72` (7.2 em).

Where the widths come from: no font library was available on the
machine, and none was added. A script in `end2end/tools/` loads the
font files in Chromium, with the Playwright the tests already use,
measures each character alone (kerning ignored) and writes
`src/literata_widths.rs`. Rejected: reading the font files at each
build, which needs a library to decompress woff2.

The site's tables: the PK bits table needs 12.83 em and the "After
Save Event" table 15.357 em, so both stay tables at every tested
width; the hybris table needs 34.777 em, a record viewer up to 767
pixels and a table from 768.

The readability rule now lets the viewer's box, and the one-line
window of its record number, hide content vertically: the box shows
one record by design, the others are reached by gesture or keyboard,
the number says how many there are, and screen readers get each
record as a group. Sideways cutting stays forbidden. New browser tests
check, on every built page with tables and at every tested width, one
form shown, labels and values aligned within half a pixel, the right
record number, the keyboard moving one record, and the hybris table
switching between 767 and 768 pixels.

One workaround: Chromium did not attach the record number's scroll
timeline inside a container query until the box's style changed once,
so the number stayed on "1 / n"; a 1 ms animation on the box makes it
attach. **The user tried the built article page on a phone,**
published as a private page on claude.ai, and found everything
working: the number, the gesture, the labels, the two-column table
staying a table, the viewer in landscape. Not tried: Firefox, which
should show the number inside each record instead, and a real mouse
wheel in Chromium.

## Slice 3: the hybris PK article back in the checks (2026-11-05)

The article joined the readability and content-layout checks, and the
code block test now checks its page width too. Everything passed but
one case.

**A known limit, accepted by the user as for code:** with the
reader's text spacing (WCAG 1.4.12), the hybris table in table form
is 770 pixels wide in a 704-pixel column at 768 and 810 pixels, and
the page scrolls sideways by up to 34 pixels. The build measures words
without that spacing, which makes them about 25% wider, so just above
the threshold the table cannot shrink below its unbreakable values
(varchar(255), currentValue). Above about 880 pixels the overflow
falls in the empty space beside the column. No content is lost, which
is what 1.4.12 asks. The text spacing test measures the page with the
tables in table form taken out; everything else, the record viewer
included, must fit. Rejected: counting the spacing in the threshold,
which would make the hybris table a record viewer at every width,
since the column never exceeds 35.2 em; letting single-value cells
break as a last resort, which CSS cannot ask for directly and which
risked bringing back "63–44" split in two.

Not reproduced: the spike's observation that the page was a few
pixels too wide at 320 pixels with doubled text. The check that
doubled the page's base size showed no change in the text at all,
because the stylesheet sets text sizes in pixels; that went to the
[WCAG card](../backlog/cover-wcag-criteria-axe-cannot-check.md) as a
question on 1.4.4.

## Left to other cards

- What a wide table does on a wide screen, where the space beside the
  text column stays empty:
  [use the space beside the text on wide screens](../backlog/use-space-beside-text-on-wide-screens.md).
- What a Leptos island weighs, shared part and component part, no
  longer needed by tables; **chosen, by the user:** measured with the
  first island the site needs,
  [measure what a Leptos island weighs](../backlog/measure-weight-of-a-leptos-island.md).
- A header that stays in view while a long table scrolls, and the
  record viewer's copies on a long table; **chosen, by the user:** a
  card in Later, [handle long tables](../backlog/handle-long-tables.md).
- Wrapped code broken by grammar rather than by width:
  [break wrapped code lines by grammar](../backlog/break-wrapped-code-by-grammar.md).
