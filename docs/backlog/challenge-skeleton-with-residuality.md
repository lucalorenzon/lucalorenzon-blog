# Backlog: challenge the live skeleton with `residuality`

- **Opened:** 2026-10-26
- **Status:** ready; starts when the user calls `residuality`.

## Why

The working rules (`CLAUDE.md`, "How the product grows") say that once
the first version is live, `residuality` stresses it, and the next
increments are decided from that outcome. The skeleton went live on
2026-10-26, so this is the planned next step: find out which choices made
for a one-page site would break first as the site changes (articles, more
pages, a domain, interactivity), before building on top of them.

## What is being challenged

The live skeleton as of 2026-10-26: one Leptos component rendered to
`index.html` by a small build program, Tailwind compiled by its
standalone tool, a GitHub Actions workflow that checks, builds, tests in
a browser, publishes to GitHub Pages and tests the live site again.

## Input already known

Things noticed while building the skeleton, to feed into the analysis
rather than to decide here:

- **Static generation.** The build program was chosen over `leptos_axum`'s
  built-in static generation because it is thinner for one page
  (`docs/plan.md`, decision 3). How it holds up with many pages and
  articles is an open question.
- **Site address.** Sub-path or custom domain is still open
  (`docs/backlog/choose-site-address.md`); a sub-path affects every link
  once pages link to each other.
- **Browser test setup.** The test depends on Ubuntu's package server at
  every run (`docs/backlog/run-browser-tests-in-playwright-image.md`).
- **Single platform build.** `scripts/build-site.sh` supports only Linux
  x64, the platform of both the author's machine and the pipeline.
- **Leptos dependencies.** `paste`, pulled in by Leptos, is reported as
  unmaintained (RUSTSEC-2024-0436); no safe upgrade exists at this layer.
- **Version alignment.** Leptos 0.9 is at release-candidate stage; GitHub
  is moving `ubuntu-latest` to Ubuntu 26.

## Done when

The analysis is written down, and the user has decided which residues to
act on; those become the next slices in `docs/plan.md`.
