# Read the articles' title, description and date from the front matter

- **Opened:** 2026-11-08, by the user and the agent, splitting the card
  on moving titles, descriptions and dates into the Markdown's front
  matter, opened on 2026-11-06 while choosing where the sitemap's dates
  come from ([card](2026-11-07_add-sitemap.md)). The other parts of
  that card: [the other pages' fields](../backlog/read-page-fields-from-markdown.md)
  and [the addresses from a slug](../backlog/take-page-addresses-from-slug.md).
- **Status:** done, on 2026-11-08.

## Why

The user wants the Markdown to be the only source of the site's
content. Before this card the body of each article was in its Markdown
file under `content/articles/`, but its title, tab title, description
and date were in the list `ARTICLES` of `src/article_page.rs`.

It matters for the sitemap: its date of a page's last change comes from
the git history of the page's Markdown file only
([card](2026-11-07_add-sitemap.md)). As long as the title, the
description and the date were in the code, changing them did not move
that date. Now that they are in the Markdown, it does.

This card was the first slice: the articles only, and only the fields
the author writes. It anticipates part of the
[run of `residuality`](../backlog/run-full-residuality-round.md), to
which the model of an "article" was left
([card](2026-11-06_list-articles-on-home-page.md), "What each entry
shows").

## Answers

### Which fields?

The title, the description and the date, in a front matter in YAML at
the top of each article's Markdown file. The order of the fields, and
blank lines between them, do not matter: the user asked for it, and a
unit test keeps it so.

- **Why these three:** they are what the author writes about an
  article, besides its text; the home page shows them too.
- **The tab title is computed,** as the title followed by
  " - Luca Lorenzon": both articles already followed that rule, so their
  pages did not change, and a new article cannot get it wrong.
- **The address stays in the code** for now, in `ARTICLE_FILES`,
  together with the list of files. Taking it from a slug brings
  questions of its own, such as two articles with the same slug and
  the redirects from an old address: a
  [card of its own](../backlog/take-page-addresses-from-slug.md).
  Finding the files by reading the folder, instead of the list in the
  code, is [another](../backlog/find-articles-in-content-folder.md).
- **Rejected:** a tab title written by hand in the front matter, one
  field more that only repeats the title.

### What does the build do when a field is missing or wrong?

The same as for a missing field, a wrong one: a date that is not a
day of the calendar, a file without a front matter, a description
longer than 160 characters, a field the front matter does not know.
The last one was proposed by the agent and accepted by the user: a
misspelt field, such as `descripton`, is reported by its name instead
of only as the missing `description`.

- **On the author's machine:** the article is skipped, with a warning;
  at the end, the build lists the skipped articles and why.
- **In the pipeline:** the build stops, after listing every problem, so
  they are all seen at once.
- **Why two rules:** it is the rule the sitemap already follows for a
  missing date (`src/sitemap.rs`). On the author's machine the rest of
  the site stays visible while an article is being written. In the
  pipeline nobody reads a warning, and what it builds is what gets
  published: an article would disappear from the site with a line in a
  log as its only trace. The browser tests would probably notice today,
  since they list the articles by name; the card on
  [checking behaviour, not articles](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
  means to remove those lists.
- **What changes for the description's length:** before, a unit test on
  each constant failed on the author's machine too. Now an article with
  a long description is skipped there, and listed.
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
  It uses serde, which was already in the project through Leptos and
  is now declared directly.
- **What it brings,** with the reading part only: fourteen libraries
  new to the project, counting serde-saphyr, the main one being
  granit-parser, the reader itself, by the same author. All are under
  MIT or Apache 2.0, one with data under BSD 3-Clause (encoding_rs). The
  agent first counted five, the library's own, before trying. It needs
  Rust 1.89 or later; the project follows the stable release.
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

While the YAML is read, following "parse, don't validate": the title,
the description and the date are types that can only hold a valid
value, built with `TryFrom`. The front matter is declared with those
types, and serde builds each of them as it reads
(`#[serde(try_from = "String")]`): if the front matter is read, its
fields are valid, and nothing checks them again further on. An article
is built, with `TryFrom` too, from its file: it splits the front matter
from the text, reads the front matter and adds the address and the
file's path.

- **Why:** the user chose it, so the checks live in one place, next to
  the values they protect.
- **Every error is an error of the front matter,** `FrontMatterError`,
  as the user proposed: missing, not closed, or not valid, with the
  reader's message. The message names the file, the field, the line and
  column of the file, and why, for example:

  ```text
  content/articles/after-save-event.md: front matter: error: line 4 column 7: field `date`: `2023-02-30` is not a day of the calendar
  ```

  The front matter is passed to the reader with its opening `---`, the
  start of a YAML document, so the lines it counts are the file's; a
  unit test keeps it so.
- **The error types are written by hand.** Moving them, and the rest of
  the code's errors, to [thiserror](https://crates.io/crates/thiserror)
  is a [card of its own](2026-11-09_refactor-errors-with-thiserror.md),
  as the user asked; it may also remove the doubled "error:" that comes
  from the reader's message.
- **Rejected:** the first version, where the front matter was read into
  three plain strings and the article's constructor turned each into
  its type, translating its error. The user saw, reviewing the code,
  that it kept a structure of unchecked values alive for a few lines,
  the very thing "parse, don't validate" avoids.

### What was left to other cards

Questions raised while reviewing the code, each kept to a card of its
own so that each decision stays in a commit of its own:

- [an empty text, and HTML in the text](../backlog/check-page-bodies-in-the-build.md),
  checked together in the build, so the type for the text is born once;
- [dates in a module of their own](2026-11-09_give-dates-a-module-of-their-own.md),
  or a widely used library, instead of the two types of day in
  `src/article_page.rs` and `src/sitemap.rs`;
- [one model for the information pages](../backlog/unify-the-information-pages-model.md),
  the licence, about and curriculum pages;
- [a page's own licence terms](../backlog/let-a-page-state-its-own-licence-terms.md),
  which the user first thought of as an optional field of the front
  matter.

## Results

- **The pages written are the same as before,** byte for byte: the site
  built before the change and the one built after were compared.
- **The checks of the pipeline pass:** formatting, clippy, 227 unit
  tests, the browser tests' types and 911 browser tests, Lighthouse
  included.
- **The front matter of the user's example**, with the date before the
  description and blank lines, builds the same pages as the usual
  order.
- **What the build prints with two broken articles,** on the author's
  machine (the build ends well):

  ```text
  warning: article skipped: content/articles/after-save-event.md: front matter: error: line 3 column 1: missing field `description`
  2 article(s) skipped, not in the site:
    - content/articles/after-save-event.md: front matter: error: line 3 column 1: missing field `description`
    - ...
  ```

  and in the pipeline (the build stops):

  ```text
  error: 2 article(s) have problems:
    - ...
  ```
- **Code:** commit `78932e1`.
