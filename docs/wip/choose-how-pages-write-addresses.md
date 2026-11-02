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
- **Option A, base path in every address** (today's ADR-002): every
  address carries the prefix. Straightforward to read; a gateway with
  another prefix has to rewrite every address of every page.
- **Option B, one `<base href>` element** (raised by the user): one
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

## Questions

- How much the gateway under another prefix weighs, today a
  hypothesis, against in-page links, which will come with the articles.
- Whether the remedies option B needs are acceptable, and where they
  live (build program, content rules, tests).
- Whether ADR-002 stays, changes, or is replaced.
