# Domain types: Article.body, optional-field normalization — EP-001-UC-001-S003

Scope: this document covers the `/parse-dont-validate` step of
[EP-001-UC-001-S003](../stories/EP-001-UC-001-S003-generare-sito-statico-ssg.md)'s
design pipeline, following
[docs/architecture/hexagonal.md](../architecture/hexagonal.md) (S003 section:
ports/adapters, `src/pages/` presentation module) and preceding
`/sw-practices`. It extends
[docs/design/article.md](article.md) (S001) — same `Article`/`RawFrontmatter`
types, same `thiserror`/no-`anyhow` conventions — rather than replacing it.

## `Body`

### What it represents
The markdown content of an article — everything after the frontmatter block.

### Invariants
- Non-empty after trimming whitespace. An article with no body has nothing
  to publish; this is caught at construction, not downstream when
  ARTICLE-PAGE tries to render an empty content area.
- No length, encoding, or markdown-well-formedness constraint. Nothing in
  EP-001 or S003's AC requires one, and inventing a limit (e.g. a max
  length) now would be validating a scenario that isn't asked for — the
  moment a real constraint surfaces (a rendering NFR, a storage limit) it
  becomes a new invariant on this same type, not a redesign.

### Input origin
`EXTERNAL` — raw `Option<String>` field of `RawFrontmatter`, everything
`FilesystemContentSource::extract_yaml_block` currently discards after the
second `---` fence.

### Illegal states eliminated
An article can no longer be constructed with no content — `Article::new`
rejects it the same way it already rejects a missing title or date.

### Smart constructor
`Body::parse(raw: Option<&str>) -> Result<Body, BodyError>`

### Error type
```rust
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BodyError {
    #[error("article body is missing or empty")]
    Missing,
}
```
Single variant, unlike `Title`/`PublicationDate`/`Slug` — there is no
"malformed" shape for free-text markdown, only "present" or "absent".

### Boundary location
`Article::new`, via `Body::parse`. `Article` gains a `body: Body` field;
`ArticleError` gains `Body(#[from] BodyError)`, same one-variant-per-field
pattern as `Date`/`Slug`/`Tags`/`Title`.

---

## `Abstract` and `ImagePath` — optional fields, normalized on parse

`abstract_text`/`image` stay **optional** at the `Article` level (S001
already decided this — their *effective* display value is S003's presentation
concern, not a domain invariant). What S001 left unaddressed, and this story's
AC surfaces concretely (`abstract`/`immagine di sintesi assente`), is a real
gap: today both fields are plain `Option<String>` with **zero normalization**.
Frontmatter `abstract: ""` (present but blank) is stored as `Some("")`, not
`None` — a value that reads as "present" to any caller checking
`.is_some()`, even though it isn't a usable abstract. Left as-is, S003's
fallback logic (`effective_abstract`/`effective_image`, below) would fail to
trigger for a blank-but-present frontmatter key: a real illegal state,
reachable from real frontmatter, not a hypothetical.

Both types share the same shape — a non-blank `String` wrapper — and the
same constructor shape: parsing an *optional* field never rejects a blank
value as an error; it treats blank the same as absent and normalizes to
`None`. There is no `Missing` error variant because there is nothing to
reject — `None` is a fully legal outcome, not a failure.

### `Abstract`
**What it represents:** an author-supplied abstract for an article, when
one was written.
**Invariant:** if present at all (`Some`), non-blank after trimming.
**Smart constructor:** `Abstract::parse(raw: Option<&str>) -> Option<Abstract>`
— no `Result`, no error type: a blank or absent input isn't rejected, it
collapses to `None`.

### `ImagePath`
**What it represents:** an author-supplied summary-image reference, when
one was written. No format constraint beyond non-blank (whether it's a
repo-relative path, an absolute `/assets/...` path, or a URL is a content
convention, not something this type enforces — nothing today requires
choosing one).
**Invariant:** if present at all (`Some`), non-blank after trimming.
**Smart constructor:** `ImagePath::parse(raw: Option<&str>) -> Option<ImagePath>`

