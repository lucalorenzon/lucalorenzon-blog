# Refactor the code's errors with thiserror

- **Opened:** 2026-11-08, by the user, while choosing how the articles'
  front matter is checked
  ([card](2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** done, 2026-11-09.

## Why

The user wanted the code refactored to use
[thiserror](https://crates.io/crates/thiserror), the library that
writes the boilerplate of an error type from a short declaration. The
errors were written in three ways: plain text (`Result<_, String>`, in
`src/sitemap.rs`, `src/robots_txt.rs` and the articles' `settle`), a
generic box in the build program (`Box<dyn Error>`, in `src/main.rs`),
and six error types written by hand. The YAML reader's messages also
began with a doubled "error:", as in
`front matter: error: line 4 column 7: ...`.

## Answers

All chosen by the user on 2026-11-09, on the agent's questions and
proposals, unless said otherwise.

### Which errors move to thiserror? All of them

**Chosen:** all three ways give way to types declared with thiserror,
and the hand-written `Display` and `Error` implementations are removed.
One mechanism, the same annotations on every error type, is the common
structure the user asked for.

**Why:** the card was opened because errors were written in several
ways. The question as first written, "only those matched by the code",
separated nothing: no error was told apart by the build's code, only
by the unit tests.

**Rejected:** moving only the six hand-written types, which would have
left the plain text and the generic box, so still several ways.

### What does an error hold? Where and what

**Chosen:** each error carries where the problem is (a file, a line, a
column, a field, a variable) and what is wrong, as data rather than
inside a sentence.

### Who prints? Only the adapter

**Chosen:** errors start in the domain and are returned as
`Result<T, E>`, with `E` declared with thiserror; the domain composes
no message for the user. Only `src/main.rs`, today the only adapter,
prints.

**Warnings are errors too,** in the user's words: a missing content
date, a missing dates file, a missing list of blocked robots, an
article that cannot be read is the same error whether it stops the
build or not. In the pipeline it stops it, anywhere else it is a
warning. So the domain returns these errors as values, `is_ci` left it,
and one function of `src/main.rs`, `tolerate`, decides between a
warning and a stop. The adapter no longer passes the domain the reason
why a file could not be read, as text: it passes the content, or
nothing.

### Errors first, or the separation of the domain from the adapters? Errors first

**Chosen:** this card gave the domain its errors without designing any
port; the errors on the adapter's side (a file that cannot be read, a
variable not set, a write that fails) are gathered in `BuildError`, in
`src/main.rs`. Separating the domain from the adapters is a
[card of its own](../backlog/separate-the-domain-from-the-adapters.md),
which revisits the errors too.

**Why:** most errors were pure domain already (a title, a description,
a front matter, a line of the dates file or of the robots list) and do
not change with the separation. Starting from the separation would have
put two decisions in one slice: ports carrying the old errors, or
errors redesigned together with them.

**Rejected:** the separation first, which the user considered: errors
designed once, in their final place, but inside a much larger change
with many open decisions.

### Do the messages stay the same? Only where a new one says no more

**Chosen:** a message may change when the new form carries more
information. Each changed message was shown to the user before and
after, before its commit. The built site stays byte-identical.

**Chosen by the agent,** with the user informed: a warning reads as the
error, then what the build does instead (`<error>; <instead>`), so it
holds word for word the error that would stop the pipeline; and the
YAML reader's position is written as compilers write it,
`<file>:<line>:<column>:`, which terminals and editors turn into a
link.

**Rejected:** messages strictly identical, which would have left the
doubled "error:" to another card.

### thiserror 2.0.21

**Chosen:** thiserror 2, by David Tolnay, under MIT or Apache 2.0. It
was already compiled as a dependency of Leptos (2.0.21, with
thiserror-impl), so it adds no code to the build: `Cargo.lock` gained
one line.

## Results

- Six commits of code, one per slice, each written by a sub-agent
  (Claude Sonnet 5.5) and reviewed before its commit:
  `f5c5e05` the six hand-written types; `2e930b1` the robots list;
  `fc2a92a` the sitemap; `f66f0ca` the articles, `settle` leaving the
  domain; `6ccfdd0` `BuildError` in place of the generic box;
  `7a68629` the YAML reader's position and message taken apart.
  Together they add 691 lines and remove 400 in `src/`, `Cargo.toml`
  and `Cargo.lock`; the unit tests went from 245 to 261, since the
  choice between warning and stop and the messages are now tested on
  their own.
- The site built after each commit is byte-identical to the one built
  before it. The messages were captured before and after each slice;
  only the ones announced changed, for example:

  | Case | Before | After |
  |---|---|---|
  | Robots list missing, locally | `warning: no blocked robots list (BLOCKED_ROBOTS_FILE is not set): the robots.txt blocks only the training signal` | `warning: no blocked robots list: BLOCKED_ROBOTS_FILE is not set; the robots.txt blocks only the training signal` |
  | Article left out, locally | `warning: article skipped: content/articles/x.md: front matter: ...` | `warning: content/articles/x.md: front matter: ...; the article is left out of the site` |
  | Site address not valid | `error: invalid site address ...` | `error: SITE_ADDRESS: invalid site address ...` |
  | Output folder cannot be made | `error: Not a directory (os error 20)` | ``error: cannot create the folder `.../out`: Not a directory (os error 20)`` |
  | A field missing in an article | ``content/articles/x.md: front matter: error: line 3 column 1: missing field `description` `` | ``content/articles/x.md:3:1: front matter: missing field `description` `` |

  With the position, a wrong date in an article points at the field
  again, which jiff's message alone no longer named
  ([card](2026-11-09_give-dates-a-module-of-their-own.md)).
- **Found on the way:** Tailwind reads every word of the Rust code. A
  parameter named `contents` added `.contents{display:contents}` to the
  stylesheet; the byte-by-byte comparison caught it and the parameter
  was renamed. It is the third time Tailwind reads more than it should;
  the user tied it to the missing separation of the domain from the
  adapters ([card](../backlog/let-tailwind-read-only-the-presentation.md)).
