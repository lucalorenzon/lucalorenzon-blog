# Refactor the code's errors with thiserror

- **Opened:** 2026-11-08, by the user, while choosing how the articles'
  front matter is checked
  ([card](../wip/read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

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

- **A refactor:** what the build writes and the messages it prints stay
  the same; only the code's structure changes.
- **thiserror is a new dependency,** to be audited as any other.
- **Two types hold a day:** `Date` in `src/article_page.rs` and
  `ContentDate` in `src/sitemap.rs`, each with its own reading of
  `YYYY-MM-DD` once the articles' dates come from the front matter.

## Questions

- Which errors move to thiserror: all of them, or only those that are
  matched by the code, leaving plain text where the message is all
  that matters.
- Whether the two types of day become one.
