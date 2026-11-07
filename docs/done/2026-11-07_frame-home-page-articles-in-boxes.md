# Frame each article on the home page in a box

- **Opened:** 2026-11-07, by the user, looking at the home page after
  version 0.10.0; in Now.
- **Status:** done on 2026-11-07. Each article on the home page sits
  in an amber box that leads to the article, with the date on its
  border; validated by the user on the real page. Not published yet:
  it goes out with the next version.

## Why

The home page lists the articles, newest first: for each one the title
as a heading, the description and the date
([card](../done/2026-11-06_list-articles-on-home-page.md)). The items
are kept apart only by the space between them. The user proposed
framing each item in a box, its border in the amber accent already used
for the lines of the page.

## Known from the start

- **The colour is already decided.** ADR-003 keeps amber for the lines
  and frames of the page; a box is a frame, so its border takes the
  tone of lines and frames, `accent-line` (`#dd9911` on the dark page,
  `#c06d0f` on the light page, 3.7:1 on paper-white, above the 3:1
  asked of graphics). The ADR lists the frame of code blocks among the
  uses of amber, not frames in general: the boxes were added to that
  list when this card was done
  ([ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md)).
- **What the list touches today:** the component in `src/home_page.rs`
  (an `ul` with the class `article-list`, one `li` per article), the
  `article-list` style in `style/input.css`, the unit tests in
  `src/home_page.rs` and the browser test `end2end/tests/home.spec.ts`.
  The readability test checks spacing between paragraphs, which the
  padding of a box may affect.
- **How the result is judged:** on the real page, at phone and laptop
  width, on the light and the dark background, with alternatives side
  by side before a commit; and with a layout test on a range of
  devices (no overflow, no overlap, text legible), not just the rule.

## The answers (2026-11-07)

### What the box is for a reader

**Chosen, by the user: the whole box leads to the article.** A box
reads as one object, and a reader expects to click it anywhere; a box
where only the title is clickable would separate the articles without
telling where to click.

**How, chosen by the user: the link stays on the title, and its
clickable area covers the whole box.** A screen reader reads only the
title as the link's name, as it does today, and a reader moving by
headings finds the same list as before. The cost: selecting part of a
description with the mouse becomes hard, since a click there follows
the link.

Rejected: the whole box inside the link. A screen reader would read
title, description and date as the link's name, and the heading would
sit inside a link, making the list of headings harder to follow.

### How the box shows it is active

**Chosen, by the user: the border thickens**, in the amber of lines,
both under the mouse and with the keyboard focus; the inner space
shrinks by as much as the border grows, so the text does not move. The
link's own focus outline is removed, since it would frame only the
title; the box shows the focus instead. The thicker border was chosen
on a trial from 1 to 3 pixels; the widths were then set with the
resting border (below): from 2 to 4 pixels. Between the active and the
resting box, the 2 pixels that turn amber contrast 3.7:1 on the light
page and 8.64:1 on the dark one, above the 3:1 asked of a focus
indicator; and a change 2 pixels thick is what WCAG's highest level
asks of a focus indicator's size. Compared side by side on the real
page, at phone and laptop width, light and dark.

Rejected:

| Alternative | Why rejected |
|---|---|
| A tinted fill, the grey of code blocks | The fill differs from the page by 1.18:1 on the light page and 1.21:1 on the dark one, far below 3:1; and on the light page the amber title on that grey falls to 3.9:1, which ADR-003 already rules out |
| A second amber ring, 3 pixels outside the border | Contrast as good as the thicker border; the user preferred the thicker border |

**On a touch screen the active state is never seen, and that is
accepted, by the user.** There is no hovering with a finger, and a tap
opens the article at once; on a phone the box tells it can be tapped
only by its resting look and by the habit readers have of tappable
boxes. The user noticed it in Firefox's phone simulation and judged it
the expected behaviour of a touch screen, not a defect.

### The border at rest

**Chosen, by the user: 2 pixels at rest, 4 when active.** On the light
page a 1-pixel border in the amber of lines (3.7:1) looked faint. With
2 pixels at rest, the active border goes to 4, so the change stays 2
pixels thick, as with the trial from 1 to 3; at 3 pixels the change
would have been 1 pixel, barely visible. Compared side by side at
phone width, at a phone's pixel density, and at laptop width at normal
and high density, light and dark.

Rejected:

| Alternative | Why rejected |
|---|---|
| 1 pixel at rest, 3 when active | Faint on the light page |
| 1.5 pixels at rest | Chromium paints it as 1 pixel, at normal and at high density, and the text moves by half a pixel when the box turns active, since the inner space is reduced for 1.5 pixels that are not painted |

### The corners

**Chosen, by the user: rounded, with an 8-pixel radius.** The curve
stays visible in both states: inside the border it is 6 pixels at rest
and 4 when active. The clickable area follows the corners. Compared
side by side at phone and laptop width, at a phone's pixel density,
light and dark, with the corner enlarged four times.

