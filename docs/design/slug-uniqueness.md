# Domain concepts: slug-uniqueness check — EP-001-UC-001-S002

Scope: this document covers `SlugUniquenessError`, `ensure_slug_is_unique`, and `ContentSource::exists`'s contract. It is the `/parse-dont-validate` step of this story's design pipeline, following [docs/architecture/hexagonal.md](../architecture/hexagonal.md) (S002 section) and preceding `/sw-practices`.

## Dependency decisions

None new — reuses `thiserror`, already a dependency since S001. No new crate needed.

---

## `SlugUniquenessError`

### What it represents
Why a candidate slug failed the uniqueness check against the content source — either because it's already taken, or because the check itself could not be completed.

### Invariants
- `AlreadyExists` always carries the exact `Slug` that was found occupied (debuggability — same "carry the offending value" precedent as `ArticleError`'s field variants).
- `CheckFailed` always carries the underlying `FetchError` that made verification impossible — never a generic/string error.
- No third "unknown" variant: a call either proves the slug free, proves it taken, or fails to prove either.

### Input origin
`EXTERNAL` — wraps whatever `ContentSource::exists` returns (filesystem state via an I/O boundary), itself already typed as `Result<bool, FetchError>`.

### Illegal states eliminated
A caller that receives `Ok(())` from `ensure_slug_is_unique` never needs to re-check "but is it really free" — the raw `bool` from the adapter has already been consumed and translated into a domain-meaningful gate. A caller can no longer mistake "check failed for infrastructure reasons" for "slug is free" — the two are distinct `Err` variants, never silently coalesced into one boolean.

### Error type
```rust
#[derive(Debug, thiserror::Error)]
pub enum SlugUniquenessError {
    #[error("slug already in use: {slug}")]
    AlreadyExists { slug: Slug },
    #[error("could not verify slug uniqueness: {0}")]
    CheckFailed(FetchError),
}
```

### Boundary location
`ensure_slug_is_unique` — the only place this error is constructed.

---

## `ensure_slug_is_unique` — the parse boundary for this story

### What it represents
The domain service that turns `ContentSource::exists`'s raw `bool` into a domain-meaningful outcome: a proven "free" gate (`Ok(())`) or a typed reason it isn't (`Err(SlugUniquenessError)`). This function *is* the parse step for this story — there is no separate raw-input type to parse into a value object, because the raw input is the adapter's boolean answer, not user-supplied data.

### Signature
```rust
pub fn ensure_slug_is_unique(
    source: &impl ContentSource,
    candidate: &Slug,
) -> Result<(), SlugUniquenessError> {
    match source.exists(candidate) {
        Ok(false) => Ok(()),
        Ok(true) => Err(SlugUniquenessError::AlreadyExists { slug: candidate.clone() }),
        Err(err) => Err(SlugUniquenessError::CheckFailed(err)),
    }
}
```

Revises the `unreachable!()`-on-unexpected-variant shape floated during `/hexagonal-architecture`: since `exists` shares `FetchError` with `get_by_slug`/`list_published`, the type system cannot statically rule out `NotFound`/`Malformed`/`NotImplemented` reaching this match arm, even though a well-behaved adapter never produces them here. Panicking on an adapter-contract violation would be inconsistent with the rest of this codebase — no panics anywhere else, every failure is a typed `Result`. The catch-all `Err(err) => CheckFailed(err)` keeps that discipline: it degrades to "couldn't verify," never to a crash or to a silent "slug is free."

### Why no smart constructor / no `VerifiedSlug` wrapper type
Considered and rejected: wrapping a successfully-checked slug in a new type (e.g. `VerifiedUniqueSlug`) so the compiler would statically prevent code from skipping the check. Rejected because `Slug` remains freely constructible and is used throughout the domain for unrelated purposes (reading an existing article, generating a URL) — a wrapper here would not make "unverified slug" unrepresentable anywhere else in the codebase, only at this one call site. It would be speculative scaffolding for a consumer (S004's publish entrypoint) that doesn't exist yet. `Result<(), SlugUniquenessError>` used as a gate (`ensure_slug_is_unique(&source, &candidate)?`) is the idiomatic Rust shape for "check and propagate, no new data produced" — revisit only if S004 surfaces a real need to carry the proof forward.

### Input origin
`EXTERNAL` (via `ContentSource::exists`).

### Boundary location
Called from wherever S004 wires the publish/CI validation entrypoint — not wired anywhere yet, consistent with S001/S002 leaving the composition root untouched.

---

## `ContentSource::exists` — contract note

Not a new domain type, but its contract is worth fixing here since it is this story's only port extension:
- `Ok(true)` / `Ok(false)` answer *presence only* — unrelated to whether the file's content would parse as a valid `Article`.
- `Err(FetchError::Io(_))` is the only error variant a correct adapter implementation should produce; `NotFound`/`Malformed`/`NotImplemented` are structurally reachable (shared `FetchError`) but never actually returned by `exists` — enforced by adapter tests, not by the type system (test list fixed in `/sw-practices`).
