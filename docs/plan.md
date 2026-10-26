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
| 3 | Leptos + Tailwind build that generates a static `index.html` with "Sito in costruzione" | The page opens locally from the build output | done |
| 4 | GitHub Actions workflow publishing to GitHub Pages, guarded by a browser test before and after publishing; Pages enabled on the repository | The site is live, and a test proves a visitor sees "Sito in costruzione" | done (live at https://lucalorenzon.github.io/lucalorenzon-blog/) |
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
5. **End-to-end browser test: before and after publishing.** One
   Playwright test (Chromium) opens the page, checks that its main heading
   reads "Sito in costruzione" and that the stylesheet loads. The same test
   runs twice in the pipeline: on the freshly built site served locally,
   so a broken page is never published; and on the public address right
   after publishing, because only that run proves what a visitor actually
   sees (sub-path, Pages settings, the deployment itself). The second run
   can only report a problem once the site is already published; that was
   accepted as the price of checking the live site.

### Open decisions

2. **Site address.** GitHub Pages sub-path
   (`lucalorenzon.github.io/lucalorenzon-blog/`) or a custom domain.
   Not blocking now: the single page refers to its stylesheet by a
   relative path, so it works the same under either address. It becomes
   a real decision once pages link to each other. Until then the site goes
   live on the standard GitHub Pages address.

Closed without a decision: **old remote branches.** By the time it came
up, GitHub held only `main`; the other branches of the previous site were
already gone from the remote, and no pull request was open.

## Next — challenge the skeleton

Run `residuality` against the live skeleton and decide the following
increments from its outcome.

Input already known for that review:
[run the browser tests inside the official Playwright image](backlog/run-browser-tests-in-playwright-image.md)
(deferred: an occasional slowdown of Ubuntu's package server delayed the
first publication by about nine minutes).

## Later

Articles and everything around them. Not planned yet, on purpose.
