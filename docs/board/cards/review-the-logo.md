# Review the logo

- **Created:** 2026-11-05
- **Status:** Closed
- **Closed:** 2026-11-22

## Why

Opened by the user on 2026-11-05, while building the light background in
the card [give the articles' code, quotes and tables a look](design-look-of-articles.md),
and placed in the plan's Now at the time.

From 768 pixels up the logo is drawn in characters, in the pixel font
at 16 pixels. On the light page the user found it illegible. The
drawing is not made of letters but of slashes, underscores and
backslashes (`/ _ \ |`): on black the light strokes merge into the
shape of the letters, on paper-white the dark strokes stay separate
lines, and the eye struggles to read them as letters. Below 768
pixels the logo is the name written in the same pixel font at 32
pixels, "Lvk@73r", and reads well on both backgrounds.

The light background was committed with the drawn logo as it was, so
this card could be decided on its own.

Taken into work on 2026-11-22, before the page of the frame in the
style guide
([show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md)),
since the logo's change at 768 pixels is one of the two things that
page would show and put to the test, and the user expects it to go.

## What is known

- **Not a matter of colour.** The user compared, on the light page,
  the logo in the heading shade (`#a9600d`, 4.61:1), in a darker shade
  of the same hue (`#7e480a`, 7.14:1), with titles and headings darker
  too, and in the shade of code inside a sentence (`#6f3f09`,
  8.42:1): the lightest read best.
- **A black header was tried and rejected** by the user: the whole
  header black on the light page, the header and the footer black, and
  only the band above the header line black.
- **The user's leaning, before 2026-11-22:** drop the drawn logo, and
  show the written logo everywhere, on both backgrounds. The answer
  below keeps the written name everywhere and adds an icon beside it.
- **The idea of a fixed icon** in place of the two logos came up in
  the session of 2026-11-22 (minute
  `docs/board/sessions/2026-11-22-grow-the-frame-text-within-ranges.md`),
  where the user also said they expect the logo's change at 768 pixels
  to go, and tied the title's size to the text rather than to the
  logo
  ([let the frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md)).
- **The favicon is already a circular crop of a photo of the user**
  (`assets/favicon.ico`, one image of 16 by 16 pixels): the user
  pushing a twin pushchair on a beach, under a blue cloth printed with
  sea animals. Checked by the agent on 2026-11-22 by enlarging the
  favicon and comparing it with the original photo, which the user
  copied, uncommitted, to `assets/logo-original.jpg` (a JPEG of 842
  by 1122 pixels; its metadata holds only the name of the program
  that saved it, no position).
- **The photo shows more than the user**: two small children with their
  faces visible, and other people on the beach. The repository and the
  site are public.
- **A photo cannot follow the theme's colours**, as the site's icons do
  (`src/icons.rs` turns a one-colour drawing into a CSS mask): it stays
  a fixed image. A circle with a transparent outside sits on both the
  dark and the light background; JPEG has no transparency.
- **What the drawn logo touches today** (checked on 2026-11-22): the
  frame shared by every page (`src/page_frame.rs`, the `LOGO` constant
  and its `div`, inside the link named "Lvk@73r, home page"); the rules
  `text-logo-large` and `art-logo-left` in `style/input.css`; the unit
  tests of the logo in `src/home_page.rs` and, the same ones, in
  `src/curriculum_page.rs`; the header's browser test
  (`end2end/tests/header-layout.spec.ts`, which expects the drawn logo
  from 768 pixels up); the accessibility test, which drops the
  contrast findings on the drawn logo only (WCAG exempts logos from
  contrast) and would no longer need to; the font roles' test
  (`end2end/tests/font-roles.spec.ts`, one case on the drawn logo);
  comments in `end2end/tests/home.spec.ts` and
  `end2end/tests/support.ts`, whose list of screen widths names the
  two logos.
- **ADR-003** calls the logo in the pixel font "the one sign of 1985
  left in the frame" and says a logo drawn in characters needs a
  monospaced font: to be updated if the drawing goes.
- **The logo's name was once lost by a screen reader**: Orca 51 did
  not speak the name of the drawn logo when it was a `<pre>`
  ([logo name not read by Orca 51](logo-name-not-read-by-orca-51.md)).

## Questions to answer

The ones known when the card was taken into work; there may be more.

- **Which logo takes the place of the two forms.**

  **Answered by the user on 2026-11-22.** An icon, the photo of the
  favicon cut to a circle, which also leaves out what does not belong
  to it; with the same framing as the favicon, so that each recalls
  the other. Beside the icon, the name "Lvk@73r" stays, written as
  text, at every width. If the name and the icon one day do not fit
  somewhere, only the icon stays there.

- **What of the photo goes into the repository.**

  **Proposed by the agent and agreed by the user on 2026-11-22.** The
  original photo is never committed, since the repository is public
  and a file committed once stays in its history: only the circular
  images made from it, at the sizes the site needs.

- **Who holds the rights on the photo, and under which licence its
  images are published.** The person who took the photo holds its
  copyright; the site's content has licences of its own
  (`content/README.md`), different from the code's.

  **Answered by the user on 2026-11-22, for the rights.** The user's
  wife took the photo, with the user's phone, so the rights are hers;
  she agrees that the photo, cut to the circle, is published on the
  site and in the repository as the logo and as the favicon. The
  licence is still open: the favicon, already made from this photo,
  sits in `assets/` with no note, so it falls under the repository's
  MIT licence, which lets anyone reuse it; a logo is usually kept out
  of free licences.

  **Answered by the user on 2026-11-22, for the licence.** The logo and
  the favicon are under no licence of the repository or of the site:
  all rights reserved, out of the MIT licence. Today nothing says so:
  `README.md` ("License") and `content/README.md` name the code's and
  the text's licences, and the site's licence page
  (`content/licence.md`) says that, unless a page says otherwise, what
  is read on the site may be reused, and that images by other authors
  name their source.

  **Answered by the user on 2026-11-22, for the notice.** The notice
  names Gioia Greifenberg as the author of the photo, as the site's
  licence page asks of images by other authors.
- **How large the icon and the name are**, and whether they grow
  within a range of the page's width, as the title does.

  **Answered by the user on 2026-11-22, for the icon.** The icon grows
  as the title does, from 48 pixels on narrow screens to 64 on wide
  ones, and crosses the header's line: the line passes where the
  girls' feet are, about 80 per cent of the circle's height down, as
  if the pushchair stepped over it. Chosen on screenshots of the real
  header (icon at 32, 48 and 64 pixels, then on the line at 48 and
  64), made by the agent with throwaway scripts outside the
  repository. What the screenshots showed: at 32 pixels the photo is
  a blue blot; from 48 the figure, the cloth and the pushchair are
  recognised; at these sizes the feet themselves cannot be made out,
  so the line reads as a circle resting on it.

  **Answered by the user on 2026-11-22, for the name.** The name is
  written in Literata bold italic, the size of the title (from 24 to
  32 pixels), with its baseline on the header's line, so the "@"
  reaches below it as the pushchair's wheels do. The pixel font
  (IBM VGA 8x16) leaves the site with it: the user drops for good the
  reference to the BBS of 1985 that ADR-003 kept in the logo. Chosen
  on screenshots at double pixel density, the user's screen, among the
  pixel font, Literata bold, regular and bold italic: bold looked like
  a second title beside the page's, regular looked thin beside the
  icon. Then tried in a copy of the built site served on this machine:
  the title, too, with its baseline on the line, from 1280 pixels up
  where it sits beside the name; the user rejected it, since the page
  looked badly formatted and the title read worse, so the title stays
  where the site puts it today.

  **Answered by the user on 2026-11-22, for the header and the
  line.** The header keeps the room above the icon and the name: there
  is more room above than below, and the line becomes part of the
  logo. On the user's idea, under the name the line disappears, like a
  thread that runs inside the letters: it reaches the foot of the "L",
  is left only where each letter touches it, and comes out at the foot
  of the "r". The name is closer to the icon (6 pixels instead of 12)
  and slanted 5 degrees more than Literata's italic. Tried in the copy
  of the site by the agent: the stretch of line hidden runs from the
  first to the last pixel the letters ink just above the baseline,
  measured on a canvas in the same font. Done for real, those two
  points depend on the font and its size: the build could compute them
  from Literata's measures, as it does for the tables' widths, or a
  browser test could check them. The extra slant is a skew of an italic
  already drawn: unnoticed at 5 degrees, stretched beyond. The thread
  went to a card of its own (see "What it leaves"); the name closer to
  the icon and its extra slant stay here.

  Still open: how far the circle may reach into the title below the
  line on narrow screens, with long titles.

  **Answered by the user on 2026-11-22, for the title below.** Measured
  by the agent the same day on the copy of the site, on three pages,
  the longest title among them, at widths from 320 to 1279 pixels
  (from 1280 the title sits beside the logo): the circle reaches from
  10 to 13 pixels below the line, into the 24 pixels of room the title
  keeps above itself, and leaves between 12 and 15 pixels before the
  title's highest line of text, also when the title wraps and starts
  under the circle. The user finds that distance right, and asked for
  a test that protects it: the circle never reaches the title's text.
- **How the link keeps one name.** With an alternative text on the
  icon and the name written beside it, a screen reader may say the
  name twice; the link already has a name of its own.

  **Proposed by the agent and agreed by the user on 2026-11-22.** The
  icon has an empty alternative text, so it is not announced, and the
  link keeps its own name (`aria-label="Lvk@73r, home page"` in
  `src/page_frame.rs`).
- **In which formats and sizes the images are made**, screens with
  dense pixels included, and whether the favicon is made again from
  the same crop.

  **Answered by the user on 2026-11-22, in part.** The user's screen
  has double pixel density; on it the icon is clear, while at normal
  density it looked grainy, since a 48-pixel image holds only 48
  pixels of the photo. The icon is the sharpened version: the crop
  reduced to size, then sharpened (unsharp mask, radius 1.2, 80 per
  cent, threshold 2), with colour raised by 15 per cent and contrast
  by 8. The formats and the favicon are still open.

  **Answered by the user on 2026-11-22, for the formats.** The icon
  must look as approved on screens of normal density too, so it is
  made in several WebP files, each sharpened at its own size, and the
  browser picks one; JPEG has no transparency, and WebP weighs less
  than PNG. Before choosing WebP the user asked to check its
  vulnerabilities. What the agent found the same day: the one the user
  remembered is CVE-2023-4863, a heap overflow in libwebp's decoder,
  exploited in the wild through crafted images in 2023 and fixed in
  libwebp 1.3.2 and in the browsers of September 2023; no new flaw of
  libwebp itself was found for 2025 or 2026 (the one of 2026,
  CVE-2026-58586, is in a Perl module that carries a copy of libwebp
  from 2013). The danger lies in decoding a crafted file: the site
  serves only files made by its author, from the author's own photo,
  and the tool that makes them here, Pillow 12.1.1, uses the system's
  libwebp 1.5.0.

  **Answered by the user on 2026-11-22, for the sizes in between.** The
  icon grows smoothly, so between 48 and 64 pixels the browser takes
  the next larger file and shrinks it a little itself, which is not
  exactly what the trial showed at 48 and 64. The user accepts that
  small reduction rather than an icon that grows by steps.

  **Answered by the user on 2026-11-22, for the recipe.** On the
  agent's proposal, the way the images are made is written in
  `assets/logo/README.md`, and the images are made once by a script
  kept out of the repository; the card of the favicon decides whether
  a committed script is worth it. The agent found again the crop's
  place in the original photo, written nowhere until then: the square
  of 760 pixels at 0 from the left and 110 from the top.

  **Decided by the agent on 2026-11-22, the user informed.** The files
  are lossy WebP, quality 90: from 2,510 to 9,546 bytes, against 6,120
  to 30,906 lossless.

  **Decided by the user on 2026-11-22, for the favicon.** The favicon
  went to a card of its own (see "What it leaves"); the formats and
  sizes of the icon's images stay here.
- **What ADR-003 says once the drawn logo is gone**, and the pixel
  font with it.

  **Decided by the user on 2026-11-22.** It went, with the removal of
  the pixel font's files, to a card of its own (see "What it leaves").
- **Whether the logo becomes a component of its own**, apart from the
  page frame. Asked by the user on 2026-11-22, at the end of the
  session that took the card into work; not discussed yet.

  **Decided by the user on 2026-11-22.** It went to a card of its own
  (see "What it leaves").
- **How the card is cut.** It now holds the icon and its images, the
  name in Literata on the line, the notice on the rights, the favicon
  and the removal of the pixel font, which also touches the fonts'
  configuration, the fonts page, the font roles' test and ADR-003.
  Noticed by the agent on 2026-11-22.

  **Answered by the user on 2026-11-22.** Asked which result should
  work first on the built site, the user chose the icon and the name
  on the line, without the thread. This card keeps:
  - the round icon, sharpened, growing from 48 to 64 pixels and
    crossing the line, in place of both forms of the logo, at every
    width;
  - the name in Literata bold italic with its baseline on the line;
  - the role `logo` moved to Literata (below);
  - the link's name (below);
  - how far the circle may reach into the title's row on narrow
    screens;
  - the images, in the sizes the icon needs;
  - the notice on the rights (below).

  The thread, the removal of the pixel font's files with ADR-003, the
  favicon and the question of a component of its own went to cards of
  their own (see "What it leaves").
- **Whether the role `logo` moves to Literata in this card.** In
  `config/fonts.txt` the role `logo` names the pixel font, and the
  whole header uses it (`font-logo` on the header's container), not
  only the logo. Either the role moves to Literata, one line, and the
  pixel font is left with no role; or the name uses Literata directly
  and the role keeps a font the logo no longer uses. Raised by the
  agent on 2026-11-22.

  **Answered by the user on 2026-11-22.** The role moves to Literata
  in this card. What follows, checked by the agent the same day: the
  build declares in the stylesheet, and shows on the fonts page, only
  the fonts that play a role, so the pixel font leaves both by itself,
  and Literata is shown once with two roles, text and logo. The fonts
  page's text (`content/style-guide/fonts.md`) must then lose the piece
  `## ibm-vga-8x16 | logo` and gain a piece `## literata | logo`,
  without which the build stops; the font roles' test
  (`end2end/tests/font-roles.spec.ts`) changes its case on the drawn
  logo into one on the name. Literata bold italic is already in the
  configuration (`config/fonts/literata.txt`).
- **What the fonts page says of Literata as the logo's font.** The
  piece is the author's text.

  **Answered by the user on 2026-11-22.** On a draft of the agent, the
  user chose the shorter of two, which leaves out the history of the
  pixel font:

  > The name in the logo is written in Literata too, in bold italic,
  > beside a photo cut to a circle: bold looked like a second title
  > beside the page's own, and regular looked thin beside the photo.
- **Where the notice on the rights is written.** The images and their
  notice must arrive in the same commit: an image published with no
  note falls under the repository's MIT licence. The notice is legal
  text and may belong in three places, `README.md`,
  `content/README.md` and the site's licence page
  (`content/licence.md`), which changes the card in a way different
  from the rest. Raised by the agent on 2026-11-22.

  **Answered by the user on 2026-11-22.** The notice reaches both
  readers: the one who reads the repository and could copy the files
  from `assets/`, with a note beside the files as the fonts have
  (`assets/fonts/<folder>/README.md`), and the one who visits the site,
  on the licence page. Asked by the user, the notice is also written
  inside each image file the site serves, as metadata: one line only,
  not the terms of use, in English:

  > © 2011 Gioia Greifenberg. All rights reserved.

  2011 is the year the photo was taken. What the agent said of the
  metadata, the same day: PNG, WebP and JPEG all accept an XMP block
  (Extensible Metadata Platform), whose IPTC photo fields hold the
  creator and the copyright notice, and Google Images shows them; the
  notice then travels with the file outside the site; in the European
  Union, embedded rights information is protected against deliberate
  removal (directive 2001/29/EC, article 7), an indication and not a
  legal opinion. Metadata are easily stripped, by social networks and
  by image tools, ours included, so a test checks that they are in the
  files the site publishes. One line weighs a few tens of bytes, the
  full set of fields one or two kilobytes.

  **Answered by the user on 2026-11-22, for the text.** On drafts of
  the agent, the images go in a folder of their own, `assets/logo/`,
  and the notice is written in three places, the favicon included,
  since it is under all rights reserved too:
  - `README.md`, at the end of the section "License":
    "The logo's images in `assets/logo/` and the favicon are under none
    of these licences: all rights reserved ([details](assets/logo/README.md))."
  - `assets/logo/README.md`, beside the files: the images (the round
    icons of the folder, and `assets/favicon.ico`, cut from a photo by
    Gioia Greifenberg), the line "© 2011 Gioia Greifenberg. All rights
    reserved.", and that they are under none of the repository's
    licences, may not be copied, changed or reused, and carry the same
    notice inside each WebP file.
  - The site's licence page (`content/licence.md`), a section "The
    logo" before "Material by others": "The logo and the site's icon
    are cut from a photo by Gioia Greifenberg. © 2011 Gioia
    Greifenberg. All rights reserved: they may not be reused."

  The page's opening ("Unless a page says otherwise, you may reuse
  what you read here") gets no sentence on the exception: the user
  keeps it short, and the new section says it.

## Dependencies

None left. While the slice waited for the screenshots' approval, its
code sat uncommitted in the working tree; it is now in the commit named
in the recap.

## Acceptance tests

Agreed with the user on 2026-11-22. Each one names how it is checked.

The logo in the header:

1. On every page and at every width of the browser tests' list of
   screens, the header shows the round icon and the name "Lvk@73r"
   written as text; neither the logo drawn in characters nor the name
   in the pixel font appears anywhere. *Browser test.*
2. The icon is 48 pixels wide at a page of 320 pixels and 64 from 1366
   up, growing in a straight line in between, as the title does.
   *Browser test.*
3. The header's line crosses the icon at 80 per cent of its height,
   where the girls' feet are, in both forms of the header, within one
   pixel. *Browser test.*
4. The name is in Literata bold italic, the title's size, with its
   baseline on the header's line; it stands 6 pixels from the icon and
   leans 5 degrees more than Literata's italic. *Browser test.*
5. Below 1280 pixels, with the longest title, the circle never reaches
   the title's text. *Browser test.*

Accessibility:

6. The link to the home page has one name only, "Lvk@73r, home page";
   the icon has an empty alternative text and is not announced. *Unit
   test and browser test on the accessibility tree.*
7. The accessibility check passes on every page without dropping any
   finding on the logo: the exception for the drawn logo goes.
   *Browser test.*

Images and rights:

8. The icon is served as WebP files of 48, 64, 96 and 128 pixels, each
   sharpened at its own size, with a transparent outside, and the
   browser picks one by the icon's size and the screen's density.
   *Unit test on the markup, check on the files.*
9. Every WebP file the site publishes holds in its metadata the line
   "© 2011 Gioia Greifenberg. All rights reserved." *Automatic test on
   the files.*
10. The notice is in `README.md`, in `assets/logo/README.md` and on the
    licence page, with the agreed text. *Check by eye.*
11. The original photo is not in the repository. *Check with
    `git ls-files`.*

Fonts:

12. The role `logo` is Literata: the stylesheet no longer declares the
    pixel font, and the fonts page shows Literata once, with the roles
    text and logo and the chosen piece, and no section for IBM VGA
    8x16. *Unit tests and the font roles' browser test.*

Before the commit:

13. The user sees screenshots of the real header on a narrow screen and
    on a laptop, on both backgrounds, and approves them. *Check by
    eye.*

    **Approved by the user on 2026-11-22.** The one remark, the line
    that runs on under the letters of the name, is the thread this card
    left out on purpose
    ([hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md)).
14. The pipeline's checks pass: formatting, clippy, unit tests, type
    check, browser tests; the rows of `end2end/README.md` follow the
    tests that change. *Automatic.*

## Recap

- The card taken into work on 2026-11-22 (`1bccff9`), and its
  questions answered over that day's sessions: the photo and its
  rights (`1c1ff90`, `2dc9918`), the icon on the line (`aa96ceb`), the
  name in Literata bold italic (`11edd01`), the line as a thread
  (`6abb71b`); the card cut and its acceptance tests agreed
  (`3e1d13d`, `ba94ac6`); the slice built and left waiting for the
  screenshots (`eefb948`).
- The slice, in one commit (`3b44229`): the round icon in place of the
  logo drawn in characters and of the name in the pixel font, at every
  width, growing from 48 pixels at 320 to 64 at 1366 and crossing the
  header's line at 80 per cent of its height; the name "Lvk@73r" in
  Literata bold italic, the title's size, with its baseline on the
  line, 6 pixels from the icon and leaning 5 degrees more than the
  italic; the link's one name, the icon with an empty alternative
  text; the role `logo` moved to Literata, with the fonts page's piece;
  the four WebP files in `assets/logo/` with the notice inside each,
  and the notice in `README.md`, `assets/logo/README.md` and the
  licence page; the browser tests of the header, the accessibility
  check without its exception for the drawn logo, and the font roles'
  test on the name.

## Cautions

- **The name stands on the line with no measure of the font.** The
  logo's link is a line of text of height 0, the icon sits on its
  baseline with a margin of -20 per cent, and the name takes no height
  (`style/input.css`, `logo-link`, `logo-icon`, `logo-name`). The
  header's browser test checks the result, not the way it is reached.
- **The original photo lives only on the user's machine**
  (`assets/logo-original.jpg`, listed in `.git/info/exclude`). The way
  the icons are made from it, the crop's position included, is written
  in `assets/logo/README.md`; without the photo they cannot be made
  again at other sizes.
- **The trial files are in `target/logo-look/`**, wiped by a clean
  build: the screenshots and the script that made the images.

## What it leaves

Cut by the user on 2026-11-22.

- The line that disappears under the name like a thread:
  [hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md).
- The pixel font's files, its description and its licence, and what
  ADR-003 says once it is gone:
  [remove the pixel font](remove-the-pixel-font.md).
- The favicon, made again from the logo's crop:
  [make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md).
- Whether the logo becomes a component of its own:
  [decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md).

## References

- [Hide the header's line under the logo's name](hide-the-header-line-under-the-logo-name.md),
  [remove the pixel font](remove-the-pixel-font.md),
  [make the favicon from the logo's crop](make-the-favicon-from-the-logo-crop.md)
  and
  [decide whether the logo is a component of its own](decide-whether-the-logo-is-a-component.md):
  the cards cut from this one.
- [Give the articles' code, quotes and tables a look](design-look-of-articles.md):
  the card that brought the light background, where this one was
  opened.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that waits for this card.
- [Let the frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  where the title stopped following the logo.
- [Logo name not read by Orca 51](logo-name-not-read-by-orca-51.md):
  the earlier problem with the logo's name.