### Illegal states eliminated
`Article.abstract_text: Option<Abstract>` and `Article.image: Option<ImagePath>`
can no longer hold a blank-but-`Some` value. `.is_some()` becomes a
trustworthy "the author actually wrote one" check — exactly the check
`effective_abstract`/`effective_image` (below) depend on.

### Input origin
`EXTERNAL` — same `RawFrontmatter` fields as today, just parsed instead of
passed through.

### Boundary location
`Article::new`, via `Abstract::parse`/`ImagePath::parse`. No new
`ArticleError` variant — these constructors cannot fail.

---

## Presentation view-model: `effective_abstract`

**Not a domain type.** Confirmed in `/hexagonal-architecture`
(`docs/architecture/hexagonal.md`, Presentation section): a plain function in
`src/pages/view_model.rs`, deliberately outside `domain/` — the truncation
length is presentation/brand policy, not a fact about what an `Article` is.
(`effective_image` — the analogous function for images — no longer exists on
its own: superseded below by `resolve_image` + `effective_image_path`, split
across domain and presentation once the "referenced but missing on disk"
case required I/O.)

### Why its output stays a plain `String`, not a new type
Parse-dont-validate protects a value against **re-validation downstream** —
it earns its keep when a value crosses a boundary and gets checked again and
again. `effective_abstract`'s output has no such future: it is produced once,
inside a single render call, interpolated directly into a view, and then
discarded — never stored, never passed to another function that would
re-check it, never crosses another boundary. There is no illegal state a
wrapper type could rule out that the truncation function doesn't already
rule out by construction (it always returns non-empty, already-plain text).
Introducing `EffectiveAbstract(String)` here would be a type with no
invariant of its own to enforce — ceremony, not protection.

### Truncation rule (confirmed by Luca, 2026-08-30: 200 characters)

```
effective_abstract(article: &Article) -> String
  // article.abstract_text (Abstract) present → its text, as-is
  // absent → truncate article.body (Body):
  //   body.char_count() <= 200 → the body, unchanged (nothing was cut,
  //     no ellipsis)
  //   body.char_count() > 200  → the body cut at the last word boundary
  //     at or before 200 characters, with a trailing "…"
```

Word-boundary cut (never mid-word) and the "…" suffix are my own default,
not something Luca confirmed beyond the 200-character figure — a standard,
low-impact, easily-revisited convention, not a design decision worth its own
checkpoint. The exact character-by-character result for a given test body is
left to `/test` (same tactic already used for markdown→HTML rendering,
below), not hand-computed here.

### Boundary location
Called from `src/pages/{article_page,listing_page,home_page}.rs` when
building each page's view — not from the domain, not from `ContentSource`.

---

## Presentation view-model: `ResolvedImage` — image existence resolution (residuality extension, agreed 2026-08-30)

`/residuality`'s Stressor Analysis (2026-08-30, full detail in Hindsight bank
`personal-blog`, tag `story:EP-001-UC-001-S003`) surfaced a real gap in the AC
"immagine di sintesi assente": an article can also **reference** an image
that was never committed to the content repo, or was later deleted — a
distinct case from "no image was set at all". `effective_image` above only
handles the latter.

