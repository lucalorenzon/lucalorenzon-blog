# Improve the site's readability

- **Opened:** 2026-11-01, by the user, reading the published site. It
  joins two cards opened the same day, "space out the lines of text"
  and "make the text colours easier to read", and adds the font.
- **Status:** done on 2026-11-02, published in version 0.5.0. It also
  carried the checks for WCAG criteria 1.4.8 and 1.4.12, moved here on
  2026-11-01 from the card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md).

## Why

Reading the curriculum on the live site, the user found the text hard
to read: the lines sit too close together, and amber on black does not
help. While the line spacing was being discussed, the user asked to
consider the font too.

The three questions depend on one another, so they were joined in one
card:

- **The font sets the rules for the line spacing.** Sizes and spacings
  are multiples of 8px only because IBM VGA 8x16 is a pixel font; with
  another font that rule goes, and the right line spacing depends on the
  new font's proportions.
- **Font and colour together make the old-terminal look** chosen on
  2026-10-28
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md)):
  changing either one decides how much of that look is left.

The card on the WCAG criteria had been put before this one so
readability would be judged against criteria rather than impressions
only. Moving the two criteria here keeps that aim, and lets the rest of
the WCAG card come afterwards.

## What the stylesheet did before

In `style/input.css`, up to version 0.4.0:

- **Line height equals font size:** 16px on 16px, a ratio of 1.0, set
  on `html, body`. Paragraphs and list items are 8px apart (`mt-2`);
  lines inside a paragraph have no space between them.
- **Amber `#FFB000` on pure black `#000000`,** everywhere, lines
  included.
- **IBM VGA 8x16** for all text, served by the site itself.

## Why the build did not catch it

The build checks accessibility with axe, and no axe rule looks at what
the user noticed:

- **Criterion 1.4.8** (level AAA, visual presentation) asks for line
  spacing of at least 1.5 inside paragraphs, and space between
  paragraphs at least 1.5 times that. axe 4.14.0 has no rule for it.
  The build runs only the three AAA rules axe can check (enhanced
  contrast, links with the same text, no automatic refresh); the card
  [check accessibility in the build](../done/2026-10-30_check-accessibility-in-the-build.md)
  already says the site is not AAA, since most AAA criteria can only be
  judged by a person.
- **Criterion 1.4.12** (level AA, text spacing) does not ask the author
  for a wide line height: it asks that the page still works when the
  reader raises line height and spacing on their own. axe's only rule
  for it, `avoid-inline-spacing`, looks only at spacing written in an
  element's `style` attribute; the page has none, and the stylesheet is
  not examined.
- **Colours:** the only thing WCAG measures is contrast, and amber on
  black passes it at 11.46:1, above the 7:1 of level AAA. A bright
  saturated colour on pure black can still glare ("halation"), more so
  for readers with astigmatism; no WCAG criterion covers it.

## Known from the start

- **The header keeps its own line heights:** the title (24px or 32px),
  the text logo (32px) and the ASCII-art logo (1.0, which its drawing
  needs) set them in the stylesheet, so a change on `html, body` does
  not reach them. It reaches the text column, the roles in inverse
  video and the footer.
- **The footer is sticky:** a taller line makes it take more of a small
  phone's screen (320 by 568 pixels is the smallest tested).
- **Criterion 1.4.8 can be read two ways** for the space between
  paragraphs: as the empty space between them, or as the distance from
  the last line of one to the first line of the next. With a 24px line
  height that is 40px or 16px; today it is 8px.
- **Every colour is bound to 7:1** on normal text by the build's
  accessibility check.
- **History of the colour.** On the previous attempt of the site the
  user had ruled out colours other than white for accessibility
  reasons; amber was a deliberate try.
