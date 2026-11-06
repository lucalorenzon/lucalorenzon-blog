# Session 2026-11-06 — The canonical link, the description and the Open Graph tags

**Participants:** the user (author); the agent (Claude Opus 5.5), with
the sub-agent Sonnet 5.5 (the code of both slices and the home page's
tab title, with their tests).
**Goal:** take the card on SEO and Open Graph tags from the backlog and
build its first slices. The card was split twice; two parts were built
and closed,
[add the canonical link to every page](../done/2026-11-06_add-canonical-link-to-every-page.md)
and
[add the description and the Open Graph tags to every indexed page](../done/2026-11-06_add-description-and-open-graph-tags.md),
while [check SEO with Lighthouse in the build](../backlog/check-seo-with-lighthouse.md)
and [add preview images](../backlog/add-preview-images.md) wait.
Reasons and rejected options are in the cards. Nothing was pushed.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | The first slice is the canonical link on the articles, with its own tests; the compliance check later | user | agent | — |
| 2 | The card split: the canonical link to a card of its own, in Now | user | agent | — |
| 3 | The canonical link on every page, not only the articles, so the page frame keeps one head | user | agent | Sonnet |
| 4 | The "page not found" asks not to be indexed (`noindex`), in this slice, with no canonical link | user | agent | Sonnet |
| 5 | Every page gives the page frame either its indexing values or "do not index"; never an optional tag | agent | Sonnet | user |
| 6 | The canonical link card closed, the SEO card resumed for its next slice | user | — | agent |
| 7 | The next slice: every indexed page with its description and every Open Graph tag it needs; the missing values written in this session | user | agent | — |
| 8 | The preview images in a session of their own, maybe after the logo is restyled | user | agent | — |
| 9 | The SEO compliance check with Lighthouse's rules, anticipating the performance card's choice of tool | user | agent | — |
| 10 | The card split in three: description and Open Graph tags (Now), the Lighthouse check (Now, after), the preview images (Next) | user | agent | — |
| 11 | `og:locale` is `en_GB`; `lang="en-GB"` left to the card on declared languages | user | agent | — |
| 12 | The preview's title is the page's heading, without the site's name; "Home Page" kept | user | agent | — |
| 13 | `og:site_name` is "Luca Lorenzon", not "Luca Lorenzon Blog" | user | agent | — |
| 14 | `og:type`: `profile` for the curriculum and the about page, `website` for the home and licence pages, `article` for the articles | user | agent | — |
| 15 | The four new descriptions, reworded together and approved one by one | user | agent | — |
| 16 | The home page's tab title is "Luca Lorenzon - Product Engineer" | user | agent | Sonnet |
| 17 | The description card closed; the Lighthouse check before pushing and publishing | user | — | agent |

## Discussion

1. **What the first slice gives.** The agent pointed out that the
   articles' fields already lived in the code, unlike what the card
   assumed, and that the canonical link was the only part losing
   something each day it waited, while Medium's copies claim to be the
   original. Asked what a first slice must give, the user chose the
   canonical link on the articles, the check later; the agent asked
   whether "later" meant no tests, and the user confirmed the slice
   keeps its own tests. → decisions 1, 2
2. **Two heads in the page frame.** Sonnet's first version needed two
   almost identical heads, since Leptos leaves a marker for a missing
   optional tag; the agent proposed keeping them, with a note for the
   later tags. The user asked whether a canonical link on every page
   would remove the problem; the agent agreed, and raised the "page not
   found" as the exception. → decision 3
3. **The "page not found".** The user asked whether that page should be
   indexed at all. The agent checked the live site: a missing address
   answers 404, but `/404.html` asked directly answers 200, so it could
   be indexed; it proposed "do not index" in place of the canonical
   link. The agent asked which of two readings the user's answer meant;
   the user chose "noindex" in this slice. → decisions 4, 5
4. **The next slice.** The user asked for every indexed page's
   description and Open Graph tags, the missing values written in the
   session. The agent listed the tags read from the protocol and raised
   the image, which would anticipate the logo card; the user moved the
   images to a session of their own. → decisions 6, 7, 8
5. **The compliance check.** The user put it in the slice. The agent
   set out Lighthouse's rules against a list of the project's own; the
   user chose Lighthouse. The agent then pointed out that the check
   held decisions of its own and proposed two cards created together;
   the user agreed, and the images became a third. → decisions 9, 10
6. **The values, one question at a time.** The agent asked about the
   locale, the preview's title, the site's name and the page types,
   naming the tensions found: "Home Page" alone in a preview, and the
   about page speaking of the author as the curriculum does. The user
   chose each value; on the site's name the user was undecided, and
   chose after the agent set out both sides. → decisions 11 to 14
7. **The descriptions.** The agent drafted four; the user reworded the
   home page's and the curriculum's, and dropped the reuse part from
   the about page's. Asked for an opinion, the agent suggested
   concrete words for "other solutions", a longer about description,
   and wrote "if it is true" about the international context; the user
   explained a career spent with a multinational group's headquarters
   and markets on several continents, and with an American company's
   departments abroad. The agent acknowledged it had only the
   anonymised curriculum to go by and should have asked; the wording
   was kept. → decision 15
8. **How the previews look.** The user asked how the site would appear
   on Google and LinkedIn. The agent sketched both, and noticed Google
   shows the tab title, not the preview's, so the home page would read
   "Home Page - Luca Lorenzon"; the user chose a new tab title. →
   decision 16
9. **Closing.** The user asked to close the card and the session,
   without pushing, and to do the Lighthouse check first. → decision 17

## Outcome

- The first split and the canonical link: `39e4db4` card to `wip`,
  `f75df46` split, `05bbd2c` answers, `886b357` code, `eb533a5` card
  closed.
- The second split and the Open Graph tags: `a6f7df2` card resumed,
  `3d51b6f` split in three, `763f5c6` answers, `0c98c76` code,
  `be31a0a` and `fc6ff5b` the home page's tab title, `4626970` card
  closed.
- Every check of the pipeline passed locally on the last code commit:
  173 unit tests, 848 browser tests; the stylesheet is unchanged.
- Nothing pushed: `main` is 16 commits ahead of `origin/main`, with
  this minute: the 13 of this session and 3 of the previous one
  (`20209d9`, `d5515a4`, `9833ef0`), never pushed either.

## Open points

- The published curriculum shows the international context explicitly
  only once; the agent suggested making it visible without employer
  names, in a card of its own. The user did not answer; no card was
  opened.

## Next steps

- Check SEO with Lighthouse in the build, with the block-list the
  "page not found" needs, before pushing and publishing: card
  [check SEO with Lighthouse in the build](../backlog/check-seo-with-lighthouse.md).
- After the next version is published, point the canonical link of
  each Medium story at the site, by the author: card
  [add the canonical link to every page](../done/2026-11-06_add-canonical-link-to-every-page.md),
  "Left to the author".
- The preview images: card
  [add preview images](../backlog/add-preview-images.md).
- `lang="en-GB"` on the pages: card
  [check the declared languages in the build](../backlog/check-declared-languages-in-the-build.md).
- The home page's new tab title in the reference transcript: card
  [update the screen reader check for an English site](../backlog/update-screen-reader-check-for-english-site.md).
