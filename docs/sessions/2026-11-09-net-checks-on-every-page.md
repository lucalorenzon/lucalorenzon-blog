# Session 2026-11-08/09 — Check every page with tests written once

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Sonnet 5.5 (test code and an on-demand script, four
tasks).
**Goal:** pick up the plan and do the last slice of
[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md),
checking every page with tests written once, split during the session
into
[take the net's pages from the sitemap](../done/2026-11-09_take-the-nets-pages-from-the-sitemap.md),
[check accessibility on every page with the net](../done/2026-11-09_check-accessibility-on-every-page-with-the-net.md)
and
[check the layout on every page with the net](../done/2026-11-09_check-the-layout-on-every-page-with-the-net.md).
Technical details and reasons are in those cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Do the last slice of ADR-004 before the card on dates | user | agent | — |
| 2 | Sideways scroll stays in the net; a card on breaking long words in the text | user | agent | — |
| 3 | The slice split in three parts, one decision each | user | agent | — |
| 4 | `hasRoles`, `hasConsecutiveParagraphs` and "at least one level-2 heading" dropped as content facts | user | agent | — |
| 5 | The net's pages come from the build's sitemap, one test per page; the live check copies only that sitemap | user | agent | — |
| 6 | axe runs once per page, at 1280; the components' accessibility is checked in their own tests | user | agent | — |
| 7 | The headings' structure by three axe rules named one by one; the whole best practice group on demand, locally | user | agent | — |
| 8 | The banner and footer checked by their shape, the exact texts left to Rust's unit tests | user | agent | — |
| 9 | The loss of the section titles accepted; a card on a periodic check of the published text against its Markdown | user | agent | — |
| 10 | A check of our own against repeated identifiers | user | agent | — |
| 11 | The frame keeps three widths, on the home page and the "page not found" | user | agent | — |
| 12 | Components checked once, on a real article taken as worst case, with a guard that it still is | user | agent | — |
| 13 | An empty table header is refused by the build, in the card on the pages' text | user | agent | — |
| 14 | Sideways scroll with the reader's spacing moves to the net; ADR-004 carries a note | user | agent | — |
| 15 | Three pushes, to see each part run in the pipeline | user | agent | — |

## Discussion

1. **Where to start.** The agent read the plan and proposed finishing
   ADR-004's last slice before the card on dates, since it is half done
   and the suite grows with each article meanwhile; the user agreed. →
   decision 1
2. **Sideways scroll and long words.** The agent first proposed moving
   sideways scroll to the components; the user objected that it would
   then have the same problem as the 80 characters per line. The agent
   checked and found it had been wrong: the text breaks only at spaces,
   so a long word scrolls the page and the check depends on the text.
   The user then reasoned that 80 of the widest characters fitting the
   screen would rule out scrolling; the agent measured that they never
   fit, and that the 80 characters bound lines, not words. The user
   asked for a card on a style rule breaking long words. → decision 2
3. **Roles.** The user objected to dropping `hasRoles`, reading it as
   accessibility roles; the agent explained it names the curriculum's
   job roles, level-3 headings, and asserts a fact about the text. The
   user agreed to drop it. → decision 4
4. **Splitting.** The agent proposed three parts, one decision each;
   the user agreed. → decision 3
5. **The list of pages.** The user asked why the tests could not simply
   be parametric, then pointed out that the build already has every
   file before the browser tests, and that what is tested is what the
   build generated, not the published site. The agent had first
   proposed copying the published site; on the user's point, the live
   check takes only the sitemap from the build's output, which also
   keeps the table tests skipping there. → decision 5
6. **axe and the components.** The agent raised that the record viewer
   and the code toggle exist only at some widths; the user moved their
   checks to the components' own tests. The first version ran them on
   every article at five widths: 60 more tests, growing with each
   article. The agent stopped before committing; the user chose the
   worst case at two widths, and the agent measured that the hybris
   article is it. → decisions 6, 12
7. **Best practices.** The agent found axe's heading rules off, being
   best practices; the user chose three by name in the tests and asked
   to see how the site fares on the whole group, then to keep the check
   as an on-demand script. The site passes all 27 rules. → decision 7
8. **The tree.** The agent found that the expected texts of the banner
   and footer come from the code and are already checked by Rust's
   unit tests, and that the two expected trees were identical; the
   agent proposed checking the shape, and the user agreed. → decision 8
9. **Lost sections.** The user accepted that a section lost while a
   page is composed is no longer noticed by the tests, and proposed a
   periodic check of the published text against its Markdown instead.
   The agent noted that the published site is the last tag, so the
   comparison must use that tag's Markdown. → decision 9
10. **Repeated identifiers.** The agent found the pages have none and
    proposed leaving them to axe's remaining rule; the user wanted a
    check of our own. → decision 10
11. **The frame.** The agent proposed 1280 for the net and the home
    page plus the "page not found" for the frame at three widths; the
    user agreed. → decisions 6, 11
12. **Empty table headers.** A fault made on purpose went unnoticed;
    the user asked whether the build catches it. The agent found it
    does not, and noted that a rule in the build guarantees it for
    every article; the user put it in the card on the pages' text. →
    decision 13
13. **Scroll with the reader's spacing.** The agent pointed out that,
    by the criterion of point 2, ADR-004's classification of this check
    as a component no longer held; the user moved it to the net. →
    decision 14
14. **The layout's worst case.** The agent proposed After Save Event,
    which has every kind of text element except level-4 headings. The
    user agreed for now, saying their instinct is that components are
    tested on texts written for the purpose, without being able to
    argue it yet; kept as an open point. → decision 12
15. **Sub-agents' work, checked.** The agent re-measured every count a
    sub-agent reported (one explained a count wrongly), ran the
    on-demand script again on a fresh build, and fixed two misplaced
    or stale comments; each new check was seen failing on a fault made
    on purpose.
16. **Measuring.** The user had each part pushed and its build checked.
    The tests went from 300 to 227 with almost the same step time; the
    agent traced it to the SEO check and the container's setup. →
    decision 15

## Outcome

- Browser tests: 315 → 300 → 227 (913 when ADR-004 was written); in the
  pipeline the browser test job took 124, 117 and 113 seconds (runs
  42571013921, 42576172246, 42578666728).
- The card on checking every page with tests written once split in
  three (`a675f8b`); all three closed:
  [take the net's pages from the sitemap](../done/2026-11-09_take-the-nets-pages-from-the-sitemap.md)
  (`5cc93a7`, `4f2c9bc`),
  [check accessibility on every page with the net](../done/2026-11-09_check-accessibility-on-every-page-with-the-net.md)
  (`ef36c9f`, `43f415e`, `ed27a1d`, `dac34de`),
  [check the layout on every page with the net](../done/2026-11-09_check-the-layout-on-every-page-with-the-net.md)
  (`e6107e8`, `d767f91`, `cf491de`).
- On-demand check of axe's best practices:
  `end2end/tools/check-axe-best-practices.mjs` (`43f415e`).
- Cards opened:
  [break long words in the text](../backlog/break-long-words-in-the-text.md)
  (`8373e39`),
  [check periodically that the published text matches its Markdown](../backlog/check-published-text-against-markdown.md)
  (`c69a638`); card extended:
  [check the pages' text in the build](../backlog/check-page-bodies-in-the-build.md),
  with empty table headers (`d6f91a4`).
- ADR-004 carries a note: the long word with the reader's spacing is
  no longer a loss.
- The generated site is unchanged; pushed up to `cf491de`, this minute
  is local.

## Open points

- The user is not fully convinced that components should be checked
  on a real article taken as the worst case; the question belongs to
  the components' test pages.
- The live check's new step, which copies the build's sitemap, has not
  run yet: it runs only when a version tag is pushed.
- Level-4 headings, only in the curriculum, are not covered by the
  layout's worst case.
- With 227 tests the pipeline's browser test step still takes about
  1.3 minutes: the SEO check, about 30 seconds alone, and the container's
  setup now bound it.

## Next steps

- Put the components' cases in test pages shaped as a catalogue: no
  card yet, to be written with the user (ADR-004, Consequences; plan,
  Now).
- Start the SEO check first: card
  [start the SEO check first](../backlog/start-the-seo-check-first.md).
- Break long words in the text: card
  [break long words in the text](../backlog/break-long-words-in-the-text.md).
- Check the published text against its Markdown: card
  [check periodically that the published text matches its Markdown](../backlog/check-published-text-against-markdown.md).
- Refuse empty table headers in the build: card
  [check the pages' text in the build](../backlog/check-page-bodies-in-the-build.md).
