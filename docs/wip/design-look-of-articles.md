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

## Questions

- Code inside a sentence: off-white like the blocks, or amber as now.
- The look of each kind, within the readability rules.
- The text size, and whether it changes for the curriculum too.
- How the article styles are kept apart from the curriculum's, which
  share the `long-text-column` rules today.
