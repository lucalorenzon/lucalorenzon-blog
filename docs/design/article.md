# Domain types: Article and its value objects — EP-001-UC-001-S001

Scope: this document covers only what [EP-001-UC-001-S001](../stories/EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md) delivers — the `Article` entity, its four value objects, the `RawFrontmatter` boundary DTO, and `ArticleError`. It is the `/parse-dont-validate` step of that story's design pipeline, following [docs/architecture/hexagonal.md](../architecture/hexagonal.md) (ports/adapters) and preceding `/sw-practices` (naming, error handling, bootstrap — including the actual dependency addition and first `src/` files).

## Dependency decisions (agreed 2026-08-23)

- **`thiserror`**: adopted for `ArticleError` and all value-object errors below. Evaluated against hand-written `Display`/`Error` impls — rejected as unnecessary boilerplate for five near-identical small enums; `thiserror` is compile-time only (proc-macro), no runtime cost.
- **`anyhow`**: explicitly excluded from the domain layer. It type-erases into `anyhow::Error`, which would break the AC requirement that a construction failure identify its causing field via pattern matching (`ArticleError::Slug(...)`). Reserved, if ever, for an application/boundary layer (e.g. mapping errors to an HTTP response) in a future story — not decided here.
- **Calendar date validation**: hand-rolled, `std`-only. The standard library has no calendar-aware date type or parser (`std::time` is `SystemTime`/`Instant`/`Duration` only — not calendar-aware, no `FromStr` for a date). `chrono` was considered and deferred: it would pay off if a later story (S003, SSG generation) needs date arithmetic/formatting across many articles, but a single self-contained validation function does not justify the dependency now.

---

## `PublicationDate`

### What it represents
The publication date declared in an article's frontmatter.

