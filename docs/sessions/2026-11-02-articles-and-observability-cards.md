# Session 2026-11-02 — Cards on articles, observability and licence

**Participants:** the user (author); the agent (Claude Opus 5.5).
**Goal:** the part of the session that followed the minute
[the text set in Literata, version 0.5.0](2026-11-02-readability-in-literata.md):
the user's reading on a phone, and new cards for the plan.

## Decisions

Roles as in MADR: *decision-makers* decided, *consulted* gave input,
*informed* were told afterwards.

| # | Decision | Decision-makers | Consulted | Informed |
|---|---|---|---|---|
| 1 | Record the phone reading of version 0.5.0 in the closed readability card now; judge again the next morning | user | agent | — |
| 2 | A card to observe the visitors in production, any tool; GA4 and GTM only as known examples; cookie policy included; in Next | user | agent | — |
| 3 | The navigation moves from Later to Next | user | — | agent |
| 4 | A card to add the two articles already on Medium, copied under `content/` for now, in Now | user | agent | — |
| 5 | The articles' licence moves to Now, before the articles | user | agent | — |

## Discussion

1. **Phone reading.** The user had read version 0.5.0 on a phone: the
   font reads well, the page looks a little dim, the role headings in
   inverse video may read worse. The agent noted the darker amber under
   automatic brightness and the regular Literata on a narrow amber
   block, as things to look at again. → decision 1
2. **Analytics.** The user asked for GA4 and GTM. The agent wrote the
   card noting that Google Fonts had been rejected for visitors'
   privacy. The user explained the need behind it: no observability in
   production, not knowing the site's readers; GA4 and GTM were only the
   tools known, and a cookie policy is needed too. The agent reframed
   the card and asked whether Next fits, the user having said it makes
   no sense with one page. → decisions 2, 3
3. **Articles.** The user put the two Medium articles in Now, with the
   navigation in Next. The agent read the sources and recorded what they
   need (HTML in the Markdown, code blocks, image alternatives, a page
   per article, canonical address), and pointed out the licence card
   asks to be decided before the first article. → decisions 4, 5

## Outcome

- `b892027`: the phone reading in the closed card
  [improve the site's readability](../done/2026-11-02_improve-site-readability.md).
- `70cfe52`, `28e299b`: the card
  [observe the visitors in production](../backlog/observe-visitors-in-production.md),
  first opened as "add Google Analytics 4 and Google Tag Manager".
- `14f67c3`: the card
  [add the two articles already on Medium](../backlog/add-the-two-medium-articles.md)
  in Now; the navigation in Next.
- `884f1e4`: the card
  [choose the licence of the articles](../backlog/choose-articles-licence.md)
  in Now, before the articles.

## Open points

- Whether the page's brightness and the inverse-video headings need
  revisiting: the user's judgement the next morning.

## Next steps

- Look again at version 0.5.0 on the phone; if needed, open a card from
  the note in the closed readability card.
- Choose the licence of the articles: card
  [choose the licence of the articles](../backlog/choose-articles-licence.md).
- Bring the two Medium articles onto the site: card
  [add the two articles already on Medium](../backlog/add-the-two-medium-articles.md).
