# Review the logo

- **Created:** 2026-11-05
- **Status:** In work

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
  already drawn: unnoticed at 5 degrees, stretched beyond.

  Still open: how far the circle may reach into the title below the
  line on narrow screens, with long titles.
- **How the link keeps one name.** With an alternative text on the
  icon and the name written beside it, a screen reader may say the
  name twice; the link already has a name of its own.
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
- **What ADR-003 says once the drawn logo is gone**, and the pixel
  font with it.
- **Whether the logo becomes a component of its own**, apart from the
  page frame. Asked by the user on 2026-11-22, at the end of the
  session that took the card into work; not discussed yet.
- **How the card is cut.** It now holds the icon and its images, the
  name in Literata on the line, the notice on the rights, the favicon
  and the removal of the pixel font, which also touches the fonts'
  configuration, the fonts page, the font roles' test and ADR-003.
  Noticed by the agent on 2026-11-22.

## Dependencies

- **The trial files**, outside the repository and not committed: the
  original photo in `assets/logo-original.jpg`, kept out of git on
  purpose (see the answer on what goes into the repository), and the
  trial in `target/logo-look/`: the circular crops (`circle-*.png`,
  the sharpened ones `circle-sharp-*.png`), the screenshots and the
  copy of the site with the logo (`mock-site/`, its script
  `logo-mock.js`). `target/` is wiped by a clean build.

## References

- [Give the articles' code, quotes and tables a look](design-look-of-articles.md):
  the card that brought the light background, where this one was
  opened.
- [Show the page frame in the style guide](show-the-page-frame-in-the-style-guide.md):
  the page that waits for this card.
- [Let the frame's text grow within the injected ranges](let-the-frame-text-grow-within-the-injected-ranges.md):
  where the title stopped following the logo.
- [Logo name not read by Orca 51](logo-name-not-read-by-orca-51.md):
  the earlier problem with the logo's name.
