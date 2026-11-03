# Give each article a page of its own

- **Opened:** 2026-11-03, by the user, splitting the card on adding the
  two articles already on Medium; in Now.
- **Status:** noticed, not started.

## Why

The texts of the two articles from Medium are prepared in their own
cards ([After Save Event](add-after-save-event-markdown.md),
[hybris PK values](add-generate-pk-article-markdown.md)); this card
makes them visible: it decides the address of each article and how it
is reached, then writes the pages in the simplest way. Their look
(code, quotes, tables) is the [look card](design-look-of-articles.md);
the canonical link, telling search engines that the site's copy is the
original, is the card
[add SEO and Open Graph tags](add-seo-and-open-graph-tags.md).

## Known from the start

- **What the build writes today:** `index.html`, `about.html`,
  `licence.html` and `404.html`. Each page is a Markdown file under
  `content/` and a Leptos page in `src/`; there is no "article" with
  fields of its own. The user expects that to come out of the card
  [run a full round of `residuality`](run-full-residuality-round.md),
  and wants the simplest pages here.
- **Tables are not converted yet:** `src/markdown.rs` runs
  pulldown-cmark with no extensions, and both articles now hold tables.
  Tables are an extension of pulldown-cmark, to be switched on.
- **An article's footer** reads "Reuse with attribution", linking to
  `about.html` ([card](../done/2026-11-03_show-licences-on-the-site.md)).
- **No navigation yet:** the card
  [add site navigation](add-site-navigation.md) comes later, when the
  home page becomes the list of contents.
- **Long lines of code:** some lines go well beyond 80 characters. The
  readability test does not count characters in code, but its checks on
  horizontal scroll and cut-off text apply to the whole page, also
  before the look card exists.

## Questions

- The address of each article.
- How an article is reached while the navigation does not exist: a link
  from the home page, or only the address.
- What long code lines do on a phone until the look card decides it,
  so the readability test passes.
