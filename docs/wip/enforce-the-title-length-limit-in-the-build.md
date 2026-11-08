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
