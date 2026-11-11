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

## Questions

- Which made-up texts the samples use.
- Whether the explanations of each colour leave the comments of
  `style/input.css` for the Rust module, so the page can print them.
- How the index page presents the style guide.
