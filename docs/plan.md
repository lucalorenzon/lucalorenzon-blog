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
| 2 | Project working rules, README, this plan, session minutes folder | Future sessions start from the new rules | in progress |
| 3 | Leptos + Tailwind build that generates a static `index.html` with "Site under construction" | The page opens locally from the build output | waiting for decisions 1 and 3 |
| 4 | GitHub Actions workflow publishing to GitHub Pages; Pages enabled on the repository | The site is live | waiting for decision 2 |
| 5 | Memory reset: local Claude Code memory updated, Hindsight bank emptied, ticket in `ll-dev-setup` for the missing `project-init` template | No outdated rule survives | in progress |

### Open decisions

Each one is discussed on its own, when its slice is reached.

1. **Leptos version.** 0.8.22 (latest stable, 2026-10-05) or 0.9.0-rc0
   (release candidate, 2026-10-15).
2. **Site address.** GitHub Pages sub-path
   (`lucalorenzon.github.io/lucalorenzon-blog/`) or a custom domain.
   A sub-path affects every link and asset path in the generated pages.
3. **How static HTML is generated.** A small build program that renders the
   page to `index.html` (no server, no WebAssembly until something
   interactive is needed), or the static generation built into
   `leptos_axum`.
4. **Old remote branches.** Whether to delete the branches left on GitHub
   from the previous site (`chore/ci-build-check` and the Dependabot
   branches). Deleting a remote branch cannot be undone.

## Next — challenge the skeleton

Run `residuality` against the live skeleton and decide the following
increments from its outcome.

## Later

Articles and everything around them. Not planned yet, on purpose.
