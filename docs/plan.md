# Plan

Last updated: 2026-10-26

## Goal

Put a minimal version of the site live, then grow it by small increments,
one decision at a time.

## Now — working skeleton

Done when: a Leptos-built home page saying "Site under construction" is
reachable on its public URL, and the repository holds nothing of the
previous site.

| # | Slice | Visible result | Status |
|---|---|---|---|
| 1 | Remove the previous site from the repository | Only what is reused remains | done |
| 2 | Project working rules, README, this plan, session minutes folder | Future sessions start from the new rules | done |
| 3 | Leptos + Tailwind build that generates a static `index.html` with "Sito in costruzione" | The page opens locally from the build output | in progress |
| 4 | GitHub Actions workflow publishing to GitHub Pages; Pages enabled on the repository | The site is live | to do |
| 5 | Memory reset: local Claude Code memory updated, Hindsight bank emptied, ticket in `ll-dev-setup` for the missing `project-init` template | No outdated rule survives | done |

### Decisions taken

1. **Leptos version: 0.8.22**, the latest stable release (2026-10-05).
   0.9.0-rc0 was available but is a release candidate: it adds nothing a
   one-page skeleton needs, and moving to 0.9 once it is stable will be a
   small upgrade while the code is still minimal.
3. **Static HTML: a small build program.** A Rust program renders the
   Leptos page component to `index.html`; Tailwind is compiled by its
   official standalone command-line tool, downloaded at build time and
   checked against a pinned checksum. No server, no WebAssembly and no
   `cargo-leptos` for now: `cargo-leptos` is built around a server plus a
   WebAssembly front end and is added when the first interactive part
   arrives. Rejected: `leptos_axum`'s built-in static generation, the
   official path that scales to many pages, because it brings a web
   server, an async runtime and WebAssembly into the build to produce one
   page of text. Comparing the two again is a natural question for the
   `residuality` challenge once articles arrive.

### Open decisions

2. **Site address.** GitHub Pages sub-path
   (`lucalorenzon.github.io/lucalorenzon-blog/`) or a custom domain.
   Not blocking now: the single page refers to its stylesheet by a
   relative path, so it works the same under either address. It becomes
   a real decision once pages link to each other. Until then the site goes
   live on the standard GitHub Pages address.
4. **Old remote branches.** Whether to delete the branches left on GitHub
   from the previous site (`chore/ci-build-check` and the Dependabot
   branches). Deleting a remote branch cannot be undone.

## Next — challenge the skeleton

Run `residuality` against the live skeleton and decide the following
increments from its outcome.

## Later

Articles and everything around them. Not planned yet, on purpose.
