# CLAUDE.md — lucalorenzon-blog

## Purpose

A personal blog that lets the author showcase their professional experience
through articles on a range of topics.

## Why this file exists

The project was restarted from scratch on 2026-10-26. The previous attempt
followed the global development pipeline step by step and kept stalling in
ceremony instead of shipping. The rules below replace that pipeline for this
repository only. The previous site (code and documents) is kept on the local
branch `old_main`, for reference only.

## Global rules: what is reused, what is overridden

Reused unchanged from `~/.claude/CLAUDE.md`: global skills, permissions,
git workflow, response-quality rules, the Hindsight rules.

Overridden for this repository:

| Global rule | In this repository |
|---|---|
| Track B pipeline (epic → use case → story split → acceptance tests → story size → design chain) | Does not apply. See "How the product grows". |
| Epic check on thin lanes | Does not apply. Epics, use cases and stories are written only when the user asks, and only the ones the user asks for. |
| Cost Logging, and `story-size`'s calibration log in `~/gtheat-llm-sw-estimation-history` | Does not apply (personal project). Do not log, and do not probe for that repository. |

## How the product grows

- **Incremental, agile.** The site grows by small successive increments, each
  one usable on its own.
- **Elephant Carpaccio, one decision at a time.** Each slice isolates a single
  decision; the decision is discussed and agreed with the user before work on
  it starts. One slice = one commit.
- **Working skeleton first.** The first increment is the thinnest version of
  the site that goes live end to end (build → deploy → reachable URL).
- **Then challenge it.** Once the skeleton is live, `residuality` is used to
  stress the first version; what comes next is decided from that outcome.
- **The user directs.** Other skills are invoked by the user when needed. Do
  the agreed step, report the outcome, then wait.

## Model routing

| Kind of work | Model |
|---|---|
| Deterministic tasks: mechanical file operations, running builds, tests and git commands from exact instructions | `haiku` |
| Writing code | `sonnet` (Sonnet 5.5) |
| Planning and design: analysis, decisions, documentation | `opus` (Opus 5.5) |

The main session does the planning and design work and hands the other kinds
of work to sub-agents run on the matching model.

## Communication

- Documentation and file names: English.
- Chat: Italian, in plain full sentences. No acronyms or abbreviations used as
  shorthand, no broken or unfinished sentences.

## Session minutes

Every working session ends with a minute in `docs/sessions/`, named
`YYYY-MM-DD-<topic>.md`, containing: the session's goal, the decisions taken
and why, what was done (with commit references), open points, and the agreed
next step. The minute is committed as the session's last commit.

## Stack

- [Leptos](https://github.com/leptos-rs/leptos) 0.8.22, used only to render
  pages to static HTML at build time (no server, no WebAssembly yet)
- [Tailwind CSS](https://tailwindcss.com/) v4, standalone command-line tool
  pinned by version and checksum in `scripts/build-site.sh`
- Hosting on GitHub Pages, published by GitHub Actions
- [Playwright](https://playwright.dev/) end-to-end test, run before and after
  each publication
- Rust stable toolchain, pinned in `rust-toolchain.toml`

Why each choice was made: `docs/plan.md`, "Decisions taken".

## Repository map

| Path | Content |
|---|---|
| `src/` | The Leptos page component and the build program that writes `index.html` |
| `style/input.css` | Tailwind entry point |
| `scripts/build-site.sh` | The single build entry point, used locally and in the pipeline |
| `end2end/` | Playwright end-to-end test |
| `.github/workflows/deploy.yml` | Check, build, test, publish, then test the live site |
| `docs/plan.md` | Current plan (Now / Next / Later), decisions taken and open |
| `docs/sessions/` | One minute per working session |
| `docs/licences/third-party-usage.md` | Usage log for the CC BY-NC-SA `modularity` plugin, enabled in `.claude/settings.json` (required by the global licensing rule) |

## Memory

Hindsight bank: `personal-blog`. Emptied on 2026-10-26 together with the
restart; nothing in it predates the restart.
