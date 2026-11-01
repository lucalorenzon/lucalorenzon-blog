# Session 2026-10-31/11-01 — Curriculum on the home page

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Haiku 5.5 (card moves and commits of the content) and
Claude Sonnet 5.5 (page, look and their tests).
**Goal:** do the card
[put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md),
up to a version ready to publish. Reasons and technical details are in
the card.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The page title becomes "Curriculum Vitae" | user | agent | — |
| 2 | The Markdown is converted by a library at build time, not by hand; `pulldown-cmark` | user | agent | — |
| 3 | No email, phone number or address on the site; the TryHackMe profile stays | user | agent | — |
| 4 | The current employer becomes "Italian Luxury Brand"; the clients are anonymized, except Telecom Italia | user | agent | — |
| 5 | A version written for the site, drafted by the agent from the pieces of the private curriculum repository and from LinkedIn | user | agent | — |
| 6 | The site version is copied into this repository for now; fetching from other repositories becomes its own card | user | agent | — |
| 7 | The site is in English; Italian is removed | user | agent | — |
| 8 | Education: degree and thesis only | user | agent | — |
| 9 | The current role lists its recent work, except a project just started and a training course still being defined | user | agent | — |
| 10 | Reader and first impression: someone deciding whether to engage the author; the profile says "product engineer" | user | agent | — |
| 11 | Each role becomes one line plus its main work; about 1,800 words accepted for this increment | user | agent | — |
| 12 | The competence is kept apart from the employer's inventory of systems, with three questions per line | user | agent | — |
| 13 | The first committed text is replaced before any push, not corrected on top; nothing is pushed until the session's end | user | agent | — |
| 14 | Accessibility tree test: header and footer in full, content by its headings; version 0.4.0, still at 0 | user | agent | — |
| 15 | A temporary look in the terminal style; abstracted at the second article, tied to a design system at the third | user | agent | — |
| 16 | The page slice gets a minimal text column, so every commit passes its tests | user | agent | — |
| 17 | No line above the first section; space between the content and the footer | user | agent | — |
| 18 | A card for the screen reader check of the English site | agent | — | user |
| 19 | The card's section on what is made public stays short and names no internal rule | user | agent | — |
| 20 | A link to the LinkedIn profile, which carries the references until the site stands on its own | user | agent | — |

## Discussion

1. **Title, library and personal data.** The agent listed the
   constraints at once (one level-1 heading, ADR-001, the language, a new
   dependency, personal data). The user answered the title, the library
   and the data together; the agent then pointed out that a library
   needs the file in this repository, which is public, so the user chose
   to clean the file before it is committed. → decisions 1, 2, 3, 4
2. **Which text.** The agent read the private curriculum repository and
   found that the composed version was written for automatic screening.
   The user chose a version for the site, drafted by the agent, and to
   copy it here for now, since fetching it would bring credentials into
   the card. → decisions 5, 6
3. **Language.** The agent noted that an English text in a page declared
   Italian is mispronounced by screen readers. The user made the whole
   site English. → decisions 7, 18
4. **The draft.** The agent found that a "Projects" section referred to
   in the curriculum did not exist; the user pasted the projects from
   LinkedIn and added recent work from memory, correcting the agent
   where it had attributed work to the wrong period. → decisions 4, 8, 9
5. **Length.** The agent asked who reads the page; the user named the
   reader and four messages for the first half minute. The agent
   proposed two structures; the user picked the one without HTML. The
   agent had estimated a 30–40% cut; the result was about 11%, which it
   reported. The user accepted the length for now. → decisions 10, 11
6. **Security.** Rereading the committed text, the user saw that it
   described the employer's systems. The agent listed what each line
   exposed and proposed three questions to decide each one; the user set
   the limit at what a colleague's LinkedIn profile says. The unpushed
   commit was replaced. → decisions 12, 13
7. **Building it.** The page slice stopped on the old centring style,
   which laid the curriculum out as one wide row; the user chose a
   minimal column inside that slice. The user approved the look and,
   seeing it served locally, asked for two fixes. → decisions 14, 15,
   16, 17
8. **The card's text.** The agent noticed that the card described the
   reasoning about the employer in a public repository; the user
   rewrote the section, shorter. The user then asked whether hiding the
   clients cut away references; the agent observed that the site no
   longer led to LinkedIn, where the references are. → decisions 19, 20

## Outcome

- `8545db8`, `0417794`: card started; card on content from several
  repositories opened.
- `b11b743`: the site version of the curriculum.
- `2f1b629`: the page renders it, in English, as "Curriculum Vitae".
- `9d57ea0`, `40b4cb8`, `af6c52d`: the look, with its layout test.
- `4e67ede`, `cfa1d50`: card closed; plan, changelog (0.4.0), README,
  `CLAUDE.md`, ADR-001 and the screen reader card updated.
- `5c01bc0`: the link to the LinkedIn profile.
- Two Dependabot updates of the browser tests' tools arrived from
  GitHub during the session; nothing in them conflicted with this work.
- Nothing pushed; this minute is the session's last commit.

## Open points

- The site should stand on its own, references included, without
  naming the clients. A card is to be opened when the articles arrive,
  together with a review of the curriculum.
- Folding details behind a click without HTML inside the Markdown.

## Next steps

- Push and publish version 0.4.0 with the tag `v0.4.0`: card
  [put the curriculum on the home page](../done/2026-11-01_put-curriculum-on-home-page.md).
- Choose the site's public address: card
  [choose the site's public address](../backlog/choose-site-address.md).
- Run the screen reader check on the English site: card
  [update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md).
