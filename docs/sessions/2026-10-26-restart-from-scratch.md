# Session 2026-10-26 — Restart from scratch

## Goal

Restart the project from an empty base and reach two outcomes:

1. a basic Leptos site, with only a home page saying "Sito in costruzione",
   live on GitHub Pages;
2. nothing left in the repository from the previous site.

Both were reached.

## Why the restart

The previous attempt applied the global development pipeline (epic, use
case, story split, acceptance tests, sizing, design chain) step by step
and kept stalling in ceremony instead of shipping. The user decided to
keep the global rules, skills, permissions and model routing, and drop
only the rigid pipeline. The previous site is kept on the local branch
`old_main`.

## Decisions taken

| # | Decision | Why |
|---|---|---|
| 1 | New working rules in `CLAUDE.md`: incremental growth, one decision per slice, working skeleton first, then `residuality`; epics, use cases and stories only on request; other skills called by the user | Replace the pipeline that stalled with a lighter flow led by the user |
| 2 | Model routing: Haiku for deterministic tasks, Sonnet for code, Opus for planning and design | User's rule, to match cost to the kind of work |
| 3 | Documentation in English, chat in Italian with full sentences and no acronyms; one minute per session in `docs/sessions/` | User's rule |
| 4 | Hindsight bank `personal-blog` deleted and recreated empty, with a new mission and no directives or mental models | Its content described the old plan and would steer future sessions the wrong way |
| 5 | Leptos 0.8.22 (stable), not 0.9.0-rc0 | A release candidate adds nothing to a one-page skeleton |
| 6 | Static HTML from a small build program; Tailwind standalone tool pinned by checksum; no server, no WebAssembly, no `cargo-leptos` yet | Thinnest working skeleton; `leptos_axum`'s static generation would bring a server stack for one page |
| 7 | Site address: standard GitHub Pages sub-path for now; custom domain postponed | With relative paths a single page works under either address |
| 8 | One Playwright test run before publishing (blocks a broken page) and after publishing (proves the live site) | The user asked for a test of the loaded site; only the second run proves what a visitor sees |
| 9 | Ruleset "Require CI on main" now requires the `build` check instead of the removed "Lint, test, build" | The old check no longer existed: every push bypassed it and Dependabot pull requests would have waited forever |
| 10 | Running the tests in the official Playwright image: deferred to the backlog | Slowdown seen once; the fix adds a version to keep aligned by hand |

Full reasoning for 5–8 and 10: `docs/plan.md` and
`docs/backlog/run-browser-tests-in-playwright-image.md`.

## What was done

Commits in this repository:

- `ed88021` — remove the previous site (code, documents, Docker, old
  pipeline, old end-to-end tests)
- `98ef114` — working rules, README, plan
- `776e263` — Leptos page and build program, Tailwind, unit tests
- `ce331cd` — GitHub Actions workflow: checks, build, browser test,
  publication, browser test on the live site
- `d382690` — backlog ticket on the Playwright image
- this minute

Outside the repository:

- GitHub Pages enabled with source "GitHub Actions"; first run
  [40409740212](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/40409740212)
  green, site live at https://lucalorenzon.github.io/lucalorenzon-blog/
- Ruleset "Require CI on main" updated (decision 9)
- Hindsight bank `personal-blog` reset (decision 4)
- Local Claude Code memory: six entries tied to the old pipeline removed
- `ll-dev-setup`: backlog ticket `docs/backlog/project-init-missing-claude-md-template.md`
  (commit `87cd1d8`): `project-init` needs a `CLAUDE.md` template that
  does not exist, and its shell injections were not expanded

## Things noticed

- Deleting a Hindsight bank removes its content but not its mission: the
  recreated bank came back with the old mission, which had to be replaced
  separately. Written, with the user's confirmation, to the `llm-rules`
  bank as a reliability learning (document
  `hindsight-delete-bank-keeps-mission`, dated to the event).
- The first live-site test took 9 min 40 s because Ubuntu's package
  server was slow (see backlog ticket).
- GitHub is moving `ubuntu-latest` to Ubuntu 26; the first run already ran
  there without problems.

## Open points

Both tracked as backlog cards:

- [Choose the site's public address](../backlog/choose-site-address.md):
  open, not blocking until pages link to each other.
- [Challenge the live skeleton with `residuality`](../backlog/challenge-skeleton-with-residuality.md):
  when the user calls it.
- [Run build, deploy and tests only when the code changes](../backlog/run-pipeline-only-on-code-changes.md):
  asked by the user after documentation-only commits triggered full
  publications; options open.

## Next step

The Chromium slowdown recurred in a later run (two slow runs out of
three, see the card's "Measurements"), so the user decided on 2026-10-27
that the next session starts with
[running the browser tests inside the official Playwright image](../backlog/run-browser-tests-in-playwright-image.md).
