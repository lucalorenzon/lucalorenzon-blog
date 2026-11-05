# Give the articles' code, quotes and tables a look

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Next. Narrowed on 2026-11-04,
  when what wide code and tables do on a phone moved to the card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md).
- **Status:** in progress since 2026-11-04.

## Why

The articles from Medium bring three kinds of content the site has no
look for: code blocks, notes in quotes and tables. Until this card is
done, the [article pages](../done/2026-11-04_add-article-pages.md) show them with the
default style. The user wants the article pages accessible and in as
good a state as the curriculum before the navigation links to them, so
this card comes before the [navigation](../backlog/add-site-navigation.md).

## Known from the start

- **Code blocks:** Java, XML, Groovy, SQL and properties. Undecided:
  font and colours. What long lines do on a phone belongs to the card
  on wide content.
- **Since 2026-11-04 code blocks are a component** (card on wide
  content, slice 1): they scroll sideways and show a "wrap" toggle
  when a line does not fit. Two things for this card. The toggle's
  threshold counts each character as half an em, the width of IBM VGA
  8x16 (`src/markdown.rs`, `style/input.css`): a new code font means a
  new factor. And when the reader widens the letter spacing, code can
  scroll with the toggle hidden, out of reach of a keyboard-only user
  on Safari: accepted for now, alternatives to look for here.
- **Quotes:** both articles open a note or a warning with a blockquote.
- **Tables:** "After Save Event" has one (the transaction cases), the
  article on hybris PK values two (the bits of a key and the
  `numberseries` table). Their look is here; what a table too wide for
  the screen does belongs to the card on wide content.
- **Rules already in force:** the readability rules and their test
  ([card](../done/2026-11-02_improve-site-readability.md)), and the
  contrast the WCAG criteria ask for.
- **"Done" means** the automatic checks pass on the article pages, as
  they do on the curriculum, and the user validates the pages in the
  browser.

## From the user's first reading of the article pages

On 2026-11-04 the user read the two article pages, built with the
default style, before closing the card that gave them a page
([card](../done/2026-11-04_add-article-pages.md)), and asked for the
following; the card was closed and these moved here.

- **The text is too small,** Literata included, "on all the text". On
  2026-11-02 the user had judged the size right
  ([card](../done/2026-11-02_improve-site-readability.md), results);
  long technical articles changed that reading. Open: whether the size
  changes for the articles only or for the curriculum too.
- **Code in a box with a visible border,** as on Medium: a styled
  block, not a form field. The minimum for code. Today code uses the
  site's monospace font, IBM VGA 8x16, the logo's pixel font, which no
  one chose for code and reads smaller than Literata at the same size.
- **Headings of articles, in the spirit of an 1980s BBS text:** section
  titles (`h2`) in inverse video like the curriculum's roles, and
  larger; sub-titles (`h3`) bold only; both set apart by blank lines.
  Today an article's `h3` takes the style of the curriculum's roles
  and weighs more than the section titles.
- **No line above the sections:** the amber line is out of place in an
  article; blank lines and bold titles instead.
- **For the articles only:** a curriculum is not an article, the
  things to highlight differ. The curriculum keeps its headings and
  its section lines.

## Slice 1: a box for code (2026-11-04)

Seeing the first slice of the card on wide content at work, the user
found code blocks "indistinguishable from the rest of the text": amber
code on the page's black, with no edge. The box this card already
named as the minimum for code came first, in a slice of its own. A
side effect: with a visible right edge, a line cut by the box reads as
continuing, which the scrolling code needed (Nielsen Norman Group:
content out of view is often missed).

**Chosen, by the user: off-white code (`#e6e6e6`) on a dark grey
(`#1a1a1a`), in a 1-pixel amber frame with an 8-pixel inner margin.**
The user compared, on the real page at iPhone SE width, the frame
alone on black and three backgrounds with the amber code, then four
backgrounds with off-white code.

- **Amber code on a lighter background** was the first idea. The site
  checks enhanced contrast (WCAG 1.4.6, at least 7:1), and the amber
  allows little: 7.16:1 on `#1a1a1a`, 7.58:1 on the amber `#1a1306`,
  6.55:1 on `#222222`, which fails. Grey and amber backgrounds of the
  same lightness were compared too, since the user's first two
  favourites differed in lightness more than in hue. Rejected: the
  background could barely leave black, and the margin over 7:1 would
  be thin for the font and size changes this card may still make.
