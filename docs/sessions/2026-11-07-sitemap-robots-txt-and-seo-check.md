# Session 2026-11-07 — The SEO check put on hold, the sitemap and the robots.txt

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agents Sonnet 5.5 (the code of the dates step, the sitemap, the
test of the fixed pages' sources, the robots list step and the
`robots.txt`, with their tests) and Haiku 5.5 (one comment corrected).
The session ran from 2026-11-06 to 2026-11-07.
**Goal:** take the card
[check SEO with Lighthouse in the build](../wip/check-seo-with-lighthouse.md)
and build it. Five of its questions were answered, then the card was put
on hold, still in `docs/wip/`, for two cards it turned out to need:
[add a sitemap](../done/2026-11-07_add-sitemap.md) and
[add a robots.txt](../done/2026-11-07_add-robots-txt.md), both built and
closed. Reasons and rejected options are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Take the SEO check card; run Lighthouse once outside the repository before answering its questions | user | agent | — |
| 2 | The SEO check runs on the local build and on the live site | user | agent | — |
| 3 | Lighthouse driven from the Playwright tests, the `lighthouse` package alone, version 13.6.0 | user | agent | — |
| 4 | The threshold is rule by rule, every rule of the category; "not applicable" counts as passed, "manual" is ignored, each with a card of its own | user | agent | — |
| 5 | A page with `noindex` skips the rules on indexing and description; a `robots.txt` does not replace it | user | agent | — |
| 6 | The check takes its pages from a sitemap; the card is put on hold, stays in `docs/wip/`; the next version waits for all of it | user | agent | — |
| 7 | The sitemap and the `robots.txt` are two cards | user | agent | — |
| 8 | Each sitemap entry has the date of the page's last significant change, from git; a change of appearance does not count | user | agent | — |
| 9 | First increment: the date of the page's Markdown file; titles, descriptions and dates move into a front matter, in a card of its own | user | agent | — |
| 10 | The home page has no date in the sitemap; its date, from articles added or removed, is a card of its own | user | agent | — |
| 11 | The sitemap follows the Markdown files under `content/`, with an exclusion list (`README.md`, `LICENSE-MIT-0`); revised from "the pages in the built site's folder" | user | agent | — |
| 12 | The dates come from a job of their own, on the full git history without the files' contents, handed to the build as a file | user | agent | — |
| 13 | The author's date of the commit; the publication date is a card of its own | user | agent | — |
| 14 | A missing date stops the build in the pipeline and is a warning locally | user | agent | — |
| 15 | The sitemap is checked by Rust tests, and in a browser by the SEO check | user | agent | — |
| 16 | The dates step is written in Python; porting the Python scripts to Rust, run with Docker, is a card of its own | user | agent | — |
| 17 | The build writes every page from one list in `src/sitemap.rs`, so a page cannot be written and forgotten in the sitemap | agent | Sonnet | user |
| 18 | A test checks that each fixed page's declared Markdown file is the one its text comes from | user | agent | Sonnet |
| 19 | The `robots.txt` points to the sitemap and blocks only the robots that collect text for training; reading on request and search pass; `Google-Extended` is blocked | user | agent | — |
| 20 | The training robots come from the community list `ai-robots-txt/ai.robots.txt`, filtered: only what is declared useful passes, the agents included; eight exceptions | user | agent | — |
| 21 | The shared signal `__usage_ai-training` is added beside the list | user | agent | — |
| 22 | Asking the robots of artificial intelligence for a citation joins the card on structured data | user | agent | — |
| 23 | The list is downloaded at a pinned commit with a checksum; a local command shows how far behind the pin is and what would change in the `robots.txt` | user | agent | — |
| 24 | The robots list step is in Python, beside the dates step; the pipeline job becomes `gather-inputs` | user | agent | Sonnet |
| 25 | The changelog gets the unpublished entries now, not at publication | user | agent | — |
| 26 | The session ends here; the SEO check is resumed in a new session | user | agent | — |

## Discussion

1. **The trial run.** The agent proposed running Lighthouse once before
   answering the card's questions. Every page scored 100 but the "page
   not found"; the canonical link passed although it points to the
   production domain; a missing `robots.txt` was "not applicable". The
   user then answered where the check runs, with which tool and
   package, and with which threshold, one question at a time.
   → decisions 1 to 4
2. **The "page not found".** The agent set out how to exempt it; the
   user asked whether a `robots.txt` would solve it. The agent checked
   Lighthouse's code: a page disallowed there counts as blocked too,
   and the robots would no longer see its `noindex`. The user then
   asked whether Lighthouse could skip pages, and chose to derive the
   exemption from `noindex`. → decision 5
3. **Which pages.** The agent raised that the live site has no folder
   to walk and that the "page not found" answers 404 there. The user
   proposed putting the card on hold and taking the pages from a
   sitemap, which the site lacked, as it lacked a `robots.txt`; the
   agent named the tensions with the plan one at a time. → decisions 6,
   7
4. **The sitemap's date.** The agent cited Google's rule that only a
   significant change counts. The user proposed the latest of content
   and rendering code; the agent showed that both live in the same
   files. The user answered that pure content is in the Markdown, set
   the front matter as the target, and took the Markdown's date as a
   first increment. → decisions 8, 9
5. **Revising two answers.** Asked how a page is linked to its
   Markdown, the user set out the model aimed at: the build goes
   through `content/` and reads each file's front matter. The agent
   named the two answers it changed; the user confirmed, moved the home
   page's date to a card of its own, and set the exclusion list.
   → decisions 10, 11