### Invariants
- Format `YYYY-MM-DD` (ISO-8601 calendar date), exactly 3 numeric groups (`4-2-2` digits) separated by `-`.
- Must be a calendar date that actually exists (month 1-12; day within that month's length, including leap-year Feb 29).
- No constraint on past/future: an article may be scheduled ahead of today.

### Input origin
`EXTERNAL` — raw `Option<String>` field of `RawFrontmatter`, read from the frontmatter.

### Illegal states eliminated
A `PublicationDate` in memory is always a real calendar date in canonical `YYYY-MM-DD` form — no downstream code re-checks digit ranges or leap years.

### Smart constructor
`PublicationDate::parse(raw: Option<&str>) -> Result<PublicationDate, PublicationDateError>`

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PublicationDateError {
    #[error("publication date is missing")]
    Missing,
    #[error("invalid publication date {raw:?}: expected YYYY-MM-DD, a real calendar date")]
    Malformed { raw: String },
}
```

### Boundary location
`Article::new`, via `PublicationDate::parse`.

---

## `Slug`

### What it represents
The URL-safe identifier of an article. (Cross-article uniqueness is [EP-001-UC-001-S002](../stories/EP-001-UC-001-S002-verificare-unicita-slug.md)'s concern, not this type's.)

### Invariants
- Non-empty.
- Lowercase ASCII kebab-case: `^[a-z0-9]+(-[a-z0-9]+)*$` — no uppercase, spaces, underscores, doubled or leading/trailing hyphens.

### Input origin
`EXTERNAL`.

### Illegal states eliminated
No `Article` can carry a slug with an invalid charset — every `Slug` in memory is already URL-safe.

### Smart constructor
`Slug::parse(raw: Option<&str>) -> Result<Slug, SlugError>`

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SlugError {
    #[error("slug is missing")]
    Missing,
    #[error("invalid slug {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}
```

### Boundary location
`Article::new`, via `Slug::parse`.

---

## `Tag` and `Tags`

### What they represent
`Tag`: a single thematic label on an article. `Tags`: the non-empty set of tags an article carries.

### Invariants
- `Tag`: same charset rule as `Slug` — lowercase ASCII kebab-case. Agreed 2026-08-23: reused deliberately because a tag value flows into the static tag-menu URL (UC-005); a shared charset avoids a second "tag slug" derived type later.
- `Tags`: at least one `Tag`. Modelled as its own newtype (not just a length check inside `Article::new`) so "article with zero tags" is structurally unrepresentable, not merely runtime-checked.

### Input origin
`EXTERNAL` — `Vec<String>` field of `RawFrontmatter` (empty vec = no tags supplied).

### Illegal states eliminated
An `Article` can never hold an empty tag list — `Tags` cannot be constructed from one. Every `Tag` in memory already has a valid charset.

### Smart constructors
- `Tag::parse(raw: &str) -> Result<Tag, TagError>`
- `Tags::parse(raw: Vec<String>) -> Result<Tags, TagsError>` — parses each element via `Tag::parse`; returns on the **first** invalid element (no aggregation across the list — not required by any AC/AT row, and aggregating would be speculative complexity for an untested scenario).

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagError {
    #[error("invalid tag {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagsError {
    #[error("at least one tag is required")]
    Empty,
    #[error(transparent)]
    InvalidTag(#[from] TagError),
}
```

### Boundary location
`Article::new`, via `Tags::parse`.

---

## `Title`

### What it represents
The article's display title.

### Invariants
- Non-empty after trimming leading/trailing whitespace.
- No control characters (e.g. newline, tab) — must be single-line text.
- No maximum length enforced here (SEO/UX-driven limits, if any, are a UI concern — out of this story's scope, see S003).

### Input origin
`EXTERNAL`.

### Illegal states eliminated
No `Article` can carry a blank or multi-line-corrupted title.

### Smart constructor
`Title::parse(raw: Option<&str>) -> Result<Title, TitleError>` — stores the trimmed value.

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TitleError {
    #[error("title is missing")]
    Missing,
    #[error("invalid title {raw:?}: must be non-blank, single-line text")]
    Malformed { raw: String },
}
```

### Boundary location
`Article::new`, via `Title::parse`.

---

## `RawFrontmatter` (boundary DTO, not a domain type — reconfirmed)

Raw, unvalidated fields read from an article's frontmatter. Lives next to `Article`, not in `ports`, because it is the input half of `Article`'s own smart constructor. Consumed exactly once by `Article::new`, never propagated past it (agreed in `hexagonal.md`).

```rust
pub struct RawFrontmatter {
    pub date: Option<String>,
    pub slug: Option<String>,
    pub tags: Vec<String>,
    pub title: Option<String>,
    pub abstract_text: Option<String>,
    pub image: Option<String>,
}
```

`abstract_text`/`image` are read into `Article` as-is (`Option<String>`, no further parsing) — deliberately not modelled as value objects here, since this story has no invariant for them to encode (S003's responsibility).

---

## `Article`

### What it represents
A blog article, identity = `slug`.

### Invariants
`date`, `slug`, `tags` (≥1), `title` present and well-formed; `abstract`/`image` optional, read as-is.

### Input origin
Built once from an `EXTERNAL` `RawFrontmatter`; every other reference to an already-constructed `Article` is `INTERNAL`.

### Illegal states eliminated
No `Article` value can exist with a missing/malformed mandatory field, or with zero tags.

### Smart constructor
`Article::new(raw: RawFrontmatter) -> Result<Article, ArticleError>`

Fields are checked in a fixed order — date, slug, tags, title — and construction returns on the **first** failure. Agreed 2026-08-23: no multi-field error aggregation. Every AC/AT row for this story exercises exactly one invalid field at a time; aggregating into a `Vec<ArticleError>` would add a return-shape only a future, currently untested UX need (surfacing all frontmatter errors at once to the author) would justify. If that need materialises, it is a new AC on a follow-up story, not a silent addition here.

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArticleError {
    #[error("date: {0}")]
    Date(#[from] PublicationDateError),
    #[error("slug: {0}")]
    Slug(#[from] SlugError),
    #[error("tags: {0}")]
    Tags(#[from] TagsError),
    #[error("title: {0}")]
    Title(#[from] TitleError),
}
```
The causing field is the enum variant itself — no separate stringly-typed `ArticleField` is needed to satisfy the AC ("the error identifies the causing field").

### Boundary location
Wherever a `RawFrontmatter` is produced — currently only `FilesystemContentSource` ([hexagonal.md](../architecture/hexagonal.md)), which maps the outer `ArticleError` into `FetchError::Malformed`.

---

## Complete skeleton

```rust
// src/domain/value_objects/publication_date.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicationDate {
    year: u16,
    month: u8,
    day: u8,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum PublicationDateError {
    #[error("publication date is missing")]
    Missing,
    #[error("invalid publication date {raw:?}: expected YYYY-MM-DD, a real calendar date")]
    Malformed { raw: String },
}

impl PublicationDate {
    pub fn parse(raw: Option<&str>) -> Result<Self, PublicationDateError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(PublicationDateError::Missing)?;

        let malformed = || PublicationDateError::Malformed { raw: raw.to_string() };

        let parts: Vec<&str> = raw.split('-').collect();
        let [y, m, d] = parts.as_slice() else { return Err(malformed()) };
        if y.len() != 4 || m.len() != 2 || d.len() != 2 {
            return Err(malformed());
        }
        let (year, month, day) = (
            y.parse::<u16>().map_err(|_| malformed())?,
            m.parse::<u8>().map_err(|_| malformed())?,
            d.parse::<u8>().map_err(|_| malformed())?,
        );
        if !(1..=12).contains(&month) {
            return Err(malformed());
        }
        if day < 1 || day > days_in_month(year, month) {
            return Err(malformed());
        }
        Ok(Self { year, month, day })
    }
}

fn is_leap_year(year: u16) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => unreachable!("month already validated to 1..=12"),
    }
}
```

```rust
// src/domain/value_objects/slug.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slug(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum SlugError {
    #[error("slug is missing")]
    Missing,
    #[error("invalid slug {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}

impl Slug {
    pub fn parse(raw: Option<&str>) -> Result<Self, SlugError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(SlugError::Missing)?;

        if is_kebab_case(raw) {
            Ok(Self(raw.to_string()))
        } else {
            Err(SlugError::Malformed { raw: raw.to_string() })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Shared by Slug and Tag: lowercase ASCII kebab-case,
/// no leading/trailing/doubled hyphens.
pub(crate) fn is_kebab_case(s: &str) -> bool {
    !s.is_empty()
        && s.split('-').all(|segment| {
            !segment.is_empty() && segment.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

impl std::fmt::Display for Slug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
```

```rust
// src/domain/value_objects/tag.rs

use super::slug::is_kebab_case;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tag(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagError {
    #[error("invalid tag {raw:?}: expected lowercase kebab-case (a-z0-9, single hyphens)")]
    Malformed { raw: String },
}

impl Tag {
    pub fn parse(raw: &str) -> Result<Self, TagError> {
        let raw = raw.trim();
        if is_kebab_case(raw) {
            Ok(Self(raw.to_string()))
        } else {
            Err(TagError::Malformed { raw: raw.to_string() })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tags(Vec<Tag>);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TagsError {
    #[error("at least one tag is required")]
    Empty,
    #[error(transparent)]
    InvalidTag(#[from] TagError),
}

impl Tags {
    pub fn parse(raw: Vec<String>) -> Result<Self, TagsError> {
        if raw.is_empty() {
            return Err(TagsError::Empty);
        }
        let tags = raw
            .iter()
            .map(|t| Tag::parse(t))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self(tags))
    }

    pub fn as_slice(&self) -> &[Tag] {
        &self.0
    }
}
```

```rust
// src/domain/value_objects/title.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title(String);

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum TitleError {
    #[error("title is missing")]
    Missing,
    #[error("invalid title {raw:?}: must be non-blank, single-line text")]
    Malformed { raw: String },
}

impl Title {
    pub fn parse(raw: Option<&str>) -> Result<Self, TitleError> {
        let raw = raw
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or(TitleError::Missing)?;

        if raw.chars().any(|c| c.is_control()) {
            return Err(TitleError::Malformed { raw: raw.to_string() });
        }
        Ok(Self(raw.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
```

```rust
// src/domain/article.rs

use crate::domain::value_objects::{
    publication_date::{PublicationDate, PublicationDateError},
    slug::{Slug, SlugError},
    tag::{Tags, TagsError},
    title::{Title, TitleError},
};

pub struct RawFrontmatter {
    pub date: Option<String>,
    pub slug: Option<String>,
    pub tags: Vec<String>,
    pub title: Option<String>,
    pub abstract_text: Option<String>,
    pub image: Option<String>,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ArticleError {
    #[error("date: {0}")]
    Date(#[from] PublicationDateError),
    #[error("slug: {0}")]
    Slug(#[from] SlugError),
    #[error("tags: {0}")]
    Tags(#[from] TagsError),
    #[error("title: {0}")]
    Title(#[from] TitleError),
}

pub struct Article {
    date: PublicationDate,
    slug: Slug,
    tags: Tags,
    title: Title,
    abstract_text: Option<String>,
    image: Option<String>,
}

impl Article {
    pub fn new(raw: RawFrontmatter) -> Result<Self, ArticleError> {
        let date = PublicationDate::parse(raw.date.as_deref())?;
        let slug = Slug::parse(raw.slug.as_deref())?;
        let tags = Tags::parse(raw.tags)?;
        let title = Title::parse(raw.title.as_deref())?;

        Ok(Self {
            date,
            slug,
            tags,
            title,
            abstract_text: raw.abstract_text,
            image: raw.image,
        })
    }

    pub fn slug(&self) -> &Slug {
        &self.slug
    }
}
```

---

## Next steps
- `/sw-practices` — naming/error-handling conventions, module bootstrap, and the actual `Cargo.toml` addition of `thiserror` + first `src/domain` files.
- `/story-size EP-001-UC-001-S001`.