- **Off-white code** sets code apart at a glance, as in a terminal,
  with a contrast of 13.9:1 that leaves room for those changes, and a
  neutral base for syntax colours if they ever come. Off-white rather
  than pure white, which smears on a dark background for readers with
  astigmatism. The cost: the site gets a second text colour besides
  amber. The frame and the "wrap" toggle stay amber; the toggle's
  label, amber on `#1a1a1a`, keeps 7.16:1.
- **Rejected: the lighter backgrounds** `#2a2a2a` and `#33260c` with
  off-white code, which stand out more; the user chose the darker
  grey.

Left open by this slice: code inside a sentence (`advanced.properties`)
stays amber with no background, so it no longer matches the blocks.

## Slice 2: code inside a sentence (2026-11-04)

**Chosen, by the user: off-white too,** the same `#e6e6e6` as the
blocks, with no background, so code reads as code wherever it is.
Contrast on the page's black: 16.8:1. Rejected: amber as before, which
matched the prose and no longer matched the blocks.

## A new direction for the whole site (2026-11-05)

Asked whether the bigger text was for the articles only, the user
answered: for the curriculum too. They then questioned the BBS look
itself: a desire for identity had made them underrate readability,
and a page in the style of a Kindle might suit the articles better
than a 1980s BBS. After comparing trials on the real article page, the
user set the direction recorded in
[ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md):
readability first, identity in the content; amber as an accent; the
logo in the pixel font, text and title in Literata, code in Hack (first
Meslo LG, until its licence turned out unclear);
a light or a dark background chosen by the reader.

What it changes in this card:

- **The text size changes on every page,** the curriculum included.
- **The headings asked on 2026-11-04 in a BBS style** are kept only
  where they help the reading. The trials showed section titles in
  bold Literata, larger, with no line above them, and sub-titles in
  bold: no inverse video.
- **Code inside a sentence turns amber** (slice 2 had made it
  off-white), so it still differs from the text, now off-white too.
- **Code blocks move to Hack,** which settles the font this card
  had left undecided, and changes the factor of the "wrap" toggle's
  threshold.
- **The card now carries the site-wide slices** that came out of it,
  not only the articles' look.

Slices left, one decision each, in this order:

1. Colours on black: off-white text; amber for logo, title, lines,
   frames and code inside a sentence; the title in Literata.
2. Text size, with spacing that follows it, on every page.
3. Code in Hack, served by the site, with the toggle's threshold
   updated. Joined with slice 2 when done.
4. Headings of the articles, with no line above the sections. Done for
   the curriculum too.
5. Quotes, with an amber bar on the left. Done.
6. Tables, with thin lines. Done, with stripes on alternate rows.
7. The light background, shown when the reader's browser prefers light
   pages, with darker shades of amber. Done, the shades redder.
8. The licence note at the end of an article, added by the user on
   2026-11-05 after the tables: its text smaller, being a note,
   perhaps the footer's size; some space between the article's end and
   the line above the note, today none; the line itself is off-white,
   unlike the site's other lines. Today the note is in the article's
   Markdown, a thematic break (`---`) and a plain paragraph, which the
   stylesheet does not style: Tailwind's reset gives the line the text's
   colour and no margin.

## Colours on black (2026-11-05)

The first of the slices left. Text and footer off-white (`#e6e6e6`,
16.83:1 on black); amber (8.64:1) for the logo, the page title, the
lines, the code frames, the curriculum's roles and code inside a
sentence; the page title in bold Literata, still amber. The user saw
the result on the curriculum and both articles, at iPhone SE and
laptop width, before the commit.

Three technical adjustments, decided by the agent as they change
nothing with the default settings:

- **Title line height 1.5,** so the title still starts 24 pixels under
  the header line: Literata's letters take more room above them than
  the pixel font's.
- **On wide screens the logo's column follows the logo's real width,**
  with a 24-pixel gap, instead of a fixed 368 pixels. Under the
  reader's own text spacing (WCAG 1.4.12) the logo drawn in characters
  grows, and the bold Literata title, a little wider than the pixel
  one, touched it.
- **The header test accepts a title on two lines** in the header row,
  as the agreed layout already allowed, measuring its distance from the
  logo on the title's column.

