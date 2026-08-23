# ADR-002: Content as markdown-in-git via a ContentSource port, no CMS

- **Date:** 2026-08-23
- **Status:** Accepted (domain, `ContentSource` port, and both adapters implemented and tested — see Technical Notes; dedicated content repo created)
- **Stories:** EP-001-UC-001-S001 (domain construction, port shape); EP-001-UC-001-S002/S003 (filesystem adapter, listing)

## Context

The blog needs a place to keep its articles and a way for the site to read them. The two ordinary options for a small personal site are: use a content management system (a separate app with its own database and admin screens where articles are written and stored), or keep articles as plain text files versioned in the same kind of repository that already holds the site's code.

A CMS adds real weight for a one-author blog: a database to run and back up, an admin interface to secure, and a second system to keep updated alongside the site itself — none of which this project currently has any operational capacity for (the site itself is not yet deployed). The alternative — writing articles as markdown files and committing them to git, the same way code changes are already made — reuses infrastructure and a workflow (`git pull --rebase`, commit history, review via diff) already in place for everything else in this project, at the cost of a plainer authoring experience (no rich-text editor, no upload button for images).

## Decision

Articles are authored as markdown files with frontmatter, stored in a dedicated content git repository, and read by the site exclusively through a `ContentSource` interface (a hexagonal secondary port) — never by application code reaching into the filesystem or an external API directly. No CMS, database, or headless content API is adopted.

This keeps content acquisition swappable in principle (the port could later be backed by something other than the filesystem without touching the `Article` domain type or any code that consumes it) while committing, in practice, to the simplest adapter that fits a single-author blog today: reading `.md` files with `std::fs`. The port is deliberately designed once with its full intended shape — reading a single article by slug, and listing all published articles — even though the first story to implement it (S001) only wires up the single-article read; this avoids redesigning the interface when the listing capability is needed by a later story (S002/S003), at the cost of one method temporarily returning an explicit "not yet supported" error in the interim, rather than not existing at all.

Content lives in a repository separate from the site's own code repository: article changes and site-code changes are independent concerns with independent release cadence, and keeping them apart avoids coupling a markdown typo fix to a Rust release, or vice versa.

## Consequences

**Positive:**
- No new infrastructure to run, secure, or keep patched: content storage reuses git, already the project's existing tool for everything else.
- Every article change is reviewable and revertible the same way a code change is — a commit, a diff, `git revert` if a published article needs to be pulled back.
- The domain (`Article`, its value objects) has no dependency on how content is fetched — a future adapter (a different storage backend, a static export step) could replace `FilesystemContentSource` without changing `Article::new` or anything that consumes an already-constructed `Article`.
- Deploying the site does not require deploying or licensing a CMS product.

**Negative / Risks:**
- Authoring is textual only: writing an article means editing a markdown file directly (in an editor, not a web form), and there is no built-in media library — images referenced from an article must be placed and referenced by hand.
- No draft/preview workflow beyond what git itself offers (a branch, a local build) — there is no staging environment with a "preview before publish" button.
- The dedicated content repository does not exist yet (tracked as a separate chore prerequisite on the story, not part of this decision) — until it is created, `FilesystemContentSource` has nothing real to read from and only `InMemoryContentSource` (the test fake) is exercised.
- A single git repository has no per-article access control — anyone with write access to the content repo can edit or publish any article. Acceptable for a single-author blog; would need reconsideration if a second author were added.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Headless CMS (e.g. a hosted service with a web admin and an API) | Adds a third-party dependency, a subscription or hosting cost, and an admin surface to secure, for a single-author blog that does not need a rich editing UI or multi-user workflows. |
| Self-hosted CMS with a database | Adds a database to run, back up, and keep patched, plus its own deployment — operational weight disproportionate to the actual need (one author, markdown-comfortable). |
| Content directly inside the site's own code repository (no separate content repo) | Couples article publishing to the code release process — a typo fix in an article would sit in the same repo, same history, same CI as a Rust code change; keeping them separate lets content and code evolve on independent cadences. |
| No `ContentSource` port — read files directly wherever content is needed | Would leak filesystem I/O and markdown-parsing details into application/domain code, making it impossible to unit-test article-consuming logic without real files on disk, and impossible to later swap the storage backend without touching every call site. |
| Segregate `ContentSource` into two traits (`ArticleReader` / `ArticleLister`) instead of one | Considered and rejected during the S001 design pipeline (hexagonal-architecture step): list-fetch is one variation of the same fetch concept today, not an independent capability — splitting now would be accidental complexity not justified by an actual need. |

## Technical Notes

- Port: `ContentSource` (`src/domain/ports.rs`), with `get_by_slug(&self, slug: &Slug) -> Result<Article, FetchError>` and `list_published(&self) -> Result<Vec<Article>, FetchError>`. `FetchError` is one shared error type (`NotFound | Io | Malformed(ArticleError) | NotImplemented`) across both methods.
- Adapters: `FilesystemContentSource` (`src/adapters/secondary/content_source/filesystem.rs`, `#[cfg(feature = "ssr")]`) reads a `.md` file, parses its YAML frontmatter via `serde_yaml` into `RawFrontmatter`, and calls `Article::new` — any structural parsing failure (no frontmatter block, invalid YAML, wrong field types) falls back to an empty `RawFrontmatter` rather than a second error path, so `Article::new`'s own validation is the single source of `Malformed` errors; `InMemoryContentSource` (`src/adapters/secondary/content_source/fake.rs`, `#[cfg(test)]`) is a no-I/O test fake.
- Implemented (EP-001-UC-001-S001, complete as of 2026-08-23): domain (`Article`, value objects, `RawFrontmatter`, `ArticleError`), the `ContentSource` port, `FetchError`, and both adapters — 19 tests passing (14 domain/fake under any target, +5 `FilesystemContentSource` integration tests against real temp files under `--features ssr`).
- Dedicated content repository: created — [`lucalorenzon-blog-content`](https://github.com/lucalorenzon/lucalorenzon-blog-content) (private, per ADR-003), scaffolded with a README (frontmatter reference) and an example article. `FilesystemContentSource` has a real repository to read from once implemented; branch protection there is deferred (requires the repo to go public or GitHub Pro, per ADR-003's finding — see that ADR's Consequences).
- `S001` implements `get_by_slug` only; `list_published` returns `FetchError::NotImplemented` until `S002`/`S003` add the real logic on the same interface.

## References

- Stories: EP-001-UC-001-S001 ([story](../stories/EP-001-UC-001-S001-modellare-articolo-dominio-content-source.md), Escalation section — this ADR was recommended there); EP-001-UC-001-S002, EP-001-UC-001-S003 (listing implementation, not yet written)
- Related ADRs: ADR-001 (Leptos/toolchain baseline this content layer is built on)
- Design artefacts: [docs/architecture/hexagonal.md](../architecture/hexagonal.md) (port/adapter shape), [docs/design/article.md](../design/article.md) (domain type invariants)
- Epic: EP-001 (rilancio-blog-professionale) — this ADR was flagged as recommended but not yet created in the epic's ADR table
