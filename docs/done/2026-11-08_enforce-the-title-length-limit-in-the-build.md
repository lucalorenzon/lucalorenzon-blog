# Enforce the 50-character title limit in the build

- **Opened:** 2026-11-08, by the user, while deciding which widths the
  browser tests need, in the card
  [check at every width only what depends on the width](../wip/check-at-every-width-only-what-depends-on-it.md),
  before its header check moves to the invented titles.
- **Status:** done, 2026-11-08.

## Why

On 2026-10-29 the user and the agent agreed that a title is at most 50
characters (card
[adapt the footer and the articles to screen width](../backlog/adapt-footer-and-articles-to-screen-width.md),
"Decisions already taken"): with that limit a title beside the logo
takes at most two lines, so the browser test of the header checks two
invented 50-character titles instead of every real one. Nothing made
the limit hold.

Until now this mattered little, because the text spacing test
(`end2end/tests/readability.spec.ts`) also checked the header of every
page, with its real title, at every width. The card on widths moves that
header check into the header's own test, on the invented titles: from
then on, a title longer than 50 characters would be seen by no test.
The user decided that the limit is enforced in the build first, so the
limit becomes a rule the tests can rely on.

## Answers

### What is counted?

**Graphemes** (user): what a reader sees as one character, so a letter
with a combining accent or an emoji made of several code points counts
once. The crate is
[unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation)
1.13.3, MIT or Apache-2.0, from the unicode-rs organisation: already
compiled in the build as a dependency of Leptos (through
`convert_case`) and the latest version published, so declaring it added
no package to `Cargo.lock`. The error message says "characters" to the
author.

**Rejected:** Unicode scalar values (the agent's proposal: no new
dependency, stricter only on the safe side), because they are not what
the reader counts.

### Whose rule is it: the article's title or the frame's?

**The frame's** (user): the heading the frame shows may differ from an
article's or a fixed page's title (for example a shorter form of a long
title), so the domain is not mixed with the view. The frame takes a
type of its own, `Heading` (`src/page_frame.rs`), trimmed, not empty,
at most the limit in graphemes; every page builds one to get its frame.
The article's `Title` stays a piece of content, not empty, with no
length limit. This also answers the question on the other pages'
titles: the fixed pages (home, curriculum, about, licence, "page not
found") build their `Heading` from constants, and a unit test per page
fails if one does not fit.

**Rejected:** moving `Title` out of the article page and making the
frame take it (the agent's first proposal), which would tie the
content's title to the view's rule.

### When is a title that does not fit found?

**When the article is read** (agent's proposal, user's decision):
reading an article also builds its `Heading`, so a title that does not
fit is one more problem of the front matter (`FrontMatterError::Heading`),
listed with the others, skipped with a warning locally, stopping the
build in the pipeline (`settle`), like any wrong field.

**Rejected:** finding it only when the page is built, which would stop
at the first error, locally too, unlike every other problem of an
article.

### Where does the figure live?

**In one file, for now** (user, on the agent's proposal):
`config/heading-max-graphemes.txt`, holding only the number. The build
includes it when it is compiled; the header's browser test reads it
from the disk, which works on the live check too, since the tests
always run from a copy of the repository, and counts its invented
titles in graphemes with `Intl.Segmenter`, as the build does.

The user had meant a structured configuration file read by both Rust
and JavaScript (a section of `Cargo.toml`, or a `config.toml`); the
agent took it as the file's name. Weighed afterwards: `Cargo.toml` does
not pass its metadata to the code, so it would need a build script, and
a shared TOML or YAML file needs a parser on the tests' side (JSON would
not); with a single number the plain file avoids both. The user kept
the plain file for now; a structured file is the form to take once
there is more than one setting.

**Rejected:** the build writing the limit into the page for the test to
read (markup that serves only the tests); two copies with comments
pointing at each other (nothing would notice them drifting apart).

## Results

- **A title of 61 characters**, tried on one article and then restored:
  - in the pipeline (`CI=true`) the build stops with
    `1 article(s) have problems:` and
    `content/articles/after-save-event.md: front matter: field `title`: the title has 61 characters, at most 50 fit in the page heading`;
  - locally the article is skipped with the same message as a warning,
    and listed in the summary at the end.
- **The site is unchanged:** the built site is identical, file by file,
  before and after the change.
- **Tests:** 244 unit tests (a heading at the limit, one grapheme more,
  empty, combining accents and emoji counted once, the article's
  message in both modes, every fixed page); the browser tests pass (344,
  1 skipped, the live-only one).

Commit: `1277d9a`.
