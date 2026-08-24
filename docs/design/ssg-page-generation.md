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

## Presentation view-model: `effective_abstract` / `effective_image`

**Not domain types.** Confirmed in `/hexagonal-architecture`
(`docs/architecture/hexagonal.md`, Presentation section): these are plain
functions in `src/pages/view_model.rs`, deliberately outside `domain/` —
the truncation length and the fallback image path are presentation/brand
policy, not facts about what an `Article` is.

### Why their *output* stays a plain `String`/`&str`, not a new type
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

```
effective_abstract(article: &Article) -> String
  // article.abstract_text (Abstract) present → its text, as-is
  // absent → truncate article.body (Body) to a presentation-chosen length

effective_image(article: &Article) -> &str
  // article.image (ImagePath) present → its path, as-is
  // absent → a predefined fallback asset path (asset choice: open question,
  //          see story's Open questions — not decided by this design step)
```

### Boundary location
Called from `src/pages/{article_page,listing_page,home_page}.rs` when
building each page's view — not from the domain, not from `ContentSource`.

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
