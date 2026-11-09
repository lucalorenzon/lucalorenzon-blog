# Give dates a module of their own

- **Opened:** 2026-11-08, by the user, while reviewing the code of the
  card on the articles' front matter
  ([card](2026-11-08_read-article-fields-from-front-matter.md)).
- **Status:** done, 2026-11-09.

## Why

Two types held a day, each written inside the module that used it:

- `Date` in `src/article_page.rs`, the date of an article, read from
  the front matter and written for the page in two forms, `2023-04-30`
  and `30 April 2023`;
- `ContentDate` in `src/sitemap.rs`, the date of a content file's last
  change, read from the dates file written by the content dates step.

Both read the same form of day, each in its own way, and they did not
agree: `Date` checked the days of each month, leap years included, and
refused the year zero; `ContentDate` checked the day from 1 to 31 only,
so it took `2026-02-31`. Rust's standard library has no type for a day
of the calendar. The user wanted dates handled by a widely used
library or, if one was too heavy, by a module of their own, instead of
being buried in the modules that use them. The card also took over a
question first written in the
[thiserror card](../backlog/refactor-errors-with-thiserror.md): whether
the two types of day become one.

The user kept it out of the articles' card, so that each decision
stays in a commit of its own.

## Answers

### A library or a module of the site's own? jiff

**Chosen:** [jiff](https://crates.io/crates/jiff) 0.2.41, with its
optional parts off (`default-features = false`, `std` and `serde`
only): no time zones, and only jiff-core besides itself. Licence:
Unlicense or MIT. Chosen by the user, on the agent's advice.

**Why:** the user does not want to write by hand what a widely used
library already does. jiff's standard reading of a day, the ISO form,
is strict on its own, so no check of the form is left to the site. It
writes `30 April 2023` itself, and it reads the front matter's field
directly, keeping the line and column of the error.

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
  year 1206). It would have needed a check of the form written for the
  site, and the user's condition was that a year written with two
  digits must not pass.
- **time:** five more libraries, and it takes a sign before the year.

### Do the two types become one? Yes, jiff's day

**Chosen:** `Date` and `ContentDate` are both gone; the articles and
the sitemap use jiff's day (`jiff::civil::Date`) directly. No module of
the site's own was written, despite the card's title.

**Why:** with a strict library there is no rule of the site's own left
to keep in one place.

**What changed, beyond the structure, accepted by the user:**

- the build takes forms the old types refused: `20230430`,
  `+002023-04-30`, and the year zero;
- the dates file is refused for a day that does not exist, which
  `ContentDate` took; git never writes one;
- the messages for a wrong date are jiff's. For `2023-02-30` the build
  used to say "field \`date\`: \`2023-02-30\` is not a day of the
  calendar"; it now says "parsed date is not valid: parameter 'day'
  for \`2023-02\` is invalid, must be in range \`1..=28\`". The file,
  line and column before it are unchanged. The user saw both and kept
  jiff's, rather than adding the field's name and value in code of the
  site's own.

### Does the year zero pass? Yes

**Chosen:** the build takes `0000-01-01`, as jiff does.

**Why:** no one writes the year zero in an article by mistake, and
stopping it would bring back a check of the site's own.

**Rejected:** refusing it, as the articles did: a rule of the site's
own for a mistake no one makes.

## Results

- Two commits of code, one per type: `5c5074b` for the articles and
  the home page, `c25996c` for the sitemap. Together they remove 190
  lines of `src/` and add 59, tests included.
- The site built after each commit is byte-identical to the one built
  before it, the sitemap's dates included.
- The tests on dates now go through the front matter and the dates
  file, so they check the site's rule rather than a type: a two-digit
  year is refused, and the forms the user accepted are checked to
  pass.
