# Refactor the code's errors with thiserror

- **Opened:** 2026-11-08, by the user, while choosing how the articles'
  front matter is checked
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** being worked on, since 2026-11-09.

## Why

The user wants the code refactored to use
[thiserror](https://crates.io/crates/thiserror), the library that
writes the boilerplate of an error type from a short declaration.
Today the errors are written in several ways: plain text
(`Result<_, String>`, for example in `src/sitemap.rs` and
`src/robots_txt.rs`), a generic box in the build program
(`Box<dyn Error>`, in `src/main.rs`), and, from the articles' card,
error types written by hand.

## Known from the start

- **A refactor:** what the build writes stays the same; only the
  code's structure changes. The messages it prints were meant to stay
  the same too; see the answers.
- **thiserror is a new dependency,** to be audited as any other.
- **The YAML reader's messages begin with a doubled "error:",** as in
  `front matter: error: line 4 column 7: ...`
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).

## Answers

All chosen by the user on 2026-11-09, on the agent's questions and
proposals.

### Every error moves to thiserror

**Chosen:** all three ways of writing an error give way to types
declared with thiserror: the hand-written types (`InvalidSiteAddress`,
`HeadingError`, `TitleError`, `DescriptionError`, `FrontMatterError`,
`ArticleError`), the plain text (`Result<_, String>` in
`src/sitemap.rs`, `src/robots_txt.rs` and `settle` in
`src/article_page.rs`) and the generic box (`Box<dyn Error>` in
`src/main.rs`). The hand-written `Display` and `Error`
implementations no longer needed are removed.

**Why:** the card was opened because errors are written in several
ways; one mechanism, the same annotations on every error type, is the
answer. The question as first written, "only those matched by the
code", separated nothing: no error is told apart by the build's code,
only by the unit tests.

**Rejected:** moving only the six hand-written types, which would have
left the plain text and the generic box, so still several ways.

### Every error says where and what

**Chosen:** each error carries where the problem is (a file, a line, a
field) and what is wrong, as data rather than inside a sentence.

### Errors start in the domain and are returned; only the adapter prints

**Chosen:** the domain returns `Result<T, E>`, with `E` its own error
declared with thiserror, and composes no message for the user; only
`src/main.rs`, today the only adapter, prints.

**Warnings are errors too.** A missing content date, a missing dates
file or a missing list of blocked robots is the same error whether it
stops the build or not: in the pipeline it stops it, anywhere else it
is shown as a warning. So the domain returns these errors as values,
`is_ci` leaves it, and the choice between a warning and a stop is made
in `src/main.rs`. The adapter no longer passes the domain the reason
why a file could not be read, as text: it passes the content, or
nothing.

### Errors first, the separation of the domain from the adapters after

**Chosen:** this card gives the domain its errors without designing
any port; the errors on the adapter's side (a file that cannot be read,
a variable not set, a write that fails) stay gathered in a type of
`src/main.rs`. The separation of the domain from the adapters is a
[card of its own](../backlog/separate-the-domain-from-the-adapters.md),
which revisits the errors too.

**Why:** most errors are pure domain already (a title, a description,
a front matter, a line of the dates file or of the robots list) and
would not change with the separation; the adapter's ones live in
`src/main.rs` anyway. Starting from the separation would have put two
decisions in one slice: ports carrying today's errors, or errors
redesigned together with them.

**Rejected:** the separation first, which the user considered: errors
designed once, in their final place, but inside a much larger change
with many open decisions.

### Messages may change when they say more

**Chosen:** a message may change when the new form carries more
information, for example the doubled "error:" of the YAML reader,
which goes once its position (line and column, which serde-saphyr
gives as data) is taken apart from its message. Each changed message
is shown to the user before and after, before its commit. The built
site stays byte-identical.

**Rejected:** messages strictly identical, which would have left the
doubled "error:" to another card.

### thiserror 2.0.21

**Chosen:** thiserror 2, by David Tolnay, under MIT or Apache 2.0. It
is already compiled as a dependency of Leptos (2.0.21, with
thiserror-impl), so it adds no code to the build.

### Six slices, one commit each

1. The six hand-written types move to thiserror, with the dependency;
   messages unchanged.
2. `robots.txt`: the domain receives the list or nothing, no longer
   knows about the pipeline, and returns typed errors with their line.
3. The sitemap, the same way: it returns the sitemap and the missing
   dates as typed errors; `src/main.rs` decides warning or stop.
4. The articles: `settle` leaves the domain for `src/main.rs`.
5. The build: the generic box becomes a type declared with thiserror,
   gathering the domain's errors and the adapter's; write errors gain
   the file's path.
6. The YAML errors: where and what taken apart, the doubled "error:"
   gone.

Each slice keeps the built site byte-identical and passes the
pipeline's checks before its commit.
