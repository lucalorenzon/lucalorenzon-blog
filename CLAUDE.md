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

## The plan

`docs/plan.md` is the project's board and the only file updated
continuously. It maps the road toward the project's outcome in three
horizons, kept short on purpose:

- **Now:** the card being worked on, plus the few cards that are well
  defined and clearly come first.
- **Next:** what is expected to come right after.
- **Later:** what is known or wished for, but not for now or right after.

The plan is not an order of execution. Software development is learning, so
the plan is reworked by successive approximations; its value is in making
it. It changes whenever priorities change, also during a session, when a new
card turns out more urgent than what was in Now. It holds no decisions: why
something was decided is in its card, who decided it is in the session
minute. A decision that belongs to no card, such as a technical choice made
during a session or one that spans several cards, is recorded with the `adr`
skill in `docs/adr/` (the folder is created with the first record). The user, acting as Product Owner, sets the priorities, possibly
discussing them with the agent. What matters most is not doing too many
things at once.

When a card needs many activities or decisions, the user and the agent
discuss it together:

- what can be done right away and gives value to the project at once;
- which activities make it clearer where to go, and increase learning;
- whether it can be cut vertically, for example by limiting the kind of data
  handled, so the scope is narrow and the cognitive load small.

Once a minimal slice is found and judged worth doing, its card is created in
the backlog (two or three of the same size may be created together) and work
starts. Planning stops there and resumes later: validating the plan's
assumptions with code matters more than settling every detail in advance.

## Backlog cards

A card is a file that moves through three folders, so the folder alone tells
its state:

| Folder | State | File name |
|---|---|---|
| `docs/backlog/` | Noticed, not started | `<name>.md` |
| `docs/wip/` | Being worked on now | `<name>.md` |
| `docs/done/` | Finished | `yyyy-MM-dd_<name>.md`, the date being the day the work finished |

- `docs/backlog/` collects every idea or question noticed. A card may never
  be done and end up superseded by others; from time to time a cleanup card
  clears the backlog.
- The card being worked on goes to `docs/wip/`. Usually it is the session's
  only card, but that is not a fixed rule.
- A card split into smaller ones does not go to `docs/done/`: the new cards
  go to the backlog and the original file is deleted in the same commit. The
  session minute records the split with that commit, and the git history
  keeps the original text. The plan is updated to match.
- A card's title names its subject, with no "Backlog:" prefix: the folder
  tells the state.
- A finished card reads as the answer to the questions it raised: title and
  status with the closing date; why it was opened; the answers, each with
  what was chosen and why, and the rejected options with their reason; the
  results observed, if any. Proposal-time sections ("Proposal", "When to
  pick this up") are removed.

Cards move with `git mv`, so their history follows them. When a card moves,
links to it from other documents are updated in the same commit. The date
prefix keeps `docs/done/` in the order the work was finished. Each of the
three folders holds a `.gitkeep`, so it exists, and links to it work, even
when it has no cards.

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
`YYYY-MM-DD-<topic>.md`, committed as the session's last commit. The minute
records how the session went and who contributed what; once committed it is
not edited. What a card already says is linked, not repeated. Sections, in
this order:

1. **Participants and goal:** the user, the agent and its sub-agents with
   their models; the goal, with a link to the card.
2. **Decisions:** a numbered table with the MADR roles: decision-makers (who
   decided), consulted (who gave input), informed (who was told afterwards).
3. **Discussion:** one short point per topic, saying who raised it, who
   proposed what and how it ended, with a reference to the decisions it led
   to. Only what shaped the decisions; technical details stay in the card.
4. **Outcome:** what changed, with commit references.
5. **Open points**, if any: questions left unresolved.
6. **Next steps:** the actions that came out of the session, each with a
   link to the card that carries it. When they are done is decided by the
   plan, not by the minute.

Reference: `docs/sessions/2026-10-27-run-browser-tests-in-playwright-image.md`.
Earlier minutes keep their earlier format. Why this format:
`docs/done/2026-10-27_revise-session-minute-format.md`.

## Commit attribution

The user is the only author of every commit. Work done with an agent ends
with an `Assisted-by:` line naming the model that did it (for example
`Assisted-by: Claude Haiku 5.5`), instead of `Co-Authored-By:`; a
sub-agent names its own model. The `Claude-Session:` line stays, to trace a
commit back to its session. Why: as in Fedora's policy on AI-assisted
contributions, the person stays the author and fully accountable, while
`Co-Authored-By:` would put the agent on the same level.

## Stack

- [Leptos](https://github.com/leptos-rs/leptos) 0.8.22, used only to render
  pages to static HTML at build time (no server, no WebAssembly yet)
- [Tailwind CSS](https://tailwindcss.com/) v4, standalone command-line tool
  pinned by version and checksum in `scripts/build-site.sh`
- Hosting on GitHub Pages, published by GitHub Actions
- [Playwright](https://playwright.dev/) end-to-end test, run before and after
  each publication
- Rust stable toolchain, pinned in `rust-toolchain.toml`

Why each choice was made: `docs/done/2026-10-26_put-working-skeleton-live.md`.

## Repository map

| Path | Content |
|---|---|
| `src/` | The Leptos page component and the build program that writes `index.html` |
| `style/input.css` | Tailwind entry point |
| `scripts/build-site.sh` | The single build entry point, used locally and in the pipeline |
| `end2end/` | Playwright end-to-end test |
| `.github/workflows/deploy.yml` | Check, build, test, publish, then test the live site |
| `docs/plan.md` | The project's board: Now / Next / Later (see "The plan") |
| `docs/backlog/` | Tickets: things noticed but deliberately not done yet, with the reason |
| `docs/wip/` | Backlog cards being worked on now (see "Backlog cards") |
| `docs/done/` | Finished backlog cards, prefixed with the date they were finished |
| `docs/sessions/` | One minute per working session |
| `docs/licences/third-party-usage.md` | Usage log for the CC BY-NC-SA `modularity` plugin, enabled in `.claude/settings.json` (required by the global licensing rule) |

## Memory

Hindsight bank: `personal-blog`. Emptied on 2026-10-26 together with the
restart; nothing in it predates the restart.
