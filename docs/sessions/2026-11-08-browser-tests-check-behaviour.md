# Session 2026-11-08 — Make the browser tests check behaviour, not articles

**Participants:** the user (author); the agent (Claude Opus 5.5), with
sub-agents Claude Sonnet 5.5 (test and build code, four tasks).
**Goal:** pick up the card on making the browser tests check behaviour,
not articles, since split; its decision is
[ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md).
Technical details and reasons are in the ADR and the cards linked below.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The tests certify the software, not the content; the suite must stop growing with each article | user | agent | — |
| 2 | No random sample of articles; each check is run as often as its result can change | user | agent | — |
| 3 | Four classes: frame, component on its worst case, net over every page, content facts removed | user | agent | — |
| 4 | Components tested once, like unit tests; pages by a parametric net; real faults become fixed variants | user | agent | — |
| 5 | The browser tests do not wait for a design system; components' cases go in test pages shaped as a catalogue | user | agent | — |
| 6 | New card on a design system, Holt first, Storybook as fall-back | user | agent | — |
| 7 | The card split into slices, its census and classification in ADR-004 | user | agent | — |
| 8 | Everything in the light theme; the dark theme only for tests tagged `@colours`; one guard test | user | agent | — |
| 9 | The widths card widened to every layout test, with named losses | user | agent | — |
| 10 | Five widths for what is inside the text column | user | agent | — |
| 11 | The text spacing test split, on condition that the title limit is enforced in the build first | user | agent | — |
| 12 | Graphemes counted, with `unicode-segmentation` | user | agent | — |
| 13 | The limit belongs to the frame's own `Heading`, not to the article's `Title` | user | agent | — |
| 14 | A title that does not fit is a problem found when the article is read | user | agent | — |
| 15 | The limit lives in a plain file for now, `config/heading-max-graphemes.txt` | user | agent | — |
| 16 | `DOM.Iterable` declared for the editor's TypeScript | user | agent | — |
| 17 | Two pushes, to measure the pipeline before closing each slice | user | agent | — |
| 18 | Two new cards: the SEO check's order, a structured configuration file | user | agent | — |

## Discussion

1. **The problem.** The user reported that the browser tests, nearly a
   thousand for a small site, grew page by page, repeat checks and know
   facts about articles; the repository holds the software, not the
   content, so a new article should not make the suite larger. The agent
   counted 913 tests, about 200 per article. → decision 1
2. **How to choose what to test.** The user asked whether a random
   sample of articles would do. The agent advised against it (the same
   commit could pass and fail, and an article never drawn would go live
   looking checked) and proposed the worst case for each component plus
   a light net over every page. → decisions 2, 3
3. **The design system.** The user proposed a design system (Holt) so
   that components are tested once, like unit tests, and pages by a
   parametric integration test. The agent agreed and added that some
   properties exist only in the composed page and stay in the net, and
   that real cases can become fixed variants of a component. The agent
   proposed not to wait for the design system; the user agreed and said
   Storybook would do as a fall-back. → decisions 4, 5, 6
4. **Slicing.** The agent proposed four slices and an ADR for what they
   share; the user agreed. → decision 7
5. **Themes.** The agent found that the light theme changes colours
   only, proposed the light theme for everything and a guard test; the
   user agreed. Asked whether the guard compared a theme with itself,
   the agent explained that it opens each page in both themes. →
   decision 8
6. **Widths.** Read test by test, the slice as written would have
   reduced one test. The agent proposed closing it small; the user
   rejected that and widened it. The agent measured the text column at
   the 12 widths and proposed five; the user agreed. → decisions 9, 10
7. **The header and the title limit.** Moving the header check onto
   invented titles relied on the 50-character limit agreed on
   2026-10-29 but not enforced. The user made enforcing it a condition
   and asked for a card. → decision 11
8. **Where the limit belongs.** The agent proposed graphemes or scalar
   values; the user chose graphemes. The agent proposed making the frame
   take the article's `Title`; the user rejected it, keeping the view's
   heading apart from the content's title. The agent then proposed
   checking it when the article is read, so it behaves like every other
   problem of an article; the user agreed. → decisions 12, 13, 14
9. **The configuration file.** The user asked for a configuration file;
   the agent took it as the file's name, and the user later explained
   they meant a structured file such as `Cargo.toml` or `config.toml`.
   The user kept the plain file for now. → decisions 15, 18
10. **Errors in the editor.** The user saw five errors in the editor
    that the build did not report; the agent traced them to the editor's
    older TypeScript (5.9.3 against the project's 7.0.2). → decision 16
11. **Measuring.** After the second push the browser test step took the
    same time with 199 fewer tests. The agent measured each test: the
    slice removed about 40 seconds of work, absorbed by the variance of
    the pipeline's machines; the SEO check (30 seconds, last and alone)
    and axe now bound the time. → decisions 17, 18
12. **Sub-agents' work, checked.** The agent proved each moved or new
    check can fail (the guard, the header with the reader's spacing),
    replaced a type import from a package the project does not declare,
    and corrected a test count a sub-agent had reported off by one.

## Outcome

- Browser tests: 913 → 514 → 345 → 315; in the pipeline the browser
  test job went from 189 to 121 seconds (runs 42496141109,
  42508735255, 42514256662).
- Cards closed:
  [run colour-free tests in one theme](../done/2026-11-08_run-colour-free-tests-in-one-theme.md)
  (`8641bca`),
  [check at every width only what depends on it](../done/2026-11-08_check-at-every-width-only-what-depends-on-it.md)
  (`0bfb789`, `ac44e7e`),
  [enforce the title length limit in the build](../done/2026-11-08_enforce-the-title-length-limit-in-the-build.md)
  (`1277d9a`),
  [declare DOM.Iterable for the editor](../done/2026-11-08_declare-dom-iterable-for-the-editor.md)
  (`8b2262d`).
- The original card split, with
  [ADR-004](../adr/ADR-004-test-components-once-and-pages-with-a-net.md)
  (`017269d`).
- Cards opened:
  [build a design system](../backlog/build-a-design-system.md),
  [start the SEO check first](../backlog/start-the-seo-check-first.md),
  [use a structured configuration file](../backlog/use-a-structured-configuration-file.md)
  (`b25708e`, `c0129bb`).
- The generated site is unchanged; pushed up to `ac44e7e`, the commits
  after it are local.

## Open points

- Which of the three losses listed in ADR-004 are accepted is left to
  the card on the net and to the components' test pages.
- The pipeline's timings vary by more than a third between runs on unchanged
  tests, so a saving under about 20 seconds cannot be told from noise
  with one run.

## Next steps

- Check every page with tests written once: card
  [check every page with tests written once](../backlog/check-every-page-with-tests-written-once.md).
- Put the components' cases in test pages shaped as a catalogue: no card
  yet, to be written after the net (ADR-004, Consequences).
- Start the SEO check first: card
  [start the SEO check first](../backlog/start-the-seo-check-first.md).
- Build a design system: card
  [build a design system](../backlog/build-a-design-system.md).
