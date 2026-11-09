# Give dates a module of their own

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](../done/2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** being worked on, since 2026-11-09.

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
  [thiserror card](../backlog/refactor-errors-with-thiserror.md):
  whether the two types of day become one.
- **The two types do not agree today.** `Date` checks the form, that
  the year is not zero and the days of each month, leap years
  included. `ContentDate` checks the form, the month from 1 to 12 and
  the day from 1 to 31 only, so it takes `2026-02-31` and the year
  `0000`.

## Answers

### A library: jiff

**Chosen:** [jiff](https://crates.io/crates/jiff) 0.2, with its
optional parts off (`default-features = false`, `std` only): no time
zones, and only jiff-core besides itself. Licence: Unlicense or MIT.
Chosen by the user, on the agent's advice, on 2026-11-09.

**Why:** the user does not want to write by hand what a widely used
library already does. Jiff's standard reading of a day, the ISO form,
is strict on its own: no check of the form is left to the site. It
writes `30 April 2023` itself (`strftime`), and its errors say what is
wrong, for example "parameter 'day' for \`2023-02\` is invalid, must
be in range \`1..=28\`".

What the three libraries read, tried by the agent on 2026-11-09 with
each one's optional parts off:

| Text | chrono 0.4.45 | time 0.3.55 | jiff 0.2.41 |
|---|---|---|---|
| `26-11-06`, a two-digit year | taken, as year 26 | refused | refused |
| `2026-11-6`, a one-digit day | taken | refused | refused |
| `+2026-11-06`, a sign | taken | taken | refused |
| ` 2026-11-06`, a leading space | taken | refused | refused |
| `2023-02-30`, no such day | refused | refused | refused |
| `0000-01-01`, the year zero | taken | taken | taken |
| `20230430` and `+002023-04-30` | refused | refused | taken, as the right day |
| Libraries added to the project | 1 | 6 | 2 |

**Rejected:**

- **A module written for the site:** it would write again what a
  library does.
- **chrono,** the most used, first chosen by the user: no format makes
  its reading strict (`%Y-%m-%d`, `%F` and `%0Y-%0m-%0d` all take
  `26-11-06` as year 26; `%C%y` stops it but reads `126-11-06` as
  year 1206). It would need a check of the form written for the site.
  The user's condition: a year written with two digits must not pass.
- **time:** five more libraries, and it takes a sign before the year.

### One type: jiff's day

**Chosen:** `Date` and `ContentDate` both go; the articles and the
sitemap use jiff's day (`jiff::civil::Date`) directly. No module of
the site's own is left to write.

**Why:** with a strict library there is no rule of the site's own left
to keep in one place.

**What changes, beyond the structure:** the build takes the forms jiff
takes and today's types refused (the year zero, `20230430`,
`+002023-04-30`), and the messages for a wrong date become jiff's.

### The year zero passes

**Chosen:** the build takes `0000-01-01`, as jiff does. Chosen by the
user, on the agent's proposal, on 2026-11-09.

**Why:** no one writes the year zero in an article by mistake, and
stopping it would bring back a check of the site's own.

**Rejected:** refusing it, as the articles do today: a rule of the
site's own for a mistake no one makes.