Rejected:

| Alternative | Why rejected |
|---|---|
| Square, like every other frame on the site | The user preferred the rounded corners |
| Rounded, 4-pixel radius | Barely visible at rest; when active the border is as thick as the radius, so the inside of the corner turns almost square |

The boxes are the first element of the site with rounded corners: the
header line, the frame of code blocks, the bar of quotes and the focus
outlines are square. **Accepted, by the user: the two amber frames,
boxes and code blocks, keep different shapes.** They never meet on the
same page: the boxes are on the home page, the code blocks in the
articles.

### The inner space

**Chosen, by the user: 0.75 times the text size** (13.5 pixels on a
phone, 15 on a laptop). With today's articles the inner space does not
change the number of lines of titles and descriptions; it changes the
air around the text, and each step of a quarter adds about 9 pixels to
the height of every box on a phone. Compared side by side at phone and
laptop width, light and dark, with the text width, the number of lines
and the height of the boxes measured.

Rejected:

| Alternative | Why rejected |
|---|---|
| 0.5 times the text size (9 pixels on a phone) | The text sits close to the border |
| 1 time the text size (18 pixels on a phone) | The user preferred the middle value |

### The space between boxes

**Chosen, by the user: 2.25 times the text size, as today** (40.5
pixels on a phone, 45 on a laptop). It was set equal to the space
between paragraphs when the items had no frame; with the boxes it no
longer has to separate them, and the readability test does not bind
it (it measures only paragraphs that follow each other, and none do
across two boxes). At this value the space between boxes equals the
space between the page title and the first box. Compared side by side
at phone and laptop width, light and dark, page title included.

Rejected:

| Alternative | Why rejected |
|---|---|
| 1 time the text size (18 pixels on a phone) | The user preferred today's value |
| 1.5 times the text size (27 pixels on a phone) | The user preferred today's value |

### Where the date sits

**Chosen, by the user: the date as a label on the bottom border line,
at the right**, like the legend of a fieldset. The line runs through
the middle of the date, at rest and when active; the label's
background is the page's, so the line breaks behind it; it stands 0.75
times the text size from the box's edge, clear of the rounded corner.
The date no longer takes a line of its own, so the boxes are shorter;
space is added below the description so the label does not touch it.
The date is moved only on screen, and stays after the description in
the page: on screen and for a screen reader it comes last, so the two
orders match. Compared side by side with the date inside the box at
phone and laptop width, light and dark, with the label enlarged three
times; axe found no violation on any of them.

The user first chose the label on the top border, at the left, then
changed their mind: there the date was the first thing seen in a box,
but a screen reader would have read it last, after the description.

Rejected:

| Alternative | Why rejected |
|---|---|
| The date inside the box, below the description, as before | The user preferred the label |
| The label on the top border, at the left | Seen first, read last by a screen reader |
| The label on the top border, at the right | The user preferred the bottom right |

Half of the label stands outside the box, and a click on that half
must open the article too. In the trials it did not: the clickable
area ended at the border. The build extends the clickable area by a
strip below the box, deep enough to cover the label; a click in that
strip, beside the label, opens the article too.

### The date's type

**Chosen, by the user: italic capitals, at 0.875 times the text size**
(15.75 pixels on a phone, 17.5 on a laptop), with letters spaced by
0.05 times their size. Its colour stays the text's (16.63:1 on the
light page, 16.83:1 on the dark one). Smaller and in capitals, the date
reads as a label and stands apart from the description. Compared in a
grid of three sizes and three styles at phone and laptop width, light
and dark, with the label enlarged three times; axe found no violation.

Rejected:

| Alternative | Why rejected |
|---|---|
| The text's size (1 time) | The user preferred a smaller date |
| 0.75 times the text size (13.5 pixels on a phone) | The user preferred 0.875 |
| Italic, without capitals | The user preferred capitals |
| Italic small caps | Literata, as the site serves it, has no small caps (its files carry no `smcp` or `c2sc` feature): the browser shrinks the capitals, with thinner strokes, and the text sits a little low on the line; at the smaller sizes it reads worst of the three, and ADR-003 puts readability first |

## What the build found

- **The space between boxes was lost, and nothing caught it.** The
  rule giving the boxes their 2.25em apart had the same weight as the
  text column's rule for every list item (0.75em), and won only by
  coming after it in the stylesheet. With more declarations in it,
  Tailwind wrote it before the column's rule, and the column's won:
  13.5 pixels on a phone instead of 40.5. The tests passed, since none
  measured the space; the screenshots showed it. The rule now wins by
  weight, whatever the order (`:is(ul)`, as the list's own reset
  already did), every rule of the list was checked for the same
  weakness (only this one had it), and a layout test on the range of
  devices measures the space between the boxes.
- **The last box needs room below it.** The date's label hangs about
  11 pixels below the last box, and the test that keeps 32 pixels
  between the text and the footer failed on short screens: the last
  box has 1 time the text size of space below it.