## Text size and code font (2026-11-05)

Slices 2 and 3 of the list above, joined in one commit by the user.

**Chosen, by the user: text at 18 pixels on a phone and 20 from 768
pixels up, the footer at 16 and 18, code in Hack at 85% of the text.**
The user compared, side by side in the browser, 16, 17/18, 18/20 and
19/21 pixels; preferred 19/21 but feared it set the text too far from
code and footer, still at 16 pixels. Shown 18/20 and 19/21 again with
code at 85% and the footer at 90% of the text, the user chose 18/20:
at 19/21 the footer took two lines on an iPhone SE. Spacing now
follows the text, in em, at the same ratios as before.

- **Why one commit.** With the text at 20 pixels and code still in the
  narrow pixel font at 16, a line of "After Save Event" held 82
  characters, over the 80 the readability check allows. The code's
  size had been chosen together with the text's, so the two slices
  could not stand apart. Rejected: code inside sentences at the text's
  size in the pixel font for one commit, blurred at 18 and 20 pixels.
- **Hack, not Meslo LG.** Meslo LG's licence turned out unclear, and
  DejaVu Sans Mono's web file weighs 140 KB; Hack, derived from DejaVu,
  has a 23 KB web subset with every character of the site's code
  ([ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md),
  corrected the same day).
- **The "wrap" toggle** now counts each character as 1233/2048 em,
  Hack's advance width, rounded up so the toggle may show a hair early,
  never late.
- **A long name in code inside a sentence may break** where it does
  not fit, as headings already may: under the reader's own text
  spacing, `AfterSaveEventPublisherThread` overflowed the 320-pixel
  screen. Decided by the agent, as nothing changes with the default
  settings.

## Headings (2026-11-05)

**Chosen, by the user: section titles (`h2`) in bold amber, a little
larger (1.4 times the text); sub-sections and the curriculum's roles
(`h3`) in bold italic amber, a little smaller (1.1 times); no line
above the sections, no inverse video; the same on every page,
curriculum included.** The underlined sub-headings (`h4`, "Main work")
stay as they were. Amber on black: 8.64:1.

The user first asked to try the curriculum too, reversing for a trial
the choice of 2026-11-04 that it keep its lines and inverse video, and
compared on the real pages at phone width:

- **Today's headings.** Rejected: the line and the inverse video were
  the BBS look; in the articles a sub-section weighed more than its
  section.
- **Bold headings in off-white, no lines.** Rejected: in the
  curriculum a section and a role, both bold off-white and close in
  size, hardly stood apart.
- **Section titles in amber, roles in bold off-white.** Chosen first;
  the user then asked for the sub-titles in amber too, a little smaller
  and in italic.
- **Sub-titles in regular italic.** Rejected: the role weighed little,
  and looked like the italic line below it (company and dates).

As the curriculum takes the same headings, the article styles need not
be kept apart from the curriculum's, as this card had planned: one set
of rules serves both. The content-layout test checked the line above
sections and the inverse video; it now checks that section titles and
sub-titles differ in colour from the text, the same intent, with the
contrast still checked by axe.

## Quotes (2026-11-05)

**Chosen, by the user: a 4-pixel amber bar on the left, the text
indented 1em from it, in the same off-white as the rest, with no
background and no italic;** the quote starts on the column's edge and
sits in the flow like a paragraph. The user judged it simple enough to
go without a new trial, having seen it in the trials of the direction.
The agent checked it at iPhone SE width on both articles, the hybris PK
quote holding a list among its paragraphs.

## Tables (2026-11-05)

**Chosen, by the user: thin amber lines above the table, under the
header and at the bottom, and between the columns, with no line on
the outer sides; rows in stripes, the page's black alternating with
the dark grey of the code blocks (`#1a1a1a`); the header in bold.**
The table fills the column's width, its cells have a margin of half a
line, and it sits in the flow like a paragraph. The stripes came from
the user, before the trials; with them, lines between the rows are no
longer needed. The user compared, on the real pages of both articles
at iPhone SE and laptop width, served locally:

- **A full grid,** a line around every cell. Not chosen.
- **A single line under the header,** the stripes alone parting the
  rows. Not chosen.
