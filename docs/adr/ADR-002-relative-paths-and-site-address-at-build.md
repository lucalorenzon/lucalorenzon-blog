# ADR-002: Relative paths in every page, the site's address given to the build

- **Date:** 2026-11-02
- **Status:** Accepted
- **Card:** [choose how pages write the addresses of their files](../done/2026-11-02_choose-how-pages-write-addresses.md)
- **History:** first written the same day with a different decision, a
  base path in every address; questioned by the user while reading it,
  revised after the card above. The first version is in the git
  history.

## Context

Every page refers to other files: its stylesheet, its icon, and soon
other pages. Since 2026-10-28 each page does so by a path relative to
itself ("the file next to me"), a rule the agent took because the site
then lived in a sub-folder of GitHub Pages
([card](../done/2026-10-28_add-header-footer-and-favicon.md), section
"Which layout?"). The rule was never recorded as a decision of its own.

Two needs put it to the test:

- **The site's own "page not found"**
  ([card](../done/2026-11-02_serve-own-not-found-page.md)). The host serves it
  for any address that does not exist, at any depth: at `/a/b/c`, a
  relative `style.css` becomes `/a/b/style.css`, which does not exist,
  and the page loses its look and its icon.
- **Portability.** The site lives today at the root of
  `www.lucalorenzon.com`, but the user wants to be able to put it
  behind a gateway that serves other things too, under a prefix such as
  `/blog/`, without reworking it.

## Decision

1. **Every page writes addresses relative to itself,** as it does
   today. The rule of 2026-10-28 stays, now for portability rather than
   for the GitHub Pages sub-folder.
2. **The site's full address is a value given to the build,** by
   default `https://www.lucalorenzon.com/`. It is the only setting about
   addresses.
3. **Only the "page not found" uses it,** and only its path part (`/`,
   or `/blog/` behind a gateway): it writes its addresses from there, so
   it finds its files at any depth.
4. **The full address is kept for what needs a full address,** the
   search-engine and Open Graph tags of the card
   [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md),
   when that card is done.

Why: only relative paths let a gateway expose the site under any prefix
with no new build and no rewriting of the pages, which is the
portability the user wants. The one page they cannot serve is the "page
not found", so the base path is confined to it, as the documentation
generators MkDocs and Sphinx do. Taking that path from the site's full
address, which the SEO card will need anyway, keeps one setting instead
of two that would have to agree. The user had first chosen one rule for
every page (a base path in every address); the comparison in the card
showed it gives less portability, and the user chose portability over
a single rule.

## Consequences

**Positive:**
- A gateway can expose the site under any prefix without a new build
  and without rewriting the pages; only the "page not found" needs the
  prefix, through the site's address.
- The "page not found" finds its stylesheet and icon at any depth.
- One setting, with a default that is right for the site as it is
  today, locally, in the pipeline and in production.

**Negative / Risks:**
- Two ways of writing an address: relative in every page, from the
  base path in the "page not found". A reader of the code has to know
  why that page is different; its code says so.
- The "page not found" depends on the site's address being right: a
  site built for `/` and exposed under `/blog/` shows that page without
  its look. The other pages are not affected.
- The "page not found" is served by the host, not by the site: behind a
  gateway, the gateway has to answer missing addresses with it, as
  GitHub Pages does today.
- The build computes, for every page, the relative path to the site's
  root (`../` once per level of depth). Today every page is at the root.
- The local browser tests serve the site at `127.0.0.1`, where the path
  part of the default address, `/`, is right. A different prefix is
  not tested until a test builds the site with it.

## Alternatives Considered

The full comparison, with what static site generators commonly do, is
in the [card](../done/2026-11-02_choose-how-pages-write-addresses.md).

| Alternative | Why rejected |
|---|---|
| Relative paths only, no setting | The "page not found" loses its look at any depth below the root |
| Addresses from the domain's root (`/style.css`), no setting | Breaks every page behind a gateway under a prefix |
| A base path in every address (the first version of this record; Jekyll, Hugo, Astro and most generators) | One build per prefix, and a gateway with another prefix has to rewrite the whole HTML: less portability than relative paths |
| One `<base>` element, every other address relative | Breaks in-page links (footnotes, "skip to content") that articles will bring, and changes the meaning of relative links in the Markdown |
| Full addresses with the domain | Tied to the domain; locally the pages would load files from the published site |
| A self-contained "page not found", its style written inside it | The fonts, referred to by the stylesheet, still break below the root |
| A base path of its own for the "page not found", besides the site's address | Two settings that have to agree once the SEO card needs the full address |
| A setting the build cannot run without, as on the previous attempt (`CONTEXT_ROOT`, branch `old_main`) | Forces a setting even where the default is right, which is every place the site runs today |

## Technical Notes

- Addresses inside the stylesheet (the fonts, `url("fonts/...")`) are
  resolved by the browser against the stylesheet's own address: they
  stay relative under every option.
- How the value reaches the build (an argument of the build program, an
  environment variable of `scripts/build-site.sh`) and how it is checked
  (a full `https` address ending with `/`) are left to the card that
  introduces it, the "page not found".
- The unit test `links_favicon_with_relative_path` in
  `src/home_page.rs` enforces rule 1 and stays.

## References

- Card: [choose how pages write the addresses of their files](../done/2026-11-02_choose-how-pages-write-addresses.md)
- First use: [serve the site's own "page not found"](../done/2026-11-02_serve-own-not-found-page.md)
- Earlier rule: [add header, footer and favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
- [sphinx-notfound-page, configuration](https://sphinx-notfound-page.readthedocs.io/en/stable/configuration.html);
  [Read the Docs, 404 pages and MkDocs](https://docs.ReadTheDocs.io/en/stable/reference/404-not-found.html)
- GitHub Pages: [creating a custom 404 page](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site)
