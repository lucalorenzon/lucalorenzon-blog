# Choose how pages write the addresses of their files

- **Opened:** 2026-11-02, by the user, as a spike; in Now, in progress.
- **Status:** in progress since 2026-11-02.
- **Timebox:** one working session. At the end, what is known and what
  is still open is written here, even if the answer is not complete.

## Why

While answering the card
[serve the site's own "page not found"](../backlog/serve-own-not-found-page.md),
the discussion moved to a question that is not about that page: how
every page of the site writes the addresses of its stylesheet, its icon
and the other pages. The user parked the "page not found" card and
opened this one, so the question gets its own discussion instead of
being settled inside another card's context.

The decision taken earlier the same day,
[ADR-002](../adr/ADR-002-base-path-configurable-at-build.md), is under
review until this card closes.

## The question

Which way of writing addresses keeps the site working both at any depth
of the address (the "page not found" is served at `/a/b/c` too) and
behind a gateway that exposes it under a prefix such as `/blog/`:
a base path written into every address by the build, or a single
`<base href>` element with every other address relative?

## Known from the start

- **Relative paths, the rule until today,** work under any prefix
  without configuration, but break on the "page not found" at any
  depth below the root. The rule was taken by the agent on 2026-10-28
  because the site then lived in a sub-folder of GitHub Pages
  ([card](../done/2026-10-28_add-header-footer-and-favicon.md),
  section "Which layout?").
- **Addresses from the domain's root** (`/style.css`) fix the "page not
  found" but break the site behind a gateway under a prefix.
- **The user's model of a prefix:** the gateway exposes `/blog/`, and
  the site's files sit under `/blog/` too: built with `/blog/`, the
  output holds `blog/index.html`, `blog/style.css`. Address and
  location are decided by the same value, so they cannot drift apart.
- **A gateway exposing another prefix** than the one the site was
  built with has to rewrite the incoming addresses and also the
  addresses written inside the pages: a reverse proxy that filters the
  HTML (`mod_proxy_html`, nginx `sub_filter`), since `ProxyPassReverse`
  and `proxy_redirect` rewrite headers only. Without it nothing opens.
- **Base path in every address** (option D below, ADR-002): every
  address carries the prefix. Straightforward to read; a gateway with
  another prefix has to rewrite every address of every page.
- **One `<base href>` element** (option E below, raised by the user): one
  absolute address in each page's head, every other address relative;
  a gateway with another prefix rewrites one value per page, still with
  an HTML filter. It is commonly called an anti-pattern because it
  changes the meaning of every relative address in the page:
  - links to a point of the same page (`href="#note-1"`) lead to the
    base's page instead: none today, but footnotes from pulldown-cmark
    (when enabled) and a "skip to content" link (WCAG 2.4.1) are
    written that way;
  - a relative link in an article's Markdown is resolved from the
    site's root, not from the article's folder;
  - references inside inline SVG (`fill="url(#gradient)"`) break in
    some browsers: none today.
  Each has a remedy (the build writes in-page links with the page's
  full path; a rule that addresses in the content are relative to the
  site's root), at the cost of the build correcting what the element
  breaks.
- **The fonts** are referred to from inside the stylesheet and resolved
  against the stylesheet's own address: they work under either option.
- **The browser tests** serve `target/site` at the root of
  `127.0.0.1`; with a prefix, they open the site at that prefix.

## Options compared

Asked by the user on 2026-11-02: every way of writing addresses, with
its benefits and drawbacks, and what static sites commonly do.

**What other tools do.** Most static site generators write the prefix
into every address at build: Jekyll (`baseurl`, the default on GitHub
Pages), Hugo (`baseURL`), Astro (`base`), Docusaurus (`baseUrl`),
Next.js (`basePath`), Gatsby (`pathPrefix`). Documentation generators
(MkDocs, Sphinx) write paths relative to each page instead, and treat
only the "page not found" apart: Sphinx has an extension,
`sphinx-notfound-page`, that prefixes that page's addresses so it works
at any depth; MkDocs loads that page's files from its `site_url`. The
`<base>` element is usual in single-page applications (Angular,
`--base-href`), rare in static sites: Eleventy's "HTML base" plugin
applies the prefix at build time rather than relying on the element,
and its documentation calls it a bit unreliable in edge cases.

| | Option | Used by | Benefits | Drawbacks |
|---|---|---|---|---|
| A | Paths relative to the page (`style.css`, `../style.css`): the rule until today | MkDocs, Sphinx, Hugo with `relativeURLs` | No setting; works under any prefix with no new build and no rewriting in the gateway; opens from disk too | Breaks on the "page not found" served below the root; the build computes each page's depth |
| B | A, plus a base path for the "page not found" only | MkDocs, Sphinx with its 404 extension | All of A's portability, and the "page not found" works | Two rules; the "page not found" depends on a build value; the gateway must serve that page |
| C | Addresses from the domain's root (`/style.css`), no setting | Many small hand-written sites | The simplest; the "page not found" works | No prefix: behind a gateway under `/blog/` everything breaks |
| D | A base path in every address (ADR-002) | Jekyll, Hugo, Astro, Docusaurus, Next.js, Gatsby: the most common | One rule; the "page not found" works; known to everyone | One build per prefix; a gateway with another prefix rewrites the whole HTML; a wrong value gives pages without their look |
| E | One `<base>` element, every other address relative | Angular and single-page applications; rare in static sites | One value per page; a trivial rewrite for the gateway | Breaks `#...` links (footnotes, "skip to content"), changes the meaning of relative links in the Markdown, breaks `url(#...)` in inline SVG |
| F | Full addresses with the domain (`https://www.lucalorenzon.com/style.css`) | Hugo's `absURL`, RSS feeds | No ambiguity | Tied to the domain; locally the pages point at the published site; another domain needs another build |
| G | A self-contained "page not found", its style written inside it | Web servers' error pages | No path to know | The fonts, referred to by the stylesheet, still break below the root; a heavier page |

Two things the comparison brought out:

- **Only relative paths give the portability the user is after.** A
  gateway can expose the site under any prefix without rewriting and
  without a new build. A base path in every address (D) gives less,
  not more: it fixes the prefix at build time. The only page with a
  problem is the "page not found".
- **A build value comes anyway.** The card
  [add SEO and Open Graph tags to every page](../backlog/add-seo-and-open-graph-tags.md)
  needs the site's full address (`og:url`, `og:image`, the canonical
  link). MkDocs takes the "page not found"'s path from that same value.

Sources: [Eleventy, HTML base plugin](https://11ty.dev/docs/plugins/html-base);
[sphinx-notfound-page, configuration](https://sphinx-notfound-page.readthedocs.io/en/stable/configuration.html)
and [FAQ](https://sphinx-notfound-page.readthedocs.io/en/stable/faq.html);
[Read the Docs, 404 pages and MkDocs](https://docs.ReadTheDocs.io/en/stable/reference/404-not-found.html).

## Answers so far

- **Chosen: B,** by the user on 2026-11-02. Every page keeps paths
  relative to itself; only the "page not found" writes its addresses
  from a base path given to the build, default `/`. Why: it keeps the
  site movable under any prefix with no new build and no rewriting in
  the gateway, which is the portability the user wants, and confines
  the base path to the one page relative paths cannot serve. The user
  had first asked for one rule for all pages (ADR-002, option D); the
  comparison showed that one rule gives less portability, and the user
  chose portability over a single rule.
- **Rejected:** C, which breaks behind a gateway under a prefix; D, one
  build per prefix and a full HTML rewrite in a gateway with another
  prefix; E, which breaks in-page links that articles will bring; F,
  tied to the domain and to the published site even locally; G, whose
  fonts still break below the root.

## Questions

- Where the "page not found"'s base path comes from: a value of its
  own, or the path part of the site's full address that the SEO and
  Open Graph card will need.
- ADR-002 is to be replaced by a record of this choice.