- **Three lines,** above, under the header and at the bottom, as in
  the tables of books and papers. The user asked for it with the lines
  between the columns, which became the choice.
- **Lines between the columns thinner or dimmer:** half a pixel of
  amber, or amber at 50% (`#6e4c08`, 2.7:1 on black) or 40%
  (`#583d07`, 2.09:1). Rejected by the user: no added value, and half a
  pixel shows thinner only on high-density screens; on others the
  browser rounds it to one pixel or drops it.

**Why the stripes are not lighter.** The `numberseries` table holds
code inside a sentence, in amber, which keeps 7.16:1 on `#1a1a1a` but
falls to 6.55:1 on `#222222`, under the 7:1 the site checks. The text,
off-white, keeps 13.94:1 on the lighter stripe.

What a table too wide for a phone does, as `numberseries` with its six
columns, stays with the card on wide content.

## The light background (2026-11-05)

**Which readers get it.** A stylesheet cannot tell a browser with no
preference from one that prefers light pages: both report "light". So
following the reader's preference means most readers see the light
page. **Chosen, by the user: follow the preference now.** Rejected:
keep the dark page for everyone until the site has a control of its
own ([card](../backlog/let-readers-adjust-font-and-spacing.md)), which
would postpone the light page to that card.

**Chosen, by the user: a paper-white page (`#fbfaf7`) with near-black
text (`#1c1a17`, 16.63:1), code blocks and odd table rows on a beige
grey (`#ebe7df`), and an amber turned towards red,** a burnt orange in
three shades, one per role:

| Role | Shade | Contrast |
|---|---|---|
| Lines and frames, focus outlines | `#c06d0f` | 3.7:1 on the page, 3.13:1 on the code surface (3:1 asked of graphics) |
| Logo, page title, headings: large text only | `#a9600d` | 4.61:1 on the page (4.5:1 asked of large text) |
| Code inside a sentence, the "wrap" label | `#6f3f09` | 8.42:1 on the page, 7.12:1 on the code surface |

The dark page keeps the amber of today, `#dd9911`, for every role.

The user compared, on the curriculum and both articles at iPhone SE
and laptop width, served locally next to the dark page of today:

- **The shades computed in ADR-003, of today's hue.** They held only on
  the paper-white: on any code surface code inside a sentence fell
  under 7:1 (`#714e09`, 6.49:1 on `#f1eee8`), so the trials used shades
  a little darker, computed for each surface. Darkened that much, code
  inside a sentence read brown rather than amber. The user asked for a
  darker amber with a touch of red.
- **Two redder hues,** 32° and 24° on the colour wheel against today's
  40°, with the same saturation, each with its three shades computed
  for the same contrasts. 32°, a burnt orange, chosen; 24° read as
  terracotta, with code inside a sentence brick red.
- **Two code surfaces,** `#f1eee8`, quieter, and `#ebe7df`, which stands
  apart from the page about as much as the dark page's grey stands
  apart from black. The second chosen.

**The drawn logo is left out of the contrast check.** From 768 pixels
up the logo is drawn in 16-pixel characters, normal text for the
contrast rules, which ask 7:1; in the heading shade it reaches 4.61:1
on the light page. WCAG 1.4.3 and 1.4.6 exempt text that is part of a
logo from contrast, and the drawn logo is already hidden from screen
readers, but axe cannot know it is a logo. **Chosen, by the user: the
accessibility test drops the contrast findings on the drawn logo
only;** every other rule still checks it, and the 32-pixel logo of
narrower screens is still checked for contrast. Rejected: the drawn
logo in the shade of code inside a sentence (`#6f3f09`, 8.42:1), which
passes with no change to the test but sets the logo darker and browner
than the title beside it.

Decided by the agent, as it changes nothing a reader sees: the
browser tests run twice, once on each background. Before, they ran
only as a browser with no preference, so the light page alone would
have been checked; run on both, the whole suite takes about 13
seconds, so running it twice costs little and covers both.

Left open: on the light page the drawn logo, though within the rules,
is hard to read. The user left it as it is in this slice and moved it
to a card of its own.

Also left open: the two backgrounds now have accents of slightly different
hue. Turning the dark page's amber towards red too would be a decision
of its own.

## Questions

- How the page knows the licence note is a note (slice 8): in the
  Markdown it is only a paragraph after a thematic break.