6. **Git in the pipeline.** The user proposed the last two commits, then
   a clone with `--depth=100`; the agent measured both on the
   repository, and both gave wrong dates. The user then proposed a step
   of its own on the history without contents, and asked whether it
   holds with content from other repositories; the agent answered that
   it holds better than one build job. → decision 12
7. **The rest of the sitemap.** The agent raised that neither git date
   is the publication date; the user took the author's date and opened
   a card. The user chose strict and tolerant behaviour, Rust tests,
   Python, and asked for the porting card. → decisions 13 to 16
8. **Building the sitemap.** Sonnet built one list of pages from which
   the build writes everything; the agent accepted it and raised the
   gap for the fixed pages, and the user asked for the test. The user
   reported an error and a warning in the editor; the agent reproduced
   them with Zed's ruff and basedpyright and fixed them before the
   commit. The user asked whether the sitemap should carry the
   language; the agent answered from Google's documentation that it
   matters only with several language versions. → decisions 17, 18
9. **What the `robots.txt` says.** The agent raised the robots of
   artificial intelligence and the content's licences. The user asked
   whether a block on `ClaudeBot` would stop the agent; the agent read
   Anthropic's documentation on its three robots and answered no. The
   user blocked training only; the agent raised `Google-Extended`,
   which mixes training and grounding, and the user blocked it.
   → decision 19
10. **The list of robots.** The agent set out three ways to keep the
    list; the user chose the community list, filtered. The agent showed
    its categories; the user chose what passes and asked how to ask the
    robots for a citation. While answering, the agent found the shared
    signal `__usage_ai-training`, proposed on 2026-10-30, and read
    Google's announcement and the draft; the user added it. The agent
    proposed eight exceptions, which the user confirmed. → decisions 20
    to 22
11. **Keeping the list up to date.** The user chose a pin with a
    checksum and asked for a local command showing how far behind it
    is; asked what it shows, the user chose the changes to the
    `robots.txt`. The agent raised that the filter must then live in one
    place, and the user chose Python; the agent proposed renaming the
    pipeline job. → decisions 23, 24
12. **Counts corrected.** The agent miscounted the categorised robots
    (147 instead of 122) and gave the blocked ones before the exceptions
    (131 instead of 122); the first was found by the agent, the second
    by Sonnet, and both were corrected in the card.
13. **Closing.** Before the minute the agent offered to correct a stale
    comment in the code; the user asked for it and for the missing
    changelog entries, first left to the publication. → decisions 25,
    26

## Outcome

- The SEO check card: `99c83d4` to `wip`, `d1c9d31` to `36d2cdc` five
  answers and the trial run, `0c881f4` two cards for Later, `37ecdd0`
  on hold with the two new cards, `9ddf7d2` two links fixed.
- The sitemap: `e86e362` card to `wip`, `85ea78b` to `495dfc2` answers,
  `83d51ad`, `d4b4975` and `4d4c8c2` three new cards, `07b18e4` a note in
  the content sources card, `e83efdf` the porting card; code `c9b6819`
  (dates step), `49c4870` (sitemap), `d90530d` (fixed pages' test);
  `3383f83` card closed.
- The `robots.txt`: `16b791c` card to `wip`, `6835553` to `67c25b0`
  answers, `90b2eb3` and `7e0b444` counts corrected, `b561c12` citation
  in the structured data card; code `4291276` (robots list step),
  `cc900f2` (`robots.txt`); `6061b1f` card closed.
- `23d9808` the stale comment, `36ad8bf` the changelog.
- Checks: 199 Rust tests, 11 and 24 Python tests, 848 browser tests
  passed locally; the pipeline passed on `d90530d` and on `cc900f2`,
  and the snapshot of `cc900f2` holds the sitemap and the `robots.txt`
  as built locally. Lighthouse's rule on `robots.txt` passed on the
  local build.
- Pushed by the user up to `cc900f2`; the commits from `6061b1f` on,
  this minute included, are not pushed yet.

## Open points

- When the SEO check is resumed: with its pages taken from the sitemap,
  the "page not found" is never checked, so the exemption derived from
  `noindex` never applies; the card holds the question.
- The local command `--check-updates` has not yet met a real answer
  from GitHub with changes in the list.
- `actionlint` is not installed; the workflows were changed twice
  without it.

## Next steps

- Resume the SEO check, with its pages read from the sitemap: card
  [check SEO with Lighthouse in the build](../wip/check-seo-with-lighthouse.md).
- Watch the rules Lighthouse finds not applicable: card
  [watch the rules Lighthouse finds not applicable](../backlog/watch-lighthouse-rules-not-applicable.md).
- Structured data, and asking robots for a citation: card
  [check the structured data](../backlog/check-structured-data.md).
- Titles, descriptions and dates into a front matter: card
  [move titles, descriptions and dates into the Markdown's front matter](../backlog/move-page-fields-into-markdown-front-matter.md).
- The home page's date in the sitemap: card
  [give the home page a date in the sitemap](../backlog/date-the-home-page-in-the-sitemap.md).
- The publication date in the sitemap: card
  [use the publication date in the sitemap](../backlog/use-publication-date-in-the-sitemap.md).
- The Python scripts in Rust: card
  [port the Python scripts to Rust, run with Docker](../backlog/port-python-scripts-to-rust-with-docker.md).
- Check the workflows with `actionlint`: card
  [check the workflows with actionlint](../backlog/lint-workflows-with-actionlint.md).
