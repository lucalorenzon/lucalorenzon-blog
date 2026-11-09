# Session 2026-11-09 — Dates with jiff

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Claude Sonnet 5.5 (two tasks: the articles' dates, and
the sitemap's dates).
**Goal:** do the card
[give dates a module of their own](../done/2026-11-09_give-dates-a-module-of-their-own.md),
the first of the refactors in Now. Technical details are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | A widely used library for dates, not a module written for the site | user | agent | — |
| 2 | A year written with two digits must not pass | user | agent | — |
| 3 | jiff, which is strict on its own, instead of chrono, first chosen | user | agent | — |
| 4 | `Date` and `ContentDate` both give way to jiff's day, with no type of the site's own | user | agent | — |
| 5 | The year zero passes | user | agent | — |
| 6 | The code in two slices, one per type, each checked by a built site byte-identical to the one before | user | agent | — |
| 7 | jiff's messages for a wrong date kept as they are | user | agent | — |
| 8 | A card's move committed apart from its rewrite, so git follows the rename | agent | — | user |
| 9 | The card keeps its title, though the answer is a library | user | agent | — |
| 10 | No push at the close: the commits wait for a pipeline run worth it | agent | — | user |

## Discussion

1. **Library or module.** The agent read both types, found they did not
   agree, and advised a module of the site's own, thinking chrono
   would still need a check of the form. The user chose chrono, not
   to reinvent something widely used. → decision 1
2. **chrono's reading.** The agent tried chrono and found it takes
   `2026-11-6`, `+2026-11-06` and `26-11-06` as year 26. The user
   first proposed accepting those forms too, to avoid a module; the
   agent named the two-digit year as the one with a concrete risk, an
   article silently dated year 26, and the user ruled it out. The user
   then asked whether chrono could be given a strict format; the agent
   tried several and found none. → decision 2
3. **Other libraries.** The user asked whether Rust has only chrono.
   The agent tried time and jiff on the same cases: jiff refused every
   risky form on its own and added two libraries. The agent advised
   jiff, revising the user's first choice openly; the user chose it.
   With no rule of the site's own left, both types went. → decisions
   3, 4
4. **The year zero.** jiff takes it, the articles refused it. The agent
   proposed letting it pass, as a mistake no one makes; the user
   agreed. → decision 5
5. **The code.** The agent proposed two slices, the articles then the
   sitemap, with the built site compared before and after each; the
   user approved. The sub-agent did both. → decision 6
6. **The messages.** As promised before the first commit, the agent
   showed the build's message for `2023-02-30` before and after, and
   the new one for `26-11-06`, noting they no longer name the field.
   The user kept jiff's. → decision 7
7. **The card's moves.** Git recorded the first move as a deletion and
   a new file, since the text had changed too much; the agent redid it,
   unpushed, as a move and then a rewrite, and closed the card the same
   way. On closing, it proposed keeping the title; the user approved.
   → decisions 8, 9

## Outcome

- Card [give dates a module of their own](../done/2026-11-09_give-dates-a-module-of-their-own.md):
  started (`1166ae3`); answered with jiff (`efc2bd2`); the articles'
  dates with jiff, and jiff in the project's `CLAUDE.md` (`5c5074b`);
  the sitemap's dates with jiff (`c25996c`); moved to done
  (`e032f1c`); closed (`65a4f33`).
- 190 lines of `src/` removed and 59 added, tests included; the built
  site byte-identical after each code commit. 245 unit tests and 227
  browser tests pass, the live-only test skipped.
- Not pushed.

## Next steps

- Refactor the errors with thiserror: card
  [refactor the code's errors with thiserror](../backlog/refactor-errors-with-thiserror.md).
- Run the spike on Holt, then build the catalogue that frees the two
  articles: card [build a design system](../backlog/build-a-design-system.md).
