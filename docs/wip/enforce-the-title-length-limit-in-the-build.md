# Enforce the 50-character title limit in the build

- **Opened:** 2026-11-08, by the user, while deciding which widths the
  browser tests need, in the card
  [check at every width only what depends on the width](../wip/check-at-every-width-only-what-depends-on-it.md);
  in Now, before the header check of that card moves.
- **Status:** being worked on, since 2026-11-08.

## Why

On 2026-10-29 the user and the agent agreed that a title is at most 50
characters (card
[adapt the footer and the articles to screen width](adapt-footer-and-articles-to-screen-width.md),
"Decisions already taken"): with that limit a title beside the logo
takes at most two lines, so the browser test of the header checks two
invented 50-character titles instead of every real one. Nothing makes
the limit hold: how to enforce it is still an open question of that
card.

Until now this mattered little, because the text spacing test
(`end2end/tests/readability.spec.ts`) also checked the header of every
page, with its real title, at every width. The card on widths moves that
header check into the header's own test, on the invented titles: from
then on, a title longer than 50 characters would be seen by no test.
The user decided that the limit is enforced in the build first, so the
limit becomes a rule the tests can rely on.

## Proposal

The articles read their title from the front matter into the type
`Title` (`src/article_page.rs`), which already refuses an empty title.
A title longer than 50 characters becomes one more error of the front
matter: the build lists it with the others and stops, as for any wrong
field ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).

## Questions

- **What is counted:** characters as a reader sees them (Unicode
  scalar values, or grapheme clusters for accents and emoji), not bytes.
- **The other pages' titles** (home, curriculum, about, licence, "page
  not found") are written in the code today and are all short; whether
  they get the same check now, or when they move to the Markdown
  ([card](read-page-fields-from-markdown.md)).
- **Where the figure lives,** so the header test and the build share
  one value instead of two copies of 50.

## Decided so far

1. **Graphemes are counted** (user), what a reader sees as one
   character, not Unicode scalar values (the agent's proposal: no new
   dependency, and stricter only on the safe side). The crate is
   [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation)
   1.13.3, MIT or Apache-2.0, from the unicode-rs organisation: already
   compiled in the build as a dependency of Leptos (through
   `convert_case`) and the latest version published, so declaring it
   adds no new code.
2. **The limit belongs to the frame, not to the article's title**
   (user): the heading the frame shows may differ from an article's or
   a fixed page's title (for example a shorter form of a long title), so
   the domain is not mixed with the view. The article's `Title` stays a
   piece of content, not empty, with no length limit; the frame takes a
   type of its own (for example `Heading` in `src/page_frame.rs`) that
   holds at most 50 graphemes, and every page builds one to get its
   frame. **Rejected:** moving `Title` out of the article page and
   making the frame take it (the agent's first proposal), which would
   tie the content's title to the view's rule.
3. **Checked when the article is read** (agent's proposal, user's
   decision): reading an article also builds the frame's heading from
   its title; a title that does not fit is one more problem of the
   article, listed with the others, skipped with a warning locally and
   stopping the build in the pipeline (`settle`). **Rejected:** checking
   only when the page is built, which would stop at the first error,
   locally too, unlike every other problem of an article.
4. **The figure lives in one configuration file** (user, on the agent's
   proposal): `config/heading-max-graphemes.txt`, holding only the
   number. The build includes it when it is compiled; the header's
   browser test reads it from the disk, which works on the live check
   too, since the tests always run from a copy of the repository; the
   test counts its invented titles in graphemes (`Intl.Segmenter`), as
   the build does. **Rejected:** the build writing the limit into the
   page for the test to read (markup that serves only the tests); two
   copies with comments pointing at each other (nothing would notice
   them drifting apart).
