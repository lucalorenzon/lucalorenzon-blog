# Read the articles' title, description and date from the front matter

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on moving titles, descriptions and dates into the Markdown's front
  matter, opened on 2026-11-06 while choosing where the sitemap's dates
  come from ([card](../done/2026-11-07_add-sitemap.md)). The other parts
  of that card: [the other pages' fields](../backlog/read-page-fields-from-markdown.md)
  and [the addresses from a slug](../backlog/take-page-addresses-from-slug.md).
- **Status:** being worked on, since 2026-11-08.

## Why

The user wants the Markdown to be the only source of the site's
content. Today the body of each article is in its Markdown file under
`content/articles/`, but its title, tab title, description and date are
in the list `ARTICLES` of `src/article_page.rs`.

It matters for the sitemap: its date of a page's last change comes from
the git history of the page's Markdown file only
([card](../done/2026-11-07_add-sitemap.md)). As long as the title, the
description and the date are in the code, changing them does not move
that date. Once they are in the Markdown, it does.

This card is the first slice: the articles only, and only the fields
the author writes. It anticipates part of the
[run of `residuality`](../backlog/run-full-residuality-round.md), to
which the model of an "article" was left
([card](../done/2026-11-06_list-articles-on-home-page.md), "What each
entry shows").

## Answers so far

### Which fields?

The title, the description and the date, in a front matter in YAML at
the top of each article's Markdown file.

- **Why these three:** they are what the author writes about an
  article, besides its text; the home page shows them too.
- **The tab title is computed,** as the title followed by
  " - Luca Lorenzon": both articles already follow that rule, so their
  pages do not change, and a new article cannot get it wrong.
- **The address stays in the code** for now, in `ARTICLES`, together
  with the list of files. Taking it from a slug brings questions of its
  own, such as two articles with the same slug and the redirects from
  an old address: a [card of its own](../backlog/take-page-addresses-from-slug.md).
- **Rejected:** a tab title written by hand in the front matter, one
  field more that only repeats the title.

### What does the build do when a field is missing or wrong?

The same as for a missing field, a wrong one: a date that is not a
date, a file without a front matter, a description longer than 160
characters, a field the front matter does not know. The last one was
proposed by the agent and accepted by the user: a misspelt field, such
as `descripton`, is reported by its name instead of only as the
missing `description`.

- **On the author's machine:** the article is skipped, with a warning;
  at the end, the build lists the skipped articles and why.
- **In the pipeline:** the build stops, after listing every problem, so
  they are all seen at once.
- **Why two rules:** it is the rule the sitemap already follows for a
  missing date (`src/sitemap.rs`). On the author's machine the rest of
  the site stays visible while an article is being written. In the
  pipeline nobody reads a warning, and what it builds is what gets
  published: an article would disappear from the site with a line in a
  log as its only trace. Today the browser tests would probably notice,
  since they list the articles by name; the card on
  [checking behaviour, not articles](../backlog/make-browser-tests-check-behaviour.md)
  means to remove those lists.
- **What changes for the description's length:** today a unit test on
  each constant fails on the author's machine too. With this rule, an
  article with a long description is skipped there, and listed.
- **Rejected:** a warning in the pipeline as well, the user's first
  proposal, given up for the reason above.

### Which tool reads the YAML?

[serde-saphyr](https://crates.io/crates/serde-saphyr) 2.1, with its
reading part only (`default-features = false, features =
["deserialize"]`).

- **Why a real YAML reader:** the user chose that the front matter be
  YAML with no surprises: a title with a colon, or in quotes, is read as
  any editor shows it.
- **Why this one:** it is maintained (version 2.1.1 of 2026-11-05),
  licensed MIT or Apache 2.0, and reads the front matter straight into
  a Rust structure, reporting a missing field or one of the wrong type.
  It uses serde, already in the project through Leptos.
- **What it brings,** with the reading part only: five libraries of its
  own, the main one being granit-parser, the reader itself, by the same
  author; with theirs, fourteen libraries new to the project, counting
  serde-saphyr. All are under MIT or Apache 2.0, one with data under
  BSD 3-Clause (encoding_rs). The first count, five, was the agent's,
  made before trying. It needs Rust 1.89 or later; the project follows the stable
  release.
- **The risk to know:** both the library and its reader depend on one
  person. If they stop maintaining them, an alternative is needed.
- **Rejected:**
  - `serde_yaml`, deprecated by its author in 2024;
  - `serde_yml`, which declares itself unmaintained;
  - `yaml-rust2` 0.13, maintained, but it reads into a generic structure
    from which the code would take the fields by hand;
  - a small reader of `key: value` lines written for the site, with no
    dependency, but it would accept only part of YAML while the file
    looks like YAML.

### Where are the fields checked?

In the constructors, following "parse, don't validate": each field is a
type that can only hold a valid value, built with `TryFrom`, and an
article is built, again with `TryFrom`, from its Markdown file. A
built article is the proof that its fields are valid; nothing checks
them again further on. A failed construction returns an error that
says which file and which field, and why: the build turns it into the
warning, or the stop, described above.

- **Why:** the user chose it, so the checks live in one place, next to
  the values they protect.
- **The error types are written by hand** in this card. Moving them,
  and the rest of the code's errors, to
  [thiserror](https://crates.io/crates/thiserror) is a
  [card of its own](../backlog/refactor-errors-with-thiserror.md), as
  the user asked.
- **The text after the front matter is not checked here:** an empty
  text, and HTML in the text, are checked together in a
  [card of their own](../backlog/check-page-bodies-in-the-build.md),
  as the user chose, so the type for the text is born once, with both
  rules.
