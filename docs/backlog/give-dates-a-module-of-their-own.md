# Give dates a module of their own

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** noticed, not started.

## Why

Two types hold a day, each written inside the module that uses it:

- `Date` in `src/article_page.rs`, the date of an article: with the
  articles' card it is read from the front matter (`YYYY-MM-DD`,
  checked to be a day of the calendar), and written for the page in two
  forms, `2023-04-30` and `30 April 2023`;
- `ContentDate` in `src/sitemap.rs`, the date of a content file's last
  change, read from the dates file written by the content dates step.

Both read the same form of day, each in its own way. The user wants
dates to be handled either by a widely used library or, if one is too
heavy, by a module of their own, instead of being buried in the
modules that use them.

The user kept it out of the articles' card, so that each decision
stays in a commit of its own.

## Known from the start

- **Rust's standard library has no type for a day of the calendar.**
- **The libraries checked** on 2026-11-08 on crates.io, none of them
  already in the project, all under MIT or a similar licence:
  - chrono 0.4.45: with its optional parts off, it needs only
    num-traits, already brought in by serde-saphyr, so it would add
    itself only; it reads `2023-04-30` checking the day exists, and
    writes `30 April 2023`;
  - time 0.3.55: four more libraries;
  - jiff 0.2.40: the newest; it brings time zones by default, which the
    site does not need.
- **This card takes over a question** first written in the
  [thiserror card](refactor-errors-with-thiserror.md): whether the two
  types of day become one.

## Questions

- A library, and which one, or a module written for the site.
- Whether `Date` and `ContentDate` become one type.
