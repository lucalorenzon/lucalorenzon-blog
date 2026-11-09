# Session 2026-11-09 — Errors with thiserror

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Claude Sonnet 5.5 (six tasks, one per slice of code).
**Goal:** do the card
[refactor the code's errors with thiserror](../done/2026-11-09_refactor-errors-with-thiserror.md),
the second of the refactors in Now. Technical details are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Every error moves to thiserror, plain text and generic box included; the hand-written implementations go | user | agent | — |
| 2 | One mechanism for every error, and every error says where and what | user | agent | — |
| 3 | Errors start in the domain and are returned; only the adapter prints | user | agent | — |
| 4 | Warnings are the same errors, tolerated: a stop in the pipeline, a warning elsewhere, decided in the adapter | user | agent | — |
| 5 | Separating the domain from the adapters is a card of its own | user | agent | — |
| 6 | Errors first, the separation after, which revisits the errors | user | agent | — |
| 7 | A message may change when it says more, shown before and after each commit | user | agent | — |
| 8 | Six slices, one commit each, each with a built site byte-identical | user | agent | — |
| 9 | A warning reads as the error, then what the build does instead | agent | — | user |
| 10 | The YAML reader's position written as compilers write it, `file:line:column:` | agent | — | user |
| 11 | Tailwind reading only the presentation is a card of its own, tied to the separation | user | agent | — |
| 12 | The design system comes next; push at the close | user | agent | — |

## Discussion

1. **Which errors.** The agent listed the three ways errors were
   written and noted that the card's own criterion, "only those
   matched by the code", separated nothing, since no error was matched
   outside the tests; it advised moving all of them. The user agreed,
   asking for the hand-written code to go and for one structure, which
   the user then defined as one mechanism and where plus what. →
   decisions 1, 2
2. **Domain and adapters.** The user set the principle: errors start in
   the domain, only adapters print. The agent found the domain still
   wrote warnings as sentences; the user answered that warnings are
   errors tolerated, and that the real difficulty is that the code has
   no boundary between domain and adapters, the file system standing
   for several concepts. The agent proposed reading this as two cards;
   the user confirmed, then doubted the order. The agent advised errors
   first, since most of them are pure domain and would survive the
   separation, at a confidence below 90% because some types may change
   with it; the user chose errors first, the separation revisiting
   them. → decisions 3, 4, 5, 6
3. **Messages.** The agent noted the card promised identical messages
   while hoping to remove the doubled "error:"; the user allowed
   changes that carry more information. The agent proposed six slices,
   approved by the user. → decisions 7, 8
4. **The shape of a message.** While doing the slices the agent chose a
   common shape for warnings and the compilers' form for positions,
   showing every changed message before its commit; the user approved
   each commit. → decisions 9, 10
5. **Tailwind.** The byte-by-byte comparison caught a stray
   `.contents` rule, born from a parameter's name. The user recalled
   that Tailwind had already been limited to the files that matter,
   and tied the cause to the missing separation; the agent found the
   two earlier cases and proposed a card of its own, linked to the
   separation's, since some answers need not wait for it; the user
   agreed. → decision 11
6. **The close.** The agent offered three things before the minute:
   a stale link in a code comment, a push, and what enters Now. The
   user chose the design system next and the push. → decision 12

## Outcome

- Card [refactor the code's errors with thiserror](../done/2026-11-09_refactor-errors-with-thiserror.md):
  started (`09646f8`); answered (`adff226`); six code commits, one per
  slice: the hand-written types (`f5c5e05`), the robots list
  (`2e930b1`), the sitemap (`fc2a92a`), the articles (`f66f0ca`), the
  build's error type (`6ccfdd0`), the YAML reader's position
  (`7a68629`); moved to done (`16b0cb9`); closed (`7867cf4`).
- Cards opened: [separate the domain from the adapters](../backlog/separate-the-domain-from-the-adapters.md)
  (`8c512ff`) and [let Tailwind read only the presentation](../backlog/let-tailwind-read-only-the-presentation.md)
  (`ac93e47`), both in Next.
- 691 lines added and 400 removed in `src/`, `Cargo.toml` and
  `Cargo.lock`; the built site byte-identical after each code commit.
  261 unit tests and 227 browser tests pass, the live-only test
  skipped.
- Pushed at the close, with the dates session's commits.

## Open points

- The comment at the top of `src/robots_txt.rs` still points to
  `docs/wip/add-robots-txt.md`, now
  `docs/done/2026-11-07_add-robots-txt.md`; not corrected in this
  session.

## Next steps

- Run the spike on Holt, then build the catalogue that frees the two
  articles: card [build a design system](../backlog/build-a-design-system.md).
- Separate the domain from the adapters, revisiting the errors: card
  [separate the domain from the adapters](../backlog/separate-the-domain-from-the-adapters.md).
- Keep Tailwind from reading anything but the presentation: card
  [let Tailwind read only the presentation](../backlog/let-tailwind-read-only-the-presentation.md).
