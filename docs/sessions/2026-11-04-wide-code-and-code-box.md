# Session 2026-11-04 — Wide code and a box for code

**Participants:** the user (author); the agent (Claude Opus), with
sub-agents Haiku (card moves) and Sonnet (spikes, code and tests).
**Goal:** start the card
[let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md),
code first. Technical details and reasons are in that card and in the
[look card](../wip/design-look-of-articles.md).

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | First slice: code only, "After Save Event" back in the checks | user | agent | — |
| 2 | Wide code scrolls sideways; a "wrap" toggle wraps it | user | agent | — |
| 3 | Wrap by grammar with indent as fallback; indent first | user | agent | — |
| 4 | No annotation for code: the code block itself is the signal | user | agent | — |
| 5 | The toggle shows only where a line does not fit | user | agent | — |
| 6 | The toggle, inside the scrolling box, is the keyboard's way in | user | agent | — |
| 7 | `wrap` on screen, "Wrap long lines of code" for screen readers | user | agent | — |
| 8 | Continuation 4 columns deeper than the line | user | agent | — |
| 9 | Thresholds through Tailwind's container prefix, an exception to ADR-001 | user | agent, Sonnet | — |
| 10 | Thresholds in `em`, not `ch` | agent | Sonnet | user |
| 11 | A long word in a role heading may break | agent | Sonnet | user |
| 12 | Letter-spacing limit accepted; alternatives in the look card | user | agent | — |
| 13 | Code blocks: off-white on dark grey, amber frame | user | agent | — |
| 14 | Code inside a sentence: off-white too | user | — | agent |
| 15 | Grammar breaks and syntax colours: cards in Later | user | agent | — |
| 16 | New card in Later: readers adjust font and spacing | user | — | agent |

## Discussion

1. **How thin.** The agent laid out the two articles' code and tables
   and asked how thin the first slice could be; the user chose code
   only, in one article, then "After Save Event", to close the loop:
   its only table fits a phone. → decision 1
2. **What wide code does.** The user asked to research before choosing
   between known options. The agent reported WCAG's reflow rule and
   technique G224, Nielsen Norman Group, practice on other sites; the
   user asked about Kindle, Kobo and technical publishers, and found
   wrapped code "rather illegible", proposing scrolling with a toggle
   to wrap. → decision 2
3. **Grammar or indent.** The user asked whether libraries break code
   like editors do, then for a comparison; the agent drew both on lines
   of the articles at phone width, and at the user's request at iPhone
   SE width, where grammar alone no longer fits. The user chose grammar
   with indent as fallback; on the agent's proposal, indent first, which
   the user agreed to, worried about script weight. → decisions 3, 8
4. **The annotation.** The user had proposed a comment marker; the
   agent found pulldown-cmark drops it, and showed the info string
   after the language, which the user preferred as the standard place.
   The user then chose a word describing the text; the agent pointed
   out that such a word is measurable by the build, and the user
   concluded the code block itself is enough. → decision 4
5. **Only where needed.** The user chose a toggle only where needed; the
   agent proposed per-block container queries, below 90% confidence, and
   a spike confirmed them in Chromium and WebKit. → decision 5
6. **Keyboard.** The agent set a Tab stop on every block against
   relying on the toggle; the user chose the toggle, checked with axe
   first. A second spike showed it passes only inside the scrolling
   box. → decision 6. The user chose the toggle's text; the agent
   raised the screen reader name, and the user chose a longer one.
   → decision 7
7. **Thresholds and ADR-001.** Tailwind dropped the agent's first route;
   Sonnet found Tailwind's own container prefix, and the agent preferred
   it to the agreed fallback (the build writing CSS). The user chose it.
   → decision 9. Sonnet replaced `ch` with `em` after a first-visit
   error in Chromium, and let a long heading word break to pass the
   checks; the agent accepted both and told the user. → decisions 10, 11
8. **A limit.** Sonnet found the toggle can stay hidden under the
   reader's letter spacing; the agent proposed accepting it; the user
   agreed, with alternatives for the look card, and wished for a
   Kindle-like control one day. → decisions 12, 16
9. **The box.** Before the commit the user asked to see the result and
   found code indistinguishable from the text. The agent proposed
   committing the slice first and the box as a slice of the look card,
   and showed backgrounds within the contrast the site checks; the user
   asked for white text, and chose off-white on dark grey, then the same
   colour for code inside sentences. → decisions 13, 14
10. **What next for code.** The user judged code good enough; the agent
    asked about the grammar slice; the user moved it to Later and added
    syntax colours. → decision 15

## Outcome

- Code blocks scroll sideways, with a "wrap" toggle where a line does
  not fit, and a box; "After Save Event" is back in the readability and
  content-layout checks. 85 unit tests and 220 browser tests pass
  (Chromium); the new code-block test also on WebKit.
- Commits: `1e265b8` wide-content card to `wip`, `37f023b` the wrap
  toggle, `39c37f8` card on font and spacing controls, `a4e630d` look
  card to `wip`, `6456869` the box, `1c5bf40` code inside sentences,
  `c9d0316` cards on grammar breaks and syntax colours, plus this
  minute.

## Open points

- Firefox was not tried in the spikes: its install hung on this
  machine. The browser tests run on Chromium only.

## Next steps

- The look of the articles: text size, headings, quotes, tables; the
  user's next session. Card
  [give the articles' code, quotes and tables a look](../wip/design-look-of-articles.md).
- Tables and the hybris PK article back in the checks. Card
  [let the Markdown ask for components for wide code and tables](../wip/annotate-markdown-for-wide-content.md).
- Firefox in the tests. Card
  [run the browser tests in more browsers](../backlog/run-browser-tests-in-more-browsers.md).
