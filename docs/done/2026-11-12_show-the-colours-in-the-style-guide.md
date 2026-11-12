# Show the colours in the style guide

- **Opened:** 2026-11-11, by the user, as the first page of the style
  guide in the card [build a design system](../wip/build-a-design-system.md);
  in Now.
- **Status:** done, 2026-11-12. Not published yet: the pages go live
  with the next version tag.

## Why

The style guide starts from the colours, with the light and the dark
scheme. Until this card, the six colours of the site were written only
in `style/input.css`, and their contrasts only in a comment there,
typed by hand. The style guide is meant to define the look that the
site then applies: the colours are defined once, in the style guide,
and the site takes from it only their values, not the look of its
pages.

## Decisions

Taken by the user on 2026-11-10 and 2026-11-11, each discussed before
being agreed.

1. **The colours are defined in a Rust module of the style guide**
   (`src/style_guide/colours.rs`). The build writes them to
   `target/generated/colours.css`, which `style/input.css` imports
   instead of holding the values. Why: the page that shows a colour and
   the stylesheet that applies it read the same value, so they cannot
   drift apart. The build already wrote a file that Tailwind reads
   (`target/generated/code-classes.txt`), and runs before Tailwind
   (`scripts/build-site.sh`). The price: the stylesheet is no longer
   complete on its own, a build is needed before the colours exist.
   The values stay static: they are in a file of the repository, the
   build writes static files from them, and only a version tag
   publishes them; no visitor can change them for others.
2. **An index under `/style-guide/` leads to the colours page,**
   `style-guide/index.html` and `style-guide/colours.html`, like the
   articles in `articles/`. The canonical address of the index ends
   with a slash, as the home page's does. Both pages are in the sitemap
   and indexed, because they are written for the readers like the
   articles; so the net and the SEO check cover them. No link from the
   site for now: the menu ([card](../backlog/add-site-menu.md)) and the
   article on the style guide
   ([card](../backlog/write-an-article-on-the-style-guide.md)) will add
   them.
3. **Each foreground colour is shown on the two backgrounds,** the page
   and the code surface, in both schemes, as what it is used for: the
   ink and the code accent as text of normal size, the heading accent
   as a title, the line accent as a line between two sections. Why not
   a block of colour: the thresholds depend on what is drawn (7:1 for
   normal text, 4.5:1 for large text, 3:1 for lines and frames), and a
   block would make the heading accent look fit for everything. As
   text, the samples are also checked by axe like any other text of the
   site.
4. **A forbidden pair is not shown.** The heading accent on the code
   surface (3.9:1 in the light scheme) is not on the page, and is
   forbidden in both schemes: the two schemes differ only in colours
   (`end2end/tests/themes.spec.ts`), so a colour cannot sit on a
   surface in one scheme and not in the other. The module lists the
   pairs that are allowed; a forbidden pair is one missing from the
   list.
5. **Each sample shows its numbers:** the codes, the contrast ratio and
   the threshold of its use. The build computes them (WCAG formula)
   from the module's values and stops if an allowed pair falls below
   its threshold, in either scheme; the unrounded ratio is compared, so
   6.99998:1 does not pass as 7:1. Why: the ratios are no longer typed
   by hand in a comment, and what the page prints is what the build
   checked.

The allowed pairs:

| Foreground | Backgrounds | Use | Threshold |
|---|---|---|---|
| ink | page, code surface | normal text | 7:1 |
| code accent | page, code surface | normal text: code in a sentence, the "wrap" label | 7:1 |
| heading accent | page only | large text only: logo, titles | 4.5:1 |
| line accent | page, code surface | lines, frames, focus outlines | 3:1 |

In the dark scheme the three accents have the same value, `#dd9911`; in
the light scheme each use needs its own shade.

## Answers

Given by the user on 2026-11-11.

- **What the samples say:** what their colour is for, such as "Running
  text, written in the ink." or "Code in a sentence". The line accent
  cannot carry text (3:1 is below what text needs), so its sample is a
  line between "A section above" and "A section below", written in the
  ink.
- **Where the explanations live:** in `content/style-guide/colours.md`,
  so the page reads as a text that explains the choices made: an
  introduction, then one level-2 section for each of the four
  foreground colours, titled with its role. The text says why, never
  the values: the samples print them, from the Rust module. The
  Markdown holds no HTML, like all the content, so the page's generator
  places the samples after each section; the build stops if a
  foreground colour has no section, a section names no colour, or a
  section is repeated.
- **No section for the backgrounds:** the scheme belongs to the reader,
  so the page and the code surface are not explained; the page shows
  how everything else behaves in each of the two.
- **The index:** an introduction in `content/style-guide/index.md`,
  and under it the list of the style guide's pages, written by the
  build; today only "Colours".
- **The dates:** both pages come from a Markdown file, so the sitemap
  dates them as it does the other pages.

## How the samples are painted

Decided by the user on 2026-11-11, after comparing two mock pages built
with the site's real stylesheet, fonts and frame, on a phone and a
laptop, with the browser in each scheme.

- **Chosen: two named variants, `scheme-light` and `scheme-dark`,**
  written by the build from the Rust module into
  `target/generated/colours.css`. A box with one of them becomes a
  small page in that scheme: what it holds uses the site's own styles,
  unchanged (the code accent's sample is a sentence in the ink holding
  code in the code font, as in the articles; the heading accent's is a
  real title). It keeps to
  [ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md), and the
  same mechanism may one day let the reader choose the scheme
  ([card](../backlog/let-readers-adjust-font-and-spacing.md)).
- **Rejected: the colour written on each box** (`style="..."`) from the
  module. Simpler, but an exception to ADR-001, and the samples would
  show a colour, not the site's style: in the mock, the code accent was
  a whole sentence in amber in the text's font.

Two details settled while building, shown to the user in screenshots
before the commit:

- **The frame of each box is drawn in the line accent** of the
  reader's scheme, the colour the module gives every line and frame;
  in the mock it was the ink. Without a frame, a light box on a light
  page, or a dark box on a dark page, would show no edge.
- **The heading accent's sample is a level-3 title,** "Titles and
  headings", so the page's outline stays in order under each section's
  level-2 title.

## The texts

The agent wrote placeholders telling the user what kind of text went
in each place; the user wrote the texts on 2026-11-12, and the
language was then corrected together with the agent, keeping the
user's ideas and their order. Two things changed in the content, by
the user's choice: a quotation from Deleuze and Guattari's *A Thousand
Plateaus*, which the user is fond of, was left out as out of place on
this page; and the ink's section no longer says that amber on black
was at the limit of contrast, which the sample below it (8.64:1)
contradicted: the reason was that it made reading heavy.

The two descriptions for search engines are still constants in the
Rust code of each page, as for the about and licence pages, until the
pages' fields are read from the Markdown
([card](../backlog/read-page-fields-from-markdown.md)).

## Results

- **The colours defined once** (commit `50ebaca`): a refactor, with the site
  unchanged. Every built file was identical to the one built before;
  `style.css` kept the same size and declarations, with one line in
  another order. The eleven ratios the old comment typed by hand all
  matched the computed ones to two decimals.
- **The two pages** (commit `f4125cd`), and **the texts** (commit
  `a0e0aed`). The net of browser checks covers the new pages with no
  test added: 227 browser tests before, 253 after (the net runs on
  every page of the sitemap), axe's contrast check included, in both
  schemes.
- **One workaround in the stylesheet:** inside a sample box, the text
  column's margins are reset with a doubled selector
  (`:is(p, h3, hr):is(p, h3, hr)`), which outweighs the column's rules
  by specificity. It is commented in `style/input.css`.
