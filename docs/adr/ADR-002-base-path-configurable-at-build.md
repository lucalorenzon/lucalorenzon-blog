# ADR-002: Base path configurable at build, used by every page

- **Date:** 2026-11-02
- **Status:** Accepted
- **Card:** [serve the site's own "page not found"](../wip/serve-own-not-found-page.md)
- **Replaces:** the rule "relative paths only", decided by the agent on
  2026-10-28 in the card
  [add header, footer and favicon](../done/2026-10-28_add-header-footer-and-favicon.md)
  (minute of that session, decision 3)

## Context

Every page refers to other files: its stylesheet, its icon, and soon
other pages. Until now each page did so by a relative path ("the file
next to me"), a rule taken when the site lived in a sub-folder of
GitHub Pages: a relative path works whatever folder the site sits in.

The site's own "page not found" breaks that rule. The host serves it
for any address that does not exist, at any depth: at `/a/b/c`, a
relative `style.css` becomes `/a/b/style.css`, which does not exist,
and the page loses its look and its icon. To find its files, this page
has to know where the site starts.

The user also wants the site to stay portable: today it lives at the
root of `www.lucalorenzon.com`, but it may one day sit behind a gateway
that serves other things too, under a prefix such as `/blog/`. Writing
addresses from the root of the domain (`/style.css`) would fix the
"page not found" and lose that portability.

## Decision

1. **The site's base path is a value given to the build**, the address
   at which the site starts. When none is given, it is `/`, the root,
   which is where the site lives today, locally, in the pipeline and in
   production.
2. **Every page writes every address from that base path:** the
   stylesheet, the icon, links to other pages. One rule for all pages,
   not one rule for the "page not found" and another for the rest.

Why: the base path is the only thing that tells a page where the site
starts whatever the depth of the address it is served at, so it solves
the "page not found" and keeps the site movable under a prefix with a
single change at build time. The user chose one rule for all pages over
keeping relative paths where they still work: a reader of the code
finds one way of writing an address, and every page behaves the same
under any base path.

## Consequences

**Positive:**
- The "page not found" finds its stylesheet and icon at any depth.
- Moving the site under a prefix means building it with another base
  path, without touching the pages.
- The default `/` needs no configuration for the site as it is today.

**Negative / Risks:**
- A build with the wrong base path gives pages that look broken, with
  no error from the build. A relative path could not go wrong this way.
- The pipeline builds the site once and publishes that build: a site
  published under another base path needs a build of its own.
- Today every check runs with the base path `/`; a different one is
  not tested until a test builds the site with it.
- Serving the site behind a gateway still needs the gateway to answer
  missing addresses with the site's "page not found", as GitHub Pages
  does today. The base path fixes the page's addresses, not who serves
  it.

## Alternatives Considered

| Alternative | Why rejected |
|---|---|
| Keep relative paths (the rule of 2026-10-28) | Works for every page except the "page not found", which loses its look at any depth below the root |
| Addresses from the root of the domain (`/style.css`), no setting | Fixes the "page not found", but breaks every page behind a gateway that serves the site under a prefix |
| Base path only on the "page not found", relative paths elsewhere | Two rules for writing an address; rejected by the user in favour of one rule |
| Let the gateway rewrite the addresses | No setting in the build, but depends on a feature not every gateway has |
| A "page not found" with no external files, its style written inside it | No base path needed, but its header would be a copy of the home page's, and the user wants the same header |
| A base path the build cannot run without, as on the previous attempt (`CONTEXT_ROOT`, branch `old_main`) | Forces a setting even where the default is right, which is every place the site runs today |

## Technical Notes

- **Scope:** addresses written in the HTML pages. Addresses inside the
  stylesheet (the fonts, `url("fonts/...")`) stay relative: the browser
  resolves them against the stylesheet's own address, so they follow
  the base path without any change.
- **How the value reaches the build** (an argument of the build
  program, an environment variable of `scripts/build-site.sh`) and how
  it is normalised (leading and trailing `/`) are left to the
  implementation, in the card that introduces it.
- The unit test `links_favicon_with_relative_path` in
  `src/home_page.rs` enforces the rule this decision replaces, and
  changes with it.

## References

- Card: [serve the site's own "page not found"](../wip/serve-own-not-found-page.md)
- Replaced rule: [add header, footer and favicon](../done/2026-10-28_add-header-footer-and-favicon.md),
  section "Which layout?"
- Earlier attempt: story `EP-001-UC-001-S006` on branch `old_main`
  (`CONTEXT_ROOT`)
- GitHub Pages: [creating a custom 404 page](https://docs.github.com/en/pages/getting-started-with-github-pages/creating-a-custom-404-page-for-your-github-pages-site)
