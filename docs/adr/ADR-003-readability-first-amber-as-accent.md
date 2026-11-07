# ADR-003: Readability first, amber as an accent, light or dark background

- **Date:** 2026-11-05
- **Status:** Accepted. Corrected on 2026-11-05, the same day: code is in
  Hack, not Meslo LG, whose licence turned out unclear (see Alternatives).
  Completed on 2026-11-05, when the light background was built: the
  default for readers with no preference, and the light shades of amber
  (see Consequences and Technical notes). Extended on 2026-11-07: the
  markers of lists, bullets and numbers, join the uses of amber (see
  Decision and Technical notes;
  [card](../done/2026-11-07_show-numbered-lists-and-colour-list-markers.md)).
  Extended again on 2026-11-07: the frame of the boxes that hold the
  articles on the home page joins the uses of amber (see Decision and
  Technical notes; [card](../wip/frame-home-page-articles-in-boxes.md))
- **Card:** [give the articles' code, quotes and tables a look](../done/2026-11-06_design-look-of-articles.md)

## Context

On 2026-10-28 the site took the look of a 1980s bulletin board system
(BBS): amber text on black, in the pixel font of the IBM PC, as on the
terminal the author sat at in 1985
([card](../done/2026-10-28_add-header-footer-and-favicon.md)). The look
was chosen for identity: it tells where the author comes from.

Readability has been giving way ever since. On 2026-11-02 the text moved
to Literata, a font made for reading on screens, and the amber was
toned down because it glared
([card](../done/2026-11-02_improve-site-readability.md)). On 2026-11-04
code turned off-white. Reading the long technical articles, the user
found the text too small and the BBS look too tight a fit: a desire for
identity had made them underrate readability. The site had become a
half-way house, with no rule saying which part of a page carries the
identity and which part serves reading.

The user settled the question this way: what should be recognised as
theirs is the content, the care for detail and the way of
communicating. The look has one job, letting that content be read.

## Decision

1. **Readability comes first.** A style choice is judged by whether it
   helps the reading; identity lives in the content and in a few signs
   around it.
2. **Amber is an accent, not the colour of the text.** It is kept for the
   logo, the page title, the lines, the frame of code blocks, the bar of
   quotes, code inside a sentence and, since 2026-11-07, the markers of
   lists (the bullet or the number, never the item's text) and the frame
   of the boxes that hold the articles on the home page. The text is off-white on black, or
   near-black on a paper-white background.
3. **Each font has one job, and all are served by the site:**
   - the logo keeps the pixel font, IBM VGA 8x16, the one sign of 1985
     left in the frame;
   - text and page title are in Literata;
   - code is in Hack, a monospaced font made for screens.
4. **The reader chooses a light or a dark background.** First through the
   preference the reader already set in their browser or system, with no
   script; a control on the site belongs to the card on
   [letting readers adjust font and spacing](../backlog/let-readers-adjust-font-and-spacing.md).

Why: the user compared, on the real article page at phone and laptop
width, the page of today, a BBS frame around a reading page, a neutral
page in light and in dark, and an amber accent with Literata everywhere.
A neutral page read best but lost every link to the author's story; an
amber accent turned out to sit well on both backgrounds. Literata
everywhere, logo included, was too generic, so the logo kept its pixel
font. A monospaced font made for screens replaced the pixel font for
code, which reads small and cramped next to Literata: Meslo LG in the
trials, then Hack, its close relative, with a clear licence and a
small file.

## Consequences

**Positive:**
- Every later style choice has a rule to be judged by.
- The text reads as a book page; the amber and the pixel logo still say
  where the author comes from.
- Readers who prefer a light page get one, and the same for dark.

**Negative / Risks:**
- Two backgrounds mean two sets of colours to keep within the contrast
  the site checks. On the paper-white background the amber of today
  falls short, so darker shades of it are needed (see Technical notes).
- A browser where the reader set no preference reports "light", so many
  readers will see the light page. A stylesheet cannot tell the two
  apart; when the light background was built the user chose to follow
  the preference at once, so most readers see the light page.
- One more font file to serve, Hack's, on top of Literata's: 23 KB.
- Code inside a sentence and the text no longer differ in colour on the
  dark page unless code stays amber: hence the amber for it.
- Parts of earlier choices are superseded: the amber text and the pixel
  title of 2026-10-28, the amber text of 2026-11-02, and the request of
  2026-11-04 for article headings in a BBS style, kept only where it
  also helps the reading.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Keep the BBS look everywhere | Pixel font and amber text read poorly in long articles: the reason readability was already reworked on 2026-11-02 |
| Keep today's half-way look (Literata in amber on black) | Amber text glares on black; no rule on which part of the page carries the identity |
| A BBS frame (pixel title, amber) around a reading page | Reads as well as the neutral dark page; the user preferred the amber accent with a Literata title, which also works on a light page |
| A neutral page, light or dark, without amber | Reads best, but loses every link to the author's story; the user found an amber accent does not hurt the reading |
| Literata everywhere, logo included | Too generic; the logo drawn in characters needs a monospaced font and would be lost |
| Times New Roman and Courier New, not served by the site | Not on every device (missing on Android and most Linux systems, where the browser substitutes another font); Times New Roman was made for narrow newspaper columns, with thin strokes that thin out on a screen; Courier New is among the thinnest monospaced fonts and reads poorly on screen |
| The system's own monospaced font for code | Different on every device, and the "wrap" toggle of code blocks depends on the width of the letters, which changes from font to font |
| Meslo LG for code, the font of the trials | Its files carry only the copyright of Apple, Bitstream and Tavmjong Bah, with no licence; its author writes it is a modified Apple Menlo that he cannot make open source. Serving it from a public site would redistribute Apple's work without a written permission |
| DejaVu Sans Mono for code, the ancestor of Menlo and Meslo | Looks the same and has a clear licence, but the web file at hand (Fontsource, latin) weighs 140 KB, more than Literata's four files together (84 KB); Hack, derived from it, offers a web subset of 23 KB |
| The pixel font for code | Reads smaller and more cramped than Literata at the same size; nobody chose it for code, it came with the BBS look |
| One background, chosen by the author | The user wants the reader to choose |

## Technical Notes

- **Contrast**, computed with the WCAG formula. On black (`#000000`):
  amber `#dd9911` 8.64:1, off-white `#e6e6e6` 16.83:1. On paper-white
  (`#fbfaf7`): amber `#dd9911` only 2.33:1, so the same hue darkened:
  `#c0850f` 3.05:1 for lines and frames (3:1 is asked of graphics),
  `#986a0c` 4.57:1 for logo and title (large text, 4.5:1 at the level the
  site checks), `#714e09` 7.2:1 for code inside a sentence (normal text,
  7:1). The light shades are confirmed when the light background is
  built.
- **The light shades as built** differ from those above. Those held
  only on the paper-white: on the code surface code inside a sentence
  fell under 7:1. Darkened enough, the amber read brown, and the user
  chose a redder hue, 32° on the colour wheel against the 40° of
  `#dd9911`, with the same saturation: `#c06d0f` for lines and frames,
  `#a9600d` for logo, title and headings, `#6f3f09` for code inside a
  sentence, on a code surface of `#ebe7df` and a text of `#1c1a17`.
  Contrasts and the trials:
  [card](../done/2026-11-06_design-look-of-articles.md), "The light background".
- **The markers of lists** (added on 2026-11-07) follow the same tones:
  a bullet is a graphic, so it takes the tone of lines and frames
  (`accent-line`); a number is normal-size text, so it takes the tone
  of code inside a sentence (`accent-code`). On the light page the two
  shades differ; on the dark page both are `#dd9911`. Why and the
  options rejected:
  [card](../done/2026-11-07_show-numbered-lists-and-colour-list-markers.md).
- **The boxes of the home page** (added on 2026-11-07) are a frame, so
  their border takes the tone of lines and frames (`accent-line`): 2
  pixels at rest, 4 under the mouse or with the keyboard focus. Their
  corners are rounded (8 pixels), unlike the frame of code blocks,
  which stays square; the two never meet on the same page. The date on
  the border keeps the colour of the text, not amber. Why and the
  options rejected:
  [card](../wip/frame-home-page-articles-in-boxes.md).
- **Hack** (version 3.003, npm package `hack-font` 3.3.0): MIT License
  for the Hack project, public domain for DejaVu's work, Bitstream Vera
  License for Bitstream's; the web subset made by the Hack project,
  used as distributed (`assets/fonts/hack/README.md`). The trials had
  used the "Nerd Font" build of Meslo LG installed on the author's
  computer; the first version of this record called Meslo LG's licence
  Apache 2.0, as the Nerd Fonts project presents it, without checking
  the author's own statement.
- **The "wrap" toggle of code blocks** counts each character as half an
  em, the width of IBM VGA 8x16 (`src/markdown.rs`, `style/input.css`);
  with Hack it becomes 1233/2048 em, Hack's advance width.
- **Weight:** Literata's four files are 84.4 KiB, downloaded once and
  then kept by the browser; the text shows at once in a fallback font
  while they arrive.
- **Text size** goes up on every page, the curriculum included; the
  value is chosen in its own slice (the trial used 18 pixels on a phone
  and 20 on a laptop).

## References

- Card: [give the articles' code, quotes and tables a look](../done/2026-11-06_design-look-of-articles.md)
- Earlier choices: [header, footer and favicon](../done/2026-10-28_add-header-footer-and-favicon.md),
  [site readability](../done/2026-11-02_improve-site-readability.md)
- Related ADRs: [ADR-001](ADR-001-name-styles-by-visual-variant.md)
  (styles named by visual variant, which the new styles follow)
