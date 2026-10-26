# Backlog: redirect handling for retired article URLs

**Origin:** surfaced 2026-10-26, as a consequence of
[slug-model-id-and-slug-list.md](slug-model-id-and-slug-list.md).
**Epic:** to be decided (SEO area, currently out of EP-001's scope).
**Status:** open, own story — possibly a dedicated redirect engine.

## Why

Once an article can have inactive slugs, every old address must keep leading
readers and search engines to the current page. A broken old link loses both
visitors and the search ranking that address had earned.

## Goal

- Every inactive slug resolves to its article's active slug.
- The redirect is as strong a signal to search engines as the hosting allows.
- Decide whether this is a small step of page generation or a dedicated
  engine (raised 2026-10-26 as a likely option).

## Detail — constraint from hosting

- [ADR-003](../adr/ADR-003-repo-topology-github-pages-hosting.md) puts the
  site on GitHub Pages, which serves static files only and offers no
  server-side 301 redirects.
- The usual static-site fallback is one generated HTML page per inactive slug
  with `<meta http-equiv="refresh">` and `<link rel="canonical">` pointing at
  the active URL. It works, but search engines weigh it less than a real 301.
- A dedicated engine that issues real 301s would need something in front of
  or instead of GitHub Pages (an edge/CDN rule layer, or another host). That
  would revisit ADR-003 and must be decided there, not inside this story.
