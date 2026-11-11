# Show the colours in the style guide

- **Opened:** 2026-11-11, by the user, as the first page of the style
  guide in the card [build a design system](../wip/build-a-design-system.md);
  in Now.
- **Status:** being worked on, since 2026-11-11.

## Why

The style guide starts from the colours, with the light and the dark
scheme. Today the six colours of the site are written only in
`style/input.css`, and their contrasts only in a comment there, typed
by hand. The style guide is meant to define the look that the site
then applies: the colours are defined once, in the style guide, and the
site takes from it only their values, not the look of its pages.

## Decisions

Taken by the user on 2026-11-10 and 2026-11-11, each discussed before
being agreed.

1. **The colours are defined in a Rust module of the style guide.** The
   build writes them to `target/generated/colours.css`, which
   `style/input.css` imports instead of holding the values. Why: the
   page that shows a colour and the stylesheet that applies it read the
   same value, so they cannot drift apart. The build already writes a
   file that Tailwind reads (`target/generated/code-classes.txt`), and
   runs before Tailwind (`scripts/build-site.sh`). Checked on
   2026-11-10 with Tailwind 4.3.5: the theme's colours read from an
   imported file give the same stylesheet, the same size and the same
   values, with only the order of a few lines changed, which changes
   nothing (the light scheme's block is outside Tailwind's layers, so
   it wins wherever it sits). The price: the stylesheet is no longer
   complete on its own, a build is needed before the colours exist.
   The values stay static: they are in a file of the repository, the
   build writes static files from them, and only a version tag
   publishes them; no visitor can change them for others.
2. **An index under `/style-guide/` leads to the colours page,**
   `style-guide/index.html` and `style-guide/colours.html`, like the
   articles in `articles/`. The canonical address of the index ends
   with a slash, as the home page's does. Both pages are in the
   sitemap and indexed, because they are written for the readers like
   the articles; so the net and the SEO check cover them already. No
   link from the site for now: the menu
   ([card](../backlog/add-site-menu.md)) and the article on the style guide
   ([card](../backlog/write-an-article-on-the-style-guide.md)) will add them.
3. **Each foreground colour is shown on the two backgrounds,** the page
   and the code surface, in both schemes. Each sample shows what the
   colour is used for: the ink and the code accent as text of normal
   size, the heading accent as large text, the line accent as lines.
   Why not a block of colour: the thresholds depend on what is drawn
   (7:1 for normal text, 4.5:1 for large text, 3:1 for lines and
   frames), and a block would make the heading accent look fit for
   everything. As text, the samples are also checked by axe like any
   other text of the site.
4. **A forbidden pair is not shown.** The heading accent on the code
   surface (3.9:1 in the light scheme) is not on the page. It is
   forbidden in both schemes, not only in the light one: the two
   schemes differ only in colours (`end2end/tests/themes.spec.ts`), so
   a colour cannot sit on a surface in one scheme and not in the other.
   The module lists the pairs that are allowed; a forbidden pair is one
   missing from the list.
5. **Each sample shows its numbers:** the contrast ratio and the
   threshold of its use. The build computes them (WCAG formula) from
   the module's values and stops if an allowed pair falls below its
   threshold, in either scheme. Why: the ratios are no longer typed by
   hand in a comment, and what the page prints is what the build
   checked.

The allowed pairs today, from the comment in `style/input.css`:

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
  text" for the ink or "Code in a sentence" for the code accent. The
  line accent's sample is what the line does: a line that separates two
  short sections, written in the ink of the same scheme and background.
  The line accent cannot carry text (3:1 is below what text needs).
- **Where the explanations live:** in `content/style-guide/colours.md`,
  so the page reads as a text that explains the choices made. One
  level-2 section for each of the four foreground colours, titled with
  its role. The text says why, never the values: the codes and the
  ratios are printed by the samples, from the Rust module, so they are
  written once. The Markdown holds no HTML, like all the content, so
  the page's generator places the samples after each section; the build
  stops if a foreground colour has no section, or a section names no
  colour, as it does for a wrong field of an article.
- **No section for the backgrounds:** the page and the code surface are
  not explained, because the scheme belongs to the reader; the page
  shows how everything else behaves in each of the two. The
  backgrounds appear only as the ground of the samples, with their code.
- **The index:** an introduction in `content/style-guide/index.md`,
  and under it the list of the style guide's pages, written by the
  build; today only "Colours".
- **A consequence:** both pages come from a Markdown file, so the
  sitemap dates them as it does the other pages.

The colours page, as agreed (**[MD]** from the Markdown file,
**[GEN]** drawn by the generator from the Rust module; each box is
painted with its real background, whatever scheme the reader has):

```text
Lvk@73r                  Colours                       (the frame)
--------------------------------------------------------------
[MD]  Introduction.

[MD]  ## Ink
[MD]  Why the ink is what it is.

[GEN]  LIGHT                        DARK
      +-----------------------+   +-----------------------+
page  | Running text          |   | Running text          |
      +-----------------------+   +-----------------------+
       #1c1a17 on #fbfaf7          #e6e6e6 on #000000
       16.63:1, needs 7:1          16.83:1, needs 7:1
      +-----------------------+   +-----------------------+
code  | Running text          |   | Running text          |
      +-----------------------+   +-----------------------+
       14.08:1, needs 7:1          13.94:1, needs 7:1

[MD]  ## Line accent
[MD]  Why lines have a shade of their own.

[GEN]  LIGHT                        DARK
      +-----------------------+   +-----------------------+
page  | A section above       |   | A section above       |
      | --------------------- |   | --------------------- |
      | A section below       |   | A section below       |
      +-----------------------+   +-----------------------+
       3.7:1, needs 3:1            8.64:1, needs 3:1

... the code accent and the heading accent in the same way
(the heading accent on the page only, as large text).
```

The index page:

```text
Lvk@73r                  Style guide                   (the frame)
--------------------------------------------------------------
[MD]  What this guide is.

[GEN] - Colours                        (one link per page)
```

## How the samples are painted

Decided by the user on 2026-11-11, after comparing two mock pages built
with the site's real stylesheet, fonts and frame, on a phone and a
laptop, with the browser in each scheme.

- **Chosen: two named variants, `scheme-light` and `scheme-dark`,**
  written by the build from the Rust module into
  `target/generated/colours.css`, together with the rule that follows
  the browser's preference. A box with one of them becomes a small page
  in that scheme: what it holds uses the site's own variants, unchanged.
  Why: the samples show the real components, not only a colour (the
  code accent's sample is a sentence in the ink holding code in the
  code font, as in the articles; the heading accent's is a real title),
  and it keeps to
  [ADR-001](../adr/ADR-001-name-styles-by-visual-variant.md). The same
  mechanism may one day let the reader choose the scheme
  ([card](../backlog/let-readers-adjust-font-and-spacing.md)).
- **Rejected: the colour written on each box** (`style="..."`) from the
  module. Simpler, but an exception to ADR-001, and the samples would
  show a colour, not the site's style: in the mock, the code accent was
  a whole sentence in amber in the text's font.

The numbers under each sample are those of the colour shown: for the
code accent, the code, not the ink of the sentence around it.

## Who writes the texts

The agent writes placeholder texts for `content/style-guide/index.md`
and `content/style-guide/colours.md`, each telling the user what kind
of text goes there; the user then writes the texts. The placeholders
must not be published: snapshots on `main` are not, only a version tag
publishes, so the texts are written before the next version tag.
