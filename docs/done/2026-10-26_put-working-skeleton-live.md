# Put a working skeleton of the site live

- **Opened and finished:** 2026-10-26.
- **Status:** done on 2026-10-26. Written afterwards, on 2026-10-27, when
  the decisions moved out of `docs/plan.md` (card
  [define the rules for planning and project management](../wip/define-planning-rules.md)).
  It never went through `docs/backlog/` and `docs/wip/`: the card folders
  did not exist yet.

## Why it was opened

The project restarted from scratch on 2026-10-26, after the previous
attempt kept stalling in ceremony
([minute of that session](../sessions/2026-10-26-restart-from-scratch.md)).
The working rules ask for a working skeleton first: the thinnest version
of the site that goes live end to end, from build to a reachable address.

Done when: a Leptos-built home page saying "Sito in costruzione" is
reachable on its public address, and the repository holds nothing of the
previous site.

## Answers

### What does the skeleton contain?

Five slices, all done on 2026-10-26:

| # | Slice | Visible result |
|---|---|---|
| 1 | Remove the previous site from the repository | Only what is reused remains |
| 2 | Project working rules, README, plan, session minutes folder | Future sessions start from the new rules |
| 3 | Leptos and Tailwind build that generates a static `index.html` with "Sito in costruzione" | The page opens locally from the build output |
| 4 | GitHub Actions workflow publishing to GitHub Pages, guarded by a browser test before and after publishing; Pages enabled on the repository | The site is live, and a test proves a visitor sees "Sito in costruzione" |
| 5 | Memory reset: local Claude Code memory updated, Hindsight bank emptied, ticket in `ll-dev-setup` for the missing `project-init` template | No outdated rule survives |

### Which Leptos version?

**0.8.22**, the latest stable release (2026-10-05). Rejected: 0.9.0-rc0,
a release candidate. It adds nothing a one-page skeleton needs, and
moving to 0.9 once it is stable will be a small upgrade while the code is
still minimal.

### How is the static HTML produced?

**A small build program.** A Rust program renders the Leptos page
component to `index.html`. Tailwind is compiled by its official
standalone command-line tool, downloaded at build time and checked
against a pinned checksum. No server, no WebAssembly and no
`cargo-leptos` for now: `cargo-leptos` is built around a server plus a
WebAssembly front end, and is added when the first interactive part
arrives.

Rejected: `leptos_axum`'s built-in static generation, the official path
that scales to many pages. It brings a web server, an async runtime and
WebAssembly into the build to produce one page of text. Comparing the two
again is a natural question for the `residuality` challenge once articles
arrive.

### How is the published site tested?

**One browser test, run before and after publishing.** A Playwright test
(Chromium) opens the page and checks that its main heading reads "Sito in
costruzione" and that the stylesheet loads. The same test runs twice in
the pipeline:

- on the freshly built site served locally, so a broken page is never
  published;
- on the public address right after publishing, because only that run
  proves what a visitor actually sees (sub-path, Pages settings, the
  deployment itself).

The second run can report a problem only once the site is already
published; that was accepted as the price of checking the live site.
Since 2026-10-27 both runs use Playwright's official container image
([card](2026-10-27_run-browser-tests-in-playwright-image.md)).

### Which public address?

**Not decided yet.** The site goes live on the standard GitHub Pages
address, under a sub-path. A single page that refers to its stylesheet by
a relative path works the same under a custom domain, so the choice is not
blocking until pages link to each other. It is tracked in the card
[choose the site's public address](../backlog/choose-site-address.md).

### What about the old remote branches?

**Closed without a decision.** By the time it came up, GitHub held only
`main`: the other branches of the previous site were already gone from
the remote, and no pull request was open.

## Results

- Site live at https://lucalorenzon.github.io/lucalorenzon-blog/, first
  run
  [40409740212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40409740212)
  green.
- The first test on the live site took 9 min 40 s, because Ubuntu's
  package server was slow; this led to the
  [Playwright image card](2026-10-27_run-browser-tests-in-playwright-image.md).