### The type

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolvedImage {
    Own(ImagePath),
    Fallback { attempted: Option<ImagePath> },
}
```

`attempted` distinguishes the two fallback causes: `None` means the article
never set an image (expected, not a problem); `Some(path)` means an image
*was* referenced but does not exist on disk (a real content defect worth
surfacing). Neither is an error — resolution *finding no file* always
produces a usable value by design; "the referenced file is missing" is not a
construction failure, it's an outcome this type exists to represent. A
genuine I/O failure reading the content repo is still a real error, handled
by `FetchError::Io` exactly as it already is for `ContentSource::exists` (see
Boundary location below) — the "no error" property is about the missing-file
case specifically, not an absolute absence of `Result` anywhere in the path.

### Why this needs a dedicated type, not a plain `&str` (unlike `effective_abstract`/`effective_image`)

The existing deroga (`effective_abstract`/`effective_image`'s output staying
a plain `String`/`&str`, above) applies only when the value has a single
consumer. `ResolvedImage`'s result has **two**: the path actually rendered in
the page, and a build-time audit signal (a logged warning when an author's
image reference has gone stale — an FMEA finding from the same Stressor
Analysis). Both must agree on what happened for the same resolution;
collapsing straight to a rendered path would either lose the audit signal or
force a second, independently-computed check that could drift from the
first. This distinction was found by applying the existing single-consumer
deroga to this new case and discovering it didn't hold — a design-craft
lesson in its own right, not just applied silently here.

### Boundary location — mediated by `ContentSource`, not raw I/O in the composition root (revised 2026-08-30)

Checking whether a referenced image exists on disk is I/O — but the
composition root already funnels every other piece of content-repo I/O
through `ContentSource` (`get_by_slug`/`list_published`/`exists`), never raw
`std::fs`. Doing it differently for images here would be inconsistent, and
would cost real testability: any page component test needing a
`ResolvedImage` would need real temp-dir I/O instead of the existing
`InMemoryContentSource` fake — breaking the "view_model — Unit, None" row of
the Testing strategy table below for anything that touches images. Corrected
design: `ContentSource` gains a new operation, and `resolve_image` becomes a
domain function mediated by it, parallel to `ensure_slug_is_unique`
(`src/domain/slug_uniqueness.rs`) — see
[docs/architecture/hexagonal.md](../architecture/hexagonal.md) for the full
port/service placement. `resolve_image` itself stays free of logging; the
audit warning (below) is the caller's responsibility, once it observes the
already-resolved outcome, not something baked into resolution itself:

```rust
// src/domain/image_resolution.rs
pub fn resolve_image(
    source: &impl ContentSource,
    image: Option<&ImagePath>,
) -> Result<ResolvedImage, FetchError> {
    let Some(path) = image else {
        return Ok(ResolvedImage::Fallback { attempted: None });
    };
    if source.image_exists(path)? {
        Ok(ResolvedImage::Own(path.clone()))
    } else {
        Ok(ResolvedImage::Fallback { attempted: Some(path.clone()) })
    }
}
```

### `images_dir` — where article images physically live (temporary, confirmed 2026-08-30)

Article images live in `assets/images/` inside the same dedicated content
repo checkout as the articles themselves — **not** this application repo's
own `assets/`. Now an internal detail of `FilesystemContentSource` (a second
field alongside `articles_dir`, both derived from the same
`content_repo_path` constructor argument — no new environment variable, no
composition-root-level config), not something the composition root computes
itself. Explicitly a temporary placement, confirmed by Luca as good enough
for this story: a proper asset-storage design (a separate
`AssetSource`/`ImageSource` port, possibly a different physical location) is
backlog, its own future story, not addressed here.

### The fallback asset

A dedicated SVG (`assets/images/article-image-not-found.svg` in this
**repo's own** source tree, not the content repo — a brand/UI asset, not
article content): a new minimal placeholder, not a reuse of
`ostia_sea_top_image.webp` (confirmed by Luca 2026-08-30 — that asset stays
the layout's fixed background, unrelated to article content). Its site path
is a `view_model.rs` constant; no I/O needed to reference it — the site's own
assets are guaranteed present by the build, unlike an author-referenced image
`resolve_image` must check.

**Site path corrected 2026-09-01**, found via a real `cargo leptos build`,
not assumed: `cargo-leptos`'s `assets-dir` sync flattens `assets/*` straight
into `site-root/*` — `assets/images/x.svg` in this repo's source becomes
`/images/x.svg` in the generated site, **not** `/assets/images/x.svg`. The
original `FALLBACK_IMAGE_PATH` used the wrong prefix from S003's first
implementation slice (2026-08-31) through this correction — a real,
reproducible 404 in every build until now, never caught because nothing
had exercised a real `cargo leptos build` yet.

The same real-build check surfaced a second, related gap: an article's own
`ImagePath` (e.g. `cover.webp`, whatever the author wrote) was rendered
as-is — a relative path, resolving wrong from a nested route like
`/articles/hello-world`, and pointing at a file cargo-leptos never
copies into the generated site at all (content-repo images and this
app's own `assets-dir` are different physical locations). **Minimal fix
confirmed by Luca 2026-09-01** (a full fix needs its own future
epic — multiple topics: real asset-storage location, collision
avoidance, `AssetSource`/`ImageSource` port): the composition root
(`main.rs`) copies the content repo's `images_dir` into the generated
site's `/images` directory (same directory the fallback SVG already
lives in — accepted collision risk, not solved here), and
`effective_image_path` roots an `Own` path under that same
`/images` prefix instead of using it unprefixed.

```rust
const ARTICLE_IMAGES_BASE_PATH: &str = "/images";
const FALLBACK_IMAGE_PATH: &str = "/images/article-image-not-found.svg";

pub fn effective_image_path(resolved: &ResolvedImage) -> String {
    match resolved {
        ResolvedImage::Own(path) => format!("{ARTICLE_IMAGES_BASE_PATH}/{}", path.as_str()),
        ResolvedImage::Fallback { .. } => FALLBACK_IMAGE_PATH.to_string(),
    }
}
```

### Audit signal (FMEA finding, 2026-08-30)

When `resolve_image` returns `Fallback { attempted: Some(path) }` — an image
was referenced but not found — the caller (wherever a page resolves an
`Article`'s image, via the same `ContentSource` context already in scope)
logs a build-time warning (`eprintln!`, no new dependency: the SSG generator
runs once per build, not as a long-running service, so a build-log line is a
sufficient audit trail; a real structured-logging pipeline is out of scope).
`Fallback { attempted: None }` logs nothing — the author simply didn't set
an image, not a defect. Logging is deliberately not inside `resolve_image`
itself: the domain function stays free of side effects beyond the one port
call, same discipline already followed by `ensure_slug_is_unique`.

---

## Dependency decision: markdown → HTML (agreed 2026-08-24, `/sw-practices`)

`Body` carries raw markdown text; ARTICLE-PAGE needs actual HTML. **Adopted:
`pulldown-cmark`** (checked against crates.io before adopting, per this
project's established dependency-audit practice — [docs/design/article.md](article.md)'s
own precedent: max stable `0.13.4`, last published 2026-05-20, actively
maintained, pure-Rust CommonMark pull parser, no transitive C dependency).
Rendering the `Body` → HTML string is presentation logic (same reasoning as
`effective_abstract`/`effective_image`): it belongs in `src/pages/`, not in
`domain/` — `Body` itself stays raw markdown text, never pre-rendered at
construction, so nothing in the domain needs to know HTML exists. Not added
to `Cargo.toml` yet — deferred to the `feat` commit that actually calls it
from `src/pages/article_page.rs` (avoids an added-but-unused dependency
sitting in the manifest in the meantime, same discipline as `article.md`'s
deferred `chrono`).

## Data-flow refinement (agreed 2026-08-24, `/sw-practices`)

`/hexagonal-architecture` decided article data reaches page components as
already-resolved data, no `#[server]`/`Resource` needed. Confirmed against
`leptos_router`'s actual static-route API
(`leptos-rs/leptos examples/static_routing/src/app.rs`): a routed
`<Route path=path!("/articles/:slug") view=ArticlePage ssr=SsrMode::Static(...)/>`
invokes `ArticlePage` as a zero-argument view function, the same way for
every route — it cannot receive a custom `Article` prop directly from a
parent. The refined mechanism, still consistent with "no server function,
no Resource": `ArticlePage` reads the `:slug` route param via
`leptos_router::hooks::use_params_map()`, then resolves the `Article`
synchronously through a `ContentSource` obtained via `expect_context`
(provided once, at the composition root, wrapping `<Router>` — the same
adapter instance used to build `prerender_params`). `ListingPage`/`HomePage`
(parameterless routes) read the full sorted article list the same way, via
context, rather than a route param.
