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

`docs/board/plan.md` is the project's board and the only file updated
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

## Cards

Cards follow the global rules, section "The Board": every card lives at
`docs/board/cards/<name>.md`, a path that never changes, and its state is
a symbolic link in `docs/board/backlog/`, `wip/` or `done/`, moved with
`~/.local/bin/card.sh`. A document refers to a card by its path in
`cards/`. A card's text follows the skill `card-management`, and a card
is cut as the global rules say, section "Planning": the part done now
stays in the card, the rest goes to new cards; a card is never deleted.
Why: `docs/board/cards/refer-to-cards-by-name-in-frozen-documents.md`.

What this repository adds:

- The backlog collects every idea or question noticed. A card may never
  be done and end up superseded by others; from time to time a cleanup
  card clears the backlog.
- Usually the card in work is the session's only one, but that is not a
  fixed rule.
- A card's title names its subject, with no "Backlog:" prefix: the state
  is told by the folder of its link.
- Each state folder holds a `.gitkeep`, so it exists, and links to it
  work, even when it has no cards.

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

Every working session ends with a minute in `docs/board/sessions/`,
written with the skill `session-minute`, which says what a minute holds
and how it is checked. What this repository adds:

- Reference: `docs/board/sessions/2026-10-27-run-browser-tests-in-playwright-image.md`.
- Earlier minutes keep their earlier format.
- Why this format: `docs/board/cards/revise-session-minute-format.md`.

## Commit attribution

The user is the only author of every commit. Work done with an agent ends
with an `Assisted-by:` line naming the model that did it (for example
`Assisted-by: Claude Haiku 5.5`), instead of `Co-Authored-By:`; a
sub-agent names its own model. The `Claude-Session:` line stays, to trace a
commit back to its session. Why: as in Fedora's policy on AI-assisted
contributions, the person stays the author and fully accountable, while
`Co-Authored-By:` would put the agent on the same level.

## The browser tests' README

`end2end/README.md` lists every browser test once: what it protects from,
its class, what makes it fail and why it exists, with a piece of its
declaring line to find it in the code.

- A commit that adds, removes, renames or changes a browser test updates
  its row in the same commit.
- The agent keeps the README in step and always reports a mismatch it
  notices, also outside the task at hand.
- `end2end/tests/readme.spec.ts` fails the build when a test has no row,
  or a row leads nowhere or to more than one place. It sees only the
  first column: whether "Fails when?" and "Why" are still true is the
  agent's to watch, so when a test's behaviour changes, its row's text
  is reviewed too.

Why: `docs/board/cards/describe-the-browser-tests-in-a-readme.md`.

## Stack

