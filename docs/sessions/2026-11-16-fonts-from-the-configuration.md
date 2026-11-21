# Session 2026-11-16 — The fonts from the configuration

**Participants:** the user (author); the agent (Claude Opus 5.5), with
three sub-agents Claude Sonnet 5.5 (one named the fonts by role in the
stylesheet; one moved the pixel font into a folder of its own and wrote
the test that the font files load; one made the build read the fonts
from the configuration).
**Goal:** make a change of font made for the style guide a change of
the whole site, starting from the card
[link the fonts to the build](../board/cards/link-the-fonts-to-the-build.md),
cut during the session into
[give the fonts three roles in the stylesheet](../board/cards/give-the-fonts-three-roles-in-the-stylesheet.md)
and itself. Technical details and reasons are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The stylesheet is made parametric first; the build, the fonts page and the measures on a font come after, in this order, each in a card of its own | user | agent | — |
| 2 | Three fonts named by role, `font-text`, `font-code` and `font-logo`; everything else refers to the roles | user | agent | — |
| 3 | The configuration file arrives with the build, not in the stylesheet's slice, since the stylesheet cannot read one | user | agent | — |
| 4 | The roles card's five acceptance tests | user | agent | — |
| 5 | A browser test that every font file loads, added to the first slice of the configuration card | agent | — | user |
| 6 | The roles' file holds only which font plays each role | user | agent | — |
| 7 | Each font's description in a file of its own, kept in `config/` rather than beside the font's files | user | agent | — |
| 8 | Plain text files for now; the organisation of every configuration file is reviewed with the structured configuration card | user | agent | — |
| 9 | A font is named by its folder in `assets/fonts/`; the CSS family name is written inside its description | user | agent | — |
| 10 | The pixel font gets a folder of its own, as the configuration card's first slice | user | agent | — |
| 11 | The description's form: `key=value`, keys with a free prefix for each file, style and weight written, `.name` the label the fonts page shows | user | agent | — |
| 12 | The configuration card's eight acceptance tests, with the folder name `ibm-vga-8x16` and the roles' file in `key=value` form | user | agent | — |
| 13 | Comment lines in the descriptions, a refusal of two files with the same style and weight, and the font files' test counting the built stylesheet's rules | agent | — | user |
| 14 | `--font-*: initial` written by the build before the roles, instead of in the stylesheet | sub-agent (Sonnet) | — | agent, user |
| 15 | Refusals beyond the agreed ones: names that could point outside `assets/fonts/`, a family name with a quote, a file name that is a path | sub-agent (Sonnet) | — | agent, user |
| 16 | The session closes after the configuration; the fonts page is taken up in a new session | user | agent | — |

## Discussion

1. **Where the font is written.** The user asked to make the change of
   font automatic from the style guide. The agent listed four places
   where a font is written by hand; asked by the user which are in the
   stylesheet, it found three, and on the user's remark that the
   problem was only in the fonts page, showed that the running text of
   every page took a hand-written Literata, so a change of the theme's
   font would reach the samples and not the text. The user asked to
   make the stylesheet parametric first. → decision 1
2. **Roles and a configuration file.** The user proposed three fonts
   named by role, configured in a file. The agent raised that the
   stylesheet cannot read a file, so the configuration comes with the
   build, and that a card already waits for a structured configuration
   file. The user agreed to the order. → decisions 2, 3
3. **The first card.** The agent cut the card, proposed five acceptance
   tests, which the user accepted, and found that Tailwind's default
   code font was the pixel font. A first sub-agent made the change;
   nothing visible changed. The agent found that the sub-agent's report
   on Tailwind's defaults was wrong, checked it on the previous build,
   and recorded the correction in the card. → decision 4
4. **What the configuration holds.** The agent set out the two kinds of
   information, the role's and the font's. The user chose roles only in
   the roles' file and a description per font; first beside the font's
   files, then in `config/`, which is not published. The user chose
   plain text for now, over the structured configuration card. The
   agent raised that a font's CSS name is not a file name, and the user
   confirmed the folder as the name. → decisions 6, 7, 8, 9
5. **The pixel font's folder.** The user asked for it, for uniformity;
   the agent proposed a card of its own, and the user chose the first
   slice of the configuration card. → decision 10
6. **The description's form.** The user proposed `key=value` keys with
   a prefix per file; the agent read `.name` as the fonts page's label,
   which the user confirmed, and raised that style and weight inferred
   from the prefix would close the list of weights; the user chose
   written keys. → decisions 11, 12
7. **The slices.** Reviewing the references, the agent found that no
   test checked that the font files load, and added it to the first
   slice. A second sub-agent moved the font, and the test failed on a
   broken path as it should. The agent added three details to the
   second slice's brief and told the user, who did not object. The
   third sub-agent built the configuration and took two decisions of
   its own on the way, which the agent reviewed. Nothing visible
   changed. → decisions 5, 13, 14, 15
8. **Closing.** Offered the fonts page next, the user chose to close
   and continue in a new session. → decision 16

## Outcome

- The card on the fonts' link to the build cut, the roles' card opened,
  two cards noted in the backlog, the plan updated (`36a20c7`), and the
  roles' acceptance tests agreed (`ac0747a`).
- The fonts named by role in the stylesheet (`efb5e57`); the card
  closed (`3ea9c44`).
- The configuration card taken up (`620c87c`), its answers and
  acceptance tests agreed (`0f89296`).
- The pixel font in a folder of its own, with a browser test that every
  font file loads (`f2640ca`).
- The fonts read by the build from `config/fonts.txt` and
  `config/fonts/`, the stylesheet naming no font, the repository map in
  `CLAUDE.md` updated (`eb44029`); the card closed (`3a56ca1`). Checks
  of `build.yml` run locally: 392 unit tests, 270 browser tests passed,
  one skipped (the "page not found" test that runs on the live site
  only). Nothing pushed.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- The fonts page built from the configuration: card
  [build the fonts page from the configuration](../board/cards/build-the-fonts-page-from-the-configuration.md).
- What is measured on a font, taken again when it changes: card
  [measure the fonts again when they change](../board/cards/measure-the-fonts-again-when-they-change.md).
- How every configuration file is organised, now that the fonts are a
  second setting: card
  [use a structured configuration file](../board/cards/use-a-structured-configuration-file.md).