- **History of the font.** Rejected on 2026-10-28: the system's
  monospace font (different on every device), fonts loaded from Google
  Fonts (fines in Europe over visitors' privacy), Meslo LG and VT323
  (not the 1980s home-computer look).
- **The content layout test** already checks, at 12 screen sizes, that
  consecutive blocks do not overlap; a measure of the line height fits
  there.

## Answers

### Which text gets the new line spacing?

**The whole page (chosen by the user on 2026-11-01),** not only the
text column: the footer and any future page that does not use the
column start from the same line spacing. The header is not affected,
as above.

### One decision or three?

**One decision with three parameters (chosen by the user on
2026-11-01):** the best compromise between the font and its size, the
line spacing and the colour. When the cards were joined, the plan was
to split this one again when picked up (font, then line spacing, then
colour); the user withdrew that.

- **Why:** the parameters depend on one another, so separate cards
  would mean settling one, adjusting another, then changing the first
  again. The header's adaptation to screen width was handled the same
  way: logo and title were sized together in one card, not one after
  the other
  ([card](../done/2026-10-29_adapt-header-to-screen-width.md)).
- **Rejected:** three cards, font, line spacing and colour, one after
  the other, for the back and forth above.
- **Risk kept in view:** a large card. It is contained by comparing a
  few complete combinations side by side, rather than exploring each
  parameter on its own, as the header's values were chosen by looking
  at a playground page with several devices at once.

### Where do the checks for criteria 1.4.8 and 1.4.12 go?

**In this card, together with the choice (chosen by the user on
2026-11-01):** the checks land in the same commit as the new font, line
spacing and colour, with the build green.

- **Why:** the two criteria are the yardstick of this same decision.
  The line spacing chosen has to meet 1.4.8 and to hold up when the
  reader raises it, as 1.4.12 asks; separating the checks from each
  other, or from the choice they measure, brings back the same back and
  forth.
- **Rejected:** checks written first, in cards of their own. The page
  does not meet 1.4.8 today (line height 1.0, the criterion asks for at
  least 1.5), so the build would stay red until this card is done,
  blocking every publication meanwhile, and code would be committed that
  does not pass its own tests.
- **Rejected:** a check written first and marked as an expected failure,
  which Playwright allows. The build stays green and the yardstick
  exists before the choice, but the mark has to be removed, and
  remembered, once the choice is made.

### Which combination?

**Chosen by the user on 2026-11-02,** after trying combinations in a
playground page:

| Parameter | Value |
|---|---|
| Font of the text (column and footer) | Literata, 16px, regular, italic and bold |
| Font of the header | IBM VGA 8x16, unchanged, with its own sizes |
| Line height | 1.5 (24px) |
| Space between paragraphs | 36px |
| Space between list items | 12px |
| Letter spacing | none added |
| Width of the text column | 60ch (today 80ch) |
| Colours | `#DD9911` on `#000000` everywhere amber was used: text, lines, inverse video, footer; contrast 8.64:1 |

- **Why Literata.** Proportional fonts, with or without serifs, read
  much better than every monospace one tried, pixel fonts included: the
  difference comes mostly from letters of different widths, more than
  from serifs. Among the proportional ones the user found Literata the
  most relaxing to read. It was designed for long reading on screen,
  under the SIL Open Font License, so the site can serve it itself.
- **Why the header keeps IBM VGA 8x16.** The old-terminal look stays
  where it does not weigh on reading: the logo, the title and the amber.
  The font's rules (sizes in multiples of 8px) now apply to the header
  only. The user asked not to treat the look chosen on 2026-10-28 as
  fixed: it was a deliberate try, and the problem observed (the site
  reads badly) comes first.
- **Why 36px between paragraphs.** It meets both readings of criterion
  1.4.8 noted above: the empty space between paragraphs is 1.5 times the
  line height (36px), and the distance between their lines (60px) is
  more than that. Which reading to adopt no longer needs deciding.
- **Why 60ch.** Criterion 1.4.8 asks for at most 80 characters per line.
  With a proportional font a line holds more characters than the width
  of 80 zeros, which is what 80ch measures: at 80ch Literata reaches 94
  characters on wide screens, at 67ch exactly 80. How many characters a
  line holds depends on its words, so a width tuned to today's text
  leaves no room for the next one. At 60ch the longest line is 72
  characters, a margin of 8, and inside the 45 to 75 characters usually
  recommended for reading. Measured in ch, the column keeps the same
  number of characters if the text size changes; measured in pixels it
  would not.
- **Why `#DD9911`.** A softer amber than `#FFB000`, less glaring on
  black, still well above the 7:1 the build requires. The user's first
  try, `#CC8800`, was at 7.09:1, too close to that limit.
- **Criterion 1.4.12** holds with this combination: with line height,
  paragraph, letter and word spacing raised as the criterion asks, no
  text overlaps or is cut off on any of the five devices, from 320 by
  568 pixels to 1920 by 1080, and the footer takes at most 10% of a
  small phone's screen.

**Rejected:**

- **IBM VGA 8x16 and 9x16, Olivetti Thin 8x16 and 9x16** (pixel fonts
  from the same pack as today's): the letter shapes of old terminals,
  hard to read at length. Olivetti Thin's one-pixel strokes also make
  the text look dimmer than its measured contrast.
- **IBM Plex Mono, Atkinson Hyperlegible Mono, Courier Prime**
  (monospace, Courier Prime with serifs): less readable than any
  proportional font.
- **IBM Plex Sans, IBM Plex Serif, Atkinson Hyperlegible Next**
  (proportional): readable, but less relaxing than Literata.
- **A proportional Olivetti font:** none found with a licence that
  allows serving it. OT L22, from the Olivetti historical archive, is
  sold with no licence stated, and comes from a typewriter; the other
  revivals reproduce the logo, with personal-use or no-derivatives
  licences.

**How the combination was found.** A playground page, not kept in the
repository like the header's, showed the built site in five devices
(320 by 568 to 1920 by 1080) with controls for font, size, line height,
the two spacings, letter spacing, column width and colours, and a
switch applying the spacings of criterion 1.4.12. Every limit was shown
live: contrast, line height, both readings of the paragraph space,
characters per line counted on the rendered lines, horizontal scroll,
overlaps, cut-off text, the footer's share of the screen.

### What do the checks for criteria 1.4.8 and 1.4.12 measure?

**A browser test, `end2end/tests/readability.spec.ts`, on the twelve
screen sizes of the other layout tests** (written with the combination,
in the same commit, 2026-11-02):

- **1.4.8, visual presentation:** line height of every paragraph and
  list item at least 1.5 times its font size; empty space between
  consecutive paragraphs at least 1.5 times the line height (the
  stricter of the two readings); no rendered line longer than 80
  characters; no justified text.
- **1.4.12, text spacing:** with the reader's spacings applied (line
  height 1.5, letter spacing 0.12em, word spacing 0.16em, 2em after
  paragraphs), no horizontal scroll, no overlap between blocks of the
  text or between the parts of the header, no element cutting off its
  content, and the text still ending 32px above the footer.

- **Why characters are counted on the rendered lines:** with a
  proportional font only the lines as the browser breaks them tell how
  many characters they hold. The content layout test's old rule, "80
  characters of 8px", assumed the pixel font and was removed from it.
  The build test counts 70 characters on the longest line, the
  playground 72: the two count slightly differently; the margin under 80
  is at least 8 either way.
- **Shown to catch the problem:** with the line height put back to
  16px, the 1.4.8 test failed on all twelve sizes; with the column put
  back to 80ch, it failed on the nine sizes from 659px wide, finding a
  line of 94 characters.
- **Not tested:** the other parts of 1.4.8, colours chosen by the reader
  and text enlarged to 200% without scrolling sideways, which the
  criterion lets a "mechanism" such as the browser provide. They went
  back to the card
  [cover the WCAG criteria axe cannot check](../backlog/cover-wcag-criteria-axe-cannot-check.md)
  (chosen by the user on 2026-11-02): they do not depend on the font,
  line spacing and colour chosen here.

## Results observed

- **Published** in version 0.5.0 on 2026-11-02; the build's browser
  tests, the new readability test included, passed before and after
  publication, on the live site too.
- **The user's reading on the live site:** the text is much more
  readable. On wide screens, though, the 60ch column leaves much empty
  space at the sides. The text size is fine; the empty space is what
  weighs. That is a decision on the layout of the page, not on font,
  spacing or colour, so it went to a card of its own:
  [use the space beside the text on wide screens](../backlog/use-space-beside-text-on-wide-screens.md).
