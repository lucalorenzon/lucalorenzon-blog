# Session 2026-11-02 — The licence of the articles

**Participants:** the user (author); the agent (Claude Opus 5.5), with a
sub-agent on Claude Haiku 5.5 (moving the card).
**Goal:** answer the card
[choose the licence of the articles](../done/2026-11-02_choose-articles-licence.md),
before the first articles come to the site. The reasons and the
rejected options are in the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The articles' text, and the author's own images, under CC BY 4.0 | user | agent | — |
| 2 | The articles' code samples under MIT-0: a citation is a courtesy, not a condition | user | agent | — |
| 3 | The curriculum under CC BY-ND 4.0; translations made by the author | user | agent | — |
| 4 | Quotations and images by others keep their own terms and name their source | user | agent | — |
| 5 | The licences are shown on the pages too, as one sentence for the whole site, "unless otherwise noted", the curriculum being the exception | user | agent | — |
| 6 | Showing the licences on the site is a card of its own, in Now before the articles from Medium; this card closes with the repository's files | user | agent | — |
| 7 | Creative Commons texts linked, not copied; `LICENSE` and `Cargo.toml` stay MIT for the code; no changelog entry, since the site does not change | agent | — | user |
| 8 | New card in Later: the curriculum in more languages | user | agent | — |

## Discussion

1. **What a reader may do.** The agent asked what a reader should be
   allowed to do, without proposing a licence. The user answered that
   they write to pass on knowledge, open a discussion or tell a story,
   and hope for reuse. The agent pointed out that the card had been
   opened over commercial reuse; the user said profit by others is
   welcome as long as they are cited. → decision 1
2. **Distortion.** The user accepted adaptations, but not distortions,
   doubting they can be prevented, and recalled Barthes' "death of the
   author". The agent checked the licence's legal code: changes stated,
   no endorsement, name removed on request, moral rights kept. → decision 1
3. **Code samples.** The agent raised that CC BY on code would burden
   whoever pastes a snippet. The user found it heavy and asked how
   Apache 2.0 differs from MIT and CC BY; the agent compared the three
   and asked whether a citation should stay a condition for code. The
   user chose a courtesy, and MIT-0. → decision 2
4. **The curriculum.** The agent raised that a curriculum describes a
   person rather than an idea. The user wanted it copied, not
   distorted; the agent pointed out that this forbids shared
   translations too, and the user said they will translate it
   themselves. → decisions 3, 8
5. **What is the author's own.** The user raised that images may come
   from others, and that an article on someone else's concepts is not
   wholly the author's. The agent explained that a licence covers the
   text, not the ideas, and that material by others keeps its own
   terms. → decision 4
6. **On the page, and how others do it.** The user wanted the licence
   visible on the pages. Before taking on the work, the user asked how
   other blogs do it; the agent looked at six personal blogs: four
   state only "©" or nothing, the two with a licence use one sentence
   for the whole site. The user chose the same, with the curriculum as
   the exception, and split the site's part into a card of its own. → decisions 5, 6, 7

## Outcome

- Card started (`8c2eb70`, by the Haiku sub-agent).
- New card [show the licences on the site](../backlog/show-licences-on-the-site.md)
  in Now (`303a1f3`).
- Licences declared in the repository: `content/README.md`,
  `content/LICENSE-MIT-0`, `README.md`, `CLAUDE.md` (`ce20fcd`).
- Card closed, links and plan updated (`378f040`).
- New card [publish the curriculum in more languages](../backlog/publish-curriculum-in-more-languages.md)
  in Later (`c62a668`).

## Open points

- Whether Italian law keeps the moral right to integrity with the
  author whatever the licence says was stated by the agent "as far as
  it knows", not checked with a lawyer.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Show the licence sentence on every page and the exception on the
  curriculum: card
  [show the licences on the site](../backlog/show-licences-on-the-site.md).
- Check, for each image of the articles from Medium, whether it is the
  author's and, if not, its source and terms: card
  [add the two articles already on Medium](../backlog/add-the-two-medium-articles.md).
- Translate the curriculum: card
  [publish the curriculum in more languages](../backlog/publish-curriculum-in-more-languages.md).