- [Leptos](https://github.com/leptos-rs/leptos) 0.8.23, used only to render
  pages to static HTML at build time (no server, no WebAssembly yet)
- [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) 0.13,
  converting the content's Markdown to HTML at build time; the Markdown
  holds no HTML (a unit test checks it)
- [serde-saphyr](https://crates.io/crates/serde-saphyr) 2.1, reading
  only, for the YAML front matter at the top of each article (why:
  `docs/board/cards/read-article-fields-from-front-matter.md`)
- [jiff](https://github.com/BurntSushi/jiff) 0.2, without time zones,
  reading the dates of the articles and of the sitemap (strict ISO
  form, `YYYY-MM-DD`) and writing them for the reader (why:
  `docs/board/cards/give-dates-a-module-of-their-own.md`)
- [unicode-segmentation](https://github.com/unicode-rs/unicode-segmentation)
  1.13, counting the graphemes of a page heading against its limit
  (already compiled as a dependency of Leptos; why:
  `docs/board/cards/enforce-the-title-length-limit-in-the-build.md`)
- [icondata](https://github.com/Carlosted/icondata) 0.7, the drawings of
  the site's icons, with Bootstrap Icons' set only; the build turns each
  into a CSS mask (`src/icons.rs`), so no drawing is written by hand in
  the stylesheet (why:
  `docs/board/cards/turn-the-code-wrap-toggle-into-an-icon.md`)
- [Tailwind CSS](https://tailwindcss.com/) v4, standalone command-line tool
  pinned by version and checksum in `scripts/get-tailwind.sh`
- Hosting on GitHub Pages, published by GitHub Actions only when a version
  tag `vX.Y.Z` is pushed (how: `README.md`, "Publish a version")
- [Playwright](https://playwright.dev/) end-to-end test, run on every build
  and again on the live site after each publication
- [Lighthouse](https://github.com/GoogleChrome/lighthouse) 13.6.0, driven
  from a Playwright project of its own: the search-engine rules on every
  page of the sitemap, in both moments (why:
  `docs/board/cards/check-seo-with-lighthouse.md`)
- Rust stable toolchain, pinned in `rust-toolchain.toml`

Why each choice was made: `docs/board/cards/put-working-skeleton-live.md`.

## Repository map

| Path | Content |
|---|---|
| `src/` | The page frame shared by every page (`page_frame.rs`), the text column every page writes its text in (`one_column_text.rs`), the home page, the curriculum page (`curriculum_page.rs`), the about page, the licence page, the article pages (`article_page.rs`) and the "page not found", the site's address (`site_address.rs`), the Markdown-to-HTML conversion with the code block component and the table's two forms, table and record viewer (`markdown.rs`), the widths that decide when a table becomes a record viewer (`table_widths.rs`, from Literata's character widths in `literata_widths.rs`, generated), the list of pages with the sitemap (`sitemap.rs`), the `robots.txt` (`robots_txt.rs`), the breakpoints, read from `config/` and written for the stylesheet, also giving the Rust class checks the prefixes they accept (`style_guide/breakpoints.rs`), the style guide's colours, each role with its dark and light value, the pairs allowed together and their contrast check (`style_guide/colours.rs`), the site's fonts, read from `config/` and written for the stylesheet with the fonts page's sample variants (`style_guide/fonts.rs`), the style guide's pages, its index, the colours page with its samples, the code blocks page, the tables page, the text page and the fonts page, composed by the build from the fonts' configuration and the placeholders of its text, with its samples (`style_guide/index_page.rs`, `style_guide/colours_page.rs`, `style_guide/code_page.rs`, `style_guide/tables_page.rs`, `style_guide/text_page.rs`, `style_guide/fonts_page.rs`), the icons' drawings, read from `icondata` and written as CSS masks (`icons.rs`), and the build program that writes `index.html`, `curriculum.html`, `about.html`, `licence.html`, `404.html`, the articles in `articles/`, the style guide in `style-guide/`, `sitemap.xml`, `robots.txt` and, outside the site, five files for Tailwind: the list of generated classes (`target/generated/code-classes.txt`), the colours (`target/generated/colours.css`), written only if every allowed pair reaches its contrast, the breakpoints' `@theme` (`target/generated/breakpoints.css`), the fonts' `@font-face` rules, their three roles and the sample variants of the fonts page, with the `@source inline` that lists them (`target/generated/fonts.css`), and the icons' masks (`target/generated/icons.css`) |
| `content/` | The site's text in Markdown: today the curriculum, a version written for the site and cleaned of personal and employer data, the text of the about and licence pages (`about.md`, `licence.md`), the text of the style guide's pages in `style-guide/` (written by the author, except one section of `colours.md` still a placeholder; `fonts.md` is cut in pieces by headings `## <font folder> | <role>`, which the build places in the sections it writes; `code.md`, `tables.md` and `text.md` also hold the cases the browser tests of the code block, of the table and of the text run on), and the articles in `articles/`, each with its title, description and date in a YAML front matter; its licences, different from the code's, in `README.md` and `LICENSE-MIT-0` |
| `config/` | Settings of the build and the browser tests: `breakpoints.txt`, the widths in pixels at which the layout changes, one line `name=width` (`md`, `xl`), read by the build, by the Rust class checks and by the browser tests, which take from it the two sides of each breakpoint; `heading-max-graphemes.txt`, the most graphemes a page heading may have, read by the frame (`src/page_frame.rs`) and by the header's browser test; `fonts.txt`, which font plays each role (text, code, logo), and `fonts/<folder>.txt`, each font's description (family, fallback, `font-display`, an optional native `size` in pixels, its files with label, style and weight), read by the build and by the fonts page's browser test |
| `assets/` | Files copied as they are into the site: the favicon, the fonts, one folder each, with their licences and attribution, and the licence and attribution of the icons' set (`icons/`) |
| `style/input.css` | Tailwind entry point; the colours, the fonts and the icons' masks are imported from the files the build writes, so it names no font |
| `scripts/build-site.sh` | The single build entry point, used locally and in the pipeline |
| `scripts/get-tailwind.sh` | Gets the pinned Tailwind CSS tool (version and checksum live here): checks a tool already in `target/tools/`, downloads with retries otherwise, prints its path; `--cache-info` prints the pipeline's cache key and path; `--version` prints only the pinned version; used by `scripts/build-site.sh` and by the pipeline's "Get Tailwind" step |
| `scripts/playwright-version.sh` | Prints the Playwright version locked in `end2end/`, which picks the test container image in both workflows |
| `scripts/screen-reader/` | The on-demand screen reader check: Orca 51 and Firefox ESR in a Docker container (`Dockerfile`, `entrypoint.sh`), the host program `check.py`, its unit tests and a log sample (`test_check.py`, `fixtures/`) |
| `scripts/content-dates/` | The content dates step: `content_dates.py` writes the author date of the last commit of each Markdown file in `content/` (tab-separated file in `target/generated/content-dates.tsv`, read by the build through `CONTENT_DATES_FILE`), and its unit tests (`test_content_dates.py`); in the pipeline it runs in the `gather-inputs` job, on the full history |
| `scripts/robots-list/` | The robots list step: `robots_list.py` reads the community list of AI robots from the copy committed at a pinned commit in `ai-robots-txt/` (`robots.json` and its MIT `LICENSE`; checksum checked, no download), filters it and writes the names of the robots to block (`target/generated/blocked-robots.txt`, read through `BLOCKED_ROBOTS_FILE`); with `--check-updates` (the only mode that uses the network) it shows what would change if the pin moved (`--json` prints it as one JSON object); its unit tests (`test_robots_list.py`); in the pipeline it runs in the `gather-inputs` job |
| `scripts/version-check/` | The check for newer versions of what is pinned by hand: `version_check.py` asks GitHub for Tailwind's latest release and runs `robots_list.py --check-updates --json`, and turns a newer Tailwind, or a robots list that would change the blocked robots, into findings (title, Markdown body, hidden marker); by default it prints what it would do about each one (open an issue, update the open one, or nothing because an issue already covers it), and with `--apply` it does it, through GitHub's API, with the label `version-check`; its unit tests (`test_version_check.py`) |
| `end2end/` | Playwright end-to-end test, every test listed in `end2end/README.md` (see "The browser tests' README"); in `end2end/tools/`, the script that measures Literata's character widths in Chromium and writes `src/literata_widths.rs`, the on-demand check of axe's best practices on every built page (`check-axe-best-practices.mjs`), and the on-demand page that measures how wide a browser draws the code font, for a browser the agent cannot reach (`make-code-font-page.mjs`, writes `target/code-font-page.html`) |
| `.zed/settings.json` | Editor settings for this project: CSS files are checked by Tailwind's CSS server, not the generic one, which does not know Tailwind 4's syntax |
| `.github/workflows/build.yml` | Check, build and test the site on every push and pull request; on `main`, keep the tested site in the `snapshot` Release; the pinned Tailwind tool comes from the Actions cache or is downloaded, through the local action below; never publishes |
| `.github/actions/get-tailwind/action.yml` | Local action: restore the pinned Tailwind from the Actions cache, or download it and save it in the cache; used by `build.yml` and `daily.yml` |
| `.github/workflows/daily.yml` | Scheduled every day (and by hand): restore the Tailwind cache entry so it does not expire after 7 days without a build, and recreate it if it is gone; and run the check for newer versions of the pinned tools with `--apply`, which opens or updates issues (the only job with `issues: write`) |
| `.github/workflows/deploy.yml` | On a version tag: check the tag, take the snapshot of that commit or rebuild it, publish, create the version's Release, test the live site |
| `CHANGELOG.md` | What each published version brought, and the rules for version numbers and tags |
| `docs/board/plan.md` | The project's board: Now / Next / Later (see "The plan") |
| `docs/board/cards/` | Every card, whatever its state (see "Cards") |
| `docs/board/backlog/`, `wip/`, `done/` | One symbolic link per card, showing its state: noticed and not started, being worked on, finished (in `done/` named after the day the work finished) |
| `docs/board/sessions/` | One minute per working session |
| `docs/accessibility/` | Reference transcript and record of every screen reader check |
| `docs/licences/third-party-usage.md` | Usage log for the CC BY-NC-SA `modularity` plugin, enabled in `.claude/settings.json` (required by the global licensing rule) |

## Memory

Hindsight bank: `personal-blog`. Emptied on 2026-10-26 together with the
restart; nothing in it predates the restart.
