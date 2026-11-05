# Session 2026-11-05 — Readability first, amber as an accent

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Sonnet 5.5 (code and tests of each slice) and Haiku 5.5
(formatting fix).
**Goal:** continue the card
[give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md),
starting from the text size. Reasons and rejected options are in that
card and in
[ADR-003](../adr/ADR-003-readability-first-amber-as-accent.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The bigger text applies to the curriculum too | user | agent | — |
| 2 | Readability first, identity in the content; amber as an accent (ADR-003) | user | agent | — |
| 3 | Logo in the pixel font, text and title in Literata, fonts served by the site | user | agent | — |
| 4 | Light or dark background chosen by the reader, first through the browser's preference | user | agent | — |
| 5 | Not Times New Roman and Courier New | user | agent | — |
| 6 | Code inside a sentence in amber | user | — | agent |
| 7 | Plan: ADR, cards and plan updated, seven slices | user | agent | — |
| 8 | Fix the failed build in a commit of its own | user | agent | — |
| 9 | Title line height 1.5; logo column follows the logo; header test accepts two lines | agent | Sonnet | user |
| 10 | Text 18/20 pixels, footer 16/18, code at 85% of the text | user | agent | — |
| 11 | Slices 2 and 3 joined in one commit | user | agent, Sonnet | — |
| 12 | Code in Hack, not Meslo LG nor DejaVu Sans Mono; ADR-003 corrected | user | agent | — |
| 13 | A long name in inline code may break where it does not fit | agent | Sonnet | user |
| 14 | Headings: sections bold amber, sub-sections bold italic amber, no lines, curriculum too | user | agent | — |
| 15 | The content-layout test checks headings by colour | agent | Sonnet | user |
| 16 | Quotes with an amber bar, without a new trial | user | agent | — |
| 17 | Tables in a new session | user | agent | — |

## Discussion

1. **Size, then the whole look.** The agent asked whether the bigger
   text was for the articles only; the user answered for the
   curriculum too, then questioned the BBS look: identity had been put
   before readability. The agent proposed that identity and
   readability compete only if both live in the text; the user placed
   identity in the content. → decisions 1, 2
2. **Trials.** The agent showed a BBS frame and a neutral page, light
   and dark; the user asked for a neutral dark one, then proposed an
   amber accent on both backgrounds with Literata everywhere, then the
   pixel logo back and a monospaced font for code. → decisions 2, 3, 4
3. **System fonts.** The user asked whether Times New Roman and Courier
   New would do, saving the font files; the agent reported they are
   missing on Android and most Linux systems and read worse on screen,
   with a confidence of 85% on the reading part. → decision 5
4. **The plan.** The agent proposed recording the direction as an ADR,
   then seven slices; the user added amber for code inside a sentence.
   → decisions 6, 7
5. **The failed build.** The user found the build of the previous
   session's last push red; the agent reproduced it (formatting and
   clippy in `src/markdown.rs`) and proposed a separate fix, and from
   then on gave the sub-agents the formatting and clippy checks too.
   → decision 8
6. **Header.** Sonnet stopped on header tests written for the pixel
   title; the agent measured the pages, found the logo's fixed column
   too narrow under the reader's spacing, and decided the fixes.
   → decision 9
7. **Size.** The user asked for a local server rather than
   screenshots, compared four sizes in the browser, preferred the
   largest but feared the gap with code and footer; the agent pointed
   out the gap would close with the code font, and the user chose
   18/20 from a second trial. → decision 10
8. **Joined slices.** With code still at 16 pixels a line held 82
   characters; the agent proposed joining the slices, as the user had
   chosen size and code together. → decision 11
9. **Licence.** Preparing Meslo LG, the agent found it carries no
   licence and stopped; the ADR's "Apache 2.0" had come unchecked from
   Nerd Fonts. The agent proposed DejaVu Sans Mono; the user found its
   140 KB heavy and asked about Hack, 23 KB. → decision 12
10. **Headings.** The user asked to try the curriculum too; the agent
    added amber section titles to the trial, and the user asked for
    amber italic sub-titles, choosing bold over regular italic.
    → decisions 14, 15
11. **Quotes and tables.** The user judged quotes simple enough without
    a trial, and tables complex enough for a session of their own.
    → decisions 16, 17

## Outcome

- Every page: off-white text with amber accents, bigger text, code in
  Hack, amber headings without lines; articles: quotes with an amber
  bar. 85 unit tests and 220 browser tests pass (Chromium).
- Commits: `5106984` ADR-003, `008e39f` cards and plan follow it,
  `8cc1b4a` formatting fix, `3f37e2c` colours, `181c321` ADR-003
  corrected to Hack, `a432465` text size and code font, `ed0d880`
  headings, `7f96530` quotes, `967f056` plan, plus this minute.

## Open points

- The commits of this session are not pushed: the build on GitHub stays
  red on the previous push until they are.
- Which background a reader with no preference gets, and the light
  shades of amber seen on the page: left to the light-background slice.

## Next steps

- Tables, then the light background. Card
  [give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md).
- What a table too wide for a phone does, and the hybris PK article
  back in the checks. Card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md).
- A reader's control to choose the background on the site. Card
  [let readers adjust font and spacing](../backlog/let-readers-adjust-font-and-spacing.md).
