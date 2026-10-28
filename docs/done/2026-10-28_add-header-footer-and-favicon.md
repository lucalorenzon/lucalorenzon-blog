# Give the page a header, a footer and a favicon, in a dark terminal style

- **Opened:** 2026-10-28, from the light pass of `residuality`
  ([card](2026-10-28_challenge-skeleton-with-residuality.md),
  stressor 7).
- **Status:** done, closed on 2026-10-28. Not published yet: it goes live
  with the next version.

## Why it was opened

The site exists to tie its content to its author. Until then a visitor
saw only "Sito in costruzione": the page could belong to anyone. Before
any article, the page had to say whose site it is.

The audience is mostly technical people who appreciate simplicity that
does not become trivial. The style chosen by the user, dark like an old
terminal, tells where the author comes from: a photo of them aged twelve,
in 1985, in front of a terminal in a server room. The photo explains the
style; it is not part of this card
([card](../backlog/place-1985-photo.md)).

Header, footer, style and favicon were kept in one slice, as decided by
the user: the style is a stylesheet and the favicon an existing file, so
they are not separate decisions.

## Answers

All settled with the user on 2026-10-28, one question at a time.

### Which logo, and what does a phone show?

**Chosen: the narrow ASCII-art logo of the old site** ("Lvk@73r", about
43 characters wide, from `old_main:src/components/logos/logo.rs`), on
every screen.

Rejected: the wide version (about 70 characters), which never fits a
phone; switching to the plain text "Lvk@73r" on small screens, as the
old site did. How the page adapts to screen size is a decision of its
own, moved to the card [make the site responsive](../wip/adapt-header-to-screen-width.md)
(later split; the logo is handled in its header half).

### What does the page show?

**Chosen, by the user:**

- browser tab: "Luca Lorenzon - Home Page";
- header: the logo on the left, "Home Page" centred, a horizontal line
  under it;
- body: "Sito in costruzione", centred;
- footer: a horizontal line, then "© Luca Lorenzon 2026", left-aligned,
  always at the bottom of the window.

"Home Page" was first at 16px; on seeing the page the user found it
illegible, so it went to 32px. The footer text was first centred; the
user moved it to the left. The logo and the footer text touch the left
edge of the window: offered a margin, the user kept them as they are.

Rejected: "Home Page" as the tab title alone, which would have dropped
the author's name from bookmarks and search results.

Decided by the agent, because each has one correct answer: "Home Page"
is the page's only main heading (`<h1>`), and "Sito in costruzione" a
paragraph; the logo is marked as an image named "Lvk@73r", so a screen
reader does not read its characters one by one; every file is referred
to by a relative path, so the page works under the current sub-path
address.

### Which colours?

**Chosen: amber `#FFB000` on black `#000000`,** lines in the same amber,
no other colour.

On the previous attempt of the site the user had ruled out colours other
than white for accessibility reasons, and wanted to try amber anyway. The
contrast, computed with the WCAG formula, is 11.46:1, above both
thresholds for normal text (4.5:1 for AA, 7:1 for AAA); white on black
is 21:1. To keep checking this at every build, the user asked for the
card [add automated accessibility tests](../backlog/add-automated-accessibility-tests.md).

Rejected: white on black, more contrast but not the old-terminal look the
user wants; green phosphor, not chosen by the user.

### Which font?

**Chosen: IBM VGA 8x16,** from VileR's "The Ultimate Oldschool PC Font
Pack" v2.2, web edition, served by the site itself
(`assets/fonts/`, with its licence and attribution). It is the text-mode
font of IBM PCs, the one 1980s bulletin board systems (BBS) were seen
with. It is a pixel font, so sizes are multiples of 16px.

Licence: Creative Commons BY-SA 4.0, which asks for attribution; the
licence and a README with the attribution are published next to the
font.

Rejected:

- the system's monospace font: nothing to download, but it looks
  different on every device;
- loading a font from Google Fonts: in Europe it has led to fines over
  visitors' privacy;
- Meslo LG, the user's first idea: a modern programming font, which does
  not look like the 1980s. (Menlo, the Apple font it comes from, may not
  be distributed at all);
- VT323, under the SIL Open Font License: it imitates a DEC terminal,
  less a home BBS than the IBM VGA font.

### Where do favicon and font live?

**Chosen: a new `assets/` folder,** copied as it is into the site by
`scripts/build-site.sh`. The favicon is the old site's file, unchanged.
Font and favicon are kept in the repository rather than downloaded at
build time, so the build does not depend on another website.

## Results

- Commit `a3b260a`: page, stylesheet, `assets/`, build script and tests.
  Eleven unit tests and the browser test pass. The browser test now also
  checks the heading, the text, the footer, the logo's name for screen
  readers, and that the font and the favicon are served.
- On a phone (375px wide) the logo fits, but "Home Page" wraps onto two
  lines next to it: left to the responsive card.
- The year in the footer is fixed in the code: it changes only when
  someone changes it.
- Publishing it makes the next version, which also carries the card
  [verify the publication at the next version](2026-10-28_verify-publication-at-next-version.md).
