# Session 2026-11-03 and 2026-11-04 — The Medium articles and their pages

**Participants:** the user (author); the agent (Claude Opus 5.5), with a
sub-agent on Claude Sonnet 5.5 (code).
**Goal:** bring onto the site the two articles already on Medium, the
card opened in the previous session
([minute](2026-11-03-about-page-and-views-statement.md)). The card was
split during this session (`bfa0705`); three of the new cards were
done:
[the Markdown of "After Save Event"](../done/2026-11-03_add-after-save-event-markdown.md),
[the Markdown of the article on hybris PK values](../done/2026-11-03_add-generate-pk-article-markdown.md)
and [a page for each article](../done/2026-11-04_add-article-pages.md).
The reasons and the rejected options are in the cards.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The source is the text published on Medium, not the author's notes | user | agent | — |
| 2 | Explaining the platform's behaviour is the point; the code the published text quotes is usage information and stays | user | agent | — |
| 3 | The diagrams from the vendor's documentation become tables written by the author | user | agent | — |
| 4 | The card is split in four: the Markdown of each article, the pages, their look | user | agent | — |
| 5 | No card for an "article" abstraction: a line in the `residuality` card instead | user | agent | — |
| 6 | The canonical link, with the site as the original, goes to the card on search-engine tags | user | agent | — |
| 7 | Typos and slips are corrected | user | agent | — |
| 8 | The articles live in `content/articles/` | user | agent | — |
| 9 | The link between the articles points to Medium until the pages card | user | agent | — |
| 10 | One closing sentence: what is quoted from SAP Commerce belongs to SAP, MIT-0 covers only the author's code | user | agent | — |
| 11 | In the PK article, the step that reads the counter is restored and the two final links say where they lead | user | agent | — |
| 12 | The pages are at `articles/<name>.html` | user | agent | — |
| 13 | They are reached by address only, with no link from other pages | user | — | agent |
| 14 | Nothing is published before the navigation, and the navigation waits for the pages to pass the checks and the user's validation | user | agent | — |
| 15 | The title lives in the code, removed from the Markdown | user | agent | — |
| 16 | A new card on Markdown annotations for wide code and tables, which takes that part of the look card | user | agent | — |
| 17 | The article pages stay out of the readability and layout checks until that card | user | agent | — |
| 18 | The article pages are not added to the accessibility-tree check yet | agent | — | user |
| 19 | The pages card is closed as it is; the user's observations go to the look card, for articles only | user | agent | — |

## Discussion

1. **What may be published.** The agent raised that the author's notes
   held code from the decompiled platform. The user corrected the
   source: the Medium text, not the notes, which hold more than was
   published. When the agent's concern went on to the subject itself,
   the user objected that it would forbid writing about the platform at
   all; the agent narrowed it to the form of the vendor's code and
   drawings. The user judged a vendor diagram harmless but accepted a
   table, which is also its text description. → decisions 1, 2, 3, 10
2. **The split.** The user split the card and also proposed one for
   abstracting the domain; on second thought the user saw that an
   "article" model would come out of the `residuality` round, and left
   a line there. → decisions 4, 5, 6
3. **The texts.** The user pasted both articles from Medium and asked
   for typos and slips to be corrected, since the site's copy becomes
   the original. Rebuilding the PK article, the agent found a value
   used before being read and restored the step from the notes. → decisions 7, 8, 9, 11
4. **The pages.** The user chose the address and access by address
   only, and set the condition for publishing: the article pages as
   good as the curriculum, the user's own validation included. The
   agent found that the frame's title and the Markdown's title made
   two headings; the user chose the first of the two options. → decisions 12 to 15
5. **Wide code and tables.** The readability test forbids both
   sideways scrolling and boxes that scroll, and the articles have
   both. The user saw a need bound to come, telling the presentation
   layer how to show a piece of text without HTML in the Markdown, and
   proposed annotations naming components; the agent raised that
   pulldown-cmark drops the proposed marker form, which the card now
   carries. → decisions 16, 17, 18
6. **The user's reading.** The user found the text too small and asked
   for code in a visible box, and for article headings in the spirit of
   an 1980s BBS text. The agent first read it as a change for the whole
   site; the user answered that a curriculum is not an article. → decision 19

## Outcome

- Card started (`c99ad2b`) and split (`bfa0705`).
- "After Save Event": card started (`a213597`), Markdown (`7eaa0b2`),
  card closed (`d428df2`).
- Article on hybris PK values: card started (`25efec1`), Markdown
  (`afda344`), card closed (`9a3acea`).
- Pages: card started (`7dea50d`), card on annotations opened
  (`05bb2b6`), pages written by the Sonnet sub-agent and checked by the
  agent (`74a0b21`), card closed (`3a613f5`). 78 unit tests and the
  browser tests pass.
- Pushed by the user. No version published.

## Open points

- Whether the comments in the configuration of "After Save Event" are
  the author's: the author may check them on an old backup. The text
  no longer depends on it.

## Next steps

Actions that came out of this session; when they are done is up to the
plan.

- Let wide code and tables be shown on a phone, and put the article
  pages back into the readability checks: card
  [let the Markdown ask for components for wide code and tables](../backlog/annotate-markdown-for-wide-content.md).
- Text size, code in a visible box, headings of articles, and whether
  the pages join the accessibility-tree check: card
  [give the articles' code, quotes and tables a look](../backlog/design-look-of-articles.md).
- Link the articles and publish a version: card
  [add site navigation](../backlog/add-site-navigation.md).
- The canonical link, and the setting to change on Medium: card
  [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md).
- Whether an "article" model is needed: card
  [run a full round of `residuality`](../backlog/run-full-residuality-round.md).
