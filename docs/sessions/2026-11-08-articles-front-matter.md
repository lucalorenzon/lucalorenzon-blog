# Session 2026-11-08 — The articles' title, description and date in the front matter

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Sonnet 5.5 (the code of the front matter, its changes
after the user's review, and two tests).
**Goal:** take the card on moving titles, descriptions and dates into
the Markdown's front matter, cut it into slices, and build the first:
[read the articles' title, description and date from the front matter](../done/2026-11-08_read-article-fields-from-front-matter.md),
built and closed. Reasons and rejected options are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Take the front matter card, first of Next | user | agent | — |
| 2 | A missing field skips the article with a warning and a final summary on the author's machine, and stops the build in the pipeline | user | agent | — |
| 3 | A wrong field follows the same rule as a missing one | user | agent | — |
| 4 | Fields: title, description and date; the tab title is computed; the address waits for a card of its own, with redirects | user | agent | — |
| 5 | The front matter is real YAML, read with serde-saphyr, reading part only | user | agent | — |
| 6 | The card is split in three: the articles' fields (worked on), the other pages' fields, the addresses from a slug | user | agent | — |
| 7 | The fields are checked in their constructors (`TryFrom`), following "parse, don't validate"; errors written by hand, thiserror in a card of its own | user | agent | — |
| 8 | A field the front matter does not know is a wrong field | user | agent | — |
| 9 | Dates get a card of their own, a library or a module, in Now after the articles | user | agent | — |
| 10 | The front matter is declared with the checked types, built by serde as it reads; the errors are grouped in `FrontMatterError` | user | agent | Sonnet |
| 11 | An empty text and HTML in the text are checked in the build, together, in a card of their own | user | agent | — |
| 12 | Finding the articles in the content folder is a card of its own | user | — | agent |
| 13 | Error messages name the field and the file's line | user | agent | Sonnet |
| 14 | The order of the fields and blank lines do not matter; two tests keep the order free and the line exact | user | agent | Sonnet |
| 15 | New card: one model for the information pages | user | — | agent |
| 16 | New card: a page's own licence terms, more than a single field | user | agent | — |
| 17 | The cards' commit goes before the code's, so every link works in every commit | agent | — | user |
| 18 | The pipeline's failed job is run again, twice, the download of Tailwind having failed on the server's side | agent | — | user |
| 19 | No version 0.12.0: nothing changes for the visitor | user | agent | — |
| 20 | The pipeline's failure does not belong to the articles' card; a card on the build's external downloads, first in the next session | user | agent | — |

## Discussion

1. **What a missing field does.** The user proposed to skip the article
   with a warning and a summary at the end of the build. The agent
   pointed out that the sitemap stops the build in the pipeline for a
   missing date, since nobody reads a warning there and what is built
   is published; the user chose the same two rules, then extended them
   to wrong fields. → decisions 2, 3
2. **Which fields, and which tool.** The agent set out the five fields of
   the code; the user kept three, the tab title computed and the address
   for later. Of the YAML readers checked, two were dropped as
   unmaintained; between a dependency less and a real YAML, the user
   chose real YAML and serde-saphyr. → decisions 4, 5
3. **Cutting the card.** The agent noted that the card held more than
   the agreed slice and proposed three cards in its place; the user
   agreed. → decision 6
4. **Parse, don't validate.** The user asked the sub-agent to use the
   skill, with the checks in `TryFrom` constructors, and asked for a
   card to refactor the errors with thiserror. The agent proposed that
   an unknown field be wrong, to catch a misspelt one; the user
   accepted. → decisions 7, 8
5. **The user's code review.** The user asked why the front matter was
   read into plain strings and then converted; the agent agreed it went
   against the principle chosen, and the user proposed grouping the
   errors in `FrontMatterError`. The sub-agent's demonstration showed
   that the messages lacked the field's name and counted lines one short;
   the agent proposed passing the opening `---` to the reader, which
   worked. The user asked that the fields' order not matter: the agent
   showed it already did not, with the user's example, and the user had
   two tests added. → decisions 10, 13, 14
6. **Cards born from the review.** The user raised the two types of day,
   the text's checks, the list of articles wired into the code, the
   three near copies of the information pages, and a licence per page.
   For the text the agent noted the empty text and HTML belong to one
   type; for the licence the agent set out the tensions, and the user
   explained the real case, the closing note of the SAP articles, more
   complex than a field. Each became a card of its own.
   → decisions 9, 11, 12, 15, 16
7. **Closing.** The user chose not to publish a version, since the
   pages are the same. The pipeline failed on a server error while
   downloading Tailwind, and again both times the agent ran it again.
   The user asked to try Tailwind 4.3.5 and whether the download was
   banned for them: the agent found that 4.3.5 is downloaded the same
   way, that the address failed intermittently from the pipeline and
   from the agent's commands while it worked from the user's browser and
   terminal, and that GitHub's programming interface served the same
   file with the right checksum. The user ruled that the failure does
   not belong to the articles' card, and asked for a card on the build's
   external downloads, to be taken first in a new session.
   → decisions 17 to 20

## Outcome

- Code: `78932e1`. The pages written are the same as before, byte for
  byte; 227 unit tests, 911 browser tests.
- The card split (`e6d7721`) and closed (`fcaa361`); new cards in
  `129fbcc` and `b19f65f`; the card started and its decisions recorded in
  `ebb61c4` and `255be99`; `CLAUDE.md` brought up to date in `e898af3`
  (Leptos 0.8.23, serde-saphyr, the articles' front matter); the card on
  the build's external downloads in `1d9a924`; plus this minute.
- The pipeline of `e898af3` failed three times, at the download of
  Tailwind, after formatting, lint and unit tests had passed; the
  browser tests did not run there (run
  [42465175472](https://github.com/lucalorenzon/lucalorenzon-blog/actions/runs/42465175472)).

## Open points

- The change could not be shown free of regressions in the pipeline,
  because of the failed download, which belongs to the card on
  [the build's external downloads](../backlog/make-external-downloads-in-the-build-reliable.md);
  the browser tests passed on the author's machine only.

## Next steps

- Make the build's external downloads reliable, then see the pipeline
  pass on this session's change: card
  [make the build's external downloads reliable](../backlog/make-external-downloads-in-the-build-reliable.md).
- Give dates a module of their own, or a library: card
  [give dates a module of their own](../backlog/give-dates-a-module-of-their-own.md).
- One model for the licence, about and curriculum pages: card
  [unify the model of the information pages](../backlog/unify-the-information-pages-model.md).
- A page's own licence terms: card
  [let a page state its own licence terms](../backlog/let-a-page-state-its-own-licence-terms.md).
- The other pages' fields in the Markdown: card
  [read the other pages' title and description from the Markdown](../backlog/read-page-fields-from-markdown.md).
- Addresses from a slug, with redirects: card
  [take the pages' addresses from a slug](../backlog/take-page-addresses-from-slug.md).
- The articles found in the content folder: card
  [find the articles in the content folder](../backlog/find-articles-in-content-folder.md).
- The errors with thiserror, and the YAML reader's doubled "error:":
  card [refactor the code's errors with thiserror](../backlog/refactor-errors-with-thiserror.md).
- The text checked in the build: card
  [check the pages' text in the build](../backlog/check-page-bodies-in-the-build.md).
