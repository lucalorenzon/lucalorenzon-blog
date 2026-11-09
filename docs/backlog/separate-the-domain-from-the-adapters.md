# Separate the domain from the adapters

- **Opened:** 2026-11-09, by the user, while answering the card
  [refactor the code's errors with thiserror](../wip/refactor-errors-with-thiserror.md).
- **Status:** noticed, not started. To be picked up after the thiserror
  card.

## Why

The user wants the code split into a core and its adapters. The core is
the domain: pure business, independent of any infrastructure or
technology. It does not print anything; it returns `Result<T, E>`, with
`E` a domain error declared with thiserror. The adapters take
information from the external dependencies (the file system, the HTTP
protocol) and give information back to them.

Today all the code sits at the same level, with no such boundary. And
the file system is not one concept: it implements several, which the
user named as the source of the content, the publication folder and the
configuration files.

## Known from the start

- **Errors come first,** as the user decided on 2026-11-09: the
  thiserror card gives the domain its error types and moves out of it
  the choice between a warning and a stop, and the printing. The errors
  on the adapters' side (a file that cannot be read, a variable not set,
  a write that fails) stay gathered in `src/main.rs` until this card
  shares them out among the adapters. **This card revisits the errors
  too:** some types may change once the source of the content is
  designed; `ArticleError`, for example, carries the Markdown file's
  path, a concept of the source rather than of the article.
- **Content and configuration enter the page modules at compile time,**
  through `include_str!`: the articles (`ARTICLE_FILES` in
  `src/article_page.rs`), the about, licence and curriculum pages, and
  the heading's limit in `src/page_frame.rs`. The source of the content
  is, today, the compiler.
- **`src/main.rs` does everything else at once:** it reads the
  environment variables and the generated files (content dates, blocked
  robots), writes the publication folder and prints.
- **The site has no server:** what it gives back today is files.
- **The presentation is an adapter too:** the Leptos views and their
  named styles. Today they share `src/` with the domain, so Tailwind,
  which must read the views, reads every Rust file and turns any
  matching word into a rule of the stylesheet
  ([card](let-tailwind-read-only-the-presentation.md)). Once separated,
  Tailwind could read the presentation's files only.
- **Related cards:** [find the articles in the content folder](find-articles-in-content-folder.md),
  which reads the content when the build runs instead of when it is
  compiled, and [use a structured configuration file](use-a-structured-configuration-file.md).
- **Tools:** the `hexagonal-architecture` skill for ports and adapters;
  the [full round of `residuality`](run-full-residuality-round.md)
  could stress the result.

## Questions

- Which ports the domain needs: one per concept the file system
  implements today, and whether the generated inputs (content dates,
  blocked robots) are a concept of their own.
- Whether the content stays read at compile time, or this card comes
  together with, or after, finding the articles in the content folder.
- Where the choice between a warning and a stop lives: in an
  application layer between the domain and the adapters, or in the
  adapter that starts the build.
- Which errors move between the domain and the adapters once the
  boundary exists.
